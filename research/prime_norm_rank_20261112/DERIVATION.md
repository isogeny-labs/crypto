# Prescribed norms: a reduction, not a completed algorithm

Status: written mathematical derivation and cost audit. The generic polynomial
identity and finite relaxation counterexample are Lean-checked separately;
this entire divisor/probability reduction is not formalized in Lean. Producer
standing remains unsigned [pcc:unverified].

## 1. Domain and online factor base

Let p>3 be prime, E/F_p: y²=f(x)=x³+ax+b nonsingular, and P have known odd
prime order r>=5. Let Q in <P> be nonzero. The cryptographic regime has
log p=O(log r); put n=ceil(log2 r). No special trace, embedding degree, curve
coefficient or torsion condition is imposed. Choose k=n+kappa independent
uniform labels u_i in Z/r and compute R_i=[u_i]P after receiving Q. All point
generation is charged online and no reusable/precomputed table is assumed.
Reject zero points, duplicate x fibres and fibres equal to Q's. This requires
only public curve operations/comparisons, not Q's logarithm. Assume henceforth
that the retained x_i and x_Q are all distinct. Their y coordinates are nonzero,
since the subgroup has odd order. For sufficiently large r, k is far below
the available number of distinct fibres [pcc:unverified].

A signed relation sum_i epsilon_i R_i=Q immediately yields the logarithm
sum_i epsilon_i u_i modulo r. The group calculation verifies the recovered
scalar. This is an online reduction using known labels, not a precomputation
speed claim. The unknown step is constructing the signs [pcc:unverified].

## 2. Exact norm representation of the sign witness

Let F(X)=product_i(X-x_i), G(X)=F(X)(X-x_Q), d=k+1. Seek A,B in F_p[X]
and c!=0 satisfying

    A²-f B²=cG,
    A(x_Q)-y_Q B(x_Q)=0,
    deg A<=floor(d/2), deg B<=floor((d-3)/2).

For a witness define h=A+By. The second equation makes h vanish at -Q.
The squarefree norm cG forces exactly one zero above every other x_i: if both
branches vanished, A and B would both vanish there and the norm would have a
repeated root. Every finite zero is accounted for. At O, x and y have pole
orders2 and3. The possible top degrees2deg A and3+2deg B have opposite parity,
so they cannot cancel. Since the norm has degree d, h has pole order d.
Consequently

    div(h)=sum_i(epsilon_i R_i)+(-Q)-d(O).

Here epsilon_i R_i denotes the point R_i or -R_i, not a signed divisor
coefficient. The genus1 Abel-Jacobi/group-law correspondence gives
sum_i epsilon_i R_i=Q. Recover signs by evaluating h at each branch and
checking the scalar. No unpriced root-factorization oracle is needed once
A,B are supplied [pcc:unverified].

Conversely, a signed relation makes that degree-zero divisor principal over
F_p. Its function is regular on the affine curve and belongs to L(dO), hence
has the form A+By with exactly the stated pole/degree bounds. Its norm has the
simple roots of G and no others, giving cG for c!=0. This uses standard
function-field/divisor theory; the representation itself is not a novelty
claim. See Galbraith7.7.12, the proof of7.7.11,7.9.8--7.9.9 and8.4
[pcc:unverified].

## 3. Relation existence is not witness-search speed

Temporarily analyze unfiltered independent labels. Write Q=[z]P with z!=0;
z is used in the proof, not supplied to the algorithm. Let S=2^k and X count
sign vectors epsilon with sum epsilon_i u_i=z. For k>=1,

    E[X]=S/r,
    E[X²]=S/r+S(S-2)/r²,
    Var[X]=S(r-2)/r².

Each signed sum is uniform. Distinct nonopposite sign rows are linearly
independent over F_r, so their joint target probability is1/r². Opposite
rows cannot both hit nonzero z because r is odd. Chebyshev gives

    Pr[X=0] <= (r-2)/2^k.

