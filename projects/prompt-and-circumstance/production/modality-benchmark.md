# Production Modality Benchmark

**Status:** Proposed evaluation protocol; no modality decision has been made.  
**Decision under test:** Pure AI-generated video versus hybrid live action with AI-generated prompt feeds and VFX/UI overlays.

## 1. Decision principle

Choose the production method that best preserves the franchise's central performance mechanism: four underqualified humans deliver AI-supplied language with misplaced confidence while the audience sees the gap between their words, understanding, and the systems' actions.

Do not select a method because it is novel, fashionable, or technically impressive. Evaluate both methods against the same scene briefs, target duration, and editorial requirements.

## 2. Test package

Produce the same five scene tests in both modalities:

| Test | Scene | What it tests |
|---|---|---|
| A | Boardroom bluff, Film 1 opening | Deadpan delivery, multi-person timing, legal jargon, reaction shots |
| B | Dave's Faraday deposition | One performer sustaining a straight face while receiving dense prompts; dialogue attribution |
| C | Penthouse midpoint reveal | Four-character continuity, device UI, overlapping reactions, tonal pivot |
| D | Jungle-juice server annex accident | Physical comedy, prop continuity, environmental effects, cause/effect clarity |
| E | Davos beer-pong negotiation | Group blocking, comic escalation, recurring wardrobe, performance timing |

Each test should be 45–90 seconds, with a shared beat sheet, dialogue intent, shot list, and sound brief. Dialogue may be newly written for the test; do not reproduce protected film dialogue or song lyrics.

## 3. Controlled inputs

For each modality:
- Use identical character descriptions, wardrobe references, location requirements, scene duration, and performance objective.
- Record model/tool versions, generation settings, prompt revisions, actor/voice assets, and post-production hours.
- Give each team the same time budget and number of revision rounds.
- Preserve source footage/generation outputs and edit decision records.
- Make one “first pass” version and one “best revised” version. Do not compare a polished hybrid cut to an unedited AI generation.

## 4. Evaluation dimensions

Score each dimension from 1 (unusable) to 5 (production-ready). Evaluators should score independently before discussing results.

| Dimension | Weight | 1 — Failure | 3 — Usable with work | 5 — Strong |
|---|---:|---|---|---|
| Character identity continuity | 15 | Faces/body identity drift | Recognizable with corrections | Stable across shots and revisions |
| Wardrobe/prop continuity | 10 | Clothing/devices change randomly | Minor repair needed | Reliable continuity |
| Dialogue attribution and lip-sync | 15 | Wrong speaker or unusable sync | Occasional correction | Clear, stable speaker ownership |
| Multi-actor timing | 15 | Reactions overlap or timing collapses | Edit can rescue most beats | Timing supports jokes naturally |
| Deadpan performance | 15 | Expression undermines joke | Some takes land | Controlled, repeatable comic restraint |
| Blocking and spatial continuity | 10 | Geography incoherent | Mostly understandable | Consistent and editable |
| Revision repeatability | 10 | Fixes break other shots | Some stable iteration | Targeted changes preserve everything else |
| Editability and sound integration | 5 | Cannot build a coherent scene | Workable with compromises | Clean handles, coverage, usable audio |
| Cost and throughput | 5 | Exceeds feasible budget/time | Borderline | Repeatable within declared budget |

Weighted score = sum(score ÷ 5 × dimension weight). Maximum 100.

## 5. Hard gates

A modality cannot be selected for feature production unless it:
- Scores at least 70/100 overall.
- Scores at least 3/5 for identity continuity, dialogue attribution, multi-actor timing, and deadpan performance.
- Delivers at least four of the five test scenes without a fundamental continuity failure.
- Can reproduce a corrected shot or beat without destabilizing unrelated material.
- Has a documented, repeatable workflow and an honest cost estimate.

A high average cannot conceal failure in a critical dimension.

## 6. Cost and throughput record

For each test record:
- Setup time.
- Prompting / generation time.
- Actor, location, equipment, or compute costs.
- Number of unusable takes / generations.
- Editing, compositing, cleanup, and sound hours.
- Time to implement one requested revision.
- Total elapsed time from brief to approved cut.
- Estimated cost per finished minute, with assumptions stated.

Do not compare token or generation costs alone. Human correction and editorial labour count.

## 7. Hybrid live-action protocol

Use a small cast, borrowed office / conference-room locations, practical phones and AirPods, and simple wardrobe. Actors should perform the human layer rather than pretending to understand the technical content. Add generated prompt feeds, screen inserts, and VFX/UI overlays in post.

Test whether real timing, eye-lines, physical comedy, and group chemistry outweigh location and compositing costs.

## 8. Pure AI-video protocol

Use consistent character reference assets and controlled shot prompts. Test identity persistence across separate sessions, dialogue ownership, two-shot / group coverage, prop continuity, and revision locality. Record any manual frame repair, voice replacement, or compositing required.

Do not hide failures by selecting only disconnected hero shots. The output must cut into a coherent scene.

## 9. Decision procedure

1. Freeze the five scene briefs and acceptance criteria.
2. Produce both modalities under equal constraints.
3. Collect blind scores from at least three reviewers where practical.
4. Publish raw scores, costs, sample cuts, and failure notes.
5. Apply hard gates.
6. If only one modality passes, select it for the next pilot.
7. If both pass, choose based on weighted score, repeatability, and cost per approved minute.
8. If neither passes, run a narrower hybrid-first pilot and identify the limiting capability before spending on feature assembly.

## 10. Deliverables

- Five paired scene tests.
- Prompt / shot / wardrobe packets.
- Score sheets and reviewer notes.
- Cost and time ledger.
- Revision log.
- Decision memo with recommendation and unresolved risks.

## 11. Evaluation sheet template

Copy for each reviewer:

| Dimension | Weight | Score (1–5) | Evidence / timestamp |
|---|---:|---:|---|
| Character identity continuity | 15 |  |  |
| Wardrobe/prop continuity | 10 |  |  |
| Dialogue attribution and lip-sync | 15 |  |  |
| Multi-actor timing | 15 |  |  |
| Deadpan performance | 15 |  |  |
| Blocking and spatial continuity | 10 |  |  |
| Revision repeatability | 10 |  |  |
| Editability and sound integration | 5 |  |  |
| Cost and throughput | 5 |  |  |

**Decision:** Pass / Fail / Retest  
**Critical failure:**  
**Best-performing scene:**  
**Revision that most improved the result:**  
**Reviewer confidence:** Low / Medium / High
