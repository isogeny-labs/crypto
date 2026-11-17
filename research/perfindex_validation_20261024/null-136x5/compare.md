# perfbench wall-time comparison (reanalysis)

**Performance index PI = 1.005x** (95% CI 0.985–1.019, 5 paired rounds) — **not qualified** (see warnings)

Median A/A noise across kernels: ±4.6%.

> warning: 1 verdict(s) are not corroborated by the best-of ratio (marked †): ['fp_gb/f4_solve_quart_n3_p31']

> warning: 38 kernel(s) had contended rounds (left out of their medians)

> warning: 1 kernel(s) have no A/A spread (fewer than 3 A/A pairs)

| area | weight | kernels | area index |
|:--|--:|--:|--:|
| bool_gb | 0.125 | 19 | 0.998x |
| dlp | 0.125 | 12 | 1.020x |
| field_ec | 0.125 | 25 | 0.999x |
| fp_gb | 0.125 | 19 | 1.000x |
| gf2_la | 0.125 | 19 | 1.018x |
| pdp | 0.125 | 21 | 1.008x |
| relation | 0.125 | 10 | 0.993x |
| sat | 0.125 | 11 | 1.005x |

| kernel | base ms | cand ms | speedup | 95% CI | best-of | A/A noise | rounds | verdict |
|:--|--:|--:|--:|:--|--:|--:|--:|:--|
| `bool_gb/crossbred_m2_n23` | 98.521 | 92.886 | 1.017x | 0.948–1.141 | 1.004x | ±2.7% | 5/5 | neutral |
| `bool_gb/f4_basis_m2_n17` | 146.937 | 147.469 | 0.992x | 0.973–0.998 | 0.985x | ±7.5% | 5/5 | neutral |
| `bool_gb/f4_basis_random_n18` | 137.698 | 137.177 | 1.000x | 0.958–1.094 | 1.008x | ±3.3% | 4/5 (1 contended) | neutral |
| `bool_gb/fes_gray_m2_n23` | 25.517 | 25.284 | 1.001x | 0.598–1.028 | 1.000x | ±4.4% | 5/5 | neutral |
| `bool_gb/fes_gray_random_n28` | 201.471 | 199.897 | 0.994x | 0.872–1.054 | 0.999x | ±2.2% | 5/5 | neutral |
| `bool_gb/fes_moebius_random_n22` | 54.573 | 56.124 | 0.975x | 0.743–1.002 | 0.989x | ±2.2% | 4/5 (1 contended) | neutral |
| `bool_gb/fes_monica_random_n25` | 153.361 | 154.242 | 1.003x | 0.994–1.012 | 1.001x | ±2.2% | 5/5 | neutral |
| `bool_gb/fes_wide_random_n30` | 210.137 | 210.612 | 1.003x | 0.998–1.050 | 1.001x | ±1.7% | 5/5 | neutral |
| `bool_gb/matrix_f4_deg3_m3_n23` | 29.294 | 29.100 | 1.002x | 0.801–1.272 | 1.021x | ±3.3% | 5/5 (2 contended) | neutral |
| `bool_gb/matrix_f4_deg4_m2_n23` | 102.001 | 105.921 | 0.963x | 0.932–1.004 | 0.981x | ±3.1% | 5/5 | neutral |
| `bool_gb/matrix_f4_deg4_m3_n15` | 133.660 | 132.048 | 0.994x | 0.974–1.057 | 0.966x | ±7.9% | 5/5 | neutral |
| `bool_gb/matrix_f5_deg4_m2_n23` | 110.392 | 105.046 | 1.041x | 1.024–1.211 | 1.018x | ±3.7% | 5/5 | neutral |
| `bool_gb/solve_inherited_m2_n23` | 162.869 | 166.608 | 1.003x | 0.936–1.095 | 1.002x | ±15.9% | 5/5 | neutral |
| `bool_gb/solve_inherited_m3_n15` | 51.294 | 52.128 | 0.991x | 0.802–1.056 | 1.005x | ±4.8% | 5/5 | neutral |
| `bool_gb/solve_matrixf4_m2_n17` | 204.461 | 194.977 | 1.032x | 0.967–1.091 | 0.995x | ±4.3% | 5/5 | neutral |
| `bool_gb/solve_matrixf5_m2_n17` | 201.158 | 205.719 | 0.988x | 0.852–1.071 | 1.004x | ±6.4% | 5/5 | neutral |
| `bool_gb/wide_found_m5_n15` | 273.334 | 296.982 | 0.975x | 0.911–0.994 | 1.000x | ±9.8% | 5/5 | neutral |
| `bool_gb/wide_mixed_m4_n15` | 321.634 | 339.071 | 0.958x | 0.890–1.061 | 0.996x | ±11.8% | 5/5 | neutral |
| `bool_gb/wide_refute_m3_n31` | 116.957 | 110.737 | 1.025x | 0.965–1.165 | 0.994x | ±13.6% | 5/5 | neutral |
| `dlp/aut_folded_rho_j0_p25` | 6.051 | 6.013 | 1.003x | 0.982–1.041 | 1.001x | ±2.1% | 4/5 (1 contended) | neutral |
| `dlp/bsgs_fast_toy40_w1_x4` | 151.156 | 151.490 | 0.978x | 0.952–1.031 | 0.986x | ±6.1% | 5/5 | neutral |
| `dlp/bsgs_fast_toy40_w4_interval38_x8` | 224.778 | 224.916 | 0.987x | 0.975–1.026 | 1.015x | ±3.7% | 5/5 | neutral |
| `dlp/collab_walkers_demo32_x8` | 0.790 | 0.688 | 1.071x | 0.999–1.540 | 1.009x | ±8.6% | 5/5 (1 contended) | neutral |
| `dlp/ecc2k130_certify_m131_len12` | 2.417 | 2.441 | 0.991x | 0.864–1.007 | 1.008x | ±15.9% | 5/5 | neutral |
| `dlp/gaudry_schost_negation_demo32` | 19.211 | 19.784 | 0.969x | 0.734–1.005 | 1.000x | ±2.4% | 5/5 | neutral |
| `dlp/koblitz_signed_rho_k0_n41` | 53.594 | 53.599 | 0.998x | 0.959–1.059 | 1.002x | ±5.3% | 5/5 | neutral |
| `dlp/pohlig_hellman_smooth47` | 0.106 | 0.084 | 1.278x | 0.986–1.412 | 1.000x | ±42.0% | 5/5 | neutral |
| `dlp/rho_dp_zp_multi_q34_x3` | 3.544 | 3.603 | 0.995x | 0.703–1.037 | 1.008x | ±4.8% | 5/5 | neutral |
| `dlp/rho_dp_zp_wide_q30` | 19.716 | 19.442 | 1.003x | 0.878–2.091 | 1.001x | ±1.1% | 5/5 | neutral |
| `dlp/rho_floyd_zp_q36` | 9.736 | 9.450 | 1.034x | 1.003–1.597 | 1.000x | ±4.6% | 5/5 | neutral |
| `dlp/rho_floyd_zp_wide_q30` | 33.193 | 34.431 | 0.964x | 0.797–1.060 | 0.999x | ±4.4% | 5/5 | neutral |
| `field_ec/binary_scalar_mul_sect163k1_x8` | 16.714 | 16.864 | 0.993x | 0.935–1.029 | 1.005x | ±4.5% | 4/5 (1 contended) | neutral |
| `field_ec/binary_scalar_mul_sect233k1_x4` | 15.163 | 14.749 | 1.002x | 0.967–1.719 | 0.999x | ±14.5% | 4/5 (1 contended) | neutral |
| `field_ec/ct_scalar_mul_p256_x16` | 10.759 | 10.492 | 1.002x | 0.814–1.027 | 1.004x | ±2.8% | 5/5 | neutral |
| `field_ec/ct_scalar_mul_secp256k1_x16` | 6.523 | 6.771 | 0.954x | 0.620–1.476 | 1.002x | ±11.3% | 5/5 | neutral |
| `field_ec/ecdsa_verify_p256_x4` | 46.010 | 45.393 | 1.000x | 0.817–1.031 | 0.990x | ±4.0% | 4/5 (2 contended) | neutral |
| `field_ec/ecdsa_verify_secp256k1_x4` | 48.845 | 48.850 | 0.965x | 0.695–1.030 | 0.991x | ±7.0% | 5/5 | neutral |
| `field_ec/f2m_inv_m163_2k` | 19.123 | 18.411 | 1.039x | 1.003–1.100 | 1.002x | ±7.7% | 5/5 | neutral |
| `field_ec/f2m_mul_sqr_m131_64k` | 7.531 | 7.552 | 0.999x | 0.927–1.020 | 1.004x | ±1.2% | 5/5 | neutral |
| `field_ec/f2m_mul_sqr_m163_64k` | 8.251 | 8.334 | 0.992x | 0.863–1.056 | 0.993x | ±1.3% | 5/5 | neutral |
| `field_ec/f2m_mul_sqr_m233_64k` | 9.483 | 9.367 | 1.022x | 0.963–1.461 | 0.994x | ±4.6% | 5/5 | neutral |
| `field_ec/fastfield_p61_inv` | 6.928 | 6.994 | 0.989x | 0.902–1.001 | 0.992x | ±0.7% | 5/5 | neutral |
| `field_ec/frobenius_canon_n53_bulk_256k` | 5.422 | 5.512 | 0.984x | 0.799–0.992 | 1.001x | ±1.6% | 5/5 | neutral |
| `field_ec/frobenius_canon_n53_shift_256k` | 8.211 | 8.248 | 1.003x | 0.980–1.021 | 1.000x | ±0.4% | 5/5 | neutral |
| `field_ec/gf2_n53_batch_inv_4x64k` | 3.788 | 3.732 | 1.015x | 0.986–1.236 | 0.991x | ±1.4% | 5/5 | neutral |
| `field_ec/gf2_n53_inv_16k` | 6.120 | 6.071 | 1.003x | 0.936–1.080 | 1.000x | ±0.8% | 5/5 | neutral |
| `field_ec/gf2_n53_mul_sqr_1m` | 4.223 | 4.261 | 0.991x | 0.971–1.201 | 0.994x | ±30.0% | 4/5 (1 contended) | neutral |
| `field_ec/gf3m_mul_m97` | 18.258 | 18.493 | 0.999x | 0.971–1.409 | 0.995x | ±8.2% | 4/5 (1 contended) | neutral |
| `field_ec/hyperelliptic_g2_scalar_mul_x4` | 57.141 | 50.971 | 0.965x | 0.674–1.334 | 1.018x | ±20.2% | 4/5 (1 contended) | neutral |
| `field_ec/isogeny_field_p256_mul_sqr_pow` | 37.714 | 37.995 | 0.997x | 0.963–1.038 | 0.999x | ±2.3% | 5/5 | neutral |
| `field_ec/koblitz_curve_mul_n53_1k` | 2.156 | 2.156 | 1.002x | 0.991–1.006 | 0.997x | ±31.8% | 4/5 (2 contended) | neutral |
| `field_ec/koblitz_n53_add_many_lazy_64x2k` | 5.754 | 5.665 | 1.007x | 0.804–1.253 | 0.997x | ±1.0% | 5/5 | neutral |
| `field_ec/koblitz_n53_fast_mul_2k` | 5.815 | 5.430 | 1.076x | 1.002–1.202 | 1.003x | ±11.9% | 5/5 | neutral |
| `field_ec/point_scalar_mul_ct_secp256k1_x1` | 69.826 | 68.414 | 1.016x | 0.977–1.124 | 1.001x | ±9.2% | 5/5 | neutral |
| `field_ec/point_scalar_mul_p256_x8` | 14.043 | 14.203 | 0.994x | 0.867–1.778 | 0.994x | ±3.3% | 5/5 | neutral |
| `field_ec/point_scalar_mul_secp256k1_x8` | 14.105 | 14.259 | 0.989x | 0.932–1.003 | 1.005x | ±3.7% | 5/5 | neutral |
| `fp_gb/buchberger_katsura3_p32003` | 46.988 | 47.097 | 0.992x | 0.657–1.070 | 1.003x | ±4.1% | 5/5 | neutral |
| `fp_gb/f4_autoreduce_pkm_m2_t4_x2` | 15.401 | 16.182 | 0.979x | 0.906–1.157 | 1.009x | ±2.2% | 5/5 (1 contended) | neutral |
| `fp_gb/f4_cubic_n4_p31` | 1.883 | 1.896 | 0.986x | 0.976–1.016 | 1.006x | ±0.9% | 5/5 | neutral |
| `fp_gb/f4_cyclic5_p65521` | 0.817 | 0.844 | 0.992x | 0.610–1.637 | 0.997x | ±25.0% | 5/5 | neutral |
| `fp_gb/f4_katsura6_p65521` | 3.651 | 3.719 | 1.002x | 0.858–1.036 | 1.021x | ±7.5% | 5/5 | neutral |
| `fp_gb/f4_pkm_kummer_m2_t5` | 26.584 | 26.424 | 0.984x | 0.924–1.053 | 1.005x | ±3.5% | 5/5 | neutral |
| `fp_gb/f4_pkm_kummer_m3_t2` | 3.367 | 3.337 | 1.015x | 0.985–1.074 | 0.999x | ±10.4% | 4/5 (1 contended) | neutral |
| `fp_gb/f4_quad_n7_p65521` | 12.230 | 12.113 | 1.043x | 0.626–1.174 | 1.015x | ±4.7% | 4/5 (1 contended) | neutral |
| `fp_gb/f4_solve_cubic_n3_p29` | 3.013 | 3.033 | 1.011x | 0.984–1.014 | 1.004x | ±1.2% | 5/5 | neutral |
| `fp_gb/f4_solve_quad_n4_p31` | 3.544 | 3.495 | 1.001x | 0.741–1.030 | 0.999x | ±4.0% | 4/5 (1 contended) | neutral |
| `fp_gb/f4_solve_quart_n3_p31` | 10.173 | 10.520 | 0.979x | 0.943–0.994 | 0.992x | ±0.2% | 4/5 (1 contended) | slower † |
| `fp_gb/gaudry_quartic_generic_d3_p269` | 12.294 | 12.466 | 0.998x | 0.855–1.017 | 0.996x | ±5.0% | 5/5 | neutral |
| `fp_gb/gaudry_s4_macaulay_p1039` | 32.946 | 33.693 | 0.978x | 0.878–1.273 | 0.980x | ±0.8% | 5/5 | neutral |
| `fp_gb/sig_tower_kummer_m2_n12` | 70.261 | 66.990 | 1.028x | 0.963–1.288 | 0.979x | ±5.7% | 5/5 | neutral |
| `fp_gb/sig_tower_kummer_m3_n9` | 90.511 | 91.356 | 1.011x | 0.940–1.030 | 1.008x | ±6.6% | 5/5 | neutral |
| `fp_gb/tower_f4_kummer_m2_n12` | 61.225 | 60.276 | 1.044x | 0.800–1.098 | 1.003x | ±6.7% | 5/5 | neutral |
| `fp_gb/tower_f4_kummer_m2_n12_planted` | 67.706 | 67.989 | 1.001x | 0.990–1.063 | 0.988x | ±9.2% | 4/5 (1 contended) | neutral |
| `fp_gb/tower_f4_kummer_m3_n9` | 121.658 | 121.173 | 0.995x | 0.905–1.038 | 0.986x | ±3.9% | 5/5 | neutral |
| `fp_gb/tower_f4_kummer_m4_n8` | 45.315 | 46.510 | 0.971x | 0.887–1.023 | 0.990x | ±7.1% | 5/5 | neutral |
| `gf2_la/echelon_macaulay_k23m2d4` | 36.291 | 35.646 | 1.008x | 0.851–1.079 | 1.001x | ±8.0% | 5/5 | neutral |
| `gf2_la/echelon_macaulay_k5m3d5` | 20.800 | 21.323 | 0.970x | 0.927–1.057 | 0.976x | ±4.8% | 5/5 | neutral |
| `gf2_la/f5_criterion_k23m2d6` | 157.254 | 161.911 | 0.957x | 0.902–1.090 | 1.011x | ±10.5% | 5/5 | neutral |
| `gf2_la/f5_criterion_k5m3d7` | 14.080 | 13.589 | 1.010x | 0.686–1.230 | 1.043x | ±14.0% | 5/5 | neutral |
| `gf2_la/legacy_echelon_macaulay_k23m2d3_x16` | 10.086 | 10.033 | 0.990x | 0.863–1.088 | 1.007x | ±2.4% | 4/5 (1 contended) | neutral |
| `gf2_la/legacy_rref_macaulay_k23m3d3` | 11.489 | 11.005 | 1.078x | 0.972–1.228 | 1.010x | ±11.9% | 5/5 | neutral |
| `gf2_la/legacy_rref_macaulay_k5m3d5` | 115.085 | 110.029 | 1.014x | 0.924–1.261 | 1.000x | ±15.4% | 5/5 | neutral |
| `gf2_la/rank_refute_anf_s4_n6l3d4` | 110.255 | 109.401 | 1.005x | 0.799–1.091 | 1.008x | ±8.0% | 5/5 | neutral |
| `gf2_la/rref_macaulay_k15m3d4` | 91.747 | 90.414 | 0.996x | 0.968–1.015 | 1.002x | ±5.2% | 5/5 | neutral |
| `gf2_la/rref_macaulay_k17m2d5` | 144.695 | 134.209 | 1.078x | 0.960–1.175 | 1.041x | ±2.9% | 5/5 | neutral |
| `gf2_la/rref_macaulay_k23m2d3_x16` | 13.034 | 10.252 | 1.260x | 0.652–1.562 | 1.010x | n/a | 4/5 (3 contended) | neutral |
| `gf2_la/rref_macaulay_k23m2d4` | 64.821 | 63.907 | 0.996x | 0.984–1.090 | 1.005x | ±5.5% | 5/5 (1 contended) | neutral |
| `gf2_la/rref_macaulay_k5m3d5` | 31.467 | 30.496 | 1.000x | 0.977–1.066 | 1.030x | ±2.2% | 5/5 | neutral |
| `gf2_la/rref_random_2048` | 7.023 | 7.070 | 0.993x | 0.607–1.000 | 0.988x | ±3.9% | 5/5 | neutral |
| `gf2_la/rref_random_4940x8357` | 104.134 | 104.624 | 0.987x | 0.906–1.056 | 0.994x | ±3.4% | 5/5 | neutral |
| `gf2_la/rref_random_4940x8357_w17` | 85.658 | 85.319 | 1.010x | 0.815–1.072 | 1.005x | ±5.9% | 5/5 | neutral |
| `gf2_la/sparse_finish_macaulay_k7m3d5` | 20.585 | 20.059 | 1.026x | 0.783–1.055 | 0.985x | ±11.7% | 5/5 (1 contended) | neutral |
| `gf2_la/sparse_high_macaulay_k5m3d5` | 33.506 | 33.708 | 0.998x | 0.994–1.005 | 0.999x | ±2.2% | 4/5 (1 contended) | neutral |
| `gf2_la/sparse_rank_refute_macaulay_k5m3d5` | 95.005 | 95.053 | 1.003x | 0.973–1.061 | 0.992x | ±2.5% | 5/5 | neutral |
| `pdp/build_system_m2_n23` | 0.387 | 0.308 | 1.108x | 0.891–1.507 | 1.000x | ±27.7% | 4/5 (1 contended) | neutral |
| `pdp/build_system_m3_n15` | 1.466 | 1.462 | 1.003x | 0.963–1.100 | 1.001x | ±2.2% | 4/5 (1 contended) | neutral |
| `pdp/descend_s3_n31_np20` | 9.924 | 9.497 | 1.049x | 0.992–1.332 | 0.998x | ±6.8% | 5/5 | neutral |
| `pdp/descend_s4_n23_np6` | 7.426 | 7.686 | 0.963x | 0.959–0.970 | 0.960x | ±1.9% | 4/5 (1 contended) | slower |
| `pdp/enumerate_m2_n23` | 3.575 | 3.714 | 0.960x | 0.772–1.128 | 0.987x | ±5.0% | 4/5 (2 contended) | neutral |
| `pdp/enumerate_m3_n15` | 2.438 | 2.463 | 0.989x | 0.743–1.014 | 1.001x | ±0.9% | 5/5 | neutral |
| `pdp/groebner_m2_n17` | 20.113 | 19.849 | 1.013x | 0.997–1.128 | 0.998x | ±1.2% | 4/5 (1 contended) | neutral |
| `pdp/groebner_m3_n9` | 19.512 | 20.339 | 0.959x | 0.932–1.021 | 1.000x | ±0.6% | 5/5 | neutral |
| `pdp/instantiate_m2_n23` | 5.513 | 5.642 | 0.974x | 0.827–1.032 | 0.994x | ±1.2% | 5/5 | neutral |
| `pdp/instantiate_m3_n15` | 4.740 | 4.749 | 0.999x | 0.989–1.141 | 0.999x | ±9.3% | 5/5 | neutral |
| `pdp/mitm_m3_n31` | 26.952 | 27.602 | 0.995x | 0.976–1.017 | 0.993x | ±8.1% | 5/5 (1 contended) | neutral |
| `pdp/mitm_m4_n31` | 10.670 | 10.544 | 1.004x | 0.938–1.079 | 0.998x | ±1.4% | 5/5 | neutral |
| `pdp/pair_table_build_n31_l10` | 106.723 | 98.284 | 1.114x | 0.845–1.227 | 0.995x | ±24.3% | 5/5 | neutral |
| `pdp/relation_solver_u1024` | 25.160 | 22.991 | 1.087x | 0.982–1.311 | 1.019x | ±8.8% | 4/5 (1 contended) | neutral |
| `pdp/sat_m2_n17` | 40.910 | 41.912 | 0.993x | 0.782–1.002 | 0.993x | ±2.9% | 4/5 (1 contended) | neutral |
| `pdp/sat_m6_n7` | 13.070 | 13.291 | 0.987x | 0.978–1.068 | 0.999x | ±1.6% | 5/5 (1 contended) | neutral |
| `pdp/semaev_s5_symbolic` | 8.042 | 7.918 | 1.004x | 0.962–1.043 | 1.004x | ±2.9% | 5/5 (1 contended) | neutral |
| `pdp/subspace_oracle_n20_l7` | 19.251 | 18.846 | 1.014x | 0.996–1.331 | 1.003x | ±7.9% | 5/5 | neutral |
| `pdp/symmetrised_groebner_m2_n17` | 150.798 | 150.137 | 1.013x | 0.963–1.235 | 0.998x | ±6.2% | 5/5 | neutral |
| `pdp/symmetrised_groebner_m3_n15` | 95.470 | 97.004 | 0.979x | 0.803–1.030 | 0.994x | ±1.2% | 5/5 | neutral |
| `pdp/weil_descend_s4_sym_n17_l4` | 2.575 | 2.541 | 0.985x | 0.954–1.052 | 0.996x | ±5.0% | 5/5 | neutral |
| `relation/block_wiedemann_r45_n768_w13` | 41.423 | 41.782 | 0.989x | 0.985–1.016 | 1.004x | ±1.5% | 5/5 | neutral |
| `relation/koblitz_collect_k0n41_w164_u4096` | 22.661 | 21.966 | 0.992x | 0.861–1.038 | 0.983x | ±2.6% | 5/5 | neutral |
| `relation/koblitz_collect_k0n53_w477_u1024` | 17.529 | 16.145 | 1.123x | 0.751–1.193 | 1.013x | ±11.0% | 5/5 | neutral |
| `relation/koblitz_descent_k0n53_m2_x4` | 32.185 | 32.375 | 0.971x | 0.608–1.102 | 0.988x | ±30.0% | 5/5 | neutral |
| `relation/koblitz_pair_table_k0n41_folded` | 7.586 | 7.865 | 0.958x | 0.663–1.591 | 1.032x | ±42.0% | 5/5 (1 contended) | neutral |
| `relation/koblitz_rung_k0n31` | 13.999 | 13.725 | 1.007x | 0.934–1.040 | 0.984x | ±2.7% | 5/5 | neutral |
| `relation/pq_sparse_solve_m127_c640` | 28.283 | 31.641 | 0.904x | 0.845–1.068 | 0.997x | ±3.0% | 5/5 (1 contended) | neutral |
| `relation/pq_wiedemann_m127_c160` | 36.229 | 36.091 | 1.017x | 0.999–1.042 | 1.001x | ±0.4% | 5/5 | neutral |
| `relation/residual_walk_lmw_b22_x3` | 50.445 | 50.980 | 1.003x | 0.971–1.046 | 1.018x | ±8.3% | 5/5 | neutral |
| `relation/sparse_solve_r45_c4800_w3` | 57.210 | 61.207 | 0.974x | 0.644–1.004 | 1.000x | ±4.2% | 4/5 (1 contended) | neutral |
| `sat/koblitz_bool_cnf_m3_n15_b1500_x2` | 145.891 | 151.056 | 0.991x | 0.927–1.073 | 1.002x | ±1.9% | 5/5 | neutral |
| `sat/koblitz_bool_native_m2_n19_x2` | 143.749 | 139.986 | 1.045x | 0.930–1.140 | 1.000x | ±3.9% | 5/5 | neutral |
| `sat/koblitz_bool_native_m3_n15_x3` | 59.136 | 59.623 | 1.001x | 0.973–1.015 | 1.004x | ±1.4% | 4/5 (1 contended) | neutral |
| `sat/random3sat_n150_r426_x12` | 163.839 | 161.660 | 1.013x | 0.939–1.035 | 1.000x | ±3.8% | 5/5 | neutral |
| `sat/random3sat_n200_r426_x5` | 154.146 | 157.841 | 1.007x | 0.885–1.130 | 0.999x | ±6.1% | 5/5 | neutral |
| `sat/semaev_s4_encode_n19l6_x4` | 6.867 | 6.885 | 1.009x | 0.942–1.556 | 1.002x | ±5.3% | 5/5 | neutral |
| `sat/semaev_s4_n15l5_cnf_x2` | 54.356 | 55.333 | 1.005x | 0.899–1.115 | 0.998x | ±7.8% | 4/5 (1 contended) | neutral |
| `sat/semaev_s4_n15l5_sat_x4` | 128.595 | 129.577 | 0.992x | 0.932–1.074 | 1.002x | ±9.0% | 5/5 | neutral |
| `sat/semaev_s4_n15l5_unsat_x1` | 178.090 | 179.549 | 0.995x | 0.968–1.175 | 1.006x | ±8.7% | 5/5 | neutral |
| `sat/semaev_s4_n17l6_sat_x1` | 180.149 | 172.319 | 1.005x | 0.979–1.108 | 0.996x | ±6.6% | 5/5 | neutral |
| `sat/xor_planted_n120_x6` | 154.440 | 157.048 | 0.998x | 0.934–1.083 | 1.002x | ±11.6% | 5/5 | neutral |