A union bound for zero points, duplicate ±fibres and ±Q gives bad-base
probability at most(k²+2k)/r. Thus a fresh sample is admissible and has a
norm/sign witness except with probability at most

    (r-2)/2^k+(k²+2k)/r <= 2^(-kappa)+(k²+2k)/r.

For kappa=O(log n) and large r this is small. These are written probability
derivations, not measured success rates or a solver experiment
[pcc:unverified].

Crucially, a random tested sign still hits Q with probability1/r on the
unfiltered distribution. Many witnesses do not yield a fast way to find one.
A decision routine that says yes on nearly every dense instance conveys no
witness. Search self-reduction needs correctly handled conditioned subproblems,
not just high yes-rate accuracy on full random bases [pcc:unverified].

## 4. The rank-one formulation

L(dO) has dimension d. Put all A/B coefficients in w in F_p^d. The branch
condition is a nonzero linear form ell(w)=0. Lift to a symmetric matrix M:
replace w_i w_j by M_ij. Norm coefficients become structured linear equations
L(M)=cG, and impose M ell=0. A true witness gives M=w w^T. The missing condition
is M!=0 and rank(M)=1, not a positivity condition over a finite field.
Conversely, any nonzero symmetric rank-one M over F_p has the form lambda*w*w^T
with lambda!=0; the free nonzero c absorbs lambda, and M ell=0 forces ell(w)=0.
Thus rank-one completion, with the nonzero norm coefficient, is the actual
search problem [pcc:unverified].

Linear solving without rank one is insufficient. With two supports, normalize
B=1 and A=v(X-q)+y, where Q=(q,y). Set U=q+x1+x2. The linearized constraints
are

    2*y*v=q²+a+q*U-x1*x2,

with U treated as an independent square variable. The exact remaining norm
defect, assuming Q is on the curve and that linear constraint, is

    A²-f+(X-x1)(X-x2)(X-q)=(v²-U)(X-q)².

The norm equation holds iff v²=U. This equivalence is Lean-checked over every
commutative coefficient ring (the monic quadratic detects the coefficient).
The full unconditional identity also retains the curve and linear defects;
identity record IDC1h8b0c962f231d89d5 binds its expression and formal source
[pcc:unverified].

For E/F17: y²=x³+2x+2, use R1=(5,1),R2=(6,3),Q=(0,6). Lean checks
nonsingularity, those point equations and18 affine points plus O=19.
The linearized solution has U=11,v=9,W=1, but v²-U=2. The Gram matrix
[[11,9],[9,1]] has determinant15, so it is not rank one. The actual norm
residual is2X², not zero. The linear constraint has unique v=9; Lean therefore
rules out every normalized slope satisfying that constraint on this fixture.
This falsifies the linear relaxation as a complete solver, not the full norm
approach or the existence of sub-exponential ECDLP algorithms
[pcc:unverified].

## 5. Full bit-cost requirement

For k=Theta(n), public point generation, F/G construction, witness verification,
sign extraction and final scalar verification all have polynomial bit cost.
The implicit norm instance already has Theta(k log p)=Theta(n²) coefficient
bits; a generic claim of sub-exponential time in that larger input length is
insufficient. The needed constructive witness cost is2^{o(n)} bit operations
on these curve-derived instances, with success and failures charged.

If a complete witness solver had uniform cost T_norm(k,n) on admissible bases,
the reduction would cost polynomial(n) plus O(T_norm(k,n)) expected work when
the above failure bound is bounded below1. No such T_norm=2^{o(n)} solver is
constructed here. That assumption is the essential missing breakthrough,
not a theorem discharged by Gaussian elimination [pcc:unverified].

The direct sign scan costs2^k polynomial(n), hence remains exponential.
Lower algebraic degree, a polynomial-size representation, a feasible relaxed
matrix or abundant solutions does not establish a smaller exponent. No new
empirical campaign, target solve, timing, precomputation amortization or universal
nonexistence conclusion is claimed. The result of this search is a precise
representation/solver contract and a rejected shortcut, not a novel algorithm.
