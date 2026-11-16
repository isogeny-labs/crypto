# Hypothesis G behind Theorem C: proved for binary towers in a repaired form, refuted as literally stated at toy scale (follow-up hyp-G, 2026-11-15)

**Standing.** Everything here is `[pcc:unverified]`.
- No pcc claim was filed and no `bench.record/v1` was written. Nothing ran on EKS, and nothing was committed.
- research-kb was not queried, and the lead atlas was not read.
- All runs were local toy computations on a shared 4-vCPU VM.
- The repository was fetched at origin/main `baf0411d41`. The theorem files were byte-identical to the working copy.

**What this follows up.**
- `research/prime_field_ic_closure_20261112/finish/theorem/PROOF.md`: §3, Theorem C and hypothesis G.
- `research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md`: §3.3, §3.5 item 2, §6 item 3.

**Scope.** Only the binary-tower presentation (T): W = g·μ_{2^k}, with chain x² = t₁, …, t_{k−1}² = c. Boolean bits (β) and univariate grids are not re-examined.

---

## 0. Verdict

| Item | Before | After this follow-up |
|---|---|---|
| G(i), detection-degree part, towers | open ("(T): no proven floor on δ_det") | **Proved** for every target, not only typical ones. (a) Line floor: δ_det ≥ k − m + 1. (b) **The conjectured Reed–Muller analogue δ_det ≥ mk − log₂\|V_R\| is a theorem** (Prop RM-T, a Plotkin-type induction) |
| G(i), weak-degree part, towers | open | **Proved floor:** δ_weak(R) ≥ k − m + 1 for every non-degenerate target (Prop T) |
| G(ii) as written, with \|V_R\| ≤ H/2 inside "good" and the constant 4 | "never tested; cannot be tested by sampling" | **Tested exactly** by enumerating every target on 65 tower instances. **False at some θ on 20 of 65 instances,** with G(i) true there (§3.2). It is false at the C2 threshold t = ⌈τ_m·mk⌉ on 11 of the 33 instances with m ≥ 3. In none of those 11 does C1's size condition hold at that t, so the preset's counterexample criterion is never met |
| Theorem C argument in PROOF.md | "proved as an implication" | **Has a gap.** The presentation degree Δ_R depends on the target, for at most 2^{m−1} abscissas; the runs found 0–2 per instance. Such targets show different ranks at degree Δ_R, so "every θ-good target shows the same observations" is false as written. Theorem B has the same gap, which the checker already noted. The repair costs only a constant (§2.4) |
| **Theorem C′** (repaired Theorem C for towers, no hypothesis) | — | **Proved inside RAIC (XL^J = literal XL on (T)):** E[cost] ≥ h·min((1 − 2^m/n)·H(k−m)·max(1, n/N_dec), n/2^m) (§2.5) |
| What Theorem C′ implies about rho | — | **Asymptotically** (k → ∞): exponent ≥ 1 for m = 3, 4; ≥ 1/m + h₂(1/m) for m ≥ 5. That is ≥ 1/2 exactly for **3 ≤ m ≤ 11**, and 0.497 at m = 12. **At concrete sizes:** ≥ √n only for m ≤ 7 at n = 2^128 and m ≤ 8 at n = 2^256 (κ = c = 1) |
| G for towers at m ≥ 12 asymptotically, or m ≥ 9 at 256 bits | open | **Still open.** The proved weak floor k − m + 1 is too low, so a stronger weak-degree floor is needed |
| F4 / polynomial calculus | outside the model | **Unchanged.** msolve refutes tower systems at degree ≈ Δ (PROOF §5.5), far below θ. Theorem C′ does not touch A4-PKM |

**Answer to "prove or refute G".**
1. **Refuted as literally stated.** G(θ) fails at specific θ on toy towers. Clause (ii) fails through the |V_R| ≤ H/2 condition at small θ, and through the degree conditions near the Macaulay maximum.
2. **Proved where it matters, after a repair.** The repair drops the |V_R| clause, which is a proof artefact, and handles target-dependent Δ_R. With it, the part of G that Theorem C actually uses holds for every target at θ − Δ = k − m + 1, for all binary towers.
3. **The resulting bound.** "Index calculus cannot beat rho's exponent within the XL model" holds unconditionally for 3 ≤ m ≤ 11 asymptotically, and for m ≤ 8 at 256 bits.
4. **Still open.** Larger m, and anything about F4.

