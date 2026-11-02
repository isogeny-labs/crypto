# P3 protocol: ECC2K-130 isogeny class, IC yield / Dreg (preregistered, predicted null)

Status: DRAFT for the director to freeze (`protocol.json`, hash, commit not yet made). Study: `../DESIGN.md`.
Date of draft: 2026-11-02. Revision 2 (same day, before any data of the full run): C2 redefined as a minimum over a declared
presentation family including b-adapted presentations (arms `badapt`, `bsparse`, `nbadapt`, sections 1, 4, 6), the n=43 cost
estimate corrected (section 7). Revision 1 (C2 = "gap vanishes with random V") is withdrawn, see section 1. Harness: `harness/` (Rust, `p3h`). No Python (AGENTS.md).

## 1. Question and claim classes

Do relation yield, resolving degree (Dreg), and F4 time of the Weil-descended m=3 decomposition system differ between
curves of the ECC2K-130 isogeny class (E0 and the 262 level-263 curves), and at a same-shape Koblitz analogue?

Prediction (class: heuristic estimate): null. The system depends on the curve only through `b`
(`S3 = (a+c)^2 d^2 + acd + (ac)^2 + b`); a random-subspace V makes the instance distribution presentation-free. Expected
differences: none beyond presentation effects of the standard subspace (b = 1 vs dense b).

Claims each outcome would license (class: comparative/statistical, needs controls):
- C1 (null): "within the measured cells, the per-curve Dreg distribution of class curves lies inside the random-b null
  and Frobenius orbits are exchangeable". Licensed only if the positive control detects its planted effect (section 4).
- C2 (presentation, family-minimised): "over the declared presentation family F (section 4: the standard V in several moduli,
  and V = span(1,g,..,g^(k-1)) in the b-adapted presentations, where the modulus is the minimal polynomial of g and b is
  mapped to its image), the minimum over F of a floor curve's Dreg and F4 time equals E0's minimum over the same family F,
  with E0 minimised by the same procedure". This replaces revision 1 ("zero gap with random V"), which could not test the
  claim: with a random V the presentation is not adapted to b, a floor curve's b stays dense in every random modulus, while
  b = 1 is fixed by every field isomorphism and is the first basis vector of every power basis. An attacker chooses the
  presentation (min over presentations is what is paid, as in the prior d_reg 8 vs 10-11 refutation, `cryptanalysis/
  experiments/ecc2k130-isogeny-class/REPORT.md` section 8 and `gaps/G5-*`: V = span{b^j} and span{(b+1)^j}), so both sides are
  minimised over F. Licensed only if the instrument checks pass and the positive control for the presentation effect is
  seen (standard V, n=131, k=6, E0 Dreg 6 vs floor Dreg 7).
