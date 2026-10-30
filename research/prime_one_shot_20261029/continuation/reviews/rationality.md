# Independent review: random-coset rationality lemma

Reviewer: `/root/theory_review`, 2026-10-30 UTC. Read `/workspace/audit/continuation-math.md` sections 1–8 as present at review time. This is an independent mathematical derivation and quantifier check, not a formal proof certificate, new scientific run, novelty certification, solver-speed result, or ECDLP attack.

## Verdict

The stated **scoped rationality lemma is supported by the derivation**, with its assumptions preserved: p>3; a nonsingular short-Weierstrass curve; odd whole-curve order #E(Fp); a scalar coset of the 2-power subgroup mu_d with d|p-1; distinct residual abscissas; nonzero rational affine target; the selected norm chart; and exclusion of short signed geometric zero sums of lengths 3 through m-2.

The branch-independence argument, conservative Semaev-degree union bound, signed-base second moment, and stated conservative asymptotic constant c<sqrt(2/3) check out. No counterexample was found under those assumptions. The known F17 mixed-twist example is correctly retained as an example where the bad event occurs and the union bound is vacuous.

This lemma removes a field-of-definition obstacle with high probability over a random admissible Kummer coset. It does **not** provide a fast way to solve the coefficient system, an efficient certificate that a particular coset is good, target coverage, or an unconditional sub-exponential ECDLP algorithm.

## 1. Odd order and branch independence

For p odd, a nonzero rational two-torsion point has y=0 and x a rational root of f. Odd #E(Fp) therefore implies f has no Fp root. Since f is cubic, it is irreducible, its roots form one Frobenius 3-cycle, and b=f(0) is nonzero.

If two distinct roots have ratio zeta in Fp, Frobenius cycling forces zeta^3=1. If that ratio also belongs to the 2-power subgroup H, its order divides both 3 and d, hence is one, a contradiction. Thus for distinct w,v in H, f(wG) and f(vG) have no common root over Fpbar.

Each polynomial is squarefree with nonzero finite roots, since E is nonsingular and w is nonzero. In any nonempty product of distinct f(wG), a root belonging to exactly one factor has odd valuation. Therefore their squareclasses are independent in Fpbar(G)^*/squares. Shared ramification at infinity does not undermine this argument: the distinct finite branch valuations already detect every nonempty product.

The argument covers special j, including j=0, provided the curve-order and 2-power assumptions hold. It should not be extended automatically to even curve order, a subgroup H with an order-three factor, Dickson/norm-one torus fibres, or auxiliary isogeny towers.

## 2. Semaev substitution is nonzero

For a fixed distinct w-subset of size s, suppose S_s(Gw_1,...,Gw_s) vanished identically. Over the multiquadratic function-field extension adjoining the independent square roots sqrt(f(Gw_i)), the defining summation-polynomial property supplies one signed relation among P_i=(Gw_i,sqrt(f(Gw_i))). The Galois automorphism flipping only the i-th square root fixes the other points. Subtracting the two relations forces [2]P_i=O.

That is impossible because two-torsion points on the constant curve have constant abscissas, whereas Gw_i is nonconstant. Hence the substituted univariate polynomial is nonzero. The argument needs the ordinary algebraic-closure summation-polynomial interface, not a first-fall heuristic or an Fp-rational-only root equivalence.

The conservative degree bound s*2^(s-2) follows from per-variable degrees at most 2^(s-2). This is enough for the note's bound. The usual sharper total degree (s-1)2^(s-2) is also compatible with a direct weighted-homogeneity derivation: assign weights 1 to each x, 2 to a, and 3 to b; S3 has weighted degree four. The resultant recursion has T-degrees 2^(s-3) and 2, and resultant weight b*D_A+a*D_B-a*b*wt(T), giving weighted degree (s-1)2^(s-2). Fixing a,b can only reduce the ordinary x degree. The sharper value is unnecessary for the conservative theorem and should not be substituted without retaining the exact summation-polynomial convention.

For each w-subset, a nonzero polynomial of degree at most s*2^(s-2) has at most that many nonzero Fp roots. Semaev already existentially includes the signs; another factor 2^s would double-count them. Union bound over binomial(d,s) subsets and then 3<=s<=m-2 gives the stated Delta. Sampling g uniformly samples each H-coset equally often, so there is no additional probability correction for the d representations of a coset.

