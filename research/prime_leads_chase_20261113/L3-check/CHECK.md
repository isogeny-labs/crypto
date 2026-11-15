# Red-team check of L3-digit (Theorem F, its projections and measurements), 2026-11-13

Reviewer: red team (Opus 5.5). Every number in this note is `[pcc:unverified]`. research-kb was not
available, so no prior-art or novelty statement here has been checked against the knowledge base.
I did not modify the lab directory `../L3-digit/`. Everything I produced is under this directory.

## 0. What was reviewed, and how durable it is

- **Evidence reviewed:** the lab directory as frozen by `L3-digit/MANIFEST.sha256` (written 02:39:02Z).
- **Durability:** this is scratch evidence, not durable. It is not committed to Git, has no pcc claim
  and no `bench.record/v1`. Two files changed after the manifest was written:
  - `pcdeg.rs` (02:40:44Z) gained two `eprintln!` progress lines. The closure code is unchanged.
  - `AMENDMENTS.md` gained A4, the upper-half test at m = 3, k = 6..8, which was still running.
- **Recovering the frozen source:** my first read of `pcdeg.rs` had captured the manifest version, which
  I recovered as `src/pcdeg_orig.rs` (sha256 `0e23bd07…` = manifest). Compiling it with
  `rustc -O` gives a binary that is bit-identical to the lab's: sha256 `efde7e52…` = manifest = `pcdeg.sha256`.
- **Missing artifacts that limit the review:**
  - The theorem-lab `PROOF.md` that defines hypothesis G, Δ, H and κ is not available here. The
    `$SCR=/tmp/claude-0/-home-user/ad79db08-…` directory no longer exists, so only the closure note's
    summary was available.
  - No producer report states the KILL verdict in words. I read it off the frozen rule (PROTOCOL.md) and
    `project.txt`.

## 1. Reproduction (single core, prlimit 3 GB, timeout 1200 s)

| check | result |
|---|---|
| `lemmaF_check.py` on the 24 manifest-hashed record files (copied to `frozen/`, hashes re-verified) | `2215/2215 closure levels below D match` (identical to `LEMMAF_CHECK.txt`) |
| `analyze.py`, `census.py` on the same files | output byte-identical to `TABLES_main.md` and `CENSUS.md` |
| pcdeg re-runs with the producer's exact arguments (`spot.sh`, `out/s*.jsonl`): bool m2 k6 (x2), b3 m2 k3 (x2), bool m3 k4 (x2), uniint m3 B4 (x2), bool m2 k5 planted (x2), lin k8 (x2), bool m4 k3 (x1) | 13/13 records identical to the frozen records (all fields except wall time) |
| pcdeg's S4/S5 formulas (ported) vs. sympy resultants (`scripts/s45_port_check.py`) | ratio exactly 1 at random points. The sympy S4/S5 vanish on genuine 4- and 5-point decompositions |
| f_top claim: the x-top part of S_{m+1}(x_1..x_m, r) is (x_1⋯x_m)^{2^{m−1}} (`scripts/semaev_top.py`) | degree exactly m·2^{m−1}, ratio 1, for m = 2, 3, **and 4** (random r and y; numeric, not symbolic) |
| Wilson rank formula as cited (`scripts/wilson_check.py`) | 488/488 matches over v ≤ 9, t ≤ min(K, v−K), p ∈ {2,3,5,7} |
| Boolean j* = ⌊(k−e)/2⌋+1 with the actual weights 2^i | holds for every p > k tested (k ≤ 10, e ∈ {2,4}). Fails for some p ≤ k (e.g. k=8, e=2, p=5: j* = 3, not 4), so the p > k condition is needed |

**How much the LEMMAF_CHECK headline actually covers.** The 2215 levels are not 2215 independent tests
of the digit-presentation case:
- 1803 of the levels (81%) come from the univariate families, where Lemma F is trivial: multiplication
  by x^e on F[x]/(x^B).
- 240 levels are at c = deg f, where the rank is 1 trivially.
- The 36 `supp` levels are not covered by Lemma F at all.
- For the digit families with deg f = m·2^{m−1} (the regime of Theorem F), there are 143 levels: 60
  trivial ones (c = deg f), 41 at c = deg f + 1 and 42 at c ≥ deg f + 2.
- Every m = 3 digit cell has ffd − m·e ∈ {1, 2}.
- Every m = 4 cell is degenerate (k < e, deg f < m·e), so Theorem F's statement is never exercised at m = 4.

The theorem is proved, so this does not make it wrong. It only means the empirical confirmation of the
digit case is thin.

## 2. Findings, ordered by severity

### F1 (unsupported scope). "Lower bound for every degree-bounded PC/F4/XL-with-mutants strategy" holds only for the axiom set {f} ∪ digit equations

