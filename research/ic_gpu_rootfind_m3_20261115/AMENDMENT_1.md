# Amendment 1 (2026-11-15 03:12Z, before any rf3 record existed)

The frozen `specs/rf3.json` asked for `prime_search` curves of 30, 32, 34 and 36 bits. On the pod,
`ecbench run` refused it before running anything: "prime_search takes 8 ≤ bits ≤ 32, not 34"
(`raw/out_rm3/run-rf3.log` once pulled). The smoke and rf2d sessions of the same job ran as frozen.

Change: `specs/rf3-amend1.json` is `specs/rf3.json` with the curves 26, 28, 30 and 32 bits (same seed),
nothing else changed (arms, targets, rounds, measurement). It runs on the same pod with the same
binaries (`job_rf3.sh`), after this amendment is pushed. Choosing the smaller sizes was made without
seeing any m = 3 result.