---

## 1. What is proved and what is assumed

| Statement | Status | Where |
|---|---|---|
| **Lemma L (tower line lemma).** Nonzero q on g·μ_{2^k} with Q·q = P (deg P < deg Q = r < 2^k) has wt-deg(q) ≥ k − ⌈log₂ r⌉. Equality is attained | **Proved** on paper. The recurrence core and the top-run bit lemma are **Lean-checked**: `recurrence_kills` and `top_run_bits`, standard axioms, no `sorry`. The reduction mod x^N − c is not formalised | §2.1; `lean/LineLemma.lean` |
| **Lemma L′.** Restricting a function to an axis line does not raise its weight-degree | Proved (paper, elementary) | §2.1 |
| **Prop T(a).** δ_det(R) ≥ k − ⌈log₂ s_R⌉ ≥ k − m + 1 for every R with V_R ≠ ∅ | Proved (paper) | §2.2 |
| **Prop T(b).** δ_weak(R) ≥ k − m + 1 for every non-degenerate R. Also δ(1/S_R) ≥ k − m + 1 when V_R = ∅ | Proved (paper). Assumes k ≥ m and that W holds no 2-torsion abscissa; the latter is automatic for W ⊂ F_p and n odd | §2.2 |
| **Prop RM-T.** Every nonzero f ∈ L_t on a product of binary towers of total height K has ≥ 2^{K−t} nonzeros. Hence δ_det(R) ≥ mk − log₂\|V_R\| | Proved (paper; Plotkin (u\|u+v) induction, char ≠ 2) | §2.3 |
| Uncertainty floor H(δ_det)·\|V_R\| ≥ \|W\|^m | Proved (paper); weaker than RM-T and kept as a cross-check | §2.3 |
| **Lemma Δ.** At most 2^{m−1} target abscissas have Δ_R below the generic value | Proved (paper, one line) | §2.4 |
| **Theorem C′** and Corollary C′1 | Proved inside RAIC | §2.5 |
| Asymptotic exponent e_m, finite-size floors, k_min | Computed | `out/exponent_floor.jsonl`, `out/thresholds_hypG.jsonl` |
| PROOF.md Fact 0, Lemmas 2, 4, 6, 9, `master_sq` | **Used, not re-proved.** The R5 checker confirmed them; `master_sq` is in Lean | — |
| Semaev's characterisation of S_{m+1} | **Assumed** (literature) | — |
| Model axioms A1 (cost = rank), A2 (uniform independent targets; stopping only on ranks and flags), A3 (h ≥ c\|B\|) | **Assumed** (they define the model) | — |
| Original G(ii) with constant 4 and the \|V_R\| clause | **Not proved; false at some θ at toy scale** | §2.6, §3.2 |
| Anything about F4 / polynomial calculus | Not addressed | — |

---

## 2. Proofs

Notation follows PROOF.md.
- N = 2^k and W = {x : x^N = c}, with c ≠ 0 and char ≠ 2.
- A function on W has a unique reduced form Σ_{a<N} C_a x^a. Its **weight-degree** is wt-deg := max{popcount(a) : C_a ≠ 0}. This is the degree of its normal form in (T), because (T) is an H-basis (PROOF §1.2).
- On W^m the weight of a monomial is the sum of the blockwise weights. L_t and H(t) are as in PROOF §1.3.

### 2.1 Line lemma

**Lemma L.** Let q: W → K̄ be nonzero, with Q·q = P on W, where deg Q = r, 1 ≤ r ≤ N−1 and deg P < r. Then wt-deg(q) ≥ k − ⌈log₂ r⌉.

*Proof.*
1. **Recurrence.** Q(x)·Σ C_a x^a ≡ P(x) mod (x^N − c). Reduction only moves exponents ≥ N into [0, r−1]. So for r ≤ b ≤ N−1, Σ_{i=0}^{r} Q_i C_{b−i} = P_b = 0.
2. **Top run.** Suppose wt-deg(q) ≤ t ≤ k − 1. Every a ∈ [N − 2^{k−t−1}, N) has its top t+1 bits set (Lean `top_run_bits`). So C vanishes on a top run of length 2^{k−t−1}.
3. **Propagation.** If 2^{k−t−1} ≥ r, the order-r recurrence with Q_r ≠ 0 propagates the zeros down to every index (Lean `recurrence_kills`). Then q = 0, a contradiction.
4. **Conclusion.** Hence 2^{k−t−1} ≤ r − 1. That is t ≥ k − 1 − log₂(r − 1) for r ≥ 2; for r = 1 no t ≤ k − 1 is possible. Both cases give wt-deg ≥ k − ⌈log₂ r⌉.
5. **Equality.** Take Q = x^{2^e} − α, or the indicator of a coset of μ_{2^e}. With y = x^{2^e}, 1/(y − α) on a coset of μ_{2^{k−e}} has every coefficient nonzero. $\square$

