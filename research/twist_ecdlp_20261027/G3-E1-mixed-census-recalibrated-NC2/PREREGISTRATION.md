# PREREGISTRATION G3-E1-mixed-census-recalibrated-NC2

Status: FROZEN before any data. Written 2026-10-26. No result of this experiment exists. Only toolchain sanity (gp 2.15.4 runs,
`ellcard(ellinit([1,2],nextprime(2^20)))` prints 1050224) was run. Directory:
/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G3-E1-mixed-census-recalibrated-NC2
Language: PARI/GP only (AGENTS.md "Implementation language"). Python only plots recorded values. No Rust, no main-repo build.
No ECDLP cost claim, no rho arm, no GAE pricing: this is a structural census of Semaev solutions, not a cost comparison.

## 0. Why this experiment (inheritance from E1, disclosed)

E1 (../E1-mixed-solution-census, replicated bit-for-bit in ../E1-replication) is formally INCONCLUSIVE: the preregistered NC2 gate
passed in 3 of 16 cells; the null ignored the realised class imbalance nE/n and per-target clustering; even the post-hoc cluster
bootstrap failed in 2 of 16 cells. E1 also misquoted positive-control counts (PC2 is 400/400, PC3 is 800/800; PC1 400/400).
E1 data are used here ONLY for design (cost per target, cell sizing, and the observed maximal NC2 deviation 0.0219, which informs but
does not tune the equivalence margin in section 6). All numbers of this experiment are produced by new runs under a new seed.
Substantive structure (E1, reproduced by independent group-law code): A = B = T in E[2](F_q); m=2 has no mixed solutions; for m=3
mixed solutions equal a predictor S and need n_T > 0. This experiment tests that with a gate that can pass or fail honestly.

## 1. Claim classification (controlled-experiment skill)

- Exact algebra (derived in E1 prereg section 2, no coprimality needed): Frobenius gives 2A = 2B = O, A = B = T in E[2](F_q).
  Consequences C1 (m=2: no mixed solutions), C2 (m=3, n_T=0: only degenerate mixed {x,x,x_R}), C3 (m=3, n_T>0: mixed = the family
  {x1, tau_T(x1), tau_T(x_R)} with x1, tau_T(x1) twist-class and in V). tau_T(x) = e + f'(e)/(x-e) = x(P+T), T=(e,0).
- Empirical claim tested: "no non-degenerate mixed solutions when n_T = 0 (any m); for n_T > 0 mixed solutions occur only for m=3 and
  at rate about c*n_T/q relative to E-only solutions". Split into two sub-claims:
  Z (zero claim): n_T=0 or m=2 gives N_M_nd = 0 (m=2: also N_M_deg = 0), and for m=3 N_M = S (structure) exactly.
  Rt (rate claim): for m=3, n_T>0, f = N_M_nd/(N_M_nd+N_E_nd) satisfies f*q/n_T within a factor 2 of c_ref (section 4), for q >= 2^16.
- Not claimed: any ECDLP cost, speedup or improvement over index calculus. A negative census says the twist adds no relations for
  R in E(F_q) beyond the structured family above.

## 2. Setting and definitions (as E1, restated)

E: y^2 = f(x) = x^3+ax+b over F_q, q prime > 3; d = least non-residue; twist E_d: d y^2 = f(x). R in E(F_q), 2R != O.
Class of x in F_q: E (f nonzero square), W (f non-square, twist point), T2 (f = 0). E-only solution = multiset {x_1..x_m} of V with
S_{m+1}(x_1..x_m,x_R) = 0 and no element of class W; mixed = at least one W element. Non-degenerate (nd): x_i pairwise distinct and
none equal x_R; else degenerate (deg). n_T = #roots of f in F_q in {0,1,3}. S_3, S_4 as E1 prereg section 1.
nE = #{x in V: class E}, nW = #{x in V: class W}, nT2 = #{class T2}, n = |V|. Realised values are recorded.

## 3. Hypotheses, numeric predictions

H1 (exact, Z): for every config and target: N_M(m=2) = 0 (nd and deg); for m=3, N_M_nd = S_nd and N_M_deg = S_deg per target;
  therefore N_M_nd = 0 for every n_T=0 config, m=3.
