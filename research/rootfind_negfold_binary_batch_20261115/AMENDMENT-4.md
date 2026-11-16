# Before-data amendment 4: share signed-pair denominators in the scan

Status: specified before any execution or outcome row in this study,
2026-11-15 PST. This amendment changes only the `rootfind-negfold-batch`
`m=3` relation-scan implementation. It leaves the fixed spec, six arms,
targets, seeds, repeats, resource limits, failure retention, and the primary
wall decision in [Amendment 3](AMENDMENT-3.md) unchanged. The original
F4/F5/F6 panel has a separate method and source receipt.

The signed binary factor base stores the two points above an abscissa next
to each other. For a fixed scan point `R`, both `R-P` and `R+P` use the
same field denominator `x_R + x_P`. The existing eight-point batch forms
eight denominator entries, including duplicates. The candidate now uses
`FastCurve::add_many_lazy_paired_x`: each adjacent equal-abscissa run
contributes one denominator to the batch product chain, then its inverse
is used for both slopes. The K13 base starts with one self-negative point,
then 16 adjacent signed pairs. Each fixed eight-point block therefore
contains three complete pairs and two boundary singles: five denominator
entries rather than eight. The current four-lane
`Gf2::batch_inv` executes one product per nonzero input in its forward
pass, two per nonzero input in its backward pass and nine fixed lane
combination products. Thus an ordinary full K13 block changes this path
from 33 to 24 field products, a difference of nine; both versions still make one
field inversion per block. The two point sums,
abscissae, slopes, lookup order, first witness, and abstract addition
charge remain the same. A final singleton has its own denominator.
Infinity, a block with no adjacent equal abscissae, or an available
eight-lane SIMD kernel uses the existing lazy batch routine;
equal-abscissa exceptional sums use ordinary point addition. These
arithmetic counts are derived from the source for a nonexceptional K13
full block on the portable path, not measured timing or a complete-cost
claim.

The focused arithmetic test compares ordered lazy sums and completed
points against the original path, including exceptional points, a final
singleton and a non-paired fallback. The ecbench parity test compares
first witness, lookup count and addition bounds against the scalar engine
on its fixed K13 point set. Passing these tests is a correctness gate,
not a wall result. The unchanged study controls still require all 48
same-target pairs to finish correctly with L2, an A/A interval containing
one, and the target-cluster wall threshold of Amendment 3. Every failed
or censored row is retained. A new pushed-source publication receipt must
bind this amendment and the changed source before the study can execute.
