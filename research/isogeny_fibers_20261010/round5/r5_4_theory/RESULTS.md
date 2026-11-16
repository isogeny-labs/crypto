# Lead R5.4: Conjecture 10 over prime fields, and the oracle-dependence of the saturation law

**Date.** 2026-10-12. **Type.** Theory round with two small native sanity
checks (`examples/r5_4_theory_checks.rs`). **Background.**
`RESEARCH_ISOGENY_FIBERS_ECDLP.md` §13–§15, §17.4, §17.7;
`round4/16_7_agm_isogeny_oracles/RESULTS.md` (Theorems 1, 6, 7, Prop. 9,
Conjecture 10); `round4/16_4_zeta_fold_fp2/RESULTS.md`;
`a1_m31/RESULTS.md` (§7.A1-R).

**How to read the labels.** *Theorem / Proposition / Lemma / Corollary*:
proved here, from standard facts that are cited. *Cited*: a published
result I am confident of. *Cited (recalled)*: a result I remember but did not
re-open in this round; nothing proved here depends on one unless the
statement says so. *Heuristic*: depends on the class-factor uniformity
heuristic of §14.1 or on a heuristic running-time analysis.
*Conjecture*: not proved. Numbers are **measured** (exact counts from the
example) or **derived** (arithmetic on recorded numbers, shown).

---

## 0. Verdict in brief

**Part (i): Conjecture 10 over prime fields.**

1. Over F_p (p ≥ 5), every known ECDLP attack family has a cost that is a
   function of the class invariants (p, t) (equivalently (p, N)) times a
   member-dependent factor bounded by an absolute constant. Generic and rho
   with automorphisms: constant ≤ √3 (Theorem 1). Pairing transfers: the
   embedding degree and the transfer itself are the same for every member
   (Theorem 2). Anomalous attacks: apply to every member or none (Theorem 3a).
   Weil descent, GHS and summation-polynomial descent: inapplicable to every
   member, because F_p has no proper subfield (Theorem 3b). Summation-polynomial
   index calculus over F_p: system shape class-invariant except at
   j ∈ {0, 1728}, where the change is exactly the extra automorphism
   (Proposition 5); relation-yield constants bounded by |Aut| ≤ 6 and h
   (heuristic, §15.2). The endomorphism ring enters only through
   Aut(E) = O^× (|O^×| ≤ 6, ratio ≤ 3 within a class), through the group
   structure of the cofactor part E(F_p)/G (bounded by h), and through the
   isogeny transport maps themselves (Theorem 6).
2. **Theorem 4 (exponent propagation).** If two members' endomorphism rings
   differ only at primes ≤ Δ, the DLP on one reduces to the DLP on the other
   with per-instance cost poly(Δ, log p) (vertical part unconditional;
   horizontal part needs GRH for a short path, or heuristic Õ(p^{1/4})
   one-time path finding). So **Conjecture 10 can fail only between members
   whose orders differ at a prime ℓ ≥ p^ε**. Measured: the fraction of
   ordinary isogeny classes (traces) with such a prime falls from 1.7·10⁻²
   at ε = 0.1 to 2.7·10⁻⁴ at ε = 0.25 and 3.8·10⁻⁶ at ε = 0.4 for
   p ≈ 2³⁶ (exact count over all 1 048 576 traces).
3. **What remains open, sharply (Conjecture 10′).** For ordinary E, E′ over
   F_p with #E(F_p) = #E′(F_p), the best (non-uniform, per-instance)
   exponents agree. A counterexample must be a prime-field algorithm whose
   cost depends on End(E) beyond O^× and E(F_p)[h] (for example an index
   calculus whose relation probability depends on the conductor), on a class
   with a large prime conductor gap. Proving the conjecture needs lower
   bounds for non-generic algorithms, which no current technique gives.
4. **Contrast.** Over F_{2^N} with N composite, and over F_{p^k} with k > 1,
   the hypothesis that fails is "the base field has no proper subfield":
   Weil restriction to an intermediate field gives cover attacks whose genus
   depends on the coefficients of the member (the GHS magic number), not on
   (q, t). Theorem 4 still caps the separation at the transport cost there
   (Galbraith–Hess–Smart 2002 is exactly this walk).

**Part (ii): the saturation law and the oracle's enumeration domain.**

5. **Proposition 15.2′.** For an oracle that sweeps (m − k)-tuples from a
   domain Ω and solves a k-summand subproblem per call, a kernel-saturated
   base pays, relative to a plain base of the same size,
   **μ^{m−2}·(|Ω|/|F|)^{m−k}**. Sweeping F: μ^{m−2} (Prop. 15.2). Sweeping a
   transversal of F/M (equivalently φ(F) on E/M): μ^{k−2}, so **1 for the
   pair-table mitm oracle (k = 2)** and μ^{m−2} for a single algebraic solve
   (k = m) that keeps only F-relations. The ratio between the two domains is
   **exact** (μ^{m−k}, no heuristic) by Lemma 7. Conditions: M rational
   pointwise, r ∤ μ, a computable M-invariant coordinate on F (any function
   of x∘φ) to pick the transversal once; no per-lookup isogeny evaluation;
   only the separable part of an isogeny counts.
6. **Reconciliation of §7.A1-R's 2.95×.** The law with Ω = F predicts
   μ^{m−1}(|F_x|/|F_u|)² = 4·(2049/2357)² = **3.02** per independent relation
   (derived); measured 2.95. It factors as 2.17 (enumerating F instead of
   F/M; law 2, removable) × 1.36 (an index-2 image base of half the size;
   law 1.51, the same factor the trace-zero arm pays).
7. **Characteristic 2 (Theorem 9).** On y² + xy = x³ + ax² + b the quotient
   by T = (0, √b) is the Frobenius conjugate E^{(1/2)}, and the quotient
   coordinate is u = x + √b/x = √(x∘[2]). Consequently [2](F_u) = F_tz
   exactly: **a quotient-domain oracle on the T-saturated u-frame is the
   trace-zero (2E) oracle on the same curve**, and every algebraic system in
   a T-invariant coordinate is, after the F₂-linear substitution X = u², the
   plain x-frame system with target x(2R). In characteristic 2,
   2-torsion symmetrisation of a Weierstrass x-frame is therefore null as an
   algebraic device. Measured on 30 bases over 10 binary curves
   (n = 11, 13, 17): the identity, the halving criterion, D_m^G equality and
   the exact factor-2 hit duplication hold with 0 exceptions. Predictions
   for lead R5.1 are in §9.

