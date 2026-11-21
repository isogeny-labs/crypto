# Solving degree alongside d_ff and d_lf — results (2026-11-15)

Protocol: `PROTOCOL.md` (frozen 02:05Z; two dated amendments on naming and location, predictions
unchanged). Data lanes and the proof note live in ml-cryptanalysis, branch
`claude/lfd-proofs-20261108`: `experiments/ic-bounds-solving/lane{A,B,C,D}*/` and
`docs/proofs/SD1-solving-degree.md`. The in-repository computation is `rust_check/` (Rust, no
crates). Every number below is [pcc:unverified]: no validator has run and no pcc claim id exists.

## 1. The three invariants, side by side

| invariant | definition | unit prime chain, m summands, depth k |
|---|---|---|
| d_ff | first strict fall of the iterated closure V_{F,D} | 4 (m = 2, measured k = 3..9); 7 (m = 3, k = 2, 3) |
| d_lf | last fall degree (Huang–Kosters–Yeo); equals Caminata–Gorla's solving degree for a unit ideal (their Thm 3.1: sd^CG = max(d_lf, max.GB.deg), re-read) | ⌊(k−10)/14⌋+1 ≤ d_lf ≤ k+2 (proof: N8, U1); measured 4,4,4,5,5,5 at k = 3..8 (lane A); recorded exact 5 at k = 9 (ic-bounds-exact10), 6 at k = 10..13 and 7 at k = 14 (ic-bounds-k14 unit p = 65537 runs: 1 ∉ V_5 / 1 ∈ V_6, resp. 1 ∉ V_6 / 1 ∈ V_7); law ⌊(k+14)/4⌋ matches all of them |
| sd^stat | static (XL, no-mutant) Macaulay degree: least D with 1 ∈ span{m·f : deg(m f) ≤ D} | **= δ + maxwt(supp g^{-1})** (proof, SD1 §3); = δ + mk iff c_top ≠ 0; measured 2k+2 (m = 2), 3k+6 (m = 3) |

Proof sketch (SD1, lane D red-team found no gap): the chains are a Gröbner basis with coprime
leading terms, so ker NF ∩ R_{≤D} is spanned by chain multiples of degree ≤ D; multiplying by a
variable raises the Hamming weight by at most one, so NF(Mac_D) = g·M_{≤D−δ}; for a unit g the
only h with gh = 1 is g^{-1}, so 1 ∈ Mac_D ⇔ g^{-1} ∈ M_{≤D−δ}. The top coefficient is the character
sum c_top = N^{-m} Σ_{α∈H^m} (α₁⋯α_m)/ĝ(α); for fixed (a, b) it vanishes on at most 4^k − 1 unit
targets t unless it vanishes identically (bound attained at k = 1), so the exceptional proportion is
O(1/p) at fixed k only. δ is the ordinary degree of ĝ (2 for the normalized S_3, 6 for NF(S_4), 5 on
5t³+3at+6b = 0), not the weight of g.

## 2. Preregistered predictions

| id | prediction | outcome |
|---|---|---|
| P1 | brute-force ordinary-ring sd^stat = 2 + maxwt(supp g^{-1}) | **held**: 18,828/18,828 instances, k = 3..6 (lane A); independent brute force 174/174 incl. raw S_3 and random generators (lane D); m = 3: 40/40 at k = 2, 3 (lane B) |
| P2 | fraction with sd^stat = 2k+2 ≥ 1 − 3/p at k = 3..12 | **held** in every (k, p) cell with unit instances; exceptions only at k = 3, 4 (159 of 29,775 rows), rate ≈ 1/p (p = 73: 40/3060 = 1.31% vs 1/p = 1.37%), all with c_top = 0 and sd^stat = 2k+1; none at k = 5..13 (1/p too small to see). Rust: p = 17, k = 3: 112/1983 exceptions, 0 at p ≥ 257 |
| P3 | d_lf ≤ k+2 < sd^stat, gap ≥ k | **held**: gap 4, 6, 8, 9, 11, 13 at k = 3..8 (measured d_lf), 15, 16, 18, 20, 22 at k = 9..13 (recorded exact d_lf 5, 6, 6, 6, 6); equality gap = k only on the k = 3 exceptions |
| P4 | m = 3: sd^stat = δ₃ + 3k, δ₃ = 6 | **held**: 6 + 3k on all generic unit instances k = 2..8 at p ≥ 1153; exceptions (≈ 1/p) at p ≤ 113 with sd = 3k + 5; locus δ = 5 |
| P5 | binary: d_full ≥ d_lf on every row; d_full grows, d_lf does not | **held**: 0 rows with d_full < LFD or D* > d_full; on proper m = 2 systems d_full = k+1 or k+2 on 185/185 exact rows (k = 2..8; LFD 3 → k+2, LFD 4 → k+1 at k = 7, 8), ≥ 8 at k = 9, ≥ 7 at k = 10; LFD ∈ {3,4,5}, d_ff ∈ {2,3} |
| P6 | the written proof survives an adversarial read | **held**, with qualifiers: δ = ordinary degree of ĝ; no lower bound without genericity (ĝ = u_{1,k−1} has sd^stat = 2); the O(1/p) bound is vacuous once 4^k ≥ p |

