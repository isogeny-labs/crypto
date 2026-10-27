# Frozen first m=4 query gate for the retained structured N83 base

**Status:** protocol, no run receipt yet. Freeze this file in a commit before
executing the query gate. A later change records actual commands, source
revision, environment, all outcomes and independent replay.

## Question

Does the native four-summand chained-S3 path construct and issue a bounded
query against the retained dimension-12 signed-Frobenius factor base on
`icv1-f2m83-tm6151469093347-debefd74`? This first gate measures execution
and correctness on one base. It does not choose a factor base or establish a
complete index-calculus cost.

## Frozen inputs

| Input | Identity |
|---|---|
| Curve | a=0, GF(2^83), modulus low terms `[0,1,2,45]`, subgroup and Frobenius eigenvalue checked by the worker against the archive |
| Structured object | `aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389.jsonl.gz`, 10,058,074 compressed bytes, 2,001 signed-Frobenius columns, 332,166 points; compressed BLAKE3 `aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389` |
| Manifest | [retained manifest](../koblitz_n83_factor_base_sweep_20261008/verification/structured-d12-orbit-20261010/manifest.json), worker-pinned BLAKE3 `d6adad9bec74350f37bf917e737006869bc0f343692f668fa4dc893afe332745`, file SHA-256 `bd4202ed599f2659f6279dd07dbdc6b5948594df79db969e66530c19e3cf5dca` |
| Public corpus | [probe-corpus.json](../koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json), file SHA-256 `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd` |
| Known-answer validation | [probe-validation.json](../koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-validation.json), file SHA-256 `1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f` |
| Source | worker and library commit pinned in the later run configuration; the worker checks its compiled source hash in output and imports only the pinned object bytes |

The object remains at the S3 URI pinned in the manifest. Download to an
ephemeral input directory, verify both compressed hash and byte length, and
pass that directory read-only to the worker. Do not rewrite the retained
object, manifest or historical receipts.

## Execution and limits

Use the native worker's `verify` mode first, including its manifest/object
tamper tests. Build a Linux worker from the checked commit and run `run4`
inside a 4 GiB hard-memory, zero-swap cgroup. The outer supervisor uses thin
shell orchestration and a hard wall deadline. Do not use the historical Python
supervisor for this new research run.
Each query container has one CPU, no network, all capabilities dropped,
`no-new-privileges`, a 900-second worker budget and a 960-second outer cap.
The input mounts are read-only and the output directory is new and writable.

The Linux build uses the local `rust:1.93.1-bookworm` image pinned to
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`
on arm64, `cargo build --release --offline --locked --no-default-features
--example koblitz_n83_structured_orbit_worker`, a read-only source and Cargo
registry mount, no network, four CPUs, 8 GiB memory with zero swap, and a
600-second outer build cap. Record the binary SHA-256 and Docker exit/OOM
state. A build cap or missing offline dependency blocks the run and is
preserved as an infrastructure outcome.

| Parameter | Frozen value |
|---|---:|
| Summands | 4 |
| Strategy | `ChainedS3FrobeniusOrbit` |
| Query seed | `2026100901` |
| Direct relation policy | disabled |
| `max_trials` | 8 |
| `conflict_budget` per query | 5,000 |
| `max_models` per query | 32 |
| `max_variables` | 150,000 |
| `max_domain_clauses` | 12,000,000 |
| Worker budget | 900 seconds |
| Memory limit | 4,294,967,296 bytes, zero swap |

Run the unchanged five-summand `run` command on the same inputs and limits
as a source-path control. Report construction, import, query, group-lift,
rank, memory and wall intervals separately; these arms solve different
decomposition arities, so their timing ratio is not an algorithm speedup.
Retain the full event stream and independent replay of any returned relation.

The existing degree-7 library tests are correctness controls for m=4,
including original-group lifting and cap handling. They do not replace a
degree-83 planted-relation control. That control remains an open requirement
before an observed natural yield or rank is promoted to a base-quality claim.

## Outcomes and stop conditions

- A verified relation must replay against the original group, signed-orbit
  labels and exact wide coefficient row before its rank increment is counted.
- `UNKNOWN_solver_cap`, `UNKNOWN_budget` and an unfinished trial remain
  censored. Zero completed relations at a cap is a bounded observation.
- A source, object, field, lift or answer mismatch is `PRODUCER_FAILURE` and
  blocks comparison until repaired. Missing cgroup, object or worker access is
  an infrastructure outcome.
- The next paired gate uses this structured object and the near-size-matched
  K=2,048 explicit-orbit object under one corpus, matched controls and all
  charged cold phases. The [open problem](../../problems/n83-prebuilt-base-quality-m2-m4.md)
  remains unresolved by this first query gate.
