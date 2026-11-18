# Red-team review: Track C global-lift desk analysis

**Verdict: HOLDS WITH CORRECTIONS.** The exponent closure is valid in a narrow setting: a fixed relation dimension k, fresh uniform masks in each trial, lifts of polynomial size over Q (or over a number field of bounded degree), relations only among the sampled lifted points, and function fields of bounded Euler characteristic chi. It is not a new result. The same closure was committed on 2026-11-12, using Petsche 2006 where this note uses Masser, and a checker has already reviewed it. Several of the note's wider claims are unsupported: number fields of polylogarithmic degree, the bit thresholds in §3.4, "closed" for p-dependent curves in R2, and the statement that the five leads are closed. All statements below are `[pcc:unverified]`.

Tags: [src] = I read it in the source file; [lit] = I checked it against the primary literature; [inf] = my own inference or computation.

## What was reviewed

- **Subject.** `/home/user/crypto/research/ic_lead_queue_20261114/C_global_lift/DESK.md`, sha256 `c52892a0ec50d4c3de6b5afd831cffadaa976d6c8f3bc2fbe6fb955719de16c5`.
  - The file is **untracked**: `git status` shows `?? research/ic_lead_queue_20261114/`. It is not durable evidence.
  - This review is pinned to the hash above. Any edit voids it.
  - The note's inventory sources (`LEADS_UNTESTED.md`, `LEADS_UNCATALOGUED.md`) live only in the session scratchpad.
- **Lean sources.**
  - `research/jksst_xedni_audit_20261101/lean/JKSSTConditionalHeightCore.lean`, sha256 `fd45c906…`.
  - `JKSSTSuccessComposition.lean` in the same directory, sha256 `b5447035…`.
