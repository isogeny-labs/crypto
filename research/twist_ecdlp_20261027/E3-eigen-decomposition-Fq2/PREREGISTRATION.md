# Preregistration: E3-eigen-decomposition-Fq2

Written 2026-10-21 before any experimental data. Only toolchain sanity was run (`sanity.gp`, output `sanity.out`): gp 2.15.4 starts, `ellinit` over `ffgen(p^2)` works, and for p=101, E:y^2=x^3+3x+5 it printed #E=115, #E'=89, gcd=1, #E(F_{p^2})=10235=115*89. The curve is only a toolchain check and is not in the grid. `gp -q file.gp` stays interactive unless the script ends with `quit`; every script below does.

Directory: `/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E3-eigen-decomposition-Fq2` (all paths below are relative to it).

## 1. Hypotheses

Setting: p prime, E/F_p: y^2=x^3+ax+b with #E=p+1-t and #E'=p+1+t both odd, gcd(#E,#E')=1. Factor base V={0,1,...,N-1} in F_p. Every x in V gives a point of E(F_p) when f(x)=x^3+ax+b is a nonzero square (E-type, count N_E) and a point of E'(F_p) when f(x) is a non-square (T-type, count N_T=N-N_E). Odd #E excludes f(x)=0. In E(F_{p^2}) a T-type point is (x, s*sqrt(d)) with f(x)=d*s^2, d a fixed non-residue, and Frobenius pi acts as -1 on it.

H1 (cost, the prediction under test). Take a target R' = A + B in E(F_{p^2}) with A uniform nonzero in E(F_p) and B uniform nonzero in E'(F_p). Decompose R' with m2=2m terms with x in V. Projecting by (1+pi)/2 and (1-pi)/2 gives an E-relation for A and a T-relation for B. Then the cost per useful E-relation of this route exceeds the cost of direct Semaev decomposition of a target R in E(F_p) with m terms over the same V.

H2 (term count). The F_{p^2} route needs about 2m terms, since only the E-type terms (about m2*N_E/N of them) contribute to the E-relation.

