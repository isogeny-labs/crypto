# L4-lifting adversarial check (Red Team, 2026-11-13)

Standing: every number here is `[pcc:unverified]`. No pcc claim exists for any item. research-kb was
unavailable; cited theorems below are checked from memory or numerically only, as marked.

## 0. Evidence reviewed and its durability

- Lab dir: `chase/L4-lifting/` under
  `/tmp/claude-0/-home-user/56e3c4bf-6e3c-5268-b3a1-8d397ec01ddb/scratchpad/`. Files: `PROOF.md`,
  `PROTOCOL.md`, `AMENDMENTS.md`, `REPORT.md`, `code/`, `out/`.
- `sha256sum -c MANIFEST.sha256`: 38/38 OK. `PROTOCOL.sha256` matches `PROTOCOL.md` (33da07ed…).
- **Durability gap.** These files live only on a session scratchpad. They are not committed and not in S3.
  For durable evidence they would need copying into the repo, or into S3 by SHA-256. I reviewed them
  frozen by the manifest hashes only.
- **Missing comparator.** The closure note's Prop 3 and Theorem 2 texts (`$SCR/finish/lifting/` of
  session ad79db08) no longer exist. "Closes Prop 3" can therefore be checked only against the closure
  note's one-line summary (`research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md` §5.2).
- I did not edit any lab file. Re-runs used copies in `chase/L4-check/rerun/`.
- Importing the lab's modules created a `code/__pycache__/` directory inside the lab. I deleted it; the lab's
  MANIFEST still verifies 38/38.

## 1. Verdicts per item

| Item | Verdict | One-line reason |
|---|---|---|
| Lemma 1 (exact 1/q) | sound | `u_i = s_i + t_i x` are i.i.d. uniform. |
| Lemma 2 (K-relations reduce) | sound given H1 | Over degree-d fields H1 follows from Merel–Oesterlé only for `d < log_3 q` (about 161 at 256 bits). |
| Lemma 3 (Cramer–Hadamard–Hermite) | sound | Works in `E'(K)/tors` with a basis of Λ itself, so torsion and non-saturation are handled. Also `1+s/4 ≥` the Minkowski bound for every s ≤ 3000 (checked numerically). |
| Theorem X | sound with gaps | The core statement is correct. Remark (b) has a proof gap that can be fixed. Remark (a) needs the scope restriction "one lifted point per mask per relation". |
| Corollary X1, S* numbers | sound (reproduced) | One wording error: the unconditional ρ₀ excludes only S ≤ 1, not "nothing beyond S = 2". |
| Theorem F, r* numbers | sound (reproduced) | The "ρ" in F is `B/λ(E'(K))`, a different quantity from X's span ratio, yet it is tabulated on the same grid. |
| 256-bit margin arithmetic | sound (reproduced exactly) | 89.4/66.3, 64.7/34.9, 8.3/−4.8 bits; ρ* = 11.04/5.04/1.92. |
| (ii) verdict "rank 7, 8 excluded at measured ρ" | unsupported | Wrong population, a tail of 2^−128 is required, and a post-hoc selection filter was applied (F1, F5). |
| (iii) "Theorem X closes the Prop 3 gaps" | partly: unsupported as stated | The conditional form does extend. H1 range, the unconditional ρ₀ step over K, and Galois-conjugate lifts are open (F2). |
| (iv) "fixed factor-base points: Thm X" | sound with gap | The masked coefficient of the Cramer relation is not guaranteed as written (F3). |
| Toy KILL | sound per frozen rule (reproduced) | Scope is narrow: ±2 box, no rank-lowering conditions (F8). |

## 2. Findings, by severity

