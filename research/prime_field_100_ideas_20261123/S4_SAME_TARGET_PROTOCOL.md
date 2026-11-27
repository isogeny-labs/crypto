# Frozen same-target S4 construction and bounded pipeline comparison

Frozen 2026-11-24 (America/Los_Angeles), draft PR #5165. This extends [the previous matched S4 report](S4_MATCHED_REPORT.md), where `z=0` and `z=320` differed in both coefficient support and relation density. Here both arms use **the same target point** and exactly the same candidate and signed relation set. Semaev 2004 Theorem 1 defines `S3` and `S4=Res_T(S3(X,Y,T),S3(T,W,Z))`; the prior exact run checked S4 zeros against signed point sums on this curve. `[pcc:unverified]`

Fixed public input: `E/F_1009:y²=x³+x+14`, prime order 1013, target `R=(0,425)` as GP's deterministic square root of 14, first 16 *nonzero* liftable abscissae ascending with GP square-root choices, all 4096 ordered triples, all eight signs per triple. The source asserts these quantities and the earlier frozen result (88/125 support, 24 zero triples, 24 signed rows, rank 4 over `F_1013`).

Two exact algebraic arms: **generic4_to_z0** constructs the four-variable S4 symbolic resultant, then substitutes `Z=0` once; **direct_z0** constructs the three-variable S4 resultant with `Z=0` already in the second S3 factor. Each arm counts all 125 coefficient slots, pair-specializes `X,Y`, evaluates the same 4096 `W` values, independently enumerates all signed point sums and exact witness rows, verifies every S4 zero iff a signed target point sum exists, and computes the 17-column modular rank. Require ordered signed rows to match bit for bit across arms. Preserve the full generic construction and substitution in its setup charge; neither arm gets free precomputation. Four fixed passes in `generic, direct, direct, generic` order reduce simple warmup/order bias, with no random inputs or post hoc changes. `[pcc:unverified]`

PARI/GP `getabstime` and `getwalltime` report CPU and wall milliseconds separately for resultant construction, setup including support scan, evaluation/verification/rank, and total bounded pipeline. This is a **component-level full-path** charge, including its symbolic construction and exact reference validator for each arm. It does not include complete relation collection, linear algebra across a full factor base, descent, or same-target rho; no end-to-end solver claim follows. The generic arm is allowed to pre-specialize `Z=0` after constructing the four-variable object, which is the natural optimized generic comparator. The direct arm retains the same evaluation and verification method. Timing is a four-pass local diagnostic, not a population benchmark.

The checkpoint gate remains failed (`.resume/RESUME.md` absent; prior S3 `InvalidAccessKeyId`). The user's explicit local bypass and renewed public EC run instruction admit this bounded local experiment, not a successful checkpoint. No cloud spend or external target. Stop on GP stderr, missing sentinel, shell failure, degree/support mismatch, S4/group disagreement, row disagreement, or unexpected counts. Preserve raw stdout/stderr/exit even if GP emits a later sentinel after an error; do not retry silently. Source SHA, GP version, exact one-shot command, and results are recorded below before and after execution respectively.

Frozen [GP source](S4_SAME_TARGET_GP.gp) SHA-256: `653064dd4f314052774252c268bf76624f263c8e498bed021db13c6207c6f0bd`. PARI/GP 2.17.3, arm64 macOS. One invocation from repository root:

```sh
gp -q < research/prime_field_100_ideas_20261123/S4_SAME_TARGET_GP.gp > research/prime_field_100_ideas_20261123/S4_SAME_TARGET.stdout.txt 2> research/prime_field_100_ideas_20261123/S4_SAME_TARGET.stderr.txt
```

Record the shell status immediately in `S4_SAME_TARGET.exit.txt`. The frozen run has not started at this protocol point.

## Observed one-shot local run

The frozen command ran once with GP 2.17.3. Shell exit was `0`, stderr was empty, and all four pass lines plus the completion sentinel appeared. Every arm reported support 88/125, 24 S4-zero triples, 24 exact signed witness rows, rank 4/17 over `F_1013`, and 65,536 reference group additions. The script compared all ordered signed witness rows to the first arm and stopped on a mismatch; all four comparisons passed. The same target, factor base, and candidate count were held fixed. This is a local producer observation `[pcc:unverified]`.

| Pass | Arm | Resultant construction CPU / wall ms | Setup including support CPU / wall ms | Evaluation, exact group validation, rank CPU / wall ms | Total CPU / wall ms |
|---:|---|---:|---:|---:|---:|
| 1 | Generic four-variable then `Z=0` | 1 / 1 | 1 / 2 | 73 / 76 | 74 / 78 |
| 2 | Direct `Z=0` | 0 / 0 | 0 / 0 | 74 / 76 | 74 / 76 |
| 3 | Direct `Z=0` | 0 / 0 | 0 / 0 | 74 / 76 | 74 / 76 |
| 4 | Generic four-variable then `Z=0` | 1 / 1 | 1 / 1 | 83 / 93 | 84 / 94 |

The 0 ms direct construction entries are below GP's integer-millisecond resolution, not zero work. Generic construction cost about 1 ms in these passes; the 73–83 ms evaluation/verification/rank stage dominates the bounded pipeline. With only two passes per arm, order effects and measurement noise prevent a reliable runtime speed claim. The exact row/rank equality removes the earlier `z=0` versus `z=320` relation-density confounder. This does not isolate generic/specialized polynomial-evaluation algorithms because both arms use the same post-specialization grid evaluator. It tests the cost of constructing a four-variable S4 and specializing it once versus constructing S4 directly at the fixed target, including identical downstream validation. A larger same-target timing panel would be needed to estimate a reproducible construction gain; a complete index-calculus/rho comparison requires further components.

| Preserved artifact | SHA-256 |
|---|---|
| [Frozen source](S4_SAME_TARGET_GP.gp) | `653064dd4f314052774252c268bf76624f263c8e498bed021db13c6207c6f0bd` |
| [Stdout](S4_SAME_TARGET.stdout.txt) | `4191b1310074d0745896e5827a26be532ca50e6d9ef9520aa8151b47a986bfcd` |
| [Stderr](S4_SAME_TARGET.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Exit record](S4_SAME_TARGET.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
