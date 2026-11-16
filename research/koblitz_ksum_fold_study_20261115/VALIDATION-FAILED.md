# Validation failed: VAL-20261115-f7df34 [pcc:unverified]

The registered comparison (kf3-c1g-sr vs lazy-c4 on K1 n23, 0.78–0.83×) replays, recomputes and reproduces on fresh seeds, but it does not
support an improvement claim:

- **D1 (major), wrong incumbent.** `research/ecbench_generator_orbit_20261022` (validated) already has an orbit-layout table with
  `generator_orbit=1` that is 1.26–3.20× cheaper than lazy. Against it (`gen-layout-c4`, test pre-declared before fresh sessions F4–F6)
  the candidate is 1.051 [0.980, 1.126] / 1.089 / 1.175× at w = 0 / 0.1 / 1. The lazy oracle on the candidate's generator-orbit base
  matches the candidate (0.977–1.006, every interval contains 1): the gain is the base, not the k-sum oracle.
- **D2 (major), matched rho.** `rho.signed_frobenius_v5` costs about 0.25–0.30× `rho.signed_frobenius_strong` in the same binary; against
  v5 the candidate is 3.47 / 4.21 / 7.41×.
- Minor: deterministic A/A, dirty tree during the run, raw `.gz` above the artifact threshold, no memory column.

No leaderboard or `benchmarks/README.md` row. Report: [validator/](validator/).