**F1 (high; unsupported). The (ii) verdicts read the measured ρ distribution as if it bounded the ρ of
successful lifts.**
- **Where:** `REPORT.md` "Verdicts" (ii); `PROTOCOL.md` Q2 rule; `out/rho_summary.txt`.
- **What Theorem X gives:** `Pr[success] ≤ |M(ρ₀)|/q + Pr_attacker[ρ > ρ₀]`.
- **Why the data cannot carry the verdict:**
  - Excluding rank 7 at 256 bits needs `Pr[ρ > 11.0] ≲ 2^−128` under the attacker's lift distribution.
  - That distribution is adaptive: Remark (a) lets the attacker try unboundedly many lifts per mask and keep any successful one.
  - By Theorem X itself, successful configurations sit at `ρ > ρ*` except with probability ≤ 2^−128. Conditioning on success moves ρ into exactly the tail that was never sampled.
- **What the measurements are:** n = 147 / 44 / 14 Mestre configurations, none of them successful, plus 50 independent 6-point lifts. With zero exceedances, the rule of three bounds `Pr[ρ > ρ*]` at about 2.0 % / 6.8 % / 21 % at best. That is 120+ bits short of what is needed.
- **Which numbers survive:** the thresholds ρ*(s). They come from the bound alone and need no data. "Excluded at measured ρ" is not a statement about xedni.
- **A side note:** REPORT calls the Mestre configurations "independent". They are dependent (`ΣP_i = O`, k = s+1 points of rank s). Only the `lift_rho` sample is independent.
- **Cheapest test:** restate (ii) as "per-trial success ≥ 2^−128 at span rank 7/8/9 (k = s+1) requires ρ > 11.0 / 5.04 / 1.92". Then, at 8–12-bit toy size where honest useful relations occur, compare ρ_G of the successful configurations with the unconditioned ones. That shows directly whether success shifts ρ.

**F2 (high/medium; unsupported). "Theorem X closes the Prop 3 (number-field / Q-curve) gaps" overstates.**
What is settled:
- Theorem X's conditional form uses only ratios of ĥ on one curve, a homomorphic `red` and an injective `ι`, so normalisation and residue degree drop out. That part is sound.

What remains open:
- (a) **H1 for degree d.** Merel–Oesterlé (`ℓ ≤ (3^{d/2}+1)^2`, cited) guarantees `q ∤ #tors` only for `d < log_3 q`. Above that, Lemma 2 has no proof, so "independent of d" holds only on that range.
- (b) **The unconditional ρ₀ ≤ poly(dβ) step (§2, §4).** It uses `σ ≤ log|Δ_min|/log 11`, which is a fact about Q (conductor ≥ 11). Over K the conductor norm can be 1: curves with everywhere good reduction exist, for example over some real quadratic fields. Then Petsche's bound is vacuous (`log|NΔ| = 0`). No unconditional λ lower bound over K is cited.
- (c) **Galois conjugates.** Over an inert 𝔭 with E' σ-stable, `σP_i` is a second lift of the same `ι R_i` that comes for free. Relations mixing `P_i` and `σP_i` lie outside the success definition (one point per mask), and Cramer can return trivial collapsed relations such as `P_i − σP_i`.
- (d) Prop 3's own text is gone, so nothing can be checked against it.

Cheapest test: write the K-version of the ρ₀ bound with a cited lower bound valid for everywhere-good-reduction curves (Masser/David type), or restrict the claim to "Q and K with a nontrivial conductor, d < log_3 q, one point per mask".

**F3 (medium; unsupported as written, fixable). Theorem X Remark (b), the mixed fixed factor-base form behind (iv).**
- **Where:** `PROOF.md` §2 Remark (b).
- **The gap:** the proof picks an arbitrary maximal independent set I and an arbitrary `j ∉ I`. If the fixed-point lifts `P'_j` are dependent among themselves, Lemma 3 may return a relation supported only on fixed points. Its event `ΣmF = 0` is deterministic, not 1/q, and the union bound fails.
- **The fix:**
  - Take I to contain a maximal independent subset of the fixed points, and take j masked. Such a j exists whenever a useful relation exists.
  - Then `m'_j = ±det A_{−j} ≠ 0`, and `|m'_j| < q`.
  - The proof must say this. Until it does, (iv) "fixed factor-base points: Thm X" is a sketch.

