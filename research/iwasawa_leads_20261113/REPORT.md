# Iwasawa leads I-12, I-13, I-14 for prime-field ECDLP

Date 2026-11-13. Decision: all three **close** as derivations; no lead survives as stated.
No experiment was run and no timing was taken; this note is a derivation with a diagram
(`obstructions.svg`). Every result below is a derivation by the producing session and is an
unverified assertion [pcc:unverified]. Standard facts are cited to the literature, not re-proved in Lean.

Setting throughout: E/F_p ordinary, p > 3, n = #E(F_p) prime, n ≠ p, P a generator, Q = [x]P.

## 0. Where the leads came from

A brainstorm of fourteen Iwasawa-flavoured ideas (session of 2026-11-13) sorted eleven of them
against four known obstructions:

- **W1 torsion wall.** On every smooth lift with p ∤ n, E(F_p) is the torsion of Ẽ(Z_p); every
  homomorphism to (Q_p, +) kills it (`research/prime_field_leads_20261031/C1_lift_torsion/REPORT.md`,
  itself a derivation [pcc:unverified]).
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
