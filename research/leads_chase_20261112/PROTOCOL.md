# Protocol: chasing the five top-tier leads of `research/leads_20261112/` (frozen before data)

Date 2026-11-12. Owner's request: "chase those leads, 1 through 5, all of them." The preregistered records are
the ones merged in PR #3956 (`research/leads_20261112/lanes/`); this file fixes the cells, instance rules,
predictions and falsifiers that those records left to the director, before any run. Every number below is
`[pcc:unverified]` until a record names it. Runs are local (4 vCPU, 15 GB; no AWS identity in this session, so
isolab-eks and the S3 checkpoint are unavailable: every launcher carries `CHECKPOINT_SKIP`, recorded in the pcc
ledger per the hook's rule, and raw data are committed to Git with SHA-256 digests in place of S3).

Engines (read-only, by path; commit hashes recorded in the results):
- `ml-cryptanalysis/scripts/ic_bounds_growth_lib.py` (dense exact closure, numpy int64; ≤ about 5,000 columns),
- `ml-cryptanalysis/csd/ic_bounds_big.py` (float64 tiled closure, CPU; used at 21,700 columns),
- `tools/unit_test.py` (this directory): exact unit-ideal test for any prime modulus by the resultant
  `Res_{x1}(S_3, x1^N − 1)` computed in `F_l[x2]/(x2^N − 1)` by Lucas doubling, then `gcd` with `x2^N − 1`.
  Validated here against brute force on rational `μ_N` (p = 65537, 12289, 40961; k = 4..10): gcd degree equals the
  number of distinct `x2` among the zeros in all 12 cases, 0 iff unit.
- `research/quartic_n4v2_20261020/v2.rs` built with `rustc --edition 2021 -O` (binary SHA-256 in the results).

## S1 `LEAD-SYN-char-and-generator-crossing`

Cells and the four hypotheses' predictions are those of the record (synthesis.md, "prediction.table"). Fixed here:

- **Instances mod l ∈ {17, 97, 257}.** Seeded draws `(a, b, t)` from `numpy.random.default_rng(seed)` with
  `seed = sha256("s1-<l>-<i>")[:8]`, i = 1, 2, 3, …; accept iff `4a³ + 27b² ≠ 0`, `t ≠ 0`, all nine coefficient
  classes of `S_3` are nonzero mod l, and the chain ideal is **unit at depth 10** by `unit_test.py` (hence unit at
  every depth ≤ 10). The first four accepted per l are the instances; cells use instances 1–4 (scoring ≥ 3 of 4).
- **Instances mod 12289 (arms a, d).** `y² = x³ + a·x`, a ∈ {1, 2, 3, 5} in that order, `t` the least non-square
  ≥ 2 with `t³ + a t` a nonzero square (unit at every k ≤ 11 by the kernel lemma; re-checked by brute force at
  k = 10). The first four such (a, t) are the instances.
- **Second inputs.** σ_l(g): exponents multiplied by l mod 2^k, coefficients unchanged (equals g^l in F_l[G]).
  C-d1: h = random swap-symmetric element supported on {0,1,2}², seeded `sha256("s1-h-<i>")`, accepted iff
  g and h have no common zero on μ_{1024}² (brute force). C-d2: σ_s(g), s = 1 + 2^(k−2) = 257 at k = 10, accepted
  iff no common zero with g. Negative control d0: h = x1·g.
- **Engines per cell.** Growth lib for D ≤ 4, k ≤ 9 (closure started from the stacked inputs) and for D = 5, k ≤ 7;
  big engine (`--nosplit`, CPU) for D = 5 at k = 8..10, with a subclass that feeds the second input's monomial
  multiples as further generator rows. Arm (a) must reproduce (0, 0, 2, 138, 3163, 18569) at (D, k) = (5, 10) and
  the N8 tables at k ≤ 9 on the same build, else the run is void.
- **Reading.** "1 ∈ V_D(k)" is read from the closure (rank hits the constant column). Law values: LFD = 4 at k = 4, 5;
  5 at k = 6..9; 6 at k = 10. A cell reads "below law" if 1 ∈ V_{law−1}(k) on ≥ 3 of 4 instances.
- **Scored cells**: C-c17 (F_17, {g, σ_17 g}, k = 6, D = 4), C-c97 (F_97, same, k = 6, D = 4), C-c257 (F_257,
  {g, σ_257 g}, k = 10, D = 5), C-d1 (F_12289, {g, h}, k = 10, D = 5), C-d2 (F_12289, {g, σ_257 g}, k = 10, D = 5).
  Reported, not scored: arm (b) one generator mod l at k = 4..10 (D ≤ 5), arm (d) at k = 6.
- **Falsifiers** as in the record: L dies on any scored cell reading "law"; Dm dies if any C-c cell reads "below law";
  F-links dies if C-d1 reads "below law" or a C-c cell reads "law"; S dies if any scored cell reads "below law" while
  arm (a) reproduces the law.

## P1 `LEAD-P-kummer-penultimate-quotient-law`, stages 0–1

- **Stage 0 (desk, done before this file):** the 12 committed cells recomputed from `N8-growth.md` §6.2 and
  `ic-bounds-k14/RESULTS.md` (`tools/p1_stage0.txt`): q/2^k = 2.500, 4.375, 5.250, 5.156 (D = 3, k = 2..5);
  2.953, 5.016, 5.688, 5.365 (D = 4, k = 6..9); 3.058, 5.083, 5.712, 5.374 (D = 5, k = 10..13); 3.067 (D = 6, k = 14).
  Lane P's table reproduced to the digit. Note found while validating `unit_test.py`: the N8 main instance
  (12289, 4021, 9917, 4408) is unit at k ≤ 6 and proper at k = 8, 9 (2 and 16 zeros), so the D = 4 rows at j = 2, 3
  are proper-instance values.
- **Stage 1 (out of sample).** Primes 13313, 15361, 19457 (2^10 | p − 1). Per prime: two unit instances
  `y² = x³ + a x`, a = 1, 2, with t the least non-square ≥ 2 having `t³ + a t` a nonzero square; one proper control:
  random nonsingular curve and random t, seeded `sha256("p1-<p>")`, accepted iff proper at k = 9 (gcd degree > 0).
  Runs: `ic_bounds_growth_ideal.py p a b t 3 2 9` and `… 4 4 9`.
- **Predictions.** Unit instances: dim V_3(k) = 2k + 1 at k = 2..9 and 1 ∈ V_4(k) iff k ≤ 5; q/2^k at the plateau
  cells equal to the committed values within 2% (D = 3, k = 2..5: 2.500, 4.375, 5.250, 5.156; D = 4, k = 6..9:
  2.953, 5.016, 5.688, 5.365). Proper control: the same q/2^k within 5% (the record says instance-independent).
- **Falsifier.** Any plateau cell with q/2^k outside [2.5, 6.0], or more than 5% from the committed value.

## J2 `LEAD-J-dl1-percolation-fb-size`

- **Grid.** `v2 ic <q> <seed> D 3.0 <c> 4 4` (arms D, DL1, DL; `c_S = 3.0, γ = 4, block = 4` frozen as PARAMS.md)
  for c ∈ {1.40, 1.50, 1.60, 1.75, 2.00} × q ∈ {65479 (k 16), 262111 (k 18), 1048507 (k 20)} × seeds 1, 2, 3; then
  q = 4194271 (k 22) × c ∈ {1.60, 1.75, 2.00} × seeds 1, 2, 3. Three `v2 calib <q> <i>` per q; M-eq per record =
  Σ_phases (mul + r_inv·inv + r_lmul·lmul) with the medians (PARAMS.md rule). Success = `ok` on attempt 1 at the
  starting c. The record's giant-component and cycle-rank census needs an instrumentation patch and is **not** run
  here; only the success curve and the paired cost are.
- **Predictions (from the record).** Single-attempt success at c = 1.75 ≥ 0.90 at each k (3 seeds: 3 of 3 expected);
  c_50 ∈ [1.40, 1.75] (success at 1.40 and 1.50 mostly fails, at 1.75 and 2.00 succeeds); paired DL1 cost
  (charged attempts, w = 0) at c = 1.75 over c = 2.00 in [0.70, 0.85] per k.
- **Controls.** c = 2.00 seed 1 at each q must reproduce the committed records' `ok` and attempt count (replay
  property); A/A: a repeated run of one unit gives identical counters.
