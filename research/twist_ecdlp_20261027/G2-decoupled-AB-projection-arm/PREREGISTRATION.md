# Preregistration: G2-decoupled-AB-projection-arm (E3 rerun)

Written 2026-10-26 before any data. Only toolchain checks were done (gp 2.15.4 starts; E3 scripts e3_lib.gp / e3_grid.gp / e3_curves.gp were read, nothing run). Directory: `/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G2-decoupled-AB-projection-arm` (paths below relative to it; E3 = `../E3-eigen-decomposition-Fq2`, read-only, its `e3_lib.gp`, `e3_grid.gp`, `e3_curves.gp` are re-used unmodified and hashed). Language: PARI/GP only; Python only to plot recorded values. No Rust, no main-repo build.

## 0. Setting and the verifier issues addressed

E3 priced only the coupled route, in which one x-set must hit A and B together (yield = Y_D(j)*Y_T(k) by construction, work = a full enumeration over all of V). This rerun adds: (a) the decoupled projection arm; (b) a same-term-count direct arm D_{2m}; (c) positive controls showing the detector can read rho < 1.

Notation (as E3): l=#E (prime), l'=#E'; V={0..N-1}, N_E E-type, N_T=N-N_E T-type; m in {2,3}, m2=2m; j E-terms, k T-terms. Semaev work unit (root-solves x degree), from E3 section 2: W_D(j)=C(N_E,j-1)2^(j-1); W_T(k)=C(N_T,k-1)2^(k-1) (same enumerator on the twist E_T: Y^2=X^3+a d^2 X+b d^3, X=d x); coupled W_X(m2)=C(N,m2-1)2^(m2-1). Yields per target: Y_D(j)=#{E-type x-sets of size j with a sign vector summing to A in Z/l}; Y_T(k) likewise for B in Z/l'.

Target classes (both drawn from the same seeded stream, shared by all arms, paired):
- Class S (subfield): R = A in E(F_p), B=O. This is the ECDLP-relevant target.
- Class F (full): R' = A+B in E(F_{p^2}) with A, B uniform nonzero (as E3).

## 1. Arms (all use the same V, same targets, same work unit and same wall-clock harness)