**Lemma L′.** If F ∈ L_t, then on every axis line its restriction has weight-degree ≤ t.

*Proof.* Substituting grid points for the other blocks does not raise the degree. Taking the normal form modulo the block's tower Gröbner basis does not raise it either. $\square$

### 2.2 All-target floors (Prop T)

Assume k ≥ m, and that W contains no abscissa of a 2-torsion point of E.

**(a) Detection.**
1. Take f ≠ 0 in L_t ∩ F_{V_R}, and pick v with f(v) ≠ 0. Let ℓ be the x_m-line through v.
2. f|_ℓ ≠ 0 vanishes off ℓ ∩ V_R. That set is the grid roots of S_R(v₁..v_{m−1}, X), which is nonzero by Fact 0, so s := |ℓ ∩ V_R| ≤ 2^{m−1} < N.
3. Apply Lemma L with Q = Π_{u∈ℓ∩V_R}(X − u) and P = 0, then Lemma L′. This gives t ≥ k − ⌈log₂ s⌉ ≥ k − m + 1. $\square$

**(b) Weak degree.**
1. Weak success at degree D gives F ∈ L_{D−Δ} with F = u(x₁)/S_R off V_R. Here u ≠ 0 on W and u = 0 on π₁(V_R) (PROOF Lemma 4).
2. Pick a with u(a) ≠ 0. Then no point of V_R has x₁ = a.
3. On ℓ = {(a, w₂..w_{m−1}, X)}: F|_ℓ·Q = u(a) ≠ 0, where Q(X) = S_R(a, w₂.., X) has no root in W and r = deg Q ≤ 2^{m−1}.
4. **Q is non-constant.** Take points A, W_i with these abscissas. If every sum R + εA + Σε_iW_i were O, two of them differing only in ε would give 2A = O, which is excluded. A non-O sum has an abscissa that is a root of Q, by Semaev.
5. Lemmas L and L′ give D − Δ ≥ k − m + 1. Degenerate targets have D_weak = ∞ by PROOF's definition. With V_R = ∅ and u = 1 the same argument gives the strong bound. $\square$

### 2.3 Reed–Muller analogue for towers (Prop RM-T)

**Claim.** Let G be a product of tower cosets of heights k₁..k_m, with K = Σk_i. Every nonzero f ∈ L_t on G has ≥ 2^{K−t} nonzeros.

*Proof.* Induction on K. For K = 0 the claim is trivial.
1. **Split.** Split on the top chain variable τ of a block with k_i ≥ 1. Since τ² = c_i, τ = ±s with s ≠ 0, and G = G₊ ⊔ G₋. Each half is a product of tower cosets of total height K − 1; the block's chain now ends t_{k_i−2}² = ±s.
2. **Halves.** Write f = f₀ + τf₁ (multilinear), with deg f₀ ≤ t and deg f₁ ≤ t − 1. Then f|_{G±} = g± := f₀ ± s·f₁, each of degree ≤ t.
3. Multilinear monomials form a basis of functions on each half.
4. **Both g± nonzero.** Each half contributes ≥ 2^{K−1−t} nonzeros.
5. **g₊ = 0.** Then g₋ = −2s·f₁ ≠ 0 with degree ≤ t − 1, giving ≥ 2^{K−t} nonzeros. The case g₋ = 0 is symmetric. $\square$

**Corollary.** δ_det(R) ≥ mk − log₂|V_R| for every target. This is exactly the floor PROOF §3 conjectured ("held on all 370 tower targets, minimum margin 0").

**Tightness.** Equality holds on subgroup cosets, on products of cosets and on the diagonal: the diagonal has δ_det = k with |S| = 2^k (§3.3).

### 2.4 Target dependence of Δ (Lemma Δ: a gap in PROOF.md Theorems B and C)

