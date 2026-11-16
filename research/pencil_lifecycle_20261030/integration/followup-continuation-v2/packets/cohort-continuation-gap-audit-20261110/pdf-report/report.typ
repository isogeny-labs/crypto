#set page(paper:"a4",margin:16mm,numbering:"1")
#set text(size:10pt)
= Full 1,260-target continuation: source gap map
#text("[pcc:unverified] Source audit dated2026-11-10; derived report T-2100. Repository revision 36a0004f7857e06f5e3a8357819986ad1a88621d. No targets, solvers, tests, compilers, materializers or cloud jobs ran for this audit or report.")

#image("gap-map.svg",width:100%)
== Fixed denominator and null results
#text("The retained protocol specifies7 exact-order levels ×3 models ×60 targets =1,260targets; pencil, rho and rho_AA arms;5 measured streams and1 warmup per target/arm. Retained counts are22,680execution slots:18,900measured and3,780warmups, with6,300candidate/reference pairs and36pricing cells. These are protocol counts copied from the source audit, not new executions.")

#text("All current complete candidate/rho cost ratios, wall ratios and natural duplicate yield remain null where evidence is missing. Missing or ineligible pairs keep the full affected ratio null rather than shrinking its denominator. Historical543 solved workload keys,1cap-only and716absent keys belong to earlier variants; their union with later rows is not this cohort.")

#pagebreak()
= Implemented components and next code gates
#text("C1 — exact full cohort plan and controls. Recorded status: implemented_preparation_no_solver_execution. Sources: research/pencil_large_prime_20261029/prepared_specs/manifest.json, research/pencil_large_prime_20261029/prepared_native_plans/result.json")

#text("C2 — fail closed full1260 aggregation. Recorded status: implemented. Sources: research/pencil_large_prime_20261029/native_price/src/aggregate.rs")

#text("C3 — exact 36 cell finite price mapping. Recorded status: implemented_requires_final_source_bound_review. Sources: research/pencil_large_prime_20261029/native_price/src/analyzer.rs, research/pencil_large_prime_20261029/native_price/src/importer.rs")

#text("C4 — pencil relation trace and offline replay components. Recorded status: implemented_for_unmanaged_donor_path_not_composed_with_managed_progress. Sources: research/pencil_large_prime_20261029/src/relation_stream.rs, research/pencil_lifecycle_20261030/integration/followup-continuation-v2/target-stream/src/lib.rs, research/pencil_lifecycle_20261030/integration/followup-continuation-v2/relation-adapter/src/lib.rs")

== I1 unify trace and managed progress solver
#text("Recorded status: missing. managed progress selects a separate copied solver with no ordered relation trace. Required outcome: one shared algorithmic core; no-sink, progress-only, trace-only, and combined modes preserve arithmetic, ordering, terminal result, and independently checked source identity.")

== I2 retained native session exporter
#text("Recorded status: missing. host, plan, preparation, child and owner files exist but are not joined into a checked retained actual-session export. Required outcome: hash-bound FilePins and postrun session report, separate external custody, no invented unknown fields, one attempt identity per expected slot.")

== I3 production target local ordered relation stream
#text("Recorded status: partial_uncomposed. actual managed collector streams checked equals zero. Required outcome: atomic target-local stream from actual managed path with exact shared rows, admissions, rejected/capped/pending/terminal dispositions and external identity custody.")

== I4 offline rank collector derivative
#text("Recorded status: partial_component_only. offline rank and sign adapter exist but are not bound to actual full-cohort streams or independent scalar replay. Required outcome: generated and incorporated prefix ranks, consistency, target identifiability, pending state, native pivot agreement, work/memory, and group verification joined for every attempt.")

#pagebreak()
= Remaining accounting and altered collectors
== I5 final source bound complete finite cost export
#text("Recorded status: partial_mapping_complete_final_binary_binding_missing. root report declares incomplete cost, group-only lower-bound GAE, unpriced work and null candidate/rho ratio. Required outcome: exact final binary/source/protocol coverage of all nine categories and calibration; every unconverted resource retained separately; no uncovered primitive silently free.")

== I7 terminal process and attempt join
#text("Recorded status: missing_complete_join. normal solver terminal, trace failure, timeout, controller history, resource charge and retry identity are not one retained record. Required outcome: durable partial evidence plus explicit terminal absence for failures, all attempt identities and costs retained, no automatic scientific retry or zero-cost timeout.")

== I6 matched clock endpoint
#text("Recorded status: missing_if_wall_cost_is_required. candidate/rho phase and cold endpoint mismatch; old calls below threshold; wall validity false. Required outcome: prospectively reviewed common cold endpoint, method-specific online mapping, fresh call state, A/A, observer policy, and fixed block estimand if needed.")

== K1 signed duplicate label integration
#text("Recorded status: fixed_and_archive_components_only. replace production target-local dedup set with reviewed first-label map at actual accepted signed-divisor gate; preserve graph behavior; verify candidate; charge complete work. A distinct method identity and full1,260target cohort are required for this changed method.")

== K2 degree one signed recovery and collector rule
#text("Recorded status: fixed_component_passed_not_production. integrate boundary recovery, close caller premises and define justified degree-one graph equation so recovered divisors are not dropped. A distinct method identity and full1,260target cohort are required for this changed method.")

== K3 signed kummer ordered stream
#text("Recorded status: missing_production_adapter. hook before dedup and around SignedGraph edge with shared equation assembly, labels, all bypass dispositions and signed node0 offline replay. A distinct method identity and full1,260target cohort are required for this changed method.")

#pagebreak()
= Retained component observations and publication gates
#text("K1 historical archive:420 dispositions,218ineligible,202compared,174first and28redundant; informative natural duplicates0 in that archive, natural yield null, complete cost ratio null. K2 isolated fixed component:84/84 exact recoveries,4new degree-one recoveries, production admission false and cost ratio null. These are copied historical observations, not fresh runs or population estimates.")

#text("1. freeze and publish exact source binary full1260 protocol plans seeds mapper launch command resources deadline retry and exporter identities")

#text("2. complete separate 18-slot three-arm gate and fresh independent replay of all 15 measured executions")

#text("3. run all 1260 targets and 22680 primary slots for the selected final method")

#text("4. retain matched rho and rho_AA, every failure cap missing slot attempt resource charge and fresh replay")

#text("5. recompute 6300 measured pairs in every one of 36 cells; incomplete denominator keeps ratio null")

#text("6. publish standardized rows cost grid resources replay validator graphs neutral PDF checksums and artifact pointers")

== Reading and provenance
#text("Read this report alongside ../GAP_MAP.md and ../gap-map.json. The original report contains exact source paths, line references, caveats and literature pointers. source-bindings.json pins all four originals. The copied editable gap-map.svg is byte-identical to the original; no original file changed.")

#text("The source audit used retained protocols, exact code and prior reviews because the shared research KB was unavailable. Retained literature pointers: Handbook of Applied Cryptography chapter3 Algorithm3.60; Chung–Costello–Smith signed recovery sections2,6,7. This rendering adds no literature search, mathematical result, measurement or independent validation.")

#text("Recorded code order: I1 → I2 → I3 → I4 → I5 → I7 → I6 → gate18 → full1260 → publication. This is the source audit’s proposed ordering, not an execution release. Standardized gaps and null ratios are in gates.csv and report.json. Unknown costs are not zero.")

