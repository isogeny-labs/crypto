# Fresh end-to-end checks of three ECDLP leads

This is a new execution, not a reconstruction of the unlocated historical runs. The complete claim-by-claim verdicts are in [CLAIMS.md](CLAIMS.md). Exact frozen observations are in [results/](results/), with source models, subgroup witnesses, input specs, sealed sessions and replay receipts.

The study covers 21 binary curves, 24 odd-characteristic coverage cases, and 18 PKM cells with 23 eligible primes each. Decomposition uses 3,520 target workloads. Genus-2 transfer uses two separate 1,260-target cohorts: one indexed by field size, and one by actual prime-subgroup size through 40 bits. Each genus target has four solver arms. Primary execution therefore comprises 23,952 solves, with 84 independent/unfiltered control solves and 168 supplementary trace-filter solves. Every completed answer is checked on its original elliptic curve; rejected fixtures and unsuccessful development attempts are preserved separately.

## What the costs mean

All figures below are cold one-target operation-count observations. Algorithms receive P, Q and r, never the planted scalar. Charge destination construction, both transfers, base construction/projection, all initialization, roots and rational lifts, relation collection, elimination, failed work, retries and source verification. Common fixture generation/order certification is separately recorded.

Odd quadratic arithmetic expands into F_p M/S/I operations. Binary arithmetic uses the specified ambient-field M/S/I unit. Twelve arithmetic prices use S/M={0,0.5,1}, I/M={1,10,100,1000}. The retained ecbench GAE conversion uses the separately declared S/M=0.75, I/M=10. Native charges {0,0.1,1} source-addition equivalents apply symmetrically to both arms. These logical native counters do not measure allocations, physical hash buckets or CPU instructions. L0 walls are descriptive; no hardware-speed or fastest-tool claim follows.

The matched references are memoized r-adding rho with and without negation. Their collision-table memory is reported alongside the IC graph. `rho_neg` is the predeclared main reference; `rho_plain` remains visible throughout.

## Genus-2 transfer

The implementation uses the published Scholten family and an explicit one-infinity quintic, `12Y²=12+100t+299t²+380t³+187t⁴+30t⁵`. Pullback plus conjugation defines tau. The dual norm checks `norm(tau(P))=2P`; addition, exceptional points, descent and odd-prime subgroup preservation are checked. See [TRANSFER.md](TRANSFER.md), the exact [polynomial certificate](certificates/cover-identity-v1.json) and [small zeta/order checks](certificates/small-independent-zeta-orders.json). The certificate is an integer polynomial witness plus the repository's identity checker, not a Lean proof of the geometric theorem.

The primary collector discards repeated split divisor classes up to sign. It restarts on repeated walk divisors and cannot recover a scalar from those collisions. A signed negative cycle only anchors a large-prime component; extraction waits for a useful target dependence. Initial scalar work, restarts, all root work and graph/scalar operations remain charged.

| Cohort / size label | Targets | IC/rho-neg, 12 arithmetic prices | Including native charge band | Declared default ratio | rho wins / 12 |
|:--|--:|--:|--:|--:|--:|
| field / 16 | 180 | 3.811–7.044 | 3.613–7.044 | 5.465 | 12 |
| field / 20 | 180 | 4.017–7.933 | 3.968–7.933 | 6.107 | 12 |
| field / 24 | 180 | 4.396–10.422 | 4.396–10.422 | 8.127 | 12 |
| field / 28 | 180 | 3.018–13.865 | 3.018–13.865 | 10.002 | 12 |
| field / 32 | 180 | 2.879–28.203 | 2.879–28.203 | 17.806 | 12 |
| field / 36 | 180 | 4.039–32.655 | 4.039–32.655 | 21.016 | 12 |
| field / 40 | 180 | 4.600–39.658 | 4.600–39.658 | 25.054 | 12 |
| subgroup / 16 | 180 | 3.398–7.779 | 3.398–7.779 | 5.990 | 12 |
| subgroup / 20 | 180 | 2.409–9.295 | 2.409–9.295 | 6.611 | 12 |
| subgroup / 24 | 180 | 1.928–12.555 | 1.928–12.555 | 8.521 | 12 |
| subgroup / 28 | 180 | 1.596–15.293 | 1.596–15.293 | 9.692 | 12 |
| subgroup / 32 | 180 | 1.466–14.861 | 1.466–14.861 | 9.249 | 12 |
| subgroup / 36 | 180 | 1.496–17.300 | 1.496–17.300 | 10.603 | 12 |
| subgroup / 40 | 180 | 1.421–18.362 | 1.421–18.362 | 11.091 | 12 |

