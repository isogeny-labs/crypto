# Frozen source-to-target projective calibration

Correctness/alignment only; no inverse lift, complete mapper, timing or10xclaim.
Prior target gatec99ae1c7dc89 passed; source calibration remains separate.

Pinned public inputs: kernel54points and active27alpha scalars extracted from
coldbuild13/RUN.txt at7117af90f867, original public curve/D1, existing group/
Weil/theta/Kummer source copies. Original source evaluator's homogeneous8
sections are(13Phi0²,Tr(z^j Phi1Phi2),j0..6). Field-trace shortcut is used ONLY
on base-field source inputs; no extension-valued shortcut or new normalization.
PUBLIC.txt contains model/order/knownD1, not the target discrete logarithm.
The caller supplies no expected image or label to Source::eval.

Construct target same five-point model,8sections and beta centering as original
gate. Load no serialized QClass image across group references. Instead rebuild
known geometric generator image from original publicmap13V2certificate:
qref=[1005372]2*((43,125)-(101,43)), using its registered n853and inv2n.
This is a prior geometric isogeny image used for setup/calibration, not a cold
construction or independent replay. Require nonzero/ell annihilation.

Scan calibration n1..64ascending only until16joint source/target chart samples;
retain all encountered exclusions. First12fit, remaining4holdout. Each pair
is source eight-section value at nD1 and centered target value at n*qref.
Fit M using all28cross-products per pair,64unknowns. Require rank63/nullity1,
invertible8x8M, all12training and4unused scalar holdouts projectively equal.
No calibration subgroup scalar enters a target-DLP solve.

Mixed holdouts not used in fitting: B+D1,2B,-B+D1,B+2D1, where public B=2*(3,21).
Reconstruct prior geometric B image from[203071]2*((28,86)-(18,52)), n38;
require nonzero/ell annihilation. These are verification oracles only, not
fit rows. Require all4 source/target charts available and transformed values
projectively equal. Any failure retained; do not change holdouts after failure.
Raw Bdegree2is explicitly unsupported by the source evaluator and this premise
is checked. The fixed B+D1translate anticipates subtracting qref AFTER inverse
lifting; it does not yet constitute direct Bmapping or a complete source atlas.
A corrupted M must fail a mixed holdout, constructed by perturbing one entry
acting on a nonzero source coordinate and a different nonzero-target row.

Global scope: assuming the already identified polarized quotient/Jacobian and
complete2Theta bases, their coordinate change is projective linear. Unique
calibration determines that change under those premises. Finite holdouts alone
do not prove a new isogeny, independent arithmetic or full general-map coverage.
Primary Tian4.2.2-4.2.3 provides established projective alignment/lifting context:
https://arxiv.org/html/2007.03172 . This avoids full2torsion alignment by using
one prior geometric image; cost of obtaining that image remains charged in
any later cold experiment. No novelty or amortized-to-cold substitution.

Guard600seconds. Commit source/lock/protocol/fingerprints and checkpoint before
one execution to RUN.txt/RUN.stderr/EXIT.txt. Preserve any failure, no rerun
without amendment. Shared resume/defaults/closed records untouched. Actual
inverse solving, chart completion, full cost and genus2/4 remain open.
