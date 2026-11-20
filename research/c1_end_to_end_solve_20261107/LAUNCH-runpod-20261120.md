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

