# Conditional-label incidence-Gram reduction

The production conditional-expectation labeler now computes the exact same
first-minimum choices without reevaluating every ordered edge pair for every
candidate. This is an algebraic reduction of the existing objective, not a
different heuristic or a relaxed packing rule. `[pcc:unverified]`

## Row score

Let `q` be the label alphabet. While row vertex `v` is being assigned, all
column vertices are still unassigned. For an earlier row vertex `w`, write
`m(v,r)` for the number of edges from `v` to column vertex `r`. If `v` and `w`
receive different labels, their cross-edge pairs contribute zero. If they
receive the same label, their candidate-dependent contribution to the original
objective is, after removing a positive common factor,

```text
W_row(v,w) = deg(v) deg(w)
             + (q-1) sum_r m(v,r)m(w,r).
```

The first term is accumulated from the total earlier degree in each label
bucket. The second is accumulated from sparse `(column vertex, label)` port
counts. Ordered-pair duplication, future vertices, pairs internal to `v`, and
the omitted variance constant are identical for all candidate labels.

## Column score

After row labels are fixed, give a column vertex `z` the profile

```text
P_z(alpha) = number of its ports whose row endpoint has label alpha.
```

For an earlier column vertex `y`, the candidate-dependent contribution when
`z` and `y` receive the same label is, again up to a positive common factor,

```text
W_column(z,y) = sum_alpha P_z(alpha) P_y(alpha).
```

Sparse maps from fixed row labels to earlier column-label buckets therefore
give every candidate score directly. Strict comparison while scanning labels
in increasing order preserves the original first-minimum tie rule.
`[pcc:unverified]`

## Executable equivalence and cost

The old quadratic objective remains compiled in tests as an independent
reference. A native exhaustive control compares both label vectors on 3,280
cases: zero through three ordered edges on a `3 x 3` bipartite vertex set,
one through four labels, with isolates, repeated endpoints, and parallel edges.
All choices agree exactly. The separate schema-3 packing-provenance verifier
also retains its own ordered-edge-pair replay.

Ignoring integer widths and map logarithms, the repeated reference path uses
`O(V q M^2)` edge-pair visits. The sparse incidence-Gram path uses
`O((V+M)q)` weighted additions in the worst label-dense case and stores
`O(M+V+q)` nonzero aggregate data. Meter charges cover candidates, sparse
overlaps, and incidence updates. `[pcc:unverified]`

On the first radix-two guard-boundary child, the same single-path warm debug
test fell from 107.32 seconds to 0.02 seconds (about 5,300x). The expanded
four-path control completes in 0.06 seconds. These are local diagnostic timings,
not calibrated asymptotic cost evidence.

## Closure consequence

The reduction exposes the auxiliary-variable proof object directly. Rows are
controlled by degree-bucket mass plus an incidence Gram form; columns are
controlled by Gram products of row-label profiles. A recursive closure proof
can therefore work with packet-pattern orbit aggregates instead of the full
edge-pair objective. The remaining task is to show these greedy bucket scores
imply the required class and per-side group-load caps at every strict radix-64
child. `[pcc:unverified]`
