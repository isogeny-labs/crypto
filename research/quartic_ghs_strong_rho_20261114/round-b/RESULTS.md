# Round B partial result: correctness and counted work

Status: deterministic local correctness/count control passed; profile,
isolated timing, and complete frontier grid not run. These producer results
are not independently admitted `[pcc:unverified]`.

## Outcome

The five-product cubic square returned the same field elements as the generic
multiply for 200 deterministic field inputs. The existing standalone
self-test also recovered every planted logarithm at widths 1, 8, and 32.

On the fixed `q=223`, seed 1, target 0, `strong-b8` control, Round A and Round
B recovered the same scalar after the same walk. Steps (1,487), distinguished
points (76), restarts (0), look-ahead retries (53), cycles (2), cycle additions
(10), every group count, and every native count were identical. The priced
base-field multiplication delta was exactly one per cubic-field square:

| phase | `f3_sq` | Round A `fq_mul` | Round B `fq_mul` | delta |
|:--|--:|--:|--:|--:|
| setup | 3,897 | 98,420 | 94,523 | -3,897 |
| walk | 1,553 | 55,378 | 53,825 | -1,553 |
| verify | 50 | 1,260 | 1,210 | -50 |
| total | 5,500 | 155,058 | 149,558 | -5,500 |

The candidate also executed five fewer counted base-field additions per
square in this implementation. Additions are retained in the raw record but
are not priced in the frozen Round A `M-eq` unit. This is a code/count result
for one correctness cell, not a measured throughput result or an end-to-end
IC/rho boundary `[pcc:unverified]`.

## Provenance

- Baseline revision: `9ec88b7637aac4ce816739155f12f518b393da37`.
- Frozen Round A source SHA-256: `73abd342613dee852add1e3459c94bc7f949801c94eb875a309cdfc4f6b0f5de`.
- Round B candidate source SHA-256: `0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`.
- Shared `main.rs` SHA-256: `b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8`.
- Exact-main baseline binary SHA-256: `e4ead95e208e087d88d25f6d329021e10e356957a3c0e6a712cf291218539bb4`.
- Candidate binary SHA-256: `82403c47442e32d28ff5860e004c2ca267db3e2a8007d10bdc610806cd90f248`.
- Compiler: `rustc 1.100.0 (0ef1bc386 2026-11-09)`, LLVM 23.1.5,
  `x86_64-unknown-linux-gnu`.
- Host: Linux 6.18.57, Intel Xeon Platinum 8370C, container-visible 5 CPUs.
- Timestamp: `2026-11-18T06:18:20Z`.

The baseline was materialized from the exact revision with `git archive` and
built with `rustc --edition 2021 -O`. Both control invocations were
`GHS_CALIB_TRIPLES=1 <binary> run 223 1 0 1 ic-repeat strong-b8`. Raw JSONL
is retained beside this file. Calibration ratios and wall times differ because
the calls were neither isolated nor interleaved; neither is used for a
comparison.

## Blocker and next discriminating run

`scripts/checkpoint.sh push` and `gate` both exited 1: this runtime has no
`ecc-challenges-new` AWS profile, upload token, or checkpoint token file. Per
the repository gate, no profile, `q=4051` diagnostic, A/A screen, ABAB panel,
or paid grid was run. The exact refusal is retained in `checkpoint-gate.txt`.

After checkpoint transport is restored, profile both source-bound binaries,
then run the frozen A/A and five-pair ABAB diagnostic. A complete frontier
statement still requires the Round A grid's isolation, same-target,
calibration, rho-step, validation, PCC, and publication gates. Until then the
historical restricted GHS-weak IC exception remains separate from this pending
matched-rho re-establishment.

## Test status

The edited standalone source compiled and its source-bound self-test passed.
The repository-required `cargo test --release --lib` did not compile on the
unchanged base. Its first attempt found a sparse-checkout omission; after the
tracked path dependencies were materialized, the second attempt ended with 39
errors in untouched modules (duplicate definitions, missing included fixtures,
and mismatched F4 diagnostic APIs). `git diff origin/main -- src Cargo.toml
Cargo.lock` was empty. The retained summary is `full-test-status.txt`; these
base failures are not attributed to Round B.
