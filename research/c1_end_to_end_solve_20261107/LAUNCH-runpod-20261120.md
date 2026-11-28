# Stage 1 launch on RunPod (2026-11-20)

The user asked to run stage 1. The Iso Lab route named in the protocol is blocked (CodeBuild queue limit 0, invalid artifact-store keys). This uses the RunPod route proven on 2026-11-17 (`research/trace_c1_2lp_20261027/runpod-20261117/`).

| field | value |
|---|---|
| specs | `specs/c1-e2e-m5-std.json` (SHA-256 `c98774c0…`), `specs/c1-e2e-m5-tw.json` (`bb4fb317…`), unchanged since the freeze |
| binary | `ecbench` built at the freeze commit `234e6193833f9db0f3811e79cfae61c6f743dee8`. Current main changed `band.rs` and `methods.rs`, so it is not admissible under the protocol's identity rule. SHA-256 recorded below after the build |
| host | one RunPod SECURE `cpu5c` pod, 8 vCPU, `runpod/base:1.0.2-ubuntu2404`, created through REST v2. Files move through `research/runpod_kit_20261112`. The pod is deleted after the run |
| execution | the two specs run concurrently as two `ecbench run` processes (one thread each), then `ecbench verify --replay-all` per session |
| isolation | **deviation from the protocol:** a RunPod pod reaches at most L1, not the L2 the specs require. Wall time is not admitted. The counted-operation (GAE) comparison and the §5a band are unaffected |
| analysis | `analyze.sh STD TW` as frozen (SHA-256 `6ab14fac…`) |
| ceiling | USD 2, failures included |
| checkpoint | S3 is unavailable; this pushed commit and draft PR #4855 are the checkpoint |

## Log

- Binary: built at the freeze commit `234e6193` in place, SHA-256 `3c04b4be2d9b79cd02632852d2402eb9ead0a00e9e11228b0926ce9ec66443f8`. A one-target local check passed: 2/2 verified, 2/2 replays reproduced.
- 2026-11-20T22:08Z: pod `r94vjjpl59a3nh` created (`cpu5c`, 8 vCPU, USD 0.28/h, EUR-IS-1). Binary and both specs were uploaded and hash-checked.
- The specs were launched concurrently. `ecbench` permits one timed session per host (`/tmp/crypto-bench.lock`), so the std run was refused at once (rc 2, "another benchmark holds /tmp/crypto-bench.lock"). This is a launch error on my side. std was re-queued with `--wait` as a separate job.
- **tw completed:** `run rc=0 wall=672s`, then `ecbench verify --replay-all` gave "audit OK: 180 records, 180 verified, 150 replays reproduced, 0 problems". The packed `results.tgz` had SHA-256 `ef3592598ade31c16b184c360a227cfb5797bf91dd023a1eb23bbdf9d06871b9`.
- **The pod disappeared before anything was fetched.** By 2026-11-20T23:00Z, `GET /v2/pods/r94vjjpl59a3nh` returned 404, and the account listed no pods at all, including another session's `replead-r1-20261116`. The REST v2 API does not show the account balance, and the cause was not established: possibly the balance running out, as on 2026-11-09, or an account-wide termination.
- **Lost:** the tw sessions (only the audit line above survives) and the std run, which was mid-way. **Measured rows kept: 0.** The tw audit line is a receipt that the run completed. It is not a result, because its records were not preserved.
- **Lesson:** fetch each spec's sessions as soon as that spec finishes, not at the end.