**What is sound.** The closure argument is correct for:
- PC with lines in A, or in the ordinary ring with Π(d − c) as axioms;
- rules = F-linear combination plus multiplication by a variable, under a degree cap;
- F4/XL/MutantXL in any degree-compatible order (V_{F,c} = rowsp M_c, Caminata–Gorla as summarised in
  RESEARCH_FALL_DEGREE_BOUNDS.md §0);
- any extension field;
- any variable order, since V_c depends on none;
- affine changes of digits.

**What it does not cover:**
- other encodings of the same decomposition problem;
- non-affine re-coordinatisations;
- extension variables;
- target-parametric (amortised) runs.

**Test I ran (`bin/splitpc.rs --split 1`, `out/split_test.jsonl`).** The split S_3 chain
{S_3(ℓ_1, ℓ_2, u), S_3(u, ℓ_3, r)}, with one unrestricted auxiliary variable u, was run on the same
curves and targets as the frozen records:

| presentation (m = 3) | unsplit D (= ffd) | split D | split cols at D (unsplit cols) |
|---|---|---|---|
| bool k=2 | 7 | 6 | 256 (64) |
| bool k=3 | 10 | 7 | 1793 (512) |
| bool k=4 | 13 | 8 | 12381 (4096) |
| b3 k=2 | 13 | 8 | 2223 (729) |
| mix k=3 | 13 | 8 | 5244 (1728) |
| uniint B=8 | 16 | 12 | 1610 (477) |
| uniint B=12 | 20 | 16 | 4635 (1276) |

Control: on 3 planted (satisfiable) targets the split closure never contained 1, and its corank equalled
the number of solutions (6, 6, 3).

**Conclusion:**
- Theorem F is a lower bound for *this presentation*, not for the problem.
- It also does not cover the split system the closure note already lists as outside the model (§3.5).
- At toy size the split encoding is lower in degree but larger in columns, so this does not show that
  splitting is cheaper.

**Cheapest settling test:** split D and cols at m = 3, Boolean k = 5..8, on the pod (k = 5 needs about
85k columns, so dense rows exceed 3 GB). Alternatively, run msolve on the split chain.

### F2 (unpriced). The cost proxy cols(D) = dim A_{≤D} is not a lower bound on any solver's cost, and the "measured full rank" is not evidence for it

- **Full rank is automatic.** If 1 ∈ V_D, then every monomial of degree ≤ D is reachable from 1 by
  multiplying by variables, so V_D = A_{≤D}. "Rank at D ≈ dim A_{≤D}" is therefore a consequence of
  closing to the fixed point, not an observation about cost.
- **Ordering does not change it.** In my order test (`out/order_test*.jsonl`: LIFO as in pcdeg, FIFO,
  random), the rank at the moment 1 first appears was equal to cols (or cols − 1..5) in every case. The
  closure algorithm fills the space before it finds 1, whatever the order.
- **F4 does not pay that cost.** The lab's own msolve runs on univariate bases reach max degree B + 2
  with matrices of 265×759 (B = 128) and 521×1527 (B = 256), against cols(D) = 8634 at B = 128. So a
  degree lower bound that is linear in B coexists with F4 matrices of O(B) columns.
- **Status of the charge.** The only size lower bounds that follow are weak:
  - every PC proof contains f, and |supp f| ≥ C(k, 2^{m−1})^m on Boolean digits;
  - Impagliazzo–Pudlák–Sgall size–degree gives exp(Ω(k/m)) (recalled from memory).

  The full-rank charge is therefore a model assumption: dense XL at degree D.
- **Cheapest test:** read the per-step F4 matrix sizes from the msolve Boolean logs (bool m = 2, k = 2..8)
  against B. A log_B(cols) that keeps falling (2.73 → 2.16 in MS_TABLE) would put the m = 2 slope below
  the cols(D) charge.

### F3 (wrong as stated). The univariate bound D ≥ B + (m−1)·2^{m−1} ("fully proved") needs B > 2^{m−1}

- The lab's own certified rows contradict it:
  - uniint m = 3: B = 3 → D = 7, and B = 4 → D = 10. The bound says 11 and 12. I re-ran B = 4 and
    reproduced D = 10 twice.
  - uniint m = 4: B = 2..6 → D = 5..21. The bound says B + 24.
- **Why:** for B ≤ e, x^e = 0 in F[x]/(x^B), so f_top ≠ (x_1⋯x_m)^e and deg f < m·e.
- `analyze.py` handles the case correctly (`if top < e`), but THEORY.md does not.
- **Repair:** add "B ≥ 2^{m−1} + 1" (and, for Boolean, k ≥ 2^{m−1}, which THEORY does state).
- For general m, f_top = (x_1⋯x_m)^{2^{m−1}} is checked symbolically only for m ≤ 3 (by the producer) and
  numerically at m = 4 (by me). It is unproved for m ≥ 5.

