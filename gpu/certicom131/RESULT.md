# Native Certicom-131 arithmetic result

Date: 2026-10-18 (America/Los_Angeles)

## Scope

This result covers arithmetic correctness for the exact ECCp-131 and ECC2-131
published parameters.  It does not cover a persistent walk, distinguished-point
storage, collision replay, challenge search, or throughput.

The initial frozen source candidate is commit
`45f38b33bc93dafd8ffe5c49d305434c252c95f9`; the final PR head also replaces
namespace-stored constants with device-safe parameter accessors and is
revalidated below.
The historical fixture generator was preserved byte-for-byte and was not run.

## Frozen evidence

| File | SHA-256 |
|---|---|
| `oracle131.py` | `2c1822ac76a592ad8c736c5247ae2fb3dd87ff506e00d1f4c556df5a023171b3` |
| `params131.h` | `a8eb458b6971c4b1c586cd733576bb3f26c2aff1fe6cae4d50e524973b1b1a8f` |
| `vectors131.h` | `e8ee77658edfc9b91da7a06acbc547b99b0fa0fb2c208689637d68f52f0b890c` |

## Host validation

Environment:

- Apple clang 17.0.0, target `arm64-apple-darwin25.6.0`;
- macOS/Darwin 25.6.0 on Apple Silicon;
- command: `make -C gpu/certicom131 host-test`.

Observed result:

```text
CERTICOM131 HOST VERIFY: PASS prime_field_vectors=256 binary_field_vectors=256 scalar_vectors_per_curve=16 subgroup_checks=4 elapsed_s=1.14681
```

The verifier checked every frozen prime multiplication/inversion vector,
every frozen binary multiplication/squaring/inversion vector, both published
points on each curve, `G+Q`, `2G`, identity and inverse-point cases, 16 scalar
vectors per curve, and the generator and target multiplied by the published
subgroup order.  The optimized host executable SHA-256 was
`1cc7edd33eb5822837875767536adf4f275d0c850fa329a40ec0a4fd9a884376`.

An ASan/UBSan executable was built, but the fully instrumented fixed-vector
run produced no verdict within the bounded 150-second window and was stopped.
That diagnostic is incomplete, not a pass or a detected arithmetic failure.

## Repository validation

The first `cargo test --release --lib` run had 4,382 passing tests, 118
ignored tests and eight failures.  All eight were missing-file failures from
tracked frozen inputs excluded by the inherited sparse-checkout rules:

- `research/notes/ecc2k130/n37_four_policy_support_20261003/SOURCE42.jsonl`;
- `research/cryptopro_b_chain_sweep_20261006/frozen_inputs/chains.constants.json`.

After materialising those exact tracked files with the repository sparse
checkout helper, all ten targeted tests passed.  The complete rerun on the
initial candidate was 4,390 passing tests.  After merging the latest main,
the required final-head rerun was:

```text
test result: ok. 4399 passed; 0 failed; 118 ignored; 0 measured; 0 filtered out
```

## CUDA validation gate

The preregistered CUDA replay targeted one AWS `g7e.2xlarge`, reported by the
EC2 API as one 96 GiB `RTX PRO Server 6000`.  Current on-demand pricing read
from the AWS Pricing API was `$3.36312/hour`; the job had a 30-minute TTL, so
its compute ceiling was about `$1.68` before minor storage charges.

No instance was created.  The native ephemeral launcher returned
`VcpuLimitExceeded`: this account's instance bucket has a vCPU limit of zero.
AWS instance readback found no instance for either launch record.  Therefore:

- no GPU compute charge accrued;
- the CUDA translation unit has not been compiler-validated;
- the device verifier has not run; and
- no GPU correctness or performance claim is made.

The two recorded launch attempts were:

- `20261019-014156-certicom131-cuda-verify-f56570`: the macOS Bash launcher
  stopped locally on GNU `stat`/`mapfile` assumptions and left a stale
  `launching` record with no instance;
- `20261019-014843-certicom131-cuda-verify-6c051f`: the native Rust launcher
  recorded the `VcpuLimitExceeded` error, with no instance.

## Campaign readback

Read-only Modal queries reported no active apps.  The `ecc2k130` volume held
36 checkpoint headers and 36 full checkpoints.  The highest Modal GPU run ID
was 8023; no run-8024 checkpoint existed.  The newest header was run 8011,
modified 2026-10-16 01:32 PDT.  This task did not redeploy the already-landed
campaign image and did not start a fleet.

## Requirement status

| Requirement | Status |
|---|---|
| Preserve recovered fixtures and provenance | verified complete |
| ECCp-131 native field and point arithmetic | verified on host |
| ECC2-131 native field and point arithmetic | verified on host |
| Exact known-answer and subgroup checks | verified on host and by the Rust library suite |
| CUDA implementation and device verifier | implemented, device validation blocked by provider quota |
| Persistent worker, durable DP store and collision replay | not implemented in this change |
| Throughput or campaign-rate comparison | not attempted |

The next validation action is an exact replay of `make cuda-test` on an
approved CUDA 13.x `sm_120` runner after the account has nonzero `g7e` quota,
or on another repository-approved RTX PRO 6000 runner.  The result remains
partial until that device receipt exists.
