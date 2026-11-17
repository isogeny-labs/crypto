# PREREGISTRATION: E2-twin-target-amortization (E and E' together)

Directory: `/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E2-twin-target-amortization`
Status at writing: NO experimental data exist. Only tool-chain sanity checks were run
(section 0). Every number reported later must come from a file written by running the
frozen code below; missing or failed cells are reported as missing.

## 0. Pre-data disclosure (toolchain sanity only)

`sanity_toolchain.gp` (in this directory) was run once on a toy field (p = nextprime(2^20), a=17, b=29,
3000 random pairs, 200 twist pairs). It showed the x-only pair formulas are
consistent, and it printed `version()`=[2,15,4]. Those counts are NOT results and are not reused;
section 6 re-derives them under controls L1/L2 with the frozen code. Practical GP notes
learned: multi-line loops in a script need `{ }`; `gp` must be run with `< /dev/null` or end with
`quit` or it waits on stdin.

## 1. Question and hypotheses

Setting: E: y^2 = f(x) = x^3+ax+b over F_p (p prime), twist E': d y^2 = f(x) with d a non-residue
(isomorphic over F_p to Y^2 = X^3 + a d^2 X + b d^3 via X = d x, Y = d^2 y). The summation-polynomial
(Kummer, x-only) arithmetic is identical on both: with s = 2((x1 x2 + a)(x1 + x2) + 2b)/(x1-x2)^2 and
pr = ((x1 x2 - a)^2 - 4b(x1+x2))/(x1-x2)^2, the values x(P+Q), x(P-Q) are the two roots of
z^2 - s z + pr (doubling: x(2P) = ((x^2-a)^2 - 8bx)/(4 f(x))). A point with abscissa x is on E if
f(x) is a nonzero square (E-type) and on E' if f(x) is a non-square (E'-type).

