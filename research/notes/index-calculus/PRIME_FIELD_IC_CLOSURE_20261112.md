# Prime-field index calculus as an ECDLP route: closure note (2026-11-12)

**Standing.** Every verdict, number and argument in this note is `[pcc:unverified]`.
- **How the work ran:** local toy runs on a shared VM (4 vCPU, load average 8–12, a 14.3 GB memory limit shared with other agents' jobs).
- **What was not done:** no pcc claim was filed and no `bench.record/v1` was written. Nothing ran on EKS, no resume checkpoint was made, nothing passed the validation gate, and nothing was committed.
- **Prior art was never checked properly.** research-kb could not be reached in any round (`scripts/research-kb status`: "no executable found"), and the lead-atlas checkout was missing. Every novelty or "no prior closure" statement below therefore rests on grepping the repo only.
- **Independent checks.** Each lab in round R5 was checked by a separate adversarial checker, which re-ran parts of it. The checker's judgement is the one recorded here. Where the producing lab claimed more than the checker accepted, this note gives the checker's reading.

**Rounds combined**
- **R1:** desk review of 15 ideas.
- **R2:** chase of 5 ideas.
- **R3:** prior-art pass, stopped by a safety stop with no output.
- **R4:** closure round, six labs. Its ledger is `$SCR/closure-ledger.md`.
- **R5 (this note):** three finishing labs, each with an adversarial check:
  - `theorem`: a restricted-model lower bound;
  - `pkm-msolve`: F4 degree measurements for the PKM chain at m = 4–7;
  - `lifting`: a paper proof plus a toy xedni.

`$SCR` = `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad`.

---

## 1. Summary verdict

**At the scale and rigour this lab has applied, prime-field index calculus is not a viable route to beating Pollard rho** `[pcc:unverified]`. Every construction we measured falls into one of three groups:
- it costs an exponent of at least about 0.67 against rho's 0.5;
- it only relabels the factor base;
- it gives a constant factor that rho matches.

This is strong evidence from toy runs plus arguments, but **it is not a proof that index calculus cannot beat rho**.

R5 adds three things:

1. **A lower bound inside a restricted model (paper proof; small Lean parts).** The model is XL/Macaulay solving of the unsplit Semaev system, with cost charged as the full rank and relations required in proportion to |B|. Inside it:
   - **Theorem A:** at arity m = 2, index calculus cannot beat rho's exponent.
   - **Theorem B:** a factor base without auxiliary variables costs exponent 1. This holds for a univariate base, and for the image of a rational map of bounded degree.
   - **Theorem C:** auxiliary-variable bases cost exponent at least 1/2, but only if hypothesis G holds. G is unproved for towers, and one of its two clauses was never tested.
   - **Limits:** the model leaves out F4. The checker found that F4 also solves at a much lower degree than XL on univariate bases, so the closures under Theorem B depend on the model as much as the PKM case does.
2. **PKM chain at m ≥ 6: KILL at toy scale for m = 4–7** under the preset rule, with msolve's F4 on the raw Kummer chain at N ≤ 21.
   - The checker judged this verdict fragile. The m = 6 and m = 7 KILLs each rest on a slope measured over a single step, and that step is censored (m = 6) or comes from a small-base fallback (m = 7).
   - It does not close PKM for m ≥ 8. If D rises one degree per t-step, then at m = 8 the w = 1 cost model gives exponent 0.43, below rho's 0.5.
3. **Lifting (A8): ruled out asymptotically within a stated model.** The model has fixed r, fresh uniform masks and lifts over Q of polynomial size. The proof assumes no conjecture and uses only published theorems (Petsche 2006 Thm 2, Silverman 1990, Mazur). It is a paper proof and is not machine-checked.
   - It does not settle concrete sizes: r ≥ 7 is not excluded at 256 bits.
   - Silverman–Suzuki-type lifts with growing rank stay open.

**Answer to "ideally a proof":** this round reached a proof only for the restricted XL model (Section 3) and an asymptotic proof for lifting in the fresh-mask model (Section 5). **An unconditional lower bound for ECDLP against all index-calculus algorithms is an open problem.** This round neither solves it nor comes close (Section 6).

---

## 2. Route table

Strength key:
- **T+R:** toy measurement, independently re-run with matching output.
- **T+r:** toy measurement, partly re-run.
- **T:** toy measurement, not re-run.
- **D:** desk or paper argument, not machine-checked.
- **D\*:** paper argument that an adversarial checker went through line by line.
- **L:** literature, known only through the repo's summary.
- **Lp:** literature, checked against the primary source.
- **P:** machine-checked.

All entries are `[pcc:unverified]`.

### A. Exponent-level routes (the "equation budget")

| # | Route | Verdict after R5 | Strength | Distance remaining to matched rho |
|---|---|---|---|---|
| A1 | Weil descent route 1 (base in GF(p), lifted to GF(p^k)) | Ruled out | T+R, D | No gain: the k−1 extra equations constrain only the twist component (predicted = measured on 12/12) |
| A2 | Weil descent route 2 (subspace base outside GF(p)) | Ruled out | T+R, D | Cost ≈ p^1 against p^{1/2} |
| A3 | Galois-orbit / trace base Tr(S) | Ruled out | T+R (k ≤ 3, m = 2, p ≤ 809); D for all S | \|F\| ≈ 0.6p, so exponent 1 against 0.5 |
| A4 | Rational-map image base x = φ(t) | Ruled out in the presentations tested. **Inside the XL model:** Theorem B covers maps of bounded degree only. **PKM chain:** see row A4-PKM | T+r, D; D\* (Theorem B, XL only) | Parametrised case is a relabel. Lattès/Vélu bases keep 0.34–0.66 of the yield. Algebraic μ_{2^k}: exponent 1 under XL |
| A4-PKM | PKM Kummer tower/chain with auxiliary unknowns, m ≥ 6 | **KILL at toy scale for m = 4–7** (msolve F4, N ≤ 21). Fragile at m = 6 and 7. **Open for m ≥ 8 and asymptotically** | T+r (11 msolve runs re-run, all matching; independent closure re-implementation); D\* (Lemmas S, X, B) | m = 6 is the tightest row: implied exponent ≥ 0.645 against 0.5. Under the checker's slope-halving scenario, m = 6 reads 0.443 and m = 7 reads 0.370 (INCONCLUSIVE) |
| A5 | Fixed-degree polynomial-root base | Ruled out. **Inside the XL model it is a theorem** (Theorem B, univariate case) | T+R; D (K1); D\*; P (K1 counting only) | Exponent 1 within XL. F4 solves at a lower degree than XL here (Section 3.4) |
| A6 | Semaev m = 3/4 with an interval or low-bit base (MITM, Boolean Macaulay, APS) | Ruled out for the oracles measured. APS: Theorem B within XL | T+R, L, D; D\* | Fitted exponents 0.69 / 0.80 (MITM) and ≥ 1.10 (Boolean Macaulay) against 0.5 |
| A7 | Equation-budget-A, first attempt | Superseded by A3–A5 | none | n/a |
| A8 | Lifting: xedni/JKSST, Q-curve and number-field lifts, Silverman–Suzuki, p-adic | **Ruled out asymptotically for fixed r** (fresh-mask model, lifts over Q of polynomial size). Q-curve and number-field lifts: only a sketch. Silverman–Suzuki and concrete sizes for r ≥ 7: open. Homomorphic p-adic transfers: ruled out | D\*; Lp (Petsche); L (Silverman 1990, recalled); P (JKSST packing sub-lemmas only, not rebuilt); T+R (toy at 8–16 bits) | Per-trial success ≤ (log p)^{O(r²)}/q, so the expected number of trials is q^{1−o(1)} against √q |
| A9 | BKK / sparse-resultant presentation | Ruled out as an exponent lead | D | Saving is a constant independent of p |
| A10 | Richelot-glued (2,2)-split Jacobian | Ruled out | D plus a repo measurement (quoted) | 9.8–42.9x rho |
| A11 | Isogeny pushforward with a kernel-closed base | Ruled out (a relabel) | D | Reduces to IC on E′ with base \|B\|/l |

### B. Constant-factor and accounting routes inside the explicit-table family

These are unchanged from R4. In summary:
- **Every arm stays 4.5x–33x above matched rho**, and the gap grows with group size wherever growth was measured. There is also an analytic floor of at least 1.6·√h × rho-neg for any member of the explicit-table family.
- **B1 (CM-orbit fold on j = 0/1728 curves)** is the only lever that improves the IC baseline. Its net gain is 1.25–1.62x, it is already in the repo, and it does not change the exponent.
- **B2 (paired translate)** stays unresolved under §5a: the order flips across the 0–1 GAE band.
- **The other B rows** are ruled out or already implemented: B3 hybrid collision, B4 isogeny-class shopping, B5 Legendre prefilter, B6 linear-algebra share, B7–B12 desk rejections, and B13 Montgomery batching (already in the baseline).

Full table: `$SCR/closure-ledger.md` §1B.

---

## 3. The restricted theorem and its proof status

Producer: lab `theorem`. Checker verdict: **sound with gaps**. Everything here is `[pcc:unverified]`.

### 3.1 Model

The theorem covers index calculus at arity m on a prime-order E(F_p), with n = #E. Its axioms are:
- **A1/A2.** The factor base is cut out by a polynomial presentation: a radical ideal, possibly with auxiliary variables.
  - Relations come from the **unsplit** Semaev polynomial S_{m+1} plus the base equations, solved by XL at degree D.
  - The cost of one attempt is at least the rank of the space XL computes.
  - Each target is charged a fresh rank.
  - A relation counts only when it is exposed through a univariate polynomial in block 1.
- **A3.** The algorithm needs at least c·|B| relations (Theorem B needs only one).
- **Targets and stopping.** Targets are uniform and independent. An attempt may stop early, but only on observed ranks or success flags.

### 3.2 Lemmas the checker confirmed line by line

| Item | Content | Checker's judgement |
|---|---|---|
| Lemma 4 (exact-degree lemma) | The exact XL degree at which XL refutes, eliminates or learns any univariate fact equals deg S_R plus an interpolation degree of 1/S_R (or g/S_R) on the factor-base grid. This is exact when the base generators have no degree falls. The cost lower bound is the grid's Hilbert function. It does not use the first-fall-degree heuristic | Correct. Reproduced exactly against the literal Macaulay matrix on 80/80 re-run rows (uni2 42, kummer2 15, tower2 23). Graph presentation: predicted degree ≤ actual on 15/15 |
| Lemma 1 (K1) | \|B\| ≤ 2·deg f | Correct; also Lean-checked |
| Lemma 2 (counting) | At most \|B\|^m decomposable targets; bounds on grid zeros; at most 2^m\|W\|^{m−1} degenerate targets | Correct. The exhaustion check counts one vacuous base among its "10 bases", and the toy margins are loose |
| Lemma 3 (surplus) | Equation surplus is exactly 1 for any number of auxiliary variables | Correct only for complete intersections. **It is not an obstruction:** equation count does not decide cost, as the F4 tower data show |
| Lemma 5(U) (base blindness) | With no auxiliary variables, XL learns nothing below degree d = deg f | Correct for univariate bases |
| Lemma 6 (palindromic Hilbert series, median bound) | — | Correct |
| Lemma 7, Wald ratio, master inequality | — | Correct. Wald measured 82.58 attempts against 82.29 predicted (s.e. 0.37) |

### 3.3 Theorems

**Theorem A (m = 2, any base): E[cost] ≥ c√n.**
- **Status:** proved within the model, but the checker found **two hidden assumptions**.
  1. Each attempt yields at most one relation, or cost is at least the number of relations output. When |B|² > n, one target can carry several decompositions, and without this axiom the c|B| branch fails.
  2. Axiom A3 (h ≥ c|B|) is false for bases whose logs are known, for example B = {iP}. There the bound needs the cost of building the base, which the model does not charge.
- **Reading:** at m = 2, IC cannot beat rho's exponent within the model, once both axioms are stated.

**Theorem B (univariate base, or the image of a rational map; XL at any degree): E[cost] ≥ n·C(m+d−Δ, m)/C(2d+m−1, m), which tends to 2^{−m}·n.**
- **Univariate (U) case:** proved. Exponent 1 inside the XL model, which closes A5, A6-APS and the parametrised A4 cases within the model.
- **Minor gap:** "below degree d, ranks do not depend on the target" assumes Δ_R is the same for every target. At most about 2^m special x(R) break this. The effect is negligible, but the step is not exact as written.
- **Graph (Γ) family:**
  - For literal XL there is a gap: Γ is not an H-basis, so the rank may depend on R.
  - For XL^J the checker supplied a repair, which is **not in PROOF.md**: J_{≤D} = I(C^m)_{≤D} below d_T/(e+1), so the rank does not depend on R.
  - The constant is (2(e+1))^{−m}·n, so **exponent 1 holds only for map degree e = n^{o(1)}**. A composed or iterated map (e = 2^k, a PKM chain written without auxiliary variables) makes the bound vacuous. Power-map and μ_n cases with large e are not closed by Γ.
- **Constants E/n** (XL^J, worst case |B| = 2d). The checker recomputed m = 3, d = 2^8 as 0.0986 for XL^J and 0.1103 for literal XL:

| d | m = 2 | m = 3 | m = 4 |
|---|---|---|---|
| 2^8 | 0.241 | 0.099 | vacuous (−0.0085) |
| 2^12 | 0.249 | 0.123 | 0.058 |
| 2^20 | 0.250 | 0.125 | 0.0625 |

**Theorem C (auxiliary-variable bases): E[cost] ≥ c√n whenever G(θ) holds at a degree θ with H(θ−Δ) ≥ 4κ|B|^{m/2−1}.**
- **Status:** **proved as an implication; its hypothesis is unproved.**
  - For binary towers and Boolean bits, G is needed only at θ−Δ ≥ τ_m·mk, where τ_m = 0.0246, 0.0417, 0.0615 and 0.0725 for m = 3, 4, 6 and 8, rising to 0.110 as m grows.
- **Clause (i) of G** (detection degree): proved for Boolean bits by the Reed–Muller distance bound. It held on the sampled tower targets. Toy thresholds are trivial, though: for example m = 3, k = 5 gives τ_3·mk = 0.37 degrees, so passing there says nothing about the asymptotics.
- **Clause (ii) of G** (low-degree decomposable targets ≤ (4/H)·min(N_dec, n)): **never tested.** PROOF §5.4(3) itself says sampling cannot test it. The producer's summary, "every toy instance satisfied the hypothesis", is an overclaim.
- **Random-function model:** refuted quantitatively.
  - The frozen detection-degree rule was **refuted**. Counts were 494 / 175 / 1 / 6 at deviations 0 / −1 / −2 / −3, out of 676 non-degenerate targets.
  - Weak degree is refuted on univariate grids, at −2 (d = 32) and −7 (d = 64).
  - The ratios stay large: at least 0.667 of the maximum for both weak and non-degenerate detection degree, and rank at least 0.81 of brute-force size.

### 3.4 What is machine-checked

Lean 4.34.1 with Mathlib v4.34.1, file `ICCounting.lean`:
- **Result:** 12 theorems, exit 0, no `sorry`, standard axioms only (propext, Classical.choice, Quot.sound). The checker re-ran it: exit 0 in 3.4 s with identical axiom output.
- **What they cover:**
  - K1 root and point bounds;
  - |sums| ≤ |B|^m;
  - surplus = 1;
  - refutation and elimination both need degree ≥ deg f;
  - (n/b + b)² ≥ 4n and the master inequality;
  - the palindromic median step.
- **Fidelity caveat (checker):**
  - `surplus_eq_one` is the arithmetic tautology (m(1+s)+1) − m(1+s) = 1. It formalises nothing about presentations.
  - `refutation_needs_degree` and `elimination_needs_degree` cover literal XL only and are close to trivial.
  - **None of the load-bearing pieces is formalised:** Lemma 4, Lemma 6, the Wald step, Theorems B and C, and Semaev's theorem.
  - **"12 theorems check" must not be read as "the theorem is machine-checked."**

### 3.5 Open parts and scope limits of the restricted theorem

1. **F4 and polynomial-calculus solvers are outside the model.**
   - **Towers** (m = 2, k = 2..9): F4's maximum degree is 4–5 against an exact XL degree of 8–22. F4 matrices reach at most 23,644 × 14,284, where XL would need 495 up to 1.1·10^11 columns.
   - **Univariate bases (checker's own msolve run, not mentioned by the producer):**
     - m = 2: F4 degree d+2 against XL's 2d+2;
     - m = 3: F4 degree d+8 against XL's 3d+9;
     - columns grow about d², against grids of 64 to 13,824.
   - **Consequence:** Theorem B's closures are as model-relative as the PKM case. Empirically, F4 cost per attempt still grows about d^{m−1} on univariate bases, which would keep the exponent near 1, **but nothing proves it.**
2. **Hypothesis G is unproved for towers**, and clause (ii) is untested (Section 3.3).
3. **Gaps the checker found that are missing from the producer's list:**
   - Split-Semaev systems are outside the model. These are S_3 chains with m − 2 intermediate unknowns, as used by PKM.
   - Amortisation across targets is excluded, for example a single parametric XL with x(R) as a variable.
   - Relations count only through a block-1 univariate polynomial. This is load-bearing for degenerate targets.
   - Sparse XL is excluded because cost is charged as the full rank.
   - Collision and known-log variants are excluded by A3.
4. **Excluded by scope:** lifting (see Section 5), Weil descent and explicit-table oracles. **Not reached:** m ≥ 6, and grids beyond 65,536 points.
5. **Inconsistency in PROOF.md:**
   - §5.2 says degenerate targets occur only on tiny bases. Yet §5.4 counts 192 of 1,040 sampled decomposable targets as degenerate, because the sampler weights targets by their number of decompositions.
   - The degenerate targets' minimum δ_det/ρ is 0.5, and this is not reported.

---

## 4. PKM chain at m ≥ 6

Producer: lab `pkm-msolve`. Checker verdict: **sound with gaps**. Everything here is `[pcc:unverified]`.

### 4.1 Setup

- **Systems:** Kummer tower V = g·μ_{2^t}, raw presentation, with the Semaev S3 chain and N = m·t variables.
- **Solver:** msolve 0.6.5 (F4), `-t T -v 2 -g 2`.
- **Primes:**
  - p = 40961 (16-bit prime-order curve; main arm A);
  - p = 769 and 12289 (replication);
  - P1 = 2013265921 (31-bit control);
  - 4 committed repo systems.
- **Generator check:** the generator reproduces 5 committed repo systems polynomial for polynomial.
- **Kill rule (preset, frozen 18:31Z, sha256 `cc630314…`):** σ = ΔD/ΔN from t = 2 to t = 3, taking the lower of the two draws.
  - KILL if σ ≥ α*(m,1) = H⁻¹(1 − 3/m).
  - WEAK-KILL if σ ≥ α*(m,2).

### 4.2 Results (arm A, two random targets per cell; "exact" = certified by a degree-(D−1) closure)

| m | t = 1 | t = 2 | t = 3 |
|---|---|---|---|
| 4 | 5/5 exact | 6/6 exact | 7/7 exact |
| 5 | 5/5 exact | 6/6 exact | ≥8/≥8 (censored) |
| 6 | 6/6 exact | 7/7 (exact for r0) | ≥8/≥8 (censored; OOM at 6.9 GB) |
| 7 | 6/6 exact | ≥8/≥8 (censored) | not run |

- **Across primes:** D depends on (m, t), not on the prime. At t = 2 the widths W match exactly at every prime tested, and the first degree-8 round falls at the same round with the same matrix shape.
- **Checks:**
  - "No solution" agrees with exhaustive enumeration on 44/44 finished runs.
  - Planted points vanish on the basis in 8/8.
  - The basis vanishes at 312/312 sampled solutions.
  - 27 exact closures. On all 15 certified random draws, the certified degree equals msolve's D.
- **Strategy-independent rise from 6 to 7 at m = 4:** confirmed by the checker's independent closure (exact sparse modular elimination, a different representation). This is exact in the quotient-ring polynomial-calculus sense, for this presentation.

| m | step | D | σ | α*(m,1) | α*(m,2) | implied exponent if σ persists (w = 1) |
|---|---|---|---|---|---|---|
| 4 | t 2→3 | 6→7 (certified) | 0.250 | 0.042 | 0.017 | 1.14 |
| 5 | t 2→3 | 6→≥8 | ≥0.400 | 0.079 | 0.031 | ≥1.75 |
| 6 | t 2→3 | 7→≥8 | ≥0.167 | 0.110 | 0.042 | ≥0.645 |
| 7 | t 1→2 (fallback) | 6→≥8 | ≥0.286 | 0.135 | 0.050 | ≥1.02 |

The checker recomputed α*, σ and the implied exponents: all correct.

### 4.3 Verdict as the checker judged it

**KILL at m = 4–7 at toy scale under the preset rule.** The "closed" reading is limited to m ≤ 7 and depends on how the slope is extrapolated.

1. **Fragility.**
   - The m = 6 and m = 7 KILLs rest on one-step slopes: a censored t = 3 value at m = 6, and a t = 1→2 fallback at m = 7. Both come from small-base cells (|V| = 2^t < m), which Stage C flags as atypical. At t = 1, m = 7 the planted draw reads D = 7 while the random draws read 6.
   - The repo's own m = 4 ladder on P1 (D = 6, 7, 7, ≥8 at N = 8, 12, 16, 20) has a long-run slope of about 1/8 to 1/6, against the first step's 1/4. The report does not mention this.
   - Under the w = 1 model:
     - one degree per t-step puts m = 7 at exponent 0.518, essentially at parity;
     - halving the slope puts m = 6 at 0.443 and m = 7 at 0.370, both below rho's 0.5, and the item reads INCONCLUSIVE.
2. **Falsification check never exercised.** The preset's only route to SURVIVE was A m4 t4, and it was not run. The repo's committed P1 value for that cell (D = 7, flat) would still read KILL under the t = 2→4 slope (σ = 0.125).
3. **m ≥ 8 is not closed.**
   - If σ = 1/m, then w = 1 already beats rho at m = 8 (exponent 0.43).
   - Under w = 2, σ = 1/m falls below α*(m,2) from m = 14 on (0.0714 against 0.0774).
   - The report's "α*(m,2) ≤ 0.11 is below every measured slope" is therefore misleading.
4. **Censored values.**
   - A stopped F4 run gives only D_msolve ≥ 8. It does not certify that the degree-7 polynomial-calculus closure misses 1.
   - Support for an optimal degree ≥ 8 at m = 5 and 6, t = 3 comes from the repo's closures (RESULTS_STAGE_C Table 2: "cmax 7, no 1"), at other primes.
   - The repo README infers "msolve reached 8, so the degree-7 closure lacks 1". That inference is invalid.
5. **Minor inconsistencies.**
   - `tables_final.txt` and `analysis.json` still list B m5/m6 t3 as "7+".
   - The W lower bounds count only completed rounds; partial rounds reach 171,215 columns (m5 t3) and 237,706 (m7 t2 r0).
   - The quoted s_W values mix t-steps.

### 4.4 Proofs in the PKM lab (checker's reading)

- **Lemma S (auxiliary unknowns add no grid equation):** correct but close to a tautology. Every ideal of F_p^{grid} is principal and is the ideal of its zero set. The "one net equation" reading is a heuristic that the lemma does not prove, and the lemma says nothing about degree.
- **Lemma X (lower bound on the XL degree for any presentation, auxiliary unknowns included):** correct. Both cases and the Cauchy–Davenport step check out; the latter needs E(F_p) of prime order and the **assumed** hypothesis |S| ≥ c·2^t.
  - Its conclusion is weak: degree Ω(N/(m log N)), so D/N → 0. That is below the kill budget for every m ≥ 5.
  - The report's claim that it "removes the no-auxiliary-unknowns assumption from A4–A6" **overclaims**: no exponent-level conclusion is restored. It is about XL only.
- **Lemma B (break-even):** the AM–GM step is correct. Its use depends entirely on the **unproved** assumption that F4's cost is at least M(N,D)^w.
- **Not proved:** a lower bound on the F4 or polynomial-calculus degree that grows with N. That is the theorem that would close PKM rigorously.

### 4.5 Owner decision

The owner decision of 2026-11-11 stopped measurement of the **first fall degree** and named the **last fall degree** as the live question. This lab measured the solving degree D, which is at least the last fall degree, so the decision does not cover it.

The decisive PKM cells (m = 4 at N = 20; m = 2 at t = 13/14) are still pending with the owner and were not run. **The item is not closed by decision.**

---

## 5. Lifting (A8)

Producer: lab `lifting`. Checker verdict: **sound with gaps**. Everything here is `[pcc:unverified]`.

### 5.1 Theorem 2 (asymptotic; paper proof, not machine-checked)

- **Statement.** Take any lifting procedure that draws fresh uniform masks per trial and whose lift outputs have at most β bits. Its per-trial success probability is at most O_r(β^{3r(r−1)} log^{r(r−1)} β)/q.
- **Consequence.** For lifts computable in polynomial time, the expected number of trials is at least q/(log p)^{O(r²)} = q^{1−o(1)}, far above rho's √q.
- **Key new step.** Petsche 2006 Thm 2 replaces Lang's conjecture.
  - Over Q, every curve has Szpiro ratio σ ≤ log|Δ_min|/log 11, so the minimal non-torsion height satisfies λ ≥ 1/poly(log p).
  - The checker verified this against the primary source: arXiv math/0508160v2, whose sha256 matches the corpus record. The constant is c(d,σ) = 1/(10^15 d³σ⁶ log²(104613 d σ²)), and σ ≥ 1 follows from Ogg.
- **Checked line by line:** Lemma 1, Lemma 2 (exact 1/q for a fresh mask), Lemma 3 (packing), Theorem 1(i)/(ii), and the exponent arithmetic of Theorem 2.
- **Scope, as the checker corrected it:**
  - r fixed (the O_r);
  - log log p = o(log q);
  - fresh uniform masks, where every mask draw counts as a trial, including rejected or aborted ones;
  - lifts over Q of polynomial size;
  - relations only among the sampled lifted points.
  - **Not covered:** algorithms that also use other rational points of E′ (Mordell–Weil generators with unknown logs, or Silverman–Suzuki hybrids across trials). For r that grows with p, the numerator is no longer q^{o(1)}.
  - **A strength the producer did not state:** the per-mask bound does not depend on the lift, so trying many lifts of the same mask is still bounded.
- **Inputs not re-read:** Silverman 1990 Thm 1.1 and the positive-definiteness of ĥ are recalled from memory. The checker notes that Silverman's μ(E) may carry b₂ and 2* terms. This leaves the asymptotic result unaffected, but changes the concrete constants.

### 5.2 Other lifting results

| Item | Status (checker's reading) |
|---|---|
| JKSST hypotheses H1, H2 (comparison of height and discriminant; Lang) | Not needed for the exponent. They only buy a constant numerator |
| H5–H8 | Not needed under the fresh-mask model |
| H3 packing sub-lemma | Lean-checked in the repo's JKSST audit; not rebuilt this round (no toolchain in that lab) |
| Q-curve and number-field lifts (Prop 3) | **Only a sketch.** Absolute versus relative height normalisation and the meaning of β are not settled. There is an invariance gap for residue degree f > 1 |
| Silverman–Suzuki / Miller (Prop 4) | **Open.** Beating √q needs lifted rank Ω(log p / log log p), and no unconditional bound forbids that. The "circularity" remark is not an obstruction, because it assumes ECDLP is hard. Huang–Kueh–Tan was not read (abstract only) |
| p-adic homomorphic transfers | Ruled out: for q ≠ p, every homomorphism from Z/q into a group with no q-torsion is trivial |
| p-adic non-homomorphic section defects | Only measured as generic (repo H110, F2). **No proof** |

### 5.3 Concrete sizes (not settled)

- **Conditional on a height-ratio certificate (Theorem 1 at 256 bits).**
  - The producer's margins assumed trivial torsion. With Mazur's bound T = 16 (checker's recomputation), the margins in bits are:

    | r | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
    |---|---|---|---|---|---|---|---|---|
    | margin | +114.8 | +102.0 | +81.8 | +54.0 | +18.1 | −27.8 | −83.5 | −149.5 |

  - So r ≤ 6 is excluded at 256 bits, and **r ≥ 7 is not**.
  - These margins are optimistic on three counts: they use the cell-median ρ_G rather than the worst case (largest single trial 2.68); λ_G is at least the true λ, so ρ_G understates the true ratio; and r = 7–9 reuse the r = 6 value.
  - Under the worst-case ρ, r ≥ 6 is open.
- **"Theorem 2 thresholds" (265 / 842 / 1773 / 3134 / 5015 bits for r = 2..6).** These are toy fits extrapolated 15–300x, not outputs of the theorem.

### 5.4 Toy (simplified xedni: 7-coefficient Weierstrass fit, r ≤ 6, 8–16 bits)

- **Honest arm (69,425 trials):**
  - 1,879 relations, all verified and all holding mod q;
  - 1,818 trivial collisions, against 1,833.5 predicted;
  - 61 non-trivial relations, all with ‖n‖∞ ≤ 2, every one at 8–10 bits;
  - 0 non-trivial relations at 12–16 bits.
- **Rank certification:** 2,440 of 2,440 relation-free trials had rank r certified.
- **Planted arms:** collinear realisation rate ≈ 1.9 ln p / p in Monte Carlo; the positive control was detected 1,316/1,316.
- **Checker re-runs:** bit-identical, apart from one cosmetic label caused by a later edit to `heights.py`. The honest-arm slope "p^{−1.7…−2}" rests on 1–2 counts and is statistically meaningless.

**Suggested A8 row (checker):** "Ruled out asymptotically for fixed r, unconditionally modulo the published theorems. Open at concrete sizes for r ≥ 7 (≥ 6 under worst-case ρ), for Silverman–Suzuki with growing rank, and outside the fresh-mask model." `[pcc:unverified]`

---

## 6. What is NOT proven

1. **An unconditional lower bound for ECDLP against all index-calculus algorithms is an open problem.**
   - This round did not prove one, and nothing here approaches it.
   - The generic-group lower bound of order √q (Shoup/Nechaev; L) does not apply, because index calculus is not a generic algorithm.
   - **The restricted theorem in Section 3 is not such a bound.** It holds only inside an XL/Macaulay cost model that F4 is shown to escape, on tower bases and on univariate bases alike.
2. **No lower bound on the F4 or polynomial-calculus degree, or on cost, is known for Semaev systems,** split or unsplit. This is the single missing theorem behind both the PKM item and the F4 caveat on Theorem B.
3. **Hypothesis G (Theorem C) is unproved for towers.** Its clause (ii) has never been tested and cannot be tested by sampling.
4. **PKM for m ≥ 8 and asymptotically is open.** The m = 6 and m = 7 KILLs are toy-scale and fragile.
5. **Lifting at concrete sizes** (r ≥ 7 at 256 bits), **with growing rank (Silverman–Suzuki)**, **over number fields** (Prop 3 is a sketch) and **with non-homomorphic p-adic sections** is not proved ruled out.
6. **No machine-checked proof** covers any load-bearing step:
   - the Lean in `ICCounting.lean` covers counting and arithmetic pieces only;
   - the JKSST Lean audit covers scalar sub-lemmas;
   - Theorem 2 of the lifting proof is paper only.
7. **Not in any model:** Weil descent beyond A1–A3, norm maps and GHS covers to higher genus, Semaev with m ≥ 5 at scale, trace bases of dimension at least 2, amortisation across targets, and sparse XL.
8. **No standing:** every item in this note is `[pcc:unverified]`. No novelty claim (Lemmas S and X, the Petsche-plus-Szpiro step, Lemma 4) has been checked against research-kb.

---

## 7. Evidence paths

All paths are on the session's scratchpad and are **temporary**. Before this note is committed, they must be copied into the repo or into S3 by SHA-256.

`$SCR` = `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad`.

**Ledger (R1–R4)**
- `$SCR/closure-ledger.md`
- `$SCR/prime-field-ideas.md`
- `$SCR/chase-synthesis.md`, `$SCR/chase/`
- `$SCR/closure/` (galois, ratmap, ratmap-verify, rootfam, semaev-scaling, semaev-verify, minor-constants)

**Restricted theorem (R5)**
- Proof: `$SCR/finish/theorem/PROOF.md` (sha256 `bd19b3a6…`)
- Lean: `$SCR/finish/theorem/lean/ICCounting/ICCounting.lean` (sha256 `b9f7c49e…`) and `$SCR/finish/theorem/lean/ICCounting/check.log`. The Lean toolchain and Mathlib cache (about 4.9 GB) under `$SCR/finish/theorem/lean/` can be deleted after review.
- Code and outputs: `$SCR/finish/theorem/code/`, `$SCR/finish/theorem/out/`
- Adversarial check: `$SCR/finish/theorem-check/`

**PKM msolve (R5)**
- `$SCR/finish/pkm-msolve/`:
  - `PRESET.txt` (sha256 `cc630314…`)
  - `progress.txt`
  - `runs/*.log`, `runs/*.res.json`
  - `checks.jsonl`, `closure/certs.jsonl`
  - `analysis.json`, `tables_final.txt`
  - `breakeven.json`, `enum_pre.txt`
  - `scripts/`
  - `MANIFEST.sha256` (the checker verified 25/25)
- Adversarial check: `$SCR/finish/pkm-msolve-check/` (including `scripts/indep_closure.py`)

**Lifting (R5)**
- `$SCR/finish/lifting/PROOF.md`
- `$SCR/finish/lifting/code/` (`xedni_toy.py`, `heights.py`, `bounds.py`, `crossover.py`)
- `$SCR/finish/lifting/out/` (`tables.md`, `bounds.md`, `rate_fits.md`, `summary.json`, `collinear_mc.json`, `rates.svg`, `*.jsonl`)
- Adversarial check: `$SCR/finish/lifting-check/`

**Repo files cited**
- `/home/user/crypto/research/jksst_xedni_audit_20261101/README.md`
- `/home/user/crypto/research/ic_next10_20261014/03-pkm-degree/README.md` (m = 4 ladder, lines 11–21)
- RESULTS_STAGE_C.md, Table 2 (closures on P1/P0)
- `/home/user/crypto/docs/session-logs/2026-11-11.md` (21:45Z and 22:15Z entries)
- `/home/user/crypto/research/first_fall_closure_20261111/README.md`
- `/home/user/crypto/docs/ic/LEADERBOARD.md:462`

**Before this note becomes a recorded closure**
1. Run research-kb (`kb_status`, plus `search_knowledge` including the negative forms, plus `search_theorems`) and the lead-atlas brief, and record `source_id` and `repo:path` under "Sources checked".
2. File the verdicts as pcc claims, so that they stop being `[pcc:unverified]`.
3. Write `bench.record/v1` records for the msolve and toy runs, with raw data in S3, if any of them is to be cited as a benchmark.
4. Fix the producer overclaims listed in Sections 3.3, 3.5, 4.3, 4.4 and 5.3 in the proof files themselves, before citing those files.
