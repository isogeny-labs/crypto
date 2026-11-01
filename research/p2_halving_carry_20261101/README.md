# H111: first-layer p² halving carry

This draft freezes a falsifiable transfer experiment before implementation.

For a public target `Q=[k]G` in a prime-order subgroup, form the unique group half
`H=[(r+1)/2]Q`, lift `H,Q` with the coordinate-fixed Hensel section, and measure
the reduction-kernel carry
`C₂(Q)=[2]s₀(H)-s₀(Q)`.  The frozen candidate family asks whether one of 24
constant public predicates of its formal coordinate gives the parity of `k`.

The experiment covers 28 curve/generator rows and 9,096 nonzero targets, with
random-section, matched-label, coordinate-isomorphism, torsion-section, and
generic-algorithm controls.  An exact predicate must support complete
repeated-halving reconstruction, EC verification, a uniform proof, Lean
interfaces, and total cost accounting before any algorithmic claim is promoted.

Status: protocol frozen; measurement not run.  External novelty and any
subexponential claim are not established.
