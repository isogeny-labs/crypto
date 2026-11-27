# A graph-factor obstruction to six-agreement interpolation

Written mathematical derivation, 24 November 2026. Producer assertion only;
no Lean/native implementation or independent signed theorem is supplied.
This is a specialization of classical valuation, divisor and multiplicity
arguments, with no novelty claim. [pcc:unverified]

## Exact hypotheses and conclusion

Let E/k be a smooth projective geometrically integral curve. Let H be an
effective divisor of degree6, T a rational point, D=H-T of degree5, and
r in L(H) but not L(D). Let P_1,...,P_N be distinct rational points outside
supp(H), {T} and supp(A), where A is any divisor. Put d=deg(A). Suppose
N>=8, s>=1, and

    Q(Z)=sum_i a_i Z^i in k(E)[Z],   a_i in L(A-iD).

At every (P_j,r(P_j)), require Q to belong to the s-th power of the maximal
ideal in the regular local ring of E x A^1. This is total local
multiplicity, not just univariate multiplicity in Z. Then:

* If d<6s, Q=0.
* If d=6s, every nonzero Q is b(P)*(Z-r)^s for some nonzero b in k(E).
  Its only function-field root is r, which is outside L(D).

For the equality case, nonzero b necessarily satisfies
div(b)+A-sH=0. Existence for an arbitrary A is not asserted. For A=sH,
the explicit universal boundary polynomial (Z-r)^s exists. The threshold
is therefore exact in pole budget, given these hypotheses; no claim that
the point-count threshold8 is optimal is made. [pcc:unverified]

## 1. Weighted valuation is multiplicative

For a nonzero polynomial F=sum_i f_i Z^i, define the divisor

    W_D(F)_v = max_{i:f_i!=0} (i D_v - v(f_i)).

It has finite support, can have negative coefficients, and obeys:

    coefficients(F)_i in L(B-iD) iff W_D(F)<=B;
    W_D(FG)=W_D(F)+W_D(G).

Here is the cancellation argument for the second identity. At a place v,
minimize v(f_i)-iD_v and take the LARGEST index attaining that minimum;
do the same for G. In the coefficient at the sum of those two indices,
their product is the unique summand of minimum weighted valuation. Any
other tied summand would require an index larger than one of the chosen
largest minimizing indices. All remaining summands have strictly larger
valuation, so cancellation cannot raise the minimum. This works in every
characteristic, with no differentiation or separability assumption.

The assumptions r in L(H)\L(H-T) imply

    W_D(Z-r)=H.

Away from T, D=H dominates the pole order of r. At T, the failure of
membership in L(H-T) says v_T(r)=-H_T exactly, so the maximum is H_T
there as well. Consequently, if F=(Z-r)G and W_D(F)<=B, then

    W_D(G)<=B-H.                                      (1)

If F has Z-degree l, its nonzero leading coefficient is in L(B-lD).
A principal divisor has degree0; thus

    5l <= deg(B).                                    (2)

Negative degree on the right is already incompatible with nonzero F.
[pcc:unverified]

## 2. Evaluation and extraction of a graph factor

If W_D(F)<=B and deg_Z(F)=l, then, because r in L(H)=L(D+T),

    F(r) in L(B+lT).                                 (3)

Indeed, each f_i*r^i is in L(B+iT), which is contained in L(B+lT).
If F has multiplicity at least u at all N graph points, substitution
Z=r gives at least u zeros at each P_j. Unless F(r)=0, (3) and the
degree0 property of principal divisors imply

    uN <= deg(B)+l.                                  (4)

Suppose now deg(B)<=6u and u>=1. Equation (2) gives l<=deg(B)/5, hence

    deg(B)+l <= (6/5)*deg(B) <= (36/5)*u < 8u <= Nu.

This contradicts (4). Therefore F(r)=0 and Z-r divides F in k(E)[Z].
Its quotient has the coefficient bound (1). At each interpolation point,
Z-r is a regular parameter transverse to the graph, with total order1.
The regular local ring has a domain as its associated graded ring, so
orders add under multiplication; the quotient has multiplicity at least
u-1. Merely obtaining ONE factor does not complete the argument: the
decreased pole budget is essential for repeating this step.
[pcc:unverified]

## 3. Induction, including the boundary

Starting with Q, after t factors the nonzero quotient Q_t has

    W_D(Q_t)<=A-tH,
    multiplicity >=s-t at each graph point,
    deg(A-tH)=d-6t <=6(s-t).

For every t<s, the preceding step therefore extracts another graph
factor. This either encounters the immediate negative-degree contradiction
or reaches Q=(Z-r)^s Q_s, with W_D(Q_s)<=A-sH.

If d<6s, the final negative degree forces Q_s=0. If d=6s, (2) forces
Q_s to have Z-degree0, say b(P). Its coefficient condition is
div(b)+A-sH>=0 with degree0; hence that divisor is0. This proves both
asserted conclusions. A Frobenius-sparse polynomial is still an ordinary
polynomial in k(E)[Z]. Replacing s by p^e*s therefore does not escape this
argument when total multiplicity and coefficient bounds are preserved.
[pcc:unverified]

## 4. The existing regular elliptic charts

On the nonsingular Montgomery model y^2=x^3+A_curve*x^2+x in characteristic
different from2, assume
R=(r_0,s_0) is finite with s_0!=0. The existing section basis is

    1, x, y, x^2, xy, phi_R=(y+s_0)/(x-r_0).

The usual orders are poles(x)=2O, poles(y)=3O and poles(xy)=5O.
The numerator cancels at -R, has value2s_0 at R, and the quotient has a
simple pole at O; thus poles(phi_R)=O+R. Put H=5O+R.

| Normalized term | Unknown-message space | Received function | T |
| --- | --- | --- | --- |
| coefficient of xy is1 | L(4O+R) | -xy | O |
| coefficient of phi_R is1 | L(5O) | -phi_R | R |

Both are instances of the same argument. The first message is the span
of 1,x,y,x^2,phi_R; the second the span of 1,x,y,x^2,xy. Genus1
Riemann--Roch gives dimension5, and their distinct pole orders show the
listed sections form the respective bases. This is a written instance
bridge, not a formal proof of the Rust section compiler. [pcc:unverified]

More generally, normalizing a nonzero section value in a rational fibre
of L(H) leaves L(H-T) as the unknown hyperplane and has this same
obstruction. Arbitrary coefficient hyperplanes need not be complete
Riemann--Roch spaces of degree5; no assertion that every normalization
has that form is made. Identity targets, pole-point interpolation,
non-rational/aliased evaluation points, nonuniform multiplicities and
different ambient function spaces require separate statements.

## Route decision

Close the standard all-point GS interpolation route for these charts,
uniform multiplicity and the six-agreement degree certificate. Increasing
the list degree, multiplicity or Frobenius exponent cannot provide a
nonzero admissible interpolant below6s; at6s its roots remain universal.
This is an algebraic obstruction to this certificate, not a solver timeout,
an attack lower bound, or a rejection of joint coefficient solving,
power decoding, masks or every index-calculus method. [pcc:unverified]
