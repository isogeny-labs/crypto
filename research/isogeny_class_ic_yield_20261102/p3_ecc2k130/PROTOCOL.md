# P3 protocol: ECC2K-130 isogeny class, IC yield / Dreg (preregistered, predicted null)

Status: DRAFT for the director to freeze (`protocol.json`, hash, commit not yet made). Study: `../DESIGN.md`.
Date of draft: 2026-11-02. Harness: `harness/` (Rust, `p3h`). No Python (AGENTS.md).

## 1. Question and claim classes

Do relation yield, resolving degree (Dreg), and F4 time of the Weil-descended m=3 decomposition system differ between
curves of the ECC2K-130 isogeny class (E0 and the 262 level-263 curves), and at a same-shape Koblitz analogue?

Prediction (class: heuristic estimate): null. The system depends on the curve only through `b`
(`S3 = (a+c)^2 d^2 + acd + (ac)^2 + b`); a random-subspace V makes the instance distribution presentation-free. Expected
differences: none beyond presentation effects of the standard subspace (b = 1 vs dense b).

Claims each outcome would license (class: comparative/statistical, needs controls):
- C1 (null): "within the measured cells, the per-curve Dreg distribution of class curves lies inside the random-b null
  and Frobenius orbits are exchangeable". Licensed only if the positive control detects its planted effect (section 4).
- C2 (presentation): "the E0 vs floor Dreg gap with standard V is a presentation effect" (zero gap with random V).
- C3 (yield): at n=43 yield is not observable (expected count per attempt ~ 2^(3k-n)); only the exact count is reported, and
  the claim is "no curve differs from the Poisson prediction", with the stated power limit.
A null is a recorded result, not an improvement over the IC baseline (CLAUDE.md success criterion). Gaps to matched rho are
not computed here (no end-to-end S); nothing in this study is a speedup claim.

## 2. Instrument (identical on every arm)

`p3h measure`: builds the descended system with 3k (m=3) or 2k (m=2) Boolean unknowns (x_i = sum c_ij v_j) and n F_2 equations
(Semaev S4 via the quadratic resultant, or S3 for m=2), in the Boolean ring (c^2 = c). Metrics:
- `dreg`: resolving degree = first Macaulay degree d <= dmax at which the constant 1 enters the row space (refutation) or all
  occurring variables are pinned (library `solving_degree_from`); `outcome = atleast|capshit` means dreg = dmax+1 is a bound,
  not a value (capshit = F4 size caps, a resource limit).
- `solutions`: exact count of x-triples (m=3) / pairs (m=2) in V with S4/S3 = 0 (n<64: library Artin-Schreier chained count;
  n=131, 3k<=21: numeric). This is the per-attempt relation yield (before lifting to points).
- time `build_s + dreg_s` (single thread, `RAYON_NUM_THREADS=1`), `solve_s` for `--solve`. A clock is evidence only against
  the A/A interval of its own run; counts (dreg, rows, cols, solutions) are the primary metrics.
Validated by `p3h selftest` (smoke/selftest.txt): brute-force evaluation of the system over all 2^3k points equals the
numeric S4 count and (n<64) the library chained count; planted points satisfy all equations; the m=2 system equals the
numeric count; the field isomorphisms between presentations preserve arithmetic.

## 3. Targets and populations

