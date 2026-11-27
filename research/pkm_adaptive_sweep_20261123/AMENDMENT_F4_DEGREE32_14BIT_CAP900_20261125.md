# Amendment — 14-bit single-curve panel, 900-second call cap — 2026-11-25

The first 14-bit run (`degree32_14bit_single_curve_20261301.jsonl`) censored both
interval arms at the frozen 240-second cap (exit 124, no JSON), while both
candidate arms completed (97.8 s and 83.7 s wall). The size gate was 15/15 and
both candidates verified.

Because the baseline censors, the 240-second panel cannot produce a paired
ratio. This amendment keeps the binary, factor bases, oracles, 16 repeats,
1,000 target trials, five-second solver budget, one rho run and mirrored order
unchanged, and raises only the whole-call cap to **900 seconds**. Seed
`20261301` is rerun under the new cap; seeds `20261302` and `20261303` are run
only if the first extended panel completes both arms and verifies all 16
scalars.

A completed extended panel is still a one-curve, three-target-replicate
14-bit component test. A censored baseline is recorded as a censoring signal,
not as a measured ratio.
