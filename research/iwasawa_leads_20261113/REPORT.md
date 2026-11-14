# Iwasawa leads I-12, I-13, I-14 for prime-field ECDLP

Date 2026-11-13. Decision: I-12..I-14 **close** as derivations. Part II re-checks every step from
first principles and chases three leads (R1..R3) opened by that check: R1 closes on a preregistered
toy computation, R2 and R3 close by derivation. No timing was taken. Diagrams: `obstructions.svg`,
`r1_carry/s2_by_curve.svg`. Every result below is a derivation by the producing session and is an
unverified assertion [pcc:unverified]. Standard facts are cited to the literature, not re-proved in Lean.

Setting throughout: E/F_p ordinary, p > 3, n = #E(F_p) prime, n ≠ p, P a generator, Q = [x]P.

## 0. Where the leads came from

A brainstorm of fourteen Iwasawa-flavoured ideas (session of 2026-11-13) sorted eleven of them
against four known obstructions:

- **W1 torsion wall.** On every smooth lift with p ∤ n, E(F_p) is the torsion of Ẽ(Z_p); every
  homomorphism to (Q_p, +) kills it. Self-contained proof in Part II, §5.
- **W2 MOV.** n-adic Galois structure in the constant direction appears first in F_{p^k}, k the
  embedding degree.
- **W3 lift freedom.** Global lifts of (P, Q) are free choices and lose the relation (xedni).
- **W4 field degree.** n-torsion over a number field needs degree of order n or more.

Three ideas were not obviously caught. This note chases them. There was no idea numbered 15.

| Lead | Idea | Closes by |
|---|---|---|
| I-12 | Signature calculus / Poitou–Tate with Kolyvagin (Euler-system) classes | Lemma A + Huang–Raskind equivalence (W4) |
| I-13 | Specialised Ulmer-tower sections as an index-calculus factor base | Lemma B (W3) + Legendre parity |
| I-14 | Function-field Iwasawa: non-constant Z_n-tower over F_p(t) | Lemma C: no such tower exists (reduces to W2) |

## 1. I-12: signature calculus with Euler-system classes

**Prior art.** Huang and Raskind lift the DLP to a number field and use global (Poitou–Tate)
duality; the local Tate pairings of a global class sum to zero over all places. Their abstract states that
computing the ramification "signature" is random-polynomial-time equivalent to the DLP, and
that the index-calculus analogue does not go through in the elliptic-curve case [pcc:unverified] (arXiv:0710.2363,
LMS J. Comput. Math. 2009). For n ≠ p and good reduction, the local Kummer image at p is
H¹_ur(Q_p, Ẽ[n]) ≅ E(F_p)/n = E(F_p), so the instance lives in local cohomology at p.

**What the lead added.** Kolyvagin derivative classes (and level-raising variants) are global
classes in H¹(K, Ẽ[n]) with controlled ramification at auxiliary primes ℓ, built without choosing
a free lift of Q. The hope was a supply of global classes with known signatures, cheaper than
generic.

**Lemma A (admissible primes are large).** Every auxiliary prime ℓ used by an Euler system for
Ẽ[n] (Kolyvagin primes, Bertolini–Darmon n-admissible primes) has Frob_ℓ acting on Ẽ[n] with an
eigenvalue ±1 mod n, hence n divides #Ẽ(F_ℓ) = ℓ + 1 − a_ℓ or its twist ℓ + 1 + a_ℓ. Hasse gives
both at most (√ℓ + 1)², so ℓ ≥ (√n − 1)², of order n.

*Consequence.* A Kolyvagin class at ℓ lives over the ring class field K[ℓ], whose layer over the
Hilbert class field is cyclic of degree (ℓ + 1)/u_K ≥ (√n − 1)²/3, and the derivative operator
D_ℓ = Σ i·σ^i sums ℓ conjugate Heegner points. For Kolyvagin primes proper, n | ℓ + 1 forces
ℓ ≥ 2n − 1. The work to write down even one class is therefore Ω(n) field operations, against
O(√n) group operations for rho. A further cost, not needed for the bound: Heegner points need a
modular parametrisation of a lift Ẽ/Q, and a lift with coefficients below p has conductor up to
about p³.

**Verdict.** Closes (W4). Euler systems give no supply of global classes cheaper than the field
degree of the n-torsion. Huang–Raskind's equivalence stands, so any sub-√n producer of
signatures would itself be a DLP algorithm; Lemma A shows that the Euler-system route is not one.

