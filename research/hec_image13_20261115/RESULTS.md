# Quotient image relations and singular-quartic candidates

**Current status:** the post-run [global certificate](GLOBAL-CERTIFICATE.md) now proves all eight cubics and the unique globally singular quartic for each image. The original empirical-only record below is preserved; see the explicit proof supplement at the end.

Both descended genus3quotient maps produce the same exact fitted pattern:
no quadrics, eight cubics and a unique quartic whose eight partial derivatives
span the fitted cubic space. All unused holdout evaluations pass.
The finite interpolation gate does not certify the nonzero polynomials as
global identities; raw output preserves global_identity_certified=false.
The fullrank quadratic evaluation DOES exclude any globalquadric relation,
because such a relation would necessarily vanish at the trainingpoints.

## Frozen construction
Protocol5e22454497 precedes sourceb1fb249426b6 and pre-run checkpoint
20261115T145452Z-b1fb249426b6. The original curve,54kernel records,54alpha
scalars and both8function coordinate bases remain fixed. Their descriptor
lines are required verbatim. New publicmultiples n1001..4000are considered
ascending; the first192jointregular points are retained,160training then
32holdout. Cap600seconds, no scientific rerun or favorable replacement.

The search ends at1199, with7exclusions:1013,1017,1032(kernel0Theta zero),
1050,1128,1144(kernel1Theta zero),1197(kernel0unsupported originalchart).
Training ends at1166; holdouts1167..1199exclude1197. All attempts and both
384coordinate vectors are in RUN.txt. Only base evaluations are used here;
the prior full-kernel action checks are not repeated.

Native F127RREF computes complete nullspaces of160x36quadratic and
160x120cubic matrices. All8gradient rows per trainingpoint form1280x330
quartic-gradient matrices. Monomial exponents/order and every coefficient
are printed in the raw file. Nullspace vectors are checked against the
original matrices. No expected nullity is imposed on real inputs.

| Quantity | Kernel0 | Kernel1 |
|---|---:|---:|
| Training / holdout points |160 /32|160 /32|
| Quadratic evaluation rank / nullity |36 /0|36 /0|
| Cubic evaluation rank / nullity |112 /8|112 /8|
| Quartic-gradient rank / nullity |329 /1|329 /1|
| Exact coefficient rank of quartic gradients |8|8|
| Gradient span equals fitted cubic space |yes|yes|
| Holdout polynomial and gradient checks |544pass|544pass|
| Corrupted-quartic holdout failures |32|32|

Quartic value is checked as well as all8partials; char127doesnotdivide4,
consistent with Euler's identity. Exact gradient membership is coefficient
linear algebra, not a pointwise fit. Each quartic is unique up to scalar
within the sampledgradientkernel. Both complete coefficient lists retained.
No favorable kernel selected. Values/ranks are finite algebraic quantities,
not timing ratios or independent validation.

## Controls and interpretation
Before realpoints, the rationalnormalcurve[1,t,...,t7]at t0..49has expected
quadraticnullity21andcubicnullity98. Allthese relations also vanish on t60..69.
The square of Q=x0*x1-x2*x3and its8gradients vanish on20fixedQ0points;
adding x0^4is detected at all20. Realquarticcorruptions are detected at every
holdout. Buildclean, runexit0emptystderr, no cap/mismatch.

Tian Remark3.4 describes the complex nonhyperelliptic Jacobian Kummer as the
singular locus of a Coble quartic. Our two patterns are therefore Coble-type
candidates. They are not yet asserted to cut out the complete image, prove
indecomposability, recover a quotientcurve, settle the Serre obstruction or
provide divisor transport. Holdoutzeros do not by themselves supply global
identities. Arithmetic shares the prior field/Cantor/evaluator stack.

A post-run primary-source search identified an alternative dimension proof:
if the even-section bound h0(6M)^+=112 applies with our finite-field ppav
linearization, then the globalcubic relation space has dimensionatleast8;
its containment in the eight-dimensional sampledspace proves equality.
The coefficientgradient test would then certify both singularquartics globally.
The exact hypothesis checks remain pending; NEXT-CERTIFICATE.md records the
argument/source and a tame-Lefschetz route. No source execution is modified
or a global-standing claim silently promoted. A C^3zero-bound certificate
remains an alternative.

## Reproduction and custody

```sh
export PATH=/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH
export CARGO_TARGET_DIR=/Users/adamburan/pq-work/hec-image13-target
cargo build --offline --locked --release --manifest-path research/hec_image13_20261115/Cargo.toml
$CARGO_TARGET_DIR/release/image13 \
 research/hec_torsion13_20261115/CERTIFICATE.txt \
 research/hec_kernel_admission_20261115/fixtures/select-g3-p127-f0.out \
 research/hec_theta13_20261115/RUN.txt \
 research/hec_kummer13_20261115/RUN.txt
```

Originalrunterminal; source/dependency/binary/sharedbasis/alpha/certificate
hashes pinned. HostmacOSARM, no performanceisolation or walltimeclaim. Native
Rustscience only. No cloudworker, performancebenchmark, oldrecord/default/
leaderboard mutation. ExistingnativeCI does not execute this standalonegate.
PDF/SVG rendered and visually inspected before publication.

Sources: pinned priorcoordinate/normalization reports; primaryTian
https://arxiv.org/html/2007.03172 Remark3.4; postrunMüller paperlinked in
NEXT-CERTIFICATE.md. KB ready lexical9141e386254340a9bdba3036f2bb706a,
refresh2026-11-14T21:44:48Z; twoqueries gave adjacentinternal records, not
validation. AdditionalLefschetzsearches did not yet verify the exact algebraic
hypotheses. Established interpolation/gradientlinearalgebra; no noveltyclaim.
Nextglobalequationcertificate and geometry/curve/descent/divisortransport,
then complete costs. Full mathematical10xgenus2/3/4objective stays active.

## Post-run global certificate, 2026-11-15
The finite-field hypotheses have now been checked using Donovan's primary
Lefschetz-Riemann-Roch theorem and its Brauer trace. GLOBAL-CERTIFICATE.md
proves h0(A,6M)^+=112 for these symmetric ppavs without assuming a curve.
The global cubic kernel has dimensionatleast8 and is contained in the
sampled kernel of dimension8, hence they coincide. All eight fittedcubics
are global identities. Exact gradient membership and Euler then prove the
recorded quartic is globally singular along the coordinate image, uniquely
up to scalar. This does not prove scheme equality with its gradient locus.
The prior rawfalse flag and candidate report preserve the empirical-only
status at execution; this explicit mathematical supplement supplies the
additional proof, without scientific rerun or independent signature.