- **Masser.** Bull. SMF 117 (1989) 247–265, fetched from Numdam (`https://www.numdam.org/item/10.24033/bsmf.2120.pdf`), sha256 `5b78c165…`. I read page 249 as an image: the Theorem and Corollaries 1–3.
- **Prior committed work** (commit `454757e9fa`, 2026-11-12; an ancestor of the note's base `0c2cc1b408`):
  - `research/prime_field_ic_closure_20261112/finish/lifting/PROOF.md`;
  - §5 of `research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md`, which records the checker's verdict and its scope corrections.

## What holds

1. **The two Lean statements are quoted faithfully.** [src]
   - `bounded_relation_of_certified_height_ratio` concludes `∃ n ≠ 0, |n i| ≤ coefficientCutoff T (finrank E) (card ι) K ∧ ∑ n i • P i = 0`. The cutoff unfolds to `T·⌈1 + 2·(card ι)·√K⌉^(finrank E)`, which matches the note.
     - Its hypotheses: `rank_gap : finrank E + 1 ≤ card ι`, `ratio_bound : M/m ≤ K`, `radius_bound : R ≤ card ι · C · √M`, `range_mem` (every sum `∑ c_i P_i` with `c ∈ [0,C]^ι` lies in `S`), `mapsTo`, `fiberBound ≤ T`, `inside : ‖y‖ ≤ R`, and `separated : √m ≤ ‖y − z‖`.
     - Heights enter only through `radius_bound` and `inside`. Taking `S` as that coefficient box, `π` into `⟨P̃⟩ ⊗ ℝ` normed by `√ĥ`, `R = k·C·√M` and `m = λ₁` meets every hypothesis, with `dim = rank⟨P̃_j⟩ ≤ k − 1` when the lifted points are dependent.
   - `ordinary_success_bound_of_group_relation_certificates` concludes `success.card · |F| ≤ ((2C+1)^card ι − 1) · (|F|^card ι)²`. Its hypotheses are `hC : C < ringChar F`, `relationCertificate` (for every success, some bounded nonzero group relation exists) and `scalarReductionBridge` (every such relation implies `dot(n,s) = x·dot(n,t)`). The sample space is uniform over `(F^ι)²`.
   - No `sorry`, `admit`, `axiom`, `native_decide` or `implemented_by` appears in any `.lean` file of the directory. Five plain `decide` calls are kernel-checked.
   - `AxiomAudit.lean` lists both declarations with the allowed axioms `propext`, `Classical.choice` and `Quot.sound`.
   - Proof-gate logs record `PASS 294 thm 0 trusted research/jksst_xedni_audit_20261101/lean`, for example `research/inside_gap_n4_20261102/results/lean-proof-gate-41484047855.txt`, line 92. That log carries no source hash, and I did not rebuild.
2. **The composition (★) is within those hypotheses**, provided:
   - success implies that the lifted points are dependent;
   - one uniform bound C(p) covers every possible trial output;
   - the reduction is a homomorphism carrying lifted point j to `R_j`. [src][inf]
3. **Masser's Corollary 1 is quoted exactly.** [lit]
   - The statement is `q(P) ≥ 1/(C₁ w D³ (w + log D)²)` for non-torsion `P ∈ E(K)`, with `[K:k] ≤ D`, `w ≥ 1` bounding `h(1 : g₂ : g₃)` for `y² = 4x³ − g₂x − g₃`, and `C₁` "effective, depending only on the degree of k".
   - Corollary 2 is the torsion bound `C₂ √w D (w + log D)`.
   - For `E/Q` (so `k = Q` and `D = 1`) with `w = O(log p)` (or poly(log p) for any polynomial-time construction), this gives `λ₁ ≥ 1/(C₁ w³)`, which is polylogarithmic.
   - Unlike Petsche, it needs no prime of bad reduction. That is a genuine small addition over the 2026-11-12 proof, for curves over a fixed number field with everywhere good reduction.
   - Masser normalises `q(P)` from the height of `(1:x:y)`, which differs from the usual ĥ by a constant factor. This is harmless for the exponent.
4. **Exponent arithmetic.** With polylogarithmic K, `C = T(1+2k√K)^(k−1)` is polylogarithmic, and `k·log_q(2C+1) = O(k² log log p / log p) → 0` for fixed k. [inf]
5. **Function-field facts are correct.** [lit, recalled]
   - Over P¹ with χ ≥ 1: `b₂ = 12χ − 2`, Igusa's `ρ ≤ b₂`, and Shioda–Tate give `r ≤ 12χ − 4` (8 for rational surfaces).
   - Shioda's local contributions lie in `(1/|Φ_v|)Z`. Since `Σ e(F_v) = 12χ`, the denominator L is bounded in terms of χ, so `λ₁ ≥ 1/L`.
   - Specialisation at a smooth fibre is a homomorphism.
6. **The §3.4 arithmetic is right as computed.**
   - `16·19⁸ ≈ 2^38`.
   - The box count `(2·2^38+1)^9 ≈ 2^351` gives `~700` bits.
   - The Cramér–Minkowski cutoff `T·γ₈⁴·K⁴ = 256K⁴` (Hermite with γ₈ = 2, plus Hadamard) gives `~162` bits at K = 1 and `~401` bits at K = 10. [inf]

## Findings, by severity

### H1. The closure duplicates a committed, checker-reviewed result without citing it (unsupported novelty; scope dropped)

- **Where.** §0 ("Sources checked"); §3.1 ("(★) is the general form…"); §6 ("(★) is a new derived composition"); the verdict bullet "needs Masser's unconditional height lower bound … and not Lang's conjecture".
- **The problem.** Commit `454757e9fa` (2026-11-12) already contains the same argument in `research/prime_field_ic_closure_20261112/finish/lifting/PROOF.md`:
  - Theorem 1: a height-ratio certificate plus the same two Lean declarations;
  - Theorem 2: Petsche 2006 Thm 2 with Szpiro ratio `σ ≤ log|Δ|/log 11`, replacing Lang; result `ε ≤ (log p)^{O(r²)}/q`.
  - The closure note records the checker's verdict "sound with gaps" (`PRIME_FIELD_IC_CLOSURE_20261112.md` §5).
  - The note's base `0c2cc1b408` contains that commit. A text search for `jksst_xedni_audit` finds it.
- **Scope the checker had already corrected, which the note drops:**
  - every mask draw counts as a trial;
  - relations only among the sampled lifted points;
  - number-field lifts are "only a sketch";
  - Silverman–Suzuki with growing rank is open;
  - r ≥ 7 (≥ 6 under worst-case ρ) is open at concrete sizes.
- **Smallest correction.**
  - Cite PROOF.md and the checker record.
  - Restate the note's contribution as "Masser Cor. 1 as an alternative to Petsche Thm 2". It covers curves with everywhere good reduction over a fixed number field, but its constant is not explicit.
  - Adopt the checker's scope list.
- **Cheapest test.** Diff (★), Theorem 1 and Theorem 2 side by side. [src]

### H2. The extension to number fields of polylogarithmic degree is unsupported

- **Where.** §3.2: "Over a number field of degree n, Masser's Corollaries 1–2 with D = n keep K and T polynomial in n and log p", and "This covers SNFS, cyclotomic and Solinas fields of polylogarithmic degree".
- **The problem.** Masser's C₁ and C₂ depend "only on the degree of k", where k is the field over which E is defined. The paper does not make that dependence explicit (page 249). [lit]
  - Taking D = n is legitimate only when `Ẽ` is defined over Q and only the points lie in K.
  - In SNFS, cyclotomic and Solinas lifts the curve's coefficients are in K. Then `k = K`, `D = 1`, and `C₁ = C₁([K:Q])` has unknown growth in n.
  - If C₁ or C₂ grew like `n^{O(n)}`, then at `n ≈ log p` the bound becomes `p^{Ω(log log p)}` and the o(1) fails.
- **Smallest correction.** Restrict the claim to bounded `[K:Q]`, or to `Ẽ/Q` with points over K. Alternatively use Petsche Thms 1–2, which are explicit and polynomial in d (factor `d³`) but need at least one bad prime, and inherit the checker's open points (absolute versus relative normalisation, residue degree).
- **Cheapest test.** Find an explicit `[k:Q]` dependence in Masser §6 or in a later explicit version, such as David 1997. Otherwise demote the claim to bounded degree. [lit][inf]

### M1. The "non-adaptive versus pooled" split is not exhaustive, and the adaptive-trial obligation is mishandled

- **Where.** §3.1 ("non-adaptive lift family … fresh points"); §3.2 ("Obligation 7 … handled by restricting to non-adaptive trials (§3.3)"); §4 regime 2; §7.
- **The problems:**
  1. **Wrong cross-reference.** §3.3 contains no discussion of adaptivity. [src]
  2. **The restriction is stronger than needed.** Committed Theorem 1(ii) allows t chosen by any history-dependent rule, provided s is fresh. It bounds `Pr[success within N trials] ≤ Nε` by a union bound over (trial, relation) pairs, without any hazard recurrence. [src]
  3. **A realistic regime is missing.** Neither "fresh per trial" nor "fixed global object (R2)" covers a pool of N reduced points reused across tuples, each tuple with its own global object, where tuples are chosen **by inspecting coordinates**.
     - For such tuples the 1/q event bound does not apply. Mod-p collinearity, a Semaev condition visible from coordinates, *is* a bounded relation.
     - The union bound over all `C(N,k)` tuples only forces `N ≳ q^{1/k}/(2C+1)`. No closure follows; the cost moves into an unpriced selection search, the summation-polynomial / k-sum problem. [inf]
  4. **Further uncovered variants.** Relations that also use other Mordell–Weil points with unknown logs, or endomorphism or isogeny images. The second case covers the Cheon 2-isogeny variant named in the ICP-char0 lead; the committed Prop 3 needs a box exponent `e·k` and known-scalar action for it. [src][inf]
- **Smallest correction.**
  - State (★) as "per tested tuple whose masks are fresh and drawn without inspecting point coordinates; every draw is a trial".
  - Add a residue R3, "pooled points with coordinate-based tuple selection", with the `C(N,k)(2C+1)^k/q` bound, routed to the summation-polynomial family.
  - Fix the §3.3 reference, or adopt Theorem 1(ii).

### M2. The headline threshold is mislabelled: "vacuous below roughly 160–700 bits" is a conditional-K figure

- **Where.** The verdict bullet ("Its explicit constant is vacuous below roughly 160–700 bits of q, or more") and §3.4.
- **The problem.** 160, 400 and 700 bits use `K = 1` or `K ≈ 10`, which are Lang-like, i.e. conditional values. The unconditional route has `K = O((log p)^4)·C₁`, and C₁ is effective but not explicit, so no threshold can be computed from the paper.
  - Even with every constant set to 1 (`K = (ln q)^4`, k = 9), the bound is vacuous below about **3,390 bits** with the Cramér–Minkowski cutoff and about **3,990 bits** with the Lean cutoff.
  - At 256 bits the requirement is ≈2,300–2,800 bits. These figures are my computation. [inf]
  - The committed Petsche-based numbers, extrapolated from toy fits, are already 265–5,015 bits for r = 2–6.
- **Smallest correction.** Change the bullet to: "The unconditional bound has no explicit constant. With all constants set to 1 it bites only above ~3,400 bits at k = 9. The 160–700-bit figures assume K ≤ 10 (Lang-type) and are conditional." Also write `(2C+1)^k`, not `|C|`.

### M3. The leads are not closed as the verdict wording suggests; the closure answers a reformulated question

- **Where.** Verdict ("Closed in the exponent for every family the five leads specify"), the §3.2 function-field result, and §3.3.
- **The problems:**
  - **LEAD-EXO-global-lift-height.** The live branch (ii) in its PROTOCOL §1 is `T_section(d, χ, p)`. That is exactly R2, which stays open, so the lead is not closed. [src]
  - **The function-field leads** (ICP-xedni-function-field-sections, TRF-function-field-xedni-lift). Their "lift exponent" is `P_lift`, the probability that a random family member hits a prescribed fibre datum. That is a per-point quantity, α = 3 (PROTOCOL §2, atlas line 25). (★) bounds a different quantity: the per-trial probability of a relation among uniform targets.
    - §4 itself concedes that the point-level α is not decisive.
    - "α = 3 … consistent with this and stronger" therefore compares two different exponents. [src][inf]
  - **LEAD-TRF-global-lift-small-height.** The number-field part is covered only for bounded degree (H2).
- **Smallest correction.** Reword the verdict: "the per-trial relation-probability exponent is closed for fixed k under fresh masks; LEAD-EXO branch (ii) (R2) and residue R3 (M1) are open; the function-field reopen clause is reinterpreted from per-point to per-relation probability."

### M4. "Closed" for p-dependent curves in R2 is unsupported

- **Where.** §5 R2: "Curves over Q, fixed or p-dependent: closed for the lattice-scan decider … `M(X) = O((log X)^{r/2})`. Hence `β ≫ 1/2`."
- **The problem.** The implied constant in `M(X) ~ c_E (log X)^{r/2}` depends on the curve (regulator, λ₁) and on r, so it is not uniform for p-dependent curves.
  - With rank `r ≈ log p / log log p` and `λ₁ ≥ (log p)^{−3}`, M can reach `p^{Θ(1)}` at `X = p^{O(1)}`. [inf]
  - This is the Silverman–Suzuki/Miller regime, which committed Prop 4 and the checker record as **open**. The note's claim contradicts them without argument.
- **Smallest correction.** Restrict "closed" to fixed `Ẽ` (fixed r) as p → ∞. Alternatively supply a counting bound uniform in the curve, of Bombieri–Pila or Heath-Brown type, and redo β with the lattice-scan per-relation cost `X²/M`.

### L1. The note relies on k ≤ 9 while saying "Obligation 8 (full row rank) is not used"

- **Where.** §3.2.
- **The problem.** "k ≤ 9 since a plane cubic has 10 coefficients" *is* the full-row-rank step. The Lean `fullRowRank_points_on_nonzero_cubic_le_nine` is conditional on full row rank. Otherwise k ≤ 9 is just the xedni specification (JKSST §3.2: `2 ≤ r ≤ 9`).
  - (★) holds for every fixed k, so the exponent is unaffected. The §3.4 numbers and the phrase "every plane-cubic variant" do depend on k = 9.
- **Smallest correction.** "k ≤ 9 by the xedni specification; (★) needs only that k is fixed." [src]

### L2. The R2 comparison for rational elliptic surfaces is a constant-level argument in mismatched units

- **Where.** §5 R2 (`18m²` Macaulay columns against `0.9n²`).
- **The problem.** Both sides grow like n², so this is a constant factor of about 20, not an exponent gap.
  - The left side counts field-operation columns. The right side is rho's group-operation budget, and each group operation is about 6–12 field multiplications.
  - F4 does not necessarily touch every Macaulay column.
  - It is unpriced under §5a (0, 0.1 and 1 GAE).
- **Smallest correction.** Change "closed against Macaulay-type solvers" to "no exponent gain; constant-level comparison not priced". [inf]

### L3. Lean usage details and verification wording

- The bound needs `hC : C < q`. This is trivial for polylogarithmic C, and when it fails the bound is vacuous anyway, but (★) should state it. [src]
- Sign convention: Lean uses `dot(n,s) = x·dot(n,t)`, while the note writes `R_j = [s_j]P + [t_j]Q`. They differ by `x ↦ −x`, which is harmless. [src]
- `P : Sample → ι → G` takes one fixed group G. A per-trial curve `Ẽ_z` needs packaging, for example a product group. This matters only for the proposed formalisation. [src]
- The union bound counts `n` and `−n` separately, a factor-2 slack. [src]
- "kernel-checked" is a verification claim. No pcc claim covers these declarations: the note itself cites `pcc.py standing`, and only two unrelated APS claims mention "lifting". Write "`[pcc:unverified]`; proof-gate PASS recorded (gate log above, not hash-bound)". [src]

### L4. Wording and notation

- §4: "cost per relation is `(r+1)·T_dec·N/M`" is the *total* for r+1 relations. [src]
- §3.2: "A forced dependency needs `k ≤ r + 1` sections" should read "`k = r + 1` forces dependency (`k ≤ 12χ − 3`)". [src]
- §4: "coefficient heights ≥ `p^{Ω(1/k)}`" is right if it means relation coefficients (`C ≥ q^{1/(2k)}`). For the point-height ratio the requirement is `K ≥ q^{Ω(1/k²)}`. Say which is meant. [inf]
- The rho baseline is written in p; it should be q (cofactor). [src]

### L5. The heuristic "about 14χ − 3 conditions beyond the parameters" (§5 R1) cannot be reproduced from the note

- My count (minimal sections with `deg x ≤ 2χ`, `deg y ≤ 3χ`, and k = r + 1) gives a quadratic in χ, not `14χ − 3`.
- Smallest correction: show the count, or drop it. [inf]

### L6. The note's description of H-XEDN-4a5b50 is inferred

- §3.3 calls it a search that certifies only `|n_j| ≤ c_max` among k ≤ 9 points. The note itself says the id has no definition file, and the lead text says "dependent pres[cribed]…", i.e. prescribed rather than uniform pairs.
- Label the description as inferred. [src]

### L7. Function-field torsion is not stated

- C(p) at bounded χ needs a torsion bound T. It is bounded for χ ≥ 1 through the component groups, but the note never states it. [inf]

## Answers to the six checks

1. **Lean.** Both declarations are proved, with no holes or extra axioms, and a gate PASS is recorded. The composition stays within their hypotheses for fixed k, with (s, t) uniform (s fresh is enough), a uniform C(p) < q, and success implying dependence. A success must certify only that the lifted points are dependent; the bounded relation follows from the height core, and usefulness is not needed for the upper bound.
2. **Masser.** Corollary 1 bounds the canonical height below in terms of `D = [K:k]` and `w = h(1:g₂:g₃)`, with a constant depending on `[k:Q]`. The paper has no point-count parameter beyond the Theorem's count. For `E/Q` with coefficient height O(log p) it legitimately gives `λ₁ ≥ Ω((log p)^{−3})`, with an inexplicit C₁. It is weaker in shape than Lang's conjecture (it decreases in w), but enough for the exponent. Over number fields it is valid only for bounded degree or for `E/Q` (H2).
3. **The step to α.** The derivation is correct. "O(log p)" heights can be relaxed to poly(log p) for any polynomial-time construction. "Fresh points per trial" excludes coordinate-selected pooled tuples and hybrids with extra points or endomorphisms (M1). It does not need non-adaptivity in t.
4. **Function fields.** Correct, apart from L4, L7 and the exponent conflation in M3.
5. **Scope.** The closure is narrower than the five leads' claims. LEAD-EXO (ii) is open, and the function-field "lift exponent" is a different quantity (M3). R1 and R2 are not the only residues (M1). R2's "closed for p-dependent curves" is unsupported (M4).
6. **Wording under the sabotage rule.** The blanket `[pcc:unverified]` is present, but "kernel-checked" (L3), "explicit constant … 160–700 bits" (M2) and the closure verdict overstate. The note also omits the prior committed proof (H1).