---

## 1. Question, hypothesis, falsifier

*Questions.* (i) State Conjecture 10 (§17.7) precisely and prove the
strongest true parts. (ii) Make the saturation cost factor a function of the
oracle's enumeration domain, prove it, and explain the A1-R loss; identify any
characteristic-2 obstruction and give R5.1 testable predictions.

*Hypotheses.* (i) Every known attack family's exponent is a function of class
invariants; the endomorphism ring changes only constants. (ii) The μ^{m−2}
factor is the cost of a redundant sweep over M-translates and disappears for
an oracle on F/M.

*Falsifiers, fixed in advance.* (i) An attack family over F_p whose exponent
provably differs between two members of one class. (ii) A toy census in which
a transversal sweep misses a decomposable target that the full sweep finds,
or in which the hit ratio is not exactly μ^{m−k}; in characteristic 2, a
single point violating [2](F_u) = F_tz.

*Outcome.* (i) Not falsified; the part that is open is isolated (Theorem 4,
Conjecture 10′). (ii) Not falsified: 0 exceptions in every exact check (§8).

---

## Part I. Conjecture 10 over prime fields

## 2. Setting, invariants, and what varies

Throughout, p ≥ 5 is prime, E/F_p is ordinary (t ≠ 0), N = #E(F_p) = p + 1 − t
= h·r with r prime and r ∤ h, and G = E(F_p)[r] (cyclic of order r). The
isogeny class 𝒞(p, t) is the set of curves over F_p with trace t (Tate:
F_p-isogenous ⇔ same N; cited). Write D_π = t² − 4p = f_π²·D_K, K = Q(√D_K),
O_K its maximal order, and Z[π] ⊆ End(E) = O_E ⊆ O_K, with conductor
f_E | f_π (Deuring/Waterhouse; cited). Every order between Z[π] and O_K
occurs (Waterhouse 1969; cited).

**Lemma 1 (class invariants).** The following are equal for all members of
𝒞(p, t): N and #E(F_{p^k}) for all k; t, D_π, K, f_π; the order r of G and
the cofactor h; the embedding degree k(r) = ord_r(p) (r ≠ p); whether the
class is anomalous (N = p); the action of π on E[r] up to conjugacy (its
characteristic polynomial X² − tX + p mod r).
*Proof.* All are functions of (p, t). ∎

**What varies with the member** (and nothing else does, among the quantities
any attack in this note reads): End(E) = O_E; Aut(E) = O_E^×; j(E) and the
coefficients (a, b); the group structure of E(F_p) at primes ℓ with ℓ² | N
(Lenstra: E(F_p) ≅ O_E/(π − 1)O_E as O_E-modules; cited), hence the cofactor
group A = E(F_p)/G and its rational torsion; the set of small-norm
endomorphisms (GLV); the isogeny paths to other members.

**Lemma 2 (automorphisms).** For ordinary E/F_p, p ≥ 5,
w(E) := |Aut(E)| = |O_E^×| ∈ {2, 4, 6}; w = 4 only if O_E = Z[i] (so
D_K = −4, f_E = 1, j = 1728), w = 6 only if O_E = Z[ζ₃] (D_K = −3, f_E = 1,
j = 0). Within one class, max w / min w ≤ 3.
*Proof.* Aut(E) = O_E^× for ordinary E (automorphisms are the units of
End(E) since every endomorphism is defined over F_p up to the Frobenius
order; Silverman, AEC III.10; cited). The unit group of an imaginary
quadratic order is {±1} except Z[i] and Z[ζ₃]. A class with D_K ∉ {−3, −4}
has w = 2 for all members; with D_K = −4 the members have w ∈ {2, 4}; with
D_K = −3, w ∈ {2, 6}. ∎

**Definition (exponent).** For an algorithm family 𝒜 and a member E, let
C_𝒜(E) be its expected cost (field operations) on a uniformly random
instance in G. Two members are *𝒜-equivalent up to κ* if
C_𝒜(E)/C_𝒜(E′) ∈ [1/κ, κ]. For the best algorithm overall write
α*(E) := inf{α : DLP in G has per-instance cost p^{α + o(1)} given
instance-independent advice depending only on E} (non-uniform per-instance
exponent; it ignores one-time precomputation, as the multi-target and
precomputation literature does).

## 3. Each known attack family is class-determined over F_p

**Theorem 1 (generic algorithms, with endomorphisms).**
(a) Baby-step giant-step solves DLP in G with ≤ 2⌈√r⌉ group operations on
every member (deterministic; cited, Shanks).
(b) In the generic group model with oracles for every rational isogeny and
endomorphism of every member, together with their scalars on G, an algorithm
making Q queries succeeds with probability ≤ (Q + 2)²/r + 1/r on every member
(Thm 15.1, round 3).
(c) Pollard rho on classes under Aut(E) has heuristic expected cost
√(πr/(2·w(E))) (Duursma–Gaudry–Morain 1999; negation map Wiener–Zuccherato,
Gallant–Lambert–Vanstone; cited, heuristic). Across a class the ratio of
these costs is ≤ √3.
*Proof.* (a), (b) as cited/proved. (c) Lemma 2. ∎

*Where End(E) enters and why only a constant.* An endomorphism α of degree
≥ 2 acts on G as a scalar λ (Lemma 2 of round 4 16.7: λ_α = (a + b)/f for
α = (a + bπ)/f). If α^t = 1 on G with α^t ≠ 1 in O_E then r | deg(α^t − 1)
≤ (deg(α)^{t/2} + 1)², so orbits of non-unit endomorphisms have length
t ≳ log r / log deg α; by (b) no use of them as maps changes the √r
exponent, and §14.3 (B ≥ t) shows orbit folding in index calculus sits on
the √r line. Units give at most w(E) ≤ 6.

