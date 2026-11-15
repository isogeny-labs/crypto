# Closure ledger: prime-field (GF(p)) index calculus as an ECDLP route

**Standing.** Every verdict, number and argument here is `[pcc:unverified]`.
- **Where the numbers come from:** local toy runs (pure Python or python-flint, plus one C microbenchmark) on a shared L0 VM. Some numbers are quoted from repo records.
- **What was not done:** no pcc claim was filed and no `bench.record/v1` was written. Nothing ran on EKS, nothing was committed, and nothing passed the validation gate.
- **Prior art was never checked properly.** No round could reach research-kb (`scripts/research-kb status`: "no executable found") or the lead atlas (sibling checkout missing). Prior art was found by grepping repo notes. Any "no prior closure found" in this ledger therefore rests on grep alone.

**Rounds combined**
- **R1, desk review of 15 ideas.** One toy script, not recorded in pcc: `/tmp/claude-0/s.py`. Synthesis: `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/prime-field-ideas.md`.
- **R2, chase of 5 ideas.** Synthesis: `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase-synthesis.md`. Evidence: `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase/`.
- **R3, prior-art pass.** A safety stop interrupted it. No computations, no output directory.
- **R4, this closure round.** Six labs. Evidence: `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/closure/`.

**Evidence strength key**
- **T+R:** toy measurement, independently re-run with matching output.
- **T+r:** toy measurement, partly re-run.
- **T:** toy measurement, not re-run.
- **D:** desk or paper argument, not machine-checked.
- **L:** literature, known only through the repo's summary. The original papers were not read.
- **P:** machine-checked proof. No route in this ledger has one. The JKSST Lean audit proves only sub-lemmas.

## 1. Route table

### A. Exponent-level routes (the "equation budget": a factor-base condition that adds equations without adding unknowns)

