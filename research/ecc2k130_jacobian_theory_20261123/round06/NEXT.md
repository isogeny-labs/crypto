# Proposed next calculation: ramified hyperelliptic base

This is a proposed argument to check, not a result established in the
round06 report. Do not change that pinned report to add it.

Assume the genus260 option and suppose its ordinary genus130 base D
is hyperelliptic. Reduced L_D=Phi131 should force its131 geometric
hyperelliptic branch points to form one Frobenius orbit, using the
same parity at1/131 as round05. Thus the rational ramification point
P of X->D is not a Weierstrass point. Its hyperelliptic conjugate P'
is distinct and rational.

Let iota be the hyperelliptic involution of D. Form W as the compositum
of X and its iota-transform over D. Over an algebraic closure they
have disjoint ramification supports P and P', so the two quadratic
Artin–Schreier classes are independent. Try the explicit equations
z1²+z1=h, z2²+z2=iota(h): the flips a,b and the swap s above iota
should give a constant D4 action over F2, with s²=1 and s*a*s=b.
Check geometric connectedness and this descent, not just the group
order.

Predicted inertia over P1: at the131 branch points of D, inertia is
an order2 reflection in the class{s,abs}, with invariant1. At the
image of P/P', inertia is a reflection in class{a,b}, invariant1.
Verify these assertions in the compositum, including local splitting
over the algebraic closure; do not assume tame ramification.

If correct, Riemann–Hurwitz predicts g(W)=521. The quotient
D''=W/<ab,s> should have genus0 (branched only at the image of P/P')
and Y=W/<s> should have genus130, giving a hyperelliptic Y/F2.
Compute the fixed-point/different contributions exactly: the s-fixed
points above the131 branch points should number262 and contribute
524 to the different in W->Y, so1040=2*(2*g(Y)-2)+524.

Then split the rational odd-l cohomology by D4 representations.
H1(X) contains the base character for D plus one fixed line in each
copy of the2-dimensional representation; H1(Y) should have only the
latter, since the other degree2 quotient D'' has genus0. If the
constant F2-action and quotient descent hold, this would give
Prym(X/D) F2-isogenous to J(Y). The round05 hyperelliptic genus130
exclusion would then contradict P~A.

Check every inference before promoting this to an exclusion. In
particular, an abstract characteristic-not2 bigonal result is not a
substitute for the characteristic2 inertia and F2-descent calculations.
If the proof fails, retain the failed step and leave the ramified
hyperelliptic-base option open. Neither outcome settles arbitrary
nonhyperelliptic bases or the original curve/map question.
