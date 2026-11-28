# Frozen protocol: T20 prime-field elliptic norms

2026-11-12. New component after scientific030e and reconciled metadata bot
5bec81e36ac9b6f7b91ddc5437766248bf7aacfe. This protocol is written and pinned
before new implementation or native runs. Standing is unsigned. [pcc:unverified]

## Question, sources and competing outcomes

Can the actual M=N_C20 R20†:J_T20→E2×F0 be evaluated directly on every
rational balanced T20 input with F101 arithmetic and no input-support
roots? E2=(59,27),F0=(44,21),orders109/113. Raw T20 coefficients in
ascending order are[25,3,69,63,52,62,76]. Use y_raw=50*y_monic and
f_monic=[100,12,74,50,6,46,1];this is a square coordinate isomorphism,
not a retwist. Explicit50 fixes infinity+ independently of sqrt convention.
[pcc:unverified]

Sources checked before approach/code:BD0902.3480§2 Lemma2.6,§7 Lemma7.5,
§8;Galbraith corrected chapter8§8.3 Theorem8.3.8/Corollary8.3.10 and its
proof;DIRECT_NORMS_PROOF.md,CLOSED_FIBRES_PROOF.md,
T20_WHOLE_CORRESPONDENCE_PROOF.md,PRIMITIVE_FOUR_T20_GEOMETRY_PROOF.md;
actual RichelotMaps/EvenModel/EvenCover/Chart/WholeContext and old
consumer Balanced::check/classes contracts. External CT50 exactdd4077d1
constrains the forced bidual branch;no external research script is run.
New KB status/two distinct mechanism/negative queries/theorem query all
return127 because no backend executable;no MCP tools exist. Keep exact
commands/receipts in evidence/t20-direct-norm-primary-02. No novelty or
absence inference follows. [pcc:unverified]

H1:correct reverse quadratic-fibre sums,adjoint sign and two-torsion
translation give two degree-four F101 covers inducing M. H2:a normalization,
translation or exceptional-fibre error prevents coefficient descent or
whole-domain agreement. Falsifiers are any non-F101 coefficient,wrong
cleared curve equation,wrong degree,unsupported valid balanced input,
reference mismatch,failed control,changed old input or absent original
receipt. A failure is preserved;fixes are new attempts,not relaxed criteria.
[pcc:unverified]

## Implementation boundary and exact derivation

Use the actual reversed factors and orientation from RichelotMaps::new;
do not rebuild the cubic embedding or select signs from DLP examples.
Derive r(z),s(z) with existing polynomial operations so the raw cover is
(r(z),Y_raw*s(z)). T20_DIRECT_NORM_DERIVATION.md fixes the formulas,
positive-adjoint negation and coherent translation. Normalize rational
polynomials by gcd/monic denominator,project every coefficient toF101,
check exact cleared target equations and max numerator/denominator degree4
for each r. No function-value interpolation replaces coefficient checks.
[pcc:unverified]

Construction may use the existing F101^12 factory,actual infinity fibre
and cover norms;this setup and its costs remain explicit. Only the final
coefficient representation and balanced-input evaluator are claimed to
use F101. Its type contains Zp,Vec<u64>,Curve<u64>,Pt<u64>;its evaluation
path accepts no FpR,root cache,index,source enumeration position or DLP
label. Rational-function assembly is an adapter over existing poly
add/mul/gcd/divrem/invmod,not a second field/poly/Cantor implementation.
[pcc:unverified]

Use a thin tagged(u,v,n) input adapter with canonical coefficient,monic
degree≤2,ordinate degree,infinity-count and v²=f_monic mod u checks,
matching the existing Balanced::check contract. No new balanced group
engine is introduced. Obtain the complete corpus by inverse projective
Chart transformation of the accepted complete rational T20 classes,
including signed infinity cases;require F101 projection,distinctness,
validation and exact Chart roundtrip. This avoids copying the consumer's
balanced arithmetic engine into the root crate. Its old inputs stay frozen.
[pcc:unverified]

Remove support only when it is an actual pole of the completed cover,
proved through the normalized r denominator/cleared curve relation.
Keep original support degree in(2n+degree−2)*image(infinity+). Evaluate
the remaining quadratic algebra using invmod and the fixed linear-image
sum rules,including repeated jets,coincident and opposite images. No
sqrt/root factoring is used in runtime evaluation. Pole/infinity limits
must be checked before the whole-domain acceptance. [pcc:unverified]

## Acceptance and controls

