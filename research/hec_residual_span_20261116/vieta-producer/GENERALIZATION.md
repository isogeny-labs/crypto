# Truncated residual coefficients for genera 2,3,4

[pcc:unverified] Derived algebra for independent review; not yet executed or a genus3/4 solve claim. Standard monic polynomial division and Vieta recurrence, no novelty claim. Conditions: f=X^(2g+1)+c, u monic degree g, deg v<g, u divides f-v², and w=v+t u. Then h=(f-w²)/u is monic degree g+1. Two distinct known roots x1,x2 outside u imply pair divides h; the quotient Q has degree g-1.

Write u=X^g+U1 X^(g-1)+U2 X^(g-2)+..., w=t X^g+W1 X^(g-1)+W2 X^(g-2)+..., Wj=t Uj+v_(g-j). Set H0=1. Comparing descending coefficients of u*h=f-w² gives:

    H1 = -t²-U1
    H2 = -2t W1-U1 H1-U2
    H3 = -(2t W2+W1²)-U1 H2-U2 H1-U3.

Only H1..H_(g-1) are needed for g<=4. Let s=x1+x2,r=x1*x2. Pair is X²-sX+r, and descending quotient coefficients satisfy Q0=1, Q1=H1+s, Qj=Hj+s Q_(j-1)-r Q_(j-2).

For g2 remaining quotient is X+Q1 (root -Q1). For g3 it is X²+Q1 X+Q2. For g4 it is X³+Q1 X²+Q2 X+Q3. Existence and monicity of the exact quotient prove that reconstructing these leading coefficients determines it entirely; no omitted low coefficient can vary. Multiplicity of an additional root equal to x1 or x2 remains valid. If the two selected roots coincide, divisibility by their square needs a derivative/multiplicity proof and is NOT supplied by two evaluations; this method requires x1!=x2 as before.

Prospective qualification must compare this Q against two exact polynomial divisions and original principal identity on valid anchors, include repeated residual roots and denominator-zero exclusions, and charge root solving. It must not assume the remaining quadratic/cubic splits. General f introduces descending f coefficients in H recurrences; that extension is not covered by binomial executable evidence. No new scientific invocation has been made for this generalization.
