# N=51 honest-prior GHS genus distribution (2026-10-24)

## Frozen question and sample

This is phase 1 of [the seed-grind protocol](../../RESEARCH_SEED_GRIND_TRAPDOOR_20261002.md#11-n51-honest-prior-phase-frozen-before-the-run-2026-10-24). It measures the minimum genus of applicable GHS rows over proper intermediate subfields, with no grind selection. The protocol fixed 16,384 indices, the 32-byte seed `2026102420261024202610242026102420261024202610242026102420261024`, and thresholds 0, 1, 2, 3, 4, 8 before the run. The field is GF(2^51) with polynomial x^51+x^6+x^3+x+1. The public generator `ghs-honest-prior-v1-blake3-xof` derives a and nonzero b for each index; `audit_curve` retains every l=1,3,17 row. The minimum excludes l=1 and requires `descends=true`.

## Observation

| Minimum applicable genus | Count | Share |
| --- | ---: | ---: |
| 4, through l=17 | 16,384 | 1 |
| Other or no applicable row | 0 | 0 |

The thresholds g≤0,1,2,3 each had 0/16,384 hits. Modeling the indexed BLAKE3 outputs as independent uniform draws, the exact one-sided 95% binomial upper bound on each corresponding event probability is `1−0.05^(1/16384) = 0.0001828282722735608`. This is a model-conditional upper bound, not a measured positive frequency or a finite estimate of seed-grinding work. The thresholds g≤4 and g≤8 each had 16,384/16,384 hits and empirical probability 1 (empirical grind bits 0). The [figure](genus-histogram.svg) displays the sample counts.

This distribution is a structural diagnostic of the sampled curve family. It does not measure hidden-route secrecy, detector accuracy, destination solving, cold end-to-end cost, or a matched rho reference. The sample result does not assert the distribution for other degrees, construction families, or all degree-51 curves. A sampled g=4 outcome alone does not establish a useful trapdoor.

The canonical `docs/index-calculus-scoreboard.html` gets this panel. The existing `docs/ic/progress-timeline.json` and `docs/bounds/frontier.json` were checked and left unchanged because this phase has no priced pipeline or method bound to add.

## Reproduction and integrity

Source: `src/cryptanalysis/seed_grind_trapdoor.rs`, `examples/seed_grind_honest_prior.rs`, base revision `0200e2b2af7c608b100930be9397124441c2710b` plus this PR. Run on Darwin arm64, kernel 25.6.0, without CPU isolation. No timing or speed comparison is reported.

```sh
cargo run --release --example seed_grind_honest_prior -- 16384 2026102420261024202610242026102420261024202610242026102420261024 research/notes/index-calculus/seed_grind_trapdoor_20261024_run
cargo run --release --example seed_grind_honest_prior -- --verify research/notes/index-calculus/seed_grind_trapdoor_20261024_run
```

The first command creates a *new* output directory and refuses to overwrite an existing one; the displayed path was absent when run. The verification command rederived all 16,384 rows and aggregate values and completed successfully. [Raw observations](../../seed_grind_trapdoor_20261024_run/observations.jsonl): 4,970,702 bytes; BLAKE3 `034878466ddb0598ded498d54fb252599f0366d4abd63ae8993a52932930f00c`, also recorded in [summary.json](../../seed_grind_trapdoor_20261024_run/summary.json). The records contain each index and complete audit, including excluded l=1 rows; the curve coefficients are reproducibly derived from the frozen seed and index. The raw digest covers observations, not this report or PDF. There were no recorded execution failures. The replay is in the same implementation and host; an independent implementation has not yet audited the results.

## Next discrimination

Freeze a generator freedom model and detector rule, then compare the observed genotype frequency with matched controls under a prospective grind and report missed detections and false positives. Keep any later descended DLP and same-instance rho comparison as separately accounted work.
