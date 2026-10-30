# E6-genus2-gluing: results

Language PARI/GP 2.15.4. Frozen scripts (sha256 verified unchanged: `sha256sum -c out/SHA256SUMS.txt`, 0 failures). Driver: `run.sh`. Every number below is in `out/summary.txt`, `out/totals.txt`, `out/wilson_totals.txt`, `out/diag_distinct_j.txt` or `out/rc.txt`. Seeds: setrand(20261021+p) main, setrand(20261021+p+7) controls.

## Outcome
Prediction confirmed. P1-P8 all held; none of F1-F6 fired. H1-H3 (Galois-module isomorphism of E[2], E'[2]; non-trivial equivariant phi gives C: Y^2=f(a x^2-mu) whose Jacobian has #Jac=#E*#E' and charpoly (T^2-tT+p)(T^2+tT+p)) are verified in the tested range. H4 (injective pullback E->Jac, odd parts) is NOT computed here (no Jacobian group law), only #E | #Jac. H5 (no gain) is a cost-model statement, not a measurement: no genus-2 index calculus was implemented.

## Controls (run first)
Unit: count of curves / pairs. Wilson 95% intervals.
- PC1 positive control (random split sextics, charpoly = product of quotient charpolys): 300/300 at p=101,211,409,1009 and 100/100 at p=4001; 1300/1300 pooled, Wilson lower 0.99705. Detector sees the effect.
- NC1 different-2-division-pattern pairs flagged isomorphic: 0/300 at p=101,211,409,1009 (upper 0.0126), 0/100 at p=4001 (upper 0.0370).
- NC2 null (random squarefree sextics, hit = charpoly of the form T^4+(2p-t^2)T^2+p^2, t^2<=4p): 6/3000 (p=101, Wilson [0.00092,0.00436]), 3/3000 (211, [0.00034,0.00294]), 2/2000 (409, [0.00027,0.00364]), 0/1000 (1009, [0,0.00383]), 0/200 (4001, [0,0.0188]; essentially no power). All point estimates under the P8 thresholds (3%, 1.5%, 1%).
- NC3 identity permutation: g_2=0 (no curve) for 221/221 curves.

