# Confirmation: F4 round 4 on jv_cover (frozen before data, 2026-11-21 04:37 UTC)

**Question.** Does round 4 make `jv_cover --exp dlp` faster, end to end, with identical relations and a correct logarithm?

1. **The fixed Hilbert+trace mode.** Old: `--stop hilbert:16` on the pre-round-4 binary, the best mode available before this round. New: `--stop hilbert:16 --trace` on the round-4 binary.
2. **The default full-F4 mode.** The same command on both binaries.

Round 4 changes:
- trace through the Hilbert guard;
- the recording run is charged;
- u32 accumulators;
- byte pack/unpack;
- the u64 divisor scan;
- frozen replay steps widened by support union.

**Arms.** All arms run `jv_cover --exp dlp --sizes 53 --seeds 1 --rho-runs 0` with RAYON_NUM_THREADS=4.
- `H0`: pre-round-4 binary (`94ba28d783919d60…`, branch state at round 3), `--stop hilbert:16`.
- `H1`: round-4 binary (`9e87d5f0afc7cb4a…`), `--stop hilbert:16 --trace`.
- `H1a`: a second copy of `H1` in each round, the A/A control.
- `F0`: pre-round-4 binary, default (full F4).
- `F1`: round-4 binary, default (full F4).

**Schedule.** 10 rounds. Each round runs the five arms once, in an order rotated by the round. No build or other job runs at the same time. Wall time comes from `wall_ms`; CPU time from the shell's `time`.

**Identity check.** Every run must find 31 relations from 19,020 residuals and a correct logarithm. Counted oracle multiplications must equal:
- 9,623,663,419 for H0;
- 7,303,025,579 for H1 (deterministic, measured before this protocol);
- 17,873,891,895 for F0 and F1.

**Basis of the predictions.** These are deterministic callgrind F4 counts on `--exp ccov` (104 tests, p = 53, one thread), not timing:
- Hilbert: 1.096 G on the old binary, 0.774 G for Hilbert+trace with frozen steps on the new one, about 1.42× on F4.
- Full F4: 1.138 G → 0.998 G, 1.14× on F4.

**Statistic.** Per-round paired ratios H0/H1, F0/F1 and H1/H1a of wall time and CPU time. A comparison counts when at least 9 of 10 rounds favour it (sign test, p = 0.011) and its median exceeds the median absolute A/A deviation.

**Predictions.**
- Primary: H0/H1 wall median in [1.15, 1.50], at least 9 of 10 rounds above 1.
- F0/F1 wall median in [1.03, 1.15].
- H1/H1a within ±3 % median.
- Counted S: H0/H1 = 392.28/298.09 = 1.316, deterministic. This is a counted-cost reduction from replaying only the recorded useful rows, with the recording runs charged.

**Conditions.** One shared 4-vCPU VM, isolation L0, no PMU. All results are `[pcc:unverified]`.
