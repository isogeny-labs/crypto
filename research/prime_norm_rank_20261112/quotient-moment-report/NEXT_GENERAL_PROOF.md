# Next proof target: CRT coefficient reciprocity

Unproved plan, not a result or public-input algorithm. The coefficient detector
lemmas and the a=2 planted control are checked separately; this note identifies
the missing higher-dimensional bridge [pcc:unverified].

Let F,H be coprime monic polynomials of positive degrees a,b. For a polynomial R
of degree below a+b-1, take the canonical remainders

    S = R * inverse(H modulo F) modulo F,
    T = R * inverse(F modulo H) modulo H.

The intended lemma is

    S*H + T*F = R,
    coeff_(a-1)(S) + coeff_(b-1)(T) = 0.

Proof route to formalize: both F and H divide S*H+T*F-R, hence so does F*H.
That difference has degree at most a+b-1, strictly below degree(F*H), so it is
zero. Taking coefficient a+b-1 gives the second identity. Do not replace this
with a numeric rank assumption or an assumed reciprocal pairing [pcc:unverified].

For the even-degree norm construction, set F to the monic associate of A,
H to the monic associate of B, with deg A=a and deg B=a-2. The moment
mu(P)=coeff_(a-1)(P/(fB) modulo F) should then satisfy, for R of degree<=2a-4,

    mu(fR) = -1/leadingCoeff(B) * coeff_(a-3)(R/F modulo H).

Together with unit-weighted coefficient detection, this should identify the
lower Hankel radical as span(B). The upper radical follows from the analogous
quotient coefficient pairing. Formalize the reduction of degree<=a vectors
modulo F and the monic divisibility/degree scalar-quotient step explicitly.
The b=0 case is separate and is the one already checked by the planted control
[pcc:unverified].

Then establish the full two-dimensional radical, mu(G)=0 (so the form is in the
actual coefficient-constraint pencil), and prove that restricting to the target
branch leaves exactly the witness line without introducing extra radical vectors.
Degree drops, coordinate charts, nonzero norm/branch conditions and their
probabilities need their own treatment. Choosing an even pole degree is allowed
by selecting an odd number of online support points, but any end-to-end cost
and success statement still requires its full proof [pcc:unverified].

Sources already reviewed: pinned polynomial monic-division, coprime-divisibility,
quotient-coordinate and coefficient declarations. More exact library lookup and
a faithful Lean proof are owed before promotion. No priority is asserted for
this standard CRT/quotient-pairing route. Even complete certificate existence
would not compute the moments from public data: that finder remains essential.
