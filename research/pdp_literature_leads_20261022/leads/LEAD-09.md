# LEAD-09 — Ore / linearized-polynomial quotient-algebra elimination for the subspace constraint

**Verdict.** closed-by-measurement for the card as written (an explicit resultant in `x₁`, then `gcd` with `L_V(x₁)`). The measurement is an unverified desk computation, exact over `F_{2^n}` at `ℓ = 3…8`, to be re-run by the mechanical tier (§6). Implicit (non-materialised) variants are not excluded and are not new: they are the bilinear system of LEAD-01/05/07 (§8).
**Confidence.** medium. The `x₁`-resultant is dense: its degree is exactly `2^{ℓ+1}` (proved) and every coefficient but at most one was non-zero in 16 instances (computed); the dense output alone puts the method above the card's kill line. Asymptotic density at `ℓ = 14…16` is extrapolated, not computed.
**Expected gain if it works.** none against the matched oracles. Output-size floor for `m = 2`: `2^{ℓ+1}+1` coefficients, `0.80–0.88×` rf-mitm's counted multiplications at `ℓ = 10, 14, 16`, so at least `2.4×` above the kill threshold `rf-mitm/3`. For the `m = 3` pairs residual the floor is `3.2–3.9×` above rf-pairs. A constant against the PS-0 quartic arm is conceivable, but rf-pairs already beats that arm 40–68× and rf-mitm 3,007–115,160×. Moves no leaderboard row; the gap to rho (`S_IC/S_rho` = 3,053 at `n = 31`, 25,116 at `n = 37`, rf-mitm) is unchanged.
**Cost of the discriminating test.** about 0.25 person-day, under 1 CPU-hour, under 1 GB. The first stage (resultant density at `ℓ = 4…10`) decides the card's kill criterion by itself.

## 1. Mechanism (precise)

**Objects.** `K = F_{2^n}`, `V ⊂ K` an `F₂`-subspace of dimension `ℓ` (repository: `⟨1, z, …, z^{ℓ−1}⟩`), `L_V(x) = Σ_{i≤ℓ} a_i x^{2^i}` monic, separable (`L_V′ = a₀ ≠ 0`). `B = K[x]/(L_V)` has `K`-dimension `2^ℓ` and is split, `B ≅ K^{V}`; it is not a field. For the curve `y²+xy = x³+ax²+b`, the repository's `binary_semaev.rs` (line 70) gives
`S₃(x₁,x₂,x₃) = (x₁+x₂)²x₃² + x₁x₂x₃ + (x₁x₂)² + b`.