H2 (rate, Rt): for m=3, n_T>0, q >= 2^16: c_hat = f*q/n_T satisfies 0.5 <= c_hat/c_ref <= 2 where
  c_ref = 1.5 * (nW/nE)^2 * (#E/q) per config (derivation in section 4); asymptotically c_ref -> 1.5 for balanced V.
H3 (F4 breach, 12-bit constants): E1 gave f*q/n_T of about 3.6 (e.g. E1 cell 12_0_3_m3_interval_0.65: f=0.00419, q=2609, n_T=3) against
  1.5. Prediction: the 12-bit excess is a finite-size effect that shrinks with q: pooled c_hat/c_ref has 95% interval inside
  [0.8,1.25] at 16 and 20 bits (category A or B below), and the 1.5 n_T/q heuristic is right asymptotically while the larger 12-bit
  constants describe 12-bit data only. Falsified (category C) if c_hat/c_ref at 16 or 20 bits is outside [0.8,1.25] with the interval
  excluding it.
Numeric prediction summary: N_M_nd(m=2)=0 in all targets; N_M_nd(m=3,n_T=0)=0 in all targets; target-level mismatches N_M != S = 0;
pooled 20-bit n_T=3 m=3 c_hat/c_ref in [0.8,1.25]; mixed counts at the 20-bit resolution cells (section 5) >= 20 and at the 24-bit
cell >= 10 (sizing heuristic, expected about 1.5e-5 mixed per target there: n_T n^3/(4q^2) with n=1500).

## 4. Baselines

- Naive: B_naive = n^m/(m! q) E-only per target. Refined: B_ref = 2^m nE^m/(m! #E) (E1 section 4).
- Mixed baseline for exact claims: 0 (H1) with the independent structural predictor S (Oracle S) as exact comparator.
- Refined rate baseline (derived before data): tau_T maps W onto W, so for x1 in V cap W, P(tau_T(x1) in V cap W) = 2nW/q (|W| ~ q/2);
  P(tau_T(x_R) in V) = 2nE/q. Hence S_ref = n_T * (nW^2/q) * (2nE/q) = 2 n_T nW^2 nE/q^2; E_ref = B_ref(m=3) = 4 nE^3/(3 #E);
  f_ref = S_ref/E_ref = c_ref n_T/q with c_ref = 1.5 (nW/nE)^2 (#E/q). The naive heuristic is c = 1.5 (balanced V, #E = q).
  Both reported: c_hat vs 1.5 and c_hat/c_ref.
- Foil (H_rand): zeros independent of class, mixed fraction 1 - p0 with p0 the conditional null of section 6 (NC2).

## 5. Parameter grid

Seeds: S0 = 20261026 (all `setrand` calls derive from S0; env G3_SEED). Bootstrap seed S0+1 (hierarchical, section 7), B = 2000.
Primes: stream per bit size: setrand(S0+100000*bits); q_k = nextprime(2^(bits-1)+random(2^(bits-1))), keep the first K distinct;
 K = 4 (12 bits), 4 (16), 3 (20), 2 (24) = 13 primes (E1 had 7). pidx = 0..K-1.
Curves per prime: cidx 0..11 (E1 had 4): stratum n_T = [0,1,3][cidx mod 3]; cidx 0 and 3 (n_T=0) seek gcd_odd(#E,#E') > 1 within
 20000 draws, falling back (flagged FALLBACK) to any n_T=0 curve; cidx 6 and 9 are plain n_T=0. Draw: setrand(Wc), Wc = Wp + 10*(cidx+1),
 Wp = S0 + 100000*bits + 1000*pidx; a,b uniform, disc != 0. 156 (prime,curve) units: 52 with n_T=3.
m in {2,3}. Factor bases (as E1): interval, random, twistmult, farey; n = min(ceil(q^alpha), cap), alpha in {0.5,0.65,0.8},
 cap 16384 (m=2) / 1500 (m=3); duplicates after capping run once. FB seeds Wc+31 (random), Wc+41 (twistmult).
Targets: 200 (m=2), 100 (m=3) per config, R = aP+bQ with seed Wc+7 (paired across FB, alpha, m on a curve), reject R=O, 2R=O.
Time guard 120 s of target loop per config (E1: 0 truncations at 60 s); truncated configs report targets done.
Main grid: 13 primes x 12 curves x 2 m x 4 FB = 1248 jobs (each runs 1-3 alphas).

Resolution stage (new, the 20- and 24-bit n_T=3 gate cells; m=3, targets from seed Wc+10^9+1000*j for block j, so disjoint from the grid):
 R20: bits=20, pidx in {0,1}, cidx in {2,5} (n_T=3), n = 1500 (alpha 0.65), FB in {random, farey, twistmult}, 2 blocks of 10000 targets
      (E1 measured about 0.02 s/target at n=1500, 20 bits) -> 12 world-FB cells x 20000 = 240,000 targets, 24 jobs.
      Heuristic expected mixed per world-FB about 100 (E1 observed 2 per 100 targets in some cells); twistmult predicted 0 (structure).
 R24: bits=24, pidx=0, cidx in {2,5}, n = 1500 (alpha 0.5, capped), FB = random, 6 blocks of 60000 targets per curve (E1: 0.01 s/target)
      -> 720,000 targets, 12 jobs, each about 10 minutes. Heuristic expected mixed about 11 in total.
 Resolution rule (fixed now): a resolution cell is RESOLVED iff pooled S_nd (the independent structural count, computed in the same loop)
 >= 20 (R20, per FB for random and farey pooled over worlds) or >= 10 (R24 pooled). Otherwise UNRESOLVED: it may not support or refute Rt.
 Per-target lines are compact (t, E_nd, M_nd, S_nd, M_deg, S_deg) to bound disk.

## 6. Detectors, oracles, controls and gates

Detector A (production), Oracle B (brute force, n <= 120), Oracle G (group law over F_{q^2}, n <= 120), Oracle S (structural predictor):
unchanged code from e1_census.gp (sha256 4f73a036...0e63, recorded below). Stage 0 (V0): bits=12, pidx 0,
cidx in {0,1,2,3}, all 4 FB, n=96, m=2,3, 20 targets, plus the toy block V = F_q for q in {101,103}: A = B = G, mixed counts = S.
Any disagreement stops the experiment (gate G0).

Positive controls (sensitivity must be 100%): PC1 planted nd mixed (n_T >= 1, m=3; one block per rational 2-torsion point), PC2 planted
 degenerate mixed {x',x',x_R}, PC3 planted E-only (m=2 and m=3). Worlds: bits in {12,16,20,24}, pidx 0, cidx in {0,1,2} (n_T = 0,1,3);
 base V random with n = min(ceil(q^0.5), 400) (cap keeps 24-bit controls fast), 50 trials per block. Expected counts by construction
 (to be checked against the file, not copied from it): PC1 4 blocks x 4 bit sizes x 50 = 800; PC2 3 x 4 x 50 = 600; PC3 3 x 4 x 2 x 50 = 1200.
 Gate G1: every trial found, correct class, Oracle G confirms.
Negative controls:
 NC1 (E-only FB x(kG), k=1..n; same worlds as PC, n=100 oracle-checked and n=min(ceil(q^0.65),1500) detector-only): mixed = 0 everywhere. Gate G2.
 NC3 (target on the twist; mirror): bits in {12,16,20}, pidx 0, cidx 0..5, FB random and twistmult, alpha 0.65, targets on the twist
   (seed Wc+7): m=2 gives 0 "mixed" (some E-class x_i); m=3 equals the swapped structural predictor. Gate G4: 0 mismatches.
 NC2-CALIBRATED (G3, the fixed null; replaces the E1 null). Instrument question: when the lemma cannot hold, does the detector see mixed
   zeros at the rate implied by the realised class composition? Procedure: same V, same targets as the main grid, at alpha 0.65, on
   (prime, curve) units cidx 0..5 of every prime; Semaev polynomials built with b' = b+1 (b+2 if singular), classification of x_i by the
   ORIGINAL f. Per target t, over non-degenerate solutions only: N_t = e_t + m_t (E-only e_t, mixed m_t). Conditional null computed per
   target from the realised composition: n_t = |V minus {x_R}|, nW = #W in V, p0_t = C(n_t - nW, m)/C(n_t, m) (hypergeometric: P a random
   m-subset of V has no W element; equals the conditional P(zero is E-only | nE/n)), null mixed share pi_t = 1 - p0_t.
   Statistic D = (sum_t m_t - sum_t N_t pi_t) / sum_t N_t  (weighted deviation of mixed share from the conditional null).
   Interval: HIERARCHICAL percentile bootstrap, 95%, B=2000: resample primes with replacement within a (bits,FB,m) cell, then curves
   within each drawn prime, then targets within each drawn curve; the unit of inference is target/curve/prime, never solutions.
   Cell = (bits, FB, m): 4 x 4 x 2 = 32 cells. A cell PASSES iff the 95% interval of D lies inside [-0.05, +0.05] (equivalence margin
   0.05, small against the effect to be detected: the null mixed share is >= 0.3 in every cell by construction of pi_t, so a failure to
   see mixed zeros would show as |D| >= 0.3; E1's maximal deviation was 0.0219) AND mean pi_t >= 0.3 (power condition).
   Gate G3 PASSES iff all 32 cells pass. If k cells fail, G3 FAILS; failing cells are listed and the main-grid cells with the same
   (bits, FB, m) are INADMISSIBLE for any zero-count evidence (they remain reported). No post-hoc change of null, margin or unit.
 Gate G5 (reproducibility): jobs NR%60==1 of the main joblist are re-run (same seeds) and their TGT lines must be byte-identical.
 Gate G0 and G1 failing voids everything; G2, G3, G4 failing makes the affected Z evidence INADMISSIBLE (not a refutation).

## 7. Metric and statistical treatment

Per config: bits, prime, curve, FB, alpha, m, n, nE, nW, nT2, #E, #E', gcd_odd, n_T, targets done, N_E_nd, N_E_deg, N_M_nd, N_M_deg, S_nd, S_deg,
per-target counts (raw TSV), wall times. Primary metrics:
 (P1) mismatches N_M != S over (config, target) pairs (headline pass = 0) and N_M_nd total for n_T=0 or m=2 (pass = 0);
 (P2) c_hat = f*q/n_T per config (m=3, n_T>0) and pooled by stratum (bits, n_T, FB), with c_hat/c_ref and c_hat/1.5;
 (P3) NC2 statistic D per cell (section 6).
Uncertainty for f (F4 test), per config: Kish effective sample size over targets ESS_K = (sum N_t)^2 / sum N_t^2, and ratio-estimator
 variance Var(f) = (T/(T-1)) sum_t u_t^2 / (sum N_t)^2 with u_t = M_t - f N_t (T targets); ESS_V = f(1-f)/Var(f). Interval: Wilson with
 (f*ESS, ESS) using ESS = min(ESS_K, ESS_V) when M >= 1; when M = 0 the rule-of-three upper bound f <= 3/ESS_K. Pooled strata use the
 hierarchical bootstrap of section 6 (primes, then curves, then targets) with the same seed. Per-config intervals are NOT used for gates;
 only the hierarchical strata intervals are.
F4 categories (pooled, bits-level, m=3, n_T>0, random+interval+farey FB; twistmult excluded since S=0 by structure and reported separately):
  A: hierarchical 95% interval of c_hat/c_ref inside [0.8,1.25] at 12, 16 and 20 bits -> the 1.5 n_T/q heuristic (with c_ref) is right at all sizes.
  B: at 12 bits the interval excludes [0.8,1.25] but at 16 and 20 bits it is inside -> heuristic right asymptotically, the larger 12-bit constants are finite-size.
  C: otherwise (interval at 16 or 20 bits outside [0.8,1.25]) -> the heuristic is wrong at the sizes of interest; larger constants stand, with the fitted
     c_hat/c_ref table as the answer. Also report: per config, the literal E1 test (Wilson lower bound of f > 5/q) and its count.
  Cells whose pooled expected mixed count (sum of S_ref over targets) is < 5 are UNRESOLVED and excluded from categories (listed).

## 8. Falsifiers (explicit)

F1: any mixed solution (nd or deg) with m=2 for x_R in class E, not 2-torsion (needs G0 and G1 pass to count). Refutes Z.
F2: any nd mixed solution with m=3, n_T=0. Refutes Z.
F3: any target (m=3) with N_M_nd != S_nd or N_M_deg != S_deg (any m=2 mismatch is F1). Refutes the structure; pooled over gcd_odd>1 and =1 strata.
F4: Rt refuted if, in a RESOLVED 20-bit (R20) n_T=3 stratum or the pooled 16- or 20-bit grid strata (random, interval, farey), the hierarchical 95%
    interval of c_hat/c_ref lies entirely above 2 or entirely below 0.5. F4 category C is a refutation of H3 (heuristic wrong), reported separately.
F5 (instrument): G3 fails in any cell: the corresponding zero evidence is inadmissible and the overall outcome for Z cannot be "decisive".
F6 (resolution): R20 or R24 UNRESOLVED: Rt is not tested at that size; outcome for Rt is INCONCLUSIVE (at 20 bits) or "not tested at 24 bits" (R24).

## 9. Outcome rule (fixed now)

DECISIVE SUPPORT of "no non-degenerate mixed solutions when n_T = 0, and mixed rate about c*n_T/q otherwise" iff G0..G5 all pass, F1-F4 do not fire,
 R20 RESOLVED, and total N_E_nd over n_T=0 m=3 configs >= 1e7 (E1 had 4.2e7). If R24 is UNRESOLVED this is reported as an explicit limitation (rate not
 tested at 24 bits), and the support is stated for q up to 2^20.
DECISIVE REFUTATION iff any of F1-F4 fires while G0, G1 pass (F1-F3 need G2/G4 admissibility only for null-count claims, not for counterexamples).
INCONCLUSIVE otherwise (any of G2-G5 failing, or R20 UNRESOLVED), with the failing gates listed. No ECDLP statement either way.

## 10. Freeze and exact commands

Scripts to be written BEFORE any data and hashed (frozen): g3_census.gp (derived from E1 e1_census.gp with these and only these changes: env prefix
G3_; S0 default 20261026; prime stream and curve strata of section 5; stages `oracle`, `controls` (PC worlds bits 12..24, cidx 0..2; NC1; NC3),
`grid`, `nc2` (adds NCT lines with e_t, m_t, n_t, nW per target), `res` (resolution blocks, compact lines); the unchanged functions are listed in
E1 SHA below), g3_aggregate.gp (hierarchical bootstrap, ESS, F4 categories, gates, verdict), plot_g3.py (reads TSV only), run.sh.
Base hashes: e1_census.gp 4f73a03605097f41e998444cc95d2247d865b3b2d211673cf455ebe0d9016e63; e1_aggregate.gp
0c6c40b892f70012ea40b8645a6631f864ab296e369fd72c737f40cb35f7160e. Any later change of a frozen script is a DEVIATION recorded in DEVIATIONS.md.

```sh
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G3-E1-mixed-census-recalibrated-NC2
cd $D && mkdir -p raw && ln -sfn raw out
sha256sum PREREGISTRATION.md > PREREGISTRATION.sha256
sha256sum g3_census.gp g3_aggregate.gp plot_g3.py run.sh > SCRIPTS.sha256 ; cp SCRIPTS.sha256 raw/script_hashes.txt   # before any run
# Stage 0 (G0)
G3_STAGE=oracle G3_SEED=20261026 gp -q g3_census.gp < /dev/null > raw/stage0_oracle.out 2> raw/stage0_oracle.err || exit 1
# Stage 1 (G1, G2, G4)
G3_STAGE=controls G3_SEED=20261026 gp -q g3_census.gp < /dev/null > raw/stage1_controls.out 2> raw/stage1_controls.err || exit 1
# Stage 2 main grid, 2 concurrent workers (4 cores shared)
for b in 12 16 20 24; do K=$(case $b in 12|16) echo 4;; 20) echo 3;; 24) echo 2;; esac)
 for p in $(seq 0 $((K-1))); do for c in $(seq 0 11); do for m in 2 3; do for fb in interval random twistmult farey; do echo "$b $p $c $m $fb"; done; done; done; done; done > raw/joblist.txt
xargs -P 2 -L 1 sh -c 'G3_STAGE=grid G3_SEED=20261026 G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) G3_GUARD_S=120 gp -q g3_census.gp < /dev/null > raw/grid_$0_$1_$2_m$3_$4.out 2> raw/grid_$0_$1_$2_m$3_$4.err' < raw/joblist.txt
# Stage 2b NC2-calibrated (cidx 0..5, alpha 0.65, same targets), 32 cells' units
for b in 12 16 20 24; do K=$(case $b in 12|16) echo 4;; 20) echo 3;; 24) echo 2;; esac)
 for p in $(seq 0 $((K-1))); do for c in 0 1 2 3 4 5; do for m in 2 3; do for fb in interval random twistmult farey; do echo "$b $p $c $m $fb"; done; done; done; done; done > raw/nc2list.txt
xargs -P 2 -L 1 sh -c 'G3_STAGE=nc2 G3_SEED=20261026 G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) gp -q g3_census.gp < /dev/null > raw/nc2_$0_$1_$2_m$3_$4.out 2> raw/nc2_$0_$1_$2_m$3_$4.err' < raw/nc2list.txt
# Stage 2c resolution cells (m=3): "bits pidx cidx fb block ntargets guard"
( for p in 0 1; do for c in 2 5; do for fb in random farey twistmult; do for j in 0 1; do echo "20 $p $c $fb $j 10000 800"; done; done; done; done
  for c in 2 5; do for j in 0 1 2 3 4 5; do echo "24 0 $c random $j 60000 850"; done; done ) > raw/reslist.txt
xargs -P 2 -L 1 sh -c 'G3_STAGE=res G3_SEED=20261026 G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=3 G3_FB=$3 G3_BLOCK=$4 G3_TARGETS=$5 G3_GUARD_S=$6 gp -q g3_census.gp < /dev/null > raw/res_$0_$1_$2_$3_b$4.out 2> raw/res_$0_$1_$2_$3_b$4.err' < raw/reslist.txt
# Stage G5 reproducibility: re-run jobs NR%60==1 of raw/joblist.txt into raw_rep/ and diff TGT lines
mkdir -p raw_rep; awk 'NR%60==1' raw/joblist.txt > raw_rep/joblist.txt
xargs -P 2 -L 1 sh -c 'G3_STAGE=grid G3_SEED=20261026 G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) G3_GUARD_S=120 gp -q g3_census.gp < /dev/null > raw_rep/grid_$0_$1_$2_m$3_$4.out 2> /dev/null' < raw_rep/joblist.txt
for f in raw_rep/grid_*.out; do cmp <(grep '^TGT' $f) <(grep '^TGT' raw/$(basename $f)) || echo "G5 MISMATCH $f"; done > raw_rep/g5.txt   # bash
# Stage 3 aggregation (all gates and the verdict come from files), then plots of recorded values only
gp -q g3_aggregate.gp < /dev/null > summary.tsv 2> raw/aggregate.err
/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python plot_g3.py summary.tsv
```

Budget (estimates from E1 timings, not results): stage 0 about 3 min, stage 1 about 10 min, main grid about 5.6x E1's 25 min wall = about 2.3 h at 2 workers,
NC2 about 40 min, resolution about 1 h, no single job above about 15 min (guards 120/800/850 s on target loops). Peak disk expected < 300 MB; delete raw_rep
bulk after G5 is recorded. If the machine is loaded the run may be split over sessions; seeds make job order irrelevant. Missing or failed jobs are
reported as such and block any gate that depends on them.

## 11. Reporting commitments

RESULTS.md will copy every number from summary.tsv, pooled.tsv, controls_summary.tsv, nc2_cells.tsv, f4_table.tsv, verdict.txt, results.json; state the
E1 quotations as PC1 400/400, PC2 400/400, PC3 800/800; list failed or missing runs; state gate results G0-G5 and the F4 category verbatim; make no ECDLP claim.
