# PREREGISTRATION G1-sound-negation-map-rho-baseline

Status: written and frozen before any data of this study. Frozen at the time in `FROZEN_AT.txt`; file hashes in `FROZEN_SHA256.txt`
(scripts, instance files, and read-only copies of the E5 raw cells that are reused). `run_g1.sh` aborts if any frozen file differs.

Directory D = /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G1-sound-negation-map-rho-baseline
E5  = /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk (read only)

Toolchain sanity checks that ran before the freeze (not data, outputs in `dev/`, never read by the analysis): gp 2.15.4 starts; `gen_ext.gp` produced 10
extension curves in 1.6 s; `check_ext_H.gp` (an instance property, not a walk result); one 3-trial smoke run of arm n1 and of arm n1sab at b = 28
instance 1 (to see the scripts execute and the verification path works) and one run of `analyze_g1.gp` on those 3 trials plus the E5 cells
(to see it executes; its numbers are meaningless and are not used). No result of this study exists.

Scope: scratchpad study only (AGENTS.md "Implementation language": all computation in PARI/GP; Python is not used here at all; no Rust crate; no repository
file is touched; no benchmarks/ row, leaderboard change or PR is made). The separation lemma is not used. If the result is later promoted into the repository, that
PR follows the repository rules (research-visuals, benchmarks/, leaderboard, validation gate) separately. Success criterion (CLAUDE.md): the gap to the matched rho is
reported as distance remaining, not as pass/fail; relabelling or accounting changes are not improvements.

---------------------------------------------------------------------------------------------------
## 1. Hypothesis