1. D_m: direct decomposition of the class-S target with m E-type terms (baseline).
2. P (decoupled projection): compute A=(R'+pi R')/2, B=(R'-pi R')/2 (actual F_{p^2} arithmetic, timed). Class S: B=O (odd #E'), so A=R and the arm is D_j on A with j in {m..2m} E-terms and no twist term. Class F: find A-relations with D_j (E-type x only) and B-relations with T_k (T-type x only) separately, work W_D(j)+W_T(k), j+k=m2 for (j,k) in {(m,m)} and the sweep j=1..2m-1, k=m2-j; a relation of R' is a pair (A-set, B-set), x-sets disjoint automatically.
3. D_{m2}: same-term-count direct arm, m2=2m E-type terms in E(F_p) over V_E (target class S). Also reported: every D_j, j=m..2m, and "best direct" D* = argmin_j Z_D(j) over j in [m, 2m] (model Z_D(j)=l*j/(2(N_E-j+1)), increasing in j, so D* is expected to be D_m; checked, not assumed).
4. X (coupled, E3): work W_X(m2), yield Y_D*Y_T (recomputed by the same table code, plus read back from E3 `out/` as a cross-check where cells coincide).

## 2. Hypotheses and numeric predictions (all registered before data)

H1 (class S, ECDLP-relevant). The P arm on class-S targets has the same relation sets and work as D_j. Predicted rho_E(P,S) = Z_P/Z_D = 1.000 exactly on every cell (0 count differences over all targets): the projection is a no-op for R in E(F_p), so the twist route offers an exact tie, no gain, at best. Wall ratio P/D >= 1 (projection overhead; magnitude not predicted).

H2 (class F, independent E-relation metric). Z_E = (W_D(j)+W_T(k))/Y_D(j), cost per independent A-relation, all work charged (twist work is overhead for an ECDLP on E). Then rho_E(P,F) = 1+W_T(k)/W_D(j) > 1 exactly (yield cancels). With j=k=m: 1+N_T/N_E, expected 1.5 to 3 across the grid. With k=1 (j=m2-1=2m-1 is not like-for-like; the like-for-like cell is j=m, k=1, m+1 terms): rho = 1+1/W_D(m), about 1.03 to 1.15 for m=2, N in 16..40. So the closest decoupled approach to 1 is predicted from above, at distance about 1/W_D(m), and it brings no E-relation beyond D_m.

H3 (pair-count metric, accounting only). Counting each (A-set,B-set) pair as a relation of R': Z_pair=(W_D(j)+W_T(k))/(Y_D(j) Y_T(k)). Predicted: Z_pair/Z_D(m) < 1 in at least one powered cell (m=2, j=k=2, large N, where Y_T(2) > 1+W_T/W_D), and Z_pair is below the coupled Z_X by the factor (W_D+W_T)/W_X(m2) (< 0.2 in every cell). This is a relabelling: pairs are products of Y_D independent E-relations and Y_T independent T-relations, so rank is bounded by Y_D (E part) and Y_T (T part). Rank check: number of distinct A x-sets and distinct B x-sets and rank of the stacked 0/1 incidence matrix of A-sets reported per cell; the claim "rank <= Y_D" must hold exactly. The pair metric is NOT counted as an improvement and cannot satisfy the success/falsifier criteria below.

H4 (same-term-count control). Z_D(m2) > Z_D(m) in every powered cell (cost per relation grows with terms in the Semaev unit); best twist arm vs D_{m2}: rho_E(P,F,j=k=m)/ (Z_D(m2)/Z_D(m)) reported as a second corrected rho. Not predicted to cross 1.

H5 (E3 conclusion robust). On the independent E-relation metric, no twist arm (P class S, P class F, X) has a 99% curve-level interval for rho entirely below 1 in any powered cell. The E3 conclusion (twist route does not reduce the cost of E-relations) is robust to decoupling. Coupled rho from E3 (53.3 to 2812, rho_gen) is expected to be reproduced to the same range (instrument check against E3 results.csv, ratio within [0.9,1.1] cell by cell, since same seeds and same code).

H6 (wall time). Wall per root-solve: X_4 over F_{p^2} >> D_2 (E3: 61.6 to 119.5). Decoupled class F arm per solve: D_2 on E plus T_2 on E_T, each about the D_2 time, so its per-relation wall ratio vs D_2 is about 1+N_T/N_E for j=k=m, plus projection overhead. Predicted decoupled/coupled wall ratio < 0.1 in cells where the coupled arm was timed. Magnitudes are exploratory; only the orderings (P > D_m >= 1; P < X) are predicted.

## 3. Baseline

D_m on class-S targets on E3's six curves (p=101,211,401 x c=1,2: (a,b,l,l') = p101: (51,91,97,107),(2,36,97,107); p211: (57,128,199,225),(33,145,193,231); p401: (170,292,397,407),(242,53,373,431) as recorded in E3 RESULTS.md; recomputed by `e3_curves.gp` seeds 1000*p+c, and the script aborts if they differ). Extension curves c=3,4 per p (same seed rule, stream continues) are labelled exploratory extension and are not in the primary interval.

## 4. Controls

Positive controls (detector can see rho < 1 and rho = 1):
- PC1 injection: replace the baseline work by 2*W (handicapped) and 0.5*W (boosted) after the run; rho_P must come out 0.500 and 2.000 (exact arithmetic, tolerance 1e-9), and the 99% interval must exclude 1 on the right side in every powered cell. Shows the interval logic reports rho < 1 when it is < 1.
- PC2 forced B=O: class-S targets (B=O forced) through the full projection code path, F_{p^2} arithmetic included; rho_E must be 1.000 and relation sets identical to D_j (0 differences). Wall ratio recorded.
- PC3 real cheaper-route cell: pair-count metric (H3) on cells where Y_T(2)>2; at least one cell must show Z_pair/Z_D(m)<1 (the detector sees a real <1). Plus "extreme-twist" curves: per p in {101,211,401}, the curve of the stream `setrand(31000+p)` with prime #E, odd #E', gcd=1 and smallest #E' among the first 400 admissible draws (record a,b,#E,#E',t); same arms and grid. Expected still rho_E > 1 (Hasse prevents tiny #E' over F_p: #E' >= p+1-2*sqrt(p)); this is registered as a prediction, not a guarantee that a cheaper regime exists.
- PC4 counting instrument: the D_m measured mean yield within [0.8,1.25] of model Y_D in >= 80% of cells with model Y_D>=0.5 (as E3 PC3).
- PC5 Semaev vs table: D_2, T_2, X_4 solver relation sets equal the table sets on every target in cells (101,12),(101,16),(211,16), curve 1, 10 targets; any mismatch voids the wall-time results.

