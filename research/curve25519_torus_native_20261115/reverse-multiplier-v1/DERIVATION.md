# A target-dependent multiplier fibre

Status: written construction for qualification, not a native solver, formal
theorem, novelty clearance or attack-cost result. Original source and route
criteria remain frozen. The target-dependent object here differs from the
received-ideal universal equation: the public target ordinate/abscissa enter
its constraint and determine which multipliers are candidates. [pcc:unverified]

## 1. Original family, with an explicitly variable base

Use p=2^255-19, E:v²=u³+A*u²+u, A486662, Fp2=Fp[sqrt(2)], norm-one T.
Retain original h of order a*q, a60824497,q213156431,K20633. The raw indexed
parameter set is S={h^(j+a*t):1<=j<=K,0<=t<q}; actual cardinality qK assumes
the recorded exact order, K<a. No full enumeration is required for this note.
For a multiplier sigma in T define S_sigma=sigma*S. These are DIFFERENT
parameter bases unless their stabilizer identifies them. [pcc:unverified]

Fix template parameters alpha_i in S, with raw labels known as public template
constants, not recovered hidden target labels. Solve for sigma. Do not supply
a planted sigma or a candidate multiplier list to the query. A returned row
belongs to the shifted base S_sigma, not automatically to the unshifted
original base or its current Circuit. A shifted-map compiler/projection is a
new fidelity obligation. [pcc:unverified]

## 2. Two-leaf candidate equation

Use Cayley coordinate c in Fp:

    sigma(c)=(1+c*sqrt(2))/(1-c*sqrt(2))
            =((1+2c²)+2c*sqrt(2))/(1-2c²).

Its missing projective point is sigma=-1 at c=infinity. For alpha_i=(a_i,b_i),
norm(alpha_i)=1, put

    V=1-2c²
    U_i=a_i(1+2c²)+4b_i*c
    tau_i=real(alpha_i*sigma)=U_i/V
    N_i=A*(epsilon_i*V-U_i), D_i=2U_i, epsilon_i in{+1,-1}
    u_i=N_i/D_i.

Each N_i,D_i has degree at most2. Keep V,U_i nonzero guards; reject malformed
norm/canonical inputs. The actual fixed fields have2 nonsquare; this does not
justify dropping explicit guards from a compiler. Rationality of the raw
ordinate and nonidentity eightfold image still need checking. [pcc:unverified]

For finite raw target R=(r,s), Montgomery's classical third summation
polynomial is

    f3(x,y,r)=(x-y)²*r²
             -2*((x+y)*(x*y+1)+2A*x*y)*r+(x*y-1)².

Let Z=D_1*D_2. Clearing denominators gives the proposed numerator

    P(c)=(N_1*D_2-N_2*D_1)²*r²
         -2*((N_1*D_2+N_2*D_1)*(N_1*N_2+Z)
              +2A*N_1*N_2*Z)*r
         +(N_1*N_2-Z)².

Every displayed product has degree at most8 in c. There are four branch
choices epsilon_1,epsilon_2. No q-power/window polynomial appears in this
candidate because sigma is free and the two relative labels were fixed.
That is the mechanism and also the change in the original search problem.
[pcc:unverified]

Next qualification must check this clearing identity and the raw group-law
bridge, not assume them from a degree count. A nonzero P has at most8 finite
roots per branch; identically zero P is DegenerateFamily/unsupported and
must not become an absence claim. Include the infinity multiplier separately.
Root candidates require rational ordinate extraction, all signs, exact sum
P_1+P_2=R, nonzero cofactor images and raw label/index replay relative to sigma.
The Semaev equation alone only supplies algebraic-closure ordinate candidates.
[pcc:unverified]

For public nonidentity Q in the prime subgroup, compute public R=[8^-1 mod ell]Q.
Completeness for [8](sum P_i)=Q requires all8 rational torsion translates R+T,
T in E[8](Fp), or an explicitly narrower admitted sector. No hidden target
scalar is needed for this public multiplication. Identity targets require
their own exact branch and cannot be counted as a negative finite-root case.
[pcc:unverified]

## 3. Six leaves and the source of the apparent cheapness

Semaev f_(m+1) has degree2^(m-1) in each x_i. Substitution of degree2
numerators/denominators and clearing yields the written upper bound

    degree_c <= 2*m*2^(m-1) = m*2^m.

Thus m6 gives384 per branch; forming the summation polynomial/resultants,
factoring, filtering, all64 branch choices and torsion sectors are still real
work. This is a fixed-arity degree bound, not a total-cost/solving-degree
measurement or a six-leaf implementation. Zero polynomials, chart poles,
extension-field-only roots and incomplete factorization stay explicit.
[pcc:unverified]

The fixed-base problem has not acquired such a bound. Requiring a prescribed
sigma or one prescribed coset class reintroduces the original selection
constraint. Treating a fresh base for each query as reusable precomputation
would silently move the indexing and logarithm problem. FAMILY_GATE.md gives
the exact shared-column/recovery discriminator. [pcc:unverified]

## 4. A separate possible degree-form sharpening

On the old draft's smooth multiquadratic cover, assume the elliptic maps Phi_i
and independent involutions iota_i really have the asserted degrees and
sign-flip action. Pullback degree is a quadratic form d on maps modulo
constants, invariant under domain automorphisms. For i!=j, composing by
iota_i sends Phi_i+Phi_j to -Phi_i+Phi_j, so the corresponding degrees agree.
The parallelogram law then makes the degree cross term vanish. Bilinearity
of the polarization gives d(sum n_i Phi_i)=sum n_i²*d(Phi_i).
[pcc:unverified]

This conditional argument uses classical quadratic-form/cube laws. It may
replace an L1-square collision estimate by a coefficient-square estimate;
it supplies no coefficient solver and no new exponent. Construction of the
smooth cover, its maps/degrees and fidelity to actual branch selection remain
the old geometric obligations. It is a secondary proof audit, not a reason
to call theory-v1 proved or to rewrite its frozen text. [pcc:unverified]