The field cohort labels are approximate field-size levels; actual field and subgroup bits/cofactors are retained in the manifests. The order cohort requires exact subgroup bit lengths and cofactor at most 64. A 40-bit field is not a 40-bit prime-order subgroup. Tables report ratios of sums over matched targets, not a claim that every individual target has the same cost. Per-curve rows and all price cells remain in the machine-readable tables.

A generic full-degree Jacobian addition in the largest field fixture costs **70M+3S+I** in this backend; the matched source addition costs **9M+4S+I**, in base-field units. The supplied 41M+I Jacobian formula does not describe this backend; the source formula is reconciled below. Independent-trial and unfiltered controls, including all scalar-generation cost, are in [genus-controls.json](results/genus-controls.json). First graph cycles versus useful solve positions are in [genus-cycle-events.json](results/genus-cycle-events.json).

## Subfield and PKM decomposition

The binary linearized solver and odd S3 solver verify every rational lift by point addition. Factor-base logarithms are unknown variables; incremental elimination recovers the target. Cofactor offsets let relation collection visit the full relevant group, followed by projection into the prime subgroup. The corrected sampler tests initial/restarted targets and uses seeded counted jump choices; it does not assume a fixed point-partition map reaches a tiny sumset.

| Family | Curves / targets | Per-curve IC/rho-neg across all 12 arithmetic prices | With native charge band | Whole-oracle M / decomposition call |
|:--|--:|--:|--:|--:|
| Binary quadratic | 21 / 168 | 2.089–70.311 | 2.089–113.842 | 12.141–16.187 |
| Odd quadratic | 5 / 40 | 2.079–3.505 | 2.079–3.505 | 46.270–150.778 |
| Kummer/Dickson | 414 / 3312 | 4.263–1695.104 | 4.263–1695.104 | 0.000–937.312 |

Binary q=512 mean full-group coverage is **0.373152**, compatible with approach to 3/8 over this range. Every x in each tested subfield has absolute trace zero; all 21 doubled images equal the trace kernel. This proves the finite coset obstruction, not a universal 3/8 limit. Prime-subgroup coverage is separately retained; a trace filter rejects no target in an odd-order subgroup. The 4.7% number compares two whole-group hit probabilities, not all relation multiplicities or the subgroup attack yield.

The 18 PKM cells have fresh maximum |z| **4.255** under the predeclared paired-prime statistic, with maximum absolute structured-minus-control coverage **0.00008987**. Thus the supplied |z|≤1.84 bound is not reproduced. Each curve has 32 sign-orbit/torsion-matched random controls. All effects and bootstrap intervals remain visible; failure to reject zero is not a proof of random equivalence.

The complete oracle census checks **9,378,482 targets across 854 curve/base combinations**, with zero disagreements against direct point-pair sumsets. Cost comparisons include complete solving on fixed tower fibers and matched random bases. This measures the implemented two-summand solving route; it does not exhaust every possible multivariate or auxiliary-isogeny PKM solver.

## Reproduction and retained failures

Build the common engine with `cargo build --release --offline --bin ecbench`, and the standalone driver with `cargo build --release --offline --manifest-path research/three_leads_e2e_20261018/native/Cargo.toml`. The [protocol](PROTOCOL.md) and [order-cohort protocol](GENUS_ORDER_PROTOCOL.md) fix eligibility, seeds, caps and prices. `run-batch.sh` records each session; `verify --replay-all` re-executes every deterministic record. Archived sessions must be extracted to their original study-relative paths before replay.