## 2. I-13: specialised Ulmer sections as a factor base

**Setup.** Ulmer's Legendre curve y² = x(x + 1)(x + t) over K_d = F_p(μ_d, u), u^d = t,
d = p^f + 1, has explicit points P(ζu), ζ ∈ μ_d, generating a subgroup V_d of rank d − 2 and finite
index in E(K_d) (Ulmer, arXiv:1002.3313; Conceição–Hall–Ulmer, arXiv:1307.4251). Specialising
u ↦ u₀ gives d points of the fibre curve E_{t₀}, t₀ = u₀^d.

**Lemma B (specialisation does not hand over logs).** Let Λ be a Mordell–Weil lattice of rank r
and sp: Λ → E_{t₀}(F_q) specialisation at a good fibre, with image the prime-order group of order
n. Then:

1. The relations known for free among the factor base {sp(P_i)} are the relations already true
   in Λ. For Ulmer's points these are the 2 Galois-trace relations (d points, rank d − 2) plus
   torsion. They give no logs.
2. The logs of the factor base are equivalent to a basis of ker sp, a sublattice of index n.
   Computing that basis is the DLP in E_{t₀}(F_q).
3. Decomposing Q means finding R ∈ Λ with sp(R) = Q. That is finding a lattice point in a fixed
   coset of the unknown index-n sublattice. With no inverse to sp, and sp of small-height points
   spread over the group, this needs about n trials. It is the xedni pattern (W3) in function-field
   dress.

**Two incompatibilities with the target, independent of Lemma B.**

- The Legendre family has full rational 2-torsion, so 4 divides #E_{t₀}(F_q). No fibre is a
  prime-order curve, so the family cannot contain the target E.
- The specialised points have x-coordinates in μ_d·u₀ with d = p^f + 1. Since μ_d ⊄ F_p for
  d > 2, they lie in E(F_{p^{2f}}), not E(F_p).

**Verdict.** Closes (W3). A family without forced torsion, such as Ulmer's 2002 curve
y² + xy = x³ − t^d, removes the parity problem but not Lemma B.

## 3. I-14: non-constant Z_n-towers over F_p(t)

**Lemma C (the only Z_ℓ-extension is constant).** Let K be a global function field over F_q and
ℓ ≠ p. Then K has exactly one Z_ℓ-extension, the constant one K·F_{q^{ℓ^∞}}.

*Proof sketch.* By class field theory, the Galois group G_S of the maximal abelian pro-ℓ extension
unramified outside a finite set S is the pro-ℓ completion of the idele class group modulo the
unit groups O_v^* for v ∉ S. For v ∈ S, O_v^* = F_v^* × (1 + m_v), and 1 + m_v is pro-p, so its
ℓ-part is finite. Pic⁰(K) is finite. So G_S ≅ Z_ℓ × T with T finite, the Z_ℓ direction given by
degree. Any surjection G_S → Z_ℓ kills T, so it factors through G_S/T ≅ Z_ℓ. Every Z_ℓ-quotient is
the degree quotient, which is the constant extension. Every Z_ℓ-extension is ramified at only
finitely many places, so this covers all of them.

*Consequence for ℓ = n.* Any Z_n-tower over the base of an elliptic surface ℰ → P¹ is the constant
tower F_{p^{n^k}}(t). Specialised to the fibre E, Selmer growth along it is governed by the action
of Frobenius on E[n^∞]. It shows up first where E[n] becomes rational, the embedding-degree
extension. That is W2 (MOV).

**The two ways around Lemma C also close.**

- ℓ = p (Artin–Schreier–Witt towers, which are geometric and plentiful): these control the
  p-primary Selmer group. Specialisation lands in E(F_p)[p^∞] = 0 because p ∤ n. That is W1 in
  characteristic p.
- The non-abelian n-division tower F_p(t)(ℰ[n^k]), geometric part SL₂(Z_n): its first layer is
  the function field of a modular curve of level n, of genus of order n³ (W4). At a fibre it
  specialises to the field of definition of E[n], again an embedding-degree extension (W2).

**Verdict.** Closes (reduces to W2). Function-field Iwasawa theory at ℓ = n has no non-constant
tower to work with.

## 4. What is left