**Theorem 2 (pairing transfers: MOV / Frey–Rück).** Let r ∤ p and
k = ord_r(p). For every member E of 𝒞(p, t) there is an injective
homomorphism G → μ_r ⊂ F_{p^k}^× computable in time poly(k log p) (Miller's
algorithm), and k is the same for all members.
*Proof.* k depends only on p and r (Lemma 1).
*Case k = 1* (r | p − 1). Since r ∥ N, E(F_p) = G ⊕ H with r ∤ |H|, so
rE(F_p) = H and G → E(F_p)/rE(F_p) is an isomorphism. The Tate pairing
E(F_p)[r] × E(F_p)/rE(F_p) → μ_r is non-degenerate when μ_r ⊆ F_p
(Frey–Rück 1994, Lichtenbaum; cited), so P ↦ t_r(P, P₀) with P₀ a generator
of G is injective on G.
*Case k ≥ 2* (r ∤ p − 1). π acts on E[r] ≅ (Z/r)² with characteristic
polynomial X² − tX + p ≡ (X − 1)(X − p) mod r (because r | N). The
eigenvalues 1 and p are distinct, so E[r] = G ⊕ ⟨Q⟩ with πQ = pQ, and
π^k Q = Q, so Q ∈ E(F_{p^k}). The Weil pairing is alternating and
non-degenerate, so e_r(P₀, Q) is a primitive r-th root of unity and
P ↦ e_r(P, Q) is injective on G (Menezes–Okamoto–Vanstone 1993; cited).
Both arguments use only N, r, p. ∎
The target DLP is in the order-r subgroup of F_{p^k}^×, so its cost is a
function of (p, k, r) only.

**Theorem 3.** (a) *Anomalous attacks.* For p ≥ 7, p | N ⇔ t = 1, a class
invariant; when it holds every member has G = E(F_p) of order p, and the
p-adic elliptic-logarithm attack (Smart 1999; Semaev 1998; Satoh–Araki 1998;
cited) solves DLP in polynomial time on every member.
(b) *Descent attacks.* Weil descent (GHS), cover attacks through Weil
restriction, and Gaudry/Diem summation-polynomial descent require a base
field F_{q^n} with n ≥ 2 over a subfield F_q. F_p has no proper subfield, so
Res_{F_p/F_p} E = E and none of these applies to any member.
*Proof.* (a) p | N ⇔ t ≡ 1 mod p, and |t − 1| ≤ 2√p + 1 < p for p ≥ 7.
(b) Definitions. ∎

*Remark (covers over F_p itself).* A cover C → E over F_p transfers G into
Jac(C)(F_p), an abelian variety of dimension g over the same field, where G
still has order r ≈ p/h. Whether such covers exist does depend on the member
(for g = 2, Kani gluing needs an anti-isometry E[n] ≅ E′[n]), but every
index calculus on a g-dimensional Jacobian over F_p that I know costs at least
about p^{2−2/g} ≥ p for g ≥ 2 (Gaudry; Gaudry–Thomé–Thériault–Diem double
large prime; cited (recalled) for the exact exponent), above rho. This is one
of the routes Conjecture 10′ must exclude (§5).

**Proposition 5 (summation polynomials over F_p).** Let 𝔛 ⊆ F_p be a set
fixed independently of the curve (Semaev's small-x base, or a structured
x-set), and F_E := {P ∈ E(F_p) : x(P) ∈ 𝔛} on a short Weierstrass model.
(a) The decomposition system S_{m+1}^E(x₁, …, x_m, x(R)) = 0, x_i ∈ 𝔛, has
degree 2^{m−1} in each variable for every member, and the same monomial
support for every member with ab ≠ 0 (Prop. 3.2).
(b) If ab = 0 the support is a subset, and the restriction is exactly the
semi-invariance of S_m under the extra automorphism: for a = 0,
S_3(ωx₁, ωx₂, ωx₃) = ω·S_3(x) (every monomial has degree ≡ 1 mod 3), and for
b = 0, S_3 is even. Such members exist only for D_K ∈ {−3, −4}, f_E = 1.
(c) |F_E| = 2·#{x ∈ 𝔛 : x³ + ax + b is a non-zero square} + O(1), which by
character-sum bounds is |𝔛|(1 + O(p^{1/2}·log p/|𝔛|)) for intervals 𝔛
(Pólya–Vinogradov, Weil; cited), the same leading term for every member.
(d) (Heuristic, §14.1.) The class factor of F_E is c = 1/h when its points
spread uniformly over A = E(F_p)/G, and at most i/h for image-adapted bases
(i ≤ h); so the yield per target is member-independent up to h.
*Proof.* (a) Prop. 3.2. (b) Expand S_3 = (x₁ − x₂)²x₃² − 2((x₁ + x₂)(x₁x₂ + a)
+ 2b)x₃ + (x₁x₂ − a)² − 4b(x₁ + x₂): with a = 0 the monomials have degrees
4, 4, 1, 4, 1; with b = 0 degrees 4, 4, 2, 4, 2, 0. For m > 3 the zero locus
of S_m is invariant under the automorphism, so S_m is semi-invariant
(S_m is irreducible, Semaev 2004; cited). (c), (d) as cited / heuristic. ∎

*Reading.* The coefficients of the system vary with (a, b) and the model,
and 𝔛 is not invariant under isomorphism (x ↦ u²x). So the statement is
about *families of systems*, not about a particular model, and Proposition 5
says nothing about algorithms whose factor base is chosen *as a function of
End(E)*. That is where Conjecture 10′ is open.

## 4. Exponent propagation through isogenies

