# Non-generic ECDLP leads for prime, binary and Koblitz curves

**Date.** 2026-10-18. **Kind.** Search round (theory, literature and repository
search). **Status.** Proposals and derivations only. Nothing here is a
measurement, and no security claim about any deployed curve is made or
withdrawn. Every statement carries one of four labels: *sourced* (a cited
fact), *derived* (follows from sourced facts by the argument shown here; prose,
no `IDC1` certificate yet), *computed* (shell arithmetic of a stated formula,
values in `cover-genus-budget.csv`), or *proposed* (open).

PDF: [`report.pdf`](report.pdf). Figures: [`lead-map.svg`](lead-map.svg) and
[`cover-genus-budget.svg`](cover-genus-budget.svg), with data in
[`cover-genus-budget.csv`](cover-genus-budget.csv).

## Question and scope

The owner asked for 10 to 33 new leads for **non-generic** ECDLP algorithms
over prime-field, binary and Koblitz curves. *Non-generic* means the algorithm
uses the representation (coordinates, field structure, curve coefficients,
covers). A lead is admitted only if it passes three tests:

1. It is not already on this repository's list. The list includes the
   factor-base design register, the shape-search ledger, the cover and GHS
   screens, the idea registry and the literature frontier.
2. It names a mechanism that the Corrigan-Gibbs–Kogan bound for generic
   algorithms does not cover.
3. It has a first experiment and a kill criterion.

Ten leads passed. Ten more ideas were closed on paper while drafting. They are
listed so that nobody spends budget on them.

## Method and sources

