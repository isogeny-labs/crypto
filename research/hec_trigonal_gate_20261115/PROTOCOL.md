# Rational trigonal construction gate

Freeze before code/execution. Kernel census is terminal: full20archived cohort,
two genus3tractable inputs p127family1andp1009family1; all other exclusions
retained. Full mathematical10xgenus2/3/4objective unchanged. No timing/map of
Jacobians/destination solve claim. Established Smith geometry, novelty unclaimed.

Sources: primary Smith arXiv0806.2995v2Sections3/5/9read, especially Plucker
quadric and rational-line intersection in Section5:
https://arxiv.org/html/0806.2995 . Prior KB/source receipts and exact kernel
proof/census in ../hec_kernel_admission_20261115/PROTOCOL.md and reports;
no fresh absence claim or new mechanism asserted. The kernel line is rational
even when its pair factors are permuted by Frobenius. Preserve this distinction.

## Derived implementation without extension-root coordinates
Each actual eligible input has a unique linear factor, paired with infinity;
its other factors have distinct even degrees2/4or6. In an irreducible factor
F of even degree e, the unique stable pairing in these cycle types sends a
root x to tau=x^(p^(e/2))modF. Verify tau(tau(x))=x and tau!=x.
For N=sum n_i x^i,D=sum d_i x^i, i0..3, let p_ij=n_i d_j−n_j d_i.
The paired-value condition is the linear equation
sum(i<j)p_ij (x^i tau^j−x^j tau^i)=0modF.
Expand coefficients in Fp[x]/F to obtain base-field equations; this implicitly
checks every conjugate pair. Pair(r,infinity)contributes p03+r p13+r²p23=0.
No construction of a splitting field or actual root coordinates is required.

The nullspace should have dimension2for these two inputs. In lexicographic
coordinates01,02,03,12,13,23the decomposability quadric is
Q=p01*p23−p02*p13+p03*p12. Enumerate the complete projective line of p+1
base-field points (small fixed gate only) and keep Q0. Reconstruct N,Dusing
any nonzero p_ijchart; independently verify every wedge coordinate. Require
coprime N,Dand maxdegree3, then every polynomial pairing identity plus the
linear/infinity condition. Retain all degenerate candidates and no-map outcomes.
A trigonal map is NOTthe Jacobian isogeny or non-hyperelliptic destination.

## Frozen controls and inputs
64constructed independent wedge controls,16each over5/7/11/17with fixedpivot
p01=1, deterministic remainingcoefficients; Q0and chartreconstruction exact.
Three planted x³maps over13/19/31using four distinct cubic fibres, two distinct
points per fibre (first four fibres with >=2rationalpoints). Require dimension2,
known wedge belongs to complete projective-line roots, and all valid recovered
maps satisfy original pair values. No curve/smoothness assertion for controls.
Two real inputs from census fixtures exactly p127f1andp1009f1; verify headers,
squarefreeness and factor patterns; allcandidate maps/counts retained, no
replacement or rerun if a discriminant is nonsquare. This gate uses native
Rust/cached field-polynomial rlib only, no scientific Python/cloud/timing.

Commit/checkpoint source before execution; preserve raw output/coefficients/
stderr, source and fixture hashes, native reportPDF/SVG. Any mismatch blocks
interpretation, not a mathematical negative. Next successful dependency is
Recillas equations, smooth canonical plane quartic, field/twist and divisor
transport, followed by full same-target costs. Full genus2and4scope remains.
