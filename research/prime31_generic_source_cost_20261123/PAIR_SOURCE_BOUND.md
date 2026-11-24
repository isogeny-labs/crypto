# Exact witness supply and cost of the generic four-sum pair source

Date: 2026-11-23. This bound applies to the materialized signed-pair
source in `research/prime31_graph_ecdlp_20261123/native/mitm_four.rs` and
to a campaign seeking order-`B` relation rows. It is not a lower bound on
algebraic point decomposition or on every single-target algorithm.

## Exact expectation

Let a support contain `B` distinct liftable abscissae on a prime-order
curve of order `n`. The source admits both signs and requires four distinct
support indices. There are exactly `16*C(B,4)` signed four-point tuples.
For a uniformly sampled public query `R=aG+bQ`, each fixed tuple sums to
`R` with probability `1/n`. Linearity of expectation gives

`E[witness tuples per query] = 16*C(B,4)/n`.

This statement requires no independence between tuple sums and does not
predict useful row rank. The native code deduplicates pair order, replays
each group equation, and separately measures projected rank. On the
published B677 curve `n=2147446831`, the expectation is
`16*C(677,4)/n = 64.6376197055` per query, or `711.0138167607` over eleven
uniform queries. The frozen source-bound receipt emitted 707 distinct
equations in eleven queries, a compatible observation but not a statistical
validation from one deterministic panel. Its target row is independently
replayed in `research/prime31_graph_ecdlp_20261123/PROTOCOL.md`; the
algebraic RR5 source remains a separate incomplete experiment.

### How much of B677 belongs to the 2+2 split

The public support comprises 509 liftable Dickson abscissae and 168
liftable Y7 abscissae. If exactly `k` of four distinct leaves come from
the Dickson side, the exact uniform-query raw-witness expectation is
`16*C(509,k)*C(168,4-k)/n`. The frozen generic-source receipt records
the observed classes on its eleven deterministic queries:

| Dickson leaves `k` | Expected per uniform query | Expected over 11 | Observed in receipt |
| ---: | ---: | ---: | ---: |
| 0 | 0.2386 | 2.62 | 3 |
| 1 | 2.9437 | 32.38 | 35 |
| 2 | 13.5128 | 148.64 | 154 |
| 3 | 27.3492 | 300.84 | 294 |
| 4 | 20.5933 | 226.53 | 221 |
| Total | 64.6376 | 711.01 | 707 |

The active 2-Dickson/2-Y7 algebraic branch covers the `k=2` class:
20.905% of the full raw signed-tuple expectation and 154/707 rows in
this generic-source receipt. These are reachability counts before RR5
chart conditions, finite-field root lifting, failures, deduplication
across queries and useful-rank tests. The close expected/observed class
counts do not validate the uniform model statistically from one fixed
panel. In particular, completion of the 2+2 positive control alone
would not demonstrate extraction across the other four classes or an
end-to-end source replacement.

### Signed large-prime rank on the same frozen rows

Let `L` be the signed coefficient block on the 168 Y7 large-prime
columns, reduced modulo the curve's prime group order. For any chosen
row family, the dimension of its space of LP-free linear combinations
is exactly `row_count - rank(L)`; useful factor-base rank can be lower.
Independent modular elimination of the frozen rows gives:

| Row family | Rows | Occupied LP columns | `rank(L)` | LP-free dimension |
| --- | ---: | ---: | ---: | ---: |
| 2 Dickson + 2 Y7 | 154 | 142 | 134 | 20 |
| 3 Dickson + 1 Y7 | 294 | 138 | 138 | 156 |
| Both families | 448 | 162 | 162 | 286 |

The 2+2 signed-edge graph has 142 vertices and nine connected
components. Its ordinary cycle dimension is `154-142+9=21`, but one
component is sign-unbalanced, so the modular LP kernel has dimension
20. Adding the 2+2 rows after the 3+1 rows increases the combined
LP-free dimension by `286-156=130`, even though 2+2 alone supplies at
most 20. This measures a real cross-family graph effect, **not** 130
new independent factor-base equations or a completed algebraic-source
gain. The 221 direct four-Dickson rows have no LP columns and are
separate from this table.

The exact source receipt is
`research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/forty-cap-full-cost.json`
with SHA-256 `6214084754ef21a0fa830f50a623b3789214010879a08e53c8768346dcbb73bd`.
The public support classification was recomputed from the fixture in
`research/prime31_graph_ecdlp_20261123/evidence/rr5-b677-prerequisite-01/structured-positive.stdout` using
the ten Dickson iterations; that file's SHA-256 is
`146b5893d8bd5f5d55392bae6de20c4792f5ee950a2131b54ca079e9630b6468`.

## Cost within this materialized-pair design

The source constructs `4*C(B,2)` signed pairs, then probes one complement
per signed pair for every query. Counting those stages alone gives

`W_pair(B,T) = 4*C(B,2)*(1+T)` group-addition-scale operations.

For B677 and `T=11`, this is `915304*12 = 10983648` operations. The
source-bound ledger counts `10987751` group additions including support
validation, scalar/query work, relation replay and target replay; the
pair stages alone account for `10983648` of them. The same-target rho
control in the protocol counts `99970` group additions. This is a
source-specific accounting comparison, not a universal complexity claim.

If a campaign requires an expected `cB` signed rows for a constant
`c>0`, it needs `T >= cBn/(16*C(B,4))` uniform queries and at least one
query. In this materialized-pair design, the planning cost therefore has
the form `Omega(B^2 + n/B)` for `B` in the range where the four-tuple
approximation applies; its minimum scales as `n^(2/3)` at
`B` of order `n^(1/3)`. Correlation, duplicates, failed extraction and
rank deficiency are not credited by this raw-tuple bound and can reduce
the useful rank obtained from its expected witness count.
This does **not** prove an `n^(2/3)` lower bound for target-span recovery:
some targets can be determined before full base rank, as the B677 control
itself demonstrates. Nor does it apply to a source that does not enumerate
and probe every signed pair.

## Decision for the active M31 objective

Graph filtering can save work after relations exist, but rearranging the
same support does not change the exact expectation above. A proposal for
an asymptotic relation-finding gain must change source cost or source yield
and show completed useful rank, not just improve the projection of these
generic rows. The current split-selector RR5 branch is a candidate for
that different source mechanism; its capped controls must be judged on
completed extraction rather than input degree or partial solver degree.