The earlier v1 batches are diagnostic evidence, not silently discarded targets: unsplit-only genus walks and fixed-partition decomposition walks could exhaust. Their original implementations remain registered for exact replay. Corrected v2 cohorts use the same original models/targets/caps; the additional exact-order cohort is separately identified. The general-cubic coverage diagnostic and failed build/check attempts are retained as development evidence.

The report graph data are native Rust outputs. Python only renders the retained observations. All individual session reports and plots are indexed in the report bundle; the main plots and PDF are mirrored under `benchmarks/20261018-three-leads-e2e`. Canonical dashboard entries remain operation-model diagnostics outside fully priced speed rankings. No claim of fastest implementation or deployed-curve security is made.

## Supplemental checks and implementation cost

The 168 additional binary trace-filter solves all verify and stop at exactly the same relation/trial positions as their unfiltered counterparts. The filter skips 3,044 of 6,046 candidate relations (50.35%). It tests relation-walk points in the full group; it rejects no odd-prime-subgroup DLP target. Its mask costs n(n−1) squarings once, followed by four logical bit operations per finite candidate. At q=512 it reduces the arithmetic-model cost to 0.971–0.987 of the unfiltered route; the native sensitivity band reaches 1.004. This is inexpensive filtering, not free setup. See [trace-filter.json](results/trace-filter.json).

The source formula 9M+4S+I becomes exactly 12M+I at S/M=0.75. Thus the supplied source-addition formula is compatible with that convention; 41M+I does not describe this Jacobian backend.

In the PKM decomposition pipeline, 562 of 6,624 structured/random-base returns occurred before any decomposition, including 64 empty-base returns. Those are generic identity hits rather than relation-based IC successes. All costs include that behavior; the result does not certify a pure IC mechanism on every tiny target. See [decomposition-mechanisms.json](results/decomposition-mechanisms.json).

The supplementary uniform-random-x null (32 controls per curve, added after the conditional analysis) gives maximum absolute PKM z=2.867, compared with 4.255 for controls conditioned on actual point count. Neither reproduces 1.84. These nominal cell standardizations are not a multiplicity-adjusted universal significance claim.

At actual 40-bit subgroup size, complete instrumented solves retire 7.4868 times as many user-space instructions as negation rho (paired curve/target bootstrap 95% interval 6.6571–8.3716), or 5.7823 times plain rho (5.2151–6.4286). All 180 pairs use the same binary and valid unmultiplexed counters. These counts include native work and instrumentation overhead. A five-pair identical-rho A/A control has ratio range 0.9999703–1.0000121 and identical primitive/native counters. See [retired-instruction-summary.json](results/retired-instruction-summary.json) and [instruction-AA.json](results/instruction-AA.json). Both attempted L2 timing preflights rejected a busy host; no qualified wall-time result is claimed.

All 945 sealed study sessions pass their audits. Every one of 42,862 successful measured records replays identically. The 134 exhausted development records were separately rerun and reproduce their exhaustion and exact counters; the standard replay command skips unsuccessful records. The A/A session adds ten measured records and two warmups separately.

The complete [evidence and reproduction guide](EVIDENCE.md) documents the deterministic raw archive, exact replay commands, source snapshots and validation limits. Actual field/subgroup bit lengths and cofactors for all 42 genus models are in [genus-fixture-sizes.csv](results/genus-fixture-sizes.csv).

Retired instructions favor the tiny subfield pipeline in some cases despite its arithmetic-model loss: binary q=8 gives 0.4491× rho-neg, q=16 gives 0.4942×; odd p=5 gives 0.4363× and p=7 gives 0.4854×. All 18 structured PKM cell instruction ratios exceed one (1.928–8.190). These complete instrumented-backend observations are a separate unit; they establish no qualified wall-speed or fastest-implementation claim.
