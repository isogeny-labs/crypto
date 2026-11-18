# Next lead: an oriented point image through coordinate ratios

Unimplemented proposal, not a claimed algorithmic result or novelty. Review the
existing j=0 division-function literature and CT115/sign-relaxed/Weber work before
implementation. The same dense degree dependence remains. [pcc:unverified]

For a global r-torsion generator T, write W=x(T)^3/B and the already-used
division functions x([k]T)=x(T)f_k(W), y([k]T)=y(T)g_k(W). Their ratios are
in the Weber field L=K(W): the numerator and denominator transform together
under the six CM units. This suggests selecting source orientation inside L
rather than adjoining an entire sixfold point field or recovering a scalar
first. This field-membership/scalar argument is a proof obligation, not a
formalized bridge.

An implementation to declare and test prospectively:

1. Keep fixed source/target generators P,R and return O for source O. For
   nonidentity order-r inputs, r>3 makes the relevant x/y coordinates nonzero.
2. Hensel-lift the fixed W root and input Weber root as before. Lift an x-ratio
   f_p with f_p^3=w_Q/w_P, starting from Q.x/P.x. Lift a y-ratio g_p with
   g_p^2=(w_Q+1)/(w_P+1), starting from Q.y/P.y.
3. Recognize f in the same field using a source congruence lattice. Accept only
   exact beta=W*f^3 with C(beta)=0 and the full source-ratio check.
4. Recognize g in that field and require the exact compatibility equation
   (W+1)*g^2=beta+1, plus the full source-ratio check. Do not accept unrelated
   local fits or merely numerical roots. Reject bad candidates without
   treating their lost p-adic digits as a whole-query failure.
5. Return S=(R.x*specialize(f), R.y*specialize(g)), checking target curve/order.
   Emit this point before calling a target solver. The map API must not receive
   or recover k, or try source scalar candidates.
6. Separately solve the target DLP and check the single returned scalar on the
   source. Retain source negation/addition, genuine-wrong-root and CRT-chimera
   controls. There must be no CM-unit scalar search hidden in the mapping API.

The intended written argument is f=f_k and g=g_k: global kernel membership
plus source x/y orientation selects the same [k]T, and the universal division
identities then evaluate to [k]R. It needs a faithful common-field/endomorphism
and specialization proof. A partial recognizer would remain partial even if
its accepted images are oriented. No claim of complete coverage or improved
cost follows from this proposal.

Use a fresh version and prospective fixed fixtures/precision/caps. Start with
the small all18-point auditor and the already fixed public inputs; never use
the scalar or an output table to choose f or g. Charge both recognizers and
all failures. Exact algebra and the existing source/target gates remain binding.