**Theorem 4.** Let E, E′ ∈ 𝒞(p, t), r > 2√p, and let Δ = Δ(E, E′) be the
largest prime ℓ at which O_E ⊗ Z_ℓ ≠ O_{E′} ⊗ Z_ℓ (Δ = 1 if O_E = O_{E′}).
Then:
(a) There is an F_p-rational isogeny ψ: E → E′ of degree prime to r that
factors as vertical isogenies of prime degrees ℓ ≤ Δ (at most
v_ℓ(f_π) of each) followed by horizontal isogenies between curves with the
same endomorphism ring.
(b) *Vertical part, unconditional.* Each vertical ℓ-step is evaluated on a
point in O(ℓ) operations in F_p from its kernel polynomial (Vélu, Kohel;
cited), which is found once in poly(ℓ, log p) (factor of the ℓ-division
polynomial, Elkies; cited; measured exponent 2.61 in round 4 §17.3).
(c) *Horizontal part.* Under GRH there is a horizontal path of length
O(log p) with prime degrees ≤ O(log² p) (Bach's bound on generators of
Cl(O), as used by Jao–Miller–Venkatesan 2005; cited), evaluated per point in
poly(log p). Finding such a path is instance-independent; Galbraith's
algorithm (1999) finds a path in heuristic expected Õ(p^{1/4}) (cited,
heuristic).
(d) Consequently DLP on E reduces to DLP on E′ with per-instance overhead
poly(Δ, log p) and instance-independent advice; so, under GRH,
**α*(E) ≤ max(α*(E′), log_p Δ + o(1))**, and symmetrically. With one-time
precomputation charged: cost exponent ≤ max(α(E′), 1/4, c·log_p Δ) with c
the kernel-finding exponent (heuristic for the 1/4).
*Proof.* (a) Kohel's description of ℓ-isogeny volcanoes (thesis 1996; cited):
for each ℓ | f_π, an F_p-rational ℓ-isogeny changes the ℓ-adic level by at
most one, and descending/ascending edges exist at every non-extremal level;
at levels where the orders agree locally the curves are linked by horizontal
isogenies, and Cl(O) acts transitively on curves with End = O. Degrees are
≤ 2√p < r or small primes, so prime to r. (b)–(c) as cited. (d) Transport
the instance (P, X) along ψ: by Lemma 1 of round 4 16.7, ψ maps G
isomorphically to G′ with a known scalar λ_ψ (advice), so
log_{ψ(P)} ψ(X) = log_P X. ∎

**Corollary 4.1 (where Conjecture 10 can fail).** Under GRH, members of a
class whose endomorphism rings differ only at primes ℓ ≤ p^{o(1)} have the
same non-uniform per-instance exponent α*. A class can contain members of
different α* only if f_π has a prime factor ℓ ≥ p^ε and the two members sit
at different ℓ-levels.

**Measured: how many classes are left open.** `conductor` mode of the
example counts, for p the least prime above 2^b and *every* ordinary trace
t, the largest prime ℓ_max(f_π) of the conductor index (exact; trial
division to |D|^{1/3} plus a perfect-square test, which is exact for the
square part). Fraction of classes with ℓ_max ≥ p^ε (measured), and the union
bound Σ_{ℓ ≥ p^ε odd} (1 + (p/ℓ))/ℓ² (derived heuristic):

| b | p | classes | f_π = 1 | ε = 0.10 | 0.20 | 0.25 | 0.30 | 0.40 | heuristic at 0.25 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 16 | 65537 | 1024 | 746 | 2.7e−2 | 2.7e−2 | 1.6e−2 | 3.9e−3 | 0 | 1.7e−2 |
| 20 | 1048583 | 4096 | 2852 | 7.1e−2 | 2.0e−2 | 6.3e−3 | 3.4e−3 | 4.9e−4 | 6.6e−3 |
| 24 | 16777259 | 16384 | 10492 | 7.2e−2 | 1.1e−2 | 2.3e−3 | 8.5e−4 | 0 | 2.3e−3 |
| 28 | 268435459 | 65536 | 34466 | 2.0e−2 | 4.0e−3 | 1.4e−3 | 4.9e−4 | 3.1e−5 | 1.3e−3 |
| 32 | 4294967311 | 262144 | 138110 | 1.8e−2 | 2.3e−3 | 6.7e−4 | 1.4e−4 | 7.6e−6 | 6.9e−4 |
| 36 | 68719476767 | 1048576 | 741290 | 1.7e−2 | 1.1e−3 | 2.7e−4 | 6.9e−5 | 3.8e−6 | 2.7e−4 |

The measured fractions track the heuristic Σ 2/ℓ² ≈ p^{−ε}/(ε ln p) closely
(figure `conductor_gap.svg`). Weighted by traces, not by curves; class sizes
(≈ class numbers) are not weighted here.

## 5. Classification, and the conjecture that remains

**Theorem 6 (no known family separates members over F_p).** For every
family in the table, the cost on a member E of 𝒞(p, t) equals a function of
(p, t) times a factor κ(E) with the stated bound:

| family | class-invariant part | member factor κ(E) | where End(E) enters | status |
|:--|:--|:--|:--|:--|
| generic, BSGS, rho | √r | √(2/w(E)) ∈ [1/√3, 1] | O_E^× only | Thm 1 (rho constant heuristic) |
| endomorphisms as maps (GLV orbits, isogeny walks) | √r | 1 | scalars λ_α on G | Thm 1(b), §14.3 |
| pairing transfer (MOV/FR) | (p, k(r), r) | 1 | none | Thm 2 |
| anomalous (Smart, SSAS) | t = 1 | 1 | none | Thm 3a |
| Weil descent / GHS / Gaudry–Diem descent | n/a | n/a | none | Thm 3b |
| summation-polynomial IC on curve-independent x-sets | (p, |𝔛|, m, h) | ≤ 6 (Aut fold) × ≤ h (image/saturation) | O_E^×, E(F_p)[h] | Prop 5, Prop 15.2 (heuristic) |
| any of the above on a member reachable by Theorem 4 | min over reachable members | poly(Δ) overhead | the transport path | Thm 4 |

So the exponent of every known family is a class invariant over F_p, and
the endomorphism ring changes only constants: |O^×| ≤ 6 (ratio ≤ 3) and the
cofactor structure (≤ h). ∎ (By collecting Theorems 1–4 and Proposition 5.)

**Conjecture 10′ (sharp form).** Let p be prime and E, E′ ordinary over F_p
with #E(F_p) = #E′(F_p). Then α*(E) = α*(E′).
By Corollary 4.1 it is a theorem (under GRH) unless the members' orders
differ at a prime ℓ ≥ p^ε. What would refute it: a prime-field algorithm
whose cost depends on End(E) beyond O_E^× and E(F_p)[h], used on a class with
such a gap. Concretely, the open question is: **is there a prime-field index
calculus whose relation probability depends on the conductor f_E** (for
example a factor base built from the CM structure, ideals of small norm, or a
cover that exists only at one level), and does it beat √r on one level while
the levels across a large-prime gap stay out of reach?

*Why it is a conjecture.* Proving it would need lower bounds on non-generic
algorithms for a specific curve, which no technique supplies (the AGM is
vacuous for DLP, round 4 16.7 §3). Theorem 4 only shows that a weakness
spreads to every member reachable cheaply.

