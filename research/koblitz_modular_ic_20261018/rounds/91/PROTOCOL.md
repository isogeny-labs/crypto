# R91 fixed-size rank-32 RR transport, frozen before execution

R90 is preserved as a rejected attempt. Its checked forward first column
cleared at power 61, but column 2 hit a dimension error when Singular
converted a vector whose 32nd component was zero into a 31-by-1 matrix.
R91 changes only the power-vector representation used by the matrix
products. The source explicitly assigns all 32 components into a declared
`matrix powerMatrix91[32][1]` on every power trial and checks its dimensions
before `S*W`, `B*C=F*W`, and the corrupted-coordinate comparison. A small
Singular scratch test confirmed a trailing-zero vector converts dynamically
to 31 rows but fills a declared 32-row matrix with final entry zero.

The mathematical input remains the R50 rank-32 PID basis `B`, denominator
`F(z)` of degree 37, embedding `B*S=F I`, and 32 monomials `T^i V^j` for
`0<=i<16`, `0<=j<2`. Evaluate every generator under the accepted R85/R86
forward and R88 inverse diamond maps. Search overlap powers 0 through 61
using `h=z+V` forward and `k=z+V+z²` reverse. For each of 64 images,
reconstruct the 32-component power vector, require polynomial PID
coordinates, check the full matrix product, reject one corrupted coordinate,
and export powers, vectors, and coordinates for independent reload.

Run one native Singular/FLINT process, seed 91, one thread, 4 GiB virtual
memory and 1200 seconds. Pin Git source/protocol bytes and all donor hashes
before execution. Accept only exit zero, exact completion marker, no `?`
errors or `RR91_FATAL`, 64 explicit column lines, 64 full products, 64
corruption rejections, and a complete export. Retain any failure and freeze
a fresh attempt for a repair; do not mutate R91 data or increase the bound
silently. This is exact computation, with no timed comparison.

The backend remains the pinned local Singular CAS installation for this
continuation. A digest-pinned custom image is required before moving this
native workload to EKS Spot. This calculation concerns localized rank-32
module membership; proper boundary normalization and direct transfer are
separate obligations.

Sources checked: R27 `CM.md` cusp valuations, R50 `RR-ALGEBRA.md` and
`NEXT_SOURCE_NORMALIZATION.md`, R85-R88 `CM.md`, R89 erratum, and R90
`CM.md` plus raw log. The
[Singular manual](https://www.singular.uni-kl.de/ftp/pub/Math/Singular/src/4-1-1/singular.pdf)
explains the top-level `ERROR` behavior handled in R90 and retained here.
