# Tables: lin21 wall-gap pilot (written by summary.rs)

Every number is an unverified pilot assertion [pcc:unverified]. Wall times are single runs on a shared machine.

Rows 822; censored 0; rows whose verdict differs from the pair oracle or whose SAT model fails a check: 0; targets with any arm disagreeing with the oracle: 0 of 176; SAT models 192, of which lift to points summing to the target: 192. Profiling copy against the unmodified solver (conflicts, decisions, propagations, GJE calls): 0 mismatches in 118 targets. Conflicts of lin21-fast equal to lin21 on 155 of 176 targets (all kinds).

## Wall time per target, non-decomposable uniform targets (median ms)

Ratios in the first three ratio columns are ratios of medians (the rule's quantity). Paired = median [min, max] of the per-target ratio. A/A = second lin21-fast run / first, same process, the original lin21 run between them.

| l | n | targets | scan | lin21 | lin21-fast | lin21-fast (2nd) | fast / scan | lin21 / scan | lin21 / fast | fast / scan paired | lin21 / fast paired | A/A paired |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 5 | 17 | 16 | 0.0109 | 20.4 | 0.5228 | 0.4745 | 47.9 | 1866 | 39.0 | 48.5 [39.7, 66.5] | 38.5 [26.7, 41.7] | 0.9031 [0.6225, 0.9847] |
| 6 | 20 | 16 | 0.0381 | 68.3 | 1.53 | 1.38 | 40.1 | 1794 | 44.7 | 40.1 [36.6, 49.4] | 41.1 [30.9, 55.3] | 0.9460 [0.6284, 1.25] |
| 7 | 23 | 16 | 0.1080 | 151 | 2.68 | 2.51 | 24.9 | 1394 | 56.1 | 26.0 [23.6, 32.7] | 55.0 [45.3, 83.0] | 0.9286 [0.5127, 1.67] |
| 8 | 26 | 16 | 0.3037 | 453 | 7.54 | 7.25 | 24.8 | 1493 | 60.2 | 24.7 [23.4, 33.4] | 55.9 [36.6, 82.5] | 0.9878 [0.5451, 1.47] |
| 9 | 29 | 16 | 0.7775 | 1061 | 16.0 | 15.8 | 20.6 | 1364 | 66.3 | 21.0 [20.0, 28.2] | 67.3 [42.2, 73.8] | 0.9795 [0.6114, 1.51] |
| 10 | 32 | 16 | 1.86 | 2704 | 44.0 | 38.8 | 23.7 | 1456 | 61.5 | 23.3 [21.6, 34.1] | 62.1 [39.8, 87.7] | 0.8854 [0.5869, 1.42] |
| 11 | 35 | 16 | 4.22 | 6785 | 104 | 96.7 | 24.5 | 1606 | 65.5 | 24.5 [21.5, 32.4] | 66.3 [44.1, 77.4] | 0.9892 [0.5807, 1.28] |
| 12 | 38 | 16 | 9.52 | 15358 | 221 | 222 | 23.2 | 1613 | 69.6 | 22.7 [20.6, 33.5] | 72.2 [47.6, 90.5] | 1.00 [0.6986, 1.38] |
| 13 | 41 | 6 | 22.3 | 39631 | 558 | 529 | 25.0 | 1777 | 71.1 | 23.4 [21.7, 28.0] | 72.0 [64.2, 77.1] | 0.9457 [0.8196, 1.13] |

## Conflicts and Gauss work, non-decomposable uniform targets (medians)

Gauss work in 64-bit words touched by row operations. scan: 2 words per row XOR (column + combination). lin21-fast: variable range + mask word per row XOR; its column-index XORs and checkpoint copies are listed separately. lin21 (original): variable words + combination words per row XOR, counted by the profiling copy on the targets it ran on.

| l | conflicts lin21 | conflicts lin21-fast | equal on | row XORs scan | row XORs fast | row XORs lin21 | words scan | words fast rows | words fast col index | words fast copies | words lin21 | lin21 targets counted | fast rows+cols / scan | lin21 / fast rows+cols |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 5 | 46 | 46 | 16 of 16 | 1124 | 9968 | 84366 | 2249 | 21212 | 4096 | 84460 | 586476 | 16 | 11.3 | 23.2 |
| 6 | 99 | 99 | 16 of 16 | 4054 | 33996 | 258736 | 8109 | 83700 | 19369 | 267658 | 2130918 | 16 | 12.7 | 20.7 |
| 7 | 182 | 182 | 16 of 16 | 9098 | 67645 | 631164 | 18196 | 143514 | 45687 | 665927 | 6176025 | 16 | 10.4 | 32.6 |
| 8 | 382 | 382 | 16 of 16 | 28006 | 232712 | 1750111 | 56012 | 581872 | 171206 | 1842557 | 19721398 | 16 | 13.4 | 26.2 |
| 9 | 774 | 774 | 16 of 16 | 77892 | 541578 | 4166380 | 155784 | 1153806 | 496282 | 4170812 | 48433988 | 16 | 10.6 | 29.4 |
| 10 | 1527 | 1527 | 16 of 16 | 190884 | 1424484 | 11310520 | 381767 | 3610328 | 1228905 | 10287907 | 142214931 | 4 | 12.7 | 29.4 |
| 11 | 3066 | 3066 | 16 of 16 | 468408 | 3076357 | 24569587 | 936815 | 8547140 | 3205405 | 25302830 | 358422944 | 3 | 12.5 | 30.5 |
| 12 | 6150 | 6150 | 16 of 16 | 1155835 | 8131838 | 60595894 | 2311670 | 20483452 | 9106856 | 55095641 | 906697941 | 3 | 12.8 | 30.6 |
| 13 | 12351 | 12351 | 6 of 6 | 2730757 | 16263946 | 133322176 | 5461514 | 41978766 | 20883130 | 132177925 | 2166330451 | 4 | 11.5 | 34.5 |

## Slopes (bits per unit of l, least squares on medians)

- log2(lin21-fast / scan), l >= 9: 0.053; all l: -0.104
- log2(wall), l >= 9: scan 1.204, lin21-fast 1.258, lin21 1.295

## Decision by the pre-registered rule (l = 9..12)

**gap closes** [pcc:unverified]. Largest lin21-fast / scan ratio of medians over l = 9..12: 24.5; threshold 30. Verdicts clean: true. lin21 / lin21-fast ratio of medians: l=9 66.3, l=10 61.5, l=11 65.5, l=12 69.6. A/A band over l = 9..12: per-target second/first in [0.5807, 1.51], per-l medians 0.9795, 0.8854, 0.9892, 1.00.

- l = 9: ratio 20.6 x per-target A/A [min, max] = [12.6, 31.1]; 30 inside: true. Ratio x A/A median = 20.1; largest single-target lin21-fast / scan = 28.2.

- l = 10: ratio 23.7 x per-target A/A [min, max] = [13.9, 33.7]; 30 inside: true. Ratio x A/A median = 21.0; largest single-target lin21-fast / scan = 34.1.

- l = 11: ratio 24.5 x per-target A/A [min, max] = [14.2, 31.4]; 30 inside: true. Ratio x A/A median = 24.3; largest single-target lin21-fast / scan = 32.4.

- l = 12: ratio 23.2 x per-target A/A [min, max] = [16.2, 32.0]; 30 inside: true. Ratio x A/A median = 23.2; largest single-target lin21-fast / scan = 33.5.

Extension cell l = 13 (6 targets): lin21-fast / scan = 25.0, lin21 / lin21-fast = 71.1; inside the 30x threshold: true.

## Time shares by phase (sum over the non-decomposable uniform targets, % of the summed phase timers)

lin21-fast:

| l | clause_prop | unit_rows | gate_rows | analyze | backtrack | pick_branch | matrix_copy |
|---|---|---|---|---|---|---|---|
| 5 | 4.7 | 13.9 | 60.3 | 2.8 | 2.5 | 5.1 | 10.7 |
| 6 | 4.1 | 10.8 | 68.2 | 2.7 | 2.2 | 4.3 | 7.6 |
| 7 | 5.5 | 9.9 | 66.2 | 2.7 | 2.1 | 4.6 | 9.0 |
| 8 | 4.6 | 7.0 | 72.5 | 2.4 | 1.8 | 3.8 | 7.8 |
| 9 | 5.7 | 6.3 | 73.0 | 2.4 | 1.8 | 4.1 | 6.7 |
| 10 | 6.4 | 5.8 | 74.9 | 2.2 | 1.4 | 3.5 | 5.8 |
| 11 | 8.8 | 5.1 | 73.7 | 2.0 | 1.3 | 3.3 | 5.8 |
| 12 | 13.2 | 4.6 | 70.4 | 1.8 | 1.2 | 3.1 | 5.7 |
| 13 | 23.2 | 4.3 | 61.1 | 1.7 | 1.0 | 2.8 | 5.9 |

lin21-prof:

| l | clause_prop | gate_inject | gje_build | gje_eliminate | gje_scan_explain | analyze | backtrack | pick_branch | learn_add |
|---|---|---|---|---|---|---|---|---|---|
| 5 | 0.3 | 4.2 | 16.9 | 43.1 | 30.6 | 3.8 | 0.9 | 0.2 | 0.1 |
| 6 | 0.3 | 3.9 | 14.7 | 42.7 | 34.1 | 3.4 | 0.6 | 0.1 | 0.1 |
| 7 | 0.3 | 3.6 | 13.5 | 41.7 | 36.7 | 3.1 | 0.9 | 0.1 | 0.1 |
| 8 | 0.4 | 3.3 | 12.1 | 41.1 | 39.5 | 2.7 | 0.8 | 0.1 | 0.1 |
| 9 | 0.5 | 3.0 | 10.5 | 41.1 | 41.1 | 2.7 | 0.9 | 0.1 | 0.2 |
| 10 | 0.8 | 2.8 | 9.2 | 41.6 | 41.7 | 2.6 | 1.1 | 0.1 | 0.2 |
| 11 | 1.4 | 2.6 | 7.9 | 40.5 | 43.9 | 2.5 | 0.8 | 0.1 | 0.3 |
| 12 | 2.2 | 2.6 | 7.1 | 41.5 | 43.3 | 2.4 | 0.5 | 0.1 | 0.3 |
| 13 | 3.2 | 2.4 | 6.0 | 38.9 | 46.1 | 2.5 | 0.6 | 0.0 | 0.3 |

## Decomposable targets (planted and uniform-dec): verdicts and conflicts

| l | targets | all arms SAT with lifted models | conflicts lin21 (sum) | conflicts lin21-fast (sum) | wall lin21 ms (sum) | wall lin21-fast ms (sum) | wall scan ms (sum, full scan) |
|---|---|---|---|---|---|---|---|
| 5 | 4 | 4 | 59 | 59 | 42.2 | 1.40 | 0.0518 |
| 6 | 5 | 5 | 92 | 88 | 82.8 | 4.38 | 0.2043 |
| 7 | 4 | 4 | 133 | 135 | 132 | 2.81 | 0.4506 |
| 8 | 4 | 4 | 357 | 357 | 429 | 7.87 | 1.37 |
| 9 | 5 | 5 | 1399 | 1398 | 1977 | 32.4 | 4.17 |
| 10 | 5 | 5 | 3806 | 3803 | 7638 | 112 | 9.43 |
| 11 | 5 | 5 | 2359 | 2361 | 5038 | 98.2 | 23.6 |
| 12 | 8 | 8 | 15832 | 15831 | 46764 | 593 | 81.4 |
| 13 | 2 | 2 | 13948 | 13947 | 43161 | 526 | 43.5 |
