# Protocol: x-only siblings in the k = 5 sum-table build (P2 KSUM-SIB), 2026-11-18

Frozen before any measured row. Every statement here is `[pcc:unverified]`.

## Change
`ksum:m=5,sib=1` (`src/cryptanalysis/ic_boundary.rs` `KSumTable::build_negation_folded_full`, `src/cryptanalysis/ic_framework/plugins.rs` `KSumOracle`): at the table's last level each base pair ±P gives both keys x(S + P), x(S − P) from one inversion (`CountedGroup::sibling_pair`), charged as ONE group addition plus one unit of the native counter `sibling_second_points` (`work.rs`: sqr + 2 add), the convention of the walk's `sib`. Same kept sums, keys, witnesses, collisions and insert order as without it (unit test `sibling_build_is_the_same_table_with_one_inversion_per_last_pair`).

## Arms and workloads
Specs `specs/P20.json`, `P22.json`, `P24.json`: the three Table B curves, targets and target seeds of `research/ic_tableb_ksum_20261115` (48 public targets per curve), base sizes 9 / 10 / 11 as there. Arms: `rho-neg` (reference), `ksum5` (baseline, unchanged), `ksum5-sib` (candidate), `ksum5-aa` (A/A control, the baseline again). 2 rounds, 1 warm-up, alternate order, L0 (local 4-core container, not isolated). Counted cost, so host noise does not enter S.

## Predictions (registered)
- P-a (identity): every run of both arms verifies; per target the two arms make identical trials and relations (same table, same probes).
- P-b (cost): paired `ecbench compare` S ratio ksum5-sib / ksum5 at w = 0 below 0.95 at every size, upper 95% end below 1, A/A exactly 1. Point prediction from the build count: about 0.80–0.88 (last level 2^4·C(a,5) of about 1.4·2^4·C(a,5) build additions; build about half of S).
- P-c (band): the order "ksum5-sib < ksum5" holds at w = 0, 0.1 and 1 (`ecbench band`).

## Decision
Improvement over the incumbent if P-a holds and P-b, P-c hold at all three sizes. Kill if any key/trial mismatch, or ratio ≥ 0.95 at both w = 0.1 and w = 1 at any size. Distance to rho reported as distance remaining.
