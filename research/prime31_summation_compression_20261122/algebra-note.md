# Dickson-fibre S3 norm gate: exact algebra and the remaining complexity gap

Research note for the current prime-field summation-polynomial iteration. This is a derivation and validation gate, not a novelty or ECDLP speedup claim. [pcc:unverified]

## 1. Exact S3 polynomial

Work over a field K of characteristic different from 2 and 3. Let E: y^2=f(x)=x^3+A x+B be nonsingular, and let R=(r,s) be an affine K-rational point. Write

q_x(Y)=S3(x,Y,r)=d(x)Y^2-T(x)Y+U(x),

where d(x)=(x-r)^2, C(x)=(x+r)(xr+A)+2B, T(x)=2C(x), and U(x)=(xr-A)^2-4B(x+r).

The polynomial identity T(x)^2-4d(x)U(x)=16f(x)f(r)=16f(x)s^2 holds. For x != r and a lift P=(x,y), the two roots are x(P+R) and x(P-R), with multiplicity. A valid S3 zero yields a signed decomposition of R after signs are recovered and checked with group arithmetic. A relation extractor must retain multiplicities of factor-base indices and combine repeated indices with their signs.

## 2. Full-fibre lift lemma

**Lemma.** Assume s != 0. If x,z are in K and S3(x,z,r)=0, then both x and z are abscissae of points of E(K). Moreover there exist finite P,Q in E(K), with x(P)=x and x(Q)=z, such that P+Q=R.

**Proof for x != r.** Since z is a K-root of q_x,

(2d z-T)^2=16f(x)s^2.

Thus y=(2d z-T)/(4s) satisfies y^2=f(x). The two roots of q_x are x(P+R),x(P-R) for either lift P. Selecting the sign of P and of the resulting point gives P+Q=R. These points are finite because x != r. This includes f(x)=0: the quadratic has a double root and P is rational 2-torsion.

**Proof for x=r.** Substitution gives q_r(Y)=-4s^2(Y-x(2R)). Therefore z=x(2R), and (-R)+(2R)=R supplies the decomposition. Doubling is finite because s != 0. If 3R=O, z=r; this double use of one abscissa must not be discarded.

No independent quadratic-residue filter is needed for an S3 solution at a non-2-torsion rational public target.

**Uniform explicit witnesses, including the diagonal.** The exact centered identity is

