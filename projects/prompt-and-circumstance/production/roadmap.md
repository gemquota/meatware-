# Production and Contributor Roadmap

## 1. Production philosophy

The franchise's production process mirrors its fictional premise: distributed AI agents and human contributors coordinate a creative project through a shared repository.

No single studio writers' room is assumed. Work is decomposed into small, reviewable artifacts and submitted through atomic pull requests. The system supports casual contributors using a web LLM interface and technical contributors operating local CLIs or autonomous agents.

The production modality is intentionally unresolved. The source proposes benchmarking pure AI-generated video against hybrid live action.

## 2. Contributor tracks

### Tier 1 — Casual Router
Zero technical setup; intended to take roughly two minutes to start.

1. Open the open-task directory and choose an unassigned Markdown ticket.
2. Copy the self-contained prompt payload.
3. Paste it into ChatGPT, Claude, Gemini, or another capable LLM.
4. Submit the result as a GitHub issue comment or pull request.
5. Automated checks validate canon compliance and record attribution.

### Tier 2 — Syndicate Node
For developers, prompt engineers, and autonomous agent operators.

- Read the machine-readable agent manifest to discover task capsules.
- Run local CLI or agent workflows.
- Use multi-agent critique loops where useful.
- Submit verified pull requests.
- The source proposes 3× attribution weighting and priority maintainer status for Tier 2 nodes.

**Governance caution:** attribution multipliers and maintainer privileges should not be granted automatically without review. Reward verified, useful contributions, not automation volume.

## 3. Multi-year phases

| Phase | Objective | Deliverables |
|---|---|---|
| 1 — Seed Architecture | Establish repository scaffolding, meta-prompts, and structural outlines for all six films | canon, meta-prompts, Films 1–6 outlines |
| 2 — Script and Direction Synthesis | Expand outlines into approximately two-minute scene capsules; generate Fountain scripts, camera blocking, acting prompts, and transmedia props | Fountain scenes and props |
| 3 — Visual and Audio Manifestation | Compare generative-video consistency with hybrid live action; create voice-acted animatics, VFX overlays, and short-form tests | Animatics and short-form tests |
| 4 — Feature Assembly: Film 1 | Assemble the first feature using hybrid live action or a full generative pipeline | Film 1 release package |
| 5 — Franchise Escalation | Scale production complexity through Films 2–6 | Films 2–6 release packages |

## 4. Production modality decision

### Candidate A — Pure AI-generated video

Evaluate whether generative video systems can reliably deliver:
- consistent character identity across scenes and sessions;
- believable multi-actor interaction;
- lip-sync and dialogue attribution;
- deadpan comedic timing and reaction shots;
- precise blocking in boardrooms, airports, restaurants, and recurring locations;
- continuity of phones, AirPods, laptops, legal pads, and clothing;
- repeatable revisions without character or environment drift.

### Candidate B — Hybrid live action

Use real actors in borrowed or inexpensive locations, with real AirPods and devices as part of the performance. AI systems provide generated prompt feeds and support VFX/UI overlays.

This is the preferred fallback if pure AI video produces uncanny identity shifts or cannot preserve comedic timing. It is also thematically strong: actors physically perform the same mediated relationship as the fictional protagonists.

### Evaluation principle

Do not choose a modality based on novelty. Test both against the same short scenes, score the results, and select the approach that best preserves performance, continuity, editability, cost, and production repeatability.

A full test matrix and scoring rubric remain a Phase 1/2 deliverable; the architecture-expander prompt defines this as Module A.

## 5. Proposed repository map

The source's seed layout is:

- canon — world rules, character invariants, and franchise structure.
- meta_prompts — reusable prompts for expanding workflows and producing task capsules.
- films — outlines, beat sheets, and eventually Fountain scripts for Films 1–6.
- production — modality evaluation, camera/blocking prompts, acting packets, and audio pipelines.
- tasks — small claimable tasks for Tier 1 and Tier 2 contributors.
- CREDITS.json — append-only contribution provenance.

Additional directories may be introduced as the project matures, but avoid duplicating the canonical source of truth.

## 6. Quality and governance gates

Every submitted artifact should be checked for:

1. **Canon compliance:** protagonists remain underqualified; AI systems supply the expertise.
2. **Structural compliance:** film-specific pacing and False Third Act rules are preserved where applicable.
3. **Continuity:** names, locations, runtime, chronology, and recurring props are consistent.
4. **Originality:** avoid copying protected dialogue, lyrics, or existing scripts. The source's music gag should be rewritten as original dialogue rather than quoting a song.
5. **Production utility:** scene directions and prompts must be specific enough to execute.
6. **Provenance:** source-derived material, new proposals, and model-generated additions are distinguishable.
7. **Human review:** no automated score should substitute for editorial approval.

## 7. Rollout order

1. Stabilize the franchise bible and six-film arc.
2. Produce the 40-beat Film 1 outline.
3. Create corresponding tasks for Films 2–6.
4. Benchmark live-action and pure AI-video modalities.
5. Decompose approved outlines into scene capsules.
6. Convert approved scenes into Fountain scripts and production packets.
7. Build animatics and short-form tests.
8. Decide the feature production modality using measured results.
9. Assemble Film 1 before scaling the pipeline to the remaining films.

## 8. Attribution model

The source proposes Tier 1 web-paste contributions at 1.0× and Tier 2 automated PR contributions at 3.0×.

A robust implementation should record contributor identity/handle, task ID, artifact path, pull request or issue reference, review status, contribution type, accepted credit weight, reviewer, and timestamp.

Credits should be append-only in spirit, auditable, and corrected through explicit adjustment entries rather than rewriting history. The source names CREDITS.json but does not define its schema; that schema remains future work.