## 3. Rationality argument and all exclusion cases

All roots of a monic divisor U of X^d-g^d lie in Fp, because that polynomial is split and separable. The recovered residual y values satisfy y^2 in Fp, so all residual points lie in E(Fp^2). Frobenius fixes a rational residual and negates a nonrational one.

Let T be the sum of the nonrational residuals. Applying Frobenius to the residual sum R and subtracting gives [2]T=O. There is no two-torsion over Fp^2: an irreducible cubic cannot acquire a root in a degree-two extension. Equivalently, if r=#E(Fp) is odd, then

    #E(Fp^2)=r(2p+2-r)

is a product of odd integers. Thus T=O.

For a positive number a of nonrational residuals:

- a=1 would make a finite nonzero point equal O.
- a=2 makes the two points inverses, contradicting distinct abscissas.
- a=m makes R=O, excluded by the target assumption.
- a=m-1 makes the sole rational residual equal R, contradicting U(x_R)!=0.
- Every remaining possibility lies in 3<=a<=m-2 and supplies a short signed zero sum, excluded outside Z_g.

Consequently all residual points are rational. The zeros of A+C y, including -R, are rational, and its pole divisor is exactly (m+1)O. Frobenius gives another function with the same divisor; their ratio is constant, and monic A fixes that constant. Hence A,C are rational coefficient polynomials. U was rational already as the monic product of its rational roots.

The result is simultaneous over all nonzero rational targets on one good coset, because Z_g is defined without the target. This is stronger than an average-over-target statement and is justified by the proof.

For m=3 the possible mixed subset range is empty, so selected-chart geometric core solutions are rational deterministically. The same exclusion reasoning would also cover arity four in a suitable even-arity norm chart, but that is not part of this odd-arity implementation. The first mixed rational/twist model at m=5 is consistent with the lemma: its nonrational triple explicitly witnesses Z_g.

The conclusion must remain scoped to the core and uniquely determined auxiliaries. Arbitrarily redundant free slack coordinates need not be rational. In the current retained-quotient membership circuit, monic division makes the quotient and remainder coefficients unique. Properly bounded Bezout witnesses are unique as well; adding unconstrained higher-degree syzygy slack would change that.

## 4. Signed base-size second moment

With B counting signed rational points, not canonical abscissas,

    B(g)=sum_w [1+chi(f(gw))]=d+S(g).

The diagonal terms of S^2 are exactly one because f has no rational roots. For w!=v, f(wG)f(vG) is squarefree of degree six. The standard quadratic-character Weil bound, plus removing G=0, gives absolute nonzero-field sum at most 5 sqrt(p)+1. Therefore

    E S^2 <= d+d(d-1)(5 sqrt(p)+1)/(p-1).

Markov on S^2 yields the note's probability bound for B<d/2. No claim that E S is exactly zero is needed. This proof correctly uses a second moment about d, rather than silently replacing it by a variance about an uncomputed mean.

A useful optional strengthening with the same proof is

    Pr(|B-d|>eta*d)
      <= 1/(eta^2*d)+(5 sqrt(p)+1)/(eta^2*(p-1)).

Taking eta=1/m makes B^m/d^m stable within an exp(O(1)) factor and still gives negligible failure in the stated regime. This is an optional refinement, not needed for the conservative d/2 construction in the note. A canonical-x base would instead have mean approximately d/2; signed and unsigned counts must not be interchanged in the representation formula.

The Weil bound is an explicit external theorem used by the proof. The note correctly does not call it independently re-proved or machine-certified here.

## 5. Asymptotic constants and field restrictions

The consecutive union-bound summands satisfy

    N_(s-1)/N_s=(s-1)/(2(d-s+1)).

For d>=2m this is less than one half over the relevant range, so the full sum is at most twice its last term. Retaining the factorials gives precisely the displayed bound in section6. Dropping binomial factorials would destroy the useful estimate and must be avoided.

The note chooses the least 2-power d at least twice (m!r)^(1/m), so d is below four times that quantity. Its rounding/density safety margin introduces the conservative factor 8^m. Thus

    log2 Delta <=3m-2n/m+O(log m), n=log2 r.

For m=c sqrt(n)+O(1), every fixed c<sqrt(2/3) gives exponential decay in sqrt(n). The stated c=1/2 exponent -2.5 sqrt(n) is correct. Hasse makes r/(p-1)=1+o(1), because r is the whole curve order, not an arbitrary small subgroup order.