Negative controls:
- NC1 lemma: for 200 class-S targets per curve (R in E(F_p)), the projection returns B=O and A=R in F_{p^2} arithmetic: 0 violations expected; for the even-order curves of E3 (p=101: a,b=1,27; p=211: 97,172) violations are expected > 0 (B != O possible), reported as the boundary of the claim.
- NC2 B-independence: redrawing B with an independent seed leaves rho_E(P,F) unchanged exactly (it is B-independent by construction) and changes Y_T-dependent pair-count only within sampling noise; two-sample test alpha=0.01.
- NC3 timer null: D_m timed against an identical second copy of D_m (same code, interleaved); the wall ratio's 99% interval must contain 1; its half-width defines the minimum detectable wall effect.
- NC4 E-only factor base: restrict P class F to V_E; relations need k=0 and B=O, so class-F yield for B != O is exactly 0.

## 5. Metric

Primary: rho_E = Z_arm / Z_D*, Z = total work in Semaev units / number of independent E-relations (distinct A x-sets) over the targets of the cell, using ratio of totals. Where yield is 0, Z is infinite and rho is reported as a lower bound by the rule of three (yield upper bound 3/T per target). Secondary: pair-count rho (H3, labelled relabelling); rank of the incidence matrix; rho vs D_{m2}; coupled rho (reproduction of E3); wall time per solve and per relation; distance to 1 (rho-1) for every arm. No GAE or matched-rho number is claimed or reported: any cross-method statement requires the AGENTS.md 5a band (0, 0.1, 1 GAE per native operation, ranking only where the order holds across the band), which this within-index-calculus experiment does not compute; rho here is arm-versus-arm in the Semaev unit only and the success criterion (verified measured improvement over the baseline) is assessed only on that.

Uncertainty: 99% intervals by resampling CURVES with replacement (primary set: the 6 E3 curves; B=10000, seed 9002), pooled ratio of totals within each (family, p-class, N) cell and for the pooled grid. With only 6 (or 12 with extension) curves these intervals are coarse and conditional on the curve family; a secondary within-curve target bootstrap (T=200, B=2000, seed 9001, 99%) is reported next to it. Cells share curves, targets and factor-base prefixes (nested V), so cells are strongly correlated; no per-cell p-values are combined, interval counts are not treated as independent tests, and no multiplicity adjustment is claimed beyond the pre-stated falsifier of one failing cell.

## 6. Falsifier (explicit)