### F4 (wrong under one-relation accounting). The protocol's E_m = 2/(m+1−θ) is used for θ > 1, which puts B above p^{1/m}

- At the "balanced" B = p^{1/(m+1−θ)} with θ > 1, the number of trials per relation, p/B^m, is below 1.
  That is fractional trials.
- With at most one relation per solve (B ≤ p^{1/m}), the exponent is E = (1+θ)/m for 1 < θ < m − 1.
- The protocol formula is valid only if one solve on a target with about B^m/p decompositions outputs all
  of them at the refutation cost. That assumption is unstated and unmeasured; every measured target had
  B^m ≪ p. It is the same hidden axiom the closure-note checker found in Theorem A.
- Results (`out/proj_check.txt`, LB model D = ⌈k/2⌉+1, k = 160):

| m | θ | E_protocol (ω=1) | E_one-relation (ω=1) | E_one-relation (ω=2) |
|---|---|---|---|---|
| 4 | 2.166 | 0.706 | 0.791 | ≥1 |
| 6 | 2.478 | 0.442 | 0.580 | 0.993 |
| 7 | 2.595 | 0.370 | 0.514 | 0.884 |
| 8 | 2.695 | 0.317 | 0.462 | 0.799 |

- E < 0.5 first holds at m ≥ 6 → **m ≥ 8** (ω = 1) and m ≥ 9 → **m ≥ 15** (ω = 2).
- The error favours the attack, so every KILL reading (E ≥ 0.5) survives. "E_4 ≥ 0.70" is true but loose
  (≥ 0.79).
- "E_m < 0.5 for m ≥ 6 at ω = 1" is wrong under one-relation accounting. It is correct only under the
  unstated many-relations-per-solve assumption.

### F5 (unsupported). "Settles hypothesis G clause (ii) for these presentations"

**What Theorem F does give.** For every target, nothing in V_c with c < ffd has degree < m·e. In fact
more holds (my derivation from Lemma F): every monomial in the top component t_top·f_top has degree
≥ e in *every* block. So no consequence below ffd is a block-1-only (or any fewer-block) polynomial.
Clause (ii) therefore holds vacuously at every threshold θ < ffd, for the unsplit digit presentation.

**Whether that is the θ Theorem C needs.** Per closure note §3.3, G is needed at θ − Δ ≥ τ_m·m·k.
With Δ = deg f = m·e (my assumption; PROOF.md is not available), Theorem F covers that θ only when
τ_m·m·k ≤ ⌊(k−e)/2⌋:

| m | covered for | 256-bit p allows |
|---|---|---|
| 3 | k ≥ 6 | – |
| 4 | k ≥ 14 | – |
| 6 | k ≥ 124 | k ≤ 42 |
| 8 | never (τ_8·8 = 0.58 > 1/2) | – |

**What the census adds.** The census tests m = 2 only. At m = 2, Theorem A applies, not C. The census
therefore checks Theorem F (0 targets below ffd); it does not test clause (ii) where Theorem C uses it.

**Remaining scope limits:**
- split (auxiliary-variable) systems are not covered (F1);
- Theorem C's own scope is auxiliary-variable bases.

**Cheapest test:** obtain PROOF.md and state Δ and the required θ explicitly for each m.

### F6 (unsupported). The KILL label versus "D grows linearly in k"

- **They are consistent, within scope.** D ≥ ffd with slope 1/2 and n = mk variables gives
  θ → m·H(1/(2m)): a constant exponent, so the cost is polynomial in B. The frozen rule projects to
  m = 4 only. There θ ≈ 2.17 ≥ 1 gives E_4 ≥ 0.79, so KILL under the frozen rule, within the cols(D)
  charge.
