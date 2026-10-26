from j_part1 import *
row(25,
 cl("Cached-separator / shared-left whole-cold median 1.383x; actual GF83 support-only 1.141x (historical Python)", True, "no qualified claim (stated)", "engineering", "L0 (isolation unqualified, 1 CPU/4 GiB per worker)", "no (README: no A/A, no matched genuine F4/F5/F6 panel)", U,
    "3/3 recompute, but the row itself states there is no qualified wall claim; a wall-time ratio on an unqualified host with no A/A cannot validate a speedup."))
row(26,
 cl("find_prime_order_curve plan time 217.6 s -> 0.137 s at 26 bits (1,592x) by stepping one point over the Hasse interval instead of counting points", "partial", "engineering (setup only)", "engineering (algorithmic: O(p) -> O(sqrt p) in curve selection)", "L0 (shared, unpinned)", "no", U,
    "Unvalidated under the tier rule (wall-time speedup at L0). The ratio is an asymptotic, ten-thousand-fold change, far outside any L0 noise, so the direction and order of magnitude are not in doubt; exact seconds are not supported. 7/7 recompute, 1 claim no_raw. Identical-curve claim (288 workload IDs re-plan) is partial: only the committed replan-check record was read, the re-plan was not re-run. No measured result changes."))
row(27,
 cl("Instructions per rho step 781 -> 718 (-8.1%, 1.09x), outputs identical at n=41,53,61", True, "engineering", "engineering", "counts (callgrind Ir, deterministic)", "n/a for Ir", V,
    "Ir is deterministic and recomputes; outputs identical (4/4 comparisons, baseline records absent at n41/n61 so partial there)."),
 cl("Native ns/step 1.008x at n53 and 1.025x at n61", True, "not established (stated)", "null", "isolated_bench reserved CPU 3 on KVM (L1-class), 4/32 contended excluded", "yes: A/A 5.6% (n53), 2.4% (n61)", V, "Row itself does not claim the wall gain; correct."))
row(28,
 cl("Instructions per rho step 938 -> 781 (-16.8%, 1.20x), outputs identical", True, "engineering", "engineering", "counts (callgrind Ir)", "n/a", V, "Recomputes 7/7."),
 cl("Native ns/step 1.173x at n53 (A/A 20.6%), 1.135x at n61 (A/A 12.1%)", True, "not established (stated)", "null", "isolated_bench CPU 3, 0/32 contended (VM)", "yes: A/A 20.6%, 12.1% exceed the gain at n53", V, "Row correctly says not established: the A/A spread is larger than the effect."))
row(29,
 cl("Branch-free Fp64 rho kernel 0.40-0.44x (fp_backend) and 0.45-0.47x (prime_fast) time at B>=4096; rho_parallel 98.9/98.6 -> 68.2/71.1 ns per step (0.69, 0.72)", True, "engineering (9 frozen claims PASS)", "engineering", "L0 (KVM, no PMU)", "yes: unchanged controls within 3%; alternated A1 B1 A2 B2", U,
    "6/6 recompute but the speedup is a wall-time ratio at L0, which cannot support the claim. Direction is very likely real: ~2.3x effect against a 3% control spread, and bit-identical outputs asserted (no raw comparison committed, no_raw_in_repo). Needs a PMU host or a reserved-core rerun to validate; the study itself says 'Repeat on a host with a PMU'. Dependent sub chains slower by 18-33% (controls also moved), honestly disclosed."))
row(30,
 cl("Blocked pair reduction 0.78x on quad_n10, 0.89x on quad_n11 vs row-by-row product (same binary); vs msolve 0.78/0.75x quad_n10/n11, 1.11-1.49x elsewhere", True, "engineering", "engineering", "L0 (shared, unpinned, 3 rounds)", "same-binary control arm (F4_FP_FL_BLOCKED=0); no A/A", U,
    "7/7 recompute; identical bases 54/54. Wall at L0 with 3 rounds and no A/A. msolve is a faster external reference at small n (12-24x per row 115), so the 0.78x 'vs msolve' ratio at quad_n10/n11 would deserve a pinned rerun before any comparison is quoted."))
