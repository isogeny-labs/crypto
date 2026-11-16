# E3-eigen-decomposition-Fq2: results

Protocol: PREREGISTRATION.md (frozen); deviations and implementation choices: DEVIATIONS.md. Driver: `run.sh` (57 s wall in the original run; all gp runs rc=0, `out/timing.txt`). Script hashes: `SCRIPTS.sha256`, verified `out/scripts_check.txt`. gp 2.15.4. Every number below is in `out/analysis.out`, `out/results.csv` or `results.json`.
Scope: two index-calculus arms (direct Semaev decomposition in E(F_p) vs the F_{p^2} eigen-decomposition route) compared in Semaev work units (root-solves x degree). No cross-method ranking against rho, no GAE pricing, no leaderboard claim. This is not an improvement of the IC baseline; the route is slower.

## Curves (`out/curves.out`)
Odd family (a,b,#E,#E'): p=101 c1 (51,91,97,107); c2 (2,36,97,107); p=211 c1 (57,128,199,225); c2 (33,145,193,231); p=401 c1 (170,292,397,407); c2 (242,53,373,431). Even-order (H4): p=101 (1,27,96,108); p=211 (97,172,216,208).

## Controls (positive first)
- PC1 (group-law coherence, `out/lemma.out`): 6 curves, 0 mismatches in each of: #E(F_{p^2})=#E*#E', generator orders, pi=+1/-1 on generators and on 28 factor-base points, and 50 random pair sums (full point and x-coordinate). 0 of 300 pair sums mismatched.
- PC2 (Semaev vs table, `out/semaev.out`): cells (101,12), (101,16), (211,16), curve 1, 10 targets each: D_2 and X_4 counts equal the table on 30/30 targets, and the x-set sets are identical (0 set mismatches). Note the (211,16) cell has only 1 D_2 and 1 X_4 relation in total (low power).
- PC3: D_2/D_3 mean yield within [0.8,1.25] of model Y_D in 24 of 24 cells with model Y_D >= 0.5.
- NC1: group-law audit, odd curves: 1,965,600 decompositions, 85,854 with R in E(F_p), 2,952 of those mixed (both E and T terms): 0 violations of (2B=O, B=O, A=R). Table version (X hits with B=0 whose E-subset is not a direct D relation): 0.
- NC2 (B redrawn, permutation test per cell): 0 of 36 cells with p<0.01 (chance expectation 0.36); smallest p = 0.106. No coding artifact detected.
- NC3 (E-only factor base, B nonzero): 0 hits in all cells.
- Other consistency: X count = D_j*T_k in every cell (0 mismatches); work counters equal the formulas in every cell; sign-vector multiplicities (rare): 2582 over all cells.

## Primary: cost per E-relation, ratio rho = Z_X / Z_D (T=200 targets per cell, ratio of totals, paired bootstrap B=2000 seed 9001, 99% interval)
Cells: 36 (Family A 24, Family B 12). Powered cells (D total >= 20): 33. Unpowered: A p=211 c1 N=16 (D=11), B p=211 c1 N=12 (D=0, model undefined since N_E=2<m), B p=211 c1 N=16 (D=5). Totals over all cells (including unpowered): Family A D=9021, X(j=1..m)=77900 relations; Family B D=4745, X(j=1..m)=113128 (sums of the results.csv columns).
- rho_gen over the 33 powered cells: minimum 53.3 (B, p=101 c2 N=12; 99% CI 37.4 to 74.0), maximum 2812 (A, p=401 c1 N=16; CI 1332 to 11200). The lowest interval end over all powered cells is 27.3 (B, p=211 c1 N=20). Ranges of rho_gen per curve over powered cells (A: p/c): 101/1 133 to 162; 101/2 123 to 163; 211/1 154 to 233; 211/2 222 to 311; 401/1 759 to 2812; 401/2 248 to 487. (B): 101/1 119 to 136; 101/2 53.3 to 130; 211/1 60.9 (one powered cell); 211/2 174 to 234.
- rho_exact (j=m) over powered cells: 237.8 to 2812.
- Control X_m (same term count as baseline): rho 20.1 to 665 over powered cells (3 cells with zero X_m yield, reported as rule-of-three lower bounds). All-j X yield ("rho_all", every X relation counted as one relation): 51.9 to 816.5 over powered cells. Best-N per curve: best Z_Xgen / best Z_D between 60.9 and 759 (`out/analysis.out`, secondary section). The "joint (E+T) pair" cost is accounting only (relabelling) and does not change the ordering.
- rho_gen / rho_model over powered cells: 0.705 to 2.40.
- Semaev wall time (CPU, shared machine, 3 cells): us per root-solve D_2 = 2.4, 4.6, 6.0; X_4 = 286.8, 370.2, 369.8; ratio X_4/D_2 = 119.5, 81.0, 61.6.

## Prediction outcomes (as registered)
- P1: held. 33 of 33 powered cells have rho_gen >= 2, and the 99% interval lower end is >= 2 in all 33. H1 falsifier (interval entirely below 1): 0 cells; 0 powered cells have an interval spanning 1. H1 supported.
- P2: held. 33 of 33 powered cells have rho_gen within a factor 3 of rho_model (threshold 80%). The mental m=2 guess (10^2 to 10^3 for Family A): measured powered Family A cells range 122.8 to 2812; all are >= 10^2 and 3 cells (p=401 c1 N=16, 24, 32: 2812, 1682, 1072) exceed 10^3.
- P3: held, 33 of 33 (rho_exact >= rho_gen; holds by construction since the exact yield is a subset of the generous yield with the same work).
- P4 (H2): pooled mean j/m2 over all 235,155 X relations = 0.442 (powered cells only 0.444), inside [0.35,0.65]; in each of 36 cells within 0.15 of N_E/N. H2 held.
- P5 (H3): 0 exceptions (every E-part of an X relation is a direct relation of A with the same j E-terms). Held.
- P6 (H4): odd curves 0 violations in 1,965,600 decompositions. Even-order curves: B != O (and A != R) in 2860 of 655,200 decompositions (1664 for p=101, 1196 for p=211; 54,748 decompositions with R in E(F_p)), 2B != O never. Both halves held.
- P7: held. PC2 0 mismatches; X_4 per-solve time exceeds D_2 by 61.6 to 119.5 times (threshold 4).

Outcome: prediction confirmed in every registered component, with these limits: a Semaev work-unit comparison on toy fields (p=101..401, N<=40, 200 targets per cell); 3 of 36 cells unpowered; the Semaev PC2 cells have small relation counts; wall times are from a shared 4-core machine and single runs.

## Operation counts (primary cost metric; `out/opcounts.csv`, `results.json` key `op_counts`; DEVIATIONS.md items 7-8)
Derived by `e3_opcounts.gp` from the recorded `out/results.csv` (no new experiment). Machine-independent. Unit: univariate root-solves (the Semaev step), "work" = root-solves x degree (the frozen unit), and the table arm's sign-vector group sums (one per candidate x-set and sign vector). The script's work counts equal T*WD and T*WX of the frozen run in all 36 cells (0 mismatches).
- Over the 33 powered cells, total root-solves: direct D = 93,000; F_{p^2} route X = 36,296,000 (work 258,800 vs 662,348,800). Over all 36 cells: D = 94,400 root-solves, X = 37,440,000; table-arm group sums D = 10,044, X = 25,629,568.
- Per E-relation, X (j=1..m) over D, powered cells: root-solves per relation ratio 6.66 to 703 (the direct arm solves only E-type x; the X arm must solve over all N values of x, at 2^(m2-1) degree); work ratio 53.3 (B, p=101 c2 N=12) to 2812 (A, p=401 c1 N=16), identical to rho_gen; table-arm group-sum ratio 93.3 to 2031. All three ratios are > 1 in all 33 powered cells.
- Example (A, p=101 c1 N=16, T=200): D = 168 E-relations from 1,400 root-solves (2,800 work, 8.33 solves/relation); X = 351 relations from 112,000 root-solves (896,000 work, 319.1 solves/relation); ratio of work per relation 153.16 (= rho_gen).
- The 3 Semaev PC2 cells (10 targets): D_2 root-solves 2500, 3500, 1500; X_4 root-solves 2200, 5600, 5600 (`out/semaev.csv`; X_4 uses C(N,3) solves of degree 8 over F_{p^2}).
- Not measured: field multiplications and polynomial-evaluation counts (the frozen scripts did not instrument them). The wall-time ratio of X_4 over D_2 per solve therefore stays a secondary, contention-dependent number.

## Replication
A second full run of `run.sh` (into `replicate/`, then again with the final `run.sh`) gives bit-identical counts, `results.csv`, and `results.json` base content; only wall-clock fields differ. Semaev per-solve ratio X_4/D_2: original 119.5, 81.0, 61.6; first replicate 80.8, 77.9, 80.6; final replicate (load average about 14 on 4 cores, `replicate/out/analysis.out`) 177.3, 97.9, 101.5. Single runs on a shared machine, so the spread is contention; all exceed the registered threshold of 4. Compared with the original after removing the wall-time fields, `results.json` (including `op_counts`) is identical. Reduced replication subset (about 20 s of gp): `e3_curves.gp`, `e3_model.gp`, `e3_lemma.gp`, `e3_semaev.gp`; the full `run.sh` takes about 1 to 2 minutes (57 s on the original quiet run).

## Notes
- results.csv rows with value -1 mark undefined quantities (e.g. zero D yield; the model row for p=211 c1 B N=12 has D yield 0, so its rho_model is meaningless and excluded from P2).
- `e3_rho_vs_N.png/.svg` plots all cells including the unpowered ones (plot script was hashed before the run, so the unpowered cells were not excluded and the y label is clipped); `e3_jfrac.png/.svg` plots mean j/m2 vs N_E/N.
- Development test scripts and the p=61 toy-grid outputs are in `dev/` (dev/o2 csv files deleted, see DEVIATIONS.md item 6) (see DEVIATIONS.md item 3); they are not used in any reported number.
