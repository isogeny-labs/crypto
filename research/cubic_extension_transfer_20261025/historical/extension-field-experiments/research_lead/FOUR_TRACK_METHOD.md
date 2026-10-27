# Coverage, arithmetic invariants, robust transfer, and geometric index calculus

This stage follows four questions: does one extra degree-three step escape the
Serre obstruction; which invariants predict failure; can the transfer handle
exceptional inputs; and how does a stronger target solver compare with a
generic source attack? Results and measured timings are generated separately in
[FOUR_TRACK_RESULTS.md](FOUR_TRACK_RESULTS.md).

## Three distinct coverage claims

The scope is ordinary, non-subfield elliptic curves over K=F_(p³), p>=5, with
prime full group order r>3, p=1 mod 3, r=4 mod 9. A *prepared* model has scalar
minus-identity Frobenius on E[3]. The construction screens its twelve transverse
maximal isotropic kernels. A usable kernel yields the correct base-field
Jacobian rather than its quadratic twist.

1. Immediate coverage on every prepared curve is false: the certified p=109
   example has twelve obstructed candidates.
2. At most one rational degree-three step in total from every input is a
   stronger assertion than allowing an extra step after preparation. The new
   p=157 audit tests this assertion using sources with a unique rational
   degree-three isogeny into an all-obstructed prepared model.
3. One additional rational degree-three step after preparation is a separate
   open question. The neighborhood search certifies every quotient order on a
   candidate failing center and all eligible neighbors before declaring a
   counterexample to this assertion. No finite search proves it universally.

The number of steps refers to this particular transverse-kernel construction;
it is not a lower bound for every cover, kernel degree, or ECDLP algorithm.

## Search and arithmetic data

`obstruction_neighborhoods.py` has two modes. The component mode enumerates all
eligible rational degree-three neighbors, identifying Frobenius-conjugate
j-invariants and recording omitted nonprepared edges. An exhausted queue proves
closure of that eligible component up to the stated identification.

The random mode samples unrestricted short-Weierstrass coefficients, filters
ordinary prime-order inputs, and prepares them when needed. One positive chi18
test rejects an all-obstructed center; a negative center triggers full neighbor
screening. `--target-nine` additionally stratifies toward scalar minus-identity
action on E[9]. These distributions are deliberately biased and potentially
overlapping. They are not uniform samples of isogeny classes or density data.

The final targeted run rotates fields by attempted draw, so a depleted small
field cannot stall all larger fields. An initial run that rotated by accepted
case is retained as `neighborhood_random_nine_initial.json` and is not combined
with the final targeted count.

The number of K-rational x-roots of the ninth division polynomial is 4, 13, or
40 on the prepared models. To see this, write Frobenius on E[9] as -I+3A modulo
9. The kernel of multiplication by 3A on (Z/9Z)² has
3^(4-rank(A mod 3)) elements. All nonzero points have opposite pairs with the
same x-coordinate; scalar +I is incompatible with their nonzero 3-primary
reductions. Thus the number of roots is (3^(4-rank A)-1)/2. The code also
computes an explicit Frobenius matrix over F_(p^18) for flagged examples.

### Why the endomorphism order is also insufficient

The p=109 comparison has q=1295029, trace t=1861, and

    t²-4q = -1716795 = (-2355) * 27².

Here -2355 is the fundamental discriminant. For both a failing and a successful
prepared model, all forty ninth-division x-roots are rational, but only 121 of
the 364 twenty-seventh-division x-roots are rational.

Let pi be q-Frobenius. Scalar -I on E[9] implies (pi+1)/9 is an endomorphism:
an endomorphism annihilating E[9] factors through [9]. This element generates
the order of discriminant -21195=(-2355)*3². Therefore the full endomorphism
ring contains that order. Its only possible larger order is the maximal one.
In the maximal order, (pi+1)/27 is integral: it has trace 69, norm 1779, and
discriminant -2355. That would force scalar -I on E[27], contrary to the root
count. Both curves consequently have endomorphism discriminant -21195.

Exact Hilbert class polynomials provide additional checks: H_(-21195) vanishes
at both j-invariants and H_(-2355) does not. Nevertheless, their coverage
differs. Thus field, source order, this Frobenius data, and the geometric
endomorphism ring do not together determine the twelve-kernel coverage.

`cm_coverage.py` screens all 36 roots of H_(-21195) in K, grouped into twelve
Frobenius orbits. It also tests degree-five and degree-seven isogenies. These
degrees are coprime to three, so they preserve the abstract E[3] Frobenius
module while potentially changing the obstruction. A usable repair target is
independently order-certified. This is a concrete finite class audit, not a
general new coverage theorem.