Idea under test (H_idea, from the task): a decomposition solver with a per-variety precomputed
table T = {x(P_i +- P_j) : x_i, x_j in V} serves E and E' together, so
cost(E and E') ~ cost(E)(1 + eps), eps << 1.

Mechanistic counter-forecast (H_mech, my prediction): a table entry is useful to at most one of the two
curves. Reason: the separation lemma at the x-level. For a mixed pair (one E-type, one E'-type abscissa)
the discriminant of z^2 - s z + pr is f(x1) f(x2) times a square, hence a non-square, so x(P_i +- P_j)
lie in F_{p^2} \ F_p and can never equal the abscissa of a target in E(F_p) or E'(F_p). Same-type pairs
give F_p roots of the same type. So the E-part and E'-part of the table have disjoint key support, the pair
work for E and for E' is disjoint, and only the O(N) scan/classification of V is shared.
Hence cost(E and E') ~ 2 cost(E): eps ~ 1, not << 1.

Operational definitions (frozen):
* eps := C_both / C_E - 1, per run, where C_both is the total priced cost (section 4) of the JOINT arm
  (one shared table, relations collected on both curves) and C_E is the E-alone arm.
* sav := 1 - C_both / (C_E + C_Eprime) (saving of the joint arm versus the naive separate runs).
  H_idea predicts eps <= 0.25 (<=> sav >= 0.375). H_mech predicts eps ~ 1 and sav ~ 0.
* "changes nothing for a single curve": on identical E target streams, the set of verified relations found
  with the joint table equals the set found with the E-only table (exact), and the cost of using the joint
  table for E alone is >= the E-only cost (it only adds the useless half).

## 2. Numeric predictions (written before data)

P1. Paired design, medians over runs per (bits, mu) cell: eps in [0.95, 1.05]; individual runs in [0.80, 1.20]
    (random target-stream length, W ~ 10^3 to 10^4 relations).
P2. sav in [0.00, 0.02] for cells with N >= 300, in [0.00, 0.05] for N < 300; sav never above 0.10 in any run.
P3. Fraction of table entries usable by both curves: exactly 0 (disjoint key support); mixed pairs with an
    F_p-rational x(P+Q) (excluding x1 = x2 and zero-discriminant degeneracies): exactly 0.
P4. Single curve: identical relation sets (exact equality in 100% of paired streams); cost(joint used for E)
    / cost(E alone) - 1 in [0.9, 1.1] (the useless E' half roughly doubles the build).
P5. Positive-control detector (same curve, two target streams sharing ONE table, section 5 PC1):
    R_PC1 := C_shared / C_two_separate_runs. R_PC1(mu=3) <= 0.65, R_PC1(mu=0.75) >= 0.85, and
    R_PC1(mu=0.75) - R_PC1(mu=3) >= 0.20. (Derivation: with P = table work, C = collection work,
    R = (P+2C)/(2P+2C), and P/C grows as mu^3.)
P6. Unpartitioned joint arm J0 (all pairs of V, mixed pairs rejected by a Legendre test): C_J0 / (C_E + C_Eprime)
    in [1.3, 2.0] at mu >= 1.5 (mixed pairs are half of all pairs and are pure waste).
P7. Distance to rho (reported, not a gate): the relation phase alone costs roughly p^(2/3) GAE-order work
    versus ~0.886 sqrt(l) for rho, i.e. a ratio > 1 at every cell (I expect 4x to 40x at 22-30 bits).
    This is the remaining distance of this baseline, not a solve claim: no discrete log is solved here.

## 3. Falsifier (explicit)

* H_idea is FALSIFIED if: in every (bits, mu) cell the median eps >= 0.75 (equivalently median sav <= 0.125),
  AND the instrument is valid (PC1 passes P5, L1/L2 controls pass). One cell with median eps <= 0.25 and
  bootstrap 95% upper bound <= 0.25 at the same cell under all three w values (section 4) would contradict
  H_mech and must be reported as such, with the cell identified.
* H_idea is SUPPORTED if: bootstrap 95% upper bound of eps <= 0.25 in at least 3 cells spanning at least 2 bit
  sizes, under all of w in {0, 0.1, 1}, with every counted relation verified.
* Anything else (eps between 0.25 and 0.75 in the medians, or the instrument fails PC1/L1/L2): INCONCLUSIVE.
* H_mech is FALSIFIED if any run finds a verified relation on E whose decomposition uses an E'-type abscissa
  (it cannot, by the lemma, unless 2-torsion), or if L1 finds any mixed pair with F_p-rational roots.
* If PC1 fails (the detector cannot see a real shared-table saving), no verdict about eps is drawn.

## 4. Method, metric and cost accounting (all in PARI/GP 2.15.4; no Rust needed; nothing is built)

Toy-field m = 2 decomposition (a controlled stand-in for the Semaev/summation setting; it is NOT claimed to
be competitive; it isolates the sharing question).

Curves. For bits b in the grid: p = precprime(2^b - 1). Search (a, b_coef) with setrand(seed_b) for curves with
#E and #E' = p+1+t BOTH prime (so odd parts are coprime, E(F_{p^2}) = E x E', and the groups are cyclic of prime order
l_E, l_E'). Curve index c = 1, 2, 3 are the first three such curves found in the seeded search order.
d = least non-residue. E' = ellinit([a d^2, b d^3]) used only for lifting and verification (X = d x).
Generators P (E) and P' (E') = any non-O point (prime order).

Factor base. N = round(mu * p^(1/3)) abscissae of each type: V_E = the first N integers x >= 1 with f(x) a nonzero
square, V_E' = the first N with f(x) a non-square. Scan cost = number of Legendre symbol evaluations.

Table. For each unordered pair i < j in the same type class: pair step (one inversion of (x_i - x_j) squared;
~10 field multiplications; one square root of the discriminant) gives the two keys x(P_i + P_j), x(P_i - P_j);
for i = j the doubling formula gives x(2P_i). Keys are packed as key = x * 2^26 + code (code = type, index pair), stored
in one Vecsmall, `vecsort`ed. Lookup = binary search for the first key >= x * 2^26 (implemented in GP, probes counted).
Requires p < 2^36 and N < 2^12; the grid respects this.

Targets. R_k = (s0 + k) P (E) via R_{k+1} = R_k + P: ONE affine group addition per target (GAE). E' targets are the same
on E' and looked up at x = X/d (one extra multiplication). Collect until W = ceil(0.5 N ln N) verified relations (the
expected count for the relation graph on N vertices to connect) or the cap T_max = 6 * expected targets (cap hit
is reported as failure). Every hit is verified by explicit point lifting and group arithmetic (ellordinate,
elladd/ellmul) that R = +-P_i +- P_j; unverified hits are counted and excluded; verification work is excluded from all arms equally.
Descriptive extras (not gates): number of distinct FB columns touched, number of connected components of the relation
graph (union-find), and for N <= 700 the mod-l rank of the incidence-type matrix (matrank over F_l). No solve is claimed.

Cost (frozen, per section 5a of the repository rules). Priced work: A = group additions (1 GAE each) plus K = pair-step
inversions (1 GAE each, an inversion-dominated step; the ~10 multiplications are listed in raw counts only).
Unpriced native work, each op priced at w GAE with w in {0, 0.1, 1}: square roots, Legendre evaluations, sort comparisons
(nominal n ceil(log2 n)), lookup probes, the d^-1 scaling multiplications. cost(w) = A + K + w * (native ops).
All conclusions on eps / sav / R must hold at all three w to be stated; otherwise "order not stable across the band".
Raw op counts per phase are recorded, plus wall-clock (`getabstime`) per phase as a secondary column only (4 shared
cores; GP interpreter noise; no claim rests on wall time).

Relations per unit cost: rpc(arm, w) = (verified relations) / cost(w). Primary metric: eps; secondary: sav, rpc ratios.

Rho reference (distance, not a gate): a measured Pollard rho on E (r-adding walk, 16 branches, Brent cycle detection,
10 seeds per curve) counted in GAE with its comparison ops priced at w; plus the nominal 0.886 sqrt(l) listed as "nominal",
not as a measurement. Report ratio (relation phase cost) / (rho cost) per cell as distance remaining.

## 5. Arms and controls

Arms per run (same target streams across arms; seed pairing):
* A1 E-alone (baseline): scan for V_E, build E table, collect W on E.
* A2 E'-alone: same for E'.
* A3 JOINT (partitioned): one scan until both classes have N elements (max rather than sum of the two scans), build both halves,
  collect W on E and W on E' from the one table (each target looks up the single sorted array). Phases recorded
  separately: scan, tableE, tableE', collectE, collectE'.
* A4 SINGLE-JOINT (single-curve question): from A3's phases, cost for a user who wants only E but uses the joint table
  = scan + tableE + tableE' + collectE; relation set compared to A1 for equality.
* J0 UNPARTITIONED joint: all M(M-1)/2 + M pairs of V = V_E U V_E', each pair step plus one Legendre test of the discriminant;
  mixed pairs rejected. Only bits {22, 26}, curve 1.
Controls:
* L1 (x-level separation lemma, positive control of the structure): for bits 22, count mixed-type pairs of V with F_p-rational
  roots (full enumeration for N<=400 else 10^6 seeded random mixed pairs); prediction 0. Also check E-half key set and E'-half key
  set are disjoint (intersection size, predicted 0), and that every E-half key has f(key x) a square (assert).
* L2 (F_{p^2} separation lemma): for p about 2^16 and 2^20 (curves with both orders prime), 2000 random pairs (P in E(F_p),
  Q in E'(F_p)\{O}) mapped into E(F_{p^2}) via the isomorphism; check S = P + Q is in E(F_p) (predicted 0 of 2000); positive control:
  Q' = +-P'' for P'' in E'(F_p) so that B = O: S in E(F_p) (predicted 2000 of 2000); and R = A + B with B of
  order 2 impossible (odd order) so no case. Detects Frobenius acting as +1 / -1 on the two parts.
* PC1 (positive control for "the detector sees real sharing"): same curve E, TWO independent target streams (seeds s and s+100) sharing
  ONE table versus two separate full runs. R_PC1 = C_shared / C_separate_total. Bits {22, 26}, curve 1..3 (all mu).
* NC1 (negative control, wrong-variety table): table built for curve 1, targets from curve 2 (same p, different (a,b)); every hit
  must FAIL verification (predicted: verified relations 0, up to the analytic accidental-pass rate ~ O(1/p) per hit; report the count).
  Bits {22, 26}.
* NC2 (null calibration): observed hit rate of E targets vs the analytic hit rate
  y_hat = (distinct E-half keys) / ((#E - 1)/2); report ratio (predicted 1.00 within binomial 3 sigma) — proves the multiples-of-P stream
  behaves as uniform x for this purpose. Bits {22, 26, 30}.
* S0 (cost calibration microbenchmark): 20000 pair steps, lookups and group additions at bits 26 to project wall time before the grid;
  rule: any cell projected over 12 min is not run, dropped largest-first and listed in DEVIATIONS.md.

## 6. Parameter grid (frozen)

mu in units of p^(1/3): {0.75, 1.5, 3} (the cost balance N^3 ~ p ln N / 2 puts the optimum near mu ~ 1.5).
| bits | mu values | curves c | stream seeds s | runs |
|---|---|---|---|---|
| 22 | 0.75, 1.5, 3 | 1,2,3 | 1,2,3 | 27 |
| 26 | 0.75, 1.5, 3 | 1,2,3 | 1,2,3 | 27 |
| 30 | 0.75, 1.5 | 1,2 | 1,2 | 8 |
| 32 (stretch) | 1.5 | 1 | 1 | 1, only if the S0 projection is <= 12 min |
Not planned: 34+ bits (disk 1.2 GB free, runtime); mu = 3 at 30 bits (projected > 12 min). Missing cells are reported as missing.

Seeds (all `setrand`): curve search: 20261021 + bits; target start s0 = 1000*bits + 10*c + s; stream for PC1 second stream: s0 + 100000;
rho walks: 5000 + 100*c + k (k=1..10); bootstrap: setrand(424242) with 10000 resamples; L2 : 777. GP version and seeds repeated in each raw file header.

## 7. Analysis plan

`e2_analyze.gp` (GP) reads `results.csv` and computes per (bits, mu): median / min / max of eps, sav, R_PC1, R_J0, single-curve ratio, relation-set
equality fraction, and bootstrap 95% intervals of eps, at each w in {0, 0.1, 1}. Python (venv) only plots `results.csv`/`summary.csv`
(eps vs mu per bit size; PC1 vs twin; distance to rho). Negative results are reported with equal weight; no run is discarded except the pre-declared drops.

## 8. Exact commands (frozen before any data run)

```
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E2-twin-target-amortization
cd $D
# code files to be written next, then frozen: e2_lib.gp e2_run.gp e2_lemma.gp e2_rho.gp e2_analyze.gp plot.py
sha256sum e2_*.gp plot.py PREREGISTRATION.md > FREEZE.sha256         # BEFORE the first data run; code edits afterwards go to DEVIATIONS.md and invalidate pooled cells
E2_BITS=26 gp -q -f e2_micro.gp < /dev/null > raw/S0_micro.out 2>&1   # S0 calibration
E2_BITS=22 gp -q -f e2_lemma.gp < /dev/null > raw/L1_L2_b22.out 2>&1
E2_BITS=26 gp -q -f e2_lemma.gp < /dev/null > raw/L1_L2_b26.out 2>&1
# main grid, one cell per process, at most 2 in parallel, 900 s cap each; E2_ARMS=all runs A1-A4 (+ PC1 and NC1/NC2 for bits<=26, J0 for bits<=26 curve 1)
for b in 22 26; do for mu in 0.75 1.5 3; do for c in 1 2 3; do for s in 1 2 3; do
  echo "E2_BITS=$b E2_MU=$mu E2_CURVE=$c E2_SEED=$s E2_ARMS=all timeout 900 gp -q -f e2_run.gp < /dev/null > raw/run_b${b}_mu${mu}_c${c}_s${s}.out 2>&1"
done; done; done; done > grid_cmds.txt
for b in 30; do for mu in 0.75 1.5; do for c in 1 2; do for s in 1 2; do
  echo "E2_BITS=$b E2_MU=$mu E2_CURVE=$c E2_SEED=$s E2_ARMS=core timeout 900 gp -q -f e2_run.gp < /dev/null > raw/run_b${b}_mu${mu}_c${c}_s${s}.out 2>&1"
done; done; done; done >> grid_cmds.txt
xargs -P 2 -I{} sh -c "{}" < grid_cmds.txt
E2_BITS=32 E2_MU=1.5 E2_CURVE=1 E2_SEED=1 E2_ARMS=core timeout 900 gp -q -f e2_run.gp < /dev/null > raw/run_b32_mu1.5_c1_s1.out 2>&1   # stretch, only if S0 allows
for c in 1 2 3; do E2_BITS=26 E2_CURVE=$c timeout 900 gp -q -f e2_rho.gp < /dev/null > raw/rho_b26_c$c.out 2>&1; done   # also bits 22, 30
gp -q -f e2_analyze.gp < /dev/null > raw/analysis.out 2>&1         # writes results.csv, summary.csv
/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python plot.py   # plots only recorded values
```
Every `run_*.out` begins with a header (gp version, bits, p, curve (a,b), #E, #E', l_E, l_E', d, seeds, N, W) and ends with a single
`RESULT,...` CSV line that `e2_analyze.gp` ingests.

## 9. Limits stated in advance

* This toy m = 2 table decomposition is not a competitive ECDLP algorithm; only the sharing ratio is the object of study.
* GP wall time is noisy and secondary; op counts are primary.
* The per-pair `sqrt` and Legendre costs are unpriced native work priced at w in {0, 0.1, 1} GAE; no claim rests on a single w.
* Finite sizes (22-30 bits): the O(N) shared scan fraction (about 4/N) is the only structural saving and is visible at small N.
* Extension-field (F_{q^n}) factor bases with subspace V are not tested; the disjoint-key argument is type-based and predicted to carry over, which is NOT shown here.
* No full DLP is solved and no speedup over rho or the index-calculus baseline is claimed from this experiment.