1. Two named new native tests:one geometry/descent/exceptional diagnostic,
   one complete rational norm/chain diagnostic. Full successor suite43
   retains all old41 with zero failed/ignored/filtered. Existing reference
   guard,legacy guard,fmt and strictClippy remain. All protected source
   hashes are taken before each run and checked after original termination.
2. Both normalized covers have exact degree4,all coefficients inF101 and
   exact cleared elliptic curve equations. Positive infinity descent,
   2D rational,D0 rational half,c=D−D0 killed2,actual adjoint sign and
   both infinity images are checked. No input-calibrated sign choice.
3. All six geometric T20 branches,both signed infinities,all rational
   curve points,actual repeated input doubles and every denominator-pole
   point with multiplicity are compared to the actual reversed proper
   correspondence plus EvenModel norm. Geometric controls stay in the
   previously supported Frobenius6-fixed-or-negated/F101^12-support domain;
   no arbitrary F101^12 Jacobian domain is inferred.
4. Every one of12317 rational T20 classes becomes a validated distinct
   balanced input and agrees with actual N_C20 R20†. Every output pair is
   rational/valid/distinct. Check identity,negation,both generator
   translations,all109+113 scalar images/exact orders,Frobenius and
   both actual compositions with M†=R20 P20 equal[4]. Public9238 recovery
   succeeds on every class;82/85 are the component inverse4 constants.
5. Use actual A[4] and M† to construct16 geometric kernel classes,check
   all256 sums,killed4 and exactly4 killed2;compare both direct-function
   norm and original reference. This checks actual (4,4) kernel type,
   rather than deducing that type from degree16 alone.
6. A coherent two-torsion cover translation is a POSITIVE norm-invariance
   control. Wrong global adjoint sign disagrees on every nonidentity
   rational class. For omitted-infinity corrections count every applicable
   nonzero correction and require an actual mismatch;report zero or
   inapplicable cases honestly. Wrong curve/chart/malformed inputs reject
   before evaluation;all limits must preserve multiplicity. No arbitrary
   translation is mislabeled a Jacobian mismatch when applied coherently.
7. Preserve the scoped comparison on all12317 C0 classes:
   M(R20 Phi49(D))=[2]diag(alpha49,id)N_C0(D). After public normalization2,
   this is the existing direct norm/isogeny bridge. It is a structural
   equality control,not a measured computational advantage.
8. Existing geometry6141-byte/kernel24475-byte native JSON outputs stay
   byte-identical. All23 old algebra tests and ten exact old IDC outputs
   stay;add at least one native test/certificate for the new universal
   cleared quadratic-image identity. IDCv1 remains at2^61−1 and does not
   attest actual F101 coefficients. Do not change the verifier's modulus.
9. Include faithful checked Lean for the central linear-image algebra,
   four-norm/recovery/explicit constants and a readable written bridge.
   Existing39 proof modules/198 written declarations remain;add one new
   module with exact bindings/no holes/zero trusted native. Record final
   source and compiled counts from actual complete gate output. Curve,
   Picard,polarization,descent and jet bridges stay explicit where written.

All acceptance observations remain unsigned until repository signing rules
supply standing;local native/Lean/hash/CI passes are not a signature.
[pcc:unverified]

## Run, custody and publication plan

Local correctness/development only,Rust1.99.1/Lean4.35.0. Native static
bound15min,selected diagnostic10min each,complete successor30min;
algebra15min and complete proof gate10min. Preserve original stdout/stderr,
start/finish/outer exit,source snapshots,before/after hashes and JSON byte
comparisons. A missing receipt is incomplete,never reconstructed from
stdout. Freeze native inputs until terminal+original guards. Stop on
dependent archive/custody errors. [pcc:unverified]

Each attempt leaves an immutable whole raw tree as one standardtar.xz,
member/raw/archiveSHA256/exact roundtrip. Oversized data uses the now
observed documented trusted-base artifact-store route;never prune/split.
Each result/failure gets a source-linked editable visual and inspected PDF.
Commit and publish exact objects on the existing ready rootPR3877;respect
main/ledger/review/archive/signing gates. Then a distinct consumer source
pin/replay is queued without rewriting old pins/jobs. [pcc:unverified]

No timing comparison,benchmark or solver run is admitted by this protocol.
Complete construction,descent,conversion,transfer,inverse/verification and
precomputation costs require separately frozen matched references and
actual EKS budget/archive admission. Existing budget/vault/KB access gaps
do not become mathematical obstructions. Continue unblocked proof/code.
[pcc:unverified]