- C2' (what a surviving gap would mean): if a gap survives the minimum over F, in Dreg by at least one whole degree in every
  attempt or in F4 time outside the A/A band, it is attributed to **b in F_2** (E0 is the only curve of the class with b in
  F_2, i.e. the Koblitz structure: b fixed by every field isomorphism and by Frobenius), not to the isogeny class or the
  level: the descended system depends on the curve only through b, so a floor curve is, to this instrument, a curve with a
  generic b of its orbit. That reading is checked by `nbadapt` (random dense b with the same b-adapted presentation must
  give the floor curves' values, so "floor = generic" holds). C2' is a structural statement about E0 and says nothing about
  the 262 level-263 curves, which stay at the generic cost; it is not a speedup (no end-to-end S, no gap to rho). "No gap
  inside F" is not "no presentation achieves a gap": F is finite and declared.
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
| badapt | b-adapted presentation. Floor curve: g = b, modulus M_g = minimal polynomial of b over F_2 (degree n: n is prime and b is not in F_2), so b maps to z (weight 1, degree 1). V = span(1,g,..,g^(k-1)), i.e. the standard V of that presentation; x_R and b are mapped by the field isomorphism F_{M0} -> F_{M_g} (change of basis to the powers of g, computed by Gauss-Jordan on the n x n matrix of g^i). This is the presentation of the prior work's `powerb` cell (V = span{b^j}). E0: b = 1 is fixed by every isomorphism and its minimal polynomial z+1 has degree 1, so there is no adapted modulus; E0 uses the borrowed generators g = b of the first curve of each of the two (n=131) or first two (n=43) floor orbits, so that E0 is measured in exactly the presentations the first floor curves use. |
| bsparse | the b-sparse family F_b, 6 presentations per curve, each a generator g of F_{2^n} (any g outside F_2 generates it, n prime): g in {b, b+1, 1/b (= j, since b*j = 1), b^3, b^(1/3) (exponent (2^(n+1)-1)/3, so b = g^3), b^2+b}. Recorded per row: the image of b in the power basis of g (Hamming weight `b_w`, degree `b_deg`), the modulus weight `mg_w`, and `sel` = 1 on the lexicographic minimum of (weight, degree) over F_b (g = b: (1,1); g = b+1: (2,1); g = b^(1/3): (1,3); the others are dense, weight 64-75 / degree 129-130 at n=131). E0: F = {z, the 2 borrowed generators, 3 seeded random generators}, b image 1 (weight 1, degree 0) in all. Dreg and time are then minimised over the family (analysis section "presentation-family minima"). The conjugates b^(2^i) give the same modulus (same minimal polynomial) and b -> z, so they are not extra presentations. |
| nbadapt | control for C2': random dense b (the null population, 8 curves) with g = b, same V = span(1,z,..) in M_b. Must reproduce the floor curves' badapt values; if it does not, "a floor curve is a generic dense-b curve" fails and C2' cannot be read. |
| planted | positive control (detector power): three liftable abscissae drawn from V, x_R = x(P1+P2+P3) on the actual curve; the planted point must satisfy all equations (`planted_ok`), solver must find it (`planted_found`, `--solve`), and the refutation Dreg must NOT refute (outcome differs from main). Expected detection: planted_ok = 1 on 100% and solutions >= 1 |
| m=2 | m=2 control (n=43) with the same arms |

Known effect that the detector must see (positive control for the presentation claim, already seen in smoke): standard V at
n=131, k=6: E0 Dreg 6 vs floor curves Dreg 7, in the base modulus and two other moduli (4 floor curves x 3 presentations).
Smoke of the b-adapted arms at the same cell (`smoke/p3b_bp.*`, section 7a): the floor curves reach Dreg 6 in the presentations
where b is a power of g (b = g, g^3, g^(-1)) or g + 1, and Dreg 7 in the dense-image presentations (b^3, b^2+b); E0 is 6 in all.
Presentation properties that decide how the family is read: (i) at n=131, k=6 the Dreg of a floor curve follows whether b
is a Laurent monomial or binomial in g, not the Hamming weight of the image: g = 1/b has b = g^(-1) with a dense image
(weight 64-66) and still Dreg 6, so `sel` (minimum weight) is recorded but the family minimum is what is analysed; (ii) the
modulus of a b-adapted presentation is dense (weight `mg_w` 59-75) whereas E0 in the base modulus has weight 5, which makes
the system build (`build_s`, field multiplications in the modulus) several times cheaper for E0 in z; the F4 time `dreg_s`
does not depend on the modulus weight in the smoke. Time comparisons for C2 therefore use `dreg_s` (and `build_s` only
between presentations of equal `mg_w`); both are reported.

Sanity check of the isomorphism (`p3h isocheck`, output `smoke/isocheck.txt`): for random b at n = 13, 17, 19, 43, 131 and the
first four members of F_b, (a) M_g is irreducible, (b) the map is a ring isomorphism (product and sum of random elements),
(c) g maps to z, (d) the exact ordered solution count of the m=3 system (chained Artin-Schreier count for n < 64, numeric
for n=131) with V_g = span(1,g,..) in the base field equals the count in the g-presentation with the standard V, for a random x_R
and for a planted x_R (count >= 1; the planted point, which has the same coefficient vector, satisfies every equation of the
mapped system), k = 3..8; and (e) the same for the m=2 count at n=13, k=7. Any FAIL voids the b-adapted arms.

Seeds: all derived from `--seed 20261102` and (n, m, k, attempt, curve) via `derive`, never chosen after a look.
Blinding: `p3h analyze` hides arm names behind codes unless `--unmask`; unmask only after this protocol is frozen and the
masked output has been produced. Order: interleaved per attempt (shuffled). Host/instrument ledger to record per run: commit,
`rustc -Vv`, binary sha256, host CPU, `RAYON_NUM_THREADS=1`; spot preemption: a restored row keeps its counts, loses its clock
(`wall_valid: false`). Rows are appended per instance, so a preempted shard resumes by `--first-attempt`.

## 5. Cells (k grid) and stopping rule

dmax is set per cell from the measured cost (section 7): Dreg beyond the first resolving degree costs a factor ~100 per
degree at these sizes, so a cell whose Dreg exceeds dmax is reported as a bound "Dreg > dmax" (`atleast`) with its matrix shape
(rows, cols), never as a value. Revision 1 specified dmax 9 everywhere; at dmax 9 the n=43 k=5 and n=131 random-V k=5 cells run
for 100 s to more than 15 min per instance and return capshit / no answer.

- P3a-D: n=43, m=3, 259 curves. k=3: 20 attempts, dmax 9. k=4: 2 attempts, dmax 8 (all measured arms and curves resolve at 8,
  about 17 s). k=5: 2 attempts, dmax 7 (`atleast`: smoke shows Dreg > 8, indeed > 9 with capshit at 70-110 s; the cell
  only certifies that the matrix shape is identical across curves). m=2 control: k = 3,4,5,8 (cheap, dmax 9).
- P3a-C (controls and the C2 family, CTRL = 10 curves: E0 + one per orbit and more): arms main, iso, aa, planted, stdv
  (3 other presentations), badapt, bsparse, null 16 (+ nbadapt on the same 16). k=3: 20 attempts with `--solve`, dmax 9.
  k=4: 2 attempts, dmax 8, no `--solve`.
- P3a-Y: n=43, m=3, k=13 only, E0 + 6 orbit representatives, 100 attempts. Expected ordered solution count per attempt
  ~ 2^(3k-n)=2^-4 times a small multiplicity; expect ~6 events per curve per 100 attempts. Power limit stated: per-curve
  yield differences of less than about 3x are not detectable; k=14 (3k=42 ~ n) costs ~300 s per instance (not budgeted).
- P3b: n=131, m=3. Random V (263 curves, 2 attempts): k=3,4 at dmax 9 (0.01 s, 2.6 s), k=5 at dmax 7 (`atleast` 8, 10 s).
  Standard V: k=3..6 in 3 presentations (1 attempt, 263 curves, dmax 9; k=6 is the 19 s cell). b-adapted: `badapt` k=3..6 on
  all 263 curves + E0, 2 attempts (0.9 s per curve-attempt in total); `bsparse` + `nbadapt` k=5,6 on CTRL131 (E0 + 6 floor
  curves, null 8), 2 attempts. Controls iso, aa, planted (+`--solve`) with `main` in the same file, k=3,4,5, dmax 7.
  k=7 hits the F4 size cap (capshit at dmax 10) and random-V k=6 did not finish in 11 min on one instance: reported as
  not measured, never as a bound. b-adapted k=7 is not planned (not measured here).
- The control lines of revision 1 listed `iso,aa,planted,...` without `main`; the iso/A-A identity checks in `analyze` compare
  against `main` rows of the same file, so they read 0/0. `main` is now in every control line.
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
3. E0 vs floor, C2 (family minima; this replaces "gap vanishes with random V"). Per cell (n, k), curve c and attempt a:
   D(c,a) = min over the presentations of F_c of Dreg (arms stdv, badapt, bsparse pooled; F_E0 as in section 4, E0 also gets the
   stdv presentations), and T(c,a) = min over F_c of `dreg_s` (F4 time; `build_s` is reported separately, see section 4 on the
   modulus weight). Gap(c) = mean_a D(c,a) - mean_a D(E0,a), percentile-bootstrap 95% interval over attempts (paired: same
   V-free instance data, same x_R); time ratio T(c)/T(E0) against the A/A band of the run. Floor curves are pooled by orbit
   (inference unit = orbit, 2 at n=131, 6 at n=43).
   - Outcome N (null, C2 supported): |Gap| < 0.5 degree for every orbit and the T ratio inside the A/A band, with the
     single-presentation (stdv) gap present in the same run (positive control seen). Reported: "the E0 vs floor gap under
     standard V is a presentation effect; after minimising over F both sides coincide". The smoke at n=131, k=5,6 is of this
     type (Gap 0, floor Dreg 6 in the adapted presentations).
   - Outcome G (gap survives): Gap >= 1 whole degree in every attempt of every floor orbit, or the T ratio outside the A/A
     band in the same direction for every orbit. Reported as a real E0 advantage of the instrument, and read only through C2':
     a genuine b-in-F_2 (Koblitz) structural effect, since the family already contains the b-adapted presentations that remove
     the "b is short in the polynomial basis" explanation; it is not an isogeny or level effect (the floor curves differ from
     E0 in b only), and it does not transfer to the 262 level-263 curves. Before reporting G: `nbadapt` must equal the floor
     curves (else the floor curves are not generic dense-b curves and C2' is unreadable), the isocheck must pass, and the gap
     must not vanish in a presentation added after the fact (a larger F is a dated amendment, both reported).
   - Outcome U (undetermined): cells that capshit or `atleast` (n=43 k=5, n=131 k>=7) are reported as bounds ("Dreg > dmax"),
     never as a gap.
   Not an improvement over the index-calculus baseline in either outcome (no S, no rho gap).
4. Times: ratio to the A/A interval of the same run; ranks are only read for ratios outside A/A. Wall clocks of runs that
   shared a host (smoke) varied by up to 2.8x for identical counts (n=43 k=4: 10.6-23.4 s), so only a run alone on its cores
   (isolab-eks) is read, and counts (Dreg, rows, cols) are primary.
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
