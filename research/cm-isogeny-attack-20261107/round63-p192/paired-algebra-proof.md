# Signed baby algebra for the first spectral coefficient

Derived construction, 2026-11-08, T-1945. This proof specifies a public point-input constructor, a paired B-only contraction, and a signed degree-b norm algebra. Exact native counts and declared cases are recorded separately. All scientific claims remain unverified producer evidence. [pcc:unverified]

Domain. Work on E/F_p:y^2=x^3+a*x+b0 with p>3 and rational subgroup <G> of prime order r>6, r!=p. Let Q=[k]G, k not0,+1,-1. Choose P over F_(p^2) with2P nonrational and U in <G>. The desired jet is

F(s)=product_(j=0..r-1)(x(P+jG)-x(P+U+jQ)+s)=A+s*B modulo s^2.

Its coefficient B is the oracle used by the calibrated seven-shift decoder of Round62. No scalar k is supplied to the new constructor or contraction. Known labels only create small fixtures and check the returned point afterward. [pcc:unverified]

Coverage and point generation. Set b=max(1,floor(sqrt(r))/2), m=2b+1 and n=ceil(r/m). Use giant centers c_i=b+i*m and signed babies j in[-b,b], retaining only0<=c_i+j<r. The ith interval is[i*m,(i+1)*m-1], clipped at r-1. The intervals are disjoint and cover0..r-1 exactly. The final interval is retained even when its center exceeds r-1; then only negative babies contribute. The native checker explicitly marks every covered index and checks2*(paired factors)+(single factors)=r. [pcc:unverified]

For a supplied public generator T, retain0,T,...,bT and c_i*T. Starting from the input T, babies cost b-1 addition calls; mT=2*(bT)+T costs two; the remaining n-1 giant centers cost n-1. Thus the builder uses exactly b+n calls, counting doubling and identity calls, and retains b+1+n point slots, including the identity and a duplicate bT across the two arrays. Source G data is reusable; target Q data is built once per changing target. The reference harness additionally stores complete r-point graphs, so this is a component-space count, not the entire qualifier's memory. [pcc:unverified]

Signed pair. For A=(x,y) and T=(t,z), let

D_A=(x-t)^2,
N_A=t*x^2+(a+t^2)*x+a*t+2*b0,
H_A=N_A/D_A, K_A=2*y*z/D_A.

Then x(A+T)=H_A-K_A and x(A-T)=H_A+K_A. All denominators are nonzero: a nonrational translate of P cannot equal plus or minus a rational baby. The implementation inverts D_A once and reuses it for H_A and K_A. For A=P+c_iG and B=P+U+c_iQ, put L0=H_A-H_B and J0=K_A-K_B. The two uniform spectral factors have jet

(L0-J0+s)(L0+J0+s)=(L0^2-J0^2)+s*(2*L0).

A center or unpaired tail contributes(d,1). Jet products use(A,B)*(a,b)=(A*a,B*a+A*b), with no inversion of a norm factor. A zero constant term is therefore retained correctly. [pcc:unverified]

Relative-sign negative control. Replacing J0=K_A-K_B by K_A+K_B changes the constant pair product by4*K_A*K_B. All four translated y-coordinates are nonzero in the declared domain: a two-torsion translate would make2P rational, while a nonzero odd-order rational baby cannot have y=0. Thus every executed pair rejects this wrong-sign product. This is a factor-level control; it is not a claim that every wrong-sign complete zero-order norm differs. In fact the complete Round60 base norm is identical for Q and -Q. [pcc:unverified]

A signed baby algebra. For j=1,...,b, let t_j=x(jG), u_j=x(jQ), w_j=y(jG)*y(jQ). Since b<r/2, the t_j are distinct. Build

H(Z)=product_j(Z-t_j), beta(t_j)=u_j, gamma(t_j)=w_j,