Cheapest test: a two-line amendment of the proof. No data is needed.

**F4 (medium; scope). Remark (a) "any number of lifts per mask" holds only for one lifted point per mask in the relation.**
- The success definition has one `P_i` per `R_i`. An attacker who puts several points reducing to the same `ι R_i` on one curve is not covered: by F2(c) such points arise automatically from conjugates, and in general they differ by elements of `ker red`.
- That attacker is exactly the bridge to the Model-F hybrids that §3 Remark F3 leaves open. The statement and REPORT should name this boundary.

**F5 (medium; protocol). Unreported control failure, and a post-hoc filter that biases ρ downward.**
- **(a) Control failure.** `PROTOCOL.md` requires the height control `< 1e-3` relative.
  - In `out/mestre_small.jsonl` 2/40 curves fail (max 2.46e-3); in `mestre_small2.jsonl` 7/150 fail (max 2.18e-3).
  - These two files supply all the rank-8/9 data and the rank-7 worst case. REPORT does not mention the failure.
  - The numerical effect on the margins is small: error-corrected worst-case margins move by ≤ 1.3 bits.
- **(b) Post-hoc tolerance change.** AMENDMENTS 1 tightened the extra-point independence tolerance to 1e-3 after a partial run showed ρ ≈ 200–550. That run was discarded and not preserved.
  - The exact SVP-torsion check alone already catches genuine rank deficiency.
  - The extra 1e-3 eigenvalue filter also removes genuinely independent extras with small covolume. Through Hermite's bound, those are the large-ρ cases.
  - 26/62 (s = 8) and 10/21 (s = 9) cases were dropped as degenerate in `small2`.
- **Effect:** the "worst measured ρ" that the frozen rule reads is biased low, by an amount nobody has quantified.
- **Cheapest test:** rerun `mestre.py` on seed 12 with the extra-point tolerance at 1e-4 plus the exact torsion check, and report the max ρ and the drop counts.

**F6 (low/medium; unpriced attacker freedom). The margin table fixes k = s+1.** Theorem X allows any k, and
xedni can fit up to 9 points. Exact recount (`L4-check/rerun/rederive.txt`):
- ρ*(7) = 10.20 at k = 9 and 9.63 at k = 10 (vs 11.04);
- ρ*(8) = 4.73 at k = 10 (vs 5.04);
- ρ*(9) = 1.82 at k = 11 (vs 1.92).

Per-trial cost is priced at one rho step. That favours the attacker, so it is conservative for exclusion statements.

**F7 (low; wrong wording). Three misstatements:**
- **"Excludes nothing beyond S = 2" (PROOF §2) / "beyond rank 2" (REPORT).** At ρ₀ = 7.8e34 the margins are S = 1: +11.1, S = 2: −222.4. So only S ≤ 1 is excluded.
- **§4 table "needs rank ≥ S*_X − 1".** S*_X is the first non-excluded span rank, so the requirement is rank ≥ S*_X. The stated version errs in the conservative direction.
- **`out/bounds_extra.txt` vs PROOF §4.** `bounds_extra.txt` computes the rank-20 Model-F line with T = 1 (count 124.0 bits, so it does not pass), while PROOF §4's "ρ ≳ 1.3·10³" uses T = 16 (124 + 4 = 128). With T = 1 the threshold is ρ ≈ 1.7·10³.

**F8 (low; scope). Toy KILL.** It reproduces exactly and is valid under the frozen rule (honest 0 of 2,700
trials vs 1,359.0 expected from Arm B; planted 178/178 and 180/180 ×4). Its limits:
- The honest arm is the LLL-smallest plane cubic. It has no Silverman a_ℓ conditions, which are the xedni device that pushes rank down.
- Detection covers the box ±2 only.
- The rank of the span is never measured.

So the KILL says that random cubic fits are almost never dependent with small coefficients. It says nothing about lifts engineered to be dependent. Theorem X already implies the H ≤ B ordering the toy tests.

