# Proposed next UInt64 guard lemma (after completed field review)

Define the exact helper word expression:
 n1=wordSub p ((a.q*a.s)%p) a.t
 n2=(a.r*a.s)%p
 Bword=(2*wordSub p ((n1*a.t)%p) ((n2*a.s)%p))%p.

For prime5<=p<2^31 and canonical a,b, prove Bword canonical and decode equal to Degree22Guards.doubleB on decoded a. Constant2 is canonical here; no new small-literal rule needed. Use existing FieldExpr composition rather than changing the multiplication/subtraction tree.

Target source-context implication:
 crossPreludeWord p a b = some sameUDouble -> Bword=0 -> a.s!=0.

Existing cross_prelude_decode supplies the outer support guard and the ordered inverse/equal classifier. Extract crossed-s/t equalities and inverse-false from sameUDouble, preserving earlier inverse precedence. Use canonical equality reflection for B0/s0, decoded2!=0, then the reviewed field guard theorem. No polynomial validity or nonzero-weight premise is required. The conclusion selects the helper's point-doubling arm, not its eventual ResultPoint degree or validity. Direct double22 identity remains permitted outside this source context.

Prospective controls: a nonunit p7 full sameUDouble/B0 fixture; zero-ordinate sameUInverse; p5 boundary; zero-weight inverse/equal overlap; noncanonical negative fixture. Fix actual Field n1/tablesNone and p envelope, no compiler/counter/classsum inference. This file proposes work only; no new word theorem or run has been produced in this review.