This asymptotic statement is conditional on the input prime admitting the required 2-power divisor. It does not prove an arbitrary-prime result. A fixed prime such as P-224 is not an asymptotic family; P-256's p+1/Dickson construction is a different family needing a separate analogue. Efficiently generating a suitable family, if claimed later, needs its own prime-distribution/construction argument rather than an implicit assumption about random primes.

Within an already admissible input field, a random quadratic nonresidue u gives an exact-order-d root u^((p-1)/d). Full factorization of p-1 is unnecessary. Coset enumeration, quadratic-character tests and rational square-root extraction have d*poly(log p) expected bit cost using standard algorithms, so the claimed sub-exponential construction cost is reasonable. This does not include recognition of the short-relation-free event.

## 6. Good-base probability is not efficient recognition

The bound proves that a uniformly sampled coset is good with high probability. It does not efficiently certify a particular realized coset. Exhaustively checking all short supports or materializing the membership algebra can still take exponential work. The note makes this distinction explicitly and should preserve it in any algorithm description.

Randomized base selection can be useful without deterministic recognition, provided a future algorithm has:

1. A proved good-instance solver-cost bound.
2. A finite per-attempt budget, so rare bad bases cannot hide unbounded expected time.
3. Independent rational/group verification of every returned relation.
4. An explicit retry/failure policy and analysis including base rebuilding, lost work, target coverage and relation usefulness.

The current lemma supplies none of those solver-cost guarantees. It also does not justify concluding UNSAT over Fp from the existence of a nonrational geometric model; geometric refutation is sound for rational UNSAT, but geometric SAT still needs the stated good-base event or rational extraction.

Because the high-probability event is over g and simultaneous over targets, a future analysis may sample g once and condition on goodness. It still must include the probability and bounded behavior of the complement, not assume an oracle identifying good g. Verifying one valid returned relation does not certify that all geometric models on that base are rational.

## 7. Ancillary solver-strategy caveat

Section7's singular-support/Riemann-Roch matrix statement is consistent with genus-one Riemann-Roch: for degree-m divisor D and k=m-1 distinct evaluation points outside its support, l(D-sum P_i)=1, giving independent columns; for k=m, a singular minor corresponds to the degree-zero principal-divisor relation. It is a useful correctness oracle, not a support-search speedup.

The later root-guessing discussion should keep V_R=exp(o(n)) as an explicit target/support-count condition or a separately justified typical-target event. Balanced all-target mean by itself does not give that bound for every target. Also keep the guessed domain convention fixed: signed points and unsigned root blocks have different cardinalities. These qualifications do not affect the rationality lemma.

## Final status

The scoped probabilistic rationality derivation survives this independent review. It is a concrete supporting result, not the requested novel sub-exponential algorithm. The remaining uniform, complete solver-cost bound is still the decisive unproved step.

## Addendum: section9 mixed-target/nullspace protocol

After the initial review, section9 was appended to `continuation-math.md` and read separately. Its conditional-independence argument is correct. The linear transformation (a,b)->(u=a+k b,b) is invertible, so conditional on any label-free transcript of target points, oracle outputs, acceptance positions and stopping decisions, the accepted b labels remain independent uniform elements of F_r. A nonzero left-nullspace vector chosen without those labels therefore gives a uniform beta=w dot b, nonzero with probability 1-1/r. Verified group relations then recover k through -alpha/beta. No full factor-base column rank or independent factor-base logarithm supply is needed.

The restrictions are substantive: the oracle must receive only R and label-free inputs, w must not be selected using a/b labels, and policies with b fixed, correlated label walks or label-dependent acceptance are outside the argument. Repeated rows and low rank cause no contradiction. An oracle accepting just one fixed R has a tiny acceptance rate, so relation supply remains the bottleneck even though a dependency can be useful.

One quantitative refinement is needed in the cost paragraph if label-free adaptivity remains allowed. To conclude expected attempts at most (B+1)/rho and cost at most (B+1)C/rho, require a **conditional** acceptance lower bound rho and a **conditional** expected attempt-cost upper bound C at every attempt before completion, or explicitly restrict to a fixed memoryless oracle. A one-attempt marginal or overall average rho does not prevent an adaptive oracle from accepting briefly and then deadlocking. This refinement does not affect the independence lemma itself.
