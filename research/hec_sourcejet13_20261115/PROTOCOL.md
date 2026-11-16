# Frozen actual-source jet decoder gate

Input: existing F127 genus3 fixture, exact kernel certificate and alpha values.
Only same-field Jacobian kernel1; other kernel is known to give a Serre twist.
Use actual unframed quotient section [18,39,55,66,2,33,55,1], centered at psi(T),
T=[1028893](0,44). Source zero x decodes psi(x-T), up to one global sign.

Reuse the degree4 formal Mumford chart at T from geometry13. Translate to x by
CRT composition with x-T, using a nilpotent geometric-series polynomial inverse,
then ordinary Cantor reduction over the jet ring. Report nonunit/overlap/degree
charts as exclusions. Require f-v² mod u=0 through degree4 and constant Mumford
coefficients equal scalar Cantor results on every supported translation.
Check translations on fixed multiples2..17, require at least8 successes.
Existing32 multiplication/inverse jet controls plus determinant control reused.

Scan n=1..2048, x=[n]D1, in order. Scalar functions may vanish; unsupported
scalar charts excluded with counts. At every scalar section zero attempt the
translated jet; compare all27 function constants and8 section constants with
scalar evaluation, and require all section coefficients in F127. Decode using
actual gradient/Hessian, restricted to chords with4rational distinct points and
affine decoded pair. Exclude singular/nonsplit/infinity charts explicitly.
On an admitted chord require exactly one Hessian-zero pair. Stop at6 complete
cases or2048trials; require at least3. No replacement or rerun after outcomes.

For decoded affine P,Q put D(A)=n_x F_y(A)-n_y F_x(A). Compute
r=H(P,P)/(-D(P))=H(Q,Q)/D(Q), nonzero. The projective canonical tangent
identification fixes the isogeny differential up to a scalar lambda. Set lambda
from first complete case; every later r must equal ±lambda. Orient pair accordingly.
Repeat actual source jets at -x; require reversed oriented pair. This fixes one
consistent global sign, not an absolute sign independently calibrated elsewhere.
Construct target class[P-Q] with reused Jacobian arithmetic, verify nonzero,
negation and predicted group-order annihilation. Deliberately zeroed Hessian
must admit6pairs, rejected as ambiguous. Print source divisors, jets, gradients,
Hessians, pairs, scales, target encodings, all exclusions and aggregate counts.

Commit source+lock+protocol, upload source checkpoint before execution. Build
beforehand permitted. One release run,600second aggregate cap; retain stdout,
stderr/failures. No scientific rerun or adaptive patch. No performance claim,
general-public-input map, independent implementation or complete cost result.
Existing raw studies, defaults, closed records and shared resume unchanged.

Prior work: difference13 proof and controls, geometry13 translation-centered
Gamma00 reconstruction, transport13 target law, Tian arxiv:2007.03172 §4.2–5
formal-series evaluation/precision caveats. KB query 'Mumford formal group
translation Cantor derivative isogeny jet', revision9141e386254340a9bdba3036f2bb706a,
found no directly relevant implementation; absence not established. CRT/Cantor
operations are established algebra; no novelty assertion.
