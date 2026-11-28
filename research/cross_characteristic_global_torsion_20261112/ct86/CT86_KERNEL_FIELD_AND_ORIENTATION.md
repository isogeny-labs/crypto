# CT86 — Explicit degree-eight subgroup field and orientation-blind quotient

**Scientific provenance: [pcc:unverified].** Bounded algebraic-curve arithmetic / field-theory research on the CT80 11a1[7] fixture. This is neither an efficient cross-characteristic DLP algorithm nor a claimed computational lower bound.

## Exact result: 48 → 24 → 8 over Q

Let E/Q be y²+y=x³−x²−10x−20, and let T be CT80's global oriented 7-torsion point. Write K=Q(x(T),y(T)) and L=Q(x(T)).

**Hypotheses inherited from independently described CT80–82 certificates:** the 7-division polynomial ψ₇ is irreducible over Q of degree 24 (irreducible mod 3); the ordinate has degree two over L (the mod-2 Artin–Schreier trace-one witness); K thus has degree 48. There is one fixed unramified degree-one source place v|37 with T reducing to P=(9,4), and prime-to-37 torsion reduction injects E(K)[7] into E(F₃₇)[7], of exact order seven. We do not silently assume a full GL₂(F₇) Galois image.

**Theorem.** The six maps σ_m:T↦[m]T, m∈F₇×, are all the Q-automorphisms of K. They form C₆. Their restrictions to L identify Aut_Q(L)=F₇×/{±1}≈C₃. The fixed field F=K^{C₆}=L^{C₃} has degree eight over Q.

Proof: for each m prime to seven, [m]T satisfies the same defining 7-torsion equations. Choosing n=m⁻¹ mod 7 gives [n]([m]T)=T, so Q([m]T)=K; the assignment defines an automorphism and composition multiplies m. Conversely, every automorphism sends T to a nonzero element of E(K)[7], giving at most six possibilities by the injective good-reduction bound. The three maps on x(T) are distinct, since x([i]T)=x([j]T) iff i=±j mod 7. Any additional L-automorphism would send α=x(T) to another root of ψ₇ lying in L. At the fixed degree-one place of L above 37, ψ₇ is squarefree mod 37 and has only the three roots 9,27,28 in F₃₇. Such a root is integral and Hensel uniqueness permits at most one global L-root per residue; hence Aut_Q(L) has exactly three elements. Fixed-field degrees follow from Artin's theorem. This is **not** an assertion that K/Q or L/Q is Galois.

Equivalently the intermediate field diagram is

    [K:Q]=48
      | quadratic: orientation y / ±
    [L:Q]=24
      | cyclic triple: three nonzero x-classes
    [F:Q]=8
      | projective/order-seven subgroup choice
    Q

## Explicit primitive element and polynomial certificate

Let α=x(T), and let A₂(X)/D(X)=x([2]T), A₃(X)/ψ₃(X)²=x([3]T), with D, ψ₃, A₂ and A₃ defined by the classical general-Weierstrass formulas in CT85 and in each standalone source. Define the subgroup-kernel polynomial

    H_T(Z)=∏_{i=1}^{3}(Z−x([i]T))
          = Z³ − s Z² + e₂ Z − e₃.

Its first coefficient s=α+x([2]T)+x([3]T) is invariant under the six σ_m. Put B=D ψ₃² and N=X B+A₂ ψ₃²+A₃ D, so s=N(α)/B(α).

An exact independently replayable integer certificate gives ψ₇(X) dividing

    Σ_{i=0}^8 g_i N(X)^i B(X)^(8−i),

where the g_i are the coefficients (constant term first) of

    g(Z)= Z⁸ − 8 Z⁷ − 840 Z⁶ − 29862 Z⁵ − 39690 Z⁴
          − 1696464 Z³ − 6423347 Z² + 3828478 Z − 2102915.

Both Python and independent JavaScript reconstruct ψ₇ and the rational-function numerator/denominator and check this **exact** divisibility by integer pseudo-remainder; no floating-point or merely modular agreement is accepted. g mod 3 is Z⁸+Z⁷+Z²+Z+1, irreducible: Python independently applies Rabin's criterion, whereas JavaScript enumerates all monic factors through degree four. Consequently [Q(s):Q]=8, proving **Q(s)=F**. Offline SymPy's resultant of ψ₇(X) and Z B(X)−N(X) also factors as a nonzero constant times g(Z)^3; the programs' exact polynomial-identity check does **not** depend on SymPy or that resultant.

## Two specializations and the orientation-loss falsifier

