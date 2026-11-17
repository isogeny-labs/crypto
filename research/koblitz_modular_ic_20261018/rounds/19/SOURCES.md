# Source use and mathematical bridges

All new search/elimination coefficients and point values are computed by
the native code in this round, using the frozen Round18 incidence points,
independent holdouts, source circuits and G17 torsion bases. Earlier
modular/normalization and canonical bundle-class arguments are not replaced
or upgraded by these finite computations.

[Moonen, Chapter IX, The cohomology of line bundles](https://www.math.ru.nl/personal/bmoonen/BookAV/CohomLB.pdf),
Theorem9.11 and Corollary9.12, were read23 October2026. They give the
Euler-characteristic/polarization formula and its multiplication by the
degree of an isogeny. G22 applies these to its explicitly effective ample
auxiliary divisor, using the vanishing statement9.14 to identify the
section dimension. The divisibility obstruction is our calculation for
that auxiliary bundle, not a theorem asserted about the canonical3L.

[Moonen, Chapter VIII, The theta group of a line bundle](https://www.math.ru.nl/~bmoonen/BookAV/ThetaGr.pdf),
Theorem8.10 and Corollary8.11, were reread23 October2026. These describe
line-bundle descent through an isogeny and the kernel-lift requirement.
The arbitrary-field caveat in8.12 is preserved: a geometric kernel action
alone does not prove F2 descent. Our auxiliary sections have their own
explicit rotation/field-map argument. They do not supply the missing
canonical3L mixed trivializations or the second19-kernel action.

CM23's pole bounds and Q29/Q31's elimination argument are written directly
in CM.md and SATURATION.md. The projective closure is defined from the
actual source-coordinate algebra; after inverting h the native source
identities give saturation membership. Complete saturated-ideal equality,
proper normalization, divisor supports and global modular/abelian geometry
remain open unless individually proved. The Lean proof map states which
structural implications and actual finite bindings are kernel checked.

No new database facts, carrier genus counts, timing comparisons or IC
efficiency claims are introduced. Round15--18 primary sources and earlier
norm/rank evidence remain frozen.

G25 uses the actual C from G21, diagonal weights from G16 and the
existing CM relation w²+w+2. Its weighted orbit divisor has a checked
polarization identity, while the canonical symmetric Pic2 class remains
unmatched. The full companion is the recorded57[O] factor. The explicit
41 functions have forced fixed components and48 native kernel
noninvariance witnesses; they are not a proper quotient embedding.
