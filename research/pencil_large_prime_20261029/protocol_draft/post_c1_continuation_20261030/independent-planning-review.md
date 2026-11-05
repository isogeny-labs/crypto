# Independent read-only review: post-C1 seven-hour planning envelope

Scope: source/document review only, not launch review or execution authorization. No repository/cloud mutation, solver, trial, benchmark, scientific replay or new measurement performed.

## Bound inputs

Repository `/home/ec2-user/crypto-pencil-signed-graph-20261029`:

- `research/pencil_large_prime_20261029/protocol_draft/post_c1_continuation_20261030/planning-envelope.json`, SHA256201df07743540ac247e2ae0ec79c82c7b284aa16dd07d468924c87cf2a4b7a2b.
- Adjacent PLAN.md, SHA256f96fd643684f260b1ddee4509087f0f32a8154ab30ff8c234f140ae88fb03137.
- Bound terminal-g18 observation.json, SHA25689ed5c95d28c8981bace4a76f3839cad2c821d2a5de3755a8ce7b996af6f84f3; digest matches envelope.
- Existing full_cohort_continuation_20261030/PLAN.md read for unchanged scientific design.
- Native solver SHA256e71e9cf9b356db1f16781f394994b0ca13abbb05b4a7db82bfd2a6925a0ef4df and current runner SHA256838ca5bc8b12243b4c72adf2a09a122b39ee1a752cd4c762c800a34b7d92698d inspected for the correction opportunity below. Current runner inspection is source evidence, not a claim of new instrument deployment.

## Assessment

No concrete planning-scope defect found. Seven hours equals25200seconds;15:08:40Z to22:08:40Z is seven hours. The expiry is expressly an executor interpretation of a duration-only response, not a user-supplied absolute date. Review time does not reset it. Resource ceilings remain bounds, not measured runtime, feasibility evidence or spend estimates. Prior authorizations/costs are preserved; execution_admitted, protocol_frozen, review_complete and full_cohort_admitted remain false.

Full scope is preserved:21models x60targets=1260;3arms x6streams x1260=22680base rows;5measured pairs x1260=6300pairs; independent replay is additional charged work. All36pricing cells, cold phases, controls, failures and missing outcomes remain required. The plan does not shrink to a screening cohort or grant another same-instrument retry.

The observed-basis count agrees with the bound g18 observation:7records,3warmups,4measured,2pencil timeouts and zero pencil solve. Eleven of18slots remain missing. Therefore no completed candidate/reference ratio follows. The observation reports g19 CRIU dump failure; it does not establish OOM. The plan correctly treats watchdog timeout and checkpoint failure as different issues and makes no performance promise for the early-rejection fork.

The earlier full-cohort PLAN contains historical resource fields marked not supplied and older gate references. Its scientific scope can remain authoritative, but a future frozen contract must explicitly bind this new envelope and current known-data lineage rather than inherit stale deadline/budget text. That is a prospective binding task, not an error in this clearly unfrozen planning document.

## Concrete correction possible without a new scientific trial

The retained timeout records show `watchdog expired` without a parsed late child result. The current runner's `make_record` logic at runner.rs899-918 deliberately refuses an accepted report when timed_out/interrupted; if a complete late ChildOutput exists, it retains that message only in the error diagnostic. This is a correct fail-closed distinction and must not be weakened to recover a final price from a timeout.

A useful prospective implementation correction is a separately identified **partial-progress diagnostic contract** for the candidate. At completed stage boundaries, retain counters and progress independently from the final ChildOutput, with exact source/binary/workload/seed identity, a monotonic sequence and an explicit partial/censored flag. Useful source boundaries are native_solver.rs1255 rational-point setup,1326 sieve completion, candidate verification and graph relation processing around1330-1360. Do not emit from unsafe signal handlers or reinterpret partial counts as completed cost. Account for observation overhead separately and preserve the algorithm's RNG/iteration order. A software-only fixture can check monotonic snapshots, truncation, interrupted writes, counter disjointness and that timeout outcomes still have null accepted result/ratio. Existing records cannot be retroactively reconstructed from this proposed interface.

This is an evidence-quality correction, not a faster algorithm or proof of where the time went. The static source contains a full-field scan and bucket allocation per sieve (native_solver.rs850ff) repeated for up to16walks (1311ff); graph elimination is another possible cost. Without the absent phase observations, ranking those as the measured bottleneck would be unsupported. The early-rejection optimization's bound of32skipped complements cannot establish that it resolves the timeout. It may be retained as a distinct software-checked candidate, but selection needs a stated mechanism and prospective review rather than an assumed speedup.

For infrastructure, inspect retained CRIU stderr and exact supervisor/controller termination receipts for g19. Do not rerun the scientific workload merely to diagnose the dump and do not infer OOM from exit137. A narrower controller/checkpoint correction, if justified, must bind its changed artifact and preserve scientific input and cost provenance.

## Required next deliverable

Before any execution proposal, provide one source-bound diagnosis/selected-change document: distinguish observed evidence, static hypotheses, proposed correction, software validation, expected accounting differences, and unresolved uncertainty. Then independently review the known-data amendment, current source/binary/image, resource enforcement and complete gate/replay/cohort conditions. This review supplies none of those launch decisions. It does not waive the failed-gate stopping rule or authorize new trials.