All residues use the *same fixed source place* above 37 and the *same fixed target place* above 2 as CT80.

| Invariant | Source F₃₇ | Binary target F_(2¹²), polynomial basis |
|---|---:|---:|
| s = x(T)+x(2T)+x(3T) | 27 | 1545 |
| e₂ (pairwise products) | 30 | 1906 |
| e₃ (product) | 33 | 1261 |

At the binary prime, each of s,e₂,e₃ has exact residue degree four: each is fixed by 2⁴-Frobenius and not by 2²-Frobenius. In particular 1545 satisfies Z⁴+Z+1 over F₂, the polynomial whose square equals g(Z) mod 2. **Do not infer ramification from the repeated reduction of g**: Z[s] may be a nonmaximal order at 2. Good reduction of E at 2 and prime-to-2 torsion guarantee unramified division-field specialization; the calculated projective Frobenius orbit has length four.

On the *oriented* binary point U=(α,Y) in F_(2²⁴), Frob₂⁴(U)=[3]U. Thus the line/subgroup is fixed by 2⁴-Frobenius even though neither α nor U is. For the public target Q=[2]U, **applying Frob₂⁴ to Q alone while holding U fixed changes its logarithm from 2 to 6**, while all three subgroup-kernel coefficients remain unchanged. This is an exact counterexample to treating a compressed subgroup-invariant field as an oriented DLP transfer. It does not rule out algorithms that additionally retain orientation.

The two independent programs test all seven target multiples, cubic coefficients, the degree-four Frobenius condition, source group law and wrong-one-sided-Frobenius control, alongside the exact g certificate. Four local modes (Python, Python -O, Node, Node --no-opt) returned byte-identical output with SHA-256:

    ac4efd15c28c904c5eb88394736b42e4dbca637fac1ff51953ab94c824aaed96

Reproduce from repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct86
    python3 -B "$D/ct86_subgroup_field.py" >/tmp/ct86-py.txt
    python3 -O -B "$D/ct86_subgroup_field.py" >/tmp/ct86-opt.txt
    node "$D/ct86_subgroup_field_independent.js" >/tmp/ct86-js.txt
    node --no-opt "$D/ct86_subgroup_field_independent.js" >/tmp/ct86-js-noopt.txt
    cmp /tmp/ct86-py.txt /tmp/ct86-opt.txt
    cmp /tmp/ct86-py.txt /tmp/ct86-js.txt
    cmp /tmp/ct86-py.txt /tmp/ct86-js-noopt.txt
    printf '%s  %s\n' ac4efd15c28c904c5eb88394736b42e4dbca637fac1ff51953ab94c824aaed96 /tmp/ct86-py.txt | sha256sum -c

**Limits and cost scope.** The drop from global degree 48 to degree 8 is for a *subgroup invariant* and removes the six oriented generator labels. This does not compress an encoded input Q into an input-distinguishing representation. Reconstructing the oriented point still needs full K or an alternative certificate, and identifying σ_m is equivalent at the representation level to identifying m in the source subgroup. This equivalence is not a generic-group lower bound and cannot be extrapolated to larger ℓ or all covers. No matched cold-cost advantage, general point recognizer, performance numbers, or independently signed scientific PCC is claimed. No paid EKS/RunPod compute and no protected merge.

## Research-KB, prior art and next gate

Research-KB MCP status/search/theorems not available among tools; repository wrapper requires an absent checkout. Fallback consulted: CT80–85 reports; root crypto #3877, stacked #4074/#4015, cryptanalysis #1523, crypto-autoresearcher transfer records; [Magma division-polynomial documentation](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurves/function_field.html), [Magma univariate number-field factorization](https://docs.magma-maths.org/BasicRings/UnivariatePolynomialRings/factorization.html), [PARI nfroots](https://pari.math.u-bordeaux.fr/dochtml/html-stable/General_number_fields.html), classical separable good-reduction torsion injection and Artin fixed fields. We claim no novelty for the field-theoretic tools. Evidence gap: complete cross-repo indexed KB survey and independent source-level mathematical PCC.

**Next discriminating experiment:** Compute all three order-seven x-point conjugates *as roots of ψ₇ over L*, using input-independent algebraic number-field factoring, and select the unique one matching the public source coordinates at the fixed place without enumerating m. Retain the oriented y-root, specialize at the fixed binary prime, and separately account for number-field setup/factorization/selection/target solve against matched rho/IC. In addition, examine how much of the degree-eight kernel field helps construct the globally defined degree-seven isogeny, without mistaking subgroup compression for a DLP solution.