## 6. Contrast: which hypothesis fails over extension fields

| field | failing hypothesis | consequence |
|:--|:--|:--|
| F_{2^N}, N composite | F_{2^N} has proper subfields F_{2^d}, d \| N | Res to F_{2^d} has dimension N/d; the GHS cover's genus is governed by the magic number m(b) = dim_{F₂} span{(1, b^{σ^i})} (cited (recalled): Gaudry–Hess–Smart 2002, Menezes–Qu), a function of the member's coefficient, not of (q, t). Members of one class differ in exponent (GHS-weak and non-weak coexist; Galbraith–Hess–Smart 2002). |
| F_{2^N}, N prime | only F₂; cover genus ≥ 2^{N−1}-ish | GHS useless (Menezes–Qu; cited (recalled)); rows of §15.4 hold as over F_p |
| F_{p^k}, k > 1 | proper subfields | Gaudry/Diem descent applies to every member with the same system shape (Prop. 3.2), so it does not separate; GHS in odd characteristic (Diem 2003; cited (recalled)) does, through the coefficients' Galois structure. Members defined over a subfield (j ∈ F_{p^d}) carry a cheap Frobenius-of-subfield endomorphism of order k/d: a constant (√(k/d) in rho), not an exponent. |
| F_p | none of the above | Theorem 6 |

In every case Theorem 4's transport caps the separation by the transport
cost: GHS-weakness spreads across a class exactly as far as the isogeny walk
is affordable, which is what Galbraith–Hess–Smart exploit.

---

## Part II. The saturation law depends on the oracle's enumeration domain

## 7. Setting and the refined proposition

E/F_q (any characteristic), N = hr, r ∤ h, G as before. M ⊆ E(F_q) a subgroup
with r ∤ μ := |M| (so M ⊆ E(F_q)[h]); φ: E → E′ = E/M the separable isogeny
with kernel M (Vélu). A ⊆ Aut(E) acting on the base. F ⊆ E(F_q) is
*M-saturated* (F + M = F) and A-stable. "Plain" means a base F₀ of the same
size, A-stable, not saturated, not image-adapted.

*Oracle class O(Ω, k).* For a target R, the oracle enumerates the
(m − k)-tuples s ∈ Ω^{m−k} and makes one call to a subsolver Sub_k on
R − Σ s_j; Sub_k decides whether its input lies in kF (the k-fold sumset of
F) and returns one decomposition. A call costs one unit (k = 2: a lookup in
the table of 2-sums; k = m: one algebraic solve, Ω^0 = one call; k = 1: a
membership test). Per target, L(Ω, k) = |Ω|^{m−k} calls (ordered; constants
for unordered sweeps cancel in all ratios).

**Lemma 7 (transversal sweeps are complete and exactly μ^{m−k} cheaper).**
Let k ≥ 1 and Ω ⊆ F with Ω + M = F (a transversal if |Ω| = |F|/μ). For
every R,
hits_F(R) := #{s ∈ F^{m−k} : R − Σ s ∈ kF} = μ^{m−k}·hits_Ω(R) when Ω is a
transversal, and in general R ∈ mF ⇔ hits_Ω(R) > 0.
*Proof.* kF is M-stable because F is and k ≥ 1. So membership of R − Σ s_j
in kF depends only on the classes s_j + M. Each class of F/M has exactly μ
elements in F and one in a transversal, giving the factor μ^{m−k}. If
R = Σ P_i with P_i ∈ F, write P_i = s_i + t_i (s_i ∈ Ω, t_i ∈ M) for
i ≤ m − k; then R − Σ_{i≤m−k} s_i = Σ_{i>m−k} P_i + Σ t_i, and replacing
P_m by P_m + Σ t_i ∈ F exhibits it in kF. ∎

**Proposition 15.2′ (cost factor as a function of the domain).** Let
C(F, Ω, k) = U(F)·(r/D_m^G(F))·L(Ω, k) be the collection cost per full set
of relations (U = projected columns). Then:
(i) *Exact.* C(F, F, k)/C(F, Ω, k) = (|F|/|Ω|)^{m−k}; for a transversal this
is μ^{m−k}, and both find exactly the same decomposable targets.
(ii) *Heuristic (Thm 7 of round 4: D_m^G(F) ≈ D_m^G(F₀)/μ^{m−1}, U(F) = U(F₀)/μ).*
Relative to the plain base with full sweep,
  **C(F, Ω, k)/C(F₀, F₀, k) ≈ μ^{m−2}·(|Ω|/|F|)^{m−k}**,
times 1/|A| for the automorphism fold and 1/i for image containment, as in
Prop. 15.2. Cases:

| domain Ω | k = 2 (pair table, mitm) | k = m (one algebraic solve, F-relations kept) | k = 1 (membership) |
|:--|:--|:--|:--|
| F (sweep every translate) | μ^{m−2} | μ^{m−2} | μ^{m−2} |
| transversal of F/M ≅ φ(F) on E′ | **1** | μ^{m−2} | μ^{−1} |

(iii) *Algebraic oracles in the quotient coordinate.* A solve of
S^{E′}_{m+1}(x′₁, …, x′_m, x′(φR)) with x′_i ∈ V finds the decompositions of
φ(R) over F′_V = {Q ∈ E′(F_q) : x′(Q) ∈ V}. Keeping only those with every
Q_i ∈ φ(E(F_q)) (the F-relations) gives the k = m column, μ^{m−2}. Keeping
all of them is index calculus on E′ with the plain base F′_V (|F′_V| ≈ |F|),
a change of class representative (Prop. 9 of round 4), factor ≈ 1. Filtering
is free only when image membership is a linear condition in x′, which happens
in characteristic 2 (trace) and never for odd ℓ-isogenies in odd
characteristic (Kummer character) — the linearity barrier of §14.2.
*Proof.* (i) U and D_m^G do not depend on Ω; L scales as |Ω|^{m−k};
Lemma 7. (ii) Substitute the two heuristic ratios:
(1/μ)·μ^{m−1}·(|Ω|/|F|)^{m−k}. (iii) Theorem 6(b) of round 4 16.7 and the
definitions. ∎

