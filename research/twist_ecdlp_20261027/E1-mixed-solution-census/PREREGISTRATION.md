# PREREGISTRATION E1-mixed-solution-census

Status: FROZEN before any data. Written 2026-10-21. No census result exists yet.
Directory: /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E1-mixed-solution-census
Only files produced so far: this file and `toolchain_sanity.gp` / `toolchain_sanity.txt`
(gp 2.15.4; setrand reproducible; an identity check on 3660 pairs, see section 11).

## 0. Claim classification (controlled-experiment skill)

- Exact algebra (derived here, then checked by computation): the separation lemma and the
  discriminant identity of S_3 (section 2).
- Empirical claim tested by the census: "in Semaev decomposition x-tuples of a factor base
  V, solutions of S_{m+1}(x_1..x_m, x_R)=0 are E-only; mixed (some x_i on the twist) solutions
  are essentially absent (fraction ~ 1/q)".
- Not claimed: anything about the cost of ECDLP. A negative census says the twist does not add
  relations to a plain Semaev census for R in E(F_q); it is not an improvement over index
  calculus and no speedup is asserted.

## 1. Setting and definitions

E: y^2 = f(x) = x^3 + a x + b over F_q, q prime (odd, > 3). Twist E_d: d y^2 = f(x), d the least
positive non-residue mod q. R in E(F_q) with R not in E[2] (so 2R != O).
Class of an x in F_q:
- E: f(x) a nonzero square. T2: f(x)=0 (rational 2-torsion, lies on both E and E'). W: f(x) a non-square (twist point).
- "E-only solution" = an unordered multiset {x_1..x_m} of V with S_{m+1}(x_1..x_m,x_R)=0 and no x_i of class W.
- "Mixed solution" = same, with at least one x_i of class W.
- Non-degenerate: x_i pairwise distinct and no x_i = x_R. Otherwise degenerate. Both are counted, separately.
- n_T = |E[2](F_q)| - 1 in {0,1,3}, i.e. the number of roots of f in F_q.
- Semaev: S_3(x1,x2,x3) = (x1-x2)^2 x3^2 - 2[(x1+x2)(x1x2+a)+2b] x3 + (x1x2-a)^2 - 4b(x1+x2);
  S_4(x1,x2,x3,x4) = Res_z(S_3(x1,x2,z), S_3(x3,x4,z)).
  Zero over F_q of S_{m+1} with x_i in F_q  <=>  exist points P_i in E(F_qbar) with x(P_i)=x_i and
  sum eps_i P_i = O for some signs; each P_i lies in E(F_q) (class E,T2) or E'(F_q) (class W,T2).

## 2. Theory derived before data (the sharper prediction)

Lemma (stronger than the task's version, no coprimality needed). Let a solution have E-part
A = sum over E-class indices (including +-R) and twist part B = sum over W-class indices, A+B=O in
E(F_{q^2}). Apply Frobenius pi (+1 on E(F_q), -1 on E'(F_q)): A-B = O. Hence 2A = 2B = O and
A = B = -B is a rational 2-torsion point T in E[2](F_q). If #E and #E' have no common odd
factor this is the task's "B=O for odd order, A=R". Our derivation needs no coprimality; the
grid therefore includes curves with gcd_odd(#E,#E')>1 as a test of that assumption.

Consequences (exact, for R not in E[2]):
(C1) m=2: mixed solutions do not exist at all (degenerate or not). Independent proof:
     disc_{x2} S_3(x1,x2,x_R) = 16 f(x1) f(x_R) (identity, checked in section 11); f(x_R) is a
     nonzero square, so for x1 of class W the quadratic in x2 is irreducible over F_q: no F_q root.
(C2) m=3, n_T=0: mixed solutions are only degenerate, of the single form {x,x,x_R} with x of class W
     and x_R in V (P - P + R - R). No non-degenerate mixed solution.
(C3) m=3, n_T>0: additionally, for each nonzero T=(e,0) in E[2](F_q) with tau_T(x) := e + f'(e)/(x-e)
     (= x(P+T)), mixed solutions {x1, x2=tau_T(x1), x3=tau_T(x_R)} with x1,x2 in W∩V, x3 in V
     (two twist points differing by T plus an E point equal to R+T). Fixed points tau_T(x1)=x1 give
     degenerate {x,x,tau_T(x_R)}. Nothing else.
     Structural predictor S_nd(target), S_deg(target) = exact counts of these tuples, computed by
     an independent code path ("Oracle S") from V, x_R and the roots of f, never from S_4.
     Heuristic size: E[S_nd] ~ n_T * n^3/(4 q^2) versus E-only ~ n^3/(6q), so mixed/E-only ~ 1.5 n_T/q
     (a q-scaling, i.e. the task's "about 1/q", but only when rational 2-torsion exists).
Competing random model H_rand (what "mixed would be if the lemma were absent"): zeros of S are
independent of class, so mixed fraction ~ 1 - 2^-m (0.75 for m=2, 0.875 for m=3). The census must
distinguish "~0 / ~1.5 n_T/q" from H_rand; this is a very large effect.

## 3. Hypotheses

- H1 (lemma/structure): for every config, every target: N_mixed(m=2)=0; for m=3, N_mixed_nd = S_nd and
  N_mixed_deg = S_deg exactly (so 0 non-degenerate when n_T=0).
- H2 (task prediction, headline): mixed fraction f = N_mixed_nd/(N_mixed_nd+N_Eonly_nd) <= 5/q in every
  config (0 for n_T=0 or m=2; ~1.5 n_T/q for m=3, n_T>0). Resolvable only where N_Eonly_nd >= 4q
  (otherwise reported as "unresolved: interval wider than 1/q", not as pass).
- H3 (E-only baseline): mean E-only per target matches the baseline below within a factor [0.7,1.4] for
  the random FB; interval, farey, twistmult FBs are structured and are reported as ratios, not gated.

## 4. Baselines

- Naive (task): B_naive = n^m/(m! q) E-only solutions per target (n=|V|).
- Refined: B_ref = 2^m * nE^m / (m! * #E) with nE = |V ∩ class E| (derivation: each ordered E-tuple has 2^m
  sign vectors, each hits R with prob 1/#E). Both are reported; ratios obs/B with bootstrap intervals.
- Mixed baseline: B_mixed = 0 (H1) with the structural S as exact predictor; and H_rand as the
  foil (fraction 1-2^-m) used in the negative control (section 6).
- Cross-method: this is not an ECDLP cost comparison, so no GAE/rho pricing applies; no rho arm.

## 5. Parameter grid

- bits in {12,16,20,24}; prime_idx in {0,1} (24 bits: prime_idx 0 only) -> 7 primes.
  q = nextprime(2^(bits-1) + random(2^(bits-1))) under setrand(W_p), W_p = S0 + 100000*bits + 1000*prime_idx,
  prime_idx 1 re-drawn until different from prime_idx 0.
- Curves per prime (curve_idx): 0: n_T=0; 1: n_T=0 with gcd_odd(#E,#E')>1 (if none found in 5000
  draws, falls back to another n_T=0 curve, flagged); 2: n_T=1; 3: n_T=3. Curve drawn by
  setrand(W_c), W_c = W_p + 10*(curve_idx+1), a,b uniform, disc != 0. A missing stratum is recorded as MISSING.
- m in {2,3}.
- Factor bases V (size n; x as elements of F_q; no class filtering, so ~half of V is on the twist):
  interval [0,n); random (n distinct uniform x, seed W_c+31); twistmult (x(kG), k=1..n/2 for G random
  on E, and x(kG')/d, k=1..n/2 for G' random on E_d, deduped; seed W_c+41); farey (first n distinct
  u/w mod q, gcd(u,w)=1, ordered by u+w, then u). The actual |V| and nE are recorded.
- Sizes: n = min(ceil(q^alpha), cap), alpha in {0.5, 0.65, 0.8}, cap = 16384 (m=2), 1500 (m=3); duplicates
  after capping are run once.
- Targets: 200 (m=2), 100 (m=3) per (curve, FB, alpha); R = aP+bQ with P,Q random points of E(F_q),
  a,b uniform in [1,#E]; reject R=O or 2R=O. Target seed W_c+7, so the same R's are used for every FB,
  alpha and m on a curve (paired design).
- Seeds: S0 = 20261021. Bootstrap seed S0+1, B=2000 resamples of targets. Every run logs its seeds.
- Time guard: 60 s of target loop per config (table build not counted); targets are processed in fixed
  order, so truncation is non-selective; truncated configs are reported with the number of targets done.

## 6. Detectors, oracles, controls

Detector A (production, fast), m=2: for each x1 in V, class by Kronecker symbol; roots of the quadratic
S_3(x1,x2,x_R) in x2 by sqrt of the discriminant; tuple kept if x2 in V. m=3: pair table of V (all i<=j):
for each pair the roots of S_3(xi,xj,z) as keys (rational root value; infinity; or, for an irreducible
quadratic, its monic coefficient pair), sorted integer vector, bisection lookup; per target, for every
x3 in V the keys of S_3(x3,x_R,z) are looked up. Output: each solution multiset, class vector,
degenerate flag.
Oracle B (brute force, independent of A): direct evaluation of S_3 on all pairs (m=2) and of
S_4 = Res_z(S_3(.,.,z),S_3(.,.,z)) via the closed resultant (a1c2-a2c1)^2 - (a1b2-a2b1)(b1c2-b2c1) on all
unordered triples (m=3) of V. Run only where n <= 120.
Oracle G (group law, independent of Semaev): explicit points on E and E_d (x' = d x), all sign vectors,
criterion from the lemma: A = T and B = T' for some T in E[2](F_q) (T' the matching point on E_d). Run only
where n <= 120.
Oracle S: structural predictor of section 2 (C3).
Stage 0 acceptance (V0): at bits=12, prime_idx 0, all 4 curves, all 4 FBs, n=96, m=2 and 3, 20 targets:
A, B, G produce identical solution sets (with classes), and Oracle S equals mixed counts. Any
disagreement stops the experiment; no later number may be reported until fixed and V0 passes.

Positive controls (sensitivity must be exactly 100%):
- PC1 (planted non-degenerate mixed, needs n_T>=1, m=3): for each of 50 fresh P' in E_d(F_q) and each such
  curve, set V = base V (n as in grid) U {x(P'), x(P'+T), x(R+T)}: the twist zero-sum P'+(P'+T)... i.e.
  (P'+T) - P' - T = O among twist points, with E point R+T closing it. Detector A must report that
  tuple, class (W,W,E), flagged non-degenerate mixed, and Oracle G must confirm.
- PC2 (planted degenerate mixed, any curve): V U {x_R, x(P')}, tuple {x(P'),x(P'),x_R}, must be reported as degenerate mixed.
- PC3 (planted E-only): P1,P2 in E(F_q) random, P3 = R - P1 - P2, V U {x(P1),x(P2),x(P3)}: reported as non-degenerate E-only (m=3);
  m=2 with P2 = R - P1.
Negative controls:
- NC1 (E-only factor base): V = x(kG), k=1..n (all class E). Mixed must be 0 and E-only must equal the
  A/B/G values (no false mixed classification).
- NC2 (instrument can see mixed zeros when the structure is absent): evaluate S_{m+1} with b replaced by b+1
  (a different curve) on the same V, same classification by the original f. There is no lemma, so the
  mixed fraction must be consistent (Wilson 95%) with 1-2^-m. Failure means the classifier or the join cannot
  see mixed zeros, voiding "0 observed" results.
- NC3 (target on the twist, mirror check): run with x_R of class W (R' random in E'(F_q), not in E[2]) at
  bits 12 and 16, m=2 and 3. The lemma with roles swapped predicts the mirror image: W-only solutions at
  the baseline rate, "mixed" (some x_i of class E) count 0 for m=2 and, for m=3, equal to the same
  structural predictor with E and W swapped. Purpose: show the result is not an artefact of the target being on E.

## 7. Metric

Per (bits,prime,curve,FB,alpha,m): n, nE, nW, #E, #E', gcd_odd, n_T, targets completed, N_E_nd, N_E_deg,
N_M_nd, N_M_deg, S_nd, S_deg (m=3), per-target counts (raw TSV), wall time.
Reported: (i) mixed fraction f = N_M_nd/(N_M_nd+N_E_nd): exact Clopper-Pearson upper bound 1-0.025^(1/N)
when N_M_nd=0, Wilson 95% otherwise; (ii) obs/B_naive and obs/B_ref of mean E-only per target with 95%
bootstrap (percentile) intervals over targets; (iii) exact-match indicator N_M == S per target;
(iv) for n_T>0, m=3: ratio f*q/n_T with Wilson interval.
Primary metric: the count of mixed solutions N_M_nd (and the mismatch count N_M != S over targets). Headline
pass: number of mismatches = 0.
Pooling: only by stratum (m, n_T, FB), summing counts; intervals recomputed on pooled counts. No
post-hoc subsets, no dropped configs; MISSING/truncated reported as such.

## 8. Falsifiers (explicit)

F1 any mixed solution (degenerate or not) with m=2, R not in E[2], confirmed by Oracle B and G: the lemma /
discriminant claim is falsified for that config.
F2 any non-degenerate mixed solution with m=3, n_T=0, confirmed by B and G: the separation lemma
is falsified in that setting (or the Oracle G criterion is wrong); stop and investigate before any other claim.
F3 any target where N_M != S (m=3): the structural account (C2/C3) is incomplete.
F4 H2: a config with N_E_nd >= 4q and f > 5/q (Wilson lower bound > 5/q) falsifies "about 1/q".
F5 H_rand wins: pooled mixed fraction for any stratum has Wilson interval containing 1-2^-m and excluding 5/q.
F6 V0, PC1-PC3 sensitivity < 100%, NC1 or NC2 failing: the instrument is invalid; no result is reported
as evidence for or against anything (reported as a failed experiment).
Supporting, not falsifying the lemma: H3 failure means the baseline formula is wrong, not the lemma.

## 9. Commands (exact; scripts written after this freeze, hashed before the first run)

Files to be created in the directory: `e1_census.gp` (stages oracle|controls|grid, params from environment),
`e1_aggregate.gp` (reads raw/*.tsv, writes summary.tsv), `plot_e1.py` (reads only summary.tsv).
```
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E1-mixed-solution-census
cd $D
sha256sum e1_census.gp e1_aggregate.gp plot_e1.py > raw/script_hashes.txt        # before any run
# Stage 0: oracle agreement V0
E1_STAGE=oracle E1_SEED=20261021 gp -q e1_census.gp < /dev/null > raw/stage0_oracle.out 2> raw/stage0_oracle.err
# Stage 1: positive and negative controls
E1_STAGE=controls E1_SEED=20261021 gp -q e1_census.gp < /dev/null > raw/stage1_controls.out 2> raw/stage1_controls.err
# Stage 2: grid (<=2 concurrent jobs; each job <=3 configs x ~85 s)
for bits in 12 16 20 24; do for pidx in 0 1; do
  if [ $bits = 24 ] && [ $pidx = 1 ]; then continue; fi
  for curve in 0 1 2 3; do for m in 2 3; do for fb in interval random twistmult farey; do
    echo "$bits $pidx $curve $m $fb"; done; done; done; done; done > raw/joblist.txt
cat raw/joblist.txt | xargs -P 2 -L 1 sh -c 'E1_STAGE=grid E1_SEED=20261021 E1_BITS=$0 E1_PIDX=$1 E1_CURVE=$2 E1_M=$3 E1_FB=$4 E1_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) E1_GUARD_S=60 gp -q e1_census.gp < /dev/null > raw/grid_$0_$1_$2_m$3_$4.out 2> raw/grid_$0_$1_$2_m$3_$4.err'
# Stage 3: aggregation and plots (plot only recorded values)
gp -q e1_aggregate.gp < /dev/null > summary.tsv 2> raw/aggregate.err
/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python plot_e1.py summary.tsv
```
Every script begins with `default(parisizemax,2000000000); setrand(seed)`. Stop rules: stage 0 or 1 failure
halts everything (F6); if cumulative stage-2 wall time passes 4 h, stop and report the remaining
configs as NOT RUN. Any deviation is appended, with reason, to DEVIATIONS.md before reporting. No Rust, no main-repo build.

## 10. Reporting rules

Every number in the final report is copied from summary.tsv or raw/*. Failed, truncated, MISSING or not-run
cells are listed as such. The result is described as support/non-support for H1-H3 with the intervals; no
claim of attack improvement or cost reduction. Sonnet 5.5 / Haiku 5.5 sub-agents may execute the frozen
commands verbatim; a director assesses the output against sections 7-8 only.

## 11. Pre-data toolchain evidence

`toolchain_sanity.txt` (written by `toolchain_sanity.gp`): gp 2.15.4, setrand reproducible, and the identity
B^2-4AC == 16 f(x1) f(x2) held on every one of the 3660 sampled pairs for the single toy curve q=1009,
a=3, b=7 (a check of the algebra used in C1, not a result of the census).
