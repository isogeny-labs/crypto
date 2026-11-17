# LEAD-03 — ℓ-torsion symmetrisation beyond 2-torsion, and Kummer-model coordinates

**Verdict.** closed-by-measurement (for ℓ = 3; the ℓ = 4 and the Kummer-line parts are closed by theorem, §5). Not an exponent lead and not a constant-factor lead: against the plain x-line and against 2-torsion it is a measured and counted regression.
**Confidence.** high that ℓ = 3 and ℓ = 4 cannot beat the 2-torsion frame or the plain x-line at k = 3, 4: the exact counting argument (FHJRV Prop. 3), one exact degree check (9 = 3² per variable on the Hessian S′₃) and the repository's measured m = 3 row all agree. Medium that no niche is left (linear-algebra side, prime fields, non-split case: §8).
**Expected gain if it works.** None. Counted F̄_q-solutions of the descended system at k = 3: 2187 (ℓ = 3) against 16 (FHJRV 2-torsion) and 64 (plain S_n-symmetrised Semaev), i.e. 137× and 34× larger; at k = 4: 1.6·10⁶ against 512 and 4096 (3114×, 389×). Measured F4 on the closest proxy (§3): 5–8× slower than Gaudry's x-line at m = 3. No leaderboard row moves.
**Cost of the discriminating test.** About 1 person-day (one Hessian curve and chart constructor for the existing quotient engine), under 1 CPU-hour. A test of the lead's own kill criterion is already decided at the first measurement (§6).

Notation: m is the card's m, the number of unknown points (extension degree k). The relation has m + 1 points once the target is included; S′_{m+1} is FHJRV's P_{ϕ,m+1}. FHJRV's n is the number of points in the relation.

## 1. Mechanism (precise)

**Framework (FHJRV §3).** Take ϕ : E → P¹ with ϕ∘τ_T = f_T∘ϕ for a rational T of order ℓ and f_T ∈ PGL₂(F_q) of order exactly ℓ (else ϕ factors through an isogeny and nothing is gained). The factor base {ϕ ∈ P¹(F_q)} is then T-stable. The summation polynomial is invariant under G = (Z/ℓ)^{n−1} ⋊ S_n when f_T is affine, and only the fraction is invariant otherwise.

**Hessian instance, ℓ = 3.** H_d : X³+Y³+Z³ = 3dXYZ, O = (1:−1:0), −(X:Y:Z) = (Y:X:Z). Take ϕ = u = X/Y (projection from (0:0:1), degree 3). The translation by T = (1:−ω:0) is (X:Y:Z) ↦ (X:ωY:ω²Z), so u ↦ ω⁻¹u, and −1 acts as u ↦ 1/u. Together they give the dihedral group D₃ ⊂ PGL₂. This is the same chart structure as the repository's j = 0 `v`-line (§3).

**Invariant ring for m = 3 (the question asked).** The group is G(3,3,3) = (Z/3)² ⋊ S₃, order 54, acting on (u₁,u₂,u₃). Result: the ring is **polynomial, three primary invariants, no secondary invariants**:

- A = Σuᵢ³ (degree 3), B = Σ_{i<j}uᵢ³uⱼ³ (degree 6), C = u₁u₂u₃ (degree 3), with s₃ = C³.
- Derivation: the diagonal part (Z/3)^{n−1} has invariant monomials with all exponents congruent mod 3, so its ring is K[Y₁..Yₙ, eₙ]/(eₙ³ − ΠYᵢ) with Yᵢ = uᵢ³. S_n acts on the Yᵢ only, so the S_n-invariants are K[s₁..s_{n−1}, eₙ]. This is the exact analogue of FHJRV Prop. 13 with cubes in place of squares.
- General n: degrees 3, 6, …, 3(n−1), n; their product is 3^{n−1}n! = |G|. This is Shephard–Todd for G(3,3,n), recalled and checked here by a numerical Molien series for n = 3, 4, 5, which matched 1/Π(1−t^{dᵢ}) to 10 digits (unverified desk check). For the 4-point relation (target included) the group is G(3,3,4), order 648, degrees (3,6,9,4).

So the card's second kill condition ("secondary invariants add variables") does not fire. The lead fails for a different reason.