Background (E5, `E5/RESULTS.md`, `E5/DEVIATIONS.md` items 11-13): the Kummer x-only multiplicative walk was judged "not an improvement". Its preregistered
matched reference A1 (additive rho with negation map, r = 16, 2-cycle test only) stopped solving (114/120 at b28, 9/80 at b32, 0/48 at b36), so the
negation-map comparison (F5) could not be evaluated; the post-hoc a1c (look-back 8) solves to b40 but its step ratio degrades at b >= 36 (E5 summary.txt: R = 2.5 at b36, 5.3 / 8.4 at b40
against A0's 1.3-1.5). The only sound reference left in E5 is A0 (no negation), which understates the gap because the Kummer arms receive the +-
folding for free (E5: K-mix/A0 steps 0.787 against the ideal 1/sqrt2 = 0.707).

G1 hypotheses, all fixed before data:

- G1-H1 (the instrument works). A negation-map additive rho N1 with an r = 32 table, a 2-cycle test plus a Brent saved-point cycle test with escape by
  canonical doubling, distinguished-point trails and a trail cap, solves >= 95% of trials at every b in {24, 28, 32, 36, 40} on the E5 instances, and its steps are ~0.8 of A0's.
  The sabotaged N1 (cycle tests removed) is visibly worse, so the detector can see failure.
- G1-H2 (the gap). K-mix and K-pure are NOT cheaper than the properly matched N1 in counted work at any I/M in {3, 10, 30, 100} when the unpriced native
  work is priced at 0 GAE (S = 0.8 M): the lower 95% confidence limit of cost(K-mix)/cost(N1) is above 1 at every I/M. The ratio shrinks monotonically with I/M
  toward the step ratio K-mix/N1 ~ 0.98 and does not reach 1 inside the band.
- G1-H3 (pricing band, AGENTS.md 5a). Pricing the native work at 0.1 and 1 GAE per operation moves the ratio down for K (K has fewer native operations per step than N1); the order of K-mix and N1 is therefore
  not stable across the weight band at the largest I/M, and by rule 5a that cell is reported "unresolved", not a win.

---------------------------------------------------------------------------------------------------
## 2. Numeric prediction (fixed before data)

Notation: n = prime order of <P>; N1 state space n/2 (negation), R = mean T / sqrt(n/2) with T = total walk steps (failures censored at the budget) until the first verified solve.

| quantity | prediction (band) |
|---|---|
| N1 solve rate, every b in {24..40}, pooled over instances | 1.00 (>= 0.95; no instance below 0.90) |
| steps N1/A0, pooled b = 28..40 (geometric mean over instances) | 0.80 (0.72-0.92). Control band required by the task: [0.71, 0.80] |
| R of N1 (steps / sqrt(n/2)) | 1.55 (1.3-1.8) |
| steps K-mix/N1 | 0.98 (0.85-1.12); steps K-pure/N1 | 1.35 (1.15-1.6) |
| counted M, S, I per step, N1 | M 2.00-2.08, S 1.00-1.05, I 1.00-1.05 (escape = 3M + 2S + 1I) |
| counted M, S, I per step, K-mix and K-pure | 21.5 M, 12.0 S, 1.0 I (E5 measured values; +-3%) |
| N1-sab (no cycle test, r = 32): solve rate | < 0.50 at b32 and b36, ~0 expected; may stay high at b24 |
| N1-c2 (2-cycle test only, r = 32): solve rate | < 0.95 at b36 |
| N1 r = 64 (sensitivity) | solve >= 0.95, steps within +-10% of r = 32 |
| cost K-mix / N1, S = 0.8 M, w = 0 | I/M = 3: 5.6; 10: 3.1; 30: 1.8; 100: 1.25. CI lower limit > 1 at all four |
| cost K-mix / N1, w = 0.1 GAE (native set B) | 4.0; 2.4; 1.5; 1.1 |
| cost K-mix / N1, w = 1 GAE (native set B) | 1.7; 1.25; 1.04; 0.94 |
| cost K-pure / N1, w = 0 | ~7.9; 4.2; 2.5; 1.7 (K-pure above K-mix by ~ the step ratio 1.38) |
| "any I/M gives K-mix a lower CI limit below 1?" | w = 0 and w = 0.1: no. w = 1: only at I/M = 100 (possibly 30); this is a native-op convention artifact and is reported "unresolved" |
| verified improvement (upper CI limit < 1) of K-mix over N1 | none at w = 0 and 0.1 |

(Arithmetic behind the cost rows: K per step 21.5 M + 0.8 * 12.0 S + rho I = 31.1 + rho; N1 per step ~ 2.04 M + 0.82 + 1.02 rho = 2.86 + 1.02 rho; native ops per step: K 2 + ladder
iterations (mean 2.5) = 4.5, N1 5; one GAE = rho + 2.8 M; times the step ratio 0.98.)

---------------------------------------------------------------------------------------------------
## 3. Baselines

- Matched reference: N1 (this study), same curve, same Q, same a-list prefix (E5 seed rule), same DP protocol (DP bits `dpbits(n/2)`, cap 20 * 2^d, budget 40 sqrt(n/2) steps) and same verification as E5.
- A0 (additive rho without negation, r = 16, from E5 raw) is kept as the unmatched reference that E5 used; a1c (E5 post-hoc, look-back 8) is shown only for solve rate and steps, never priced.
- K-pure and K-mix: E5 arms with the SB multiplier set, byte-identical E5 code (`lib.gp`, `run_cell.gp`).
- Rho references are priced with the same 5a band as every other arm.

---------------------------------------------------------------------------------------------------
## 4. Instances and seeds

- E5 instances (reused, `raw/curves_e5.txt`, copy of `E5/raw/curves.txt`): b in {24, 28, 32, 36, 40}, instances 1 and 2. Montgomery curves y^2 = x^3 + A x^2 + x over F_p, p of b+3 bits, #E = h n (h in {4, 8}, n prime of b bits).
  Both E5 arms and N1 run on exactly these curves, points P and a-list (alsd = 20000000 + 100000 b + 10000 inst, same `alist`, same E5 code, so trial t has the same a in every arm).
- Extension instances (`gen_ext.gp`, same `gen_inst` and seed rule setrand(1234567 + 1000 b + inst), instances 3 and 4, b in {24..40}). They exist because instance-level resampling needs more than 2 clusters per b; they are additional, not substitutes.
  All 20 instances have +-H = F_n^* for the SB set (index 1, `raw/ext_H_check.txt`). The b24 instance 4 equals E5's supplementary instance.
- Seeds: table seed 30000000 + 100000 b + 10000 inst (E5 rule; the first 16 N1 table entries coincide with the E5 A0/A1 table); trial seed 40000000 + 100000 b + 10000 inst + 100 armid + TLO
  (armid n1 = 21, n1r64 = 22, n1sab = 23, n1c2 = 24; TLO = first trial of the chunk); bootstrap `setrand(31337)` before every bootstrap.

---------------------------------------------------------------------------------------------------
## 5. Arm N1 (frozen protocol; `g1_lib.gp`, `g1_run_cell.gp`)

- Group E(F_p) in affine Weierstrass-Montgomery form; negation map: canonical point has lift(y) <= (p-1)/2; exponents (alpha, beta) of alpha P + beta Q are negated together with the point.
- r-adding table: r = 32 entries W_j = u_j P + v_j Q, (u_j, v_j) random mod n. Selector j = ((lift(x) >> d) mod 32) + 1, where d = DP bits (x only, therefore negation-invariant). One step: 1 affine add (1 I + 2 M + 1 S); x == x_W with y == y_W is a doubling (3 M + 2 S + 1 I); x == x_W with y != y_W kills the trail.
- Cycle handling (the standard remedies):
  (i) 2-cycle test: new x equals the x one step back;
  (ii) Brent saved-point test: a saved x, replaced each time the step count since the save reaches the next power of two (1, 2, 4, ...); new x equal to the saved one is a cycle of any length <= the current window;
  (iii) escape = replace the current canonical point by its canonical double (exponents doubled, signs fixed; counted 3 M + 2 S + 1 I), reset the Brent state;
  (iv) trails end at a DP (lift(x) mod 2^d = 0), at the cap 20 * 2^d (restart cap), or dead.
- Trail starts: random alpha P + beta Q (uniform), canonicalised; trail start scalar multiplications are by PARI `ellmul` and are unpriced native work (counted as START events).
- DP table: `Map` keyed by lift(x); on a repeat key the candidate a = (alpha1 - alpha2)/(beta2 - beta1) is accepted only if `ellmul(E, P, a)[1] == x(Q)` (verification is not counted).
- Budget 40 sqrt(n/2) steps; exceeding it = failure; T is censored at the budget and failures stay in every mean (conservative against N1).
- Variants: `n1` (r = 32, mode 0); `n1sab` (mode 1: no cycle test at all, the negative control); `n1c2` (mode 2: 2-cycle test only, partial sabotage, E5-A1-like); `n1r64` (r = 64, sensitivity).

K arms (`E5 run_cell.gp`): x-only Montgomery ladder step, 21.5 M + 12 S + 1 I per step measured in E5; K-pure (two-sided trails) and K-mix (mixed starts); multiplier set SB = {2, 3, 5, 7, 11, 13, 17, 19}.

---------------------------------------------------------------------------------------------------
## 6. Parameter grid (trials per instance)

| arm | instances | b24 | b28 | b32 | b36 | b40 |
|---|---|---|---|---|---|---|
| n1 | 1, 2 (E5) | 80 | 80 | 60 | 40 (chunks of 20) | 40 (chunks of 20) |
| n1 | 3, 4 (ext) | 40 | 40 | 30 | 20 | 12 |
| kmix, kpure, a0 | 1, 2 (E5 raw reused, not rerun) | 80 | 60 | 40 | 24 | 12 |
| kmix, kpure, a0 | 3, 4 (ext, run with E5 `run_cell.gp`) | 40 | 40 | 30 | 20 | 12 |
| n1sab, n1c2 | 1 | 30 | 30 | 30 | 20 | - |
| n1r64 | 1, 2 | - | - | - | 20 | 12 |

N1 trials total per b (pooled, all four instances): b24 240, b28 240, b32 180, b36 120, b40 104. Cells are hard-capped at 840 s (`timeout`); a cell killed by the cap
keeps its records written so far and the missing trials are reported as missing, never imputed. Two cells run at a time (4 cores shared with another agent).

---------------------------------------------------------------------------------------------------
## 7. Metrics, controls, falsifiers (analysis = `analyze_g1.gp`, frozen)

Counted work per trial: M, S, I from code counters (K arms: xDBL 3 M + 2 S, xADD 4 M + 2 S, conversion 1 I + 1 M; N1/A0: add 2 M + 1 S + 1 I, double 3 M + 2 S + 1 I), priced as M + 0.8 S + rho I for rho = I/M in {3, 10, 30, 100}
(S = 1.0 M reported as a sensitivity row).

Unpriced native work (AGENTS.md 5a; every arm, the rho references included; lower bound if anything is missing). Counters:
- SEL selector hash, 1 per step (all arms); DPT distinguished-point test, 1 per step (all arms);
- CAN sign canonicalisation, 1 per step (N1 only; x-only K arms have none); CYC cycle comparisons, 2 per step (N1 only);
- LAD one branch per ladder iteration (K arms only; derived exactly from the multiplier histogram; iterations per multiplier 1,1,2,2,3,3,4,4);
- LOOK DP table insert or lookup, 1 per DP (all arms); START trail start, 1 per trail (all arms; its true cost is one scalar multiplication, so 1 per start is itself a lower bound).
NATIVE_A = LOOK + START (the two counters the task names); NATIVE_B = everything above (primary). Priced at w in {0, 0.1, 1} GAE per operation, one GAE = one affine addition = rho + 2 + 0.8 M.
Rows reported: w = 0; w = 0.1 and 1 with set B; w = 0.1 and 1 with set A; no calibrated cost is used. Two arms are ranked only where the order is the same at w = 0, 0.1, 1 (set B) and set A; otherwise "unresolved".

Estimator. Per instance and arm: mean cost over all trials (failures censored). Ratio X/Y per instance, then the geometric mean over instances, pooled over b in {28, 32, 36, 40} (b24 reported separately). Confidence
interval: 95% percentile interval of 2000 two-stage bootstrap resamples (stratified by b: instances drawn with replacement within each b, then trials of X and of Y resampled with replacement within each drawn instance) - this is
instance-level resampling. Headline = all four instances per b; co-primary = E5 instances 1, 2 only. A conclusion counts only if both agree.

Controls.
- PC1 (positive, solve): N1 pooled solve rate >= 0.95 at every b in {24, 28, 32, 36, 40} and >= 0.90 on every instance. Fail = the baseline is not sound; no statement about the gap is made.
- PC2 (positive, step ratio): pooled steps N1/A0 point estimate in [0.71, 0.80] = PASS; in [0.68, 0.85] = MARGINAL (reported as such, not as a pass); otherwise FAIL.
- NC1 (negative, sabotage): `n1sab` and `n1c2` at b in {24, 28, 32, 36}; the detector (Wilson upper limit of the sabotage solve rate < 0.95 and drop >= 0.20 against N1 on the same instances) must fire at some b >= 32 for `n1sab`. If it does not fire, the solve-rate instrument is declared blind and PC1 is not credited.
- NC2 (negative, null ratio): N1 split into odd and even trials; the per-instance step-ratio geometric mean must be within [0.9, 1.1] at every b (descriptive; no CI) - a bias-free pipeline check.
- Independent check of the stats code: K-mix/A0 steps (reference, from E5 data and extension) is expected near the E5 value 0.787.

Falsifier (prediction G1-H2). The prediction is falsified if, at w = 0, for any I/M in {3, 10, 30, 100}, the lower 95% limit of cost(K-mix)/cost(N1) (headline AND co-primary) is below 1. It is "a verified improvement" only if the upper limit is below 1 at every w in the band.
The headline answer to "does any I/M give K-mix a lower-bound CI below 1" is read directly from `out/summary.txt` section 4 and reported per w; the K-mix lower limit below 1 at w = 1 alone is reported as "unresolved by rule 5a".
G1-H1 is falsified if PC1 fails, if PC2 is FAIL, or if NC1 does not fire. G1-H3 is falsified if K-mix/N1 at I/M = 100 stays above 1 (lower limit) at w = 1 with set B.

Reporting rules. Every number comes from `out/summary.txt`, `out/solve.csv`, `out/ratios.csv` or the raw files; missing or failed cells are stated as such. No claim about the repository's index-calculus baseline is made. The gap is given as the ratio K-mix/N1 and K-pure/N1 (distance remaining).

---------------------------------------------------------------------------------------------------
## 8. Exact commands

```
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G1-sound-negation-map-rho-baseline
cd $D
# already done before the freeze (instances, no walk data):
#   gp -q -f gen_ext.gp </dev/null > raw/curves_ext.txt ; cat raw/curves_e5.txt raw/curves_ext.txt > raw/curves.txt
#   gp -q -f check_ext_H.gp </dev/null > raw/ext_H_check.txt
sha256sum -c FROZEN_SHA256.txt                 # must report OK for every file
bash run_g1.sh                                  # stage A (controls, r=64), B (N1 cells), C (ext K/A0 cells), then analyze_g1.gp; launch in the background
# one cell by hand, e.g. N1 at b = 36 instance 1 trials 1..20 of 40:
#   ARM=n1 B=36 INST=1 TLO=1 THI=20 TOTAL=40 timeout 840 gp -q -f g1_run_cell.gp </dev/null > raw/cell_n1_b36_i1_c1.txt
# analysis only:
#   : > out/solve.csv ; : > out/ratios.csv ; gp -q -f analyze_g1.gp </dev/null > out/summary.txt
```
Outputs: `raw/cell_*.txt` (one record per trial), `out/RC.txt` (cell exit codes), `out/solve.csv`, `out/ratios.csv`, `out/summary.txt`, `out/FILES.txt`. Disk: raw files are small (a few MB); no build directory exists.
Expected wall time with two cells in parallel: 1.5 to 2.5 hours in total; each cell is capped at 14 minutes. Any change to a frozen file after this point is logged in `DEVIATIONS.md` and the study says so.