Take any fixed representative family of S_R; the canonical one is the grid weight-degree Δ_R. Its reduced coefficients are polynomials in x_R of degree ≤ 2^{m−1}. So Δ_R is below its generic maximum Δ* for at most 2^{m−1} abscissas, which is at most 2^m targets.

**Why this is a gap.** Such a target has rank 1 at degree Δ_R, where a generic target has rank 0. It is therefore distinguishable at once.

**Measured** (`out/delta_R*.jsonl`):
- Exceptional abscissas appear in instances at m = 3 and m = 4, with 0–2 per instance and never more than 2^{m−1}.
- Some exceptional targets are decomposable, for example x_R = 32 in `G_3_2_769_c1_rand` and x_R = 726 in `G_3_5_769_c1_rand`.

**Repair.** Count these targets as non-good. They add at most 2^m to ν, which changes only constants.

### 2.5 Theorem C′ (towers; no hypothesis)

**Setting.** RAIC, presentation (T), k ≥ m ≥ 3, the assumption of §2.2, solver XL^J. Literal XL is the same here, because (T) is an H-basis. Let X be the Δ-exceptional targets, so |X| ≤ 2^m.

**Theorem C′.** For every schedule,

$$\mathbb E[\text{total}]\ \ge\ h\cdot\min\Big((1-2^m/n)\,H(k-m)\,\max\big(1,\tfrac{n}{N_{\rm dec}}\big),\ \ \tfrac{n}{2^m}\Big).$$

*Proof.*
1. Set θ := Δ* + k − m + 1. Take R ∉ X and D ≤ θ − 1.
2. **Same ranks.** The rank of Mac^J_D/J_{≤D} is H(D − Δ*). Its kernel L_{D−Δ*} ∩ F_{V_R} is zero because D − Δ* ≤ k − m < δ_det(R) (Prop T(a)).
3. **Same flags.** No weak-success or refutation flag is raised, because D < Δ* + δ_weak(R) (Prop T(b)). Degenerate targets never succeed, and refutation needs D ≥ D_weak.
4. So the decision "continue to degree θ" is the same for all R ∉ X, and the history fixes it (A2).
5. **If the attempt stops,** it can succeed only on X: π_lo ≤ 2^m/n.
6. **If it continues,** a target R ∉ X costs at least rank(θ) ≥ rank(θ − 1) = H(k − m). The rank is monotone because L_t ⊆ L_{t+1}, and this step replaces PROOF's "H − |V_R| ≥ H/2". Also π_hi ≤ min(1, N_dec/n).
7. Lemma 9 gives the bound. $\square$

**Corollary C′1.** Suppose:
- h ≥ c|B|;
- N_dec ≤ |B|^m;
- H(k − m) ≥ 2κ|B|^{m/2−1};
- n ≥ max(2^{m+1}, 4^m κ²).

Then E[total] ≥ cκ√n (`master_sq`).

**What it gives against rho** (`code/exponent_floor.py`, `code/thresholds_hypG.py`):
- **Asymptotically, at fixed m, k → ∞, |B| = n^β:** log_n E ≥ β(1 + m·h₂(1/m)) + max(0, 1 − mβ). The infimum is 1 when 1 + m·h₂(1/m) ≥ m (m = 3, 4); otherwise it is 1/m + h₂(1/m).
- That is ≥ 1/2 iff h₂(1/m) ≥ 1/2 − 1/m, which is equivalent to 1/m ≥ τ_m. This holds for 3 ≤ m ≤ 11.
- **At concrete sizes** (minimum over k ≥ m and |B| ≤ 2^{k+1}, κ = c = 1), the floor is much weaker, because (k − m)/(mk) is well below 1/m at small k.

| m | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
|---|---|---|---|---|---|---|---|---|---|---|
| asymptotic exponent floor e_m | 1 | 1 | 0.922 | 0.817 | 0.735 | 0.669 | 0.614 | 0.569 | 0.530 | 0.497 |
| log₂(floor) / log₂ n at n = 2^128 | 0.930 | 0.875 | 0.778 | 0.636 | 0.514 | **0.398** | 0.296 | 0.192 | 0.083 | 0.076 |
| log₂(floor) / log₂ n at n = 2^256 | 0.965 | 0.938 | 0.849 | 0.729 | 0.630 | 0.540 | **0.470** | 0.404 | 0.338 | 0.281 |
| least k with H(k−m) ≥ 2\|B\|^{m/2−1} | 4 | 6 | 9 | 12 | 17 | 24 | 36 | 61 | 142 | none ≤ 1199 |

