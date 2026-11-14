# R71 rational omitted-fiber residue algebra, frozen before data

Use the accepted R52 complete 528-column multiplication table of the
R70-normal rank-32 order over `F2[z]`. Specialize its 32 square columns
at each of the **two** rational omitted primes `z` and `z+1`, without
discarding nilpotents or exceptional basis elements. Check the source
unit under squaring. Iterate the full 32-by-32 Frobenius matrix until
its image rank stabilizes, at most 32 steps. The stable image is the
reduced quotient of the finite fiber algebra. Compute fixed dimensions
of every Frobenius power `1..32` on that image. Invert the gcd-degree
identity by integer totient and divisor sums to obtain every residue
degree count; require nonnegative counts and total reduced dimension.

Record the 32-by-32 square matrices, stable images, all fixed dimensions
and degree counts, plus unit and rank checks. Distinguish finite-fiber
algebra length 32 from reduced dimension and branch count. Do **not**
infer ramification indices or valuations from this calculation alone.
The degree-five `f_C` fiber and infinity are separate complete cohorts.

Exact computation, one local Singular/FLINT thread, one math job,
seed 71, 1200 seconds and 4 GiB virtual. CPU affinity is not pinned.
The source, protocol, driver and donor hashes must bind to committed Git
bytes before execution. Accept only exit 0, exact terminal marker and
zero whitespace-question-mark error lines. Preserve every failed raw
attempt, and freeze any repair under a new source revision and attempt ID.
No EKS, Diem, freezer, Conductor claim, merge, or PR retarget.

Controls: both fibers are processed by the same full table; the unit
squares to itself in each; the stable image is Frobenius invariant; the
gcd-degree reconstruction re-creates all 32 fixed dimensions. There is
no statistical comparison, baseline, A/A or power calculation for this
deterministic finite-algebra task. The next stage will add local
ramification, uniformizers, source-coordinate valuations and leading
coefficients, then the complete prescribed Riemann--Roch space.