Across the fourteen ideas, every Iwasawa mechanism falls into one of three kinds:

- p-adic analysis, which is blind to torsion prime to p (W1);
- n-adic Galois action, which costs field degree of order n or the embedding degree (W2, W4);
- a global lift, which is a free choice (W3).

The two new lemmas (A and C) are standard facts applied to this setting. They are recorded so that
later leads can cite them. No residual lead is proposed. An Iwasawa-type idea that escaped all three
kinds would need a tower whose first layer has degree o(√n), acts on the prime-to-p, n-part of
E(F_p), and is computable without a lift. Lemma C rules this out for abelian towers over function
fields of characteristic p, and Lemma A rules it out for Euler systems.

## Sources checked

- Huang, Raskind, *Global duality, signature calculus and the discrete logarithm problem*,
  arXiv:0710.2363 (LMS J. Comput. Math. 2009), abstract and summary only (web search).
- Ulmer, *Explicit points on the Legendre curve*, arXiv:1002.3313; Conceição, Hall, Ulmer,
  *Explicit points on the Legendre curve II*, arXiv:1307.4251 (web search summaries).
- `research/prime_field_leads_20261031/C1_lift_torsion/REPORT.md` (W1);
  `docs/guides/P256_PROPERTIES.md` §6 and line 413 (canonical-lift falsification).
- research-kb: not reachable in this session (no MCP tool exposed). The lead atlas was not checked
  (sibling checkout absent). Both are open before any atlas edit.

---

# Part II: first-principles audit and leads R1..R3

Requested rule: assume nothing from previous research. No lab note and no literature result is used
below as an authority. Every step is either proved here, measured here, or labelled as a heuristic
or conjecture. All of it is an unverified assertion by the producing session [pcc:unverified].

## 5. What each step rests on

| Step | Standing here | Argument |
|---|---|---|
| W1 | theorem, **narrow** | For p > 2 the formal log identifies Ẽ₁(Z_p) with pZ_p, which has no torsion. For p ∤ n the sequence 0 → Ẽ₁ → Ẽ(Z_p) → E(F_p) → 0 splits, since Ext¹(Z/n, Z_p) = Z_p/n = 0. So E(F_p) is the torsion. Homomorphisms and quadratic forms to Q_p vanish on torsion: h(T) = h(nT)/n² = 0. **Non-homomorphic functions are not covered.** |
| W2 | theorem | E[n] ⊂ E(F_{p^k}) iff Frobenius is trivial on E[n], which needs n \| p^k − 1. |
| W3 (xedni) | was a heuristic; now §7 | |
| W4 | theorem | The Galois orbit of a point of order n has size at least the image of Galois in GL₂(Z/n). |
| Lemma A | theorem | Hasse bound. |
| Lemma B 1–2 | theorem | Linear algebra on Λ → Z/n. |
| Lemma B 3 | was a heuristic; now §8 | |
| Lemma C | theorem | Class field theory; the ℓ-part of tame inertia is finite. |

## 6. R1: the carry cocycle of a non-homomorphic p-adic invariant (measured)

**Object.** L(Q) is the lift of Q to Ẽ(Z/p²) that keeps x(Q) and Hensel-lifts y. The invariant is
ψ(Q) = log(n·L(Q))/(n·p) mod p. It is computable in O(log n) operations. It is not a homomorphism:
ψ(A + B) − ψ(A) − ψ(B) is the formal log of the carry point L(A + B) − L(A) − L(B) ∈ Ẽ₁. W1 does not
cover it. For n = p it is Smart's map, which is a homomorphism.

**Protocol.** `r1_carry/PROTOCOL.md`, frozen before data. Rust tool `r1_carry/src/main.rs`
(sha256 `ceb060f8…22fb`), run locally at commit `f6a6476662`, seed 20261113. Output:
`r1_carry/results_seed20261113.jsonl` (sha256 `a8e50798…6469`). Arms: 12 test curves with n prime
and n ≠ p, p ∈ [2¹³, 2¹⁴]; 4 anomalous curves as positive control; each test table under a seeded
permutation of x as negative control. Statistics are exhaustive over x = 1..n−1.