- **Falsifier.** Success at c = 1.75 below 0.75 at any k; cost ratio upper bound above 0.90; or c = 2.00 not
  reproducing the committed records.

## M1 three-summand arity grid (desk from `ml-cryptanalysis/experiments/ic-bounds-m3-depth/`)

No new run: exact LFD = 7 at k = 5 on 12 instances and LFD ≥ 8 at k = 6 on one (plus the kernel bound
LFD ≤ 3k + 1 and the written proof LFD ≥ 7 for k ≥ 3). The §1 bar for m = 3 asks LFD/k < 0.184 (sparse linear
algebra) or < 0.522 (columns alone). The only extension that would add information is D = 8 at k = 6 (106,762
columns) and k = 7 (401,930 columns): GPU-class, owner's call.

## J1 `LEAD-J-quartic-dl1-generic-jacobian`

Step 1 (read the N4v2 builder, size the adapter) is delegated; the run protocol is written after that report.

## Amendment 2026-11-12 03:55 UTC (before any S1 cell ran): the C-c cells are void; S1 rescoped

Structural fact found while generating instances: over F_l with F_l^× = μ_{l−1} ⊂ μ_N (Fermat primes 17, 257 at
k ≥ 4, 8; and nearly so for 97, whose μ_32 ⊂ F_97^×), every F_l-point of the Semaev curve with nonzero coordinates
is a zero of g on μ_N², so the chain ideal is proper for essentially every instance: 0 unit instances in 60 seeded
draws for each of l = 17, 97, 257 at k = 6 and k = 10 (`tools/unit_test.py`). The record's cells C-c17, C-c97,
C-c257 therefore cannot be run as written (their "1 ∈ V_D" reading is impossible on proper ideals). Primes with
l ≡ 3 mod 4 admit unit instances freely (19: 50/60, 23: 40/60, 43: 50/60, 107: 57/60, 227: 59/60 at k = 10).

