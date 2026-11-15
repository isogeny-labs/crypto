# Amendment 1: remove full-field preprocessing from the projective arm

14 November 2026, after the first native finite run. The frozen control run
in `reports/run-20261115T0542Z/` passed its direct/product equality checks,
but the first implementation found `V` and `k` by enumerating all `q^7`
field elements. It therefore does **not** demonstrate the proposed
`O(q^2)` per-candidate membership bound as a complete algorithm.

Keep that run as the finite correctness control. The amended candidate arm
will compute the binary kernels of `p(sigma)` and `sigma+1` by Gaussian
elimination on the `7ell`-dimensional binary polynomial basis. It will choose
three vectors forming a `k`-basis of `V`, enumerate the projective lines in
normal form `(1,a,b)`, `(0,1,c)`, `(0,0,1)`, and test each quotient. The direct
control may still enumerate its `q^3`-element first-factor set, because it is
only an independent reference at `q=2,4`.

Repeat the same two finite fields and cubics. Success requires the amended
projective arm to match the frozen direct sets and to perform exactly
`q^2+q+1` quotient trials at most after polynomial-time kernel setup.
Retain the original receipt; place the amended run in a new write-once
directory. This amendment changes the implementation and cost interpretation,
not the mathematical definition of `W1` or the direct control.
