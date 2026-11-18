# Round C result: direct cubic adjugate inversion

Status: deterministic local correctness and exact-count controls passed on the
source-bound candidate. The checkpoint gate refused the experiment route, so
no profile, throughput result, complete frontier grid, or leaderboard change
is claimed. These producer controls are not independently admitted
`[pcc:unverified]`.

## Outcome

The direct adjugate returned the same inverse as both the retained Round B
Frobenius kernel and the frozen N4 field implementation for 200 deterministic
nonzero inputs in each constant path. With the production small constant, each
inverse used 12 rather than 16 counted `fq_mul`. In the valid fallback field
`q=223, c=18`, zero short-circuits made the observed absolute ranges 13--15 and
17--19, while every one of the 200 paired inputs retained the exact delta of
four. Both paths used exactly one `fq_inv` per inverse. The direct specialization
uses no general `f3_mul`; the reference uses one.

The existing standalone controls also matched pointwise and batched field
inversion, curve addition, and planted-log recovery at widths 1, 8, and 32.

On the fixed `q=223`, seed 1, target 0, `ic-repeat`, `strong-b8` control, the
baseline and candidate recovered `k=1765657` after the same walk: 1,487 steps,
76 distinguished points, zero restarts, 53 look-ahead retries, two cycles, and
10 cycle additions. Every group and native count was identical. The expected
inversion-local specializations were exact in every phase:

| phase | `f3_inv` | baseline `fq_mul` | candidate `fq_mul` | `fq_mul` delta | `f3_mul` delta | `fq_add` delta |
|:--|--:|--:|--:|--:|--:|--:|
| setup | 2,348 | 94,523 | 85,131 | -9,392 | -2,348 | -28,176 |
| walk | 241 | 53,825 | 52,861 | -964 | -241 | -2,892 |
| verify | 30 | 1,210 | 1,090 | -120 | -30 | -360 |
| total | 2,619 | 149,558 | 139,082 | -10,476 | -2,619 | -31,428 |

Thus `fq_mul` changed by exactly `-4*f3_inv`, `f3_mul` by
`-1*f3_inv`, and `fq_add` by `-12*f3_inv`; `fq_inv`, `f3_inv`, `f3_sq`,
batch inversions, group additions, and all walk/native fields were unchanged.
This is a deterministic code/count result. The retained development timing and
calibration fields are ignored and support no performance claim.

## Provenance and retained controls

- Baseline revision: `ad2a014c0022cf000845014499a9649b6c2cef2d`.
- Baseline `strong_rho.rs` SHA-256:
  `0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`.
- Candidate revision: `a973d173465aa6983e97a393664bf2c0548903b2`.
- Candidate `strong_rho.rs` SHA-256:
  `57e91239cef0d31ba0dce337ac08d5d247d0a141f342d5bdca6e78b633cc919b`.
- Shared `main.rs` SHA-256:
  `b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8`.
- Baseline binary SHA-256:
  `636df5139d45efc3b2d9a284a49cdd24540befe05ef75bc19943b5db36539a89`.
- Candidate binary SHA-256:
  `12015cefbe899ea8bbd78269d2e31a98c6d56b55c6478307e59bb27854b3d511`.
- Compiler: rustc 1.100.0, LLVM 23.1.5, `x86_64-unknown-linux-gnu`.

The exact builds were:

```sh
GHS_SOURCE_REVISION=ad2a014c0022cf000845014499a9649b6c2cef2d \
  rustc --edition 2021 -O research/quartic_ghs_strong_rho_20261114/main.rs \
  -o /dev/shm/ghs-leads/ghs-inv-baseline
GHS_SOURCE_REVISION=a973d173465aa6983e97a393664bf2c0548903b2 \
  rustc --edition 2021 -O research/quartic_ghs_strong_rho_20261114/main.rs \
  -o /dev/shm/ghs-leads/ghs-inv-a973d17346
/dev/shm/ghs-leads/ghs-inv-a973d17346 selftest
jq -s -f research/quartic_ghs_strong_rho_20261114/round-c-inv/check_q223.jq \
  research/quartic_ghs_strong_rho_20261114/round-c-inv/q223-baseline.jsonl \
  research/quartic_ghs_strong_rho_20261114/round-c-inv/q223-candidate.jsonl
```

The self-test transcript, raw paired JSONL, machine count/identity result,
environment receipt, checkpoint refusal, and bounded whole-tree test status
are retained beside this report.

## Blocker and integration decision

`scripts/checkpoint.sh gate` exited 2 because `.resume/RESUME.md` was absent.
Accordingly no profiler, unprofiled timing screen, q=4051 diagnostic, paid
grid, or frontier measurement was attempted. The repository-wide release test
was also bounded before reaching the crypto crate; its non-verdict is retained
in `full-test-status.txt` rather than represented as a pass.

The candidate is ready to integrate as a deterministic code/count
optimization: all frozen algebra, fallback, planted-log, walk-identity, and
exact-count requirements passed. It is not evidence of a measured throughput
gain or a newly established IC/rho boundary. Those conclusions still require
the main protocol's checkpoint, standard isolation, complete same-target grid,
validation, PCC, and publication gates.
