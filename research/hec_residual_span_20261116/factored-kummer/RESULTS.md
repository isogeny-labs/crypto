# Factored Kummer same-chain result

The once-only arithmetic gate at7ee004a87fa exited0: all3168steps,1556signedrecoveries and52non-firstcharts pass, with exact repeated setup/primitive vectors. All input labels/charts/smoothness agree with retained generic run; all6336native-control or degree-two-source records agree exactly. All9504primitive and36setup records validate the REDC identity. This is local checked evidence, not a signed independent certificate. [pcc:unverified]

For ordinary chains, repetition0 sums:

|p|source|native REDC|old kernel|new kernel|old full|new full|native/new full|
|---|---:|---:|---:|---:|---:|---:|---:|
|101|1|29400|21158|12346|50830|42018|0.699700|
|251|1|29486|21398|13300|55108|47010|0.627228|
|601|1|60424|13288|10236|109924|106872|0.565387|

Point-source kernel reductions are1.714,1.609,1.298 respectively; full primitive reductions versus the old Kummer implementation are1.210,1.172,1.029. All degree-two-source results are unchanged, including their regressions relative to native. Full candidate REDC still exceeds native in every chain group. Setup now additionally includes source constants and retains generic fallback compilation; SETUP.tsv charges these separately. Edge controls are preserved, excluded prospectively from chain ratios.

The optimistic native-full/new-kernel REDC bound now reaches5.903, still below10. That comparison gives all candidate work except the kernel zero cost, deliberately mismatches boundaries, and is not a matched addition speedup. Recovery alone still cannot supply10x on this stream. Conversely the max-component native/full bound remains1.741–3.306 for any common nonnegative weighting of the16recorded components, with no positive-native/zero-candidate denominator. These incomplete nested counters are not a CPU/memory/wall bound. No arbitrary field/scalar exchange rate or complete-solve claim follows.

Decision: the factored circuit is correct on this panel and reduces its intended component, but fails the full primitive tenfold cost case. Do not launch a full collector around this unchanged circuit. A new relation-yield mechanism or substantially different arithmetic/recovery combination requires its own explicit cost case. Broad mathematical genus2/3/4 tenfold objective remains open. Classical factorization is not a priority claim.

Frozen source, dependencies, protocol, binary/input hashes precede execution. Raw vectors, stderr, exit and original run commit remain in Git. summarize.rs reuses retained old records and never reruns the old producer. No S3/cloud action. Generic fallback and original models/sources remain intact.
