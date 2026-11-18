# R90 corrected rank-32 RR transport, frozen before execution

R89's single native attempt is rejected: its bound 51 was exhausted at the
first column in both directions, and its `ERROR`-based procedure returned to
top level before printing a false pass marker. This is a new attempt and
does not amend the R89 source, protocol, or raw record. The additive R89
erratum is a byte-pinned donor here.

Use the accepted R50 rank-32 PID basis `B`, common denominator `F(z)` of
degree 37, and saved source embedding `B*S=F I`. The 32 source monomials are
`T^i V^j` with `0<=i<16`, `0<=j<2`. R27 cusp valuations give the bound 61
derived in `../89/BOUND-ERRATUM.md`: each normalized generator has original
A-fiber pole order at most `61e` and B-fiber pole order at most `17e`.
Test successive powers 0 through 61 of `h=z+V` in the forward direction
and `k=z+V+z²` in the reverse direction. These are the accepted R85/R88
diamond overlap functions. Bound 61 is a ceiling, not an asserted minimum.

Evaluate all 32 actual PID generators through the R85/R86 forward and R88
inverse rational maps in the 32-dimensional quotient field. For each of
64 images, extract and reconstruct its power-basis vector, compute the
full `B*C=F*W` product using the saved source embedding, and require all
32 module coordinates to lie in `F2[z]`. Reject a corrupted coordinate
for every image. Export all pole powers, vectors, and coordinates for
separate reload. Any failed assertion prints `RR90_FATAL` and invokes
Singular `exit` inside the procedure; the driver also requires an exact
completion marker and no `?` diagnostics. The fatal-stop mechanism was
tested on a minimal scratch script before freezing R90.

Run one serial native Singular/FLINT process, seed 90, one thread, 4 GiB
virtual memory, and 1200 seconds. The exact source/protocol must be
committed and pushed before execution; the driver compares every source
file to that Git revision and checks every donor hash and byte count.
Retain a failed attempt rather than silently raising the bound or reusing
the attempt ID. Accept only a zero native exit, exact marker, no native
errors, 64 explicit column lines, 64 full products, 64 corrupted-coordinate
rejections, and a complete exported certificate. No timing comparison.

The backend remains the pinned local Singular build used by R89 for this
exact CAS continuation. The available EKS default image does not contain
that 242 MiB native CAS installation; moving larger symbolic jobs to EKS
requires a digest-pinned custom image and an output receipt. This run tests
localized module membership, while proper projective boundary geometry
remains a separate task.

Sources checked: R27 `CM.md` cusp table; R50 `RR-ALGEBRA.md` and
`NEXT_SOURCE_NORMALIZATION.md`; R85-R88 `CM.md`; R89 frozen failure and
erratum. The [Singular manual](https://www.singular.uni-kl.de/ftp/pub/Math/Singular/src/4-1-1/singular.pdf)
documents that `ERROR` returns to top level, explaining R89's false marker.