**Degrees (the real constraint).** FHJRV Prop. 3: ℓ | deg ϕ, so deg ϕ ≥ 3. For ℓ > 2 the dihedral group D_ℓ embeds in PGL₂ (FHJRV §3.1), so ϕ cannot be [−1]-even. The per-variable degree of the (m+1)-point summation polynomial is then (deg ϕ)^m = 3^m, not (deg ϕ)^m/2:

| object | per-variable degree |
|---|---|
| Semaev, 2^{m−1}; FHJRV 2-torsion, even degree-2 ϕ | 2^{m−1} |
| Hessian ϕ = X/Y | 3^m |

Desk checks (unverified, scratchpad, d = 2 over Q, then reduced mod 1009):

- S′₃ (three-point relation, FHJRV's P_{ϕ,3}, the m = 2 case): exactly **9 per variable**, total degree 18. It is symmetric, G(3,3,3)-invariant (all exponents congruent mod 3) and palindromic under u ↦ 1/u. It has 88 monomials in u, 24 in the σₖ and **22 in (A,B,C)**, against 5 terms for FHJRV's single-2-torsion P_{ϕ,3} and 5 for Semaev's S₃ (FHJRV Table 2).
- S′₄ (m = 3): per-variable degree **27 = 3³** by the same fibre count (not computed). Total degree 81 is extrapolated from the 2×9 pattern at S′₃ and is unverified. The support bound is at most 1587 invariant monomials, against 13 terms for FHJRV's S′₄ (Table 2, single 2-torsion) and 36 for symmetrised Semaev.
- A trap for the chained or split formulation: since ϕ(Q) does not determine Q up to ±, the naive resultant chain Res_w(S′₃(u₁,u₂,1/w), S′₃(u₃,u₄,w)) has degree 81 in u₁, three times the true 27. The splitting trick needs the intermediate point lifted, which costs a variable.

**Solution count and degree-of-regularity bound.** The only bound the formulation obeys is multi-homogeneous Bézout plus the Fröberg/Macaulay heuristic, with no sharper dreg theorem known. Quotient count of F̄_q-solutions in the invariants: n!·d_P^n/|G|, with n = m unknowns, d_P the per-variable degree and |G| = ℓ^{m−1}m!.

| m | Semaev/S_m | 2-torsion (FHJRV) | ℓ = 3 | ℓ = 4 (deg ϕ ≥ 4) |
|---|---:|---:|---:|---:|
| 2 | 4 | 2 | 27 | 64 |
| 3 | 64 | 16 | 2187 | 16384 |
| 4 | 4096 | 512 | 1 594 323 | 6.7·10⁷ |
| 5 | 2²⁰ | 65 536 | 1.0·10¹⁰ | 4.4·10¹² |

The same formula reproduces the repository's own numbers: 4096 and 512 at m = 4 (`research/gaudry_quartic_d4_20261013`) and 2^{(m−1)²} = 2¹⁶ at m = 5 (the EcGFp5 note, §3). General form: ℓ^{m²−m+1} against 2^{(m−1)²}.

**Curve conditions.**

- A rational T of order 3 forces 3 | #E(F_{q^n}), so the ECDLP subgroup has cofactor h ≥ 3.
- An affine f_T (hence polynomial invariants over F_q) needs ω ∈ F_q, with q the base of the factor base (FHJRV §3.1: D_ℓ ⊂ PGL₂(F_q) iff ℓ | q−1, q+1 or ℓ = char). Then the monic Hessian has all nine flexes rational. In a brute-force count over F_p, p = 7, 13, 19, 31, all non-singular d gave #H_d ≡ 0 mod 9 (unverified desk check), so h ≥ 9 and rho in the subgroup gains √9 = 3. The generalised Hesse form aX³+bY³+cZ³ = 3dXYZ, which includes j = 0 curves with √b rational, keeps h ≥ 3 (gain √3).
- For q ≡ 2 mod 3, f_T is non-split of order 3. It is still in PGL₂(F_q), with P¹(F_q) ≅ the norm-one torus μ_{q+1}, but it is not affine, so only the invariant field is available.
- FHJRV §3.2: the factor base divides by 3 (linear algebra 9× smaller) while the decomposition probability falls by 3^{m−1}.

## 2. Prior art: literature

- **FHJRV.** Faugère–Huot–Joux–Renault–Vitse, *Symmetrized Summation Polynomials: Using Small Order Torsion Points to Speed up Elliptic Curve Index Calculus*, Eurocrypt 2014, https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/Semaev2torsion.pdf. **read-at-source** (full text, via a local text extraction). Claims taken:
  - Prop. 3: ℓ | deg ϕ.
  - Prop. 4: only E[2], cyclic ⟨T⟩ or E[char] can act, so (Z/3)² cannot.
  - §3.3 sets up G = (Z/m)^{n−1}⋊S_n for any m.
  - Lemma 6: every degree-2 ϕ is automatically 2-torsion-equivariant. This is why they stop at m = 2, together with Prop. 5, Prop. 7 and Prop. 13 (invariant ring polynomial in any characteristic), which hold only for degree 2. The abstract says 2-torsion is "the most important case in practice".
  - §6 says higher order torsion "will be more detailed in an extended version". I found no such version.
  - §5.2: the Oakley curve has order 12·prime and "we can only consider the action of the 2-torsion or the 3-torsion points"; they used 2-torsion, with no 3-torsion example or timing.
  - Table 2 gives 5, 13 and 182 terms (single 2-torsion) for n = 3, 4, 5.
- **FGHR.** Faugère–Gaudry–Huot–Renault, *Using symmetries in the index calculus for elliptic curves discrete logarithm*, J. Cryptology 2014. recalled, as described in FHJRV §1 and the repository (twisted Edwards, 2-torsion).
- **BRS.** Barbulescu–Robert–Sarkis, *Models of Kummer lines and Galois representations*, ePrint 2025/543, https://eprint.iacr.org/2025/543. **abstract-only.** Claims taken: with one rational 2-torsion point T the Kummer-line models split into symmetric and anti-symmetric types, recovering Montgomery and theta variants; curves are classified by Galois representations and modular curves. The abstract mentions no discrete logarithm or summation polynomial.
- **Shephard–Todd 1954 / Chevalley.** recalled, G(m,p,n) invariant degrees m, 2m, …, (n−1)m, nm/p. Verified independently in §1.
- **Hessian-form papers.** Smart (CHES 2001), Joye–Quisquater (CHES 2001), Farashahi–Joye (PKC 2010). recalled, context only; the facts used were re-derived or counted in §1.
- **Galbraith–Gaudry.** *Recent progress on the elliptic curve discrete logarithm problem*, DCC 78 (2016), ePrint 2015/1022. abstract-only (search metadata; the full text was unreachable). It is the repository's source for Vitse's DLP 2014 talk on "which subgroups could be used"; no manuscript found.
- **Galbraith–Gebregiyorgis.** INDOCRYPT 2014, ePrint 2014/806. recalled via the repository (rho much faster than index calculus in characteristic 2).
- **Standing constraints** from `LEADS.md` (Corrigan-Gibbs–Kogan, Shoup, structured GGM). Not re-read.

## 3. Prior art: these repositories

- `/home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md`. **Closes ℓ = 3 at m = 3.**
  - §1.3: translation by ℓ > 2 does not act on the x-line.
  - §13: the j = 0 `y`/`v`-line with ⟨τ₃⟩, which is the Hessian's D₃ structure (fixed-target group = translations only). At m = 2 it is a small-m win (3 unknowns, degree 2). At m = 3 the relation has degree 3 in vᵢ³ against Semaev's 4, with tuple unknown Πv of degree 8 (118 terms, total degree 9); Buchberger exceeds budget.
  - §14.3 (F4, F₃₁³, base {v ∈ F_p}): solving degree 12 against 14 for Gaudry, matrix 5953×6598 against 2166×2512, 152 s against 19–35 s. Verdict "the 3-torsion frame does not pay".
  - §14.2: the one-T 2-torsion frame gives solving degree 7, matrix 740×822, 30–60× faster than Gaudry, on the same engine.
  - §15 #3, #4: 4- and 6-torsion lines are changes of curve.
  - §16: closing lemma, with ℓ-torsion carried only through N(1−α).
  - §19: FHJRV instantiate only m = 2; Vitse's talk noted.
  - §20: the end-to-end 2-torsion pipeline is 0.76–0.84 at n = 17, 23 and 4.0× worse at n = 31, because yield halves; "the symmetry is priced into the floor"; every arm is 10³–10⁵× rho.
- `RESEARCH_K5_TORSION_JOUX_VITSE.md`. **Closes ℓ = 4 on Edwards.** The 2-torsion frame cuts C″ 416× (degree 4 and 3, matrix 1513×1633). Q₄ = (1,0) acts as P ↦ (y,−x), so it fixes no function of y alone and no degree-halving symmetry exists; saturation of the residual by Q₄ costs 2.01× for 2× the rate (0.99× on S). The "4-torsion 4–10×" registration is retracted.
- `research/gaudry_quartic_d4_20261013/README.md`. **Supports** the baseline and the counting formula: D₄ at m = 4 has Bézout 512 against 4096, Macaulay degree 14 against 29, C₄ 47.5–48.4× lower, crossover 2¹⁵⁰ → 2¹³⁵ at h = 2.
- `RESEARCH_SYMMETRIZED_SEMAEV.md` and the header of `src/cryptanalysis/symmetrized_semaev.rs`. **Unrelated**: S_n-only symmetrisation, no torsion.
- `src/cryptanalysis/fghr_full.rs` header. **Supports** the 2-torsion arm at k = 3: Y-coordinate, `T` in the cofactor, folding coefficient 1, D₃ oracle. No ℓ ≥ 3 code exists.
- `research/pair46_factor_bases_20261015/12-model-small-x/RESULTS.md`. Hessian-x and x+y small-coordinate bases are statistically indistinguishable from random on 240 toy curves; Hessian models need a rational 3-torsion point. It concerns factor-base statistics, not symmetrisation, and shows no yield gain.
- `/home/user/crypto-autoresearcher/docs/ecdlp-literature-review-20260926.md` (line 319 and context). The EcGFp5 symmetrised ideal degree 2^{(m−1)²} = 2¹⁶ is unmeasured (KN-OPEN-9b4a2b; EXP-GFPN-05ff43 queued), consistent with the formula in §1. `analysis/ecdlp2m-iteration-20261009/m4-sat-cost-pilot/README.md` shows no S₅ builder exists.
- Not found: any repository file computing Hessian (general j) summation polynomials, level-3 theta coordinates or Kummer-line index-calculus coordinates. The "Kummer" hits are Kummer exponents and towers. My broad greps for "Hessian" timed out, so this absence is only partly established (§8).

## 4. Why it could lower the Gröbner cost or the degree of regularity

Each step and its assumption:

1. **The quotient is large and clean.** G(3,3,m) has order 3^{m−1}m! and polynomial invariants (verified). Assumption: ω ∈ F_q (affine f_T).
2. **The price is the degree of ϕ.** Prop. 3 forces d_P = 3^m against 2^{m−1}, a factor (3/2)^m·2 before any quotient, and ℓ > 2 loses the [−1] halving. Assumption: ϕ restricted to −Σ(A) is generically injective, true for the fibre count (9 verified at m = 2).
3. **Net: 3^{m²−m+1} against 2^{m²−m}** (plain S_m-symmetrised Semaev) **and 2^{(m−1)²}** (2-torsion). Hence 34× and 137× larger at m = 3; the gap grows with m.
4. **The F4 consequence** is not derived, since there is no dreg theorem. The measured proxy (§3) is a lower solving degree (12 against 14) on a 2.5× larger matrix and 5–8× the time. The tuple unknown carries degree 8 and F4 pays in columns.
5. **What would be needed to win:** an extra symmetry factor of about 137× at m = 3 over what E[3] supplies. Prop. 4 forbids (Z/3)² acting on one P¹, and the Hessian's second generator (the rotation) does not preserve ϕ.
6. **Linear algebra side:** factor base /3, matrix /9, but yield /3^{m−1} (9 at m = 3). The repository measured the same trade for 2-torsion (§20): constant-neutral at best, negative at n = 31. KN-FIND-007 (yield conservation) closes the rest.

No step lowers the Gröbner cost against the baseline.

**Kummer and theta models of elliptic curves.** Every coordinate on the Kummer line E/[−1] ≅ P¹ (Montgomery, level-2 theta (θ₀:θ₁), Jacobi-quartic or Edwards y) is a degree-2 even map. By FHJRV Prop. 5 each is h∘x∘τ_{−Q}, a Möbius image of x (also RESEARCH_EXOTIC_COORDINATES §1.1). Its summation polynomial is Semaev's after the substitution. The only content is whether f_T is affine, which is FHJRV Prop. 8 and Remark 9 (b square, about 3/4 of curves with a 2-torsion point). BRS's symmetric/anti-symmetric dichotomy appears to be the same condition with a Galois-representation classification; this is an inference from the abstract. It is a density statement, not a speed-up. Level-3 theta is the Hessian model itself (the Heisenberg group (Z/3)² acts linearly on P²), covered above; level-4 gives degree-4 maps, covered by the ℓ = 4 column.

## 5. Obstructions

- **Theorem (FHJRV Prop. 3 and §3.1):** ℓ | deg ϕ and D_ℓ injects for ℓ > 2. Together with the multi-homogeneous count this gives ℓ^{m²−m+1} against 2^{(m−1)²}.
- **Theorem (FHJRV Prop. 4):** no (Z/3)² or other non-cyclic action except E[2].
- **Theorem (FHJRV Prop. 5 and Lemma 6):** Kummer-line coordinates are Möbius images of x; there is nothing new to find in dimension 1.
- **Exact computation (desk, unverified):** S′₃ has degree 9 per variable and 22 invariant terms against 5; the resultant chain over-counts 81 against 27.
- **Measured (repository):** §14.3, solving degree 12 on a 2.5× larger matrix and 5–8× slower; §20, torsion symmetry halves yield and nets 0.76–4.0×; K5 §2.3, Q₄ is a wash.
- **Curve cost:** h ≥ 3 (or 9 for the monic Hessian with ω ∈ F_q) makes the matched rho √3–3× cheaper, so the distance to rho grows (AGENTS.md reports it as distance, not a gate). For q ≡ 2 mod 3 there is no polynomial-invariant structure.
- **Generic bound:** the oracle is F4 on a system built from group operations, so it sits on the Corrigan-Gibbs–Kogan/Shoup line and cannot move the exponent. At best a symmetry changes the per-solve constant, and here it raises it.
- **Success criterion:** an unfavourable constant is not an improvement. Nothing in this lead changes the headline count while lowering end-to-end S.

## 6. Minimal discriminating test (frozen protocol)

**T1, exact degree and support (no solver).**
- Instances: H_d over F_p with p ∈ {1009, 1021} (p ≡ 1 mod 3), three random d per prime with d³ ≠ 1 (record #E mod 9); plus, as control, one curve with a rational 2-torsion point and an affine frame (FHJRV Prop. 8) over the same p.
- Instrument: the repository's quotient/interpolation engine (`coordinate_quotients`, `Chart`, `run_quotient_boxed`). It needs one new Hessian curve and chart constructor with ϕ = X/Y; that coding step is not mechanical.
- Procedure: sample at least 3·1587 random triples (P₁,P₂,P₃) ∈ E(F_p)³ and set u₄ = ϕ(−ΣPᵢ). Fit the kernel of S′₄ over G(3,3,4)-invariant monomials in the box (per-variable ≤ 27, total ≤ 81). Do the same at m = 2 (box 9, total 18) to reproduce the 22 terms. For the control, run the standard 2-torsion fit (box 4, total 12).
- Record: per-variable degree, total degree, term count in the invariants, kernel dimension.
- Prediction: 27 per variable, term count in the hundreds against 13, kernel dimension 1.
- Kill (the card's): Hessian degree or term count not below FGHR's by the group-order ratio to within 2×. The prediction is that it fails outright: 9 against 2 and 22 against 5 already at m = 2.
- Cost: about 1 person-day for the constructor, under 5 CPU-minutes.

**T2, F4 proxy, no new code.** Run `examples/three_torsion.rs --descent-only` with `GB_ENGINE=f4`, `F4_MAX_DEGREE=24` on the j = 0 curve `y² = x³ + b` over F_{p³} for p ∈ {31, 37, 43} (p ≡ 1 mod 3; two decomposable targets per base, 300 s budget). Alongside, run the one-T 2-torsion `x`-line arm (`klein_descent.rs` row of §14.2) and Gaudry `x` on the same p. Record solving degree, matrix size, median F4 ms, F_p-unknowns and equations.
- Prediction: the `v`-line has solving degree ≥ 12 and median time at least 4× Gaudry's at each p, and at least 100× the one-T arm's.
- Kill: the `v`-line time falls below Gaudry's at any p, or within 2× of the one-T arm's.
- Cost: under 1 CPU-hour; mechanical tier from a runbook.

## 7. Ranking inputs

- Expected gain: none (negative; 34–137× larger systems at m = 3).
- Confidence: high that it is negative.
- Cost: T2 under 1 CPU-hour with no coding; T1 about 1 person-day. Recommendation: do not spend T1; T2 is optional confirmation.
- Tier: constant, net-negative (ℓ = 3, 4); Kummer-line coordinates: no effect (theorem).
- Depends on: nothing. The only live neighbours are the genus-3 transfer regime and LEAD-05/12 (generic-bound audit); none depends on this lead.

## 8. Open questions

1. The general-j Hessian system itself has not been run, only the j = 0 `v`-line proxy at one field (F₃₁³). The counting argument assumes the Bézout bound is attained. The S′₄ total degree 81 is extrapolated.
2. FHJRV's "extended version", Vitse's DLP 2014 talk and Huot's thesis might hold ℓ = 3 numbers. I found no manuscript; the search returned nothing and HAL gave "Access Denied" (Anubis).
3. BRS full text unread (503 twice, curl connection reset). The identification of their symmetric/anti-symmetric dichotomy with FHJRV Prop. 8 rests on the abstract.
4. Prime-field PDP: a T-stable base can be defined through w = u³, but no measurement exists and small-root methods (single-polynomial Coppersmith, reach 1/80) are already closed. Left to LEAD-10/14.
5. The non-split case (q ≡ 2 mod 3, base μ_{q+1}, Dickson-type coordinates) was not worked.
6. Genus-2 Kummer surfaces (theta level 2 in P³) are outside this card; they belong with the hyperelliptic/transfer notes.
7. The end-to-end ℓ = 3 accounting (factor base /3, linear algebra /9, yield /9) was not computed. By analogy with §20 it is a constant at best.
8. The repository-wide "Hessian" grep timed out, so the "no Hessian summation code" statement is partial.

## 9. Sources

URLs:
- https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/Semaev2torsion.pdf: HTTP 200, the tool returned the PDF as binary and its summariser failed; I extracted the text locally with `pdftotext` (892 lines) and read all of it.
- https://eprint.iacr.org/2025/543: HTTP 200, abstract page (tool paraphrase).
- https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/kummer_models.pdf: HTTP 503 twice (WebFetch), connection reset by peer (curl); unread.
- https://eprint.iacr.org/2025/543.pdf: HTTP 403. https://eprint.iacr.org/2015/1022.pdf: HTTP 403.
- https://hal.archives-ouvertes.fr/hal-01215623: Anubis "Access Denied".
- Web searches (BRS title and authors; Huot thesis, no result; Galbraith–Gaudry metadata) returned result lists only.

Repository paths read:
- `/home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md` and `LEADS.md` (standing constraints, card LEAD-03).
- `/home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md` (lines 1–140, 830–846, 1087–1511, 1767–1999, plus a grep for Hessian/Kummer/theta/Montgomery).
- `RESEARCH_SYMMETRIZED_SEMAEV.md` and `RESEARCH_K5_TORSION_JOUX_VITSE.md` (full).
- `/home/user/crypto/research/gaudry_quartic_d4_20261013/README.md` (full).
- Headers of `src/cryptanalysis/symmetrized_semaev.rs` and `fghr_full.rs`.
- `research/pair46_factor_bases_20261015/12-model-small-x/RESULTS.md` (first lines) and README (line 37).
- `/home/user/crypto-autoresearcher/docs/ecdlp-literature-review-20260926.md` (lines 306–325).
- `/home/user/crypto-autoresearcher/analysis/ecdlp2m-iteration-20261009/m4-sat-cost-pilot/README.md` (lines 30–50).

Scratchpad desk checks, unverified and not committed, in `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/calc/`: `hess3.py`, `hess3b.py`, `hess4.py`, `molien.py`, `counts.py`, `hesscount.py`.