| # | Route | Round | Verdict | Reproduced | Strength | Evidence | Residual gap to matched rho |
|---|---|---|---|---|---|---|---|
| A1 | Weil descent route 1: base x ∈ GF(p), lifted to GF(p^k) | R2 | Ruled out | Yes (3 of 4 scripts byte-identical) | T+R, D | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase/equation-budget-B/` | No gain. The extra k−1 equations constrain only the twist component. Predicted and measured counts agree on 12 of 12 targets. |
| A2 | Weil descent route 2: subspace base outside GF(p) (Gaudry/Diem in E(GF(p^k))) | R2 | Ruled out | Yes (same scripts) | T+R, D | same | Cost scales as about p^1 against rho's p^{1/2}, an exponent loss. |
| A3 | Galois-orbit / trace base F = Tr(S), S ⊂ E(GF(p^k)) | R4 galois | Ruled out | Yes (full re-run, first 12 columns identical) | T+R for lines through the origin, k ≤ 3, m = 2, p ≤ 809. D for the extension to all S. | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/closure/galois/` (re-run in `.../galois/v/`) | Coverage |Tr(S)|/#E is 0.52–0.74 and does not shrink with p, so \|F\| is about 0.6p. Collecting relations then costs at least about p, exponent 1 against 0.5. Fixing the preimage turns it into A2. |
| A4 | Rational-map image base x = φ(t): id, t², t³, Möbius, Lattès, Vélu, power map μ_n | R4 ratmap (R1 #6 Möbius, desk) | Ruled out in the tested presentations. PKM chain at m ≥ 6 left open. | Yes. yield_count is byte-identical. closure_degree re-ran 32 of 40 rows (timeout), and those match apart from timing fields. | T+r, D | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/closure/ratmap/` (re-run in `.../closure/ratmap-verify/`) | See the three cases below. |
| A5 | Fixed-degree polynomial-root base (Kummer coset, random split, low height {1..d}) | R4 rootfam | Ruled out | Yes (subset re-run, including a new seed, all matching) | T+R for the measurement. The verdict rests mainly on D (the K1 counting argument). | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/closure/rootfam/` | K1 is structural: a fixed f gives \|B\| ≤ deg f, so a base of size p^{1/m} needs deg f ≥ p^{1/m}. The surplus is exactly 1 equation, the same as with no base condition. D_ref = Σ(deg−1)+1 is linear in d. The XL-width exponent e ≥ m is an upper bound from the instrument, not a lower bound. |
| A6 | Semaev m = 3/4 on GF(p) with an interval / low-bit base: MITM table, Boolean-bit Macaulay, APS vanishing-polynomial system | R4 semaev-scaling | Ruled out for the measured oracles | Yes. Different random draws, close values (slopes 0.669 and 0.768 against the reported 0.689 and 0.796). | T+R, L, D (arity bridge) | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/closure/semaev-scaling/` (re-run in `.../closure/semaev-verify/`). The kill rule is fixed in `PRESET.txt`, written before the data. | Fitted cost exponents: 0.69 (m = 3) and 0.80 (m = 4) for MITM; at least 1.10 for Boolean Macaulay. Rho is 0.5. The gap grows from 15x to 53x (m = 3) and from 45x to 373x (m = 4) over 10–20 bits. |
| A7 | Equation-budget-A, first attempt (algebraic base conditions, general) | R2 | No result (safety stop) | n/a | none | no output directory | Superseded by A4, A5 and A3 in R4. |
| A8 | Lifting: classic xedni (JKSST), Silverman–Suzuki, Q-curve lift over a number field | R3, R4 lifting | Inconclusive | No (nothing to re-run) | L plus partial P. The JKSST obstruction is conditional, with 8 open audit obligations. | `/home/user/crypto/research/jksst_xedni_audit_20261101/README.md`; row E14 of `/home/user/crypto/research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (a folding test, not a lift) | Unmeasured. JKSST predicts success about (2C₂)^r/p per trial, which would put the exponent at about 1 or worse. That prediction is conditional on Lang's conjecture, a height–discriminant comparison and a random-relation step. |
| A9 | BKK / sparse-resultant presentation choice | R1 | Ruled out as an exponent lead | No | D | prime-field-ideas.md #7 | The saving is a p-independent constant, bounded by the ratio of D(m) to Bézout. |
| A10 | Richelot-glued (2,2)-split Jacobian | R1 | Ruled out | No | D, plus a repo measurement | prime-field-ideas.md; repo `chained_prime_transfer_20261111` (quoted) | 9.8–42.9x rho (repo figure, quoted). Mumford coordinates add as many unknowns as equations. |
| A11 | Isogeny pushforward with a kernel-closed base | R1 | Ruled out | No | D | prime-field-ideas.md | Collapses to IC on E′ with base \|B\|/l, a relabel. |

The three A4 cases:
- **Parametrised x = φ(t), t in a small set not cut out algebraically:** adds 0 net equations, and yield equals that of a random base (0.98–1.01). This is a relabel, and the gap is that of the incumbent.
- **Lattès / Vélu images:** a coset filter that cuts yield to 0.34–0.66.
- **Algebraic μ_{2^k}:** D* = deg S + m·k, and the Macaulay space at D* is the full \|V\|^m. So each relation costs at least about p: exponent 1 against 0.5.

### B. Constant-factor and accounting routes inside the explicit-table family

| # | Route | Round | Verdict | Reproduced | Strength | Evidence | Residual gap to matched rho |
|---|---|---|---|---|---|---|---|
| B1 | CM-orbit base, j = 0/1728 (order-6 orbits) | R1, R2 | Real constant-factor gain inside IC, already in the repo (E1/E6/E12). Does not change the exponent. | Partly (20,064 runs checked; summary regeneration timed out) | T+r | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase/cm-orbit/` | 4.6x rising to 7.9x against order-6 rho over 16–22 bits (w = 0, fermat). IC grows about n^0.64 against n^0.51. Net gain over the folded incumbent is 1.25–1.62, below √3. |
| B2 | Paired translate x(R±T) from one inversion (= repo `sib`) | R1, R2 | Unresolved under §5a. The order flips across the band. | Partly (projection checked by hand; toy not re-run) | T+r | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase/paired-translate/` | Projected 6.0x rising to 25.7x at w = 0, and 12.9x rising to 32.6x at w = 1, over 20–32 bits. Derived, not measured. Unchanged once rho gets siblings. |
| B3 | Hybrid collision (IC tables plus rho walks, unknown-log or known-log table) | R2 | Ruled out | Partly (20-bit rows and the theory reproduce; 24-bit rows not re-run) | T+r, D | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase/hybrid-collision/` | 4.56x / 5.94x / 6.84x rho-neg at 20/22/24 bits (w = 0). Analytic floor at least 1.6·√h × rho-neg for any explicit-table member. |
| B4 | Isogeny-class shopping | R1 (desk), R4 | Ruled out | Yes (code claims and microbenchmark re-checked) | T+R, D | `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/closure/minor-constants/` | Counted gain exactly 0; wall gain at most about 0.25%. Isogenous curves share #E, so rho benefits equally. The 20-bit baseline sits at 7.75 / 8.02 / 9.35x rho-neg at w = 0 / 0.1 / 1 (quoted from `/home/user/crypto/docs/ic/LEADERBOARD.md:462`). |
| B5 | Legendre-symbol prefilter | R1 (desk), R4 | Ruled out | Yes | T+R | same directory | The filter halves hits along with probes (49.4–50.1% of hits kept), so it is a net loss at every w. |
| B6 | Linear-algebra share at 20–32 bits | R4 (plus repo `sparse_la_timing_20261018`) | Ruled out as a lever | Yes (4,444 records) | T+R | same directory | Removing LA entirely saves at most 0.52% pooled at w = 1 (worst single record 3.8%). |
| B7 | DP = decomposable point (rho/IC dual exit) | R1 | Ruled out | No | D | prime-field-ideas.md #5 | At least min(rho, IC)·(1+ε). |
| B8 | Slope guessing λ_i | R1 | Ruled out | No | T (`/tmp/claude-0/s.py`), D | prime-field-ideas.md | 6,776 distinct slopes out of 7,140 pairs (0.949), above the 0.9 kill threshold. Equivalent to the pair table. |
| B9 | Doubling-fibred base with an x([2]P) prefilter | R1 | Ruled out | No | D | prime-field-ideas.md | x([2]P) is a function of x(P), so it adds no information. |
| B10 | Resultant against V(X) with subproduct trees | R1 | Ruled out | No | D | prime-field-ideas.md | Replaces an O(1) membership test with Õ(B). |
| B11 | Batched x^p over a product tree | R1 | Ruled out | No | D | prime-field-ideas.md | M(kd) ≥ k·M(d), so batching saves nothing. |
| B12 | Early abort of hopeless targets | R1 | Ruled out | No | D | prime-field-ideas.md | The cost per target is already one O(1) lookup, so there is nothing to abort. |
| B13 | Batched affine normalisation (Montgomery's trick) | R1 | Already implemented | n/a | code reading | `/home/user/crypto/src/cryptanalysis/ic_framework/plugins.rs:3324-3343` | Already in the baseline. |

## 2. Conclusion

### What is ruled out, and how strongly

1. **The whole explicit-table family (B1–B13): ruled out as a way to reach rho.**
   - Every measured or projected arm stays 4.5x to 33x above matched rho, and the gap grows with group size wherever growth was measured (B1, B2, B3).
   - Toy evidence (T+R / T+r) and a desk floor (at least 1.6·√h × rho-neg) support this.
   - The only lever that improves the IC baseline itself is the CM-orbit fold (B1), net 1.25–1.62 on j = 0/1728 curves. It is already in the repo, and it widens rather than closes the exponent gap.
   - **Strength:** strong empirical and analytic evidence at 16–32 bits, not a proof.
2. **The equation-budget gap, through the constructions tested: closed at toy scale.**
   - **Weil descent (A1, A2) and the Galois trace (A3):** measured, with a paper image/degree argument that is not machine-checked.
   - **Fixed-degree roots (A5):** closed mainly by a counting argument (K1), which is about as close to rigorous as anything here, though not formalised.
   - **Rational-map images (A4):** closed for the parametrised, homomorphism-image and power-map presentations. The power-map closure has a uniqueness-of-inverse argument behind its Macaulay lower bound.
   - **Semaev m = 3/4 (A6):** measured exponents 0.67–0.80 are consistent with the repo's arity bridge and with the literature as the repo summarises it (McGuire–Mueller about O(p); Yokoyama et al. 2020 lower bound).
   - **Strength:** toy evidence at 10–21 bits plus paper arguments plus literature known second-hand. Toy-scale closures are evidence, not proof. None of the six mechanisms got a counterexample or even a marginal fit; every kill margin was wide (the lowest lower CI was 0.642 against a kill line of 0.55).
3. **Nothing in this ledger has a machine-checked proof.** The only Lean work (the JKSST audit) proves scalar sub-lemmas of a conditional argument.

### What remains open (minimal set), and why

1. **Auxiliary-variable presentations of an algebraic base. This is the one live exponent question.**
   - **Cases:** PKM tower/chain presentations at m ≥ 6, and other 0-dimensional circuits with free intermediate unknowns (Kummer or digit towers, carry-bit encodings of Semaev).
   - **Why still open:** A4's uniqueness argument, A5's K2 count and A6's Boolean bound all assume no free auxiliary unknowns. The repo's prior art files PKM m = 4 as "undecided" (F4 degrees 6, 7, 7 against a break-even of 4).
   - **What would close it:** one real F4/msolve degree measurement at m ≥ 6, or a written argument that auxiliary unknowns cannot raise the net surplus above 1. msolve was not installed.
   - **Prior pressure is negative:** the measured m = 4 degrees already exceed the threshold, and the owner decision of 2026-11-11 stopped further first-fall-degree measurement. If that decision covers PKM m ≥ 6, this item is closed by decision, not by evidence.
2. **Lifting (A8): open only in the sense of being unmeasured.**
   - The literature (JKSST, conditional) predicts per-trial success of about 1/p, which would make it non-viable.
   - **Why not closed here:** three consecutive rounds stopped before any lifting toy computation, so no measurement exists. It should not be reassigned with the same framing.
   - **Two ways to close it:** (a) a reviewer reads `/home/user/crypto/research/jksst_xedni_audit_20261101/README.md`, the `lift_dense_*`, `lift_repulsion_mechanism_20261027` and `p2_lift_linear_complexity_20261110` directories, and decides on paper; or (b) the user accepts the conditional JKSST obstruction as sufficient.
   - Silverman–Suzuki and Q-curve lifts over a number field have no repo study at all.
3. **Untested neighbours, low priority:**
   - norm maps and GHS covers to higher genus (A3 covers only the trace);
   - m ≥ 5 Semaev;
   - S of dimension at least 2 for the trace base.
   
   The paper arguments for A3, and the A10 measurement for genus-2 transfer, point the same way. These are listed for completeness, not because there is a reason to expect a different outcome.

### Bottom line

At the scale and rigour this lab has applied, prime-field index calculus is not a viable route to beating Pollard rho. Every construction measured either costs an exponent of at least 0.67 (against 0.5), only relabels the base, or gives a constant factor that rho matches. That conclusion is strong toy-plus-argument evidence, consistent with the literature (as summarised in the repo), but it is not a proof.

Two gaps keep it short of "fully ruled out":
- auxiliary-variable (PKM-type) presentations at m ≥ 6;
- an unmeasured lifting route that rests on a conditional obstruction.

### Before recording this as a closure

1. **Check prior art properly.** Run research-kb (`kb_status`, plus `search_knowledge` including negative forms) and the lead-atlas brief once they are reachable. No round has done this, so the novelty and "no prior closure" statements here are grep-only.
2. **Give the verdicts standing.** File the ruled-out verdicts as pcc claims so they stop being `[pcc:unverified]`.