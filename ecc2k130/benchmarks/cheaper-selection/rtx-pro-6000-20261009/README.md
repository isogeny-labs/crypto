# TABLE_PHASE_POPC on one RTX PRO 6000 (2026-10-09)

`benchmarks/cheaper-selection/gpujob.sh`, unchanged, on Modal: one RTX PRO
6000 Blackwell Server Edition, driver 580.95.05, nvcc 13.3.73, `sm_120`.
Reference `make gpu-rtx-pro6000-20b TABLE_PHASE_POPC=0`, candidate the same
with `TABLE_PHASE_POPC=1`; both 128 registers, 400-byte stack frame, no spills.

Source: `845fd3230` plus the build repair for the 2026-10-08 union-resolve
merges (duplicated `HDRS`/`DEFS` lines in the Makefile, two unterminated
`#if` blocks in `packedkernels.cuh`, a dangling launch in `packedengine.cuh`).
Main carries its own repair of the same damage (`85748c99f`, `f1fbe4143`).
A same-container build of `4320be196`, the last commit before those merges,
produced SASS with the same instruction, `CLMAD`, `STL`, `LDL` and `CALL`
counts and ran at the same rate (5.45-5.57 against 5.53-5.57 B/s), so the
repair is runtime-neutral. Repair patch SHA-256
`ba7b9c6af3905a0d2ce088dab41fc75c875dac3ba804581642e377e355499e8e`; full
results archive (Modal volume `ecc2k130-jobs`, token
`34f794e31bef48cb8c4400647743787e`) SHA-256
`adfae8bd8fb9f6307a04975feebc4989ac6c45d4c6e36d5d5a05184526e9887c`.

| | reference | popc | ratio |
|---|---:|---:|---:|
| B complete updates/s, five alternating reps | 5.594, 5.594, 5.594, 5.597, 5.598 | 5.509, 5.503, 5.505, 5.507, 5.505 | |
| median | **5.594** | **5.505** | **0.9840** |
| paired ratios | | | 0.9849, 0.9837, 0.9841, 0.9839, 0.9834 |
| re-walk | 300/300, 0 dropped | 300/300, 0 dropped | |
| DP corpus, forced common walk count | 1,480,278 records | 1,480,278 records, identical sorted SHA-256 `2fc6f84d...` | |
| phase profile, forward share of a warp-step | 75.7% | 76.0% | |

Every pair favours the reference. The candidate is abandoned.

The absolute bars in CHEAPER-SELECTION.md section 2 (success at 20.50 B/s,
abandon below 20.20) were frozen against the 20.078 B/s pre-v3 table walk.
Cycle escape v3 (CYCLE-ESCAPE-V3.md) replaced that walk, and the same preset
now runs at 5.59 B/s, so those bars cannot be met by either arm. The relative
criterion they encoded, about +2.1% with every pair favouring the candidate,
is the operative one here, and the candidate is -1.6% with every pair against.
