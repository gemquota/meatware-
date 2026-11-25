//! Deterministic, read-only interaction scheduling.
//!
//! Rules inspect one immutable source configuration and return ordinary proposals.
//! The scheduler assigns stable proposal identities and event-local random streams;
//! it does not interpret the meaning of any emitted edit.

use crate::{
    Configuration, Edit, Endpoint, Proposal, RandomStream, RuntimeError,
};
use std::collections::BTreeSet;
use serde::{Deserialize, Serialize};

pub type RuleEvaluator = fn(
    source: &Configuration,
    participants: &[Endpoint],
    random: &mut RandomStream,
) -> Option<Proposal>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InteractionError {
    ZeroArity { rule_key: u64 },
    DuplicateRuleKey(u64),
    ProposalKeyCollision(u64),
    Runtime(RuntimeError),
}

impl From<RuntimeError> for InteractionError {
    fn from(value: RuntimeError) -> Self { Self::Runtime(value) }
}

/// A versioned rule over unordered participant combinations.
///
/// `max_evaluations` is an explicit computational budget. Candidates are drawn
/// from the selected domain in stable identity order. Each combination is evaluated
/// at most once per rule and source boundary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionRuleDescriptor {
    pub key: u64,
    pub version: u32,
    pub arity: usize,
    pub max_evaluations: usize,
    pub include_entities: bool,
    pub include_relations: bool,
}

#[derive(Clone, Copy)]
pub struct InteractionRule {
    pub key: u64,
    pub version: u32,
    pub arity: usize,
    pub max_evaluations: usize,
    pub include_entities: bool,
    pub include_relations: bool,
    pub evaluate: RuleEvaluator,
}

impl InteractionRule {
    /// Returns the portable configuration metadata for this executable rule.
    /// The evaluator implementation itself is identified by the build and rule version.
    pub fn descriptor(&self) -> InteractionRuleDescriptor {
        InteractionRuleDescriptor {
            key: self.key,
            version: self.version,
            arity: self.arity,
            max_evaluations: self.max_evaluations,
            include_entities: self.include_entities,
            include_relations: self.include_relations,
        }
    }

    pub fn entities(
        key: u64,
        version: u32,
        arity: usize,
        max_evaluations: usize,
        evaluate: RuleEvaluator,
    ) -> Self {
        Self { key, version, arity, max_evaluations, include_entities: true, include_relations: false, evaluate }
    }

    pub fn all(
        key: u64,
        version: u32,
        arity: usize,
        max_evaluations: usize,
        evaluate: RuleEvaluator,
    ) -> Self {
        Self { key, version, arity, max_evaluations, include_entities: true, include_relations: true, evaluate }
    }
}

/// Evaluate rules against the same immutable source state.
///
/// Rule and participant ordering are canonical. Randomness is isolated by
/// (seed, boundary, rule, participant combination), so adding or reordering an
/// unrelated rule does not consume another rule's random stream.
pub fn evaluate_interactions(
    source: &Configuration,
    seed: u64,
    rules: &[InteractionRule],
) -> Result<Vec<Proposal>, InteractionError> {
    source.validate()?;
    let mut ordered_rules = rules.to_vec();
    ordered_rules.sort_by_key(|rule| rule.key);

    let mut seen_rules = BTreeSet::new();
    let mut seen_proposals = BTreeSet::new();
    let mut proposals = Vec::new();

    for rule in ordered_rules {
        if rule.arity == 0 {
            return Err(InteractionError::ZeroArity { rule_key: rule.key });
        }
        if !seen_rules.insert(rule.key) {
            return Err(InteractionError::DuplicateRuleKey(rule.key));
        }
        if rule.max_evaluations == 0 { continue; }

        let mut candidates = Vec::new();
        if rule.include_entities {
            candidates.extend(source.entities.keys().copied().map(Endpoint::Entity));
        }
        if rule.include_relations {
            candidates.extend(source.relations.keys().copied().map(Endpoint::Relation));
        }
        candidates.sort();

        let mut combinations = Vec::new();
        collect_combinations(
            &candidates,
            rule.arity,
            0,
            &mut Vec::with_capacity(rule.arity),
            &mut combinations,
            rule.max_evaluations,
        );

        for participants in combinations {
            let proposal_key = stable_proposal_key(rule.key, rule.version, &participants);
            if !seen_proposals.insert(proposal_key) {
                return Err(InteractionError::ProposalKeyCollision(proposal_key));
            }
            let channel = format!("interaction-rule:{}:{}", rule.key, rule.version);
            let mut random = RandomStream::for_event(
                seed,
                source.boundary,
                proposal_key,
                &channel,
            );
            if let Some(mut proposal) = (rule.evaluate)(source, &participants, &mut random) {
                proposal.key = proposal_key;
                proposal.source_boundary = source.boundary;
                proposals.push(proposal);
            }
        }
    }

    proposals.sort_by_key(|proposal| proposal.key);
    Ok(proposals)
}

