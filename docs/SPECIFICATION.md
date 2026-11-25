# Meatware: Experimental Specification

**Status:** Authoritative project specification  
**Version:** 1.0

## 1. Purpose and research question

Meatware is an abstract computational laboratory investigating the emergence of evolution and cumulative adaptation.

> Under what conditions can interacting, persistent and variable configurations give rise to heritable change, differential propagation, selection and cumulative adaptation, without those phenomena being explicitly programmed into the substrate?

This is not an artificial-life game made from renamed biological parts. The underlying model must not know what an organism, cell, gene, species, metabolism, membrane or ecosystem is. Such concepts may be interpretations of observed patterns, not privileged simulation components.

The primary objective is understanding the emergence of evolution and cumulative adaptation. Complexity, novelty, diversity and persistence are important secondary subjects, not synonyms for evolutionary progress.

## 2. Scientific objectives

Investigate spontaneous formation of persistent configurations; repeatable transfer of state or organisation; heritable variation; differential propagation and persistence; selection arising from general interactions; accumulation of inherited changes that produce measurable environmental advantages; new organisational regimes; and conditions under which complexity increases, decreases, collapses or provides no advantage.

These are research targets, not a prescribed sequence. The experiment must not assume they will occur.

## 3. Abstract substrate

### 3.1 Entities and state

The substrate is a minimal extensible system of entities and relationships. Entities have discrete identity and may carry continuous numerical state, arbitrary properties, state vectors, spatial coordinates and relational properties. State changes through general interactions, internal transformations and environmental influence.

### 3.2 Relationships and composition

Relationships may be explicit, spatially derived or persistent independently of spatial proximity. They may be created, modified or destroyed through general rules. Relationships may themselves participate in recursive organisation.

No fixed hierarchy is imposed. Configurations may aggregate, fragment, duplicate, fuse, exchange components, reconfigure, nest or form distributed arrangements. Composite configurations may acquire properties absent from their components.

Persistence, boundaries, internal organisation and functional behaviour must emerge from substrate mechanisms. There is no privileged `reproduce()`, `become_organism()` or `evolve()` operation.

Properties and state must be extensible without requiring a new hardcoded entity class for every observed phenomenon. Practical computational limits are configurable, but there is no conceptual ceiling or fixed taxonomy of complexity.

## 4. Interaction mechanics

The architecture supports multiple interchangeable experimental substrates, not one assumed universal physical model.

Properties may include numerical values, polarity, compatibility, stability, energy-like quantities, information-bearing state, spatial characteristics and relational characteristics. Transformations arise from direct interactions, internal state changes, environmental influence and changes in relationships. General rules should permit unforeseen combinations.

### 4.1 Resources and energy

Support configurable forms and accounting mechanisms for resources, transfer, gradients, dissipation, storage, conversion, accumulation, waste and byproduct reuse.

The initial experimental configuration uses continuously replenished resources, with configurable rates and spatial distributions. Energy conservation and other physical constraints are selectable model properties, not universal assumptions across all substrates.

### 4.2 Space, networks and transport

The initial spatial model combines two-dimensional physical space with abstract networks independent of physical distance. Support local and long-range interactions, diffusion, transport, flow, migration and dynamic relationships.

### 4.3 Temperature and stochasticity

Temperature mechanisms may include local and global values, gradients, heat transfer, dissipation and temperature-dependent interaction probabilities.

Randomness may enter through initial conditions, interaction outcomes, transformations, environmental fluctuations and thermal processes. Deterministic and stochastic configurations must be comparable.

## 5. Information, memory and inheritance

Information is an explicit mathematical concept, but information storage or processing is not assumed to exist in every configuration.

Investigate information-bearing structures, repeatable state transfer, internal representations, information processing and adaptive responses. Do not grant all configurations brain-like capabilities.

Memory is any persistent state that influences future behaviour.

Inheritance is any repeatable transfer of state or organisation between configurations. It may transfer state, structural arrangements, relationships, persistent patterns or other reproducible properties. No genetic code or predefined copying mechanism is required.

Variation may arise through imperfect transfer, duplication, fusion, fragmentation, component exchange, environmental influence, internal transformations and relational reorganisation.

## 6. Evolution and cumulative adaptation

Evolution is evidenced by heritable variation combined with differential propagation or persistence, producing changes in the distribution of inherited configurations over time. Selection arises through any interaction that causes variants to differ in propagation or persistence.

There is **no universal fitness score**. Measure observable, context-dependent outcomes such as persistence duration, resource acquisition, stability under disturbance, propagation frequency, structural retention and environmental performance. These are measurements, not universal rankings.

Cumulative adaptation is the accumulation of inherited changes that produce measurable improvements in environmental performance. Evidence must establish that changes are heritable, repeatable, causally connected to prior changes, maintained across successive transitions and meaningful under controlled environmental comparisons. A later configuration merely performing better is insufficient.