- P3a analogue: prime n in 41..61 with a split conductor prime in [30,600] does NOT exist (computed: for the Koblitz
  trace, n = 41,43,47,53,59,61 have conductor primes {409,1721},{257,8599},{2603047},{68476319},{5783,77761},{1951,587551};
  only 53 and 59 contain split primes, 68476319 and 5783, out of range). Chosen: **n=43, ell=257, a2=0** (prime n; ell
  inert in Q(sqrt-7), (-7/257) = -1, so level 1 has h(O_257)=258=257+1 curves, no horizontal edges and no loops: the
  crater E0 has 258 descending neighbours instead of ECC2K-130's 262 = 263-1). Reported deviation: inert instead of split.
  A split-prime secondary (n=59, ell=5783, 5782 curves, sample only) is out of the first run.
  Level-1 set: class polynomial Phi_257(X, j0=-3375) by CRT, mod 2, factored over F_2 into 6 Frobenius orbits of 43,
  roots in F_{2^43}, **all 258 verified** (exactly one twist has order N = 2^43+1-t, t = 998717; 8 random-j false positives
  expected 0, observed 0). `data/n43_l257_curves.tsv.meta.json`, sha256 in `data/SHA256SUMS`. Modulus z^43+z^6+z^4+z^3+1.
  Cross-check of the count: 258 = h(O_ell); class polynomial route only. An independent Velu route
  (`koblitz_isogeny_class_walk`) is NOT run here (examples cover n<=23); see "not done".
- P3b: ECC2K-130 ground truth, 263 b-values copied from `cryptanalysis/experiments/ecc2k130-isogeny-class/ground_truth/ground_truth.json`
  (sha256 9bb66277207c29cd6274df7f5b230984f8b179b5ce41b4d650ad342feef7e113; `data/ground_truth.json`) to
  `data/ecc2k130_curves.tsv` (b*j = 1 asserted for all 263 in z^131+z^13+z^2+z+1).
- Effective sample size: the Frobenius map is a field automorphism, so the descended system of (b, V, x_R) and of
  (b^2, V^2, x_R^2) are linearly equivalent. Curves in one orbit are exchangeable under a paired random V; independent units
  are orbits: 2 floor orbits + E0 at n=131; 6 + E0 at n=43. Inference unit = orbit, not curve.
- Null population (right null): random b in F_{2^n}^* drawn from derived seeds, same n, same V and x_R as the class curves
  (every b is a valid summation system; all generic binary curves share it). Matched on field, k, V, x_R.

## 4. Arms and controls (same instances across arms, paired; jobs of an attempt shuffled by a seeded Fisher-Yates)

| arm | meaning |
| --- | --- |
| main | curve b, random V, random x_R (shared across all curves of the attempt) |
| null | random b (16 at n=43, 8 at n=131), same V, x_R |
| iso | isomorphic-model control: Frobenius conjugate instance (b^2, V^2, x_R^2); must give identical dreg/solutions/refuted (instrument invariance; expected 100% identical) |
| aa | A/A: the main instance repeated; counts must be identical, time ratio gives the noise floor |
| stdv | presentation control: V = span(1,z,..,z^(k-1)) in the base modulus and in `--presentations` other irreducible moduli (b and x_R mapped by the field isomorphism); the same curve in different polynomial bases. Effect of b = 1's short presentation shows here; minimum over presentations is the P3b headline |
| planted | positive control (detector power): three liftable abscissae drawn from V, x_R = x(P1+P2+P3) on the actual curve; the planted point must satisfy all equations (`planted_ok`), solver must find it (`planted_found`, `--solve`), and the refutation Dreg must NOT refute (outcome differs from main). Expected detection: planted_ok = 1 on 100% and solutions >= 1 |
| m=2 | m=2 control (n=43) with the same arms |

Known effect that the detector must see (positive control for the presentation claim, already seen in smoke): standard V at
n=131, k=6: E0 Dreg 6 vs floor curve A000 Dreg 7, over three presentations.

Seeds: all derived from `--seed 20261102` and (n, m, k, attempt, curve) via `derive`, never chosen after a look.
Blinding: `p3h analyze` hides arm names behind codes unless `--unmask`; unmask only after this protocol is frozen and the
masked output has been produced. Order: interleaved per attempt (shuffled). Host/instrument ledger to record per run: commit,
`rustc -Vv`, binary sha256, host CPU, `RAYON_NUM_THREADS=1`; spot preemption: a restored row keeps its counts, loses its clock
(`wall_valid: false`). Rows are appended per instance, so a preempted shard resumes by `--first-attempt`.

## 5. Cells (k grid) and stopping rule

- P3a-D: n=43, m=3, k = 3,4,5; 259 curves; 20 attempts per k; dmax 9. (m=2 control: k = 3,4,5,8.)
- P3a-Y: n=43, m=3, k=13 only, E0 + 6 orbit representatives, 100 attempts. Expected ordered solution count per attempt
  ~ 2^(3k-n)=2^-4 times a small multiplicity; expect ~6 events per curve per 100 attempts. Power limit stated: per-curve
  yield differences of less than about 3x are not detectable; k=14 (3k=42 ~ n) costs ~300 s per instance (not budgeted).
- P3b: n=131, m=3. Random V: k=3,4,5 (2 attempts, 263 curves). Standard V: k=3..6 in 3 presentations (1 attempt).
  k=7 hits the F4 size cap (capshit at dmax 10) and random-V k=6 did not finish in 11 min on one instance: reported as
  not measured, never as a bound.
- Stopping: fixed N as above; no sequential rule; no change after data.

## 6. Decision rule (per metric, inference unit = orbit)

Metric families: dreg (k = 3,4,5,6 cells), time ratio, solution count. For each (k, V-kind):
1. Instrument checks first: iso identical 100%, A/A counts identical 100%, planted_ok 100%. If any fails the cell is void.
2. Class vs null: for each orbit rep and E0, rank of its mean Dreg among the null curves' mean Dreg (same attempts);
   "unusual" iff two-sided empirical p = 2*min(rank tails)/(N+1) < alpha/m with alpha=0.05, m = #cells x #units tested
   (Bonferroni). Resolution: N+1 = 17 null curves at n=43 gives minimum p = 2/17 = 0.12 > alpha/m: the null set is too small to
   reject at the corrected level; therefore the headline is the effect size (difference in mean Dreg with percentile-bootstrap 95%
   interval over attempts) and an equivalence statement: all |difference| < 0.5 degree (a half of the integer step). Rejection
   is only reported if the count (not rank) differs by at least one whole degree in every attempt, which is then flagged exact
   (Dreg is an integer and the instance is paired, so a constant whole-degree shift in all attempts is a deterministic effect).
3. E0 vs floor with standard V: reported as a presentation effect if the gap vanishes with random V (C2).
4. Times: ratio to the A/A interval of the same run; ranks are only read for ratios outside A/A.
5. Yield: observed counts vs Poisson(lambda_hat) with lambda_hat the pooled mean; per-curve exact Poisson tail, Bonferroni over 7.
What does not count: changing N, k, dmax, metric, null or thresholds after data; any change is a dated amendment, both versions reported.

## 7. Pre-flight runtime (measured 2026-11-02, 4-core x86 VM, one thread; smoke/)

Per instance (random V): n=43: k=3 0.04 s, k=4 0.24 s, k=5 1.0 s (dmax 7); n=131: k=3 0.004 s, k=4 2.4 s, k=5 10 s; k=6 >660 s (unfinished).
Standard V n=131: k=5 0.07 s, k=6 0.15 s (E0, Dreg 6) to 19.8 s (floor curves, Dreg 7), k=7 capshit ~1 s.
Exact count n=43 (library AS count): k=10 2.3 s, k=12 27 s, k=13 ~100 s. dmax 9 raises the Dreg scans for capped cells by an
unmeasured factor (budgeted x3 below).
CPU-hours (single thread, sum; divide by 4 for a 4-core wall clock):
- P3a-D main 259x20x(0.04+0.24+1.0)x3 = ~6 h; P3a controls ~3 h; m=2 ~0.5 h; P3a-Y 7x100x100 s = ~19.5 h.
- P3b main 263x2x(0.004+2.4+10)x3 = ~5.5 h; stdv 263x3x(~20 s at k=6) = ~4.4 h; controls ~2 h.
Total ~41 CPU-h (~11 h wall on 4 cores). P3a-Y is the single largest line and can be dropped (yield is then predicted only).

## 8. Reporting

Table per (cell, V-kind): mean Dreg, sd, histogram, Dreg gap to the null mean (distance remaining is not applicable: no S
or rho gap is produced), solutions, time medians with the A/A band. Export: rows are JSONL, one per instance; the
experiment.export/v1 wrapper and `protocol.json` are for the director. Report-evidence rules: state exactly what was run.

## 9. Known limits (stated up front)

- Instrument is the descended resultant system with the crate's Macaulay linear algebra (F4-style row space over F_2). msolve
  is not installed here, so G5's "msolve max degree" column is not reproduced; native Dreg + `solve_boolean_system` time instead.
- n=131 random-V k>=6 and standard-V k>=7 are out of reach for the Macaulay engine within minutes; the k grid is 3..6.
- Yield at n=43 is not observable below k~13; transported targets along the isogenies are not implemented (a shared random
  (V, x_R) across curves replaces them; planted targets are curve-specific).
- Level-1 set at n=43 comes from the class-polynomial route plus an order check on every curve; no explicit Velu isogeny
  (binary_velu) cross-check was run for n=43.