Reading the table:
- Against rho's 0.5, the distance remaining is positive (the floor exceeds rho) for m ≤ 7 at 128 bits and m ≤ 8 at 256 bits.
- For larger m the floor sits below 0.5. The XL model then **does not exclude** beating rho; it only fails to prove a lower bound.

**What C′ does not say.** It is a statement about XL only. F4 escapes it (PROOF §5.5). It leaves A4-PKM where it was.

### 2.6 Where the literal G stands

At θ = Δ* + k − m + 1, the detection and weak-degree parts of "good" hold for every target outside X.

| Part | Status |
|---|---|
| G(i) | Holds when 2^{m+1}\|W\|^m/(nH(k−m+1)) ≤ 1/2 (Lemma 2c) |
| G(ii) | Needs #{R decomposable : \|V_R\| > H/2} ≤ (4/H)·min(N_dec, n) − \|X\|. Counting gives only 2^{m+1}\|W\|^m/H, which is short by a factor of about 2^{m−1}\|W\|^m/N_dec. Not proved, and false at toy scale (§3.2) |

**Why it does not matter.** The |V_R| clause in "good" served only the cost bound H − |V_R|, and rank monotonicity (§2.5, step 6) replaces it. **G(ii) should not be cited. Theorem C′ replaces Theorem C for towers.**

---

## 3. Numerical tests (all numbers produced in this follow-up)

### 3.1 Line lemma (`code/line_lemma.py` → `out/line_lemma.jsonl`)

- **Inputs.** p = 786433; k = 2..10; r ∈ {1, 2, 3, 4, 5, 7, 8, 9, 16, 17, 31, 32, 64, 128} with r < 2^k. That makes 95 (k, r) cells, each with 40 random P/Q and 40 random r-point supports: 7,600 functions.
- **Violations: 0.** The minimum of wt-deg minus the bound is 0.
- **Equality** holds in all 51 power-of-two cells for both tight families.

### 3.2 Exhaustive enumeration over all targets (`code/exhaustive_G.py`; grid `code/jobs.txt` + `jobs_rest.txt`; outputs `out/exh/`; `out/exh_table.md`, `out/exh_stats.json`)

**Design.**
- Every abscissa of E(F_p) \ {O} was processed. Each abscissa stands for 2 targets, so counts are exact, with no sampling.
- The preset `out/PRESET_hypG.txt` was written after launch and before any grid output was opened. Only the three smoke runs `out/test_*.json` had been read.
- Decomposability flags were cross-checked by brute-force group-law sums on 6 instances, at m = 2, 3, 4: 6/6 agree (`out/check_dec.jsonl`). This also validates the S₅ pickle used at m = 4.

**Coverage: 65 instances, 208,866 abscissas (417,732 targets).**

| | p = 769 | p = 12289 |
|---|---|---|
| m = 2 | k = 2..8 | k = 2..7 |
| m = 3 | k = 2..5 | k = 2..5 |
| m = 4 | k = 2, 3 | k = 2, 3 |

- **Families:** 47 random cosets, 13 subgroup cosets (g = 1), 5 j = 0 curves.
- **j = 0 notes:** no prime-order j = 0 curve exists over F_769, so all 7 j = 0 jobs there were impossible. The j = 0 instance at m = 4, k = 2, p = 12289 has |B| = 0, so it is vacuous.
- **Regime:** N_dec/n ranges from 0.000 to 0.999. 22 instances with m ≥ 3 have N_dec/n < 0.1, the index-calculus-relevant regime.
- **Not run (runtime budget):** 8 jobs at p = 12289 — m = 2, k = 8; m = 2, k = 7 second seed, g = 1 and j = 0; m = 3, k = 5 second seed; m = 4, k = 3 second seed and j = 0.

| check (every target of every instance) | violating targets |
|---|---|
| Prop T(a), line floor δ_det ≥ k − ⌈log₂ s_line⌉ | **0** |
| Prop T(a)/(b), δ_det ≥ k − m + 1 and δ_weak ≥ k − m + 1 | **0** |
| Prop RM-T, δ_det ≥ mk − log₂\|V_R\| | **0** |
| Uncertainty floor, H(δ_det)·\|V_R\| ≥ \|W\|^m | **0** |