- H1 falsified in a cell if the P-arm and D_j counts differ on any class-S target (rho_E != 1.000 exactly).
- H5 (E3 robust) is falsified if on the independent E-relation metric any powered cell has a 99% curve-level interval for rho_E (best twist arm vs D*) entirely below 1; then that cell is named and E3's conclusion is reported as not robust there. If all arms and cells are >= 1: robust; if the lowest decoupled class-F value is within 0.2 of 1, the distance rho-1 is reported (H2 predicts about 1/W_D(m) for k=1).
- H2 falsified if measured rho_E(P,F) deviates from 1+W_T/W_D by more than 1e-9 (counters are exact) or is <= 1.
- H3 falsified if no powered cell shows pair-metric rho<1 (then PC3 failed and the detector's sensitivity to rho<1 rests on PC1 only) or if rank exceeds Y_D.
- NC1 odd-order falsified by a single violation. PC1 failure (rho != 0.5/2.0) voids the interval code.
- Wall-time claims withdrawn if PC5 mismatches or NC3's interval half-width exceeds the reported effect.
- Result "inconclusive" for a cell if the D arm total yield over T targets is < 20 ("powered" rule as E3).

## 7. Parameter grid and seeds

- Curves: E3's six (primary); c=3,4 extension; extreme-twist curves (`setrand(31000+p)`); even-order curves of E3 for NC1.
- Family A (m=2, m2=4): p in {101,211,401}; N in {16,24,32,40} (E3 grid) plus N in {48,64} for p in {101,211} (decoupled and D arms only; coupled X only for N<=40 as E3). T=200 targets per cell.
- Family B (m=3, m2=6): p in {101,211}; N in {12,16,20}; tables only, T=200 (D_3, D_6, P, X_6).
- (j,k) sweep for class F: all j=1..m2-1.
- Wall cells (Semaev solver, D_2, T_2, P=D_2+T_2+projection, X_4, D_4 via numeric S_5 with resultants): (101,12),(101,16),(211,16),(211,24),(401,16), curve c=1, T_S=10 targets; X_4 only in cells with N<=16. 7 interleaved repeats per arm per cell (arm order permuted by `setrand(5100+rep)`), medians and minima reported, `/proc/loadavg` and `nproc` logged before each repeat; machine shared (4 cores, another agent), D_4 or X_4 cell stopped at 600 s, missing cells reported as missing.
- Seeds: curves `1000*p+c`; extreme curves `31000+p`; targets `7000+10*p+c` (A,B as E3, shared across arms, same as E3 so the coupled arm is comparable); NC2 B redraw `8000+10*p+c`; bootstrap `9001` (targets) and `9002` (curves); timing order `5100+rep`; PC NC1 draws `4242`.

## 8. Exact commands

Prefix: `cd /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G2-decoupled-AB-projection-arm`; `mkdir -p out`. Every gp call: `timeout 900 gp -q -f SCRIPT > out/NAME.out 2>&1`, each script begins `default(parisizemax,2000000000);` and ends with `quit;`, at most 2 gp processes at once (4 shared cores).

1. Write scripts `g2_curves.gp` (reads E3 curves.gp rule, adds c=3,4 and extreme curves, asserts E3's six), `g2_tables.gp` (arms D_j, T_k, P, X, rank, NC2, NC4, PC1 injection inputs), `g2_lemma.gp` (NC1, PC2), `g2_semaev.gp` (PC5, wall time, NC3), `g2_analyze.gp` (curve-level and target-level bootstrap, falsifier tests, `out/results.csv`), `plot_g2.py`, `run.sh`.
2. Freeze: `sha256sum PREREGISTRATION.md g2_*.gp plot_g2.py run.sh ../E3-eigen-decomposition-Fq2/e3_lib.gp ../E3-eigen-decomposition-Fq2/e3_grid.gp ../E3-eigen-decomposition-Fq2/e3_curves.gp > SCRIPTS.sha256` before step 3; `run.sh` verifies it with `sha256sum -c` and refuses to run on mismatch.
3. `timeout 900 gp -q -f g2_curves.gp > out/curves.out 2>&1`
4. `timeout 900 gp -q -f g2_lemma.gp > out/lemma.out 2>&1`
5. `timeout 900 gp -q -f g2_semaev.gp > out/semaev.out 2>&1` (positive control PC5 and NC3 before any table arm is analysed)
6. For p in 101 211 401, c in 1 2 3 4 and extreme curve, family A (and B for p in 101 211): `P=$p C=$c FAMILY=$f timeout 900 gp -q -f g2_tables.gp > out/tables_p${p}_c${c}_${f}.out 2>&1`
7. `timeout 900 gp -q -f g2_analyze.gp > out/analysis.out 2>&1`
8. `/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python plot_g2.py out/results.csv .`; then `rm -rf` any `target/` (none expected).

## 9. Reporting rules

Every number reported comes from a file in `out/`; failed, timed-out or skipped runs are reported as missing with no extrapolation. Outcomes are listed against H1-H6 and controls as registered. Deviations go in `DEVIATIONS.md`; deviated parts are exploratory. The success criterion is applied as stated: a relabelling (pair-count metric, joint E+T accounting) is not an improvement; only a verified rho_E < 1 on the independent E-relation metric counts, and each cell's gap to 1 is reported as the distance remaining.
