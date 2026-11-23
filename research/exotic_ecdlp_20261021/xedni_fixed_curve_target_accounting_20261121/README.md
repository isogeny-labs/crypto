# Anchored relation table recovers a frozen public target with explicit setup accounting

A fixed rank-one curve supplied 28 exact rational coefficient-two/six words.
Their reductions form a rank-20 matrix on 21 sign-folded columns; adding
`log_Q(Q)=1` gives rank 21. Solving that matrix yields a lookup table which
recovered and independently verified the frozen public target
`T=(37,29)`, scalar `20`, in all three alternating pairs with same-point
Floyd rho. An independent integer-group and modular-matrix audit recovered
the same scalar and checked every recorded timing sum.

| Repetition | Arm order | Table online ms | Table setup ms | Table setup + online ms | Rho online ms |
| ---: | --- | ---: | ---: | ---: | ---: |
| 1 | table, rho | 0.089417 | 81.005958 | 81.095375 | 0.191333 |
| 2 | rho, table | 0.015167 | 5.490750 | 5.505917 | 0.135666 |
| 3 | table, rho | 0.041208 | 5.504959 | 5.546167 | 0.238834 |
| Median | — | 0.041208 | 5.504959 | 5.546167 | 0.191333 |

The [protocol](protocol.json) froze the rational curve, group, 14-multiple
prefix, two word coefficients, SHA-256 target fixture law, one-target
workload, three arm orders and timing boundaries before the run. Target
construction from the fixture scalar is outside both online intervals;
both solvers receive the same public point. The table interval starts at
target fold and lookup and ends after scalar replay. Rho's interval starts
at its first target-dependent walk state and ends after scalar replay.
[`arm-rows.jsonl`](arm-rows.jsonl) preserves every start/stop timestamp,
status, scalar, setup phase, table size and rho walk count. All six arms
completed with verified status. The table setup phases are exclusive:
curve/group creation, rational relation generation, matrix build, final
linear algebra, and lookup-table build; their sum is the reported setup
interval. The rational words reach a 7,241-bit maximum affine coordinate.

The independent [`audit.py`](audit.py) reconstructs the order-43 group by
a separate integer law, rebuilds the 28×21 relation matrix, solves its
anchored system by separate modular elimination, checks scalar `20`,
replays every answer, and verifies timing and source hashes.
[`audit-result.json`](audit-result.json) is `PASS`; the checked Sage runtime
and raw stderr are retained.

## Interpretation and proof map

This is an ordinary-host correctness and component-timing diagnostic. The
table has precomputed endpoints for all 42 nonidentity points and receives
known scalar-labelled rational multiples; its online lookup reflects that
complete preparation. The **same-process** setup-plus-online median is
5.546167 ms, with the 81.095375 ms first-use observation retained. The
[protocol amendment](amendment.json) clarifies that each repetition rebuilds
the application-level table and matrix in one Sage process, whose internal
caches persist. The result field `table_cold_median_ns` names this
setup-plus-online quantity; it is not a process-fresh estimate. No Tier 0
or Tier 1 isolation receipts were collected, so these intervals remain
exploratory and are not a promoted performance comparison.

| Result | Declaration or check | Scope | Status |
| --- | --- | --- | --- |
| Rational word relation on the selected rank-one curve | `XedniRankOneAnchor.rational_relation` in the [curve packet](../xedni_anchor_rank1_search_20261121/README.md) | `P+2Q=O` for its displayed `P,Q` | Lean checked |
| Endpoint counting bound and exact prefix-14 coverage | `endpoint_card_le`, `prefix14_covers_nonzero` in the [counting packet](../xedni_endpoint_count_bound_20261121/README.md) | Bounded direct lookup and `ZMod 43` | Lean checked |
| Rank-20 28-row system and anchored rank 21 | — | Fixed reduction, 21 folded columns | Sage and independent Python elimination; Lean matrix-rank bridge open |
| Target scalar and timing phases | — | Frozen target and ordinary local host | Independent arithmetic/timing audit; empirical |

The next measurement should use a process-fresh setup for each run and a
Tier 0 or Tier 1 resource envelope if wall-time comparison is to be
promoted. A distinct algorithmic test must generate useful target relations
without enumerating a full scalar-labelled endpoint table.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_fixed_curve_target_accounting_20261121/run_sage.py"
python3 research/exotic_ecdlp_20261021/xedni_fixed_curve_target_accounting_20261121/audit.py
```