**Floors against observed minima.** The floors are far below what is observed. For example:
- m = 3, k = 5: floor 3; minimum δ_weak 13; minimum δ_det 10.
- m = 4, k = 3, p = 12289: floor 0; minimum δ_weak 10; minimum δ_det 8.

The all-targets-good′ threshold (largest t at which every target is good′) is 2–12 by instance, always ≥ k − m + 2. So the proved k − m is never the binding value at toy scale.

**Original G(θ), computed exactly at every t = θ − Δ.**
- G holds at its largest t, and the largest t with G true lies at 0.667–0.875 of ρ in every instance.
- G fails at some t on many instances. **G(ii) fails while G(i) holds** on 20 of 65 instances:

| instance (m, k, p, family, seed) | t where G(ii) fails, G(i) holds | ρ | N_dec/n |
|---|---|---|---|
| 2, 6, 769, rand, c1 / c2 | 1 / 1 | 12 | 0.97 / 0.98 |
| 3, 2, 769, rand, c1 / c2; 3, 2, 12289, rand, c1 | 5 | 6 | 0.127 / 0.057 / 0.004 |
| 3, 3, 769, rand, c1 / c2 | 1 | 9 | 0.21 / 0.23 |
| 3, 4, 769, one / rand c1 | 1 | 12 | 0.73 / 0.66 |
| 3, 5, 12289, rand, c1 | 1 | 15 | 0.68 |
| 3, 5, 769, rand, c1 / c2 | 2, 13 / 13 | 15 | 0.999 |
| 4, 2, 12289, one / rand c1 / rand c2 | 7 | 8 | 0.002–0.007 |
| 4, 2, 769, one / rand c1 | 7 / 1, 7 | 8 | 0.033 / 0.097 |
| 4, 3, 12289, one / rand c1 | 1, 10 | 12 | 0.018 |
| 4, 3, 769, rand, c2 | 2, 9, 10 | 12 | 0.77 |

**Where the failures come from.** Every small-t failure is caused by the |V_R| > H(t)/2 clause (ν equals ν_V there). Every failure near ρ is caused by the degree clauses, where the bound 4·min(N_dec, n)/H drops below 1. One example, m = 4, k = 3, p = 12289 (rand c1; N_dec = 224, n = 12421):
- **t = 1** (H(1) = 13): ν = 112, against the bound 68.9. All 112 targets have |V_R| ≥ 12, while H/2 = 6.5; non-degenerate decomposable targets have |V_R| up to 24 = m!.
- **t = 10:** ν = 16, against the bound 0.22.

**At the C2 threshold t_C2 = ⌈τ_m·mk⌉.**
- G fails on 11 of the 33 instances with m ≥ 3.
- C1's size condition H(t_C2) ≥ 4|B|^{m/2−1} fails on all 11. The preset's counterexample criterion was "G fails at t ≤ t_C2 with the size condition holding", so it is **never met**. No instance contradicts Theorem C's *use* of G.

### 3.3 Δ_R target dependence (`code/delta_R.py` → `out/delta_R.jsonl`, `out/delta_R2.jsonl`)

All 65 instances were checked: 65/65 are within the bound 2^{m−1}.

| m | instances | 0 exceptional abscissas | 1 | 2 |
|---|---|---|---|---|
| 2 | 32 | 32 | 0 | 0 |
| 3 | 21 | 11 | 10 | 0 |
| 4 | 12 | 8 | 3 | 1 |

In 5 instances an exceptional target is decomposable:

| instance | x_R |
|---|---|
| m = 3, k = 2, p = 769, c1 | 32 |
| m = 3, k = 5, p = 769, c1 | 726 |
| m = 3, k = 5, p = 769, c2 | 310 |
| m = 3, k = 5, p = 12289, c1 | 6038 |
| m = 4, k = 2, p = 769, c1 | 766 |

So in these instances PROOF's single-Δ argument does fail on a target that can yield a relation.

### 3.4 Reed–Muller analogue, before the proof (`code/rm_analogue.py`, `code/rm_structured.py`)