Complexity is neither a prerequisite nor a guarantee of adaptation. Simpler configurations may outperform more complex ones. Evolution-like processes may occur at different organisational levels; the relevant unit of selection must not be assumed.

## 7. Environment and initial conditions

The environment is an active experimental component. It supports resource replenishment, spatial gradients, variable conditions, local and global disturbances, recurring cycles, gradual drift, abrupt changes, configurable physical constraints and recorded interventions.

Environmental regimes range from static to highly dynamic or chaotic. Worlds may run indefinitely subject to computational resources.

Compare three initial-condition families:
- Unstructured random abstract components.
- Deliberately structured components and relationships.
- Replicator-seeded configurations.

Replicator-seeded worlds are comparisons, not an assumed prerequisite. Compare families under controlled conditions and multiple random seeds. No starting family is presumed to be the correct route to evolution.

## 8. Measurement and scientific validity

Measure heritable variation, transfer fidelity and diversity, differential propagation, persistence, environmental performance, accumulation of inherited changes, long-term adaptation, extinction and re-emergence.

Treat complexity as multidimensional: structural, relational, behavioural, information-bearing, dynamical, organisational, causal and diversity/novelty dimensions. Do not collapse these into a single authoritative score. Complexity may increase, decrease, fluctuate or collapse.

Use independent novelty detectors for unseen structures, behaviours, interaction patterns, organisational regimes, causal relationships and novel combinations. Novelty detection is observational and must not alter the simulation. Preserve detections for human review and false-positive analysis.

The initial control strategy uses matched worlds with identical conditions and different random seeds. Later controls may include ablation and environmental interventions.

The primary negative result is absence of persistent configurations under all tested conditions. Distinguish this from insufficient duration, inadequate resolution, poor parameters, instrumentation failures or unexplored initial conditions. Preserve negative results and anomalies.

## 9. Computational architecture

Separate four concerns:
- **Substrate:** entities, properties, state and relationships.
- **Dynamics:** interactions, transformations, scheduling and environmental processes.
- **Analysis:** measurements, hypotheses, causal analysis and comparisons.
- **Visualisation:** observation, inspection, dashboards and export.

Substrate and dynamics must not depend on biological classifications or visualisation. Analysis and visualisation observe without changing simulation state.

The preferred representation is dynamic graphs and networks with extensible entity state and recursive relationships. Support hierarchical interaction ranges, spatial locality, variable numerical precision, approximation of distant or low-activity processes, multiple resolutions with explicit approximation rules, hybrid timescale scheduling and causal consistency.

Compare multiple execution models for reproducibility and equivalence. Execution may be asynchronous, event-driven, parallel or sequential where appropriate.

Investigate adaptive computational organisation separately. Do not assume configurations can alter their own effective computational organisation.

Optimise measurable trade-offs among entity count, interaction count, simulated duration, local fidelity and independent trial throughput. Computational limits and approximation settings must be recorded.

Visual similarity is an initial screening signal only, not proof of scientific equivalence. Optimisations require statistical and dynamical comparisons against a reference implementation.

## 10. Observation, persistence and reproducibility

Expose comprehensive read-only inspection and export of entities, relationships, recursive structures, state histories, interactions, environmental conditions, transformations, causal histories and statistical summaries.

Use snapshots plus deterministic replay. Preserve world snapshots, experiment parameters, rule/configuration versions, random seeds, execution settings, approximation settings, precision and interventions.

Baseline experiments are immutable. Branches may change rules, parameters, initial conditions or environmental conditions without corrupting baseline runs.

## 11. First experimental programme

Prioritise cumulative adaptation without assuming a fixed order:
1. Persistent configurations.
2. Repeatable transfer of state or organisation.
3. Heritable variation.
4. Differential propagation.
5. Cumulative adaptation.

Compare unstructured, structured and replicator-seeded initial conditions with multiple random seeds, matched conditions and explicit controls.

## 12. Interface

The first version prioritises a high-performance engine and minimal visualisation. Expose a simple abstract-world view, entity and relationship inspection, experimental conditions, simulation time, execution state, performance/approximation indicators, measurements, snapshots, replay and experiment comparison.

Visual appearance is a secondary representation, not the definition of the experiment.

## 13. Non-negotiable invariants

1. No biological primitives in the substrate.
2. No hardcoded evolutionary outcomes.
3. No privileged reproduction operation.
4. No universal fitness metric.
5. No complexity-as-progress assumption.
6. No observer intervention in simulation state.
7. No fixed organisational hierarchy.
8. No irreversible baseline changes.
9. No unmeasured optimisation.
10. No unexplained important result; preserve causal traceability.
11. No premature framework bloat.
12. No promise that emergence is guaranteed.

## 14. Acceptance standard

The project is scientifically useful only if it can state model assumptions, reproduce runs, preserve negative results, trace important changes, compare controls, quantify approximation error and distinguish observed phenomena from analytical interpretations. A compelling visual result alone does not satisfy this standard.
