from j_part2 import *
row(41,
 cl("Best psi-chain DLP S = 1.47 [1.28, 1.69]/sqrt(N) vs +-BSGS 0.955 [0.86, 1.05] at 22 bits (ratio 1.54): chain is worse", True, "negative result", "null/negative (constant)", "L0 (taskset one core, another experiment running)", "no (null-vs-null control failed at 16 bits in item 08)", C,
    "Counted S; negative result, direction safe. 6/10 recompute, 3 cannot_define (intervals), 1 partial. psi x-evaluation 454 ns vs addition 462 ns (c_E 0.98) is a wall number at L0 and is correctly described as 'negative, constant'. Item 06 slope p^2.02 vs rho p^0.52 is a negative extrapolation ('projected relation stage ~4e6x rho'). A broad campaign: 12/13 red-team upheld, item 14 refuted."))
row(42,
 cl("Binary MITM oracle vs Gaudry pairs-and-solve: median 1.746 ms vs 1.62 s (928x) at l=11 and 8.566 ms vs 18.7 s (2,187x) at l=12", False, "engineering", "engineering (decomposition-stage oracle, constant 2^l factor)", "L0 (arm64 macOS, shared, contended by other builds)", "no", U,
    "Recomputed l=12: 8.063 ms MITM and 2,321x (claimed 8.566 ms and 2,187x): the committed per-target medians do not give the quoted ones. Wall at L0 under contention. Counted point-adds 8,068/16,092 vs 2.1M/8.4M quartic solves are the real evidence; identical answers 128/128 and group-verified witnesses recompute. No exponent and no rho claim, as stated."))
row(43,
 cl("Montgomery sqrt-elu codomain 5.58x (256-bit) / 5.27x (511-bit) Velu at l=10007; crossover ~l=150", "partial", "engineering", "engineering (algorithmic: sqrt(l) vs l)", "L0 (shared VM, one run per setting)", "no", U,
    "5/9 claims recompute from committed p13/p14 result files named in REPORT; 2 cannot_define, 1 no_raw. The l^(1/2) vs l scaling is algorithmic and the 5x order is expected at l=10^4, but single-run wall at L0 cannot validate the figure or the crossover. All outputs equal Velu: unit tests only (no_raw)."))
row(44,
 cl("Whole-process median 76.4 s -> 23.4 s (3.26x) for u64 root batching (shared Inversion via Montgomery trick)", True, "engineering", "engineering", "L0 (Apple M4 Pro shared, no isolation receipt, baseline spread 41.9-84.0 s)", "no (README concedes 'no A/A spread')", U,
    "The median ratio recomputes (76.38/23.40 = 3.26) but the baseline repetitions span 2x and the min-to-min ratio is 1.87x on the study's own admission. Wall at L0 with 3 reps cannot validate. Counted claim (~80x fewer inversions per state) is structural; recovered scalars/relations/probes identical 192/192."),
 cl("Rank stage 2.95x and per-target median 6.09x", False, "engineering", "mis-aggregated", "L0", "no", X,
    "CONTRADICTED by the committed timing records: the README table's candidate 'median' cells (rank_stage 10,261 ms, target_median 213 ms) are the candidate's fastest repetition (rep 3), while the baseline cells are medians (30,333 / 1,298). True medians give 30,333/12,303 = 2.47x and 1,298/259.5 = 5.00x. Asymmetric aggregation inflates both headline ratios (checked directly against raw/s3-*.txt)."))
row(45,
 cl("C1 (field-level splitting test) advances: whole 0.756x at 2^22, exponent delta -0.015 (12..22) and -0.034 (16..22)", True, "ADVANCES (exponent, constant, primitive)", "engineering (constant; exponent shift is small and size-range dependent)", "counts (F_q-mult equivalents), 4-core host", "yes: A/A delta -0.0012", C,
    "Deltas recompute (-0.0150, -0.0354). 'ADVANCES (exponent)' overstates: the study's own fits move the exponent by 0.015-0.034 over 2^12-2^22 while the local 21-22 exponent is 1.364 vs the 4/3 target, a distance remaining of 0.031. A/A -0.0012 is smaller than the delta, so an exponent shift is detectable, but it is a constant-factor change to the relation step in a regime where the extrapolated exponent is not established. C3 and C4 'TRADES' labels are accurate."))
row(46,
 cl("Retuned kappa and SGE: paired ratio 0.961 at 2^20 (kappa) and 0.781 -> 0.906 (SGE); baseline q^1.383", True, "A1: engineering (S fell 0.8-3.9%, exponent fell 0.010)", "engineering", "counts, unisolated 4-vCPU container", "yes: A/A fits E_BL on target 1 vs targets 2-3", C,
    "11/12 recompute (1 cannot_define). 'rebuilt baseline / rho on Jac(C) 0.16 raw at 2^14 (n = 4)' is a comparison against rho with n=4 samples at one size: not interpretable as a ratio. Distance to rho remains large at the sizes tested."))
