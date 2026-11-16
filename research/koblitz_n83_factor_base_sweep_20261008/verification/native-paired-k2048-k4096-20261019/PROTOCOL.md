# Frozen same-host K2048/K4096 bounded stage control

This is a descriptive same-query, same-host comparison of retained
signed-Frobenius K2048 and K4096 factor bases, not a selection of the best
factor base or an index-calculus total-runtime comparison. Both archives are
public, previously replayed objects from the finite N83 sweep. The K2048
compressed bytes have SHA-256
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`
(10,313,647 bytes); K4096 has SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`
(20,621,466 bytes). The exact Git sidecars and public fixture are pinned by
`verification/cloud-matched-k2048-k4096-20261018/input/fresh-inputs.json`.
Runner-readable copies reside under the already used, encrypted
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261019-015400-n83-k4096-phases-5438e8/input/`
prefix. Long-lived factor-base custody remains in the original
`crypto-autoresearcher` bucket and the hash-identical setec copy.

Run the pushed clean source commit on one native x86-64 `c7i.xlarge` host
through `taskq/deploy/ephemeral/ephemeral --detach` with a two-hour TTL.
Compile the exporter worker once. Pin the Docker image by the previous
native run's `python:3.11-slim` registry digest
`sha256:e88e9763f943ec1834f992a4b51e0f24500486803e8bc534e5767af9ea65f6ce`.
Each run is isolated to one CPU, 4 GiB hard memory, zero swap, and no
container network. The four sequential arms are **K2048, K4096, K4096,
K2048**. Each uses the same public fixture 0, the archived factor-base
selection seed `2026100801` as the CLI input, and the fixed internal relation
sampler seed `2026100901`; five summands, one trial, at most 150,000 variables, 2,000,000
domain clauses, 32 models, and 1,000 conflicts. Each has a 600-second
internal worker wall cap and 620-second outer Docker cap. Preserve all four
raw outcomes. Stop further arms on infrastructure failure; a bounded
`UNKNOWN_solver_cap` or wall-censored arm remains a recorded result.

Before launch, the exact staging preflight must pass locally with both S3
archives. On the host it repeats SHA-256 checks for archive, all panel
sidecars, and both fixture sidecars before compilation. The source snapshot
must match compiled bytes and the pushed commit. For each completed arm,
independently replay the public query, compare exact coefficients/target
across all four arms, check model/phase counters, preserve cgroup peak and
worker wall, and report the whole paid job wall. The ABBA repeats are an A/A
variability check for bounded stage cost, not independent natural-relation
samples. Do not call a faster 1,000-conflict arm the best total-runtime
factor base. Keep `selected_best_total_runtime: null` unless the separately
required full relation-yield, rank, column-log, individual-log, verification,
and matched-rho gates are complete.
