# RESULTS (round 2) [pcc:unverified] local development check

git HEAD at run time: `473c4064294aea5131fb869b8aaa30810f9ed228`. Unit: GAE_H (field mults + 3 inversions of one Cantor addition on H; field adds not counted). Native work priced at w per operation on every arm.

## Instances and end-to-end verification

| p | candidates tried | accepted | targets | logs found on C | verified k G' = Q' on H | found, not verified | ell | N/ell | b (factor base) | IC attempts failed before ok |
|--:|--:|--:|--:|--:|--:|--:|:--|:--|:--|:--|
| 101 | 80 | 3 | 9 | 9 | 9 | 0 | [26993, 91951, 23899] | [44, 12, 48] | [41, 41, 41] | [1, 1, 1] |
| 211 | 26 | 3 | 9 | 9 | 9 | 0 | [771809, 147449, 598093] | [12, 64, 16] | [59, 59, 59] | [1, 1, 1] |
| 521 | 54 | 3 | 9 | 9 | 9 | 0 | [2695267, 5864819, 5541793] | [54, 24, 24] | [92, 92, 92] | [1, 1, 1] |
| 1021 | 30 | 3 | 9 | 9 | 9 | 0 | [258446893, 32928713, 38004139] | [4, 32, 28] | [64, 64, 64] | [0, 0, 0] |

Rejections (not charged): p=101: {'no prime ell || N with N/ell <= 64': 61, 'diem driver: pool negation failed': 3, 'diem driver: class difference failed': 1, "diem driver: ell * G' !": 5, 'diem driver: cofactor clearing failed': 7}; p=211: {'no prime ell || N with N/ell <= 64': 22, 'no unique quartic': 1}; p=521: {'no prime ell || N with N/ell <= 64': 47, "diem driver: ell * G' !": 2, 'diem driver: cofactor clearing failed': 1, 'diem driver: pool negation failed': 1}; p=1021: {'no prime ell || N with N/ell <= 64': 26, "diem driver: ell * G' !": 1}