row(47,
 cl("Duplicate of README row 38 (isogeny round 5): Phi_l 4.1x, SEA 1.14x geomean, NTT bit-identical", False, "engineering", "engineering", "L0 wall", "no", U, "Same entry listed twice in benchmarks/README.md (rows 38 and 47). Same numeric discrepancies (Phi_l start 9.57-9.74 s vs quoted 9.4-9.6 s; SEA sums 2.258/1.991 s vs quoted 2.12/1.81 s; range 0.71-1.72 vs 0.82-1.42)."))
row(49,
 cl("Cross-target root-shape replay costs MORE: default/replay word ops 0.970, 0.954, 0.983; verdicts identical", True, "null/recorded loss", "null result (switch stays off)", "counts (deterministic word operations)", "pilot on the frozen ladder 1.016", V, "7/7 recompute; 15/15 verdicts equal. Honest negative."),
 cl("decompose_by_e3 costs 553-34,901x the pair-table oracle's field multiplications", True, "stage diagnostic", "baseline comparison (a worse method)", "counts", "n/a", V, "Recomputes; a comparison against the strongest in-repo oracle (pair table)."))
row(50,
 cl("Binary MITM oracle l=9/l=10: median 0.293 ms vs 724.9 ms (2,474x) and 0.735 ms vs 3,119 ms (4,244x); 0 disagreements on 128 targets", True, "engineering", "engineering (decomposition-stage constant)", "L0 (arm64 macOS, shared, contended by other builds)", "no (l=8 exhaustive control in report)", U,
    "Wall at L0 under contention. Counted claims: 134,912 'quartic solves' at l=9 does not match the committed field (131,328 = 2^9(2^9+1)/2, by the report's own formula); 128/128 agreements and group-verified witnesses recompute. No exponent, no rho, as stated."))
row(51,
 cl("Generic Fp32 F4 engine vs u32 f4_fp_tower 1.09x/1.06x (no regression, 182 rows field-for-field identical); Shoup lazy rows 2.18x/2.24x; Fp52 IFMA 2.57x Fp64", True, "engineering (5/5 preregistered claims PASS)", "engineering", "L0 (shared with another lane, single core, 7 interleaved reps)", "tower and null controls in G2 equality runs", U,
    "10/10 recompute; identical rows 182/182 (the structural claim stands). Wall ratios at L0 on a host shared with another lane are not validated. Thresholds were set from a scratch calibration that is disclosed as not committed (PROTOCOL section 7), so 'preregistered' is partial."))
row(52,
 cl("Sigma-table kernel vs fused-control kernel on RTX PRO 6000: paired median ratio 1.0239 (range 1.0238-1.0241), A/A max drift 0.099%", True, "engineering (same-walk)", "engineering", "same-device GPU benchmark, no isolation receipt (Modal)", "yes: A/A drift 0.099%", C,
    "8/8 recompute. Effect 2.4% with an A/A of 0.1% on a single device. Identical-corpora claim has no raw (production corpus under /Volumes/SSD990-2, not in repo). Modal live call 8011 (15.89 B it/s) is a descriptive sample."))
row(53,
 cl("No IC arm beats rho on any curve at any weight; compact orbit 3.8-15x signed-Frobenius rho (w=0), prime pair table 10.5 -> 101.8x rho-neg (w=0.1)", "partial", "measurement/accounting; no method changed", "baseline measurement (IC loses)", "L0 counts only (macOS), wall descriptive", "yes: rho-strong-aa and rho-neg-aa A/A arms", V,
    "Counts; both rho arms fully priced and A/A'd; 4/6 claims recompute (2 cannot_define on the band endpoints, 1 partial); replay audits 3,178/3,178 identical. Section 5a band applied. Rho is the matched strongest in-repo reference (signed-Frobenius strong rho for Koblitz). Distance remaining: IC 3.8x-101.8x rho."))
row(54,
 cl("R1 (cheaper relation primitives, same walk) trades: relation phase 0.471x, whole 0.635x at 2^22, fitted exponent delta +0.015; R2 (R1 + retuned kappa) advances: 0.617x at 2^22, exponent delta +0.0008 matches", True, "R1 TRADES; R2 ADVANCES (constant)", "engineering (constant)", "counts, 4-core host, <=2 jobs", "yes: A/A delta +0.0008", C,
    "7/7 recompute. 'Whole 0.635x' is a F_q-multiplication-equivalent count under a shadow-accounting model; the R1 exponent got worse (+0.015) so the gain is a constant concentrated at small q. Label (ADVANCES constant = engineering) is consistent with section 3."))