S3(x,z,r)=([(x-r)(z-r)-f'(r)]^2-4s^2(x+z+r)).

For any zero with s != 0, define lambda=([(x-r)(z-r)-f'(r)])/(2s). Then lambda^2=x+z+r and

f(X)-(lambda*(X-r)-s)^2=(X-r)(X-x)(X-z).

Consequently P=(x,lambda*(x-r)-s) and Q=(z,lambda*(z-r)-s) are finite rational curve points and P+Q=R. The polynomial identity handles all intersection multiplicities, including x=r, x=z, and the order-three tangent case x=z=r. This supplies deterministic signs using one inverse of 2s per target, with no square roots and no inverse of x-r. A separate group-arithmetic verification remains desirable. This is a constant-factor extraction improvement, not an exponent claim. [pcc:unverified]

**Exclusions.** For s=0, q can have a rational double root when f(x) is nonsquare, corresponding to quadratic-twist points. An arbitrary full fibre then needs a lift filter. R=O has no affine r; handle P+Q=O separately. A nonzero point in a prime-order subgroup of odd order has s != 0, but that subgroup condition must be established for the benchmark curve.

**Composition consequence.** Let F be the product over any finite rational x-set, and let H be the product over its liftable subset. On every regular S3 quadratic with a lifted x and rational R, testing F at the roots is equivalent to testing H there. For s != 0 one may also use full F as the outer x-set: S3 itself removes nonliftable roots. Consequently the inner full-fibre membership polynomial may retain its composition structure even when the filtered H does not.

## 3. A split squarefree Dickson fibre at p=2^31-1

Define D_1(X)=X and D_(2n)(X)=D_n(X)^2-2. Then D_n(t+t^-1)=t^n+t^-n. Let N=2^k and F_k(X)=D_N(X)-c.

The norm-one torus in F_(p^2)^* has order p+1=2^31. Choose eta of exact order 2^31, certified by eta^(2^30)=-1. For odd e and 1<=k<=29, set c=eta^(eN)+eta^(-eN). Then c lies in F_p, c != +/-2, and F_k splits squarefree into N F_p roots

x_j=eta^(e+j*2^(31-k))+eta^(-e-j*2^(31-k)), 0<=j<N.

Equality of two expressions t+t^-1 implies equality or inversion of the t values. The coset eta^e*mu_N is disjoint from its inverse for odd e and k<=29, proving distinctness. This construction uses a multiplicative torus, not a proper subfield of F_p.

k=30 gives c=-2 and repeated roots; k=31 gives c=2. Reject these boundaries. In general require p not dividing N, c != +/-2, and certified splitness. The actual number of liftable x roots must be counted; a half-density expectation is not an exact statement for every curve/fibre.

A fixed p has no asymptotic limit. An asymptotic statement must specify a growing family of primes with suitable 2-adic divisibility of p+1, growing N, representation and setup costs. Do not assume infinitely many Mersenne primes. The size and availability of admissible primes is part of a uniform asymptotic claim.

## 4. Quadratic quotient recurrence

Let the outer ring be A=K[X]/H, with H squarefree and no root equal to r. H may be the full F_k with X-r removed, or the filtered lifted factor-base polynomial. Since d is a unit, write q/d=Y^2+beta Y+gamma with beta=-T/d and gamma=U/d.

In A[Y]/(Y^2+beta Y+gamma), represent D_(2^j)(Y)=a_j+b_jY. Initialize (a_0,b_0)=(0,1). Then

a_(j+1)=a_j^2-gamma*b_j^2-2,

b_(j+1)=2*a_j*b_j-beta*b_j^2.

After k steps F_k(Y)=(a_k-c)+b_kY. Its norm is

N_k=(a_k-c)^2-beta*(a_k-c)*b_k+gamma*b_k^2.

Candidate x values are roots of gcd(H,N_k). This replaces degree-N Horner evaluation with k squaring stages. The full-fibre lift lemma justifies using inner F_k instead of filtered H. Handle x=r separately, since normalized quotient division is invalid there.

## 5. Denominator-free norm recurrence and proof

Initialize Q_0=d, T_0=2C, U_0=U, and compute

Q_(j+1)=Q_j^2,

T_(j+1)=T_j^2-2U_jQ_j-4Q_j^2,

U_(j+1)=U_j^2-2T_j^2+4U_jQ_j+4Q_j^2.

Define G_k=U_k-cT_k+c^2Q_k.

**Identity.** G_k(X)=Res_Y(F_k(Y),S3(X,Y,r))=product over F_k(z)=0 of q_x(z), with roots counted with multiplicity.

**Proof.** Away from d=0, let z_1,z_2 be the q roots. Put v_i=D_(2^j)(z_i), trace t=v_1+v_2, and norm u=v_1v_2. Under v_i -> v_i^2-2 these evolve as

t'=t^2-2u-4 and u'=u^2-2t^2+4u+4.

With Q_j=d^(2^j), T_j=Q_j*t and U_j=Q_j*u, the displayed homogeneous recurrence follows. Its initial values are correct because z_1+z_2=T/d and z_1z_2=U/d. Thus

G_k=d^N(D_N(z_1)-c)(D_N(z_2)-c),

the resultant wherever d != 0. Both sides are polynomials, so equality extends identically to d=0. Using F_k as the first, monic polynomial fixes the product formula and its sign through a degree drop of q. Before specialization the orders can be swapped because 2N is even; after specialization do not silently redefine the resultant using q's lower degree.

**Diagonal.** If s != 0, then q_r(Y)=-4s^2(Y-x(2R)), and hence

G_k(r)=(4s^2)^N F_k(x(2R)).

This handles the tangent branch without denominator saturation or spurious zeros. If s=0, nonsingularity gives f'(r) != 0 and q_r(Y)=f'(r)^2, so G_k(r)=f'(r)^(2N) != 0.

By induction deg Q_j,deg T_j,deg U_j <= 2^(j+1). The final degree is at most 2N. Reduction modulo outer H after every operation preserves the exact final residue. The small number of recurrence layers does not imply bounded polynomial degree or support.

## 6. Required strongest algebraic and group controls

Let the outer ring have dimension M and let C(M) count modular polynomial multiplication including reduction. A uniform safe upper bound for the recurrence is O(log N*C(M)), plus setup, gcd, root isolation, group-sign recovery and verification. This bound is loose when M is comparable to N: the state degree at layer j is at most min(M-1,2^(j+1)), so the work before quotient saturation is a geometric sum. With trimmed schoolbook polynomials and monic reduction, a sharper kernel bound is O(M^2*(1+log_2(N/M))) for 1<=M<=N, with constant adjustments for initial degree-two inputs. In particular M=Theta(N) gives O(N^2), not an intrinsic O(N^2 log N) cost. With standard degree-sensitive fast multiplication and prepared reduction, the analogous bound is O(Mult(M)*(1+log_2(N/M))) under the usual geometric-sum property of Mult. Setup, gcd and root recovery remain separate and can add logarithmic factors or dominate.

Naive expanded inner-polynomial Horner costs O(N M^2); the recurrence improves that particular weak symbolic control. However, the known-root product is stronger: each q_x(y), for fixed y in the full fibre, has degree at most two in x. Multiplying N factors successively modulo H costs O(NM), because each multiplication and the at-most-two monic reduction steps cost O(M). For M comparable to N both this product and the schoolbook norm recurrence have quadratic kernels. One must compare the measured constants instead of claiming an exponent difference from the loose upper bounds. A balanced fast product tree has an O(Mult(N) log N) safe bound, often sharper for superlinear multiplication, and shares the quasi-linear exponent of a fast recurrence. These controls are required; beating cubic Horner alone is insufficient. [pcc:unverified]

The existing group-complement scan evaluates R-P for the M factor-base points and checks membership in O(M) group operations plus the appropriate hash/lookup work. It must remain the principal relation-search control. Its setup, batch inversion, signed normalization and memory costs must be counted consistently. The composed recurrence has no established exponent advantage against this scan.

An explicit quotient-ring element has M coefficients. Generic dense support therefore imposes Omega(M) space/work merely to emit that representation. The recurrence could remain a small arithmetic circuit, but no sublinear common-root solver for those circuits has been supplied.

## 7. Precisely missing compression theorem

The new mathematical target is to intersect the succinct F_k and G_k circuits in expected O(N^(1-epsilon)*polylog(p)+output) work for a fixed epsilon>0, including per-target setup and charged amortized preprocessing, without enumerating the N roots or materializing M=Theta(N) coefficients. The theorem must specify curve/fibre/target distributions, zero and pole handling, witness extraction, and success probability. Obtaining the O(log N)-layer resultant circuit proves none of this. [pcc:unverified]

A procedure that only evaluates candidates individually has an Omega(N) classical query lower bound in the unique uniformly hidden-root oracle model. This is only an obstruction for that black-box method, not an impossibility theorem for algorithms exploiting the concrete symbolic structure.

For perspective, under the usual independent uniform relation-yield heuristic, an m-summand factor base of size B, decomposition cost B^theta per target, and B^2 linear-algebra cost lead to a balance between n*B^(theta+1-m) and B^2. In the small-yield regime where this model applies, the optimized exponent is 2/(m+1-theta). Beating n^(1/2) requires theta<m-3. Thus even a very fast two-summand oracle does not by itself establish a complete rho-beating algorithm under that architecture. This is a scoped heuristic cost calculation, not an impossibility theorem for other relation systems or large-prime methods.

## 8. Bounded correctness and support-growth gate

Implement in Rust, comparing the same complete normalized signed relation sets against: independent exhaustive group addition on small fields; strongest direct group-complement scan; filtered-H Horner; and product over known full-fibre roots. Compare the denominator-free resultant coefficients with the product coefficients, rather than relying only on matching relation counts.

Cover ordinary targets, planted P+Q targets, diagonal x=r targets, repeated factor-base indices, an order-three tangent instance on a correctness-only curve, rational 2-torsion targets as excluded-case counterexamples, and c=+/-2 fibres rejected for repeated roots. Use independent group arithmetic for relation verification. At p=2^31-1 distinguish planted correctness instances from unconditioned natural yield; planted success says nothing about natural relation probability.

At every k and recurrence layer record nonzero support after reduction, maximum degree before reduction, ring multiplication/reduction counts, setup and inversion costs, root recovery costs, peak allocated bytes, and elapsed time. The explicit falsification test is whether generic support reaches Theta(M) and complete cost exceeds the strongest direct and sparse-product controls. Lower equation degree or fewer symbolic instructions does not pass the speedup gate. [pcc:unverified]

Novelty status: the repository already contains discriminant factorization, oriented S3, quotient-gcd membership, Dickson/torus fibres and fast-composition discussions. The exact full-fibre lift lemma preserving inner composition and denominator-free diagonal handling are the proposed narrow validation delta, subject to the prior-work audit. No asymptotic relation-search or complete ECDLP speedup has been established here. [pcc:unverified]