row(31,
 cl("Shift-run eager rewrite 1.22-1.70x lower median CPU than bit scatter on 6/9 cells; 2-28% slower than BMI2 (stays default)", True, "engineering", "engineering", "L0 (shared, unpinned, CPU time, 5 rounds x 3 reps)", "unchanged-control arms (lazy, scatter)", U,
    "7/7 recompute, 80/80 targets identical (1,600/1,600 field comparisons). Word-operation counts equal BMI2's by construction, which is the load-bearing deterministic claim; CPU-time ratios at L0 are not validated."))
row(32,
 cl("Gate-first F6-IC order: 1.336-1.733x fewer word XORs on 9 cells; median CPU 1.56-2.12x lower", "partial", "engineering", "engineering", "counts for XOR claim; L0 for CPU time", "unchanged inherited-F4 control", C,
    "Word-XOR ratios (deterministic) recompute and decisions match 1,600/1,600: validated as counted work. The 1.56-2.12x CPU claim is wall at L0 (unvalidated); 'worst round pairing >= 1.25' recomputes as 1.2452, i.e. fractionally below 1.25."))
row(33,
 cl("GLV-2 via cheap isogeny-chain endomorphism vs width-w NAF: counted ratio 1.525/1.521/1.492 (model 1.523/1.524/1.494) on Bandersnatch/Tom-384/CP6-782", True, "engineering (variable time)", "engineering", "counts (counted field ops)", "n/a for counts", V,
    "Counted ratios recompute 10/10 and agree with the pre-registered model within 0.31%. Variable-time arithmetic only; Bandersnatch Edwards arithmetic not measured."),
 cl("Timed median declared GLV vs fastest baseline 1.504/1.504/1.570", True, "engineering", "engineering", "L0 (study table: isolated_bench CPU 3, admitted, uncontended, shared VM)", "yes: A/A median ratio 0.997/1.001/1.038", C,
    "A/A floor up to 3.8% on CP6; effect 50%. Direction safe; magnitude carries the VM caveat. Preregistered PROTOCOL.md exists; commit order vs data not provable."))
row(34,
 cl("j=0 staged relation collection rayon-parallel vs serial: 4.4x at 16 bits, 3.0x at 20 bits on 14 threads", None, "engineering (wall scaling)", "engineering (parallelisation)", "L0 (Apple M4 Pro, shared, no isolation receipt, 2 reps)", "no", U,
    "No raw records are committed: only the README table exists (8/8 claims no_raw_in_repo, no PROTOCOL). Cannot be recomputed. A parallel-scaling ratio on a shared host with 2 reps is unvalidated; counted work is stated unchanged but not evidenced."))
row(35,
 cl("Callgrind instructions per rho step 1,196 -> 938 (-21.5%), outputs identical (8/8 callgrind comparisons)", True, "engineering and assurance", "engineering", "counts", "n/a", V, "Recomputes; deterministic."),
 cl("Steps/ideal mean 0.994, 0.922, 0.943 (CI upper 1.044, 0.971, 0.993) on 400 fixtures", True, "assurance", "baseline calibration: the strong signed-Frobenius rho is at or below sqrt(pi r/2A) at n=53, 61", "counts", "n/a", V, "Recomputes; supports using this walk as a strong rho reference."),
 cl("Native 1.023x at n53 (A/A 10.7%) and 1.209x at n61 (A/A 18.3%)", True, "not established (stated)", "null", "isolated_bench CPU 3 (VM), 2/32 contended excluded", "yes", V, "Row states not established: A/A spread >= effect."))