Cross-check Python ↔ Rust on the shared list (`rust_check/data/CHECK_LIST.md`): identical maxwt and
sd on 4/4 rows tried by the main session (k = 5 p = 12289; k = 8 p = 1073750017; the p = 17, k = 3
exception; the k = 1 case with the four coefficients 91, 81, 81, 95).

## 3. Growth formulas (what is proved, what is measured)

* Prime, m = 2, unit ideal: d_ff = 4 (measured; proof of the homogeneous-syzygy value in
  IC_FALL_PROOFS), d_lf ∈ [⌊(k−10)/14⌋+1, k+2] (proof), sd^stat = 2k+2 for c_top ≠ 0 (proof +
  measured k = 3..13). Static/iterated gap ≥ k (proof under U1), ≈ 7k/4 under the measured law.
* Prime, m = 3: d_lf = 7 (k = 2..5), sd^stat = 3k + 6, gap 3k − 1; above the K1 bound mk+1 by 5.
* Binary, m = 2, proper: d_full^stat ∈ {k+1, k+2} (measured k ≤ 8), d_lf by law B (2 + [k+e ≥ 3] +
  [k+e ≥ 9] + [k+e ≥ 19]; item 5 of 2026-11-14 confirmed the k = 18 step), kernel bound r + 1. No
  inverse formula exists in the Boolean ring (several generators; the only unit is 1), so the binary
  static degree stays empirical.
* Reading for index calculus (SD1 §7): a plain XL / static Macaulay solver must reach degree 2k+2
  (prime, m = 2) where the mutant / iterated closure reaches d_lf ≤ k+2 and about k/4 in practice;
  the semi-regular yardstick under-predicts the binary static degree by 4–5 at k = 8.

## 3b. Independent validation (2026-11-15, after the lanes; validator agent, own code only)

Report: ml-cryptanalysis `experiments/ic-bounds-solving/validation/VALIDATOR_REPORT.md` (frozen
dispatch commits crypto ef468c885b, ml-cryptanalysis bc98cdc7; producer files pinned by sha256).

| statement | disposition | basis |
|---|---|---|
| (a) sd^stat = δ + maxwt(supp g^{-1}) | **validated** (k ≤ 5 for m = 2, k = 2 for m = 3) | proof re-derived, no gap; own literal ordinary-ring brute force vs own inverse DFT: 344/344 agree (7 primes), incl. 6 exceptions and 5 locus instances; sd = δ + mk ⇔ c_top ≠ 0 in all 344 |
| (b) sd^stat = 2k+2 (m = 2, k = 3..13), 3k+6 (m = 3, k = 2..8) | **validated** (theorem-backed beyond brute force) | own g^{-1} on 36 lane-A rows (36/36), 17 lane-B rows and 12 fresh m = 3 instances (29/29); Rust binary rebuilt, 7/7 CHECK_LIST rows reproduce |
| (c) d_lf = 4, 4, 4, 5 (k = 3..6), 7 (m = 3, k = 2) | **validated**; k = 7, 8 insufficient evidence (not recomputed) | own iterated closure, 15/15 + 1 + 3/3 |
| (d) binary d_full ∈ {k+1, k+2} | **validated** on 5 recomputed systems (k ≤ 6) and as a recount of the 185-row table; k = 7, 8 insufficient evidence | own GF(2) rank, identical rank profiles |

Validator findings, all non-numerical: the branches moved while validation ran (dispositions apply to
the dispatch commits); 33 duplicate rows in lane A's `all_instances.json`; a wording slip in SD1
Corollary 4.3's proof ("≤ 1" for "≥ 1", fixed); lane C's d_full is "first D with the condition"
where SD1 says "least D₀ from which it holds" (equal on the systems checked). pcc claim id: [pcc:PCC-20261115-514b4a] (statement kind; see §4).

## 4. Not done / open

* No Lean statement of SD1 (no Lean toolchain in this container); the chain-to-group-algebra
  bridge it needs exists in `formal/ICFallDegrees/ChainTranslation.lean`.
* c_top ≢ 0 for every nonsingular curve is open (candidate certificate: the 1/t coefficient at t = ∞).
* d_lf at k = 9..14 (prime) is taken from the repository's recorded exact runs, not re-measured here; binary d_full exact only to k = 8 (k = 9 extension running).
* The validator report is unsigned (no keyring key in this session), so a `measurement` claim with a `recompute` proof would stand `scoped`; the claim recorded for this study is `[pcc:PCC-20261115-514b4a]` (kind statement, attestation of `validation/VALIDATOR_REPORT.md`; standing unverified by construction). No leaderboard or benchmarks rows (no cost measurement).
* Research-kb: catalog snapshot (sha256 8bc55122…, revision a340f056…) queried by FTS for "solving
  degree", "static Macaulay", "group algebra inverse": prior entries KR-IC-58213c (CG Thm 3.1) and
  KR-IC-fbdb61; no record of the inverse-support formula. The live MCP server was unreachable.
