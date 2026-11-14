# Meatware Governance

**Status:** Operating draft. Items marked decision required are not adopted policy.

## Purpose
Define how decisions are proposed, reviewed, recorded and reversed.

## Roles

| Role | Responsibility | Boundary |
|---|---|---|
| Repository maintainer | Access, triage, merges, security, releases | Cannot silently rewrite canon or contributor rights |
| Project editor | Story structure, character consistency, tone, canon proposals | Records material canon changes and rationale |
| Task owner | Scope, acceptance criteria, review coordination | Cannot be sole approver of own work |
| Contributor | Submit scoped work, feedback and proposals | Submission does not itself change canon or grant rights |
| Technical reviewer | Validate tooling, manifests, automation and reproducibility | Technical validity does not confer creative approval |

One person may hold several roles while the project is small. Decisions and conflicts must still be recorded.

## Decision classes
1. **Editorial:** wording, pacing, joke selection, scene clarity.
2. **Canon:** character invariants, world rules, chronology and major plot facts. Explicit decision-log entry required.
3. **Production:** modality, budget assumptions, pipeline and delivery standards. Evidence-backed decision memo required.
4. **Technical:** repository structure, schemas and automation. Maintainer approval plus relevant review.
5. **Governance:** rights, licensing, credit, privacy, safety and commercial use. Explicit maintainer decision required; seek qualified legal advice where appropriate.
6. **Operational:** task priority, issue status and outreach cadence.

## Decision process
1. State the question and why it matters.
2. Link affected source documents and tasks.
3. Separate facts, assumptions, options and consequences.
4. Invite focused feedback when contributors or canon are affected.
5. Record decision, owner, date, rationale and affected artifacts.
6. Update the authoritative source and derived indexes.
7. Preserve superseded decisions in the decision log.

## Canon change control
- The canon directory contains authoritative current canon.
- The continuity register contains unresolved contradictions and questions.
- The development decision log records adopted creative decisions and rejected alternatives.
- Film drafts may explore proposals but cannot silently override canon.
- A proposal becomes canon only after explicit editorial adoption and synchronized updates.

## Review and merge
- Every contribution identifies its task or explains why it is unsolicited.
- Substantial changes receive review from someone other than the author.
- Review covers scope, canon, continuity, originality, provenance, safety and production utility.
- AI assistance is allowed; unreviewed output is not accepted work.
- Rejected material receives a brief reason where practical.

## Decisions still required

| Decision | Current state | Interim safeguard |
|---|---|---|
| Rights/licensing for story, scripts, art, code and generated assets | Open; issue #16 | Feedback and non-exclusive suggestions only |
| Credit weights and public display | Proposed | Attribution only for accepted work; no financial entitlement |
| Final canon authority | Proposed: project editor, maintainer resolves disputes | Record all canon changes |
| Commercial production permissions | Not decided | Do not imply permission or future participation |
| Anonymous/pseudonymous attribution | Proposed | Collect no unnecessary personal data |

Material governance changes must be recorded in the development decision log.