*Conditions, collected.* (C1) M ⊆ E(F_q) pointwise; if only M^π is rational,
saturation and the law use μ = |M^π|. (C2) r ∤ μ (else projection kills G).
(C3) The transversal must be computable: any M-invariant function on F is a
function of φ (the invariant subfield of F_q(E) under translation by M is
φ*F_q(E′)), so one evaluates x′ = x∘φ once per base point (|F| evaluations,
O(μ) each by Vélu) and keeps one point per value of x′ up to sign. It must
separate classes of F/M up to A; x′ is injective on (F/M)/± by definition of
the quotient. No isogeny evaluation is needed per lookup for k = 2, because
the 2-sum table of F is itself M-stable; an algebraic subsolver in x′ needs
φ(R) once per target (φ(R − Σs) = φ(R) − Σφ(s)). (C4) Separability: an
isogeny factors as φ_sep ∘ (Frobenius)^e; the purely inseparable part is a
bijection on E(F_q) with no kernel points, so it contributes μ = 1 and image
index 1. (C5) Characteristic: none for (i)–(ii). In characteristic 2 the
quotient has a special form (Theorem 9).

**How the round-3/4 measurements fit (derived from recorded values).**

| record | domain, k, m, μ | law | recorded |
|:--|:--|--:|:--|
| §15.2 census, T-closed, p ≈ 2^18–2^20 | F, 2, 3, 3 | 3 | 2.59 / 2.79 / 2.85 (rising with p) |
| 16.4, sweep over all F_c (derived ×3 there) | F, 2, 3, 3 | 3 | 2.67–3.62 |
| 16.4, quotient sweep on E′ | transversal, 2, 3, 3 | 1 | 0.93–1.07 size-matched |
| A1-R, u vs x per independent relation (size-corrected law μ^{m−1}(2049/2357)²) | F, 2, 3, 2 | 3.02 | 2.95 |
| A1-R, u-proj vs tz-proj decomposition GAE per trial (u = F-domain, tz = its quotient, Thm 9) | F vs transversal | 2.00 | 2.01 (144606/67 vs 90182/84) |
| this round, char-2 census hits_F / hits_Ω, 30 bases | exact, Lemma 7 | 2 | 2 exactly, every target |

**Reconciling the 2.95× of §7.A1-R.** Per independent relation (measured
GAE): x 3515, u-proj 10387, tz-proj 4780.
2.95 = (10387/4780 = 2.17) × (4780/3515 = 1.36).
* 2.17 is the F-versus-F/M factor. By Theorem 9 the tz arm is the u arm with
  a transversal domain; the law says μ^{m−k} = 2 in lookups at identical
  hits (Lemma 7). Recorded per-trial cost ratio 2.01; the rest is the hit-rate
  sampling difference (0.219 vs 0.232, within the recorded intervals
  0.173–0.303 and 0.198–0.293).
* 1.36 is the image-base factor: a base of 1179 points inside 2E against a
  plain base of 2049 points; law (2049/1179)²/2 = 1.51 per relation. In total
  cost it is more than paid back by the halved columns (S ratio tz/x 0.69–0.71
  recorded).
So the saturation "loss" is entirely the redundant sweep; the u arm with a
transversal domain is the trace-zero arm.

## 8. Characteristic 2: the quotient by T is the doubling map

**Theorem 9.** Let E: y² + xy = x³ + ax² + b over F_q, q = 2^n, b ≠ 0 (an
ordinary curve), T = (0, √b), the unique point of order 2. Put
u(P) = x + √b/x for P ∉ {O, T}.
(a) x(P + T) = √b/x(P); the subfield of F_q(x) fixed by x ↦ √b/x is F_q(u).
(b) x(2P) = u(P)².
(c) The separable isogeny with kernel ⟨T⟩ is the Verschiebung
V: E → E^{(1/2)} (coefficients √a, √b), and Fr ∘ V = [2] with
Fr: E^{(1/2)} → E, (x, y) ↦ (x², y²). So x(V(P)) = u(P); and Fr is a
bijection E^{(1/2)}(F_q) → E(F_q).
(d) For P ≠ O: P ∈ 2E(F_q) ⇔ Tr(x(P)) = Tr(a).
(e) For any X ⊆ F_q let F_u(X) = {P ∉ {O, T} : u(P) ∈ X} and
F_tz(Y) = {Q ≠ O : x(Q) ∈ Y, Tr(x(Q)) = Tr(a)}. Then F_u(X) is T-saturated
and [2] maps F_u(X) exactly 2-to-1 onto F_tz(X²).
(f) For every m, D_m^G(F_u(X)) = D_m^G(F_tz(X²)), and the projected column
counts agree.
*Proof.* (a) With λ = (y + √b)/x the chord through P and T gives
λ² + λ = (y² + xy + b + x√b)/x² = x + a + √b/x, so
x(P + T) = λ² + λ + x + a = √b/x. Translation by T is an involution of the
x-line of degree 2; by Lüroth its fixed field is F_q(u) with u = x + √b/x,
which is invariant and of degree 2 in x. (b) Doubling: λ = x + y/x,
x(2P) = λ² + λ + a = x² + b/x² (substitute y² + xy = x³ + ax² + b), which
equals (x + √b/x)². (c) For ordinary E the Verschiebung is separable of
degree 2 (Silverman, AEC II.2.12, III.6.2; cited) with Fr ∘ V = [2]; its kernel
is the étale part of E[2], i.e. ⟨T⟩. Then x(V(P))² = x(2P) = u(P)², and
squaring is injective on F_q. (d) Necessity: if P = 2Q then x(P) = λ² + λ + a
and Tr(λ² + λ) = 0. Sufficiency is the classical halving criterion
(Knudsen 1999; cited (recalled)); it is also confirmed exhaustively on every
point of the 10 curves in §8.1. (e) T-saturation: u(P + T) = u(P) by (a), and
P + T ∉ {O, T}. If P ∈ F_u(X) then x(2P) = u(P)² ∈ X² and 2P ∈ 2E, so
2P ∈ F_tz(X²) by (d) (2P ≠ O because P ∉ {O, T}). Conversely if
Q ∈ F_tz(X²), (d) gives Q = 2P with P ∉ {O, T}, and u(P)² = x(Q) ∈ X², so
u(P) ∈ X. Fibres of [2] on E(F_q) are cosets of E(F_q)[2] = {O, T}.
(f) [2] is a bijection on G (r odd). If R = ΣP_i with P_i ∈ F_u then
2R = Σ 2P_i. If 2R = Σ Q_i with Q_i = 2P_i, then ΣP_i − R ∈ {O, T}, and
replacing P₁ by P₁ + T (saturation) gives an exact decomposition. So R ↦ 2R
maps the decomposable targets of F_u bijectively onto those of F_tz. Columns:
[h]P ↦ 2[h]P is a bijection on G that identifies the pairs {P, P + T}. ∎

