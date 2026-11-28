# MST filter stage on M31, re-profiled after the incremental `eliminate` (2026-11-28)

Source: `main` at `99b949b3` (includes #5183). Input: the authentic
group-admitted M31 ten-query stream, 613 rows, sha256 `ed57ac0d…`
(`../m31-mst-strategies-20261125/INPUT_SHA256SUMS`). Runner:
`examples/lp_mst_strategies.rs` built by the `native_gain_dsu` harness,
release with debug info, rustc 1.97.0, valgrind 3.22.0 callgrind.
`lp_graph_eval` MST defaults. [pcc:unverified]

**Method.** Callgrind collects only inside a named function
(`--toggle-collect`). Each figure is `(Ir[reps=11] - Ir[reps=1]) / 10`
for `--time-only <arm>`, so the three identity-check calls cancel and
the number is per MST call. Input parsing and the input clone are
outside `filter_large_primes_mst_with` and not counted. Group probes were
run separately so nested matches cannot cancel. Per-function
differentials: `exclusive-*.txt`, `inclusive-*.txt` (from
`callgrind_diff.sh`).

## Per call

| arm | instructions per MST call | vs exhaustive |
|---|---:|---:|
| exhaustive | 7.163 M | 1.000 |
| overlap-sensitive | 10.448 M | 1.459 |
| shared-key | 6.891 M | 0.962 |

All three produce identical work outside the tree step (identical edges),
4.751 M; the tree step is exhaustive 2.412 M (measured directly; the
function is not inlined), overlap-sensitive 5.697 M and shared-key
2.140 M (by subtraction; those functions are inlined).

## Where the exhaustive arm's 7.163 M goes

| phase | instructions | share |
|---|---:|---:|
| spanning trees (`exhaustive_tree_rows`, incl. its row differences) | 2.412 M | 33.7 % |
| input normalisation (`PartialRelation::normalise`, a BTreeMap per row) | 0.923 M | 12.9 % |
| duplicate-relation check (`retain`: clone each row into a BTreeSet key) | 0.654 M | 9.1 % |
| column bookkeeping: `(degree, column)` set 0.409, member sets 0.311, column map 0.048 | 0.768 M | 10.7 % |
| member normalisation (`scale_row`) | 0.334 M | 4.7 % |
| duplicate-id set and core-vertex count (`BTreeSet<u64>`) | 0.212 M | 3.0 % |
| `WorkRow::from_partial` | 0.150 M | 2.1 % |
| peeling maps (incident lists, degrees) | 0.063 M | 0.9 % |
| output `complete()` | 0.041 M | 0.6 % |
| rest of `eliminate` body (inlined arithmetic, atom/cap sums, modular inverses, row-vector drops 0.199) | 1.606 M | 22.4 % |

Cross-cutting, overlapping the rows above: the allocator (malloc and
realloc 1.269 M, free 1.236 M) is about 35 % of the stage, and
128-bit remainders in `mul_mod` (`__umodti3`) are 0.360 M (5.0 %).

## Reading

- **No single dominant hot spot remains.** Before #5183, the per-pivot rebuild was about
  three quarters of the stage. Now the tree step is a third, and the rest is
  spread over input preparation (25 %: normalisation, duplicate check,
  id set), column bookkeeping (11 %) and allocation.
- **Exact candidates, not done here:**
  - Normalise each row by sort and merge instead of a BTreeMap.
  - Detect duplicates by sorting row indices by key, or hashing, without cloning each row.
  - Use flat vectors with lazy deletion for member sets.
  - Reuse buffers in `axpy` and `scale_row` to cut allocations.

  If all input preparation and bookkeeping cost vanished, the stage would
  fall to about 4.6 M (1.55x). That is the ceiling for this kind of work.
- At about 7 M instructions (about 1 ms) per call on 613 rows, the stage's share
  of a cold M31 solve is still unmeasured; that decides whether any of the
  above is worth doing.