row(36,
 cl("-C target-cpu=native rho kernel 1.6-2.6x faster for fp64 (ratio 0.38-0.63); FpWide no effect at 37/40 sizes", False, "engineering", "engineering (ISA flag)", "L0 (KVM, no PMU)", "alternated A1 B1 A2 B2 runs; no A/A", U,
    "Recomputed upper ratio 0.6030 (claimed 0.63). Wall at L0. Note that global target-cpu flags are discouraged by AGENTS.md section 10 (use runtime dispatch)."),
 cl("Fp64Cm branch-free kernel 0.40-0.50x Fp64 at B>=4096 (F1' PASS); a priori model within 4-8%", True, "engineering", "engineering", "L0", "no", U, "Bit-identical claim has no raw evidence in the repo (no_raw_in_repo). Wall at L0."),
 cl("IC > rho in wall time on every section-5a-ranked curve (Koblitz 4/8, prime 5/6)", True, "not an IC improvement", "baseline/comparison (IC loses)", "campaign host Xeon 2.30 GHz, L0", "n/a", C,
    "Frozen campaign records; negative comparison, direction safe. Matched rho used is the campaign's strong rho. GPU model claims (17.9% median error over 46 builds) rest on a committed derived table only."))
row(37,
 cl("isogeny_algos vs FLINT 3.0.1 and PARI 2.15.4: ours/fastest-external 0.42-0.95 (127-bit products), FLINT faster at 61/511-bit for small n; Phi_l 5.1x vs PARI; SEA 127-bit PARI 1.18x faster", True, "external comparison", "engineering vs the strongest external libraries (reported honestly in both directions)", "isolated_bench one CPU, 121 runs, 0 contended, two rounds order reversed (VM, L1-class)", "no explicit A/A; order-reversed rounds", C,
    "9/9 recompute and 562/562 digest/order comparisons equal. The best-reference rule is respected: ratios are taken against the fastest external per row and losses are reported. Magma not run. VM neighbours outside the tool's reach."))
row(38,
 cl("Phi_l (l<=89) over 2^127-1: 9.4-9.6 s -> 2.3 s (4.1x)", False, "engineering", "engineering", "L0 wall (shared 4-vCPU VM, min of two)", "no", U,
    "Recomputed main 9.569-9.742 s, branch 2.256-2.416 s, min/min 4.24x: the quoted 9.36-9.57 s starting times do not match the committed records. Wall at L0 anyway."),
 cl("SEA point count over 2^127-1: 1.14x geomean (0.82-1.42)", False, "engineering", "engineering (noise-level)", "L0", "no", U,
    "Recomputed geomean 1.125 (1.4% off) but the range is 0.71-1.72 over 24 paired records, wider than the quoted 0.82-1.42, and the sums are 2.258 s -> 1.991 s versus the quoted 2.12 -> 1.81 s. The study itself reports 10-30% single-run variation, so a 1.1x geomean at L0 is inside noise; numbers also do not match the committed records."),
 cl("NTT 10-11x Karatsuba at n=8192 (bit-identical output)", None, "engineering", "engineering", "L0", "no", U, "Bit-identical claim: records carry only verified flags (no_raw_in_repo)."))
row(39,
 cl("Calibrated hwcost model: median |error| 1.8-5.5% on B>=4096, 5/5 PASS (a priori 4/5, fp64 fails at 54.2%)", True, "model validation", "model calibration (fitted residual per field)", "L0 (KVM, no PMU)", "no", C,
    "Recompute 1.749-5.49%. The calibration fits one residual per field on the same host's small batches and tests on larger batches on the same host; not an independent validation. The a priori model fails on fp64, reported."))
row(40,
 cl("Fixed-degree isogeny pipeline: 6/6 planted prime d found; Coron faster than Cornacchia in 7/8 pairs at 61 bits; Strassen 0.42/0.26/0.24x naive; CSURF-512 action 25.8 ms", True, "end-to-end demonstration", "engineering/descriptive", "L0 (2 vCPU container shared with the build, one run per setting)", "no", U,
    "10/10 recompute, but 'Coron faster in 7/8' and wall times are single runs on a host shared with the build: unvalidated as speed claims. No comparator tool or rho involved."))