- **Exhaustive over all supports up to half the grid:** (m, k) = (1, 2), (1, 3), (1, 4), (2, 2) at p = 769, with 39,202 supports at (1, 4) and at (2, 2). **0 violations.**
- **Sampled:** (1, 5) with 2,441,448 supports; (2, 3) and (3, 2) with 92,080 each. 0 violations.
- **Structured at m = 2, k = 2..5, p = 12289:** graphs x ↦ ζ^j x and x ↦ c′/x, diagonals and anti-diagonals on subgroup cosets, products of cosets, 23–87 families per k. 0 violations. Equality holds on the diagonal: δ_det = k at |S| = 2^k.

---

## 4. Commands

`$H` = `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/hyp-G`.

**Dependencies.**
- The code imports `icmodel.py`, `fastdelta.py` and `check_G_hypothesis.py` unchanged from `finish/theorem/code/`. In the repo, put that directory on `PYTHONPATH`.
- The m = 4 runs need `S5_generic.pkl` in `../out/` relative to `check_G_hypothesis.py`. Its sha256 is in `out/RAW_MANIFEST.sha256`; rebuild it with `build_s5.py`.

```
cd $H/code
python3 line_lemma.py > ../out/line_lemma.jsonl
xargs -P 3 -L 1 ./runjob.sh < jobs.txt ; xargs -P 3 -L 1 ./runjob.sh < jobs_rest.txt   # exhaustive_G.py m k p seed {rand|one} [j0]
python3 stats.py > ../out/exh_stats.json ; python3 aggregate.py > ../out/exh_table.md
python3 delta_R.py ../out/exh/G_*.json > ../out/delta_R.jsonl
python3 check_dec.py ../out/exh/G_4_3_769_c1_rand.json ...  > ../out/check_dec.jsonl
python3 thresholds_hypG.py > ../out/thresholds_hypG.jsonl ; python3 exponent_floor.py > ../out/exponent_floor.jsonl
python3 rm_analogue.py 769 > ../out/rm_analogue.jsonl ; python3 rm_analogue.py 769 5 >> ../out/rm_analogue.jsonl
python3 rm_structured.py > ../out/rm_structured.jsonl
source <R5>/finish/theorem/lean/env.sh; cd <R5>/finish/theorem/lean/mathlib4 && lake env lean $H/lean/LineLemma.lean
```

**Lean check.** Lean 4.34.1 against the R5 Mathlib checkout: exit 0, no `sorry`. `recurrence_kills` and `top_run_bits` use only [propext, Classical.choice, Quot.sound].

**Raw data.** The raw per-target outputs (`out/exh/*.json`, 15 MB) are listed by SHA-256 in `out/RAW_MANIFEST.sha256`. They would go to S3 before any commit. The repo mirror carries per-instance summaries only (`out/exh_summaries/`).

---

## 5. Open or blocked

1. **A weak-degree floor above τ_m·mk.** This is needed for m ≥ 12 asymptotically, and for m ≥ 9 at 256 bits or m ≥ 8 at 128 bits. The proved floor k − m + 1 ≈ ρ/m is the binding limit; the data suggest δ_weak/ρ ≥ 0.667. Routes not yet tried:
   - a planar version of Lemma L, since S_R restricted to a 2-plane has bidegree (2^{m−1}, 2^{m−1});
   - a Plotkin-type split for δ_weak.
2. **F4 / polynomial calculus.** Untouched. This remains the binding open item for PKM. Theorem C′ is an XL-model result only.
3. **Boolean bits (β).** The weak-degree floor is open; a line lemma on {0..2^s−1} with multilinear degree is the natural analogue and was not attempted. Univariate grids are not re-examined.
4. **Formalisation.** Only the recurrence core and the top-run lemma are in Lean. Not formalised:
   - the reduction mod x^N − c;
   - Lemma L′;
   - Prop T;
   - Prop RM-T;
   - Lemma Δ;
   - Theorem C′.
5. **Prior art not checked** (research-kb not queried). Lemma L is a BCH-type bound for cyclic codes, and Prop RM-T is the Plotkin argument for Reed–Muller codes. Both are probably known in coding theory, so no novelty is claimed.
6. **Process.**
   - No pcc claim, no bench record, nothing committed.
   - The preset was written after launch, but before any grid data was read.
   - 8 of the 80 grid jobs were not run, and 7 more were impossible (no prime-order j = 0 curve over F_769).
   - The exhaustive checks only test the floors and G; they do not test Theorem C′'s cost bound itself, which follows from Lemmas 4, 6 and 9 (checked by R5).