## Main results (sample unit: elliptic curve E over F_p; record unit: (curve, non-identity phi, arm))
Curves tested: 221 (p=101:56, 211:60, 409:45, 1009:30, 4001:15, 10007:12, 20011:3), uniformly random with j not in {0,1728}, distinct j within (p, pattern).
- Stage A (P1): all six checks pass 221/221 (Wilson lower 0.983).
- B0 (P2): n_equiv = 3/2/6 and equals the number of F_p-rational interpolants in 221/221.
- B1 (P3): identity gives no curve 221/221; g2zero 0, singular f(-mu)=0: 0, sextic not squarefree: 0 (bound allowed a few).
- C target arm (P4): 580 records; match_target, nJ_eq_prod, quotients_ok, nE_div_nJ, ident_ok all 580/580 (Wilson lower 0.9934).
- C control arm (P5): match_control 580/580; nJ_eq_prod = 11/580, and all 11 are curves with t=0 (nE=nEt), the registered exception.
- Brute force (P6, p=101 4 curves/pattern, p=211 3 curves/pattern, both arms): brute_eq_pari 112/112 (Wilson lower 0.967); N1=p+1 on all 56/56 target-arm records.
- Cost model (P7; model-derived, not measured): per-p median of c_tie_full = sqrt(pi*#E/2)/p: 0.12471 (p=101), 0.086486 (211), 0.062500 (409), 0.039181 (1009), 0.019812 (4001), 0.012508 (10007), 0.0088727 (20011). Max never exceeds 1.2533(sqrt p+1)/p at any p (e.g. 10007: 0.012650 vs 0.012654). Log-log slope of median vs p = -0.5005 (7 primes; predicted in [-0.6,-0.4]). Median at p=10007 0.012508 <= 0.0135. With l = largest prime factor of #E, the median c_tie slope is -0.7360 (no prediction registered). nJ/p^2 lies within [0.988,1.020] at p=101 and within [1.0000452,1.0000918] at p=20011.
- Reading against the success criterion: the constant is a model distance to a tie at the exponent-1 index calculus on a group of order about p^2; it is a gap to matched rho, not a pass/fail gate, and no arm is ranked at finite p (AGENTS.md 5a pricing at 0, 0.1, 1 GAE not applicable: no counters measured). The exponent gap 1/2 (p vs sqrt(l)) is the only weight-independent statement.

## Deviations from the preregistration (none silent)
1. Driver is `run.sh`, not `run_all.sh`: same commands, seeds, grid; order changed so the controls (positive control first) run before the main runs, as the task requires. Each p is seeded independently, so order does not change any number. `run_all.sh` was not executed.
2. p=101: the grid asked for N=20 per pattern but only 16 distinct j-invariants with pattern [1,1,1] exist (out/diag_distinct_j.txt: 34, 49, 16 distinct j for [3], [1,2], [1,1,1]; p=211: 70, 105, 34). The frozen sampler loops forever on the 17th curve, so `main p=101` ended by the 900 s timeout (rc=124, no DONE line) after printing all records for 56 curves (20+20+16); the 64 brute-force records (4 curves/pattern) were complete. Not rerun; reported as 56 curves. The unfinished sampler search produced no data.
3. p=20011 has only 3 curves (N=1 per pattern, as registered); it finished (rc=0).
4. Post-hoc helper scripts added (no change to frozen ones): e6_totals.sh (sums counts), e6_wilson.gp (v1: its last two n=0 rows errored by design; v2 drops them, see DEVIATIONS.md), e6_diag_p101.gp (diagnostic syntax repaired before use), plot_e6.py (figures out/ctie_vs_p.png, out/passrates.png from summary.txt). Smoke files out/SMOKE_notdata_* are excluded from all numbers.
5. Single-seed run; no repeat. Wall-clock not measured (not a metric).

## Limits
Verification covers p<=20011, the model family Y^2=f(a x^2-mu), 3 to 20 curves per pattern and p. No Jacobian group law, no rho or index-calculus arm on Jac(C); H4 and H5 not measured. Mathematical content (the construction) is a derivation checked on 221 curves, not a proof.

## Operation counts (primary cost metric on the shared machine; derived by `e6_opcounts.sh` from the raw files, `out/opcounts.txt`)
Unit: counts. Model columns are the task's cost model (rho steps sqrt(pi*l/2) per curve, l = largest prime factor of #E; "full" uses l = #E; exponent-1 index calculus on Jac(C) = p group operations with constant 1), NOT measured group operations. The measured-in-the-sense-of-executed counts are the evaluation counts: each is an exact function of the logged records and of the loop structure of e6_main.gp / e6_controls.gp.

| p | curves | candidate root-perm tuples | Stage-C records (both arms) | brute-force records | degree-3 poly evals over F_p (Stage C, 2p per record) | degree-6 poly evals over F_{p^2} (p^2 per brute record) | model rho ops, median, largest prime l | model rho ops, median, l=#E | model IC ops / rho(l=#E) ops, median |
|---|---|---|---|---|---|---|---|---|---|
| 101 | 56 | 532 | 280 | 64 | 56560 | 652864 | 4.157 | 12.6 | 8.019 |
| 211 | 60 | 580 | 320 | 48 | 135040 | 2137008 | 5.168 | 18.25 | 11.56 |
| 409 | 45 | 435 | 240 | 0 | 196320 | 0 | 6.749 | 25.56 | 16.00 |
| 1009 | 30 | 290 | 160 | 0 | 322880 | 0 | 8.025 | 39.53 | 25.52 |
| 4001 | 15 | 145 | 80 | 0 | 640160 | 0 | 10.26 | 79.27 | 50.48 |
| 10007 | 12 | 116 | 64 | 0 | 1280896 | 0 | 15.30 | 125.2 | 79.95 |
| 20011 | 3 | 29 | 16 | 0 | 640352 | 0 | 17.32 | 177.6 | 112.7 |
| total | 221 | 2127 | 1160 | 112 | 3272208 | 2789872 | | | |

Further exact totals (out/opcounts.txt): degree-6 evaluations over F_p in the brute force 16592 (p per brute record); hyperellcharpoly calls in Stage C 1160 (one per record) plus 1300 (PC1) plus 9200 accepted NC2 samples (9240 NC2 sampler tries); PC1 degree-3 evaluations 1838200; NC1 pairs 1300. Controls totals include p=101,211,409,1009 (300 PC1 samples each) and p=4001 (100).
Not counted: field multiplications inside PARI (hyperellcharpoly, ellcard, factormod, matsolve), the F_{p^2} Legendre exponentiation per value, and the unlogged rejection-sampling attempts of the curve sampler. No group operations on Jac(C) or E are executed anywhere in this experiment (no Cantor arithmetic), so "group operations" exist only as the model columns. Reading: at every p the model exponent-1 index calculus on Jac(C) needs more group operations than rho on the target subgroup (ratio 8 at p=101, 113 at p=20011, growing like sqrt(p) at the full-group l=#E worst case, faster with l the largest prime factor); this is the distance to a tie under the model, not a measured ranking (AGENTS.md 5a pricing of unpriced native work is not applicable because none is measured).
Wall time (secondary, contention-affected, 4 cores at load average about 15 shared with other agents): original full run, from file times, about 30 minutes (15:49 to 16:19); reduced replication 1611 s (see below). Not a metric.

## Independent post-hoc cross-checks (not preregistered; `e6_xcheck1-3.gp`, outputs `out/xcheck/`)
1. xcheck1 (seed 777; 25 fresh random curves per p at p=101,211,409,1009; scans every mu in F_p of the model Y^2=f(x^2-mu), both classes of a): for every curve the number of mu whose second quotient has j(E2)=j(E) equals the number of non-identity equivariant isomorphisms (2, 1, 5 for patterns [3], [1,2], [1,1,1]) and all tested Jacobians satisfy #Jac in {#E*#E', #E^2}: count_mismatch=0, bad=0 for all four primes (100 curves). This checks H2 from the opposite direction (all mu, not the interpolation).
2. xcheck2: exhaustive distinct-j counts over all (A,B): 34/49/16 at p=101 and 70/105/34 at p=211 for patterns [3],[1,2],[1,1,1], and 16/34 by the Legendre lambda formula for [1,1,1]; confirms the p=101 deviation.
3. xcheck3: re-counts #E and #E' by Legendre-symbol summation for every curve at p=4001,10007,20011 (30 curves); all equal the logged nE, nEt, t and sum to 2p+2; median c_tie_full 0.0198117 / 0.0125081 / 0.00887266 and max below the Hasse bound, equal to the main-run summary.

## Reproduction (single driver `run.sh`)
- Full: `bash run.sh` (about 30 min wall under contention; the p=101 main run always ends at its 900 s timeout).
- Reduced replication: `E6_REDUCED=1 E6_OUT=out_reduced bash run.sh`: controls p=101,211,409,1009 and main p=409,1009,4001,10007,20011 (105 curves, 560 Stage-C records). Measured 1611 s wall (about 27 min) at an observed load average of 11-15 on 4 cores. Every one of its 9 raw files (main_p409,1009,4001,10007,20011 and ctrl_p101,211,409,1009) is byte-identical (`cmp`) to the full-run file of the same name.
- Quick subset: `E6_REDUCED=2 E6_OUT=out_quick bash run.sh`: controls p=101,211,409,1009 and main p=409,1009,4001 (90 curves, 240 Stage-C records). Measured 334 s wall under the same contention; all 7 raw files byte-identical (`cmp`) to the full run; pooled Wilson 95% lower bounds 0.959 (90/90 curves), 0.984 (240/240 Stage-C target records), 0.9968 (1200/1200 PC1).
- Re-derive summary, totals, Wilson intervals, operation counts and cross-checks from existing raw files in seconds: `E6_POSTONLY=1 [E6_OUT=dir] bash run.sh`.
- Integrity: `sha256sum -c out/SHA256SUMS_v1_orig.txt` (frozen files; only `run.sh` differs because it was revised to v2, original kept as `run_v1.sh`) and `sha256sum -c out/SHA256SUMS_v2.txt` (all raw files and the v2 scripts), both run from this directory.
- Deviations: `DEVIATIONS.md`.
