# R73: complete degree-five fiber and a degree-70 branch

The accepted `degree-five73-02` specializes all 32 source square
columns of the R70-normal order at
`f_C=z^5+z^3+z^2+z+1`. The base residue field is `F32`, so this
is the **entire** 32-dimensional `F32` fiber, or 160 dimensions over
`F2`. Five semilinear absolute Frobenius iterations on every standard
basis column produce a full 32-by-32 `F32`-linear relative Frobenius
matrix. The run checks `a^32=a`, the source unit, all 32 scalar-linearity
columns, semilinear commutation and stable-image invariance.

The relative Frobenius image stabilizes after two iterations at
dimension 20. Fixed dimensions of every power `1..32` and complete
integer gcd-degree inversion yield the reduced residue algebra

`F32^4 x F_(32^2) x F_(32^14)`.

Thus there are six branches with relative residue degrees
`1,1,1,1,2,14`, or absolute `F2` degrees `5,5,5,5,10,70`.
The full fiber length 32 includes nilpotents; no separate local
ramification index is inferred from this matrix alone.

There is a sharper consequence from the already checked trace Gram.
R65 gives `v_fC(disc O)=12`. For the normal rank-32 order, the
fundamental equality gives `sum e_i f_i=32`, while this run gives
`sum f_i=20`. The [Dedekind different bound](https://research.sabanciuniv.edu/id/eprint/577/1/3011800001170.pdf)
states `d_i >= e_i-1`, with equality exactly for tame branches.
Therefore `sum f_i(e_i-1)=12` equals the full discriminant exponent;
every branch at `f_C` is tame. In characteristic two, every `e_i` is
odd. The degree-14 branch must have `e=1`: any larger odd index alone
would consume at least 28 of the budget 12. The other five indices
remain undetermined until full local-factor splitting.

The Lean file `lean/FiveFiber73.lean` kernel-checks the degree sum,
the conditional weighted-budget implication `e_(degree14)=1`, and
the separate rational tame-exponent arithmetic. It does not
kernel-prove the native matrices or the geometric different theorem.
The accepted `degree-five-lean73-03` compiled and audited all three
authored theorems with zero failing and zero trusted declarations.

The record preserves three failures. Native attempt 01 printed a late
marker but is rejected for source-table errors after a ring switch;
the repaired attempt 02 moves the complete square matrix before the
switch and passes exit, exact marker and zero-error gates. Lean
attempt 01 lacked an Omega module in the pinned mathlib build;
attempt 02 imported core Omega without natural-number arithmetic
instances. Attempt 03 imported the pinned available foundation and
compiled/audited. All five attempts are bound to their historical
source bytes in `results.json`.

Next split the six `f_C` local factors with complete absolute
Frobenius-fixed idempotents and full multiplication operators, then
compute their local lengths and ramification indices. The rational
uniformizers/valuations and the infinity chart remain necessary for
proper pole divisors and a complete prescribed Riemann--Roch space.
