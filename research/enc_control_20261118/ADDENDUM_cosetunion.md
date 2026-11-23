# Addendum (post-hoc, exploratory) — coset-union degree dial

Written after the preregistered run's k ≤ 7 data were seen and **before** this
arm was run. Not part of the frozen protocol; reported as exploratory.

**Why.** In the main run tower-rand's `D` equals `max(4, k)`, i.e. the degree
of its own input (the closing relation `R_F` has degree `k`), just as randbase's
`D = |F| + 2` is its input degree plus 2. That suggests `D` tracks the encoding's
degree, not some further property of the set. A set that is *not* a subgroup or
a coset but has a closing relation of controlled low degree tests this directly.

**Arm `cosetunion` (`--r j`).** `F` = union of `2^j` random cosets of the
subgroup `H_{k−j}` of order `2^{k−j}`. Then
`∏(x − f) = ∏_i (x^{2^{k−j}} − c_i)` is a polynomial in `u_{k−j}` alone, so
`R_F` lives in `u_{k−j}, …, u_{k−1}` and has degree `≤ j` (unit-tested).
`j = 0` is a coset; `j = k` is `2^k` arbitrary points (generic `R_F`).
Same instrument, curve, targets and verification as the main arms; 2 draws.

**Prediction (before running).** `D = max(D_tower-H(k), j)` up to +1, i.e. `D`
follows the closing-relation degree `j`, not "subgroup vs not": flat at the
tower-H value for `j ≤ 4`, then rising one per unit of `j`.
