# Independent review: weighted distinct add11

No blocking discrepancy was found for the stated field-level claim. The review plan was written before implementation inspection. The producer manifest and every listed file matched; the Rust and Lean source pins in sources.json matched current retained bytes. WeightedAdd11, Controls and separate Readback compiled in a fresh private directory, each native exit zero, using the producer's retained dependency paths. Dependencies were reused rather than rebuilt. Readback exposes only propext, Classical.choice and Quot.sound for all five theorems. Original producer parse failure and warning receipts remain retained.

## Source fidelity and theorem scope

Gate add11 lines511–532 compute squares, cross-x products, cross-y products with the same parenthesization, then inverse before doubling, before the distinct expression. Lean output matches the resulting field operations: q=-(A+B), r=A*B, s=Y-Z, t=A*Z-B*Y, z=za*zb, w=(A-B)*(wa*wb). Squaring is self-multiplication; the native pow5 computation is represented by field exponentiation, not a proof of its UInt64 execution.

Normalization matches the source: degree-one x/z² and y/(z⁵w), degree-two q/z², r/z⁴, s/(z³w), t/(z⁵w). The cross-difference identity plus nonzero z weights and unequal normalized x derives A≠B. That condition excludes both earlier guards regardless of y, so inverse-first precedence is respected. The output-weight theorem assumes all four input weights nonzero, derives output z,w nonzero and all four normalized coefficients, then invokes the reviewed affine interpolation theorem. Input Mumford validity and the common polynomial f are explicit hypotheses, not results attributed to the collector.

This proof works over an arbitrary field, with arbitrary polynomial f. It proves normalized polynomial divisibility, not smooth genus-2 geometry, class-sum equality, canonical words, finite-word execution, counter equivalence or walk closure. The producer states these boundaries accurately. The odd-prime word bound is a proposed machine-refinement envelope, not an additional theorem established by this module.

## Nonvacuity and boundary checks

The full theorem control uses Q, a=(0,0,2,3), b=(9,486,3,2), normalized points(0,0),(1,1), f=X⁵-X+X². Explicit polynomial quotients supply both input divisibility hypotheses. The full theorem is instantiated with all nonzero and distinctness assumptions proved. This is genuine nonunit-weight coverage, not merely coefficient simplification. Output z=6,w=-216 and normalized q=-1,s=1 are also checked. Unit-weight, zero-z and equal-normalized-x/different-raw-x controls compile; the latter makes output w zero and explains why raw-coordinate inequality cannot replace normalized distinctness.

## Receipts and limits

All producer artifacts are retained under producer/ (large compiled .olean files omitted from the copied packet but verified against producer manifest before copying). Replay sources, outputs and per-command receipts are under replay/. replay-receipts.json records all native results. Only the three requested modules were freshly compiled; existing trusted dependency artifacts were reused and are separately hash-bound in dependency-artifacts.json. No scientific execution, repository change, canonical-source replacement or performance comparison occurred.
