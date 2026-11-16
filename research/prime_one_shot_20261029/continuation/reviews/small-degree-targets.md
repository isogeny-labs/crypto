# Independent review: many positive targets have small quotient degree

2026-10-30 UTC. Supporting mathematical consequence only; no solver construction, experiment or performance claim.

The proposed conclusion and constants are correct, provided the ideal is the selected norm system with split-separable quotient membership and uniquely bounded auxiliary witnesses. It applies on bases simultaneously satisfying the earlier moment, coverage and rationality conditions. It does not bound the work required to compute an initial Gröbner basis.

## Counting bound

Write a=(m!r)^(1/m). If d is the least power of2 with d>=2a, then 2a<=d<4a, including when 2a itself is a power of2. Hence

    kappa=d^m/(m!r)<4^m.

On a moment-good auxiliary base, the reviewed inequalities give

    nu_g=(1/r)sum_R V_g(R)^2 <=4K,
    K/kappa²<=5m4^m,
    nu_g<20m64^m.

The selected-chart count satisfies 0<=V'_g(R)<=V_g(R). Consequently for L=128m32^m,

    Pr_R[V'_g(R)>L]
      <= E_R[V'_g(R)^2]/L²
      <= nu_g/L²
      <20/(16384m16^m)
      =(25/64)rho0,

where rho0=1/(320m16^m). The coverage theorem gives Pr[V'_g(R)>=1]>=rho0. Therefore

    Pr_R[1<=V'_g(R)<=L] >=(39/64)rho0>rho0/2.

The parent's rho0/2 statement is conservative. This is Markov applied to the squared tuple count, rather than an unproved assertion that the individual count is close to its mean. Targets outside the selected-chart domain, including O, have count0 and are already included in the uniform-target denominator r.

## Why the count equals geometric quotient degree

Fix an admissible nonzero rational target R and the accepted auxiliary base. Each selected signed unordered m-tuple of residual points has distinct abscissas and excludes x(R). Its divisor

    sum_i (P_i)+(-R)-(m+1)(O)

is principal precisely because sum_i P_i=R. For odd m=2h-1, its unique defining function up to scalar has the form A(X)+C(X)y with deg A=h and deg C<=h-2. The even leading pole forces a nonzero leading coefficient of A, so making A monic fixes that scalar uniquely. Its norm is exactly

    A²-fC²=(X-x(R))U,

with monic U the product of the residual linear factors. Thus an unordered tuple gives exactly one core point of the norm scheme. Conversely a selected core solution recovers the signed points by y_i=-A(x_i)/C(x_i), independently of the order in which U's roots are listed. There is no extra m! or global-sign quotient factor.

The chart conditions exclude a common zero of A,C at any residual root, and the target linear equation chooses the required target sign. Split separable membership makes U a squarefree divisor of the known factor-base polynomial. On a rationality-good base every geometric selected solution has rational residuals and coefficients, so its geometric points are counted by the rational signed tuples V'_g(R); no extra extension-field solutions remain.

The earlier reducedness argument also applies here. For fixed U, N=(X-x(R))U is squarefree and coprime to f. Thus gcd(A,fC)=1. A tangent vector satisfies A deltaA=fC deltaC, forcing A to divide deltaC; the degree bounds force deltaC=deltaA=0. The membership scheme itself is finite étale. Bounded monic-division quotients, inverse remainders and degree-bounded normalized Bézout witnesses are unique graph coordinates, so they preserve this property. The target condition selects components of the finite étale scheme. Its geometric quotient length is therefore its number of geometric points, exactly V'_g(R).

This conclusion would not automatically hold for a terminal-Graeffe membership scheme with nilpotents, an unsaturated chart admitting degeneracies, arbitrary free slack variables, or nonunique Bézout witnesses. The reviewed quotient/remainder construction with the declared bounds avoids those issues.

## Algorithmic implication and remaining gap

For balanced m=O(sqrt(n)), n=log2 r, one has log2 L=7+log2 m+5m=O(sqrt(n)+log n). The selected quotient dimension is therefore subexponential on at least rho0/2=exp(-O(m)) of all targets on each simultaneous good base. With t=O(sqrt(n)+log n), the O(mt) variable count is polynomial in n.

Once a suitable initial Gröbner basis or multiplication representation of this zero-dimensional quotient is available, standard FGLM and finite-field factorization/root extraction have cost polynomial in the quotient dimension, variable count and log p, and hence are subexponential on these inputs. Expected polynomial-time finite-field factorization suffices; no new root-extraction theorem is needed. Since all geometric points here are rational and the algebra is reduced, its coordinate algebra is a product of copies of Fp.

Small final quotient dimension does not control the degrees or sizes of intermediate elimination computations, the cost of obtaining the first Gröbner basis, or the work needed to identify these targets in advance. The result is a rigorous bound on a sufficiently large set of positive inputs. It supplies neither a bounded-time relation oracle nor a small-solving-degree theorem.
