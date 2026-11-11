# Reduced paired numerator from biquadratics

Derived 2026-11-09, T-1983. Work on y^2=f(x)=x^3+a*x+b in odd characteristic, with the same public baby algebra and nonrational-shift guards as Round63. The polynomial identities below are over Z before specialization. Their coefficient certificate and finite-panel implementation remain unverified producer evidence pending independent review. [pcc:unverified]

Define N(x,t)=t*x^2+(a+t^2)*x+a*t+2b and F(x,t)=(x*t-a)^2-4b*(x+t). Direct coefficient collection gives

N(x,t)^2-4*f(x)*f(t)=(x-t)^2*F(x,t).

This is the short-Weierstrass biquadratic addition identity: with nonzero x-t, x(A+T)*x(A-T)=F(x,t)/(x-t)^2. Native identity.rs checks the polynomial equality with independent sparse integer coefficients, checked additions/multiplications/exponents, and no elliptic-curve or finite-field helper. It also checks the cancellation below with c=y*w*gamma an independent indeterminate. No specialization or sampling is used for these coefficient checks. [pcc:unverified]

In the quotient algebra K[T]/H_G let V=beta(T), W=gamma(T), A=(x,y), B=(z,w), and put D_G=(x-T)^2,D_Q=(z-V)^2,D=D_G*D_Q. Write N_G=N(x,T), N_Q=N(z,V), F_G=F(x,T), F_Q=F(z,V), and L=N_G*D_Q-N_Q*D_G. Round63's numerator coefficients are

E0=L^2-4*y^2*f(T)*D_Q^2-4*w^2*f(V)*D_G^2+8*y*w*W*D,
E1=2*L*D.

Use y^2=f(x),w^2=f(z). Expanding the square L^2 groups the first two terms as (N_G^2-4*f(x)*f(T))*D_Q^2 and (N_Q^2-4*f(z)*f(V))*D_G^2, with the remaining -2*N_G*N_Q*D and 8*y*w*W*D. Apply the biquadratic identity twice. Every term has the common factor D, giving

M0=F_G*D_Q+F_Q*D_G-2*N_G*N_Q+8*y*w*W,
M1=2*L,
E0=D*M0, E1=D*M1.

The independent coefficient checker proves E0=D*M0 in Z[x,t,z,v,a,b,c], where c represents y*w*W, and also E1=D*M1. A deliberately wrong gamma sign has a nonzero residual; full coefficient supports are retained. This certificate uses the curve equations through f(x),f(z), and treats t,v as independent before substituting beta(T). [pcc:unverified]

By the shift guards every D evaluated at a baby root is nonzero. Consequently D is a unit in the split baby algebra. The pair jet is (M0+s*M1)/D, and its complete baby product is

Norm(M0+s*M1)/(H_G(x)^2*H_Q(z)^2).

Indeed Norm(D)=product_j(x-x(jG))^2*(z-x(jQ))^2=H_G(x)^2*H_Q(z)^2. Scaling a jet by a nonzero D cannot introduce/remove a root of its constant numerator. The unit, one-common-root and multiple-common-root branches of the field norm remain valid, including a zero constant norm. Only the nonzero denominator is inverted. The target interpolants and signed gamma orientation are unchanged. [pcc:unverified]

This reduces numerator formation before reduction modulo H_G: it avoids squaring L, forming cubic f(beta), the large products D_Q^2 and D_G^2, and the final D factors in E0/E1. The native evaluator still uses classical polynomial multiplication, division and field resultants; exact component counts and full response checks are supplied separately. No fast-resultant or changed asymptotic bound is inferred from this cancellation alone. [pcc:unverified]

Sources checked. Round63 paired-algebra-proof.md specifies E0,E1 and guards; Round64 resultant-proof.md supplies singular-safe first norms; Round66 report fixes matched input/cost boundaries. Bernstein-De Feo-Leroux-Smith, https://arxiv.org/pdf/2003.10118, Lemma4.3 recalls classical biquadratic addition relations; section4.16 warns that extra coordinates complicate higher-dimensional extensions. Those motivate this simplification but do not by themselves give a fast coupled norm. KB tools/wrapper unavailable; no novelty assertion for the classical biquadratic identity. [pcc:unverified]

Fixed-polynomial consequence. After removing the separate 8*y*w*W term, M0 is of degree at most two in each giant x-coordinate x,z. The same is true of M1. Therefore fixed coefficient polynomials C_ij(T,V),J_ij(T,V),0<=i,j<=2, give

M0=sum_ij C_ij(T,V)*x^i*z^j+8*y*w*W,
M1=sum_ij J_ij(T,V)*x^i*z^j.

This follows term by term from F_G*D_Q,F_Q*D_G,N_G*N_Q and N_G*D_Q-N_Q*D_G, since every factor involving x or z has degree at most two in that coordinate. Once V=beta(T) is fixed for a public target, reduce those18 coefficient polynomials modulo H_G once and cache at most18*b coefficients. Subsequent numerator formation needs only a fixed number of scalar linear combinations, hence O(b) field operations per giant block, plus the norm work. The gamma term costs another length-b scalar update. This is a derived cache design, not an implemented result in this round; its setup, storage, independent coefficient identity and actual counts are the next qualification. [pcc:unverified]