| Statistic (measured) | Positive control (4 curves) | Test (12 curves) | Negative control (12) | Null bound (frozen) |
|---|---|---|---|---|
| S1 = #{ψ(Q_x) ≡ x·ψ(P)} | n − 1 on all | 1–5 | 1–4 | ≤ 10 |
| S2 = best affine fit in x | n − 1 on all | 10–14 | 10–12 | ≤ 2 × 12 = 24 |
| S3 zero cocycles / 20000 | 20000 on all | 0–4 | 0–5 | < 5/p × 20000 ≈ 6–10 |
| S3 χ² (15 degrees of freedom) | (degenerate: all zero) | 7.85–24.25 | 8.96–24.97 | < 37.7 on ≥ 11 of 12 |

**Verdict.** Closes at the tested scope. The positive control recovers every discrete log, so the
instrument sees a homomorphic leak. On non-anomalous curves ψ shows no affine dependence on x and
its carry cocycle is indistinguishable from uniform. Scope: affine leakage and zero-cocycle
frequency only. Non-affine leakage, for example through bits of x or low-degree structure in the
coordinates, was not tested.

## 7. R2: lifts of small height (derived)

The claim is no longer a heuristic. It is a counting bound plus one conjecture.

**Lemma D.** Let Ẽ/Q have points P̃, Q̃ reducing to P, Q with k·Q̃ = m·P̃ + T, T torsion, and n > 12.
Then:

1. k·x ≡ m (mod n). Reduction is injective on torsion of order prime to p, and by Mazur the torsion
   of Ẽ(Q) has order at most 12 < n. So the image of T in a group of prime order n is O.
2. For each B, at most (2B)(2B + 1) residues x admit such a pair with 0 < |k| ≤ B and |m| ≤ B.
   So for uniform x, Pr[min max(|k|, |m|) ≤ √n/8] ≤ 1/16 + O(n^{-1/2}).
3. The dependency puts P̃ and Q̃, modulo torsion, in a rank-1 subgroup Z·R. With P̃ = k′R and
   Q̃ = m′R, the reduced pair (k′, m′) also satisfies (1). Hence max(ĥ(P̃), ĥ(Q̃)) ≥ (n/64)·ĥ(R) with
   probability at least about 15/16.

**Conditional step.** Lang's height conjecture gives ĥ(R) ≥ c·log|Δ_min(Ẽ)|, bounded below by an
absolute constant. Under it, the coordinates of a dependent lift have Ω(n) digits. Writing them down
costs Ω(n), more than √n. Without the conjecture, an R of tiny height is not excluded here.

**Verdict.** Closes, conditional on Lang's conjecture. Dependent lifts exist, but their size is
exponential in log n. The bound is driven by the best rational reconstruction m/k of x mod n.

## 8. R3: uneven specialisation (derived)

**Part 1 (uneven images do not help).** For a fixed candidate v ∈ Λ and a uniform target Q,
Pr[sp(v) = Q] = 1/n exactly, however sp spreads small-height sections. A search that evaluates sp on
M candidates succeeds with probability at most M/n. The collision version Q − sp(v₁) = sp(v₂)
uses only group operations on Q and the r specialised points. Λ supplies only the two Galois-trace
relations among their logs, so this is a generic algorithm with r extra elements and two known
linear relations. The generic-group bound still gives Ω(√n). Non-generic help could only come from
the explicit coordinates.

**Part 2 (the explicit coordinates do not reach F_p).** The specialised points have
x ∈ u₀·μ_d with d = p^f + 1. Since p^f ≡ 1 mod p − 1, gcd(d, p − 1) = gcd(2, p − 1) = 2. So
μ_d ∩ F_p* = {±1}, and at most 4 of these points lie in E(F_p). The Legendre family also forces
4 | #E_{t₀}(F_p). On both counts the structured factor base is empty for a prime-order E/F_p.

**Verdict.** Closes. Part 1 is a theorem in the generic-group model given Λ's relations. Part 2 is
elementary.

## 9. Standing after Part II

Seventeen Iwasawa-type ideas have now been examined (14 brainstormed, then R1–R3). None survives.

- R1 is the one class of maps the torsion wall does not cover: non-homomorphic p-adic functions.
  It is empty at toy scale for affine leakage. That is a measured null, not a proof.
- The natural next test, if the line continues, is R1 with non-affine statistics: mutual
  information between bits of x and bits of ψ, and degree-bounded fits of ψ on the coordinates.
- Nothing here bears on any leaderboard row; no index-calculus baseline was measured.