**Structure the card does not state (derived here, checked on 200 random tuples per instance).**
1. In each variable separately `S₃` is `A x² + B x + C`, an *affine* `2`-linearised polynomial of `2`-degree 1. Reducing it modulo `L_V` is the Ore picture. For `x₂`: `A = (x₁+x_R)²`, `B = x₁x_R`, `C = (x₁x_R)²+b`.
2. In `F₂`-coordinates `x₁ = Σ aᵢbᵢ`, `x₂ = Σ cⱼbⱼ` (Frobenius is `F₂`-linear), `S₃ = 0` is `n` **bilinear** equations in `(a, c) ∈ F₂^ℓ × F₂^ℓ`. At `m = 2`, `n = 3ℓ` (the card's sizes) this is `3ℓ` equations in `2ℓ` unknowns.
3. `S₃(x₁, v, x_R)` as a polynomial in `x₁` has the two roots `x(R ± P_v)`. So `Res_{x₂}(S₃, L_V(x₂)) = ∏_{v∈V} S₃(x₁,v,x_R) =: ℛ(x₁)` is the vanishing polynomial, in `x₁`, of the candidate list `{x(R ± P_v)}`: the very list rf-mitm and rf-pairs enumerate, 5 multiplications per member.

**The elimination.** `ℛ = A^{2^ℓ}·N_{K(θ)/K}(L_V(θ))` with `Aθ²+Bθ+C = 0`. `L_V(θ) = r₁θ + r₀` is computed by `ℓ` Frobenius steps `θ^{2^{i+1}} = (θ^{2^i})²`, with `θ^{2^i} = p_iθ + q_i`, `p_i = (B/A)^{2^i−1}`, `q_i = p_{i−1}²C/A + q_{i−1}²`. This is exactly the repository's `roots_in_subspace` (reduce `L_V` modulo the polynomial, then `gcd`), with `x₁` kept symbolic. Then `ℛ mod L_V(x₁)` and `gcd(ℛ, L_V)` or `Res(L_V, ℛ)`; `gcd ≠ 1` iff `R` is a sum of two factor-base abscissae.

**Degree and size.** Leading coefficient of `ℛ` in `x₁` is `L_V(x_R)²` (the `x₁²` coefficients are `x_R²+v²`), non-zero iff `x_R ∉ V`; so `deg ℛ = 2^{ℓ+1}` exactly (proved). Number of terms: §4.

**Sizes.**
- `m = 2` at `n = 47`, `ℓ = 16`: `|V| = 65,536`, `deg ℛ = 131,072`, `dim_K B = 65,536`.
- `m = 3` pairs residual (`x₁` outer, `x₂,x₃` eliminated): `S₄` has degree 4 in each variable, so `Res_{x₃}(S₄, L_V(x₃)) = ∏_{v∈V}S₄(x₁,x₂,v,x_R)` has degree `4·2^ℓ` in `x₂` (262,144 at `ℓ = 16`), and the `4`-dimensional algebra `K(x₂)[θ]/(S₄)` carries the Frobenius steps.
- `m = 4` (`n = 48`, `ℓ = 12`): `S₅` has degree 8, so `8·2^ℓ` per outer pair, `2^{2ℓ}` outer pairs.

**Bound the formulation obeys.** No Gröbner step, so no degree of regularity. The bound is output size: `deg ℛ = 2^{ℓ+1}` at `m = 2`, `4·2^ℓ` per `x₁` at `m = 3`, `8·2^ℓ` per pair at `m = 4`. (Desk count, unchecked: the HKPYY `m = 2` chain has a resultant of degree `O(d)`, `d = deg λ`, so the same order.)

## 2. Prior art: literature

- Huang, Kosters, Petit, Yeo, Yun, *Quasi-subfield polynomials and the ECDLP*, J. Math. Cryptol. 14(1):25–38, 2020, doi 10.1515/jmc-2015-0049. **read-at-source** (frozen full text `inputs/HUANG-2020-JMC-QSP/paper_fulltext.md` §3.1–3.2, lines 405–634, and `KN-LIT-0a321c`). Claim: substitution `X^{q^{n′}} → λ(X)` gives `m` equations in `m` variables over `K` with no Weil descent; Lemma 3.1 cost `Õ(m^{5.188}(3d)^{4.876m²})`; Remark 3.1 needs `d ≲ q^{0.102n/m²}`. For generic `V`, `λ` is dense, `d = 2^{ℓ−1}`, and `log₂d/(n′²/n) ≈ 3`, far outside the quasi-subfield range.
- Caruso, Le Borgne, *Fast multiplication for skew polynomials*, ISSAC 2017, arXiv 1702.01665v1. **read-at-source** (abs page 200; PDF read via `pdftotext`). Claim: in `L[X,σ]` with `r = [L:K]`, multiplication of degree `d ≥ r` is `Õ(d r^{ω−1})` operations in `K`; for `d ≤ r` it is `Õ(d^{ω−2}r²)`. Euclidean division and gcd are `Õ(SM_{≥1}(d,r))` (Prop. 3.1), about `d^{1.69}r` for `d ≤ r^{0.76}` at `ω = 2.37`. Here `K` is the fixed field; for 2-linearised polynomials over `F_{2^n}` that is `F₂`, `r = n`, `d = ℓ`.
- Puchinger, Wachter-Zeh, *Fast operations on linearized polynomials and their applications in coding theory*, J. Symb. Comput. (v3, 2017), arXiv 1512.06520. **read-at-source** (same route). Claims, over `F_{q^m}` and in the `q`-degree `s`: multiplication `O(s^{min{(ω+1)/2,1.635}})`; division (Cor. 10) `O(s^{min{(ω+1)/2,1.635}} log s)`; the fast linearised Euclidean algorithm (Cor. 11) `O(s^{min{(ω+1)/2,1.635}} log²s)`; minimal subspace polynomial and multi-point evaluation (Thm. 15) `O(s^{max{log₂3,…}})`. Composition is multiplication in the skew ring (§2, "the multiplication equals the composition `a(b(x))`"), so it has the multiplication cost. **All of these are in the `q`-degree `s = ℓ`, not the ordinary degree `2^ℓ`.**
- Euler, Petit, *New results on quasi-subfield polynomials*, arXiv 1909.11326v2 (journal version not read). **not fetched by me; read via the repository notes** `KN-LIT-4fe9d2` (marked read-at-source there) and `research/notes/index-calculus/RESEARCH_QUASI_SUBFIELD.md` lines 196–227, so secondary here. Claim: for linearised `L = X^{p^{n′}} − λ` that splits completely, `β = ℓn/n′² ≥ 3/4`; target `β < 0.103`.
- Ore, *On a special class of polynomials*, Trans. AMS 1933; Lidl–Niederreiter *Finite Fields* §3.4; von zur Gathen–Gerhard *Modern Computer Algebra* (half-gcd, `Õ(N)` for degree `N`; cited by Caruso–Le Borgne as [6] Thm 11.5). **recalled.** Claims: the linearised ring is Euclidean; ordinary-degree gcd and resultant are quasi-linear in the degree.
- Faugère, Safey El Din, Spaenlehauer, Gröbner bases of bilinear (bidegree (1,1)) systems, arXiv 1001.4004 (identifier as given in LEAD-12; title not verified). **recalled via LEAD-12 only**, not fetched.

**Costs recorded for 2-linearised polynomials of 2-degree `ℓ` over `F_{2^n}`.**
- Composition = skew multiplication: PWZ `O(ℓ^{1.635})` field operations; Caruso–Le Borgne `Õ(ℓ^{ω−2}n²)` `F₂`-operations.
- Right Euclidean division: PWZ `O(ℓ^{1.635} log ℓ)`.
- gcd (linearised Euclid): PWZ `O(ℓ^{1.635} log²ℓ)`.
- Unverified desk estimate at `ℓ ≤ 20`: schoolbook is `≈ ℓ²` field multiplications (`≤ 400`), below every asymptotic crossover. This is why the repository builds `L_V` in `O(ℓ²)` (`subspace_poly`).

The cost is negligible. It applies to `L_V` and its compositions, not to `ℛ(x₁)`, which is not linearised: its ordinary degree is `2^{ℓ+1}` and the Ore machinery does not see it.

## 3. Prior art: these repositories

- `src/cryptanalysis/semaev_decomp.rs` (header and `roots_in_subspace`): **supports** the Ore side. It already does `gcd(f, L_V mod f)` in `ℓ` squarings, `O(ℓ)` per pair. The header states that any enumeration-based oracle sits at `Θ(2^n)` relation cost independent of `ℓ`. The card's new part is only the symbolic second variable.
- `src/cryptanalysis/binary_semaev.rs` line 70: the `S₃` formula used in §1.
- `src/cryptanalysis/ic_framework/rootfind.rs`: **closes** the comparison. rf-mitm per refuted target `5|F|−3` multiplications, `2|F|` squarings, one inversion, `2|F|` lookups (`m = 3`; `m = 2` probes `R` itself); rf-pairs `≈ 10` multiplications a pair over `|F|²/2` pairs.
- `research/pdp_root_finder_20261012/README.md`: rf-mitm 637 / 2,317 / 40,922 multiplications at `ℓ = 8 / 10 / 14`; rf-pairs 83,069 / 1,080,653 / 335,044,787; slopes 1.000 ± 0.027 and 1.995 ± 0.055 bits per rung (floors); class **engineering**; wall 3,007× to 115,160× below the PS-0 quartic baseline; table 804.9 MB at `ℓ = 14`.
- `research/pdp_root_finder_ecbench_20261013/README.md`: end-to-end rf-mitm `S_IC/S_rho` = 15 / 61 / 3,053 / 25,116 at `n = 19 / 23 / 31 / 37`; oracle phase 71–94 % of `S`; ≤ 1.26× over `mitm`.
- `src/cryptanalysis/quasi_subfield.rs` header and `research/notes/index-calculus/RESEARCH_QUASI_SUBFIELD.md`: **closes** the HKPYY substitution for generic `V`. Census at `n = 31`: no yield gain (`ic_leads_review_20261018` row 8: yield 0.214 vs 0.204–0.230).
- `/home/user/crypto-autoresearcher/knowledge/open-problems/KN-OPEN-007.md`: **supports** the dense reading. The recursive Semaev resultant tree `S_m = Res_y(S_k, S_{m−k+2})` is the same object; its scoped negative (bond ranks near full) is the tensor view of "dense".
- `research/pdp_literature_leads_20261022/leads/LEAD-12.md` §8 item 2: asked this exact question ("is the Ore elimination dense? count terms at `ℓ = 4…8`"). Answered in §4.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does remove Weil descent and the degree of regularity: a Gröbner-free exact decision procedure, in place of `n` bilinear equations in `2ℓ` unknowns at `n = 3ℓ`. Gröbner on that system is far above enumeration in this repository: msolve reads solving degree 4 (some 3) on the binary cells through `(ℓ, m) = (11, 4)` (LEADS.md standing constraints), F4 slopes are 1.6–2.4 bits per tower variable on the prime tower, and the root-finders sit 3,007–115,160× below F4/baseline at stage level. A Gröbner-free `Õ(2^ℓ)` decision is therefore a real change of method. That is the content of the lead; it is not a change of scale.

It does not lower the cost below the enumeration scale, for one measured reason and one structural reason.

- **Measured (unverified desk check, exact arithmetic; `scratchpad/lead09_check.py`, not committed).** `ℛ(x₁) = ∏_{v∈V}S₃(x₁,v,x_R)` over `F_{2^{3ℓ}}`, `V = ⟨1,z,…,z^{ℓ−1}⟩`, 16 instances.

| ℓ | n | `deg ℛ` | non-zero coefficients | ANF monomials of `x₁ ↦ ℛ(x₁)\|_V` non-zero |
|--:|--:|--:|--:|--:|
| 3 | 9 | 16 | 16 of 17 | 6 of 8 |
| 4 | 12 | 32 | 32–33 of 33 (8 inst.) | 16 of 16 |
| 5 | 15 | 64 | 64–65 of 65 (4 inst.) | 32 of 32 |
| 6 / 7 / 8 | 18 / 21 / 24 | 128 / 256 / 512 | 129 / 257 / 513 of the same | 64 / 128 / 256 of the same |

  The roots of `ℛ` in `V` equalled brute force on every planted instance (9 with a decomposition, `ℓ = 4, 5`; 2–3 roots each). Degree `2^{ℓ+1}`, full support, Boolean degree `ℓ`. At these sizes there is no sparse or low-weight representation of `ℛ`, and none inside `B = K[x₁]/(L_V)` either.
- **Structural.** `ℛ` is the vanishing polynomial of the `2|F|`-element candidate list. Any method that forms it has output `Ω(2^ℓ)`; rf-mitm lists the same points at 5 multiplications each. Assumption: density persists to `ℓ = 14…16` (checked to `ℓ = 8`).

**Operation counts at the card's sizes** (`n = 31, 41, 47`, `ℓ = 10, 14, 16`; rf-mitm from the README at `n = 30` and `42`, and `5·2^{15}` extrapolated at `ℓ = 16`; the Ore floor charges one operation a coefficient; Ore realistic figures are unverified estimates):

| n | ℓ | rf-mitm mults / target (`5\|F\|−3`) | Ore `m=2` floor `2^{ℓ+1}+1` | floor ÷ rf-mitm | kill line `rf-mitm/3` | Ore `m=2`, realistic | rf-pairs mults (`m=3`) | Ore `m=3` floor `4·2^{2ℓ}` |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 31 | 10 | 2,317 | 2,049 | 0.88 | 772 | ≳ 1.0·10⁴ (to 10⁶) | 1.08·10⁶ | 4.2·10⁶ |
| 41 | 14 | 40,922 | 32,769 | 0.80 | 13,641 | ≳ 2.3·10⁵ (to 10⁷) | 3.35·10⁸ | 1.07·10⁹ |
| 47 | 16 | ≈ 1.6·10⁵ | 131,073 | 0.80 | ≈ 5.5·10⁴ | ≳ 1.0·10⁶ (to 10⁸) | ≈ 5·10⁹ | 1.7·10¹⁰ |

- **Realistic `m = 2`.** The lower figure is Horner-in-`B`: `ℓ` passes over `2^ℓ` coefficients, `ℓ·2^ℓ`, about `6.4×` rf-mitm at `ℓ = 16`. The upper figure adds the norm (one product of two degree-`2^ℓ` polynomials) and the half-gcd, with `log` factors of order `ℓ` and constants above 1. Both are unverified estimates.
- **`m = 3`, `x₁` outer.** Total `2^ℓ × (≥ 4·2^ℓ)`, above rf-pairs's `5|F|² ≈ 1.25·2^{2ℓ}` by `3.2–3.9×` already at the floor, and `2.6·10⁴×` above rf-mitm at `ℓ = 14`.

**Memory is not the discriminator.** Ore is table-free, like rf-pairs. But rf-mitm's table, `|F|²` keys at 12 B, is 805 MB at `ℓ = 14` and about 12.9 GB at `ℓ = 16`, most of the container's 15 GiB. So at `n = 47` Ore's only advantage over rf-mitm would be memory, and it pays `≳ 10⁶` (up to `10⁸`) operations against `1.6·10⁵`.

**Pricing (§5a).** The only way Ore beats rf-mitm in a ranking is pricing its `K`-operations near 0 GAE. §5a forbids ranking where the order does not hold across 0, 0.1 and 1 GAE per operation; at 0.1 and 1 GAE Ore loses.

## 5. Obstructions

- **Exact:** `deg ℛ = 2^{ℓ+1}` (theorem) and density (computation at `ℓ ≤ 8`) force output `Ω(2^ℓ)`. rf-mitm's counted work is `2.5·2^ℓ` multiplications; the Ore floor is `2·2^ℓ` before any arithmetic. The kill criterion (Ore ≤ rf-mitm/3 at `n = 47`) is violated by the floor alone, by `2.4×`.
- **Theorem on the fast algorithms.** The linearised-polynomial bounds are in the `q`-degree. They do not apply to the `x₁` direction. The Ore saving is already taken in `semaev_decomp.rs` (the `x₂`, `x₃` direction, `O(ℓ)` a pair).
- **Generic line.** Not a generic-group algorithm: it does `K`-arithmetic and reads coordinates, so CGK, Shoup and DKM do not apply literally (LEAD-12, row 09). But its cost is the size of the candidate list `{x(R ± P_v)}`, so it is a membership-test-by-bulk-evaluation oracle in disguise. For `m = 3` with `x₁` outer it gives `2^ℓ · 3!2^{n−3ℓ} · Θ̃(2^{2ℓ}) = Θ̃(2^n)`, the semaev_decomp identity: the enumeration line, exponent unmoved, constant above the group-law arms.
- **HKPYY.** For generic `V`, `d = 2^{ℓ−1}` and `β ≈ 3` against the proven floor `3/4` (Euler–Petit) and target `0.103`. Rojas' `(3d)^{4.876m²}` is far above `2^{n/2}` at `m ≥ 2` (Remark 3.1: `d > q^{0.102n/m²}`).
- **Subfield case.** For composite `n` with `V = F_{2^ℓ}`, `ℓ | n`, `L_V = x^{2^ℓ}+x` is sparse and the problem is the known Diem/Gaudry descent. The card's `n ∈ {31, 41, 47}` are prime: no subfield.
- **Measured negatives.** rf-mitm 1.000 bits per rung (floor); the end-to-end gap `S_IC/S_rho` 3,053 / 25,116 at `n = 31 / 37`; the oracle is 71–94 % of `S`, and a slower oracle cannot lower that.

## 6. Minimal discriminating test (frozen protocol)

**Instrument.** A new ~150-line Rust example (it must not be Python; no such file exists yet) built on `semaev_decomp::{Gf2, Poly}`, with the product tree over `V` done by schoolbook or Karatsuba. Cross-check three instances against an external CAS resultant (msolve, Singular or Sage `resultant`) if available.

**Instances.** `n = 3ℓ`, `ℓ = 4…10`, `y²+xy = x³+1`, `V = ⟨1,z,…,z^{ℓ−1}⟩`, 16 random targets `x_R ∉ V` per `ℓ`, seeds `20261023 + ℓ`. Add the card's `n ∈ {31, 41, 47}` with `ℓ = 10, 14, 16` for the counted-operation comparison only (no coefficient counting above `ℓ = 12`).

**Recorded per instance.**
1. `deg ℛ` against `2^{ℓ+1}`.
2. Non-zero coefficient count and density.
3. ANF support of `ℛ|_V`.
4. Multiplications counted in the product tree (Ore-explicit), and, for a Horner-in-`B` route, the multiplications counted in its passes.
5. rf-mitm's counted multiplications at the same `ℓ`, taken from `pdp_root_finder_20261012/ANALYSIS.json`.
6. Root-set equality of `gcd(ℛ, L_V)` with brute force at `ℓ ≤ 8`.

**Prediction.** `deg ℛ = 2^{ℓ+1}` on every instance; density ≥ 0.99; Ore-explicit multiplications ≥ 3·2^{ℓ+1}, which is ≥ 2.4× rf-mitm's.

**Kill criterion (card's):** Ore not ≥ 3× below rf-mitm at `n = 47`. Predicted to be killed at the first stage. **Revive only if** density is ≤ 0.5 at some `ℓ ≥ 8`, or the non-zero count grows slower than `2^{0.9ℓ}` over `ℓ = 4…10`. Each would be a poly(ℓ)-size structure in `ℛ`, which nothing here predicts.

**Cost.** about 0.25 person-day (write the example, run it, tabulate), under 1 CPU-hour (`ℓ = 10` product tree of 1,024 quadratics, degree 2,048), under 1 GB. No judgement calls: every stage reads a counter against a number frozen above.

## 7. Ranking inputs

expected gain: none (≤ 1× against rf-mitm and rf-pairs; leaderboard-neutral) | confidence: medium | cost: 0.25 person-day, < 1 CPU-hour | tier: constant (negative) | depends on: nothing; the open residue (implicit variants) folds into LEAD-01 / LEAD-05 / LEAD-07.

## 8. Open questions

1. **Implicit representation.** `ℛ(a)`, `a ∈ F₂^ℓ`, has a poly(ℓ)-size circuit: `ℓ` Horner steps, each multiplication by a binomial in `x₁^{2^k}`. The question is therefore whether `∃a: ℛ(x(a)) = 0` has an algorithm below `2^ℓ`, i.e. the overdetermined bilinear system `3ℓ` equations / `(ℓ,ℓ)` unknowns. Not settled from here; no result from the Ore machinery bears on it, and it is the LEAD-01 / 05 / 07 question.
2. **Asymptotic density.** Computed to `ℓ = 8` here, to be extended to `ℓ = 10` by §6; `ℓ = 14, 16` rely on the structural argument (`ℛ` vanishes on the full candidate list).
3. **Euler–Petit** is cited through the repository note, not at source; Rojas' Theorem 2.1 and the `4.876` exponent are taken from `KN-LIT-0a321c`. CGHW's elliptic-curve theorem (ePrint 2026/384) is still unread (LEAD-12).
4. **`m = 4`** counts rest on `S₅` degree 8 per variable (recalled from the Semaev recursion `2^{m−1}`; not measured here).
5. **Realistic Ore constants** (half-gcd, polynomial product over `F_{2^n}`) are estimates; moot, since the output-size floor already fails the kill line.

## 9. Sources

Fetched:
- `https://arxiv.org/abs/1702.01665`: HTTP 200, abstract page; no complexity formulas on the page.
- `https://arxiv.org/abs/1512.06520`: HTTP 200, abstract page; no formulas on the page.
- `https://arxiv.org/pdf/1702.01665` and `https://arxiv.org/pdf/1512.06520`: HTTP 200, binary PDF. The summariser could not read it, so I converted the saved copies with `pdftotext` in the scratchpad and read the text myself (Caruso–Le Borgne intro, §3 Euclidean division and gcd; Puchinger–Wachter-Zeh Table 1, Cor. 10, Cor. 11, Thm. 15).

Repository paths read:
- `/home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md`
- `/home/user/crypto/research/pdp_literature_leads_20261022/LEADS.md` (standing constraints, LEAD-09)
- `/home/user/crypto/research/pdp_literature_leads_20261022/leads/LEAD-12.md`
- `/home/user/crypto/research/pdp_literature_leads_20261022/README.md` (grep)
- `/home/user/crypto/src/cryptanalysis/semaev_decomp.rs` (header, function index)
- `/home/user/crypto/src/cryptanalysis/binary_semaev.rs` (grep)
- `/home/user/crypto/src/cryptanalysis/ic_framework/rootfind.rs`
- `/home/user/crypto/src/cryptanalysis/quasi_subfield.rs` (header)
- `/home/user/crypto/research/pdp_root_finder_20261012/README.md`
- `/home/user/crypto/research/pdp_root_finder_ecbench_20261013/README.md`
- `/home/user/crypto/research/notes/index-calculus/RESEARCH_QUASI_SUBFIELD.md` (lines 195–240)
- `/home/user/crypto/research/ic_leads_review_20261018/README.md` (grep)
- `/home/user/crypto-autoresearcher/knowledge/literature/KN-LIT-0a321c.md`
- `/home/user/crypto-autoresearcher/knowledge/literature/KN-LIT-4fe9d2.md` (front matter only)
- `/home/user/crypto-autoresearcher/inputs/HUANG-2020-JMC-QSP/paper_fulltext.md` (lines 405–634, grep)
- `/home/user/crypto-autoresearcher/knowledge/open-problems/KN-OPEN-007.md`

Unverified desk checks (scratchpad, not committed, not measurements): `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/lead09_check.py` and `lead09_check2.py`.
