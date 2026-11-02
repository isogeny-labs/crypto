# R73 complete degree-five residue algebra, frozen before data

Specialize the accepted R52 rank-32 multiplication table at the
irreducible omitted prime `f_C=z^5+z^3+z^2+z+1`, representing its base
residue field as `F32=F2[a]/(a^5+a^3+a^2+a+1)`. Check `a^32=a`
and the source unit. Use all 32 source square columns to define the
absolute Frobenius semilinear map `F(v)=S v^[2]` on the complete
32-dimensional `F32` fiber (160 dimensions over `F2`). Iterate this
map five times on every standard basis vector to construct the full
`F32`-linear relative Frobenius matrix `M=F^5`. Verify semilinear
commutation `S M^[2]=M S` and directly check `F^5(a e_j)=a M e_j`
for all 32 columns.

Iterate `M` until its image rank stabilizes, at most 32 steps. The
stable image is the reduced quotient of the full finite fiber. Compute
fixed dimensions of all `M^d`, `d=1..32`, on that image. Invert the
integer gcd-degree identity through totient and divisor sums to get
every residue-degree multiplicity **relative to `F32`**. Require
nonnegative counts, total reduced dimension and reconstruction of
all 32 fixed dimensions. Save `S`, `M`, the stable image, fixed
dimensions and degree counts. An absolute branch has `F2` residue
degree `5f` for relative degree `f`.

This is the entire degree-five fiber algebra, not a sample of points.
It does not yet split the nilpotent local factors or determine
ramification indices, uniformizers or valuations. Its source order is
R70-normal, and R65's trace Gram has `f_C` exponent 12, which is a
later consistency check.

Exact deterministic computation; no statistical comparison, baseline,
A/A or power calculation. One local Singular/FLINT thread, one math
job, seed 73, 1200 seconds, 4 GiB virtual, CPU affinity not pinned.
Bind copied source/protocol/driver bytes to frozen Git and all donors
to hashes before execution. Accept only exit 0, exact marker and zero
whitespace-question-mark error lines. Preserve failed raw attempts;
freeze repairs under fresh revisions and IDs. No EKS, Diem, freezer,
Conductor claim, merge or PR retarget.
