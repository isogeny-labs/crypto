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
