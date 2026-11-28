# Exact Curve25519 workloads and bounded Autolab methods

This integration allows ecbench and Autolab to carry actual Curve25519 public targets and full-width recovered scalars through workload creation, method dispatch and result verification. It adds a bounded signed-pair relation collector and a matched full-width rho adapter. These are implementation and local-test observations, without signed PCC standing. [pcc:unverified]

## Behavior

- Workload facts retain the exact subgroup order and cofactor. Public target decoding rejects noncanonical, off-curve and non-subgroup inputs. Planted and public target laws replay with full-width scalars; legacy narrow sampling retains its original streams. [pcc:unverified]
- `rho.curve25519_bounded` requires an explicit iteration cap up to 1,000,000. The existing rho loop may overshoot that threshold by up to 17 counted group operations; setup and verification are additional retained work. The cap is a resource envelope, not a tuning knob. [pcc:unverified]
- `ic.curve25519_pair_v1` constructs G plus distinct signed classes of cofactor-cleared coordinate lifts. It admits only the known logarithm of G initially. Signed two-summand decompositions are checked as full group equations, then exact modular elimination attempts target-coordinate recovery and verifies the scalar against the public point. Base size is bounded at 128; scan and trial limits are explicit and bounded at 1,000,000. [pcc:unverified]
- Autolab accepts explicit method and reference configurations and freezes them across training and holdout. Existing implicit miniature routes remain supported. A reference mean is usable only for a nonempty, completely verified cohort with finite nonnegative costs; distances also require positive denominators and finite ratios. An IC improvement against its incumbent can ratchet independently of reference availability, retaining raw reference counts and a null reference cost. [pcc:unverified]

## Accounting

Both adapters retain target-validation work. Collector records include factor-base construction, validation, query/decomposition work, relation admission and recovery, plus unpriced matrix, hash and lookup API counters. Relation verification and matrix insertion currently share the target-descent timing phase. Allocations, hash-map internals and internal square-root operations are not completely priced. A primary online comparison requires the repository's complete phase/cost contract and independent replay; the local tests below do not supply that comparative evidence. [pcc:unverified]

## Validation and retained failures

| Check | Observed result | Artifact |
| --- | --- | --- |
| Frozen-source Curve25519 library tests | 35 passed, 0 failed, 1 ignored | `native-tests-migration-continuation.log` |
| All final Autolab tests | 22 passed, 0 failed, 0 ignored | `autolab-reference-guard-tests.log` |
| Device-emulator group after host compiler restoration | 6 passed, 0 failed | `device-host-recheck.log` |
| Final complete library gate | 6,086 passed, 0 failed, 130 ignored; 1285.60 seconds | `final-library-tests-attempt2.log` |

All entries are producer observations. [pcc:unverified]

`final-source-manifest.json` pins the six integration source files. The full Autolab suite includes actual ledger persistence with an unavailable reference, duplicate-ratchet rejection, legacy JSON encoding and explicit full-width routing. The collector dispatch tests include full-width recovery and exhaustion. [pcc:unverified]

Preserved failures include earlier missing tracked fixtures and the omitted rho optimization policy, interrupted native builds with unknown outcomes, and the first final-source full-library attempt's five missing-C++ failures. After reproducing ENOENT at the compiler invocation, gcc-c++ and libstdc++-devel were restored and all six emulator tests passed. The known-failed full pass was deliberately terminated (exit -15); it is not a successful complete gate. Logs and exit receipts remain separate. [pcc:unverified]

## Publication and execution status

The final full-library gate passed; final-validation.json binds its logs, terminal receipt and source hashes. Source publication and current-head PR checks remain pending. No comparative experiment has been launched for this integration. The latest cloud admission observation is `cloud-admission-20261107.json`: configured role assumption was denied, live capacity limits were unavailable, and the daily spending snapshot exceeded the shared allowance. Resume cloud work only after authoritative budget/access reconciliation and exact-source GitHub publication; preserve the original controls and finite attempt limits.
