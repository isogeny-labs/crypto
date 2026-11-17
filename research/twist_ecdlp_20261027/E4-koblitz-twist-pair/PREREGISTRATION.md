# PREREGISTRATION E4-koblitz-twist-pair

Written 2026-10-21 before any experimental data was computed. Only toolchain smoke
tests were run (gp 2.15.4 starts; `ffgen(2^7)`, `ellinit`, `elladd`, `ellcard`,
`factor` over F_{2^7} work; `fftrace` does NOT exist in this gp, so traces are
computed by an explicit loop of m squarings). The smoke test outputs are not data
for this experiment and are not quoted below.

Scratch directory: `/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E4-koblitz-twist-pair`
(abbreviated `$D`). Scripts go to `$D/scripts/`, raw outputs to `$D/out/`.
Language: PARI/GP only for computation; Python (`.../scratchpad/venv/bin/python`) only
to plot values already recorded in `$D/out/`. No Rust, no build of the main crate.

## 1. Claim classification (controlled-experiment discipline)

* Exact-computation claims (a computation either holds or fails, no statistics):
  C1 #K0 + #K1 = 2^{m+1}+2; C2 trace(K1) = -trace(K0), tau_1 = -tau_0;
  C3 every x in F_q^* is the abscissa of a point of exactly one of K0(F_q), K1(F_q);
  C4 Frobenius x -> x^2 preserves that type; C5 the Semaev S3 polynomial
  (b = 1, independent of a2) is shared by K0 and K1; C6 a pair (x1, x2), both
  nonzero, has a rational third abscissa x3 iff the two have the same type.
* Empirical/measured claims (need controls and uncertainty): cost ratio of
  the shared-base arm to the E-only arm, in counted quadratic solves and in wall
  time. The relation-count claim is exact (set equality), but is still run against
  controls so the detector is shown able to see a difference.
* Analytic reason for the prediction (stated so that a failure is informative, not so
  that it replaces the run). Semaev S3(x1,x2,x3) = (x1x2+x1x3+x2x3)^2 + x1x2x3 + 1
  is quadratic in x3: A x3^2 + B x3 + C with A=(x1+x2)^2, B=x1x2, C=B^2+1. For B != 0 it
  has roots in F_q iff Tr(AC/B^2)=0, and AC/B^2 = A + (1/x1+1/x2)^2, so the condition is
  Tr(x1+1/x1) = Tr(x2+1/x2), i.e. x1, x2 are of the same type. K0 abscissas satisfy
  Tr(x+1/x)=0, K1 abscissas Tr(x+1/x)=1 (Tr(1)=1 for odd m). Group-theoretically:
  P1 in E(F_q), P2 in E'(F_q), x(P1+P2) in F_q iff pi(P1+P2)=P1-P2 equals +-(P1+P2)
  iff 2P2=O or 2P1=O. The code must not use this shortcut; it computes Tr(AC/B^2)
  directly and compares with the independently computed types.
  Note the proof of the lemma needs no coprimality: R=A+B with pi R=R gives 2B=O.
  Over char 2 the only rational 2-torsion point is T=(0,1) (on both curves), so the
  exact statement is B in {O, T}; x=0 is excluded from all bases here and its
  pairs are counted separately.

## 2. Hypothesis

