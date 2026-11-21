# Confirmation: f4_fp per-term allocation and interning changes on jv_cover (frozen before data, 2026-11-21 01:00 UTC)

**Question.** Do the f4_fp changes of this PR reduce the end-to-end time of jv_cover with identical work, when both arms already run jemalloc? The changes are packed monomials in the final linear interreduction, lazily unpacked matrix columns and reducer multipliers, and an open-addressing interning table.

**Arms.** All arms run `jv_cover --exp dlp --sizes 53 --seeds 1 --rho-runs 0` with RAYON_NUM_THREADS=4 and no F4 toggles set.

- `B0`: baseline. main `ca6844d480` with jemalloc as a default feature (the first change of this PR), sha256 `951f18a64212e8cf…`.
- `B1`: this branch, all f4_fp changes (`94ba28d783919d60…`).
- `B1a`: a second copy of `B1` in each round, the A/A control.

Binary hashes are in `data/binaries.sha256`.

**Basis of the prediction.** These are deterministic callgrind counts, not timing (`data/callgrind/`), on `jv_cover --exp ccov` with p = 53 and 104 Nagao tests on one thread:

- F4 instructions: 1.317 G for B0 and 1.137 G for B1 (−13.6 %).
- Whole program: 3.506 G for B0 and 3.331 G for B1 (−5.0 %).

The output JSON is identical apart from timing fields.

**Schedule.** 12 rounds. Each round runs the three arms once, in an order rotated by the round. Wall time comes from the report's `wall_ms`. CPU time (user + system) comes from the shell's `time`.

**Identity check.** Every run must report the same residuals, relations, counted oracle operations and a correct logarithm. Any difference fails the run.

**Statistic.** The per-round paired wall and CPU ratios `B0/B1` and `B1/B1a`. B1 counts as faster when at least 10 of 12 rounds favour it (one-sided sign test, p = 0.019) and its median ratio exceeds the median absolute A/A deviation from 1.

**Predictions.**
- Primary: `B0/B1` wall median in [1.02, 1.10], with at least 10 of 12 rounds above 1.
- `B0/B1` CPU median in [1.02, 1.10].
- `B1/B1a` median within ±2 %.

An effect this small may not clear the noise: the A/A deviation reached 8 % in a single round on 2026-11-20. A null is a valid outcome and will be reported as one.

**Conditions.** No build or other benchmark runs at the same time. One shared 4-vCPU cloud VM, isolation L0, no PMU. All results are `[pcc:unverified]`.