**F9 (low; conservative, not wrong). Hermite constant for s ≥ 9.**
- `γ_s ≤ 1+s/4` is rigorous: it is at least Minkowski's bound for all s ≤ 3000 (checked), and asymptotically 2s/(πe) < s/4.
- Blichfeldt's classical bound `γ_9 ≤ 2.2406` already gives a worst-case rank-9 margin of +19.3 and ρ*(9) = 2.79.
- Since `γ_9 ≥ 2` (Λ_9), no Hermite-type argument can push ρ*(9) above 3.13. Cohn–Elkies is not needed for the "rank 9" flip. Any flip is still subject to F1.

**F10 (low; unsupported remark). "Essentially best possible for arguments that use only heights" (PROOF §5) is informal.** For span rank s < k−1, all k−s independent K-relations must reduce. That event has probability about q^{−(k−s)}, so the s < k−1 terms can be sharpened. The dominant term s = k−1 is unaffected.

## 3. Line-by-line notes

**Lemma 1.** `R_i = u_i P`, `u_i = s_i + t_i x`; `s_i` uniform makes `u_i` uniform, independent of `t_i` and x. Then `Σ m_i u_i` is uniform when some `m_j ≢ 0 (mod q)`. Sound. The bound holds even if the algorithm sees the masks, because the union-bound events depend only on the masks.