H0 (separation, prediction): for K0: y^2+xy=x^3+1 over F_{2^m}, m odd prime, a
factor base defined by x-coordinates in an F_2-subspace V of dimension l (and its
Frobenius-orbit classes), shared with the twist K1: y^2+xy=x^3+x^2+1, produces
exactly the same set of K0 relations (Semaev m=2 decompositions R = +-P1 +- P2)
as the E-only base V cap {x on K0}. The K1 half of the shared base contributes only
relations for K1 (E'-relations), zero cross-type relations, zero extra rank for K0,
and costs more to enumerate. No gain.

H1 (alternative being tested): the shared x-only base gives more K0 relations
(any Delta > 0) or a lower counted cost per K0 relation than the E-only base.

## 3. Numeric predictions (frozen)

Let N0V = |{x in V\{0}: Tr(x+1/x)=0}|, N1V = |{x in V\{0}: Tr(x+1/x)=1}|.

P0 (Stage 0 identities, every m in {11,13,17,19}): #K0+#K1 = 2^{m+1}+2; t(K1) =
-t(K0) from brute force, ellcard and the Lucas recurrence; #K0 = 4*odd, #K1 = 2*odd
(Koblitz cofactors for odd m, a=0 -> 4, a=1 -> 2); N0 = (#K0-2)/2, N1 = (#K1-2)/2,
N0+N1 = 2^m-1; type is constant on Frobenius orbits for every x in F_q^*;
S3 vanishes at x(P1+P2) and x(P1-P2) for random K0 point pairs. In F_{q^2}
(m in {11,13,17}): for P1 in K0(F_q), P2 in K1(F_q) lifted to F_{q^2}, x(P1+P2) is
not in F_q for every sampled pair with P1,P2 != T; for P1,P2 both in K1 it is in
F_q; for P2 = T it is in F_q.
P1 (Stage 1, exhaustive over all pairs of V\{0}): violations of "rational x3 <=>
same type" = 0; cross-type pairs with a rational root = 0; the total number of
rational (unordered pair, x3) triples equals 2*C(N0V,2)+N0V+2*C(N1V,2)+N1V exactly
(equal x1=x2 gives the single finite root x(2P1); unequal pairs give 2 distinct roots).
P2 (Stage 2): for every V, every target, K0 relation sets (sets of unordered
{x1,x2} found per target) are identical in the E-only and shared arms:
Delta_rel = 0, mismatching targets = 0, orbit-canonical row counts identical.
Cross-type relations for K0 targets = 0. K1-type targets give 0 relations from the E-only base; from the shared base
they give K1-pure relations only (counted separately as M5, not credited to K0).
P3 (cost): counted quadratic solves per target: E-only = N0V, shared = N0V+N1V, ratio
(N0V+N1V)/N0V in about [1.6, 2.5] for l >= 6 (about 2 since V is ~half type-0);
cost per K0 relation, shared/E-only, equals that ratio exactly (same relation count),
so >= 1 in every V. Wall-time ratio shared/E-only predicted in [1.3, 2.3] and above the
null band of NC1. Cost of the K1 half is pure overhead for the K0 DLP.
P4 (positive control sensitivity, below): pooled N_rel(l+1)/N_rel(l) in [3.0, 5.0];
cost-per-relation ratio (l+1)/l in [0.35, 0.70]. Scrambled-label cross fraction in
[0.35, 0.65].

## 4. Baseline

Baseline arm A_E: E-only factor base B_E(V) = {x in V\{0} : Tr(x+1/x)=0} (x-abscissas
of K0(F_q) points, the current standard decomposition setup for K0). Decomposition:
for each target x_R, for each x1 in B_E, solve the quadratic S3(x1,x2,x_R)=0 in x2 and
keep roots x2 in V that are also of K0 type. Candidate arm A_S: shared base
B_S(V) = V\{0} (no type filter), same solver, same membership test, keeping every
root x2 in V; each found relation is then classified as K0-pure, cross, or K1-pure
by the independently computed type. Both arms use the same code path, same V, same
targets. Reported effect: Delta = A_S - A_E in K0 relations, cross relations, counted
solves, cost per relation, wall time. Nothing here is a complete DLP solve and no
Pollard-rho gap is claimed (no matched rho; AGENTS.md 5a pricing is not triggered
because no cross-method ranking is made). The index-calculus leaderboard is not touched.

## 5. Metrics (primary and secondary)

Primary:
M1 Delta_rel = N_rel(A_S, K0 targets) - N_rel(A_E, K0 targets) per (m,l,V), plus the
   count of targets whose relation sets differ.
M2 N_cross = number of cross-type pairs (types of x1, x2 differ, both nonzero) with a
   rational third abscissa (Stage 1, all pairs) and cross-type relations found in Stage 2.
M3 cost ratio rho_C = C(A_S)/C(A_E), C = counted quadratic solves (each = one
   inversion plus one trace/half-trace root extraction in F_q) per target, and
   cost per relation C/N_rel; secondary wall time (gp getabstime of the sieve loop,
   median of 3 interleaved repeats ordered ABBA / BAAB / ABBA).
Secondary:
M4 columns: orbit classes in B_E vs B_S, and classes used by at least one K0 relation
   ("dead" shared columns = classes of K1 type; expected all K1 classes dead for K0).
M5 K1-target relation count from the shared base vs E-only base (shows what the K1 half
   is good for).
No rank measurement: identical relation row sets imply identical rank under any rank
function, so a rank run would not be an independent measurement. It is reported as
"not measured, determined by row-set equality".

Uncertainty: M1, M2 are exact counts (no interval needed). M3 over V draws: median and
percentile bootstrap 95% interval (2000 resamples, GP `setrand(20261021)`), per (m,l).
Rank by effect size, not significance.

## 6. Controls

Positive controls (the detector must see an effect when one exists; if any fails, a
"no gain" result is reported as INCONCLUSIVE, not supported):
* PC1 (genuine gain of known size): E-only base at dimension l+1 (V' = V + <w>, w
  random outside V) vs dimension l, same targets. Expected N_rel ratio about 4
  (pairs), solves ratio about 2, hence cost per relation ratio about 0.5. Pass:
  pooled N_rel ratio in [3.0,5.0] and cost-per-relation ratio in [0.35,0.70] for each
  (m, l) with l <= 9. This shows the metrics flag a factor-2 gain.
* PC2 (cross-type detector can fire): random permutation of the type labels over
  V\{0} (same N0V/N1V counts), then count rational-root pairs whose scrambled
  labels differ. Expected about half of all rational-root pairs. Pass: fraction in
  [0.35,0.65] pooled per m. Real labels must give 0 (M2).
* PC3 (group-level detector sees rational sums): Stage 0b, K1xK1 pairs and pairs with
  T must give x(P1+P2) in F_q (100% of samples), and K0xK0 pairs likewise, so the
  "not in F_q" outcome for mixed pairs is not an artefact of a broken membership test.

Negative controls (null distribution, same instrument):
* NC1 (equal arms): A_E run twice, second time with x1 enumeration order shuffled
  (seeded) ; expected Delta_rel = 0 and wall-time ratio 1 +- noise. The 95% band of
  this ratio across all runs is the noise floor: a shared/E-only time ratio is called
  "slower" only if it is above the band, "indistinguishable" if inside it.
* NC2 (two V populations): V_rand (uniform random independent vectors, seeded draws)
  and V_poly (basis 1, a, ..., a^{l-1} of the field presentation, one per (m,l)).
  The conclusion must hold in both; V_poly is reported separately.
* NC3 (wrong-curve targets): targets of K1 type given to the E-only base must give 0
  relations; shared base gives the E' relations (M5), which are not credited to K0.

## 7. Explicit falsifier

The prediction "identical E-relation counts, no gain" is FALSIFIED if any one of:
F1 any (m,l,V,target) has a K0 relation found by A_S and not by A_E, or the reverse
   (Delta_rel != 0 or relation-set mismatch);
F2 any cross-type pair of nonzero abscissas has a rational third abscissa (Stage 1),
   or any Stage 0b mixed pair P1 in K0, P2 in K1 (both != T) has x(P1+P2) in F_q;
F3 for any V, cost per K0 relation of A_S is lower than that of A_E (counted solves), or
   for some (m,l) the bootstrap 95% interval of the wall-time ratio S/E lies entirely
   below 1 and below the NC1 noise band;
F4 the formula in P1 (rational triples = 2C(N0V,2)+N0V+2C(N1V,2)+N1V) is violated for any V.
The result is INVALID (not a falsification, not support) if a Stage 0 gate fails
(C1 to C6, S3 formula check), if a script crashes or times out (reported as missing for
that cell), or if a positive control fails. Any cell not run is reported as not run;
no number is reported that is not read from a file in `$D/out/`.

## 8. Parameter grid and seeds

m in {11, 13, 17, 19}. Field presentation: `ffgen(2^m,'a)` (gp default irreducible
modulus; modulus recorded in each output header).
Dimension l: m=11: 4..9; m=13: 5..10; m=17: 6..10; m=19: 7..10 (2l-m gives
expected 2^(2l-m) relations per target, between about 2^-5 and 2^3; l <= 10 for runtime).
Populations: rand, draws k = 1..K with K = 20 (l <= 8), 8 (l = 9), 4 (l = 10); poly,
k = 0 (one draw).
Seed for a cell: s = 2026102100 + 1000000*m + 10000*l + 100*pop + k, pop=1 (rand) or
2 (poly). `setrand(s)` draws V (rand) and extension vector w for PC1; `setrand(s+500000000)`
draws targets; `setrand(s+900000000)` draws the shuffle for NC1 and the label
permutation for PC2. Bootstrap seed 20261021.
Targets (Stage 2): 40 base K0 points R (random(E0), x(R) != 0), each expanded to its
full Frobenius orbit x(R)^(2^j), j=0..m-1 (orbit-closed target set, 40*m abscissas);
40 base K1-type abscissas (rejection sample Tr(x+1/x)=1) likewise for NC3/M5.
Stage 0b samples: 200 pairs per class (K0xK0, K1xK1, K0xK1, K1xT) for m in {11,13,17};
Stage 0 S3 formula: 2000 random K0 point pairs per m.
Machine: 4 shared cores. At most 2 concurrent gp processes; each run under `timeout 900`
(single run <= 15 min); on timeout the cell is marked missing, largest l dropped first.
Disk: outputs are small text; nothing is built; no target/ directory exists.

## 9. Procedure and exact commands

Stages (scripts are written next, then frozen by hashing before any stage is run;
a change after freezing requires a dated amendment appended to this file):
* stage0_identities.gp: C1..C5, Lucas recurrence for t from tau (a=0: tau^2+tau+2=0;
  a=1: tau^2-tau+2=0; V_k = mu V_{k-1} - 2 V_{k-2}, V_0=2, V_1=mu; t(F_{2^m}) = V_m),
  brute-force abscissa census, ellcard, cofactors, orbit invariance of type, S3 check.
* stage0b_fq2.gp: F_{q^2} separation check (types via rationality of z in
  z^2+z = x + a2 + 1/x^2 over F_{q^2}; embedded F_q elements are g2^((q+1)*k)).
* stage1_exhaustive.gp: all pairs of V\{0}; direct Tr(AC/B^2) vs types; formula check;
  PC2 label scramble.
* stage2_sieve.gp: arms A_E, A_S, NC1 duplicate, PC1 (l+1) with counters and timing,
  K0 and K1 targets; writes CSV.
* analyze.gp: aggregates CSVs, bootstrap, control verdicts, writes `out/verdict.txt`.
* plot_e4.py: figures from `out/*.csv` only.

```
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E4-koblitz-twist-pair
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
cd $D
sha256sum PREREGISTRATION.md scripts/*.gp > out/FROZEN.sha256     # before any stage runs
for M in 11 13 17 19; do M=$M timeout 900 gp -q -f scripts/stage0_identities.gp > out/stage0_m$M.txt 2>&1; done
for M in 11 13 17;    do M=$M timeout 900 gp -q -f scripts/stage0b_fq2.gp       > out/stage0b_m$M.txt 2>&1; done
# grid (l ranges per Section 8); POP=1 rand draws, POP=2 poly
for M in 11 13 17 19; do for L in <grid(M)>; do for POP in 1 2; do
  M=$M L=$L POP=$POP timeout 900 gp -q -f scripts/stage1_exhaustive.gp >> out/stage1_m$M.csv 2>>out/stage1_m$M.err
  M=$M L=$L POP=$POP timeout 900 gp -q -f scripts/stage2_sieve.gp      >> out/stage2_m$M.csv 2>>out/stage2_m$M.err
done; done; done                                   # run at most 2 M-values concurrently
timeout 900 gp -q -f scripts/analyze.gp > out/verdict.txt 2>&1
$PY scripts/plot_e4.py                              # reads out/*.csv, writes out/*.png|svg
sha256sum out/*.csv out/*.txt > out/RESULTS.sha256
```
Every script starts with `default(parisizemax,2000000000)`, prints a header line with
m, l, pop, k, seed, modulus, gp version; every CSV row carries (m,l,pop,k,seed,...).
Raw gp stdout/stderr are kept; nothing is edited by hand.

## 10. Reporting rules

Report per (m,l): Delta_rel, N_cross, cost ratio with bootstrap interval, control
outcomes, cells missing. State plainly that the relation-count result is an exact
identity (expected by algebra) and that the only empirically uncertain quantity is
the cost ratio. Do not call relabelling or accounting changes improvements. A
negative result here is a scoped negative: Semaev m=2, x-only subspace bases, K0/K1
pair, m in {11,13,17,19}; it says nothing about larger decompositions (m>=3), non-
x-only bases, or other curves.
