# Overlap-sensitive exact Prim for the large-prime MST filter (2026-11-25)

Branch `claude/overlap-sensitive-mst-20261125`, PR #5183. Component change to
`src/cryptanalysis/large_prime_filter_mst.rs`. No timing claim, no natural
Prime31 stream, no end-to-end ECDLP claim. [pcc:unverified]

## Boundary

The original MST filter eliminates one large-prime column `j` at a time. It
normalizes the `k` member rows (coefficient of `j` = 1) and runs Prim on the
complete graph whose edge `(v,u)` is the sparse difference `r_v - r_u`, keyed
`(|small|+|large|, |sources|, v, u)`, with infeasible edges absent. It forms
all `k(k-1)/2` differences. The new opt-in `MstStrategy::OverlapSensitive`
(`filter_large_primes_mst_with`) returns the same tree while forming only the
differences of *overlapping* pairs plus the `k-1` selected rows. The default
(`filter_large_primes_mst`) is unchanged.

## Algorithm and proof

Per normalized row: `w_v` = small terms + large terms other than `j`, `q_v` =
source terms. Rows `v,u` *overlap* if they share a small column, a large
column other than `j`, or a source id.

* **Disjoint pair, exact key.** No term cancels or merges, so the key is
  `K(v,u) = (w_v+w_u, q_v+q_u, v, u)` and the resource counts are additive.
* **Subadditivity.** For any pair the support of `r_v - r_u` lies in the
  union, and each shared key merges two terms into at most one, so an
  overlapping pair's key is *strictly* below `K(v,u)` in its first two
  components.
* **Cap condition (sufficient).** If the two largest rows of the column fit
  together in every cap (`max_small_weight`, `max_sources`,
  `max_active_coefficients`, by top-two sums), then by additivity and
  subadditivity *every* pair is feasible: the feasible graph is complete.
  If it fails, that column runs the exhaustive Prim (`exhaustive_fallback_trees`).
* **Selection.** Let `m` be the visited vertex minimal in `(w,q,index)`. For
  a fixed unvisited `u`, `K(v,u)` over visited `v` is ordered as `(w_v,q_v,v)`,
  so `m` gives `u`'s best implicit edge when `u` is disjoint from `m`; if `u`
  overlaps `m`, the explicit edge `(m,u)` is strictly below `K(m,u)` and below
  every other implicit candidate. Over all `u`, `K(m,u)` is minimized by the
  first unvisited `u` in `(w,q,index)` order, and if that `u` overlaps `m` the
  explicit `(m,u)` edge is already smaller. Hence each Prim step picks
  `min(best explicit edge in a lazy heap, K(m, first unvisited))`.
  Keys include `(v,u)`, so ties break exactly as in the exhaustive Prim.
* **Materialization.** A selected implicit edge is formed by the same
  `axpy(p-1)` as the exhaustive Prim, so relation coefficients, RHS and source
  witnesses are identical. Its key is checked against the prediction; a
  mismatch would recompute the tree exhaustively and is counted in
  `implicit_key_mismatches` (tests require 0).

Two earlier ingredients were removed after mutation testing showed they could
not change the result, as the bullets above show: a per-row "exceptional
best disjoint endpoint" for rows overlapping `m`, and an overlap skip in the
implicit scan.

Work per tree: overlap graph from an inverted index (input traversal),
one difference per overlapping pair, `k-1` materializations (output), and
`O(k log k + |overlap pairs| log k)` heap/ordered-set work. With no overlap
the tree forms only its `k-1` output rows.

**Accounting choice.** The budget check and the legacy `edge_evaluations`
counter still charge `k(k-1)/2` logical edges per column, so refusals,
relations, `LargePrimeFilterReport` and every legacy `MstStats` field are
identical across strategies. New counters (serde-skipped when zero, so
existing JSON is unchanged): `explicit_edge_evaluations`,
`implicit_selected_edges`, `overlapping_pairs`, `exhaustive_fallback_trees`,
`fallback_edge_evaluations`, `implicit_key_mismatches`.

## Evidence (local, source-exact harness)

`research/lp_graph_algorithms_20261123/native_gain_dsu/run.sh`, rustc 1.97.0,
release: **43/43 tests pass** (41 existing + 2 new).

* `overlap_sensitive_prim_reproduces_exhaustive_prim_exactly`: 1200 seeded
  streams, 2..61 rows, arity 1..3, primes 3, 7, 101, 1000003, 2147446831,
  2^61-1, small-column density 4..2^20, a hub family (one light row sharing a
  hot column with about half the rows), tight weight/source caps, tight edge
  and coefficient budgets. Exact equality of `Option<Vec<CompleteRelation>>`,
  report and legacy stats; every output replays from the inputs. 267 atomic
  refusals and 369 fallback trees occur and agree.
* `overlap_sensitive_prim_forms_no_difference_on_disjoint_rows`: one column,
  40 disjoint rows over F_2147446831: 0 explicit differences, 39 implicit
  selections, 780 logical edges; plus a cap-failure fixture that falls back.
* Mutation check: flipping the `m` update, the explicit-vs-implicit
  comparison, the implicit key, the explicit-key update rule, or any one of
  the cap tests makes the equivalence tests fail.

Differences formed (explicit + materialized + all pairs of fallback trees)
versus logical edges, from the same run. These are counters on synthetic
streams, not runtimes:

| small columns | hub | formed | logical | ratio |
|---:|:-:|---:|---:|---:|
| 4 | no | 44,594 | 58,792 | 0.76 |
| 4 | yes | 21,897 | 26,033 | 0.84 |
| 8 | no | 36,092 | 53,758 | 0.67 |
| 64 | no | 30,335 | 62,043 | 0.49 |
| 1024 | no | 21,190 | 57,342 | 0.37 |
| 2^20 | no | 20,940 | 55,428 | 0.38 |
| all | | 260,287 | 446,718 | 0.58 |

The ratio stops falling at sparse small supports: rows created by earlier
merges share source ids and the remaining large columns, so residual overlap
stays even when small supports are disjoint. Fallback trees (cap condition
failed) are a further floor.

## Natural M31 stream (2026-11-25)

On the authentic group-admitted M31 ten-query stream of PR #5076 (rebuilt
from the frozen producer input; hashes match CI), the two strategies give
identical relations, witnesses, report and legacy stats, with 0 replay
failures. There is **no saving**: 1273 of 1328 pairs overlap, the
strategy forms 1280 differences against 1328, and the MST stage runs
about 1.13x *slower* (informational wall time). Every admitted row carries
the target column `u32::MAX` (10 distinct coefficients), so rows of one LP
column nearly always share it. Details: `evidence/m31-mst-strategies-20261125/`.

## Next step

The exact key of any pair is computable without forming the difference:
`key(v,u) = K(v,u) - (shared keys) - (shared keys whose normalized
coefficients are equal)`, summed over small, other-large and source keys.
On this stream the only shared key is usually the target column, so an
inverted index over the few hot keys would give every pair's exact key in
O(shared) work. Whether that is worth it depends on the hot-key structure.
The whole MST stage on the 613-row stream takes about 3 ms; its share of a
cold M31 solve has not been measured here.
