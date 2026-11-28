# Discriminant and branch data for the Smith chart [pcc:unverified]

This derives normalization input, not a canonical curve or speed result. Native polynomial identities and geometric deductions have separate evidence status. Independent review remains open.

## Discriminant square class

Use the notation of native-smith-trace/DERIVATION.md. Over a splitting field let lambda_i²=W(x_i), product lambda_i=kappa. The four oriented sums R are roots of monic Q. Pair their six differences into three complementary pairs. Each pair product is, up to sign, 4(lambda_i²-lambda_j²). Hence

    disc(Q)=2^12 product_{i<j}(W(x_i)-W(x_j))².

Write W and W² in the basis (1,x,x²) modulo G, and let T be the coefficient matrix with columns (1,W,W²). The evaluation Vandermonde matrices satisfy V_W=V_x T. Taking squared determinants yields

    disc(Q)=2^12 disc(G) det(T)².

For the unnormalized quartic P(R)=aR⁴+cR²+dR+e with a=delta4=disc(G), d=-delta1, c=delta2, e=delta0, discriminant scaling gives

    disc(P)=a^6 disc(Q)=a (64 a³ det(T))².

Thus the quartic and cubic discriminants have the same square class in K. A square factor of disc(P) is not automatically ramification of the normalized cover; it can arise from collisions in the chosen scalar coordinate. The native gate computes disc(P) by the depressed-quartic formula, then exact-divides by a and takes a polynomial square root. It does not compute det(T) symbolically, so this is a second algebraic expression for the predicted square class rather than a coefficient-by-coefficient T identity.

Because d occurs only through d² in disc(P), this polynomial computation is defined over Fp even on the four maps whose delta1 is not. It does not remove their isogeny rationality obstruction.

## Geometric branch interpretation under explicit genericity conditions

Assume the cubic discriminant a has four simple geometric roots, the norm root r has three simple geometric roots disjoint from a, and D is squarefree of degree two with gcd(F,D) of degree one. N and D are coprime. The source is the original squarefree degree-seven hyperelliptic model.

At a zero of a, the trigonal cubic has a simple collision of two x preimages. Since this value is disjoint from the norm root, neither is a hyperelliptic branch point. Local continuation swaps the two labels. On the four complementary sign triples defining the Smith quotient this acts as a transposition: one simple ramification point, contribution1 to the tame different.

At a finite zero of r, the cubic is unramified, and exactly two distinct preimages are simple hyperelliptic branch points: the norm has a zero of order2 and F is squarefree. Local continuation flips those two y signs. Its action on complementary sign triples is a double transposition: two ramification points of index2, contribution2.

Over t=infinity the cubic has three distinct preimages: the two simple roots of D and x=infinity. Exactly one root of D is a source branch point; x=infinity is also a source branch point for the odd-degree model. The same double-transposition analysis gives contribution2.

These are all branch values: away from the critical values of the trigonal map and images of the source branch points, the six-point composite cover is etale and so is its sign-triple quotient. The total tame ramification contribution is

    4*1 + 3*2 + 1*2 = 12.

If the degree-four geometric cover is connected, Riemann-Hurwitz then gives 2g-2=-8+12=4, hence g=3. This calculation does not certify nonhyperellipticity or exhibit a smooth plane quartic.

The native irreducible-specialization witnesses establish irreducibility over F61(t), i.e. arithmetic connectedness. They are not by themselves a certificate of geometric connectedness: a constant-field extension must still be excluded, or connectedness justified directly from the original Recillas hypotheses. Do not promote this conditional genus calculation to a standalone global model certificate.

## Finite implementation gate

All six frozen maps: dynamic-size Fp[t] arithmetic, exact discriminant polynomial, exact square quotient, squarefree/disjoint critical and paired-branch polynomials, squarefree D and source gcd(F,D) degree1. Retain square index coefficients and its common full power of r; residual index zeros are not labelled branch points merely because they occur in the plane discriminant.

For the two F61 maps only: compare the discriminant formula against the native degree-four discriminant at every t with a(t)!=0. Retain one irreducible separable quartic specialization per map, using gcd(Q,R^(p²)-R)=1 and R^(p⁴)=R modulo Q. Combined with primitivity (gcd(a,d)=1 follows from disjoint a,r), Gauss's lemma makes this a valid irreducibility witness over F61(t). No implementation of the nonsquare isogenies or extension-field solver is claimed.

Source: Smith https://arxiv.org/abs/0806.2995, section6, equations(5),(8)-(11). The Vandermonde and local sign-permutation arguments here are deductions, with no novelty claim. Global normalization, canonical model, complete target transport and the full10x genus2+ objective remain open.