Rescoped S1 (frozen now):
- **Arm (b′), scored.** One generator over l ∈ {19, 43, 107, 227}, four unit instances each (rule as above with
  these l), k = 4..9 at D ≤ 4 (D = 5 for k ≤ 7) on the growth lib, and k = 10 at D = 5 on the big engine. Reading:
  "law" if the first D with 1 ∈ V_D equals ⌊(k+14)/4⌋ at every k ≤ 9 and 1 ∉ V_5(10); "below law" if 1 enters
  earlier on ≥ 3 of 4 instances at any k. Prediction under S (characteristic-free): law at every l. Under L or F
  (characteristic below 2^(k+1) matters): a departure somewhere in k = 4..10; the σ_l(g) elements of these l have
  weight ≥ 6 (binary weight of l ≥ 3), so the F-links mechanism has no admissible element at D ≤ 5 and a
  departure would have to come from elsewhere; a "law" reading at every l is evidence for S at these depths and
  pushes any characteristic effect to D ≥ 6 cells (k ≥ 14, GPU-class), which are not run here.
- **Arm (c)** is run only where σ_l(g) has weight ≤ D (recorded, expected skipped at D ≤ 5).
- **Arm (d), scored as in the record** (Dm): C-d1 {g, h} and C-d2 {g, σ_257 g} over F_12289 at k = 10, D = 5 on the
  big engine; the k = 6 versions and the negative control d0 on the growth lib.
- Arm (a) control unchanged.
Everything else in the S1 section stands. Cost: the same.