## Transfer changes

`inverse_map.setup` now searches prepared models in a bounded breadth-first
isogeny walk and composes the actual elliptic-curve isogenies. Failure to find a
usable model within the budget raises an explicit exception.

`transfer_pipeline.solve(E,P,Q,r)` receives no source logarithm. It validates
the subgroup, handles Q=0, Q=P, and Q=-P directly, constructs eight symmetric
sections, and inverts the images of P,Q,P+Q over the base field. A common
nonzero source multiplier moves all three images together when an evaluation
pole or an empty inversion fiber is encountered. Their relative signs are
fixed by addition in the actual target Jacobian.

The recovered effective divisor uses interpolation when its x-coordinates are
distinct. Otherwise, x-fiber factorization and both residue coordinates identify
the correct closed place and its multiplicity. This includes points whose
y-coordinate does not belong to the field generated by x. The targeted checks
cover coincident x with distinct y, a repeated rational point, and a quadratic
closed point with rational x.

The inversion sections still use a generic ordinary tangent chart. Repeated
support on the section base locus or support at infinity can require another
source multiplier. Retry budgets, tangent-chart availability, and rare special
geometric cases remain explicit limitations; no universal chart-coverage proof
is claimed. The repeated-point check validates divisor recovery itself, not
inversion through a section base locus.

## Geometric target logarithms

For a reference rational point B, put f_P=[P-B] and kappa=[K_C-4B]. Every line
section with intersection multiplicities c_P gives the exact relation

    sum_P c_P f_P = kappa.

Keeping kappa as an unknown is essential. It is generally nonzero.

`line_index_calculus.py` uses all rational curve points as a factor base and
collects completely split line sections. These cheaply supply homogeneous
relations. Smooth reduced divisors of aS+bT then anchor the point logarithms
and the unknown logarithm of T. Intersections are checked by exact polynomial
factorization, sample line relations are checked in the actual Jacobian, and
every smooth-divisor decomposition is checked there.

`large_prime_index_calculus.py` chooses a smaller factor base of about
3*sqrt(p) rational points, including B. A line through two factor-base points
has at most two residual rational points outside the factor base. Relations
with one such point give roots for a graph; relations with two give edges.
Breadth-first substitution expresses connected large primes as linear
combinations of the small factor base and kappa. Cycles give further
homogeneous relations. The resulting modular system includes the target log.

For the anchor phase, a random initial aS+bT is translated repeatedly by S,
with explicit random restarts. This avoids two full scalar multiplications on
every trial. It is a practical walk, not independent uniform sampling. Exact
verification of the final answer does not require that heuristic assumption.

The implementation uses dense incremental elimination at the tested sizes and
does not establish the complexity of an optimized sparse implementation. This
is an implementation of known line and large-prime ideas, not a new index
calculus exponent. See [Diem--Thome's primary description](https://members.loria.fr/EThome/files/non-he-genus3.pdf)
and [Diem's later plane-curve result](https://doi.org/10.5802/jtnb.815).

## Validation and timing scope

Every successful pipeline verifies mS=T in the actual target Jacobian and
mP=Q on the original elliptic curve. Only afterward are full-subgroup BSGS and
five seeded Pollard-rho runs performed. Neither supplies information to the
transfer. Benchmark generation constructs Q from a scalar; that scalar is
withheld from the solver and compared only after recovery.

Saved graph certificates can be independently rechecked with
`validate_four_track.py`: exact line sections, parent substitutions, modular
matrix ranks and solutions, and source answers are verified. Sage's Hess
Jacobian class equality is used throughout; representation-based `is_zero()`
and `!= zero` are avoided.

Pipeline times include setup, section construction, inversion, and target log
recovery. Instance generation, independent controls, and optional basic-solver
comparisons are recorded separately. Some experiments ran concurrently in a
four-core environment; wall times are exploratory observations, not controlled
scaling estimates. Source Pollard rho uses a sixteen-entry adding-walk table
and Floyd collision detection. Walk-addition counts exclude its setup scalar
multiplications, while its wall time includes setup and final verification.

The construction and inversion build on [Tian](https://arxiv.org/abs/2012.07173),
and algebraic obstruction evaluation overlaps [Kirschmer--Narbonne--Ritzenthaler--Robert](https://arxiv.org/abs/2004.08315).
Novelty of the specific counterexamples and specialization remains unverified.
