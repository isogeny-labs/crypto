# Audit scope

The reader uses ordinary-residue affine arithmetic with bounded u128 products,
separate from APS's Montgomery/Jacobian producer. That separation is a software
property, not a signed PCC verdict [pcc:unverified].

| Obligation | Reader behavior | Remaining boundary |
| --- | --- | --- |
| Export shape | Full Draft 2020-12 canonical schema | No experiment-design admission |
| Protocol | Schema, pre-data freeze flag and hash; plan and analysis pins | No semantic proof of randomization, power, controls or pre-data filing |
| Input custody | Relative bounded files and digest/size pins | Host execution and external persistence are not attested |
| Raw record identity | Literal compact JSON seal, no reserialization | An unkeyed seal does not attest execution |
| Planned population | Exact sequence/arm/workload/seed/round/warmup binding | Missing slots stay separate; complete checking refuses them |
| Attempt history | Explicit process-attempt/restore counts and checkpoint metadata | Entropy, discarded sampling draws and actual scheduling are not attested |
| Outcome collection | Every recorded bounded failure and explicit recordless infra event retained | Missing history is rejected |
| Base | Canonical points, partitions and every uG+vQ label | Does not replay the producer's sampler |
| Curve | Explicit prime fields up to 65,521; prime subgroup and generator/target membership | Does not independently count the full curve order or register its identity |
| Six encodings | Distinct coordinate membership for the exact requested partition/block geometry | Unsupported or oversized representations are refused |
| Cubic pruning | Nontrivial cubic map, subgroup eigenvalue on G/Q, closed parts and propagated labels | No unproved orbit quotient or sixfold speed factor |
| Accepted relation | Separate signed point sum, A/B, nonzero B and scalar check | No claim about how the producer discovered it |
| Complete root sets | Finite point combinations equal reported sets when `complete` is true | Cumulative record budget is 1,000,000 sign candidates; larger audits refuse |
| Partial/late output | Partial sets checked as subsets; late witnesses remain outside timed success | No completeness assertion for early exit or timeouts |
| Literal equations | Field and canonical equation digest binding | Does not reconstruct every polynomial identity or attest solver execution |
| Metrics | Six exclusive phase sums, five online phases, enclosing interval and partial diagnostic aggregates | Consistent supplied numbers do not prove measured clocks or operation counts |
| Costs | Exact native counter tokens retained, including unpriced counters | Full operation accounting/calibration remains separate |
| Qualification | All clocks remain invalid and all execution/cost attestations false | L2/A-A, independent review and signed evidence are later gates |

The full oracle is run only for attempts claiming completeness. It enumerates
coordinate combinations and global-sign classes directly, without F4,
summation polynomials, factoring or producer recovery code. A refusal at the
work cap is an audit limit, not evidence that a root set is complete.

The public producer bridge exercises the same reader files against actual
`methods::solve` output. The standalone package tests cover source-file and
CLI reconstruction and deliberate corruptions. Development assertions do not
acquire execution standing merely because a reader agrees [pcc:unverified].

For v3 the reader additionally checks explicit unequal profiles, chart and
guessed-coordinate cell geometry, retained points/labels, cumulative draw
totals and complete-old-region authorization. `retained_full` never subtracts
an old region. Trace histories must join across attempts and their call/work
totals match the emitted diagnostics. The numerical truth of diagnostic
counters is still not execution-attested.

Optional inclusive Ir packets bind every raw profile part to an unprofiled
record, separately sealed trajectory-matching replay and separate scalar
check. A frozen protocol can require those packets; missing packets and
recordless failures remain represented and prevent qualification. See
[ACCOUNTING.md](../../research/aps_cost_leads_20261106/ACCOUNTING.md) for
the exact interval, cumulative input caps, missing/restore rules and excluded
work. The reader does not launch a profiler, invent GAE prices or validate
actual thread coverage, wall isolation or experiment design.