`verified_on_C` is inconclusive for {'101': 4, '211': 4, '521': 2, '1021': 2} targets (the scalar multiple on the quartic returned `None`); the verification on H is authoritative. Group order checks (N G = 0 on H and ell G' = 0 on C): H {'101': True, '211': True, '521': True, '1021': True}, C {'101': True, '211': True, '521': True, '1021': True}. Z3 order equals Z2B order on every curve: {'101': True, '211': True, '521': True, '1021': True}.

## Measured negation-map rho on J_H

| p | runs | verified | mean steps | steps / sqrt(ell) | mean field ops (M+I+adds) | analytic reference sqrt(pi ell/4) x ops/add | measured / reference | cycle escapes per step | A/A mean steps (set 0, set 1) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 101 | 54 | 54 | 708 | 3.69 | 1.89e+06 | 1.63e+05 | 12.66 | 0.014 | 907, 509 |
| 211 | 54 | 54 | 1042 | 1.58 | 2.22e+06 | 5.37e+05 | 4.52 | 0.014 | 996, 1088 |
| 521 | 54 | 54 | 2931 | 1.35 | 4e+06 | 1.7e+06 | 2.35 | 0.014 | 2807, 3055 |
| 1021 | 54 | 54 | 13612 | 1.42 | 1.39e+07 | 7.43e+06 | 1.87 | 0.013 | 13810, 13414 |

## S (single target, GAE_H) and S / rho

### price w = 0

| p | rho (measured) | S, Z3 charged | S / rho (Z3) | S, Z2B charged | S / rho (Z2B) | S, no order phase | S / rho (none) | main / A/A (Z3) |
|--:|--:|--:|:--|--:|:--|--:|:--|--:|
| 101 | 2.12e+03 | 6.19e+04 | 29.2 [26, 31] | 1.1e+04 | 5.21 [2.6, 7.57] | 9.94e+03 | 4.7 [2.11, 7.07] | 1.08 |
| 211 | 2.47e+03 | 3.9e+05 | 158 [141, 180] | 1.6e+04 | 6.48 [3.2, 12.1] | 1.17e+04 | 4.73 [1.61, 10.4] | 1.01 |
| 521 | 4.44e+03 | 4.94e+06 | 1.11e+03 [971, 1.47e+03] | 2.95e+04 | 6.65 [5.7, 9.05] | 4.26e+03 | 0.96 [0.731, 1.54] | 1.00 |
| 1021 | 1.55e+04 | 3.52e+07 | 2.28e+03 [1.32e+03, 3.91e+03] | 1.01e+05 | 6.5 [3.92, 11] | 4.57e+03 | 0.295 [0.249, 0.312] | 1.00 |

### price w = 0.1

| p | rho (measured) | S, Z3 charged | S / rho (Z3) | S, Z2B charged | S / rho (Z2B) | S, no order phase | S / rho (none) | main / A/A (Z3) |
|--:|--:|--:|:--|--:|:--|--:|:--|--:|
| 101 | 2.33e+03 | 1.82e+05 | 78.3 [73.3, 83.7] | 2.86e+04 | 12.3 [9.64, 14.2] | 2.65e+04 | 11.4 [8.77, 13.3] | 1.02 |
| 211 | 2.79e+03 | 1.37e+06 | 490 [438, 584] | 5.35e+04 | 19.2 [15, 23.6] | 4.47e+04 | 16 [12.2, 20.6] | 1.00 |
| 521 | 5.33e+03 | 1.92e+07 | 3.6e+03 [3.12e+03, 4.87e+03] | 1.44e+05 | 27.1 [23.3, 36.8] | 9.18e+04 | 17.2 [14.7, 23.5] | 1.00 |
| 1021 | 1.96e+04 | 1.42e+08 | 7.24e+03 [4.17e+03, 1.26e+04] | 2.85e+05 | 14.6 [8.59, 25.1] | 8.48e+04 | 4.33 [2.71, 7.25] | 1.00 |

### price w = 1

| p | rho (measured) | S, Z3 charged | S / rho (Z3) | S, Z2B charged | S / rho (Z2B) | S, no order phase | S / rho (none) | main / A/A (Z3) |
|--:|--:|--:|:--|--:|:--|--:|:--|--:|
| 101 | 4.26e+03 | 1.27e+06 | 298 [279, 333] | 1.87e+05 | 43.9 [40.2, 48.8] | 1.75e+05 | 41.2 [37.7, 45.8] | 1.00 |
| 211 | 5.64e+03 | 1.02e+07 | 1.8e+03 [1.54e+03, 2.4e+03] | 3.91e+05 | 69.4 [58.6, 88.5] | 3.42e+05 | 60.7 [51.1, 76.9] | 1.00 |
| 521 | 1.33e+04 | 1.48e+08 | 1.11e+04 [9.31e+03, 1.62e+04] | 1.18e+06 | 88.2 [73.9, 129] | 8.8e+05 | 65.9 [55.2, 96.5] | 1.00 |
| 1021 | 5.68e+04 | 1.1e+09 | 1.94e+04 [1.09e+04, 3.51e+04] | 1.95e+06 | 34.3 [19.7, 61.6] | 8.07e+05 | 14.2 [8.4, 25.2] | 1.00 |

Intervals: 95% bootstrap over the 3 curves of each p (ratio of means); with 3 curves they are wide and indicative only.

## Phase shares of S (Z3 charged; mean over curves)

- p = 101, w = 0: transfer_points_fit 0.2%, curve_setup_C 0.2%, pushforward_classes_cofactor 0.9%, relation_phase_C 0.3%, linear_algebra_C 0.0%, failed_attempts 1.2%, order_z3 84.3%, descent 12.9%, pullback_H 0.0%
- p = 101, w = 0.1: transfer_points_fit 0.1%, curve_setup_C 0.1%, pushforward_classes_cofactor 0.3%, relation_phase_C 0.4%, linear_algebra_C 8.7%, failed_attempts 0.4%, order_z3 85.5%, descent 4.5%, pullback_H 0.0%
- p = 101, w = 1: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.5%, linear_algebra_C 12.5%, failed_attempts 0.1%, order_z3 86.2%, descent 0.7%, pullback_H 0.0%
- p = 211, w = 0: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.1%, relation_phase_C 0.1%, linear_algebra_C 0.0%, failed_attempts 0.2%, order_z3 97.1%, descent 2.5%, pullback_H 0.0%
- p = 211, w = 0.1: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.1%, linear_algebra_C 2.3%, failed_attempts 0.1%, order_z3 96.7%, descent 0.7%, pullback_H 0.0%
- p = 211, w = 1: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.1%, linear_algebra_C 3.1%, failed_attempts 0.0%, order_z3 96.6%, descent 0.1%, pullback_H 0.0%
- p = 521, w = 0: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.0%, linear_algebra_C 0.0%, failed_attempts 0.0%, order_z3 99.9%, descent 0.0%, pullback_H 0.0%
- p = 521, w = 0.1: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.0%, linear_algebra_C 0.4%, failed_attempts 0.0%, order_z3 99.5%, descent 0.0%, pullback_H 0.0%
- p = 521, w = 1: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.0%, linear_algebra_C 0.6%, failed_attempts 0.0%, order_z3 99.4%, descent 0.0%, pullback_H 0.0%
- p = 1021, w = 0: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.0%, linear_algebra_C 0.0%, failed_attempts 0.0%, order_z3 100.0%, descent 0.0%, pullback_H 0.0%
- p = 1021, w = 0.1: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.0%, linear_algebra_C 0.1%, failed_attempts 0.0%, order_z3 99.9%, descent 0.0%, pullback_H 0.0%
- p = 1021, w = 1: transfer_points_fit 0.0%, curve_setup_C 0.0%, pushforward_classes_cofactor 0.0%, relation_phase_C 0.0%, linear_algebra_C 0.1%, failed_attempts 0.0%, order_z3 99.9%, descent 0.0%, pullback_H 0.0%

## Exponent fits (log-log slope of mean cost vs p, 4 points, bootstrap over curves)

| quantity | slope [95%] | points |
|:--|:--|--:|
| S_z3_w0 | 2.75 [2.72, 2.78] | 4 |
| S_z3_w0.1 | 2.88 [2.87, 2.89] | 4 |
| S_z3_w1 | 2.93 [2.93, 2.93] | 4 |
| S_z2b_w0 | 0.914 [0.752, 1.16] | 4 |
| S_z2b_w0.1 | 1.01 [0.939, 1.09] | 4 |
| S_z2b_w1 | 1.04 [1.03, 1.05] | 4 |
| S_none_w0 | -0.439 [-0.772, -0.0382] | 4 |
| S_none_w0.1 | 0.545 [0.459, 0.632] | 4 |
| S_none_w1 | 0.714 [0.696, 0.735] | 4 |
| rho_w0 | 0.826 [0.624, 1.02] | 4 |
| rho_walk_only_scaled_to_ell_p3_w0 | 1.11 [1.01, 1.26] | 4 |
| rho_scaled_to_ell_p3_w0 | 0.687 [0.592, 0.84] | 4 |
| rho_w0.1 | 0.887 [0.674, 1.08] | 4 |
| rho_walk_only_scaled_to_ell_p3_w0.1 | 1.11 [1.01, 1.25] | 4 |
| rho_scaled_to_ell_p3_w0.1 | 0.747 [0.656, 0.909] | 4 |
| rho_w1 | 1.09 [0.861, 1.3] | 4 |
| rho_walk_only_scaled_to_ell_p3_w1 | 1.11 [1.02, 1.26] | 4 |
| rho_scaled_to_ell_p3_w1 | 0.951 [0.86, 1.09] | 4 |

## S / rho with rho rescaled to ell = p^3 (removes the ell spread between curves)

| p | w | rho scaled | S/rho_scaled (Z3) | (Z2B) | (none) |
|--:|--:|--:|:--|:--|:--|
| 101 | 0 | 1.13e+04 | 5.48 [4.2, 9.26] | 0.977 [0.422, 2.26] | 0.881 [0.341, 2.11] |
| 101 | 0.1 | 1.24e+04 | 14.7 [11.9, 23.4] | 2.3 [1.56, 4.25] | 2.13 [1.42, 3.98] |
| 101 | 1 | 2.26e+04 | 56.1 [45.2, 85.6] | 8.27 [6.51, 13] | 7.76 [6.1, 12.2] |
| 211 | 0 | 1.22e+04 | 32 [22.6, 40.5] | 1.31 [0.48, 3.05] | 0.958 [0.225, 2.63] |
| 211 | 0.1 | 1.37e+04 | 100 [73.1, 126] | 3.92 [2.41, 5.96] | 3.27 [1.93, 5.2] |
| 211 | 1 | 2.69e+04 | 377 [301, 440] | 14.5 [11.1, 17.1] | 12.7 [9.63, 15.1] |
| 521 | 0 | 2.47e+04 | 200 [192, 206] | 1.2 [1.13, 1.25] | 0.173 [0.145, 0.213] |
| 521 | 0.1 | 2.95e+04 | 650 [617, 673] | 4.89 [4.6, 5.09] | 3.11 [2.92, 3.25] |
| 521 | 1 | 7.34e+04 | 2.01e+03 [1.84e+03, 2.24e+03] | 16 [14.6, 17.8] | 12 [10.9, 13.3] |
| 1021 | 0 | 5.4e+04 | 652 [619, 688] | 1.86 [1.73, 1.93] | 0.0845 [0.0471, 0.154] |
| 1021 | 0.1 | 6.81e+04 | 2.08e+03 [1.99e+03, 2.22e+03] | 4.18 [3.93, 4.41] | 1.24 [1.13, 1.33] |
| 1021 | 1 | 1.95e+05 | 5.65e+03 [5.38e+03, 6.18e+03] | 9.98 [9.51, 10.8] | 4.14 [3.87, 4.43] |

## rho without its one-off setup (32 multipliers + first start), rescaled to ell = p^3

| p | setup share of rho cost (w=0) | rho walk-only scaled, w=0 | S/rho_walk (Z2B), w=0 | S/rho_walk (none), w=0 | S/rho_walk (none), w=0.1 | S/rho_walk (none), w=1 |
|--:|--:|--:|:--|:--|:--|:--|
| 101 | 0.73 | 3.95e+03 | 2.79 [1.13, 6.21] | 2.52 [0.914, 5.8] | 5.21 [3.27, 9.37] | 11.5 [8.75, 17.7] |
| 211 | 0.61 | 5.11e+03 | 3.13 [1.38, 6.54] | 2.29 [0.645, 5.63] | 6.79 [4.74, 9.83] | 17.2 [14.3, 19.5] |
| 521 | 0.39 | 1.67e+04 | 1.77 [1.56, 2.14] | 0.256 [0.199, 0.365] | 4.27 [3.77, 5.03] | 13.5 [12, 15.7] |
| 1021 | 0.14 | 4.8e+04 | 2.1 [1.99, 2.28] | 0.0952 [0.0541, 0.162] | 1.37 [1.26, 1.46] | 4.27 [4.02, 4.64] |

## Phase costs without the order phase (GAE_H, mean over curves)

- p = 101, w = 0: transfer_points_fit 153, curve_setup_C 126, pushforward_classes_cofactor 523, relation_phase_C 158, linear_algebra_C 0, failed_attempts 706, descent 8.26e+03, pullback_H 18.7
- p = 101, w = 0.1: transfer_points_fit 153, curve_setup_C 126, pushforward_classes_cofactor 523, relation_phase_C 807, linear_algebra_C 1.58e+04, failed_attempts 795, descent 8.26e+03, pullback_H 18.7
- p = 101, w = 1: transfer_points_fit 153, curve_setup_C 126, pushforward_classes_cofactor 523, relation_phase_C 6.65e+03, linear_algebra_C 1.58e+05, failed_attempts 1.6e+03, descent 8.27e+03, pullback_H 18.7
- p = 211, w = 0: transfer_points_fit 173, curve_setup_C 115, pushforward_classes_cofactor 446, relation_phase_C 247, linear_algebra_C 0, failed_attempts 659, descent 1e+04, pullback_H 24.5
- p = 211, w = 0.1: transfer_points_fit 173, curve_setup_C 115, pushforward_classes_cofactor 446, relation_phase_C 1.37e+03, linear_algebra_C 3.17e+04, failed_attempts 827, descent 1e+04, pullback_H 24.5
- p = 211, w = 1: transfer_points_fit 173, curve_setup_C 115, pushforward_classes_cofactor 446, relation_phase_C 1.15e+04, linear_algebra_C 3.17e+05, failed_attempts 2.33e+03, descent 1e+04, pullback_H 24.5
- p = 521, w = 0: transfer_points_fit 232, curve_setup_C 110, pushforward_classes_cofactor 515, relation_phase_C 369, linear_algebra_C 0, failed_attempts 811, descent 2.2e+03, pullback_H 28.2
- p = 521, w = 0.1: transfer_points_fit 232, curve_setup_C 110, pushforward_classes_cofactor 515, relation_phase_C 2.48e+03, linear_algebra_C 8.51e+04, failed_attempts 1.21e+03, descent 2.2e+03, pullback_H 28.2
- p = 521, w = 1: transfer_points_fit 232, curve_setup_C 110, pushforward_classes_cofactor 515, relation_phase_C 2.14e+04, linear_algebra_C 8.51e+05, failed_attempts 4.8e+03, descent 2.22e+03, pullback_H 28.2
- p = 1021, w = 0: transfer_points_fit 323, curve_setup_C 125, pushforward_classes_cofactor 409, relation_phase_C 351, linear_algebra_C 0, failed_attempts 0, descent 3.32e+03, pullback_H 34.9
- p = 1021, w = 0.1: transfer_points_fit 323, curve_setup_C 125, pushforward_classes_cofactor 409, relation_phase_C 2.45e+03, linear_algebra_C 7.81e+04, failed_attempts 0, descent 3.33e+03, pullback_H 34.9
- p = 1021, w = 1: transfer_points_fit 323, curve_setup_C 125, pushforward_classes_cofactor 409, relation_phase_C 2.13e+04, linear_algebra_C 7.81e+05, failed_attempts 0, descent 3.35e+03, pullback_H 34.9

## Predictions

- Q1: `{"diem_field_ops_per_target_M_plus_I_w0": 994223.6666666666, "first_round_bsgs": 18300000.0, "ratio_bsgs_over_diem_w0": 18.406321045800894, "diem_rel_la_descent_in_field_op_equivalents": {"w0": 1064749.6666666667, "w0.1": 37196761.81303704, "w1": 362384871.1303704}, "ratio_bsgs_over_diem_by_w": {"w0": 17.187138510491824, "w0.1": 0.49197830961688876, "w1": 0.05049879688111056}}`
- Q2: `{"order_z3_share": {"w0": 0.9991368322074008, "w0.1": 0.9952170027939585, "w1": 0.9940353844343139}}`
- Q3: `{"measured_over_analytic_ref_by_p": {"101": 12.662596379822755, "211": 4.523530510729252, "521": 2.348663488122688, "1021": 1.8679796825847417}}`
- Q4: `{"w0": {"z3": [1111.9048153300382, 971.4781154582641, 1467.5338290786908], "z2b": [6.649348761205574, 5.702211666632405, 9.051880775158471], "none": [0.9597144792639546, 0.7305512642725788, 1.5441815853596652]}, "w0.1": {"z3": [3599.278450769629, 3115.0283235357774, 4872.418633280811], "z2b": [27.06245923817005, 23.25654271801212, 36.83751447512216], "none": [17.21533359729915, 14.733317143823934, 23.509947216282097]}, "w1": {"z3": [11052.313125786894, 9305.093231290153, 16192.575041225216], "z2b": [88.22722276308656, 73.94928983015215, 129.2158877432148], "none": [65.92281011969081, 55.16998587958923, 96.5391146804542]}}`

## Push-forward test re-run (plane_quartic_pic copy)

`{"instances_with_quartic": 58, "instances_without": 2, "pairs_total": 1196, "pairs_tested_under_some_base": 1029, "pairs_untested_under_all_bases": 167, "instances_single_valued_under_every_base": 58, "base_runs": 49, "panics_in_base_runs": 175, "no_phi_in_base_runs": 35, "negctl_base_runs_with_pairs": 48, "negctl_base_runs_separating": 48}`

Single base point, bound 12 only: `{"pairs_tested": 1538, "panics": 254, "no_phi": 44}`
