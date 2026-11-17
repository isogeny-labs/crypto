# Explicit smooth plane quartics and same-field quotient identification

The exact order-four theta-jet construction produced two smooth planequartics
overF127. The post-run reconstruction/descent proof identifies kernel1as a
same-field quotient Jacobian; kernel0has the nontrivial Serre sign. Both
outcomes, equations and proofs are retained. No explicit divisor transport
or complete solve timing is supplied by this gate.

## Exact models and measured algebraic results
For q=sum c_(a,b,c)x^a y^b z^c, exponent order is

(0,0,4),(0,1,3),(0,2,2),(0,3,1),(0,4,0),(1,0,3),(1,1,2),(1,2,1),
(1,3,0),(2,0,2),(2,1,1),(2,2,0),(3,0,1),(3,1,0),(4,0,0).

Kernel0coefficients:
[1,47,59,89,95,83,53,119,40,11,113,107,72,67,121].
Kernel1coefficients:
[1,119,17,98,109,32,90,122,100,58,107,5,65,44,119].

| Registered quantity | Kernel0 | Kernel1 |
|---|---:|---:|
| Rational2torsion image Hessian rank |1|1|
| Four ordinary-point Hessian ranks |4,4,4,4|4,4,4,4|
| Constant-plus-quadratic section matrix rank |7|7|
| Gamma00line dimension |1|1|
| Nonzero order-four coefficients |15|15|
| Affine smoothness ideal |unit|unit|
| Buchberger basis size / processedpairs |11 /8|11 /8|
| Infinity partialgcd / endpoint |regular|regular|
| Exact projective pointcount overF127 |127|129|
| Reconstructed Jacobian Frobenius trace |+1|-1|
| Serre sign from proof plus trace |nontrivial|trivial|

The first public factorbaseplace R=(0,44)gives T=[1028893]R nonzero,
[2]T=0, U0=x³+15x²+90x+109,V0=0. No point/curve replacement.
Local V=t0+t1x+t2x²and the explicitU2/U4solution satisfy the curve equation
through degree4in35jetmonomials. Optimized jet multiplication agrees with a
naive35²pair implementation on32registered controls;32inverse identities
and a constant determinant pass. All54Weil constant terms agree with the
previous scalar evaluator, all sectioncoefficients descend toFp, alloddjets
vanish. The unframed sections allow evaluation even at zeroTheta.

For smoothness, affine partials generate the unitideal by nativeBuchberger;
infinitygcd and the remaining endpoint separately exclude singularities.
The routine accepts Fermatx4+y4+z4and rejects x4+y4with its singularpoint.
No cap/basis/degree limit is reached. EveryprojectiveF127point is counted;
counts are direct, not a Weil estimate or time measurement.

## Geometric and descent proof
PROOF.md uses full quadraticrank36to exclude geometric product and
hyperelliptic cases, then Oort–UenoTheorem4to identify each geometric
quotient as a smooth nonhyperelliptic genus3Jacobian.

RECONSTRUCTION-PROOF.md identifies the jetquartic with its canonicalcurve:
characteristic0Gamma00containsC-C, so its nonzero leadingquartic vanishes
on the canonicalplanequartic and equals it up toscalar. A smoothquartic
coefficientlift, theta-characteristic lift, unitrank7minor and nonzerojet
specialization transfer this identity to characteristic127. The source
isogeny is étale, so its local parameters supply an invertible tangent
change. No assumption that the isogeny itself lifts is needed.

Functorial tangent-space reconstruction gives the Torelli descent curve;
Serre's original strongTorelli Theorems2/3/5 leave only a{±1}cocycle.
The quotient Frobenius trace is-1. The two exactcounts then pick the sign:
kernel1Q1has129points(trace-1), so A1≅Jac(Q1)overF127. Kernel0Q0has127
points(trace+1), so A0is the nontrivial[-1]twist ofJac(Q0), and is not a
Jacobian overF127. Pointcount alone would not identify a geometric twist;
the functorial reconstruction proof is essential.

The frozen raw jacobian_identified=false flag remains correct for the
executable's empirical-only scope. The separately stated post-run proof
supplies identification; no rawreceipt is rewritten or independent signature
claimed. Proof and data share the prior arithmetic, not an external replay.

Derived from this identification, Q1has the prior quotientLpolynomial
[1,1,-43,-1224,-5461,16129,2048383], Q0has L(-T). Their Jacobianorders
2057786and2027974are DERIVED, not independentlypointcounted overextensions.
The targetprime1028893divides the former. An explicit polarizedisomorphism
or source-to-Jac(Q1)divisor-map formula remains to be computed.

## Freeze, reproduction and custody
Protocoldfd2fa054e plus pre-executioncontrolamendment precedes source
54661e56ddfd and checkpoint20261115T152922Z-54661e56ddfd.
Originalexecutionexit0emptystderr; no cap/scientificrerun. Clean locked
offlineRustbuild onmacOSARM. All560sectionjetcoefficients, nodecoordinates,
hyperplanes, quarticcoefficients and controls are inRUN.txt; hashes pin
source/binary/dependencies/inputs. No scientificPython, cloudworker,
performancebenchmark, oldrecord/default/leaderboardmutation. NativeCI does
not automatically run this standalone scientificgate.

```sh
export PATH=/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH
export CARGO_TARGET_DIR=/Users/adamburan/pq-work/hec-geometry13-target
cargo build --offline --locked --release --manifest-path research/hec_geometry13_20261115/Cargo.toml
$CARGO_TARGET_DIR/release/geometry13 \
 research/hec_torsion13_20261115/CERTIFICATE.txt \
 research/hec_kernel_admission_20261115/fixtures/select-g3-p127-f0.out \
 research/hec_theta13_20261115/RUN.txt \
 research/hec_image13_20261115/RUN.txt
```

Reports/diagrams rendered and visually inspected. Sources and exacthypotheses
are in the two proof files: originalOort–Ueno,Müller,Pauly–PreviatoGamma00,
Serreappendix/LRZTorelli. KBready lexical9141e386254340a9bdba3036f2bb706a,
refresh2026-11-14T21:44:48Zpredatesstudies; adjacentnotesnotvalidation.
This is established geometry and exactconstruction, no novelty or10xclaim.
Next source-to-Q1divisortransport, destinationsolver and fullchargedcosts;
mathematical10xgenus2/3/4objective remains active.
