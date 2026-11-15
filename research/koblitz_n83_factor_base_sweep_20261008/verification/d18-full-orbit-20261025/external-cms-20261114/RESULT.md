# N83 primary D18 external CryptoMiniSat probe, 2026-11-14 UTC

The frozen D18 full-orbit exporter produced two source-bound XCNF inputs. The one-thread CryptoMiniSat 5.14.7 positive-control run reached its 1,000,000-conflict limit and returned `INDETERMINATE` after 420.708264458 seconds. The natural-query solver arm was not launched under the frozen positive-control gate. This run therefore supplies a measured censored solver cell for the free-first-pair, phase-zero-first D18 encoding; it supplies no selected end-to-end factor base.

## Identity and freeze

| Item | Pinned value |
| --- | --- |
| Curve | `icv1-f2m83-tm6151469093347-debefd74`, Koblitz `a=0` |
| Source | `45ce91d503ee0d0acc57e3d8d5cb49108c90ded6` |
| D18 source object | 5,418,738 enumerated points, 32,643 folded columns; compressed SHA-256 `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b` |
| Worker binary | SHA-256 `5f302b8cfd86470e4d08da34c972308f0c8b5dbc80422dc8b578625f8e075fd2` |
| Solver binary | CryptoMiniSat 5.14.7, SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac` |
| Formula lock | [EXPORT_LOCK.json](EXPORT_LOCK.json), written before the solver call |

The export uses a free first pair, `phase_zero_first`, and cadence 1. [ENCODING_CORRECTION.md](ENCODING_CORRECTION.md) records the discrepancy from the earlier ordered-first-pair, cadence-32 attempt before solver timing. These are different candidates; their solver times must not be divided into a backend speed ratio.

## Execution

| Arm | Frozen XCNF | CNF / XOR rows | Terminal status | Solver wall / cap | Peak sampled RSS |
| --- | --- | ---: | --- | ---: | ---: |
| Constructed positive control | SHA-256 `0d7bb7ef45ae19937220945129f61baccdd72b85c58bb7f16dcb56ca3ec24d7c` | 2,807,752 / 3,901 | `INDETERMINATE`, exit 15, 1,000,002 conflicts reported | 420.708 s / 1,000,000 conflicts and 600 s internal time | 341,072 KiB |
| First natural public query | SHA-256 `b0f91be5e1580c36f60e3a5bd5558193c7fc14d773f938656cae66fbcdcea4f3` | 2,807,254 / 3,901 | Exported; solver withheld by positive-control gate | null | null |

The 660-second external watchdog did not fire. Three host preflight loads were 9.65, 9.34, and 8.77, below the frozen 24 threshold; swapouts remained 164,320. An independent read of the retained solver output found exactly one `s INDETERMINATE`, the reported conflict count, and stdout/stderr hashes matching the supervisor receipt. The positive control has no SAT model to replay, so [verify_xcnf_model.py](verify_xcnf_model.py) was self-tested but was not used to promote this result. No natural-query relation, matrix, target descent, or same-point rho comparison was measured in this probe.

The recorded [PCC measurement claim](../../../../../pcc/claims/PCC-20261114-bea135.json) has `unverified` standing [pcc:unverified]: its raw receipt is unsigned, and the measurement proof contract requires independent recompute or replay. This report labels the execution receipt as retained, locally audited evidence.

The compact raw receipts are in [evidence](evidence). [SHA256_MANIFEST.json](evidence/SHA256_MANIFEST.json) pins every retained receipt and the 50 MB XCNF files and 161 MB compressed source object retained in the local `/Volumes/SSD990-2/tmp/n83-d18-cms-20261114` custody directory. The latter large files are omitted from Git. The probe's outcome is a censored cell for this exact encoding and cap, not a negative result for D18 as a construction or for either N83 curve as a whole.

The next selection step is a frozen end-to-end solver/collector/descent comparison on both N83 curves with successful controls and independently replayed same-point IC/rho pairs. The `a=1` panel remains a distinct curve and candidate set.
