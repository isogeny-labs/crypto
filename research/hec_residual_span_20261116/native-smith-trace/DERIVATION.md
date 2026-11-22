# Trace-pairing inverse for the Smith scalar chart [pcc:unverified]

This is a derived reconstruction formula, not an asserted new mathematical result or speedup. It is conditional on Smith's generic correspondence setup; independent review and smooth normalization remain open. Work over K=F61(t). More generally the argument works in characteristic different from 2 when the cubic and quartic below are separable. No field enumeration occurs in the proof.

Let G=x³+g2*x²+g1*x+g0, let a=F mod G, and suppose delta1=8 sqrt(s) is in K. Put

    Q(R)=R⁴+u R²+v R+w,
    u=delta2/delta4, v=-delta1/delta4, w=delta0/delta4,
    kappa=delta1/(8 delta4).

Restrict to delta4 != 0, Q separable and R != 0. In the cubic etale algebra L=K[x]/G compute

    W = a / G'(x)²,
    S = Tr_L(W), H = Tr_L(x W), J = Tr_L(x W²).

The inverse of G' exists because G is separable. All these quantities belong to K; this works even when G has no rational roots.

Let x1,x2,x3 be the cubic roots in a separable closure, and choose lambda_i with lambda_i²=W(x_i) and product kappa. Such a choice exists since product W(x_i)=s/delta4²=kappa². The four sign choices with that fixed product give

    R = lambda1+lambda2+lambda3,
    X = x1 lambda1+x2 lambda2+x3 lambda3.

Smith's interpolation formula identifies R with the oriented coefficient b2, and b1=X+g2 R. Expanding the four sign choices gives

    Q(R) = R⁴-2S R²-8 kappa R+(2 sum(lambda_i⁴)-S²).

This agrees with the normalized Smith quartic (with orientation fixed by delta1). The four R values are distinct on the chosen separable open. Thus X has a unique polynomial expression C(R) of degree at most 3. This expression is invariant under the Galois permutations of the cubic roots and their compatible sign changes, so its coefficients are in K.

Here is an explicit linear construction of C requiring no lambda_i. In E=K[R]/Q form the trace matrix

    M_ij = Tr_E(R^(i+j)),  0 <= i,j <= 3.

For the four sign choices, expanding R^j X and summing gives

    Tr_E(X)    = 0,
    Tr_E(R X)  = 4H,
    Tr_E(R² X) = 8 kappa (x1+x2+x3) = -8 kappa g2,
    Tr_E(R³ X) = 4(3 S H - 2 J).

For example, in the third line the only sign-invariant terms are the products involving all three lambdas; in the fourth they are lambda_i⁴ and lambda_i² lambda_j². Solve

    M c = (0, 4H, -8 kappa g2, 4(3SH-2J))^T.

The matrix is nonsingular: over a splitting field it is V^T V, where V is the square Vandermonde matrix of the four distinct R roots. Equivalently its determinant is the discriminant of the monic Q. This is an algebraic trace-pairing statement and does not require Q irreducible. Therefore c belongs to K and C(R)=sum(c_i R^i) is well defined.

Recover the full polynomial by

    sigma = C(R)+g2 R,
    tau = (a2-sigma²+2g2 sigma R-(g2²-g1)R²)/(2R),
    b(x) = tau+sigma*x+R*x².

The tau formula is the x² coefficient of b²=a mod G. By the lambda interpolation construction these coefficients recover the same b and hence satisfy all three coefficient equations, not only the x² equation. The forward map b -> R and this inverse are mutual inverses on the common generic chart with the delta1 orientation. This supplies rational inverse expressions over the function field, expressed algorithmically through cubic inversion and a four-by-four linear system; it is stronger than a finite point bijection. It is not a computed canonical map.

## Denominators and exclusions

Denominators arise from G' inverse (cubic discriminant delta4), normalization by delta4, the trace determinant disc(Q), and 2R. The preceding scalar/Veronese orientation additionally uses delta1 != 0. The native comparison keeps the earlier stricter exclusions delta0*delta1*delta2*delta4 != 0 and disc(Q) != 0 so coverage is exactly matched, without implying all these exclusions are minimal. Extension places and t=infinity require separate treatment. The formula does not certify that no exceptional points exist.

The native gate specializes all 115 previously eligible parameters and checks all 109 recorded points. It also uses a separate split sign-orbit control over F101 with x=(1,3,9), lambda=(2,5,11). Specialization checks support implementation correctness; the function-field assertion rests on the argument above, not on extrapolating from 109 points.

The setup and interpolation are from Smith, *Isogenies and the Discrete Logarithm Problem in Jacobians of Genus 3 Hyperelliptic Curves*, section 6, equations (8)-(11): https://arxiv.org/abs/0806.2995 . The trace reconstruction here is a deduction; novelty is not claimed. Canonical normalization, nonhyperellipticity, complete Jacobian/target transport, matched costs and the 10x genus2+ objective remain unproved.
