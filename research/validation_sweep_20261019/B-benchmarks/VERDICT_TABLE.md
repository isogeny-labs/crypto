# All claim verdicts (generated from verdicts.jsonl)

| claim | date | entry | claim (short) | verdict | tier | recomputed |
|:--|:--|:--|:--|:--|:--|:--|
| 3.1 | 2026-10-19 | hec-rho-branch-width | Full-width branch coefficients cut Jacobian plain S_walk: median 1.390 at N>=1e5, largest… | validated w/ caveats | counted units | True |
| 2.1 | 2026-10-18 | hec-rho-hash-mixing | SplitMix64-mixed hash lowers plain S_walk to median 1.406 at N>=1e5 (H1 <=1.55 falsified … | validated w/ caveats | counted units | True |
| 2.2 | 2026-10-18 | hec-rho-hash-mixing | Mixed fold/plain median 0.960 (H2 <=0.80 falsified) | validated | counted units | True |
| 1.1 | 2026-10-18 | hec-rho-negation | Folded (negation-class) walk uses 0.67-0.89x the group operations of the plain walk at N<… | validated w/ caveats | counted units | True |
| 1.2 | 2026-10-18 | hec-rho-negation | At N>=1e5 the median folded/plain ratio is 0.968, registered R1 (<=0.80) falsified | validated | counted units | True |
| 1.3 | 2026-10-18 | hec-rho-negation | Plain DP walk S_walk 1.82 and 2.10 vs ideal 1.2533 at the two largest rows | validated w/ caveats | counted units | True |
| 26.1 | 2026-10-17 | prime-order-search | find_prime_order_curve plan time 217.6 s -> 0.137 s at 26 bits (1,592x) by stepping one p… | unvalidated | wall time at L0 / shared | partial |
| 22.1 | 2026-10-17 | two-sylow-fp64 | Duplicate of README row 8: branch-free Fp64 neutral on two_sylow_search | validated w/ caveats | wall time at L0 / shared | True |
| 21.1 | 2026-10-17 | f5-gf2-defaults | Direct pack on by default: 1.006-2.173x on every F5 cell | unvalidated | wall time at L0 / shared | False |
| 19.1 | 2026-10-17 | gpu-nist-jac-seed | gpu/nist rho seeding: field inversions per seed ~750 -> 1 (P-384 ~1150 -> 1) by an invers… | validated w/ caveats | counted units | True |
| 19.2 | 2026-10-17 | gpu-nist-jac-seed | P-256 seeding 0.66 vs 11.53 ms/seed (17.4x) | unvalidated | wall time at L0 / shared | partial |
| 18.1 | 2026-10-17 | eigen-seta-pk-degree-test | Testing a candidate secret degree takes chain length e vs full decapsulation chain E (569… | validated | counted units | True |
| 18.2 | 2026-10-17 | eigen-seta-pk-degree-test | pk_degree_test wall 0.79 vs 1.28 s (L1), 2.72 vs 5.54 s (L3), 6.84 vs 13.34 s (L5) | unvalidated | wall time at L0 / shared | True |
| 16.1 | 2026-10-17 | isogeny-volcano-cost | Rho below IC on all 800 curves at every section-5a weight (IC ~30-60x rho) | validated w/ caveats | counted units | partial |
| 15.1 | 2026-10-17 | generic-dlp-correctness | Pohlig-Hellman total_steps 284,364 -> 10,682 (26.62x) by per-prime BSGS tables instead of… | validated | counted units | True |
| 15.2 | 2026-10-17 | generic-dlp-correctness | pollard_collab negation-map dead trails 50 -> 1, 106 -> 0, 197 -> 15 | validated | counted units | True |
| 14.1 | 2026-10-17 | generic-dlp-memory-parallelism | BSGS one-target wall total_s baseline/candidate 1.30-1.44x chain, 1.74-1.98x paired at 44… | unvalidated | wall time at L0 / shared | True |
| 14.2 | 2026-10-17 | generic-dlp-memory-parallelism | rho_parallel solve_s 5.84x at 32, 3.75x at 36, 1.82x at 40, 1.14x at 48 bits (1 thread) | unvalidated | wall time at L0 / shared | False |
| 14.3 | 2026-10-17 | generic-dlp-memory-parallelism | Candidate/PARI elllog 0.020-0.167 (baseline/PARI 0.022-0.974) on the same curves and targ… | unvalidated | wall time at L0 / shared | None |
| 8.1 | 2026-10-17 | two-sylow-fp64 | Branch-free Fp64 leaves the two_sylow halving search neutral (ratios 0.98-1.08), positive… | validated w/ caveats | wall time at L0 / shared | True |
| 5.1 | 2026-10-17 | perf-history-factor-bases | Index calculus costs 38-159x the callgrind Ir of rho on the same curves (28.1-34.6 M vs 0… | unvalidated | counted units | partial |
| 47.1 | 2026-10-16 | isogeny-round5 | Duplicate of README row 38 (isogeny round 5): Phi_l 4.1x, SEA 1.14x geomean, NTT bit-iden… | unvalidated | wall time at L0 / shared | False |
| 45.1 | 2026-10-16 | ic-improve4-01-genus3-relations | C1 (field-level splitting test) advances: whole 0.756x at 2^22, exponent delta -0.015 (12… | validated w/ caveats | counted units | partial |
| 44.1 | 2026-10-16 | u64-rootbatch | Whole-process median 76.4 s -> 23.4 s (3.26x) for u64 root batching (shared Inversion via… | unvalidated | wall time at L0 / shared | True |
| 44.2 | 2026-10-16 | u64-rootbatch | Rank stage 2.95x and per-target median 6.09x | contradicted | wall time at L0 / shared | False |
| 43.1 | 2026-10-16 | isogeny-large-ell | Montgomery sqrt-elu codomain 5.58x (256-bit) / 5.27x (511-bit) Velu at l=10007; crossover… | unvalidated | wall time at L0 / shared | partial |
| 42.1 | 2026-10-16 | binary-mitm-ladder-l11-l12 | Binary MITM oracle vs Gaudry pairs-and-solve: median 1.746 ms vs 1.62 s (928x) at l=11 an… | unvalidated | wall time at L0 / shared | False |
| 40.1 | 2026-10-16 | fixed-degree-isogenies | Fixed-degree isogeny pipeline: 6/6 planted prime d found; Coron faster than Cornacchia in… | unvalidated | wall time at L0 / shared | True |
| 39.1 | 2026-10-16 | hwcost-rho-model | Calibrated hwcost model: median /error/ 1.8-5.5% on B>=4096, 5/5 PASS (a priori 4/5, fp64… | validated w/ caveats | wall time at L0 / shared | True |
| 38.1 | 2026-10-16 | isogeny-round5 | Phi_l (l<=89) over 2^127-1: 9.4-9.6 s -> 2.3 s (4.1x) | unvalidated | wall time at L0 / shared | False |
| 38.2 | 2026-10-16 | isogeny-round5 | SEA point count over 2^127-1: 1.14x geomean (0.82-1.42) | unvalidated | wall time at L0 / shared | False |
| 38.3 | 2026-10-16 | isogeny-round5 | NTT 10-11x Karatsuba at n=8192 (bit-identical output) | unvalidated | wall time at L0 / shared | None |
| 37.1 | 2026-10-16 | isogeny-vs-flint-pari | isogeny_algos vs FLINT 3.0.1 and PARI 2.15.4: ours/fastest-external 0.42-0.95 (127-bit pr… | validated w/ caveats | wall time on a reserved core | True |
| 36.1 | 2026-10-16 | hwcost-round2 | -C target-cpu=native rho kernel 1.6-2.6x faster for fp64 (ratio 0.38-0.63); FpWide no eff… | unvalidated | wall time at L0 / shared | False |
| 36.2 | 2026-10-16 | hwcost-round2 | Fp64Cm branch-free kernel 0.40-0.50x Fp64 at B>=4096 (F1' PASS); a priori model within 4-… | unvalidated | wall time at L0 / shared | True |
| 36.3 | 2026-10-16 | hwcost-round2 | IC > rho in wall time on every section-5a-ranked curve (Koblitz 4/8, prime 5/6) | validated w/ caveats | wall time at L0 / shared | True |
| 35.1 | 2026-10-16 | strong-rho-assurance | Callgrind instructions per rho step 1,196 -> 938 (-21.5%), outputs identical (8/8 callgri… | validated | counted units | True |
| 35.2 | 2026-10-16 | strong-rho-assurance | Steps/ideal mean 0.994, 0.922, 0.943 (CI upper 1.044, 0.971, 0.993) on 400 fixtures | validated | counted units | True |
| 35.3 | 2026-10-16 | strong-rho-assurance | Native 1.023x at n53 (A/A 10.7%) and 1.209x at n61 (A/A 18.3%) | validated | wall time on a reserved core | True |
| 34.1 | 2026-10-16 | j0-parallel-relations | j=0 staged relation collection rayon-parallel vs serial: 4.4x at 16 bits, 3.0x at 20 bits… | unvalidated | wall time at L0 / shared | None |
| 33.1 | 2026-10-16 | endo-native-bench | GLV-2 via cheap isogeny-chain endomorphism vs width-w NAF: counted ratio 1.525/1.521/1.49… | validated | counted units | True |
| 33.2 | 2026-10-16 | endo-native-bench | Timed median declared GLV vs fastest baseline 1.504/1.504/1.570 | validated w/ caveats | wall time on a reserved core | True |
| 32.1 | 2026-10-16 | f6-gate-first-x86 | Gate-first F6-IC order: 1.336-1.733x fewer word XORs on 9 cells; median CPU 1.56-2.12x lo… | validated w/ caveats | counted units | partial |
| 31.1 | 2026-10-16 | inherited-f4-eager-runs | Shift-run eager rewrite 1.22-1.70x lower median CPU than bit scatter on 6/9 cells; 2-28% … | unvalidated | wall time at L0 / shared | True |
| 30.1 | 2026-10-16 | f4fp-blocked-pairs | Blocked pair reduction 0.78x on quad_n10, 0.89x on quad_n11 vs row-by-row product (same b… | unvalidated | wall time at L0 / shared | True |
| 29.1 | 2026-10-16 | fp64-branchfree | Branch-free Fp64 rho kernel 0.40-0.44x (fp_backend) and 0.45-0.47x (prime_fast) time at B… | unvalidated | wall time at L0 / shared | True |
| 28.1 | 2026-10-16 | strong-rho-canon2 | Instructions per rho step 938 -> 781 (-16.8%, 1.20x), outputs identical | validated | counted units | True |
| 28.2 | 2026-10-16 | strong-rho-canon2 | Native ns/step 1.173x at n53 (A/A 20.6%), 1.135x at n61 (A/A 12.1%) | validated | wall time on a reserved core | True |
| 27.1 | 2026-10-16 | strong-rho-step3 | Instructions per rho step 781 -> 718 (-8.1%, 1.09x), outputs identical at n=41,53,61 | validated | counted units | True |
| 27.2 | 2026-10-16 | strong-rho-step3 | Native ns/step 1.008x at n53 and 1.025x at n61 | validated | wall time on a reserved core | True |
| 25.1 | 2026-10-16 | generalized-rootfinding | Cached-separator / shared-left whole-cold median 1.383x; actual GF83 support-only 1.141x … | unvalidated | wall time at L0 / shared | True |
| 24.1 | 2026-10-16 | n83-precomputed-rho-control | Online median IC/precomputed-rho: K3300 3.74x time-matched, 40.61x memory-matched; K600 8… | validated w/ caveats | wall time at L0 / shared | partial |
| 23.1 | 2026-10-16 | sea-point-count | BSGS24/32/40 per-call 61.7->31.5, 327->118, 1937->592 us (paired 1.99x, 2.79x, 3.24x) | validated w/ caveats | wall time on a reserved core | True |
| 23.2 | 2026-10-16 | sea-point-count | SEA40 0.81x, SEA192 1.02x, cold API 1.005x all within the A/A envelope | validated | wall time on a reserved core | True |
| 96.1 | 2026-10-15 | gaudry-k4-d4-char2 | Frobenius-reduced k=4 D4 form: C4 1.98e5, 481x below plain at m=8; S Frobenius/plain 0.00… | validated w/ caveats | counted units | partial |
| 88.1 | 2026-10-15 | ic-m4-wide-chain-vs-mitm | Wide u128 chained-S3 decomposition grows faster than the pair-table MITM floor: slope 0.8… | validated w/ caveats | wall time at L0 / shared | False |
| 81.1 | 2026-10-15 | ic-linear-algebra-cado | Our block-Wiedemann pipeline 1.86x faster than CADO-NFS bwc on the same 65,858-column cor… | unvalidated | wall time at L0 / shared | partial |
| 80.1 | 2026-10-15 | joux-vitse-k5-n1 | Joux-Vitse n=5: trial 5.9-9.2e9 F_p mult. flat; complete DLPs IC/rho 4.5e7 / 1.8e8 (p=11)… | validated w/ caveats | counted units | partial |
| 79.1 | 2026-10-15 | ecdlp-speedups | Koblitz compact-orbit fast path whole-process median 60.9 s -> 37.5 s (1.63x), targets st… | unvalidated | wall time at L0 / shared | True |
| 79.2 | 2026-10-15 | ecdlp-speedups | j=0 relation stage 13.99 s -> 11.90 s (1.17x) | unvalidated | wall time at L0 / shared | None |
| 69.1 | 2026-10-15 | pkm-isogeny-tower | PKM prime-isogeny tower base with aux search charged: isogeny/plain 1.002..1.331 (search … | validated w/ caveats | counted units | partial |
| 68.1 | 2026-10-15 | ic-improve2-04-d4-2lp-fix | First-dependency stop (d4-ed): whole 0.979x [0.973, 0.986] over 27 held-out curves, never… | validated w/ caveats | counted units | partial |
| 67.1 | 2026-10-15 | ic-improve2-02-freerel-production | Free relations in production compact-orbit IC: tuned vs tuned counted 0.960x [0.946, 0.97… | validated w/ caveats | counted units | partial |
| 66.1 | 2026-10-15 | ic-improve2-03-fallrow-gate | Gated fall-row augmentation: I/G 1.83x [1.74, 1.94] n=9, 4.42x n=11, 11.9x n=13 on overde… | unvalidated | wall time at L0 / shared | partial |
| 65.1 | 2026-10-15 | cyclotomic-falsifier | Cyclotomic gamma-weight bases are yield-neutral on Koblitz n=23: all effects -0.49%..+0.6… | validated w/ caveats | counted units | False |
| 65.2 | 2026-10-15 | cyclotomic-falsifier | Cost (S, lower bounds): cyclotomic/compact 1.022 [1.021, 1.023] (K1,30 cols), 1.122 (K0,1… | validated w/ caveats | counted units | partial |
| 64.1 | 2026-10-15 | ic-improve2-05-linear-harvest | Linear-row harvest gated on gens/var < 0.9607: I/G 1.075 [1.023, 1.125] (19,7) m3, 1.073 … | validated w/ caveats | counted units | partial |
| 63.1 | 2026-10-15 | ic-improve2-01-genus3-linalg | Linear-algebra arms: whole q^1.380 [1.376, 1.384] vs incumbent q^1.398, ratio 0.653x at 2… | validated w/ caveats | counted units | partial |
| 60.1 | 2026-10-15 | d4-weighted-s5 | D4-symmetrised weighted S5 (sigma, pi grading) costs C_w 1.41-1.44e9 vs C8 2.54-2.56e10 F… | validated w/ caveats | counted units | partial |
| 59.1 | 2026-10-15 | fp-backend | fp_backend: F4 dense row Fp32 vs f4_fp_tower 1.31x (p31) and 2.65x (p32); lazy vs eager 9… | unvalidated | wall time at L0 / shared | True |
| 58.1 | 2026-10-15 | binary-mitm-oracle-l8 | Binary MITM l=8: median per-target 0.120 ms vs 12.1 ms (100.7x run 3; 94-277x across runs… | unvalidated | wall time at L0 / shared | False |
| 57.1 | 2026-10-15 | isogeny-p192-search | PKM aux-curve search: z2xz8 fast 10.06 vs baseline 30.83 us/draw (0.326x); z2xz2 13.89 vs… | unvalidated | wall time at L0 / shared | partial |
| 56.1 | 2026-10-15 | ic-improve2-06-genus3-kappa-bw | Block Wiedemann advances: exponent delta -0.008, 0.945x at 2^21, LA 0.889x; schedule trad… | validated w/ caveats | counted units | True |
| 55.1 | 2026-10-15 | ic-improve3-02-harvest-gate | Root-rank harvest gate: old/new gate CPU (29,15) 1.41x [1.32, 1.52], (37,19) 1.89x, (31,1… | validated w/ caveats | counted units | True |
| 54.1 | 2026-10-15 | ic-improve3-01-genus3-relations | R1 (cheaper relation primitives, same walk) trades: relation phase 0.471x, whole 0.635x a… | validated w/ caveats | counted units | True |
| 53.1 | 2026-10-15 | ic-frontier-campaign | No IC arm beats rho on any curve at any weight; compact orbit 3.8-15x signed-Frobenius rh… | validated | counted units | partial |
| 52.1 | 2026-10-15 | ecc2k130-modal-rollout | Sigma-table kernel vs fused-control kernel on RTX PRO 6000: paired median ratio 1.0239 (r… | validated w/ caveats | wall time at L0 / shared | True |
| 51.1 | 2026-10-15 | f4-generic-field | Generic Fp32 F4 engine vs u32 f4_fp_tower 1.09x/1.06x (no regression, 182 rows field-for-… | unvalidated | wall time at L0 / shared | True |
| 50.1 | 2026-10-15 | binary-mitm-ladder-l9-l10 | Binary MITM oracle l=9/l=10: median 0.293 ms vs 724.9 ms (2,474x) and 0.735 ms vs 3,119 m… | unvalidated | wall time at L0 / shared | partial |
| 49.1 | 2026-10-15 | pdp-root-shape-replay | Cross-target root-shape replay costs MORE: default/replay word ops 0.970, 0.954, 0.983; v… | validated | counted units | True |
| 49.2 | 2026-10-15 | pdp-root-shape-replay | decompose_by_e3 costs 553-34,901x the pair-table oracle's field multiplications | validated | counted units | True |
| 46.1 | 2026-10-15 | genus3-2lp-rebalance | Retuned kappa and SGE: paired ratio 0.961 at 2^20 (kappa) and 0.781 -> 0.906 (SGE); basel… | validated w/ caveats | counted units | True |
| 41.1 | 2026-10-15 | pair46-factor-bases | Best psi-chain DLP S = 1.47 [1.28, 1.69]/sqrt(N) vs +-BSGS 0.955 [0.86, 1.05] at 22 bits … | validated w/ caveats | counted units | True |
| 104.1 | 2026-10-14 | n71-n83-hosted-single-target | Online speedup rho/IC median 7.29x (n=71) and 12.29x (n=83) with strong rho rung 3 on the… | contradicted | wall time on a reserved core | True |
| 99.1 | 2026-10-14 | rootfinding-lanes-coverage | 4/8/16-lane synchronised prime relation collection: 72/72 exact lane pairs at equal count… | validated w/ caveats | counted units | True |
| 97.1 | 2026-10-14 | gaudry-k4-d4-cost | D4 oracle with pivot choice and weighted form: C4 2.55e10 -> 6.89e7 F_p multiplications a… | validated | counted units | True |
| 95.1 | 2026-10-14 | rootfinding-buffers | Prime lane collector buffer reuse: batch-8 relation-phase wall 0.937 (0.786-1.122), whole… | validated | wall time at L0 / shared | True |
| 92.1 | 2026-10-14 | ecbench-rho-optimized | Optimised strong-rho canonicalisation 0.802 of legacy callgrind Ir (19.8% fewer), clearin… | validated w/ caveats | counted units | partial |
| 91.1 | 2026-10-14 | ic-m3-algebraic-vs-mitm | No arm grows more slowly than the MITM floor: m=3 floor slope 0.207, f4-chain 0.534, f6-i… | validated w/ caveats | wall time at L0 / shared | partial |
| 90.1 | 2026-10-14 | greedy-factor-base-yield | Greedy coverage-driven factor base: expected trials 4.46 vs 5.29 (1.18x) on K0/2^13, 1.08… | validated w/ caveats | counted units | True |
| 89.1 | 2026-10-14 | secg-small-x-s3-cost | S3 per target at B=4096: 5.4 ms sect113r1; B+1 relations by S3 solves are 52 (sect113r1),… | validated | counted units | True |
| 87.1 | 2026-10-14 | ic-program-paper | Programme paper: admitted n=61 online 13.94x [9.81, 19.13] vs rho without a table, retire… | validated w/ caveats | counted units | partial |
| 86.1 | 2026-10-14 | rootfind-stride-targets | Stride-renew target generation: counted S stride-renew/walk 0.531 [0.506, 0.557] (n=13, d… | validated w/ caveats | counted units | partial |
| 85.1 | 2026-10-14 | fb-hyperparam-sweep | Prime a=-3 small-x IC slower than rho at 24/28 bits and not at 20 bits for m=3 (FB<=256);… | validated w/ caveats | counted units | partial |
| 84.1 | 2026-10-14 | rho-walk-leads | Best of P+-R steps speedup 1.129 (1.081-1.184) under DP, predicted 1.155; cost 1.215x a p… | validated w/ caveats | counted units | partial |
| 83.1 | 2026-10-14 | ecc2k130-signed-sigma | SIGMA_SIGN=2 signed sigma walk: iterations 0.958-0.967 on device (n=41), 0.951-0.959 emul… | validated w/ caveats | counted units | partial |
| 82.1 | 2026-10-14 | frobenius-orbit-ic-vs-rho | Frobenius-orbit IC counted S vs rho-frob 2.96 [2.55, 3.24] (n=13) ... 18.2 [15.7, 21.6] (… | validated | counted units | partial |
| 78.1 | 2026-10-14 | ic-next10-01-fp4-ladder | D4/plain S 0.0548 [0.048, 0.061] (18.2x constant, same exponent); double large primes: wh… | validated w/ caveats | counted units | partial |
| 77.1 | 2026-10-14 | ic-next10-03-pkm-degree | msolve runs m=4 N=20 to three degree-8 rounds within 5 GB where f4_fp_tower needs >14 GB;… | validated w/ caveats | counted units | True |
| 76.1 | 2026-10-14 | ic-next10-06-equivariant-f4-m4 | Equivariant Macaulay pass bit-identical to full F2 RREF at n=7-17: 44x/58x/73x CPU at n=1… | unvalidated | wall time at L0 / shared | False |
| 75.1 | 2026-10-14 | ic-next10-07-cubic-ghs-genus3 | Genus-3 IC after cubic GHS: exponent q^1.650 -> q^1.300 (q^1.377 over 2^11-2^15), for ref… | validated w/ caveats | counted units | partial |
| 74.1 | 2026-10-14 | ic-next10-09-equal-advice | Audit: Bernstein-Lange preprocessing rho with equal or smaller advice is 53-317x faster o… | validated | counted units | True |
| 74.2 | 2026-10-14 | ic-next10-09-equal-advice | 49 independent free relations at K=600 = 8.2% of the 599 needed (x1.09) | validated w/ caveats | counted units | partial |
| 73.1 | 2026-10-14 | ic-improve-04-genus3-2lp | Genus-3 IC with double large primes: q^1.385 vs incumbent q^1.910 over 2^10-2^18; ratio 0… | validated w/ caveats | counted units | partial |
| 72.1 | 2026-10-14 | ic-improve-01-d4-2lp | D4 + double large primes with tuned alpha: whole 1.025x [1.016, 1.037] REGRESSES; LA 0.82… | validated | counted units | partial |
| 71.1 | 2026-10-14 | ic-improve-03-tracezero-freerel | Free relations on compact-orbit IC: complete solve 0.957x [0.944, 0.970] counted, wall 0.… | validated w/ caveats | counted units | partial |
| 70.1 | 2026-10-14 | ic-improve-02-equivariant-oracle | G2 Frobenius-equivariant Macaulay pass as oracle: I/C 5.1x [4.85, 5.34] (n=11), 15.5x (n=… | validated w/ caveats | wall time at L0 / shared | partial |
| 110.1 | 2026-10-13 | n83-dense-root | K1798 dense-root N83: one-target online 63.324 ms vs same-point rho 1,173.293 ms (18.528x… | contradicted | wall time at L0 / shared | True |
| 109.1 | 2026-10-13 | nist-gpu-rho-ladder | gpu/nist P-256/P-384 rho: first working build 1.075/0.210 B it/s; PTX carry chains 1.19x/… | unvalidated | wall time at L0 / shared | partial |
| 108.1 | 2026-10-13 | ecbench-frontier | Admitted retained-setup online frontier is n61 rho/IC 13.9x [9.8, 19.1]; N83 exploratory … | contradicted | counted units | partial |
| 107.1 | 2026-10-13 | ic-rootfind-ecbench | Whole IC solve with the root-finder 350x faster in wall than with F4 at n=13 (admitted), … | validated w/ caveats | wall time on a reserved core | False |
| 106.1 | 2026-10-13 | gaudry-quartic-solve-speed | Gaudry quartic Macaulay solve median 1.9x/4.1x/6.4x faster at d=4/5/6 (37.4 s -> 5.9 s at… | unvalidated | wall time at L0 / shared | False |
| 105.1 | 2026-10-13 | gaudry-quartic-d4 | D4-symmetric system cuts C4 from 1.21e12 to 2.54e10 F_p multiplications (47.5-48.4x), quo… | validated w/ caveats | counted units | partial |
| 103.1 | 2026-10-13 | rootfinding-pair-size | B64/B32 counted-GAE ratio 0.830 (CI 0.697-0.962): 64 abscissae cheaper than 32; B64/rho 1… | validated w/ caveats | counted units | True |
| 102.1 | 2026-10-13 | prime-rootfinding-arity | Pair-table arity m3/m2 counted-GAE ratio 1.431 (CI 1.225-1.716): three-summand table is w… | validated | counted units | True |
| 101.1 | 2026-10-13 | prime-rootfinding-batchtable | Batched vs scalar table inversions (4,096 vs 64): cold GAE equal (batch/scalar 1.000); se… | validated w/ caveats | wall time at L0 / shared | True |
| 98.1 | 2026-10-13 | gaudry-k4-ecbench | k=4 IC end to end in ecbench: S = 7.17e8 vs rho 1.065: 6.7e8x rho (within 2x of the model) | validated | counted units | True |
| 1007.1 | 2026-10-12 | f4-f5-native-hashing | Native Boolean F5 hashing: 3.715% fewer complete-F5 instruction references (95% 3.714-3.7… | validated | counted units | True |
| 1004.1 | 2026-10-12 | glv-gls-4d-confirmation | GLV-GLS-4 D=-619 confirmation on a second host: projective GLV-GLS-4 median 105,273 ns vs… | unvalidated | wall time at L0 / shared | True |
| 1003.1 | 2026-10-12 | lit2026-w12-zero-minor | Zero-minor ECDLP vs rho.negation: counted field-operation cost 260-6,715x rho; search-pha… | validated w/ caveats | counted units | partial |
| 1002.1 | 2026-10-12 | lit2026-w5-gate-first-rref | Gate-first geometry with reduced root bases: original/candidate word-XOR ratios 1.64/1.93… | validated w/ caveats | counted units | True |
| 112.1 | 2026-10-12 | pdp-root-finder | Root-finding oracles per refuted target: rf-mitm 3.1 us -> 482 us at l=8 -> 14 vs PS-0 ba… | validated w/ caveats | wall time on a reserved core | False |
| 111.1 | 2026-10-12 | rootfinding-decomposition | Pair-table/subtraction counted-S quotient 0.0189 (prime) and 0.0570 (Koblitz); matched rh… | validated | counted units | True |
| 1035.1 | 2026-10-11 | conductor-gap-round3 | E4 direct/quotient F_p row-reduction multiplication ratios 4.12-4.26 (order 2), 17.6-19.6… | validated w/ caveats | counted units | partial |
| 1026.1 | 2026-10-11 | f4-f5-row-allocation | Boolean F4/F5 row building: cumulative requested bytes -42.47% (F4), -45.47% (F5), -50.30… | validated | counted units | True |
| 1025.1 | 2026-10-11 | fiber-ic-scaling | Fiber-aware IC vs matched rho: envelope IC/rho grows from 8.4-8.8 to 35.1-37.0 (h=4) and … | validated | counted units | True |
| 1024.1 | 2026-10-11 | glv-gls-4d-benchmark | D=-619 projective GLV-GLS-4 median 71,172 ns vs GLS-2 81,665 ns: 1.147x; across D 1.062-1… | unvalidated | wall time at L0 / shared | True |
| 1023.1 | 2026-10-11 | glv-gls-lattice-cost-model | Derived GLS-2/GLV-GLS-4 cost-model ratio 1.220x at D=-619; char-2 Verschiebung correction… | validated w/ caveats | counted units | partial |
| 1022.1 | 2026-10-11 | isogeny-vertical-margin-historical | sqrt-elu vs Velu at l~2^24: 100,663,797 vs 24,454,729 M-equivalents (4.12x); counted expo… | validated w/ caveats | counted units | True |
| 1021.1 | 2026-10-11 | koblitz-4e-historical | Closed-form Koblitz 4E table check 0.124/0.122 us at n=131/233 vs tau-NAF 160/98 us (orde… | unvalidated | wall time at L0 / shared | True |
| 1020.1 | 2026-10-11 | koblitz-m3-1955z | n=17 per-call e3 0.000116/0.00141/0.0216 s at l=3/4/5; symmetric l=5 timed out (>300 s fo… | unvalidated | wall time at L0 / shared | True |
| 1019.1 | 2026-10-11 | koblitz-m3-2150z | PinX3 vs geometric e3 per call: 0.052/0.154 ms vs 9.702/123.595 ms (n=17/19) and n=31 ful… | unvalidated | wall time at L0 / shared | True |
| 1017.1 | 2026-10-11 | lit2026-w4-early-abort | Early-abort probes: mode 1 original/candidate 1.00083-1.00378 (no change); mode 2 0.99714… | validated w/ caveats | counted units | partial |
| 1016.1 | 2026-10-11 | lit2026-w5-gate-first-lane-roots | Gate-first original/candidate word-XOR ratios 1.5675-1.7788; lane roots cost 2.6-4.0x the… | validated w/ caveats | counted units | partial |
| 1015.1 | 2026-10-11 | lit2026-w8-quotient-s3 | Direct quotient-curve m=2 solve vs symmetrised control: F4 word-operation ratios 1.340-1.… | validated | counted units | True |
| 1014.1 | 2026-10-11 | lit2026-w8-summation-models | Frobenius auxiliaries cost 57-701x more MatrixF4 word operations; convolution splitting 1… | validated w/ caveats | counted units | partial |
| 1012.1 | 2026-10-11 | pdp-ps1-algebra-gates | Group-law vs derived baseline gate count: 7,410-14,608 fused ternary-logic issues per tas… | validated w/ caveats | counted units | partial |
| 1011.1 | 2026-10-11 | pdp-ps2-scalar-group-law | Scalar group-law PDP kernel vs baseline: group-scalar/baseline per-pair ratios 0.0668-0.1… | validated w/ caveats | wall time on a reserved core | True |
| 1010.1 | 2026-10-11 | pdp-split-hybrid | Split/hybrid PDP: closest relation-phase projection 22.8x strong rho or 391x signed-Frobe… | validated w/ caveats | counted units | True |
| 116.1 | 2026-10-11 | inherited-f4-eager-rewrite | Inherited F4 eager rewrite: lazy/eager median CPU ratio 1.40 (F4) / 1.62 (F6-IC) at K0/2^… | unvalidated | wall time at L0 / shared | True |
| 115.1 | 2026-10-11 | groebner-vs-msolve-singular-polybori | f_p F4 reduced basis equals Singular on 13/13 systems; msolve 1-thread 12-24x faster than… | validated w/ caveats | wall time at L0 / shared | partial |
| 114.1 | 2026-10-11 | lit2026-w11-generic-engineering | BSGS chain -> paired 16.75 -> 3.71 M-equivalents per visited point; original/paired wall … | validated w/ caveats | counted units | True |
| 1036.1 | 2026-10-10 | isogeny-campaign-and-profiler | Frozen matched profiler panel: instruction reductions 67.958860% (screen), 5.292412% (sea… | validated | counted units | True |
