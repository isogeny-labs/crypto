# Algebraic inverse from aligned target Kummer coordinates

Full objective remains a complete10xgain for genus2+. This is the next
correctness step for the existing genus3/F127route, not a timing comparison or
new exponent claim. Established comparator: Tian4.2.3,
https://arxiv.org/html/2007.03172#S4.SS2.SSS3 . Officialmsolve0.11README/INSTALL
were read for prime-field parametrization, collected-monomial input and macOS
build. KB lexical query for msove/Kummer lifting returned no direct inverse
implementation; absence is not novelty. Prior alignment08db2132ac is fixed.

## Frozen input and construction

PUBLIC-KUMMER.txt=[82,56,50,22,59,35,77,45] is the M*source-section vector
for public B+D1, from alignment atlas RUN.txt. It is NOT the target divisor's
u,v, point support, target scalar, or serialized expected Jacobian class.
Use same Q1quartic and exact8good-function basis. Generator receives no oracle
image. Calibration cost remains setup in any later full cold experiment.

G1worker generator.rs expands each symmetric cubic tensor in six ordered
point coordinates. Three quartic constraints, seven lambda cross-products,
one w*h0-1 saturation and one fixed parameter t-(x1+3y1+5x2+7y2+11x3+13y3)
produce12equations in8variables ordered w,x1,y1,x2,y2,x3,y3,t overF127.
Saturation excludes the common raw diagonal/basepoint locus. It does not
supply a complete projective/degenerate atlas. Only affine ordered triples
with h0nonzero are represented, matching Tian's charted lifting formulation.

Separate positive control is the already known curve triple(18,52),(28,86),
(43,125); its lambda is computed from the section functions, not supplied.
Require expanded sections agree with independent direct tensor evaluation,
all12equations vanish on the control including inverse/parameter coordinates,
and a corruptedlambda fails at that control. Write-once generation directory
contains positive.ms,target.ms,control-point.txt. Original positive point is
verification data only and is never inserted into target.ms.

All monomials collected modulo127; no repeated monomials in emitted polynomials.
No rational/extension/group enumeration, seed/target substitution or input DLP.
Old basis/norm/cancellation controls are registered setup controls. G1must pass
before using generated systems. Retain failure, no rerun without amendment.

## Solver execution admission after G1

Private msolve0.11sourcecc044551f8e202d00691990fd3fd0b1226a88ea2, AppleClang,
consistent installed FLINT3.5headers/library and libomp;71upstream tests pass.
Installed binary and archivedbinary had missingdylibs; LLVM also missingz3.
The first builds/runtimechecks failed before any science; logs retained. No
system Homebrew paths were changed. deps is reproducible private build cache.

Before actual inverse solving, freeze collector and exact parametrization
verifier in a separate source commit/amendment and checkpoint. Planned cells:
positive then target,2threads, fixed primitive form, genericity-correction off
(-c0), per-cell300second and2GiBRSS limits with retained censored/failure state.
Do not launch target after a failed positive control or failed independent
polynomial substitution check. No performance ratio from these cells.

Accept actual lifting only after parsed parametrization satisfies EVERY input
polynomial identically modulo its elimination polynomial, and a recovered
closed triple yields a target Jacobian class with the input projective
coordinates. Then verify source/destination subgroup and the prior geometric
oracle, held separate from equation construction. Sign ambiguity must be
retained/resolved explicitly; source Brequires subtracting knownD1image after
lifting B+D1. A finite ideal/solver exit alone is not a verified map.

G1source/protocol/input/binaries committed and S3checkpoint/gate BEFORE first
execution. No scientific solver has run at this freeze. Full coldconstruction,
calibration, relation/LA/descent/verification and genus2/4 obligations remain.