- **Read.** `RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md`,
  `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (§§2–4),
  `RESEARCH_KOBLITZ_INDEX_CALCULUS.md` (invariant bases),
  `research/notes/index-calculus/RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md` (shapes
  A1–A17), `research/factor_base_designs_20261015/PREREGISTRATION.md` (arms
  A0–A11, E2–E16, E-K1, E-K4, E-K6),
  `research/linearization_reach_20260930/README.md`,
  `research/cheap_closures_20261007/README.md`,
  `research/jacobian_transfer_scan_20261010/README.md`,
  `research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md`,
  `docs/ECDLP_ATTACK_MATRIX.md`,
  `research_notes/Index calculus leads next steps/literature_frontier.md`,
  and `crypto-autoresearcher:ideas/idea_registry.tsv` (ECDLP-IDEA-002 to -436).
- **LMFDB** (`av_fq_isog` API, 2026-10-18). Nine isogeny classes over F₂
  returned data. The first four came from the API; the five `3.2.*` rows came
  through a second fetch after the API served a captcha. Every row reports the
  expected number of F₂-points: 2 or 4 for T₃ and T₅, and 3 for each Res(E).

  | label | object | simple | principally polarisable | Jacobians |
  |:--|:--|:--|:--|--:|
  | `2.2.ab_ab` | T₃, Koblitz a = 0 | yes | no | 0 |
  | `2.2.b_ab` | T₃, Koblitz a = 1 | yes | no | 0 |
  | `4.2.ab_ab_d_ab` | T₅, Koblitz a = 0 | yes | no | 0 |
  | `4.2.b_ab_ad_ab` | T₅, Koblitz a = 1 | yes | no | 0 |
  | `3.2.a_a_f` | Res_{F₈/F₂}(E), t₃ = −5 (Koblitz a = 1) | no | yes | 1 (Klein) |
  | `3.2.a_a_d` | Res_{F₈/F₂}(E), t₃ = −3 | yes | yes | 2 |
  | `3.2.a_a_b` | Res_{F₈/F₂}(E), t₃ = −1 | yes | yes | 2 |
  | `3.2.a_a_ab` | Res_{F₈/F₂}(E), t₃ = 1 | yes | yes | 1 |
  | `3.2.a_a_ad` | Res_{F₈/F₂}(E), t₃ = 3 | yes | yes | 0 |

  Still owed, all blocked by the captcha: `3.2.a_a_af` (t₃ = 5, the a = 0
  Koblitz curve, already excluded by point counts), the nine
  `5.2.a_a_a_a_*` classes that point counts allow (odd t₅ ≤ 5), and the n = 7 trace-zero classes
  `6.2.ab_ab_d_ab_af_h` and `6.2.b_ab_ad_ab_f_h`.
- **Computed.** Frobenius cyclotomic-coset sizes, `ord_n(2)`. Trace-zero
  L-polynomials for n = 3, 5, 7. The rho and L(1/2) budget arithmetic in
  `cover-genus-budget.csv`, via the awk lines in the "Reproduce" section.
  These are arithmetic only. No research code was written: the repository
  forbids Python for research computation, and the experiments below are
  specified for the native (Rust) harnesses.

## Bottom line

1. **Prime fields: one real lever, and it is curve-specific.** On curves whose
   models have small integer coefficients, the summation polynomial `S₃` has
   small integer coefficients. Its small-root lattice then collapses from 6
   unknown monomials to 3. With projective (Farey) factor bases the 3 unknowns
   are also balanced in size. The level-1 reach for m = 2 rises from p^(1/12)
   to p^(1/9) to p^(1/6) *(derived)*. The curves are secp256k1, Curve25519,
   Curve448, BN254 and BLS12-381 G1.
   - The NIST P-curves (a = −3) get p^(1/10). Random-coefficient curves get
     nothing.
   - m = 2 needs 1/2, m = 4 needs about 1/5, and the repository's best measured
     S₅ reach is 1/80. So this does not threaten any curve. It is a
     measurable improvement to the prime-field decomposition oracle (L1–L3).
2. **Koblitz curves: the cover route is not dead.** GHS is degenerate on
   Koblitz curves (magic number 1), so the repository marks covers N/A. Two
   facts reopen it:
   - The Klein quartic over F₂ has L-polynomial 1 + 5X³ + 8X⁶. That is
     exactly the L-polynomial of Res_{F₈/F₂}(E) for the a = 1 Koblitz curve,
     so its Jacobian is isogenous to that Weil restriction: a genus-3 curve
     for n = 3.
   - By Eichler–Shimura, twisting the CM newform of 49a1 by a character χ of
     order n with χ(2) ≠ 1 gives an abelian variety whose reduction mod 2 is
     isogenous to the trace-zero variety T_n. That holds for every prime n.

   Both are *derived* from sourced facts. Class-group index calculus over F₂
   is subexponential. A covering curve of genus at most c(n)·n would beat
   rho, where c(n) runs from 1.9 at n = 131 to 6.1 at n = 571 *(computed)*.
   The open question is the minimal genus (L6–L9).
3. **Random-b binary curves: only the cover question is new here.** A
   random-b curve does not lift to a curve over Q, so the modular mechanism
   behind the Koblitz leads is unavailable *(derived sketch)*. The one new
   binary lead (L10) asks whether a genus-n Jacobian ever realises the Weil
   restriction beyond the n = 3 cases that GHS already covers.

## The ten leads

Leads are ranked by information per unit of cost. Status uses the labels
defined above.

| # | family | lead | status | first experiment | kill criterion | if it works |
|:--|:--|:--|:--|:--|:--|:--|
| L8 | Koblitz | Étale route: curve D/F₂ with an order-n automorphism and an F₂-map to E; its twist contains E × T_n, genus 1 + n(g₀ − 1) | **measured (partial):** hits only at n = 3; none for any n from 5 to 37 over genus-2 and hyperelliptic genus-3 B | next: plane-quartic genus-3 B (#J up to 198, which reaches n = 131, 163), then genus 4 | no hit for plane quartics | the only structured family inside the L(1/2) budget for K-163, K-283, K-409 and K-571, with explicit maps |
| L6 | Koblitz | Modular twists: (f_E ⊗ χ) mod 2 ≅ T_n up to isogeny, for every n | derived | find the smallest curve whose Jacobian contains A_{f⊗χ} for n = 3, 5, 7, 11 (modular quotients, Fermat quotients of level 7ℓ, Ekedahl–Serre curves) | minimal genus grows faster than c(n)·n | an explicit, computable transfer of T_n into a Jacobian over F₂ |
| L10 | random-b binary | Genus-n Jacobians realising Res(E), beyond GHS | proposed | n = 3: all genus-3 curves over F₂, which traces t₃ are realised. n = 5: genus-5 curves with point counts 3, 5, 9, 17, 33 − 5t₅ | no genus-5 realisation for any t₅ | a genus-O(n) family would make prime-degree binary curves subexponential |
| L9 | Koblitz and binary | Certified lower bounds on the cover genus | derived (partial) | Howe's principal-polarisability test on T_n for deployed n; the manypoints (q = 32, g = 5) bound; point-count positivity | (produces bounds) | closes deployed sizes for good where the bound exceeds the budget |
| L1 | prime | Coefficient collapse plus Farey bases | derived | extend Probes B/C with the 3-term collapsed lattice and Farey parametrisation, m = 2, 3, 4, against random-coefficient controls | reach gain under 1.25× at m = 4 | a curve-specific improvement to the prime decomposition oracle |
| L2 | prime | Eisenstein–Farey bases for j = 0 | derived | same harness on secp256k1-shaped toys | invariance lost under shifts | 3× fewer columns and 9× less linear algebra at equal reach |
| L7 | Koblitz | Klein-quartic family: Z/n covers of E ramified at one place of degree ord_n(2) | derived (n = 3) | Hecke L-functions L(E, χ, X) for n = 7, 31, 73, 127 | no hit at n = 7 or 31 | structural; over budget for deployed n |
| L3 | prime | Algebraic-exposure ledger across the 176 registered prime curves | proposed | formula table of super-monomial count × model × shape | none (audit) | tells which deployed curves L1 or L2 touch |
| L4 | prime | Audit unchecked prime-field IC algorithms | proposed | native re-implementation under the matched-rho protocol | none below the current prime IC baseline | either closes four papers or improves the baseline |
| L5 | prime | Global twist-collision relations on small-coefficient curves | proposed | count squarefree twist classes of f(x) for x ≤ B; ranks of the populated twists | yield below B^0.6 | free relations valid for every p (predicted negligible) |

### L1. Coefficient-collapse lattices with projective (Farey) factor bases

**Mechanism.** Let x_R be the target. S₃(x₁, x₂, x_R) ≡ 0 (mod p) is a linear
relation among a few unknown integer polynomials in the small summands, each
multiplied by a known residue. Level-1 reach is the largest factor base for
which the product of their size bounds stays below p. Writing e₁ = x₁ + x₂ and
e₂ = x₁x₂, the count is:

- **Generic a, b.** Five unknowns e₂², e₁e₂, e₁², e₂, e₁ plus a constant give
  B¹² ≤ p, reach 1/12. This agrees with the repository's symmetric rewriting.
- **a small, b random (P-curves, a = −3).** The unknowns are (x₁−x₂)²,
  (x₁+x₂)(x₁x₂−3), (x₁x₂+3)² and x₁+x₂, plus the constant −4b·x_R. That gives
  B¹⁰ ≤ p, reach 1/10.
- **a and b small, or Montgomery with small A (collapsible).**
  S₃ = c₂x_R² − 2c₁x_R + c₀, where c₂ = (x₁−x₂)², c₁ = (x₁+x₂)x₁x₂ + 2b and
  c₀ = (x₁x₂)² − 4b(x₁+x₂) are small integers. That gives 3 unknowns and
  B⁹ ≤ p, reach 1/9. For Montgomery, c₂ = (x₁−x₂)²,
  c₁ = (x₁x₂+1)(x₁+x₂) + 2Ax₁x₂ and c₀ = (x₁x₂−1)².
- **Farey bases, x = r/s with |r|, |s| ≤ H.** The collapsed c_k become
  bihomogeneous of bidegree (2, 2), all of size H⁴. That gives H¹² ≤ p, so
  |F| ≈ H² ≤ p^(1/6). On generic curves Farey gains nothing: the constant
  becomes a sixth unknown s₁²s₂², giving H²⁴ ≤ p, again 1/12. This is
  closure C1.

| curve class (examples) | interval base | Farey base |
|:--|--:|--:|
| generic a, b (brainpool, most random curves) | 1/12 | 1/12 |
| a = −3, random b (P-256, P-384, P-521) | 1/10 | 1/10 |
| small a, b or small Montgomery A (secp256k1 b = 7, Curve25519 A = 486662, Curve448, Ed448 d = −39081, BN254 b = 3, BLS12-381 G1 b = 4, class-number-1 CM curves) | 1/9 | 1/6 |
| needed for m = 2, 3, 4 decompositions to exist | 1/2, 1/3, 1/4 | |

For m = 3 and m = 4, Farey plus collapse gives level-1 reach 1/30 and 1/144
*(derived)*. The repository's best S₅ lattice with extended shifts reaches
1/80. **The experiment is whether the collapse keeps its edge once the same
shift families are applied.** Prediction: a constant factor in the reach
exponent, far below 1/5. Report it as a stage improvement of the prime-field
oracle, not as a threat. Curve25519's A adds about 21 bits to c₁'s bound at
toy sizes, so toys should use a small A.

### L2. Eisenstein–Farey bases for j = 0 curves

For secp256k1, BN254 and BLS12-381 G1 (p ≡ 1 mod 3), F_p ≅ Z[ω]/𝔭 with
β = ω mod 𝔭. Take x = (a + bω)/(c + dω) with small a, b, c, d. Then x ↦ βx (the
GLV automorphism) maps the base to itself. Because the collapsed c_k have
integer coefficients, evaluation in Z[ω] is exact. The lattice is 6-dimensional
over Z with determinant p, giving |F| ≤ p^(1/6) *(derived)*, the same as L1.
The orbits under ⟨ζ₃, −1⟩ have size 6, so columns drop 3× beyond negation and
linear algebra drops 9×. For j = 1728 the automorphism acts on x as x ↦ −x, so
a symmetric interval is already invariant and needs no new shape.

### L3. Algebraic-exposure ledger

L1 and L2 depend on the model. Curve25519 has a small Montgomery model and a
large-coefficient Weierstrass model. A random curve has no small model: finding
one with coefficients of size C in its isogeny class costs C² ≈ p trials. Put
one row per registered prime curve (the 176 in
`research/weak_curve_leaderboard_20261012/`). Record the best super-monomial
count over the available models (Weierstrass scalings, Montgomery, Edwards,
twisted Hessian when torsion allows) and factor-base shapes, and the implied
level-1 reach. This is an audit, not a claim.

### L4. Audit of unchecked prime-field IC algorithms

The repository's literature frontier lists four prime-field IC algorithms with
no independent check:

- McGuire–Mueller, ePrint 2017/1262. Gröbner-free, and has a second
  algorithm without summation polynomials.
- Hu, ePrint 2024/1923, "pseudo-index calculus".
- Hu, ePrint 2025/015.
- Amadori–Pintore–Sala, ePrint 2017/609, a single Gröbner basis.

Re-implement each natively. Run it on the Table B prime ladder under the
matched-rho protocol with every phase charged. Any one that beats the current
prime IC baseline is a success under the owner's criterion. Otherwise the four
are closed with evidence.

### L5. Global twist-collision relations

On small-coefficient curves, a small integer x gives a point on the quadratic
twist E^{d(x)}(Q), where d(x) is the squarefree part of f(x). Two small-x
points with the same d give a relation from E^d(Q) whenever that twist has
small rank, and the relation holds modulo every p. The xedni obstruction
(Jacobson–Koblitz–Silverman–Stein–Teske) predicts that collisions are rare.
The yield should be about √B against B columns. The lead is to measure the
yield once and close it.

### L6. Modular twists contain the Koblitz trace-zero variety

**Facts.**

- 49a1: y² + xy = x³ − x² − 2x − 1 has CM by Q(√−7) and good ordinary
  reduction at 2. Its reduction is the a = 1 Koblitz curve, with a₂ = 1.
- A quadratic twist by a character of odd conductor with value −1 at 2 (for
  example conductor 3) reduces to the a = 0 Koblitz curve. That is the shape
  of ECC2K-130, K-233, K-283, K-409 and K-571. *(sourced and derived)*

**Mechanism** *(derived)*. Let χ be a Dirichlet character of prime order n and
conductor ℓ ∤ 14, with χ(2) ≠ 1, so 2 is not an n-th power mod ℓ. Then f_E ⊗ χ
is a newform of level 49ℓ² and character χ², with a₂ = χ(2)·t. Its abelian
variety has dimension n − 1, with coefficient field Q(ζ_n). Eichler–Shimura at
p = 2 gives:

> Frob² − χ(2)t·Frob + 2χ(2)² = 0, so Frob = χ(2)·π_E.

Over all conjugates, these are exactly the Frobenius eigenvalues ζπ_E, ζπ̄_E
(ζ ≠ 1) of T_n. By Tate's theorem, A_{f⊗χ} mod 2 is isogenous to T_n. So T_n
lives in the Jacobian of an explicit modular curve over F₂, with Hecke
correspondences available for the map, **for every prime n**.

**Catch.** The containing modular curves have level 49ℓ² ≥ 49(2n+1)², so their
genus is about n³. The budget is c(n)·n.

**Lead.** Find the minimal-genus curve whose Jacobian contains A_{f⊗χ}.
Candidates:

- quotients of X_H at that level;
- Fermat quotients y^N = x^a(1−x)^b of level N = 7ℓ, genus about 3.5ℓ ≈ 7n,
  tested with a Stickelberger check that the Jacobi-sum character matches
  ψ_E·χ;
- Ekedahl–Serre-type curves whose Jacobians split completely.

The positive control is n = 3 (Klein, L7). For a = 1 the newform is 49a1
itself, so K-163 is the natural first deployed target.

### L7. The Klein-quartic family: Galois-conjugate elliptic quotients

**Fact** *(sourced: Elkies, "The Klein quartic in number theory")*. The Klein
quartic x³y + y³z + z³x = 0 has 3, 5 and 24 points over F₂, F₄ and F₈. The
F₂ and F₄ counts are checked by hand in this note. So L_Klein(X) =
1 + 5X³ + 8X⁶.

For the a = 1 Koblitz curve, t₁ = 1, t₂ = −3 and t₃ = −5, so
L(Res_{F₈/F₂}E) = 1 − t₃X³ + 8X⁶ = 1 + 5X³ + 8X⁶. By Tate's theorem,
**J(Klein)/F₂ ≅ Res_{F₈/F₂}(E_{a=1}) up to isogeny** *(derived)*. LMFDB confirms that its isogeny class `3.2.a_a_f` contains exactly one Jacobian *(sourced)*. That is a
genus-n curve for n = 3, where GHS gives nothing.

The structure behind it: the order-3 automorphism (x:y:z) ↦ (y:z:x) is defined
over F₂. Its quotient is an elliptic curve, branched at the single F₄-place
{(1:ω:ω²), (1:ω²:ω)}. The involution quotients ≅ E over F₈ are conjugated by
Frobenius through that automorphism, which is the Weil-descent map, so the map
is explicit.

**Lead.** Take Z/n covers of E totally ramified at one place of degree
d = ord_n(2), the minimum for tame Z/n ramification over F₂. Their genus is
1 + d(n−1)/2. Compute the Hecke L-functions L(E, χ, X) for n = 7, 31, 73 and
127, and test for the factor (1 − ζᵃπX)(1 − ζᵃπ̄X). n = 3 is the positive
control. The genus is over budget for every deployed n (3,365 at K-233 against
691), so the value is structural: is Klein a d = 2 accident?

### L8. The étale route, the only structured family inside the budget

**Equivalence** *(derived)*. A curve C/F₂ carrying the Weil-descent structure
(an F₂-automorphism ρ of order n, and φ: C → E over F_{2ⁿ} with σ(φ) = φ∘ρ) is
the same as the ρ-twist of a curve D/F₂ with the same ρ and an *F₂-rational*
map D → E whose n translates φ∘ρⁱ are independent. Then J_C contains E × T_n,
and the maps are explicit.

Riemann–Hurwitz with tame ramification over F₂ needs n ∣ 2^{deg v} − 1 at
every branch place, which gives three options:

- quotient genus 0: g ≥ (n−1)(ord_n(2) − 2)/2;
- quotient genus 1, ramified: g ≥ 1 + ord_n(2)(n−1)/2 (this is L7);
- quotient genus g₀ ≥ 2, étale: g = 1 + n(g₀ − 1), which needs
  n ∣ #J_B(F₂), where B = D/⟨ρ⟩ covers E.

Only the étale option is linear in n. Taking the smallest g₀ that the Weil
bound (1+√2)^{2g₀} allows *(optimistic)*:

| curve | g₀ | genus | budget |
|:--|--:|--:|--:|
| K-163 | 3 | 327 | 368 |
| K-283 | 4 | 850 | 980 |
| K-409 | 4 | 1,228 | 1,901 |
| K-571 | 4 | 1,714 | 3,471 |
| K-233 | 4 | 700 | 691 |
| ECC2K-130 | 3 | 263 | 245 |

The first four are inside the budget. K-233 and ECC2K-130 are just outside.

**Test.** For unramified χ of order n, D's χ-part has L-polynomial
L(B, χ, X), of degree 2g₀ − 2. It must be divisible by (1 − cπX)(1 − cπ̄X) for
some c ∈ μ_n. With c = 1 the twist of D carries T_n; with c primitive, D
itself does.

**Result** *(measured, exact enumeration; run report
[`reports/run-20261018T2043Z`](reports/run-20261018T2043Z/README.md), crate
[`etale_search`](etale_search/src/main.rs))*. Covered: every smooth genus-2
curve over F₂ (768 models) and every smooth hyperelliptic genus-3 curve
(6,144 models).

- **Validation.** Every Jacobian group was computed from relations and passed
  the check (free rank 1, torsion = L_B(1)). No twisted L-function failed the
  Riemann-hypothesis check.
- **n = 3 only.** Characters with the T_n factor exist only for n = 3: 192 in
  genus 2 and 576 in genus 3, in six isogeny classes of B, for both a = 0 and
  a = 1. So T₃ sits in the Jacobian of an explicit genus-4 étale cover
  for a = 0. That case cannot occur at genus 3, by point counts.
- **Nothing for n ≥ 5.** Characters tested: 2,304 in genus 2 for n ∈ {5, 7,
  11, 13, 19}, and 25,536 in genus 3 for n from 5 to 37. Not one has the
  factor.
- **Genus 2, n = 5 and 7.** The counting argument predicted that only four
  L-polynomials could work. One occurs over F₂, 1 + X + 2X² + 2X³ + 4X⁴, on
  48 models, and fails the character condition. The other three do not occur.
  The genus-2 étale route is closed for n ≥ 5.
- **Deployed sizes untouched.** Over F₂, hyperelliptic genus-3 Jacobians never
  exceed 56 points, so n = 131 and 163 cannot arise from them. Those sizes
  need plane-quartic B, which is next, and genus 4 is needed for n ≥ 233.

Caveats:

- The Weil bound overstates the reachable #J_B(F₂).
- D is generally not hyperelliptic. General-curve class-group algorithms have
  a larger L(1/2) constant than Enge–Gaudry, which shrinks the budget.

### L9. Certified lower bounds on the cover genus

These are the facts so far *(derived)*:

- **Point counts.** If J_C is isogenous to T_n, then #C(F_{2ᵏ}) = 2ᵏ + 1 + t_k
  for n ∤ k. For a = 1 that gives 4 and 2 points over F₂ and F₄, which is
  impossible. So T_n is never a Jacobian for the a = 1 curve (K-163).
- **LMFDB.** T₃ and T₅ have no principal polarisation for either a.
- **Genus exactly n** (J_C isogenous to Res(E)) needs #C(F_{2ᵏ}) = 2ᵏ + 1 for
  n ∤ k and #C(F_{2ⁿ}) = 2ⁿ + 1 − n·t_n.

  | (a, n) | #C(F_{2ⁿ}) required | verdict |
  |:--|--:|:--|
  | (0, 3) | −6 | impossible |
  | (1, 3) | 24 | realised by Klein |
  | (1, 5) | −22 | impossible |
  | (0, 5) | 88 | the Serre bound for (q = 32, g = 5); open |
  | (0, 7) | 38 | open |
  | (1, 7) | 220 | open |

- **Heuristic.** Being P¹-like at every level below n has probability about
  2^(−n²/4) against about 2^(3n) curves. So unstructured genus-n realisations
  should not exist for n ≳ 12, and structured ones need n = 3 (closure C7).

**Lead.** Turn these into a per-n certified lower bound g_min(n) using:

- Howe's principal-polarisability criterion for ordinary isogeny classes;
- Serre and Oesterlé bounds;
- positivity of the places of each degree.

Wherever g_min(n) exceeds the budget, that size is closed for cover attacks.

### L10. Genus-n realisations of random-b Weil restrictions, beyond GHS

For a random-b curve over F_{2ⁿ}, Res(E) has L-polynomial
1 − t_nXⁿ + 2ⁿX²ⁿ. The same P¹-like conditions as in L9 apply. GHS gives genus
2^(m−1) with m = n.

**Why the Koblitz mechanisms do not transfer** *(derived sketch)*. A lift of a
random-b curve to a number field K of degree n with 2 inert is not a Q-curve.
Separable isogenies to its conjugates that lift Frobenius would force CM with
a ring-class field of degree about 2^(n/2). So Res_{K/Q}(E) is not of GL₂ type
and has no modular-curve parametrisation.

**Lead.**

- n = 3 (*sourced, LMFDB*). Res_{F₈/F₂}(E) is the Jacobian of a genus-3
  curve for t₃ ∈ {−5, −3, −1, 1}, and for no other trace. That is exactly the
  set the point counts allow: 9 − 3t₃ ≥ 3 points over F₈. At n = 3, genus-n
  covers therefore exist for every curve the counts permit, random-b curves
  included. GHS already gives genus 3 or 4 there, so n = 3 cannot separate
  the two routes. The lead lives at n ≥ 5.
- n = 5: positivity allows t₅ ≤ 5 (33 − 5t₅ ≥ 3). Query `5.2.a_a_a_a_*`
  for each odd t₅ in that range. Then search genus-5 curves (all
  hyperelliptic, sampled trigonal and canonical) for point counts
  3, 5, 9, 17, 33 − 5t₅.

A genus-5 hit would be the first known genus-n cover of a random-b Weil
restriction beyond n = 3.

## Closed on paper while drafting

| # | idea | why it is closed |
|:--|:--|:--|
| C1 | Farey bases on generic-coefficient curves | the constant becomes a sixth unknown; reach stays 1/12 |
| C2 | Special-prime quadratic orders (θ = 2¹²⁸ with θ² ≡ 38 for 2²⁵⁵−19; θ² ≡ 2³²+977 for secp256k1; √2 ≡ 2²⁶¹ for P-521) | with the collapse, equal to interval or Farey reach; without it, worse (1/48 against 1/18 at level 1); no automorphism gain |
| C3 | Linked expansions by non-unit endomorphisms (1 + τ on Koblitz, √−3 on secp256k1, point halving on binary curves) | equivalent to re-randomising the target unless the union has low-degree membership; towers raise D, and the repository bounds invariant pull-backs at 2 bits per summand |
| C4 | K-233 Frobenius-invariant subspaces (8 cosets of 29; m = 4, dim 59) | linear algebra ≈ 2^102 is under rho 2^110.9, but the per-solve budget 2^59 is below a degree-3 Macaulay solve on the chained system (C(702, 3)^2.8 ≈ 2^73) |
| C5 | K-571 and B-571 with m = 5 chained systems at ℓ ≈ n/5 | beats rho only if the solving degree is at most 6 to 9; that is the Semaev 2015 regime, and the repository's measured D growth already contradicts it |
| C6 | Oakley groups 3 and 4 by direct Gaudry or Joux–Vitse decomposition over F_{(2³¹)⁵} and F_{(2³⁷)⁵} | per-trial S₅/S₆ Gröbner cost dominates; the repository's GHS pricing stays the better route |
| C7 | Genus-n covers carrying an F₂-automorphism of order n | Riemann–Hurwitz plus tame ramification over F₂ allow it only for n = 3 (Klein) |
| C8 | F₂-rational norm-residue map P ↦ f_P (div f_P = the Frobenius orbit of P − n·O) | f_P is a local norm everywhere, hence a global norm (Hasse); the map is trivial |
| C9 | Characteristic-2 Farey bases (x = u/v, u and v in subspaces) | the term u₁u₂v₁v₂z raises the descended degree to 4 |
| C10 | Characters, Legendre symbols, Elkies kernels, Tate pairings modulo small ℓ | every homomorphism to a small group kills a prime-order subgroup; only cofactors are visible |

## Already on the repository's list, so not counted

- MinRank modelling (E-K4).
- Gauss-period windows (E-K1).
- Kummer, Lucas and elliptic towers (A0–A11, E2–E10).
- Learned frames (E14 and E15).
- Frobenius-equivariant F4.
- Representation filtering and higher-degree XL on the bilinear core.
- Quasi-subfield polynomials.
- Isogeny-transported bases.
- GHS screens and the Oakley pricing.
- Global lifting and xedni.
- The canonical-lift torsion profile (IDEA-436).
- Function-field lifts (IDEA-435).
- Split-Jacobian projected smoothness (IDEA-002).
- Trace-zero transfers for prime fields (IDEA-009).

## Figures

- `lead-map.svg`: the ten leads by family, with status colours and the
  closures. It is a diagram, not data.
- `cover-genus-budget.svg`: for each deployed Koblitz size, the L(1/2) budget
  g_max(n) against the genus of the étale (L8) and ramified Klein (L7)
  constructions. All values are derived; the source is
  `cover-genus-budget.csv`. Formulas:
  - rho = ½·log₂ r + ½·log₂(π/4) − ½·log₂ |Aut|, with |Aut| = 2n for Koblitz
    and 2 for random-b curves;
  - g_max solves √2·√(G ln G) = rho·ln 2 with G = g ln 2;
  - étale genus = 1 + n(g₀ − 1), with g₀ the least genus whose Weil bound
    (1+√2)^{2g₀} reaches n;
  - Klein genus = 1 + ord_n(2)·(n − 1)/2.

## Unresolved checks and owed certificates

- `identity.certificate/v1` records are owed for four claims. Until then they
  are prose:
  1. the collapsed and Farey super-monomial counts (L1, L2);
  2. L_Klein = L(Res_{F₈/F₂} E_{a=1});
  3. the Eichler–Shimura twist identity (L6);
  4. the a = 1 point-count obstruction (L9).
- LMFDB queries for the nine admissible `5.2.a_a_a_a_*` classes, the two n = 7
  trace-zero classes in dimension 6, and `3.2.a_a_af` were blocked by a
  captcha. The other n = 3 classes are in the table under "Method and
  sources".
- L8 is only partly run: plane-quartic genus-3 curves and genus 4 are not
  covered by the étale search.
- The manypoints entry for (q = 32, g = 5) has not been checked.
- Novelty audit: not done against the cover-attack literature. Check Diem and
  Scholten's trace-zero cover reports, Hess's generalised GHS, Ekedahl–Serre,
  and Elkies's Klein-quartic survey before any of L6–L10 is called new.
- The L(1/2) budget uses the hyperelliptic constant √2 and drops o(1). It is
  optimistic for an attacker.

## Graphs checked and left unchanged

These were checked and left unchanged. The only measured item, the L8
enumeration, is a lead-level search with no cost or index-calculus result, and
nothing here is certified:

- `docs/index-calculus-scoreboard.html`
- `docs/ic/progress-timeline.json`
- the index-calculus leaderboard (`docs/ic-leaderboard.html`,
  `docs/ic/LEADERBOARD.md`, `docs/ic/leaderboard.json`)
- `docs/ECDLP_ATTACK_MATRIX.md`

Row 12 of the attack matrix ("trace-zero variety, N/A for genus 1") should
cite L7 once its certificate exists.

## Reproduce

```sh
# the budget table (arithmetic of the stated formulas) and its graph
awk -f cover-genus-budget.calc.awk > cover-genus-budget.csv
awk -f cover-genus-budget.plot.awk cover-genus-budget.csv > cover-genus-budget.svg
# report.pdf: Typst, rendered from report.typ (see research-visuals skill, "Rendering")
```

## References

- Corrigan-Gibbs, Kogan. The discrete-logarithm problem with preprocessing. EUROCRYPT 2018.
- Elkies. The Klein quartic in number theory. In *The Eightfold Way*, MSRI Publ. 35, 1999.
- Enge, Gaudry. A general framework for subexponential discrete logarithm algorithms. Acta Arith. 102, 2002.
- Gaudry, Hess, Smart. Constructive and destructive facets of Weil descent on elliptic curves. J. Cryptology 15, 2002.
- Hess. Computing relations in divisor class groups of algebraic curves over finite fields. 2005.
- Howe. Principally polarized ordinary abelian varieties over finite fields. Trans. AMS 347, 1995.
- Jacobson, Koblitz, Silverman, Stein, Teske. Analysis of the xedni calculus attack. DCC 20, 2000.
- Shimura. Introduction to the Arithmetic Theory of Automorphic Functions, §7 (Eichler–Shimura).
- Tate. Endomorphisms of abelian varieties over finite fields. Invent. Math. 2, 1966.
- Ekedahl, Serre. Exemples de courbes algébriques à jacobienne complètement décomposable. C. R. Acad. Sci. 317, 1993.
- Galbraith, Granger, Merz, Petit. On index calculus algorithms for subfield curves. ePrint 2020/1315.
- LMFDB, abelian varieties over finite fields, https://www.lmfdb.org/Variety/Abelian/Fq/ (queried 2026-10-18).