**Corollary 9.1 (the quotient-domain u oracle is the trace-zero oracle).**
For a T-saturated u-frame base over an F₂-subspace V with V² = V (any
Frobenius-invariant subspace), a transversal-domain oracle on F_u(V) performs,
through P ↦ 2P, exactly the computation of the same oracle on F_tz(V) with
targets 2R. For the A1 curve icv1-f2m31-tm90707-c95f16f5 (a = 0) and the ledger's subspace
"divisor 0;1;2" (dimension 11), F_tz(V) is the base on V ∩ ker Tr, which is
the "divisor 1;2" trace-zero subspace of A1-R — *assuming* the ledger's
divisor-0 component is F₂·1 (Tr(1) = 1 for odd n, while the non-trivial
cyclotomic components lie in ker Tr). The recorded sizes agree:
|F_u| = 2357 and |F_tz| = 1179 ≈ 2357/2, projected columns 19 and 19.

**Corollary 9.2 (algebraic oracles: 2-torsion symmetrisation is null in
characteristic 2).** Every T-invariant coordinate is a rational function of
u (Theorem 9a), and S_k^{E^{(1/2)}}(u₁, …, u_k)² = S_k^E(u₁², …, u_k²)
(summation polynomials have universal integer coefficients in (a, b), and
squaring is the Frobenius on coefficients and variables). Hence the
decomposition condition for F_u, P₁ + … + P_m ∈ R + ⟨T⟩, is
S^E_{m+1}(X₁, …, X_m, x(2R)) = 0 with X_i = u_i² ∈ V². Under Weil descent the
substitution X = u² is an F₂-linear bijection of coordinate vectors, and in
the Boolean ring with field equations every polynomial satisfies g² = g. So
the descended u-system is, up to a linear change of variables, the plain
x-frame system over V² with target x(2R): same number of variables, same
degrees, same Macaulay matrices. A u-symmetrised algebraic oracle can never
be cheaper per call than the plain x-frame oracle on the same (twisted)
subspace; with the F_u columns it pays the k = m factor 2^{m−2} if the
solutions with some X_i ∉ ker(Tr + Tr a) are discarded, and nothing if they are
kept (then it is the plain x-frame index calculus). Permutation-symmetric
variables and other formulation choices apply to both systems equally.
*Proof.* As stated, from Theorem 9 and the universality of S_k. ∎

*This is the characteristic-2 obstruction the lead asked about.* The only
étale 2-subgroup of an ordinary binary curve is ⟨T⟩ (with T at x = 0), its
quotient is the Frobenius conjugate, the other 2-isogeny is the inseparable
Frobenius (μ = 1, invisible to the law), and Kronecker's congruence
Φ₂(X, Y) ≡ (X² − Y)(X − Y²) mod 2 says there are no other 2-neighbours. So in
characteristic 2 there is no new quotient curve at ℓ = 2: the quotient
coordinate is √(x∘[2]), quotient domains are 2E-image domains on the same
curve, and the only effect T-saturation can have is the μ^{m−2} accounting of
Proposition 15.2′. Ordinary binary classes have odd t, so D_π ≡ 1 mod 8 and
2 ∤ f_π: there are no 2-volcano levels either. I have not re-opened the
literature that reports char-2 gains from torsion symmetrisation
(Galbraith–Gebregiyorgis 2014; Huang–Petit–Shinohara–Takagi 2015); as I recall
it works with binary Edwards models and 4-torsion, and I do not claim
Corollary 9.2 covers those models.

### 8.1 Computational sanity check (measured)

`char2` mode: curves y² + xy = x³ + ax² + b over F_{2^n}, n ∈ {11, 13, 17},
a ∈ {0, 1}, b = 1 (Koblitz) or random; curves with r² | N or r < 50 are
skipped (2 of 12). For each curve three random F₂-subspaces V of dimension
5 / 6 / 7. Everything is enumerated exhaustively (all points, all of G, all
2- and 3-sums). Raw output: `char2.json`.

| check | instances | exceptions |
|:--|--:|--:|
| x(2P) = u(P)² | every point of 10 curves | 0 |
| P ∈ 2E ⇔ Tr(x) = Tr(a) | every point of 10 curves | 0 |
| F_u is T-saturated | 30 bases | 0 |
| [2](F_u) = F_tz(V²) as point sets | 30 bases | 0 |
| D_2^G(F_u) = D_2^G(F_tz), D_3^G(F_u) = D_3^G(F_tz) | 30 bases × 2 | 0 |
| projected columns equal | 30 bases | 0 |
| m = 3, k = 2: transversal sweep finds the same decomposable targets | 30 bases, all of G | 0 mismatches |
| hits_F(R) = 2·hits_Ω(R) for every R ∈ G | 30 bases | 0 |

One base (n = 11, a = 1, b = 1, seed 2) is empty because its V² lies in
ker Tr while Tr(a) = 1; the identity holds trivially there. Curves are toy
instances with h from 2 to 660; this checks identities, not costs.

## 9. Predictions for lead R5.1 (quotient-aware oracle on the A1 cell)

On `icv1-f2m31-tm90707-c95f16f5`, u-frame base on the dimension-11 subspace,
m = 3, mitm-frobenius style pair table:

* **P1 (exact, Theorem 9e/f).** [2](F_u) equals the tz base as a point set
  (given the component identification of Corollary 9.1); projected columns
  19 = 19; with paired targets R ↔ 2R, the decomposable sets coincide.
  *Any single exception refutes Theorem 9 or the identification.*
* **P2 (exact, Lemma 7).** A transversal sweep (one point per {P, P + T})
  finds every target the full sweep finds, with exactly half the hits and
  half the lookups.