row(55,
 cl("Root-rank harvest gate: old/new gate CPU (29,15) 1.41x [1.32, 1.52], (37,19) 1.89x, (31,16) 1.18x, (23,8) 1.05x, (31,10) 1.04x; end to end 1.20x", True, "ADVANCES (constant)", "engineering", "thread CPU time on a 4-core host shared with one other item; five A/A CIs exclude 1", "yes: A/A intervals present, 5 exclude 1 (sizes 0.915-1.038)", C,
    "9/10 recompute; counted word-op gains agree (e.g. 1.34x at (31,16)). Several small gains (1.04x-1.05x) are inside or near the A/A spread (0.915-1.038) and the study declines to count effects <=9% against an outside-1 A/A, which is the right rule; the 1.04x and 1.05x entries should be read as matches."))
row(56,
 cl("Block Wiedemann advances: exponent delta -0.008, 0.945x at 2^21, LA 0.889x; schedule trades 0.968x; combination 0.929x; 0.605x round-1 IC-2LP", True, "BW ADVANCES; schedule TRADES", "engineering (constant)", "counts, 4-core host, <=3 jobs", "yes: A/A delta -0.0004 [-0.0028, +0.0021]", C, "7/7 recompute (1 partial, 1 cannot_define). Exponent delta -0.008 sits at the A/A boundary: classify as constant."))
row(57,
 cl("PKM aux-curve search: z2xz8 fast 10.06 vs baseline 30.83 us/draw (0.326x); z2xz2 13.89 vs 24.74 (0.561x)", True, "engineering (S1-S4 pass)", "engineering", "L0 (one core taskset on a shared 4-core host, load 2.3-3.5)", "no; 7 interleaved repetitions", U,
    "9/10 recompute; one tail point (t=24: 147 observed vs 128 expected) is 15% over the model though within the study's interval. us/draw is wall at L0; the instruction-level change (a chi test via an addition chain) is a genuine algorithmic saving, but the magnitude is not validated."))
row(58,
 cl("Binary MITM l=8: median per-target 0.120 ms vs 12.1 ms (100.7x run 3; 94-277x across runs); 0 disagreements 64/64", False, "engineering (no exponent, no rho)", "engineering", "L0 (arm64 macOS, shared, unpinned)", "no", U,
    "Recomputed run-3 medians from per-target values: 0.1167 ms and 11.45 ms, ratio 98.1x vs the stored 100.7x (stored summary medians differ from the file's own per-target values). Wall at L0; counted point-adds (<=1,068 vs 32,896) and 64/64 agreement recompute."))
row(59,
 cl("fp_backend: F4 dense row Fp32 vs f4_fp_tower 1.31x (p31) and 2.65x (p32); lazy vs eager 9.5x (p31); mul vs ct_bignum 1.26-1.95x", True, "engineering (39/39 preregistered claims PASS)", "engineering", "L0 (shared with another lane, single core taskset)", "no explicit A/A; 9 interleaved reps", U,
    "9/10 recompute. Wall at L0. ct_bignum is constant-time, fp_backend variable-time: the mul comparison is not like-for-like in security class, and the 'no regression' claim is the only same-arithmetic one."))
row(60,
 cl("D4-symmetrised weighted S5 (sigma, pi grading) costs C_w 1.41-1.44e9 vs C8 2.54-2.56e10 F_p multiplications: paired x17.8-18.5", True, "engineering (a constant)", "engineering", "counts (F_p multiplications), shared host", "no", C,
    "7/9 recompute. Count discrepancy: '1 of 102 targets degenerate' vs 84 distinct (p, seed, target) in the committed per-target files. README cites a protocol commit (87fe78b63) that is not the commit git shows. 'x844-863 below the quoted 1.213e12' compares against a quoted figure, not a re-run; the handover 2^157.9 -> 2^118.9 is derived."))
row(63,
 cl("Linear-algebra arms: whole q^1.380 [1.376, 1.384] vs incumbent q^1.398, ratio 0.653x at 2^20; LA 0.463x but LA exponent 1.70 vs 1.52", "partial", "ADVANCES (constant, small exponent)", "engineering (constant); LA exponent got worse", "counts, 4-core host load 4-14", "yes: A/A delta 0.000 [-0.004, 0.004]", C,
    "Point values recompute; log-count claim '474/474 verified' recomputes as 746/748 correct in progress.log. Exponent shift -0.018 is small versus the 4/3 target (local exponent trend still +0.020/bit)."))
row(64,
 cl("Linear-row harvest gated on gens/var < 0.9607: I/G 1.075 [1.023, 1.125] (19,7) m3, 1.073 (17,9) m2, 1.166 [1.092, 1.246] (23,12) m2; no cell regresses", "partial", "ADVANCES where it fires", "engineering", "thread CPU, 4 vCPU shared, load 5-12", "yes: A/A CIs cover 1", C,
    "Point geomeans reproduce, CIs not (fit tool uses a seeded StdRng not committed). Counters identical on 60/60 non-firing-cell targets. Wall CPU gains of 1.07x under load 5-12 sit near the A/A; counted word-op gains agree."))