**Lemma 2.**
- `red(Σ m_i P_i) = Σ m_i ι R_i = ι(Σ m_i R_i)`, which has order dividing gcd(#tors, q) = 1, so it is 0, and ι is injective. Sound.
- Direction: a K-relation implies a mod-p relation. This is the direction the bound needs.
- It needs only `E'(K)[q] = 0` and good reduction at 𝔭. Mazur gives q > 7 for K = Q.

**Lemma 3.**
- Let A be the s×(s+1) coordinate matrix in a Z-basis of Λ = span of the s+1 points in `E'(K)/tors`.
- The cofactor vector `m_j = (−1)^j det A_{−j}` is nonzero (rank s) and satisfies `A m = 0`, so `Σ m_i P_i ∈ tors`.
- `|det A_{−j}| = vol/covol(Λ)`; Hadamard gives `vol ≤ H^{s/2}`; Hermite gives `covol ≥ (λ(Λ)/γ_s)^{s/2}`.
- Non-saturated Λ is fine because its own basis and minimum are used, and `λ(Λ) ≥ λ(span of all k)`. Positive definiteness of ĥ on `E'(K)⊗R` is standard (cited).

**Theorem X.**
- On success, m ≠ 0, so the span rank s < k. Take a maximal independent I and `j ∉ I`.
- The span of `I ∪ {j}` has rank s, minimum ≥ λ(span), and max height ≤ H. So `|m'|_∞ ≤ (γ_s ρ₀)^{s/2} < q`, hence `m' ≢ 0 (mod q)`.
- 𝓜 is a fixed set, so the union bound gives `|𝓜|/q` for any algorithm and any number of lifts.
- Counting: `((2C+1)^{s+1} − 1)/2` per support, `C(k,s+1)` supports; overcounting is harmless. The s = 0 term (a torsion lift point) is handled, since `R_j = 0` occurs with probability 1/q.
- Sound, with the remarks' gaps F3, F4, F10.

**Theorem F.** Distinct classes mod torsion are √λ apart, so balls of radius √λ/2 are disjoint inside radius √B + √λ/2. That gives `#S_B ≤ T(1+2√(B/λ))^r`, and R is uniform and independent of 𝓛. Sound. No H1 is needed, because T is counted. Remark F2 (BSGS, ≥ 2√q) is correct. F3 (Shoup) is cited and not checked here.

**ρ consistency.**
- **Theorem X:** ρ = max_i ĥ(P_i) / min ĥ on span⟨P_1..P_k⟩/tors.
- **`mestre.py`:** λ_G is taken by SVP on the Gram matrix of `gens`, whose span equals the span of `allpts` (P8 = −ΣP1..7). H is the max over `allpts`. This matches the theorem's ρ.
- **Model F:** uses B/λ(E'(K)), a different ratio.
- **Unconditional ρ₀:** uses λ(E'(K)) ≤ λ(span), which is the valid direction.
- **Normalisation:** Silverman's ½h, which cancels in ratios.

## 4. Re-runs (single core, prlimit 3 GB, timeout ≤ 1200 s)

| Command | Result |
|---|---|
| `python3 code/bounds.py` | byte-identical to `out/bounds.txt` (78 lines) |
| `python3 code/summarize_toy.py` on copied `out/toy/*.jsonl` | `summary.txt` and `summary.json` identical |
| `python3 code/summarize_rho.py` on copied `out/*.jsonl` | `rho_summary.txt` and `rho_summary.json` identical |
| independent `L4-check/code/rederive.py` (exact integer \|M\|, mpmath floors) | see table below |
| `L4-check/code/spot_heights.py` (ĥ with 6 vs 8 doublings, top-ρ rank-7 configs) | see the spot-check notes below |

Spot-check results:
- The two largest rank-7 ρ values were recomputed: stored 2.1376 and 2.0402; with 6 doublings 2.1362 and 2.0423; with 8 doublings 2.1349 and 2.0429.
- The parallelogram law holds at 8 doublings (relative error 5e-6 and 2e-5).
- The height code and the SVP step are sound to about 1e-3 in ρ.
- The stored `maxerr` (1.3e-2) suggests the run used 5 doublings, which the protocol allows (n ≥ 5).
- The Q2 command lines (seed, doublings, search box) are not recorded anywhere in the lab: there is no Q2 run script and no command line in the logs. This is a reproducibility gap.

Independent margin recount (`L4-check/rerun/rederive.txt`; 128 − log2|M|, k = s+1):

| s | n | ρ median / max | margin @median | margin @max | ρ* (1+s/4) | ρ* (Blichfeldt γ_9) |
|---|---|---|---|---|---|---|
| 7 | 147 (small) | 1.197 / 2.138 | +89.4 | +66.3 | 11.04 | same |
| 8 | 44 | 1.451 / 2.571 | +64.7 | +34.9 | 5.04 | same |
| 9 | 14 | 1.692 / 2.071 | +8.3 | −4.8 | 1.92 | 2.79 (margins +32.5 / +19.3) |

S*_X at 256 bits is 10/8/6/4 and at 2048 bits 24/19/15/12. r*_F at 256 bits is 70/44/29/17. Petsche ρ₀ at
β = 256 is 7.80e34. Toy pooled per-trial bound: 4.76e-4 at 14–20 bits (n = 6,300) and 1.43e-3 at 20 bits.
All match the lab.

## 5. Strongest correct statement

In the fresh-mask model, assume q ∤ #E'(K)_tors and one lifted point per mask in the relation. Then for
any lift, over any number field (with H1 guaranteed for d < log_3 q), any residue degree and any
normalisation:

`Pr[useful K-relation whose span has rank ≤ S and ρ ≤ ρ₀] ≤ |𝓜(k,S,ρ₀)|/q`.

Consequences at 256 bits with k = s+1:
- A per-trial rate ≥ 2^−128 requires ρ > 11.0 (s = 7), ρ > 5.04 (s = 8), or ρ > 1.92 (s = 9; 2.79 with Blichfeldt's γ_9), or span rank ≥ 10 at ρ ≤ 1.26.
- Unconditionally over Q (Petsche + Silverman, recalled) only S ≤ 1 is excluded at 256 bits.
- Asymptotically, for polynomial-size lifts over Q, rank ≤ Θ((log p/log log p)^{1/2}) is excluded.

Theorem F gives the fixed-lift count `L·T(1+2√(B/λ))^r/q`.

Not shown:
- the ρ of successful lifts;
- any exclusion of rank 7 or 8 for actual xedni;
- the unconditional number-field case;
- hybrids, multi-lift relations and decoders.
