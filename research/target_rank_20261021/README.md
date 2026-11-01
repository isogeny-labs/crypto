# Signed target-coefficient rank collector: full 1,260-target result

The frozen [protocol](PROTOCOL.md) and [method](../../src/cryptanalysis/three_leads_followup.rs)
were committed as `2bb1880c0` before any cohort run. The seven
[specifications](specs/) retain every original exact-order workload,
measurement seed and trial cap: 21 curves, 60 targets each, and source-order
sizes 16–40 bits. The new arm uses the same projective numerator chart and
exceptional Cantor fallback as the current best index-calculus baseline. It
charges a signed-graph preview of the *actual target coefficient* for each
new valid relation, then chooses the continuation with the higher rank-effect
score. Both prospective additions and both valid relations are retained.

**Result:** all **1,260/1,260** answers were source-verified, and all
**1,260/1,260** independent `ecbench --replay-all` checks reproduced the
records with zero audit problems. At `M + 0.5S + 10I` plus 0.1 source-addition
equivalents per declared native logical operation, complete cold one-target
costs summed over the cohort are:

| Arm | Source-addition equivalents | Rank / comparator |
| --- | ---: | ---: |
| New target-rank projective collector | 3,188,565,286 | 1.000000 |
| Existing projective two-choice collector | 3,137,323,980 | **1.016333** |
| Existing affine two-choice collector | 4,881,369,517 | 0.653211 |
| Existing affine incumbent v2 | 4,877,191,974 | 0.653771 |
| Source negation rho | 1,088,802,370 | **2.928507** |

The new policy is a **1.633% loss** against the current projective IC
baseline, despite beating the older affine collectors. It loses to source
rho at the representative price. No wall-speed claim follows from these L0
operation counts.

| Source-order bits | Rank / projective | Rank / source rho |
| ---: | ---: | ---: |
| 16 | 1.017699 | 3.299515 |
| 20 | 0.997728 | 2.846113 |
| 24 | 1.016907 | 2.923633 |
| 28 | 1.019481 | 2.873589 |
| 32 | 1.007369 | 2.754238 |
| 36 | 0.992683 | 2.957897 |
| 40 | 1.022771 | 2.932806 |

On the full **12 arithmetic × 3 logical** price grid, rank/projective spans
0.999539–1.030179, winning 8 cells and losing 28; all eight wins occur at
zero native logical-operation charge and are at most 0.0462%. Rank/rho spans
1.387016–15.114907, with **0 wins and 36 losses**. At the representative
price, nine of 21 individual curves favor rank/projective and 12 favor the
projective baseline (range 0.941988–1.076664). The
[summary](summary.json) keeps every pooled price cell and all per-size and
per-curve representative values.

The rank arm processed **221,300,578** prospective divisors versus
**221,270,948** for projective two-choice, an increase of 29,630. It made
110,655,314 charged rank previews. Of these, 110,638,044 (**99.984393%**)
were ordinary target-bearing component merges; 15,979 were target-bearing
anchors, 1,256 were target equations, and 35 were target-free. This exact
classification gives little extra discrimination when both candidates
produce ordinary relations. Graph operations increased from 260,349,717 to
518,558,056 and scalar additions from 1,799,557,710 to 2,463,716,728.
The observed extra cold cost is 51,241,306 source-addition equivalents at
the representative price. These are observations and an interpretation of
the measured score distribution, not a claim that all prospective choices
tied.

Raw session files are in the [session archive](sessions.tar.zst) and exact
replay receipts in [`replay/`](replay/). Extract with
`tar --zstd -xf research/target_rank_20261021/sessions.tar.zst -C research/target_rank_20261021`
before running the [analysis code](analyze.py). Each session records its
spec, binary, host and record digests. The analysis checks every target, seed,
answer and replay receipt against the frozen projective, affine and rho
records before pricing. The [run script](run_cohort.sh) reproduces the
build, all seven sessions, replays and analysis. `SHA256SUMS` binds the
committed evidence. The full `cargo test --release --lib -q` suite passed:
**4,420 passed, 118 ignored, 0 failed**, with its output in
[`cargo-test.log`](cargo-test.log). Direct `rustfmt --check` on the four
touched Rust files passed; workspace-wide `cargo fmt --all --check` could
not traverse sparse-checkout example modules absent from this checkout.

This single policy has a verified negative end-to-end result. A different
collector, a complete projective law, or a combined method remains a new
experiment and cannot inherit this verdict. The genus-3 Smith-negative
holdout and the other paths in the [next-lead portfolio](../next_leads_20261020/README.md)
remain open.