fn collect_combinations(
    candidates: &[Endpoint],
    arity: usize,
    start: usize,
    current: &mut Vec<Endpoint>,
    output: &mut Vec<Vec<Endpoint>>,
    limit: usize,
) {
    if output.len() >= limit { return; }
    if current.len() == arity {
        output.push(current.clone());
        return;
    }
    let needed = arity - current.len();
    if candidates.len().saturating_sub(start) < needed { return; }

    let last = candidates.len() - needed;
    for index in start..=last {
        current.push(candidates[index].clone());
        collect_combinations(candidates, arity, index + 1, current, output, limit);
        current.pop();
        if output.len() >= limit { break; }
    }
}

fn stable_proposal_key(rule_key: u64, version: u32, participants: &[Endpoint]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    let mut feed = |byte: u8| {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    };
    for byte in rule_key.to_le_bytes() { feed(byte); }
    for byte in version.to_le_bytes() { feed(byte); }
    for participant in participants {
        match participant {
            Endpoint::Entity(id) => { feed(0); for byte in id.to_le_bytes() { feed(byte); } }
            Endpoint::Relation(id) => { feed(1); for byte in id.to_le_bytes() { feed(byte); } }
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Properties, Value};

    fn emit_relation(
        _source: &Configuration,
        participants: &[Endpoint],
        random: &mut RandomStream,
    ) -> Option<Proposal> {
        let id = match (&participants[0], &participants[1]) {
            (Endpoint::Entity(a), Endpoint::Entity(b)) => 100 + *a * 10 + *b,
            _ => return None,
        };
        let mut properties = Properties::new();
        properties.insert("sample".into(), Value::Integer(random.next_u64() as i64));
        Some(Proposal {
            key: 0,
            source_boundary: 0,
            reads: Vec::new(),
            edits: vec![Edit::CreateRelation {
                id,
                kind: "association".into(),
                endpoints: participants.to_vec(),
                properties,
            }],
        })
    }

    fn source() -> Configuration {
        let mut state = Configuration::default();
        for id in 1..=3 {
            state.entities.insert(id, crate::Entity { id, properties: Properties::new() });
        }
        state.next_id = 4;
        state
    }

    #[test]
    fn pairwise_rules_emit_generic_relationship_proposals() {
        let state = source();
        let rule = InteractionRule::entities(7, 1, 2, 10, emit_relation);
        let proposals = evaluate_interactions(&state, 42, &[rule]).unwrap();
        assert_eq!(proposals.len(), 3);
        assert!(proposals.windows(2).all(|pair| pair[0].key < pair[1].key));

        let mut next = state.clone();
        next.apply_batch(&proposals).unwrap();
        assert_eq!(next.relations.len(), 3);
        assert_eq!(next.boundary, 1);
    }

    #[test]
    fn evaluation_is_reproducible_and_does_not_mutate_source() {
        let state = source();
        let before = state.state_hash().unwrap();
        let rule = InteractionRule::entities(7, 1, 2, 10, emit_relation);
        let first = evaluate_interactions(&state, 42, &[rule]).unwrap();
        let second = evaluate_interactions(&state, 42, &[rule]).unwrap();
        assert_eq!(first, second);
        assert_eq!(state.state_hash().unwrap(), before);
    }

    #[test]
    fn evaluation_budget_is_enforced_deterministically() {
        let state = source();
        let rule = InteractionRule::entities(7, 1, 2, 2, emit_relation);
        assert_eq!(evaluate_interactions(&state, 42, &[rule]).unwrap().len(), 2);
    }

    #[test]
    fn rule_descriptor_round_trips_as_portable_metadata() {
        let rule = InteractionRule::entities(19, 3, 2, 500, emit_relation);
        let descriptor = rule.descriptor();
        let encoded = serde_json::to_vec(&descriptor).unwrap();
        let decoded: InteractionRuleDescriptor = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, descriptor);
    }

    #[test]
    fn invalid_or_duplicate_rules_are_rejected() {
        let state = source();
        let zero = InteractionRule::entities(1, 1, 0, 10, emit_relation);
        assert!(matches!(evaluate_interactions(&state, 1, &[zero]), Err(InteractionError::ZeroArity { .. })));
        let a = InteractionRule::entities(1, 1, 2, 10, emit_relation);
        let b = InteractionRule::entities(1, 2, 2, 10, emit_relation);
        assert!(matches!(evaluate_interactions(&state, 1, &[a, b]), Err(InteractionError::DuplicateRuleKey(1))));
    }
}
