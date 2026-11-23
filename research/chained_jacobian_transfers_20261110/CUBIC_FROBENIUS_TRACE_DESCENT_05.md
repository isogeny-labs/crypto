# Cubic additive trace and faithful cyclic descent

`lean/CubicFrobeniusTraceDescent05.lean` derives explicit normalized-trace and cyclic-descent criteria on stated modules. Both unchanged original Lean4.35/4.34.1 gates pass17 named audited/0 failing/0 trusted under one worker/120s/original memory defaults. Seventeen statements have complete original-extractor bindings, data1421da. This is additive module trace; the multiplicative coefficient norm in full04 is a separate operation. Claims remain unsigned. [pcc:unverified]

## Project onto the fixed part

Write N(x)=x+F(x)+F²(x). The source states each required hypothesis explicitly: an IntModule, additive/scalar-linear F as needed, F³=id, an r-annihilator and integers u,v with3u+rv=1 for normalization. The r-annihilator belongs to the stated module or points; it is not an injectivity claim about an ambient geometric point group. [pcc:unverified]

Trace preserves addition and integer scalars. Its fixed-image identities are F(Nx)=Nx and N(Fx)=Nx. If F(x)=x, then N(x)=3x. The local Bezout argument gives u(3x)=x for an r-annihilated x. Thus pi(x)=uN(x) satisfies pi(x)=x iff F(x)=x, and is idempotent on the stated r-annihilated module. The source derives the complete decomposition x=pi(x)+(x−pi(x)), with N(x−pi(x))=0. No ambient torsion cancellation is assumed. [pcc:unverified]

## Faithful cyclic descent forces a fixed generator

Suppose F(P)=lambda P and trace is faithful on scalar multiples of P: every relation aN(P)=bN(P) implies aP=bP. Scalar transport and N(FP)=N(P) give lambda N(P)=N(P). Apply faithfulness to lambda and1 to obtain lambda P=P and hence F(P)=P. This is a proved necessary condition for an F-stable cyclic subgroup; dropping stability or faithfulness changes the question. [pcc:unverified]

For a fixed r-annihilated generator, normalized_fixed_input supplies the explicit left inverse uN(P)=P under3u+rv=1. This states the exact normalization available on the fixed part. It does not construct a new lower-field curve, Jacobian map or faster DLP algorithm. [pcc:unverified]

## A covariant map to a fixed target kills a nonfixed eigenline

Let f preserve integer scalars and satisfy f(Fx)=f(x), as follows for a covariant map into a Frobenius-fixed target. If F(P)=lambda P and rP=0, then r f(P)=0 and (lambda−1)f(P)=0. A supplied Bezout certificate u(lambda−1)+vr=1 gives f(P)=0 by the same local cancellation argument. The trace is a special case. The theorem does not replace invertibility by the weaker condition lambda!=1 over an arbitrary composite modulus. [pcc:unverified]

The composition_covariance lemma proves that covariance is preserved through two composable maps; it can be reused along a chain. The actual geometric maps, their scalar linearity and covariance, and identification of a fixed target remain inputs. Maps lacking those hypotheses are outside this obstruction, so no general impossibility of prime-field descent or ECDLP transfers is asserted. [pcc:unverified]

## Controls and exact bindings

On the abstract coefficient module Fin109, multiplication by45 has cube identity and trace0. On the fixed action,73(x+x+x)=x. In Fin3,3=0 and there is no inverse3. These kernel-checked finite controls document the nonfixed and noninvertible cases;45 is not asserted to be the Frobenius eigenvalue of any F101 or223 curve in this project. No curve/LP population is enumerated. [pcc:unverified]

| New declaration | Complete unsigned binding |
| --- | --- |
| `CubicFrobeniusTraceDescent05.recover_annihilated_point` | `PCC-20261116-39e216`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.trace3_add` | `PCC-20261116-714116`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.trace3_smul` | `PCC-20261116-1c2a2b`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.trace3_commutes` | `PCC-20261116-a5bc36`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.trace3_fixed_image` | `PCC-20261116-e53864`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.trace3_fixed_input` | `PCC-20261116-396522`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.normalized_fixed_input` | `PCC-20261116-2bebef`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.normalized_fixed_iff` | `PCC-20261116-45c6bc`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.normalized_idempotent` | `PCC-20261116-492961`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.trace_zero_complement` | `PCC-20261116-288c9a`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.faithful_stable_cyclic_forces_fixed` | `PCC-20261116-5a3b3c`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.fixed_target_eigenline_zero` | `PCC-20261116-1e0c33`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.trace_eigenline_zero` | `PCC-20261116-c6bc51`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.composition_covariance` | `PCC-20261116-817c13`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.nonfixed_scalar_control` | `PCC-20261116-3158e3`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.fixed_scalar_control` | `PCC-20261116-37cf8e`. [pcc:unverified] |
| `CubicFrobeniusTraceDescent05.characteristic_three_exception_control` | `PCC-20261116-77faf8`. [pcc:unverified] |

Source SHA256`46e4993775a2c0c64f1aa079b28fb1d13b6e671df035a182f038a16f2e07ef43`; the actual original audits count17 named statements and no generated equations. Only propext,Classical.choice,Quot.sound occur. The local Bezout recovery argument reuses the earlier odd-subgroup method. Full04 sourcee804f278 and all earlier original modules/verifiers/keyring/policy remain unchanged. [pcc:unverified]

Two rejected sources remain frozen. First07960c has redundant third rewrites and an eigenline automation generation/instance failure. Second82b8d6 repairs those but fails abstract scalar distribution. Both print sorryAx and are rejected. The accepted source uses original IntModule.sub_zsmul/one_zsmul/zsmul_add explicitly; no statement, hypothesis, tactic limit or original gate is weakened. [pcc:unverified]

Sources checked: exact PartialFrobenius rational bridge, OddFieldDescent Hom sign descent, TargetFrobenius matching/conjugacy, FrobeniusPairings and odd-subgroup cancellation; full04 trace/norm scope; actual Lean4.34/4.35 module-law interfaces. Preregistration091010 states the competing possibility and controls before editing. KB status/exact/negative/theorem wrappers127, conductor unavailable and locked atlas unread; no novelty or absence inference. [pcc:unverified]

Canonical chain.dot,cubic.dot and reports/prime-field-actual-04/psi.dot remain unchanged: this adds an abstract module criterion, not a new curve, Jacobian edge, actual Frobenius action or cost. Actual field/native/compiler/counter/global-map refinements, original full LP60/240/62/original360/witness/seven controls/feedback/post, runtime/admission and matched costs remain open. Current store403/startup failures115681 separately block normal merge; they are not mathematical counterexamples. [pcc:unverified]
