#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Proven-zero columns in paired transport
2026-11-02 · Derived schedule / static audit · Numerical trials: zero

*Construction.* The first nonzero column index j defines a checked zero
prefix. Record it separately: a secondary diagonal pivot can be chosen at
i greater than j. Preserve the original search and selected pivot.

#image("pipeline.svg",width:100%)

*Proof.* If A eₜ=0 and P swaps two coordinates, then
(P A P)e₍ₚ₍ₜ₎₎ = P A eₜ = 0. Thus the zero-index set follows the same
permutation as the cached column keys. Missing front columns in this set
can be materialized as zero tuples. The unchanged paired formulas still
require the established weighted-symmetry invariant.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived case],[Transport queries removed],
 [Scalar pivot with j greater than 0],[1],
 [Pair pivot with j greater than 0],[2],
 [j equals 0],[0],
 [All-zero source],[No pivot move])

*Width accounting.* Each reused column removes one forward query at that
swap's input width w, or 4w+8 instrumented native products under the preserved
kernel schedule. For two pair swaps the widths are w and w+4: removed native
products total 8w+32. Output width and all outside-query field arithmetic
remain unchanged. Counts are derived; no elapsed-time comparison was run.

*Costs and provenance.* A set retains j indices and is permuted per swap.
Every reuse materializes n zero entries. Native membership, canonical-cache,
allocation and permutation work remains. New counters record these events;
they do not measure total heap or runtime. Source identity, signature, exact
record type, prefix range and one-use checks guard the internal handoff.
They do not independently certify a forged prefix or mutated source.

*Checks.* Three complete declared-substitution AST comparisons preserve the
selector, paired arithmetic and full metric-view driver. Five Python sources
parse. Frozen references remain byte-identical. Dynamic correctness,
corruption, counter, heap and performance validation remains unexecuted under
the existing cost admission gate. No job or new build was launched.

*Next construction.* Use the same zero-coordinate information to derive
coordinate deletion and primal/dual reinsertion, with a compatible displacement
representation. That larger change is not implemented or included in counts.

*Sources.* selection\_zero.py, paired\_zero.py, driver\_zero.py,
integration.json, DERIVATION.md; unchanged finite-field pivot context:
Dumas–Pernet2018, arXiv:1802.10453, introduction and Section 2.