* **P3 (from recorded A1-R values).** The quotient-domain u arm costs about
  the tz arm: ≈ 1.07·10³ decomposition GAE per trial (vs 2158 recorded for
  u-proj), ≈ 4.8·10³ GAE per independent relation (vs 10 387), S ≈ 85
  (vs 140), i.e. ≈ 0.46–0.5 of the u-proj arm and ≈ 0.7 of the x arm in S.
* **P4.** At m = 2 nothing changes (k = 2, no sweep): u-proj stays at
  ≈ 949 GAE per relation within its interval.
* **P5 (algebraic).** A symmetrised F4 oracle in u over V does the same
  Gröbner work per call as the x-frame F4 oracle over V with target x(2R)
  (Corollary 9.2), so recorded differences between them (§7.A1-R:
  sym m = 2 S 41 063 vs x-frame 16 713) are formulation constants. A
  quotient domain cannot reduce the k = m algebraic factor 2^{m−2}.
* **P6 (odd characteristic contrast).** Where E/M is a genuinely different
  curve, a transversal sweep with k = 2 gives ≈ 1 (as 16.4 measured), and an
  algebraic k = m oracle filtered to the image keeps μ^{m−2}.

## 10. What this does not say

* **Not a proof of Conjecture 10.** Theorem 6 classifies *known* attack
  families; Theorem 4 reduces the conjecture to classes with a large prime
  conductor gap. No lower bound on non-generic algorithms is proved.
* **No hardness statement** about any curve, deployed or toy. No sub-rho claim.
* **Theorem 4(c) and Corollary 4.1 are conditional** on GRH (path existence)
  or heuristic (Õ(p^{1/4}) path finding). The vertical part is unconditional.
* **The conductor-gap fractions** are over traces, not curves, and for one
  prime per bit size.
* **The cost factors** in Proposition 15.2′(ii) are heuristic (class-factor
  uniformity); only the domain ratio (i), Lemma 7 and Theorem 9 are exact.
  The A1-R reconciliation uses recorded numbers and the stated component
  identification; R5.1 must confirm P1.
* **Corollary 9.2 is about Weierstrass binary curves and the 2-torsion
  point T.** It says nothing about 4-torsion actions, binary Edwards models,
  or odd characteristic, and nothing about whether a given F4 implementation
  reaches the same constant on both formulations.
* Literature marked "recalled" was not re-checked in this round; no web
  search was run.

## 11. Verdict, limits, next step

* (i) **Confirmed (proved)** for every listed family: exponent is a class
  invariant over F_p; End(E) changes constants only (≤ √3 rho, ≤ 6 · h in
  index calculus, heuristic for the IC constants). **Reduced**: Conjecture 10
  is open only across prime conductor gaps ℓ ≥ p^ε (Theorem 4, under GRH),
  a set of classes of measured density ≈ p^{−ε}/(ε ln p).
* (ii) **Confirmed (proved, exact part)**: the μ^{m−2} factor is
  μ^{m−k} × μ^{k−2}; the first factor is a redundant sweep that a
  transversal removes exactly (Lemma 7, 0 exceptions measured), the second is
  1 for pair-table oracles. A1-R's 2.95× is 2.17 (removable sweep) × 1.36
  (image base). In characteristic 2 the quotient oracle is the trace-zero
  oracle (Theorem 9, 0 exceptions measured) and 2-torsion symmetrisation adds
  nothing algebraically (Corollary 9.2).
* **Next step.** R5.1 tests P1–P5 on the m = 31 cell. On the theory side:
  (a) look for an index calculus over F_p whose factor base depends on O_E
  (the one escape route of Conjecture 10′); (b) check Corollary 9.2 against the
  binary-Edwards formulations of the 2014–2015 papers.

## Commands (single-threaded; wall times contended)

```sh
export CARGO_TARGET_DIR=$PWD/target CARGO_BUILD_JOBS=1
cargo run --release --example r5_4_theory_checks -- conductor > research/isogeny_fibers_20261010/round5/r5_4_theory/conductor.json
cargo run --release --example r5_4_theory_checks -- char2     > research/isogeny_fibers_20261010/round5/r5_4_theory/char2.json
venv/bin/python research/isogeny_fibers_20261010/round5/r5_4_theory/plot.py
```
`conductor` took about 4 s and `char2` about 3 min (contended).

## Files

* `RESULTS.md` (this note), `report.typ`, `report.pdf`.
* `conductor.json`, `char2.json`: raw outputs (stderr logs `*.txt`).
* `conductor_gap.svg` (fraction of classes with a prime conductor gap
  ℓ ≥ p^ε, measured and heuristic), `cost_factors.svg` (law against recorded
  cost factors), `plot.py` (plots recorded values only).
* `examples/r5_4_theory_checks.rs`.

## References

Cited (confident): Tate (isogeny ⇔ equal order); Waterhouse 1969; Deuring;
Lenstra, *Complex multiplication structure of elliptic curves*, J. Number
Theory 56 (1996); Silverman, *AEC* (III.4–III.10, II.2.12); Shanks (BSGS);
Shoup 1997; Pollard 1978; Duursma–Gaudry–Morain, ASIACRYPT 1999;
Wiener–Zuccherato, SAC 1998; Gallant–Lambert–Vanstone, Math. Comp. 2000;
Menezes–Okamoto–Vanstone, IEEE IT 1993; Frey–Rück, Math. Comp. 1994;
Smart, J. Cryptology 1999; Semaev, Math. Comp. 1998; Satoh–Araki, CMUL 1998;
Semaev, ePrint 2004/031; Gaudry, J. Symb. Comput. 2009; Diem, Compositio 2011;
Gaudry–Hess–Smart, J. Cryptology 2002; Galbraith–Hess–Smart, EUROCRYPT 2002;
Galbraith, LMS JCM 1999; Jao–Miller–Venkatesan, ASIACRYPT 2005; Kohel,
thesis 1996; Vélu 1971; Elkies (isogeny kernels); Bach, Math. Comp. 1990
(GRH bound).
Cited (recalled, not re-opened): Knudsen, ASIACRYPT 1999 (halving);
Menezes–Qu, CT-RSA 2001 (GHS for prime N); Diem 2003 (GHS odd
characteristic); Gaudry–Thomé–Thériault–Diem, Math. Comp. 2007;
Galbraith–Gebregiyorgis, INDOCRYPT 2014; Huang–Petit–Shinohara–Takagi 2015.