- **The same charge does not kill m ≥ 8** (one-relation; m ≥ 6 by the producer's own formula).
- **At p = 2^256 with only proved bounds** (`proj_check.txt` (c)), E is 0.98 (m=4), 0.70 (m=6),
  0.52 (m=7) and 0.465 (m=8).
  - For m ≥ 7, k ≤ 256/m < 2^{m−1}, so Theorem F's closed form does not apply. Only Theorem S's weak
    bound is proved.
  - The saturation D = n + 1, observed at k < e, is empirical.
- **Conclusion.** "Digit presentations: KILL" must be scoped to "m = 4, unsplit system, dense-XL charge,
  toy-scale measurements at m = 2 projected to m = 4". It is not a closure of the lever.
- **Other gaps:**
  - The `mix` family has no E_4 projection in `project.py` (b ∈ {2,3,4} only), so its frozen-rule
    verdict lacks the second condition.
  - Every m = 3 Boolean cell is saturated (bool k=4: D = 13 = n+1, cols = 2^12; k=5: cols 32752 of
    32768), so the slope law is not observed beyond m = 2.

### F7 (minor gap). Theorem S proof step

- The step "V_c(q) ⊆ V_c(L)" is shown only for c ≥ deg_x q or c < deg_A q.
- The case deg_A q ≤ c < deg_x q is not handled. It can arise when k < deg_x q ≤ 2^{m−1}. The cofactor
  q/L can then have A-degree ≥ c.
- Theorem S is superseded by Theorem F when k ≥ e. Where k < e, however (m ≥ 7 at 256-bit p), it is the
  only proved bound, so the gap is load-bearing there.

### F8 (process). Post-hoc theory and late edits

- Theorem F, jstar.py and lemmaF_check.py came after the first rows. A3 discloses this.
- The out-of-sample cells listed in A3 agree with ffd.
- The edits made after the manifest (pcdeg.rs, A4) are disclosed in AMENDMENTS but not re-hashed.

## 3. Verdicts

| claim | verdict |
|---|---|
| Theorem F (V_c = f·A_{≤c−me} below ffd; no refutation and no consequence of degree < me, for every target) | **sound with gaps**: needs f_top ≠ 0 in gr (k ≥ e / B > e, p > e) and the f_top identity for general m (m ≤ 3 symbolic, m = 4 numeric) |
| "lower bound for every degree-bounded PC/F4/XL-with-mutants strategy" | **sound with gaps** as a *degree* bound on the axiom set {f} ∪ digit equations; **unsupported** for the decomposition problem (split encoding: D = 8 < 13) and as a cost bound |
| Boolean closed form via Wilson, p > k | **sound** (formula re-checked numerically; citation still not read from source) |
| univariate D ≥ B + (m−1)2^{m−1} | **unsound as stated** (fails for B ≤ 2^{m−1}, including certified lab rows); sound with B > 2^{m−1} for the unsplit system |
| settles hypothesis G clause (ii) | **sound with gaps / partly insufficient evidence**: vacuous below ffd; covers the needed θ only for m = 3 (k ≥ 6) and m = 4 (k ≥ 14) if Δ = deg f; PROOF.md missing |
| cost model, θ, E_4 ≥ 0.70 | **sound in-model** (loose; ≥ 0.79 with one relation per trial); the cols(D) charge is a model, not a lower bound |
| E_m < 0.5 for m ≥ 6 at ω = 1 | **unsound** under one-relation accounting (m ≥ 8); holds only with an unstated many-relations-per-solve assumption, and only as k → ∞ |

## 4. Strongest correct statement

For the unsplit digit presentation, take:
- A = ⊗ F_p[d]/(Π_{c∈X}(d − c)) with all weights nonzero;
- the single axiom f = S_{m+1}(ℓ_1..ℓ_m, r);
- e = 2^{m−1}, and suppose (ℓ_1⋯ℓ_m)^e ≠ 0 in gr(A). For Boolean digits this means k ≥ e and p > k;
  for a univariate base it means B > e. This is proved for m ≤ 3 and checked numerically for m = 4.

Then, for every target and every c < ffd = m·e + j*:
- V_c(f) = f·A_{≤c−m·e}, and every nonzero element has degree ≥ m·e and involves all m blocks;
- so every PC, F4, XL or MutantXL computation on {f} ∪ digit equations (any degree-compatible order,
  any extension field) needs degree ≥ ffd to refute a target or derive a linear consequence;
- ffd = m·e + ⌊(k−e)/2⌋ + 1 for Boolean digits and B + (m−1)·e for a univariate base. The measured D
  equals ffd on every certified toy row.

This is a degree bound for this axiom set only:
- the split S_3-chain encoding refutes at lower degree (8 against 13 at m = 3);
- degree does not bound F4 cost (msolve univariate runs).

Under the dense-XL full-rank charge with one relation per trial:
- E_4 ≥ 0.79 (≥ 0.70 under the protocol formula), so KILL at m = 4 within that model;
- the same charge gives E < 0.5 for m ≥ 8 (ω = 1), so it does not close digit presentations in general.

## 5. Files

- `CHECK.md` (this file)
- `src/pcdeg_orig.rs`: recovered frozen source (sha256 = manifest)
- `bin/pcdeg`: rebuilt, bit-identical to the lab binary
- `bin/splitpc.rs`, `bin/splitpc`: red-team variant (`--order 0|1|2`, `--split 1`)
- `spot.sh`, `out/s*.jsonl`, `out/spot.log`: re-runs
- `frozen/`: manifest-hashed record copies; `out/lemmaF_rerun.txt`, `out/tables_rerun.md`, `out/census_rerun.md`
- `out/split_test.jsonl`, `out/order_test.jsonl`, `out/order_test_random.jsonl`
- `scripts/semaev_top.py`, `scripts/s45_port_check.py`, `scripts/wilson_check.py` (output `out/wilson_check.txt`)
- `scripts/proj_check.py` (output `out/proj_check.txt`)