with beta,gamma of degree<b. These are public interpolants over F_p. The gamma term is invariant under simultaneously negating both babies and changes sign when Q is negated. Keeping beta alone would lose that relative sign. The native constructor uses explicit Lagrange interpolation and charges its arithmetic; it is quadratic in b and is not assigned a fast product-tree bound. [pcc:unverified]

Norm polynomial for one complete paired block. In K[Z]/H, write T=Z,V=beta(Z),W=gamma(Z), f(X)=X^3+a*X+b0, and use the known giant coordinates A=(x,y), B=(z,w). Put

D_G=(x-T)^2, D_Q=(z-V)^2, D=D_G*D_Q,
N_G=T*x^2+(a+T^2)*x+a*T+2*b0,
N_Q=V*z^2+(a+V^2)*z+a*V+2*b0,
L=N_G*D_Q-N_Q*D_G.

The paired jet numerator is

E0=L^2-4*y^2*f(T)*D_Q^2-4*w^2*f(V)*D_G^2+8*y*w*W*D,
E1=2*L*D.

For each baby root, (E0+s*E1)/D^2 is exactly the signed pair jet. Hence the block product equals

Norm_(K[Z]/H)(E0+s*E1) / Norm_(K[Z]/H)(D^2).

The denominator is a nonzero constant. The numerator norm may have zero constant term, and its first coefficient must be preserved. This is a concrete coupled norm of dimension b, not an assumed univariate elliptic resultant in a single x-coordinate. The same target beta,gamma are reused over giant blocks and all queries. [pcc:unverified]

Singular jet determinant. Multiplication by E0+s*E1 in the power basis1,Z,...,Z^(b-1) gives a b-by-b matrix over K[s]/s^2. Its determinant is the norm. The implemented reference uses full row/column pivoting on nonzero constant entries only. If the remaining constant matrix vanishes with dimension at least two, its determinant is divisible by s^2 and is zero in this ring. If dimension one remains, multiply the last (possibly nonunit) diagonal jet into the prior pivot product without inverting it. This handles a simple marked zero. Every actual inverse is guarded by a nonzero constant assertion. Dense matrix construction and elimination are charged; no near-linear resultant complexity is imported. [pcc:unverified]

Costs and limits. The stream evaluates r underlying factors as paired or singleton jets using O(r) field work and O(sqrt(r)) point slots after constructor inputs. The baby-algebra constructor uses O(b^2) field work. The supplied dense norm reference costs O(b^3) per complete block, including power-basis multiplication matrices and elimination, hence O(n*b^3) for a query at this implementation level. Scalar-fixture generation, full-graph references, cubic decoding, output comparisons and fresh replays are additional work. Exact counters cover K multiplications/inversions and group-add calls; their definitions and scopes are retained in receipts. This step reduces explicit point generation and gives the coupled polynomial interface; it does not claim square-root total field work. [pcc:unverified]

Primary-curve scope. The full P192 layout integers are computed without allocating the arrays. Only24 prescribed27-factor blocks are executed, with b4,m9,n3 and U=[j](G-Q) for retained j. Each selected block contains its unique zero by construction. Paired first coefficients agree with independent direct27-factor prefix/suffix contraction. No complete P192 B response or arbitrary-input decoder follows from those containment controls. [pcc:unverified]

Sources checked. Round60 cross-norm-proof.md gives signed paired factors and the graph-dependent transverse coefficient; Round62 B-only-proof.md gives calibration and seven-shift extraction. Bernstein-De Feo-Leroux-Smith, https://arxiv.org/pdf/2003.10118, Lemma4.3 gives classical plus/minus coordinate relations, Lemma4.9 gives the one-curve resultant construction, Example4.13 treats jets, and section4.16 explicitly identifies extra-coordinate/multivariate issues for products of elliptic curves. Those results motivate the retained beta,gamma algebra but do not establish a fast resultant backend for it. KB MCP tools and migrated wrapper are unavailable. No novelty or prior-art absence claim is made for the standard ingredients. [pcc:unverified]