H3 (structural dominance). Every E-relation the F_{p^2} route yields is an E-relation of A that direct decomposition in E(F_p) also finds with j<=m2 terms. The route requires the twist part to match B simultaneously, which direct decomposition of A does not. The expected yield is therefore the product of two independent hit probabilities (about 1/#E times 1/#E').

H4 (separation lemma, as stated by the task). If R in E(F_p) and R=sum eps_i P_i with each P_i of E- or T-type, then the twist part B has 2B=O, so B=O for odd order, and the E-part A equals R. I expect this to hold exactly for odd #E and to fail only when rational 2-torsion makes a nonzero B possible (A=R-B then differs from R).

## 2. Numeric prediction

Let l=#E (prime), l'=#E', j = number of E-type terms, k = m2-j.

Model expected yields per target, in x-sets (unordered, pairwise distinct x):
- Direct: Y_D(m) = C(N_E,m) * 2^m / l.
- F_{p^2}: Y_X(m2,j) = C(N_E,j) * C(N_T,m2-j) * 2^m2 / (l*l'). The sign factor is the full 2^m2 and j=0 is excluded.

Model work, in the Semaev unit "univariate root-solves times degree":
- W_D(m) = C(N_E,m-1) * 2^(m-1). The direct solver enumerates only E-type x, which is free knowledge once y is computed.
- W_X(m2) = C(N,m2-1) * 2^(m2-1). It must enumerate all of V because both types are needed.

Model cost per useful E-relation is Z = W/Y. The generous X yield sums j=1..m, and the like-for-like X yield uses j=m. The model ratio is rho_model = Z_X/Z_D.

Registered predictions, in order of strength:
- P1: In every powered cell (section 7), measured rho_gen = Z_X(j in 1..m)/Z_D(m) is >= 2. The F_{p^2} route is slower per E-relation, not merely "about twice".
- P2: rho_gen lies within a factor of 3 of rho_model in at least 80% of powered cells. I did a mental check of one cell only (p=211, N=24, m=2: rho_model about 340), so I expect the m=2 family to sit in the 10^2 to 10^3 range. This is a prediction, not a result.
- P3: rho_exact = Z_X(j=m)/Z_D(m) is >= rho_gen in every cell where both are finite.
- P4 (H2): pooled over X relations, mean j / m2 lies in [0.35, 0.65], and in each cell it lies within 0.15 of N_E/N.
- P5 (H3): 100% of E-parts extracted from X relations are direct relations of A found by D_j (0 exceptions).
- P6 (H4): 0 violations for odd #E. For a separately chosen curve with even #E, the number of violations is > 0.
- P7: Semaev-solver relation counts (D_2 and X_4, small cells) equal the group-law table counts exactly, 100% of targets. X_4 wall time per solve is above D_2 wall time per solve by at least the degree ratio 4 (deg 8 over F_{p^2} versus deg 2 over F_p).

## 3. Baseline

Baseline arm D_m is direct Semaev decomposition in E(F_p): the target R=aP+bQ (log uniform in Z/l, nonzero) is decomposed into m E-type terms of V, m in {2,3}. The unit is the x-set with at least one sign vector summing to R, the same unit as the Semaev solution S_{m+1}(x_1..x_m,x_R)=0 with distinct roots in V. The comparison arm X_{2m} is the F_{p^2} route with the same V. Two controls: X_m (m terms in F_{p^2}, same term count as baseline) and the lemma audit.

The Semaev route to be implemented for the wall-time arm: S_3(x1,x2,x3)=(x1-x2)^2 x3^2 - 2((x1+x2)(x1 x2+a)+2b) x3 + ((x1 x2 - a)^2 - 4b(x1+x2)). S_{n+1} is built by resultants, S_4 symbolically once over F_p, S_5(x1..x4,X)=Res_u(S_3(x1,x2,u),S_4(x3,x4,u,X)) numerically per tuple. For D_2 the last variable is solved from a quadratic over F_p. For X_4 the root problem is degree 8 over F_{p^2} with x_R' in F_{p^2}, and roots are kept only if they lie in V. This is the Weil restriction to two F_p equations, done implicitly by solving over F_{p^2}.

Cross-method ranking against rho is out of scope: this experiment compares two index-calculus arms only. No native-work pricing (0/0.1/1 GAE) is claimed and no matched-rho gap is reported. Any such statement is not made.

## 4. Controls

Positive controls (the detector can see the effect):
- PC1 (group-law coherence). For each curve, check that the map x -> point on E_T: Y^2=X^3+a d^2 X+b d^3 (X=dx, Y=d^2 s) agrees with the F_{p^2} group law. On 50 random pairs, the F_{p^2} sum of lifted points has x-coordinate equal to the one predicted from the (E,E_T) logarithm pair. Also check #E(F_{p^2})=#E*#E' and pi acts as +1 / -1. Expect 0 mismatches.
- PC2 (Semaev versus ground truth). On cells (p=101,N=12), (101,16), (211,16), curve c=1, T_S=10 targets: the Semaev solver's x-set count equals the table count per target, for D_2 and X_4. Expect exact equality on every target.
- PC3 (yield detector). The direct arm D_2 measured mean yield is within a factor 1.25 of the model Y_D in at least 80% of cells with model Y_D>=0.5. This confirms the counting instrument can see a known expected yield.

Negative controls:
- NC1 (separation null). For R in E(F_p) (B=O) the X_4 table contains only decompositions whose twist-part sum is O, whose E-part sum is R, and whose x-sets are direct decompositions of R with <=4 E-terms (plus twist-null padding). Tested exactly, expect 0 exceptions on odd-order curves.
- NC2 (B-independence). Repeat the X_4 yield count with B replaced by an independent uniform draw. The j-distribution and E-relation yield per target must not differ in distribution from the main arm (two-sample test on per-target yields, alpha=0.01). A difference means a coding artifact.
- NC3 (E-only factor base). Run X_{2m} restricted to V_E. A relation then needs j=m2 and B=O, so yield for B nonzero must be 0 exactly.

## 5. Metric

Primary: cost per useful E-relation Z = (total W)/(total relation count) over T targets in a cell, in Semaev units (root-solves times degree). Reported with the ratio rho = Z_X/Z_D, using ratio-of-totals with a paired bootstrap (resampling targets, B=2000, seed 9001, 99% interval). Zero-yield arms: Z is infinite; the ratio is reported as a lower bound using the rule of three, yield upper bound 3/T per target.

Secondary:
- Wall time per Semaev solve and per relation, Semaev cells only (CPU time, `getabstime`).
- Mean j / m2 and the j-histogram.
- Joint cost per (E-relation + T-relation) pair. This is accounting only and is flagged as a relabelling, not an improvement.
- A "best-N" comparison: min over the N-grid of Z for each arm, per curve.
- The exact (j=m) like-for-like ratio rho_exact.
- Counts of sign-vector multiplicities (expected to be rare).

## 6. Falsifier

H1/P1 is falsified, for that cell, if the 99% bootstrap interval of rho_gen lies entirely below 1 (the F_{p^2} route is cheaper per E-relation than direct) in any powered cell. If any cell does this the headline claim "slower per E-relation" is reported as false for that cell and the cell is named. It is supported if no powered cell does. It is "weakly supported" if some cell has an interval spanning 1.

H2/P4 is falsified if the pooled mean j/m2 lies outside [0.35, 0.65].

H3/P5 and H4/P6 are falsified by a single exception (odd order, deterministic claims). P6's second half (>0 violations on even-order curve) is falsified if there are zero violations on a curve that has a rational 2-torsion point and mixed decompositions with B != O exist in a table of at least 10^5 decompositions.

P7 is falsified by any mismatch in PC2, and in that case the Semaev solver, not the claim, is treated as broken and its wall times are not reported.

The result is "inconclusive" when no cell is powered.

## 7. Parameter grid

- Curve selection (per p, c in {1,2}): `setrand(1000*p + c)`; draw (a,b) uniformly in F_p^2, keep the c-th one with #E prime, #E != p, #E' != p, #E' odd, gcd(#E,#E')=1, ellgroup(E') cyclic, 4a^3+27b^2 != 0. Record (a,b,#E,#E',t).
- Family A (m=2; D_2, X_4, X_2): p in {101, 211, 401}; N in {16, 24, 32, 40}; T=200 targets; Semaev wall-time cells: (101,12), (101,16), (211,16), curve 1, T_S=10, with timeout per run of 900 s.
- Family B (m=3; D_3, X_6, X_3, tables only, no Semaev solver): p in {101, 211}; N in {12, 16, 20}; T=200.
- A cell is "powered" when the D arm total yield over T targets is >=20 relations.
- Even-order curve for H4: one curve per p in {101,211}, chosen by the same seed rule but requiring #E even and a rational 2-torsion point.
- Seeds: curves `1000*p+c`; targets `7000+10*p+c` (A uniform in Z/l minus 0, B uniform in Z/l' minus 0, drawn once per cell and shared by all arms: paired design); bootstrap `9001`; PC1 draws `4242`.
- Limits: one gp process per (p,c), at most 2 concurrent processes, each under 900 s via `timeout`. If a run times out, the cells it did not finish are reported as missing, with no extrapolated numbers. No Rust crate is planned.

## 8. Method (frozen before data)

Group-law ground truth. Compute discrete logs of E-type points in Z/l (generator from `ellgen`/`elllog`) and of T-type points in Z/l' on E_T. A decomposition's E-part sum is an integer sum mod l and its T-part sum an integer sum mod l'. For m2 terms enumerate all C(N,m2) x-sets, and for each the 2^m2 sign vectors. Index a count array by (u mod l, w mod l', j), counting each x-set once per key (deduplicate within an x-set). Target hit = array lookup at (A, B). Direct tables use the Z/l sums over E-type subsets only. Work counters follow the formulas in section 2 and are also incremented by the code, and the two must agree.

Semaev solver (Family A small cells only). For each target and each (m-1)-subset of V enumerated in increasing order, build the univariate polynomial in the last variable, find roots in F_p (D) or in F_{p^2} (X), keep roots in V that are distinct from the other x's, and add the resulting x-set (dedupe by sorting). Record counters and `getabstime`.

## 9. Exact commands (scripts are written next, SHA-256 frozen in `SCRIPTS.sha256` before the first data run)

Common prefix: `cd /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E3-eigen-decomposition-Fq2`. Every gp call is `gp -q -f script.gp > out/NAME.out 2>&1` with `default(parisizemax,2000000000)` set inside and `quit` at the end. Prefix `timeout 900`.

1. `sha256sum e3_*.gp > SCRIPTS.sha256` (after writing the scripts, before step 3).
2. `timeout 900 gp -q -f e3_curves.gp > out/curves.out 2>&1`: selects curves, writes `out/curves.csv`; then `e3_model.gp` writes `out/model.csv` (rho_model per cell, from curve parameters only, before any table run).
3. `timeout 900 gp -q -f e3_lemma.gp > out/lemma.out 2>&1`: PC1, NC1, H4 odd and even-order audit.
4. For each p in 101 211 401 and c in 1 2: `P=$p C=$c timeout 900 gp -q -f e3_tables.gp > out/tables_p${p}_c${c}.out 2>&1` (Family A, then `FAMILY=B` for p in 101 211). Contains NC2, NC3.
5. `timeout 900 gp -q -f e3_semaev.gp > out/semaev.out 2>&1` (PC2, wall time).
6. `gp -q -f e3_analyze.gp > out/analysis.out 2>&1`: bootstrap, ratios, falsifier tests, writes `out/results.csv`.
7. Plots (only recorded values): `/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python plot_e3.py out/results.csv`.

## 10. Reporting rules

Every reported number comes from a file in `out/`. Missing, timed-out or failed runs are stated as such. Prediction outcomes are reported cell by cell against P1-P7 as registered. Any deviation from this plan (changed grid, skipped cell, changed definition) is logged in `DEVIATIONS.md` with the reason, and results from deviated parts are labelled exploratory.
