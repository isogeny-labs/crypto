# Claim-to-evidence map

This continuation claims implementation and finite producer checks. It does
not claim a new geometric theorem or compiled Lean proof. PCC-20261115-30dcbf
is unsigned. [pcc:unverified]

| Statement/domain | Evidence | Standing/remaining obligation |
|---|---|---|
| Mixed map, order-three eigenline and pullback identities over actual P-256 | Prior `../transfer_search/PROOF_MAP.md`, audit and identity receipts | Prior producer evidence; not re-proved here |
| Canonical representative coefficient transport in the fixed cyclic subgroup | `normalize`, `advance`, canonical multiplier negative control and actual-field trace | Finite checks only; no general group-mechanization proof |
| Collision scalar formula with nonzero denominator in a prime-order subgroup | `recover`, public expected-answer replay, zero-denominator control | Caller prime-order/membership hypotheses required |
| Bounded short-cycle handler is exercised | `toy_solves` requires a nonzero destination escape counter in frozen panel | Does not prove mixing or handle all long cycles |
| Expected toy answers in three specified subgroups | `results/toy.json`; `correctness_report`; expected scalar held outside solver | 54 paired deterministic cases including zero; no performance inference |
| Actual-field step invariant | `full_field_trace`, `results/trace.json` | 32 transitions each, public target 37G; no unknown logarithm recovery |
| External source adapter agrees on specified coordinates | `external_source_control` | Four fixed scalars; not strongest-solver certification |
| Runtime advantage or asymptotic improvement | No admitted evidence | Open; neither claimed nor certified |

Generic group identities are explained in the report with their hypotheses.
The reused Cantor/map routines remain part of the trusted producer code.
Lean/Lake/Mathlib and lean4checker are absent in this workspace; no existing
source-only proof is promoted to a compiled proof. An identity receipt checks
only its stated algebra, not the full geometry or performance of a solver.
