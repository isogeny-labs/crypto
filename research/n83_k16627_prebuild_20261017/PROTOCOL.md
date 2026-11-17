# Frozen N83 K=16,627 prebuilt signed-Frobenius base

**Status:** preregistered construction and replay protocol. This file is
committed before the object is generated. The result and exact run receipts
are appended in a later commit without rewriting this protocol.

## Question and baseline

Can the existing native exporter construct, store and replay one primary
`GF(2^83)` signed-Frobenius base with 16,627 columns and 2,760,082 point
records under a hard 4 GiB memory limit? This is the first declared size at
which the uniform nonidentity-target four-summand multiset ceiling reaches
one. That ceiling gives a size target, not a measured relation yield.

The current verified construction baseline is the retained primary
K=1,182 public-x-hash, seed-2026100801 object in
`research/koblitz_n83_factor_base_sweep_20261008/verification/two-hour-extension-20261010/`.
Its construction, generic replay and S3 round trip passed; no comparable
K=16,627 construction or four-summand relation/rank receipt exists. This
experiment tests construction feasibility and integrity. It does not claim a
faster constructor or a complete index-calculus improvement.

## Frozen inputs

| Field | Value |
| --- | --- |
| Curve | `icv1-f2m83-tm6151469093347-debefd74`, `a=0`, `b=1`, low modulus terms `[0,1,2,45]` |
| Source policy | `public_x_hash` |
| Seed | `2026100801` |
| Closure | full signed Frobenius, 83 phases and two signs per accepted source |
| Columns | `16627` |
| Expected point records | `2760082` |
| Exporter | `examples/koblitz_n83_factor_base_export.rs` at this protocol commit; native release build, source hash captured by the manifest |
| Design | `research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json`, authenticated by the exporter |
| Output | one new directory under `/private/tmp`, never an existing object or historical receipt |

No target scalar enters the exporter. The accepted public source x values,
subgroup projection, point labels and object bytes are determined by the
frozen policy, seed, curve and exporter revision. The full object is stored
by content hash; a manifest without its retrievable object is incomplete.

## Execution and limits

Build and test the native exporter from the clean protocol commit. Use the
checked `v2-construct-one` and `v2-replay-one` entry points on Linux arm64
inside one-CPU, 4 GiB, zero-swap, no-network containers. The pinned
`rust:1.93.1-bookworm` arm64 image digest is
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
Build with `--release --offline --locked --no-default-features`, a read-only
source and Cargo registry, at most four CPUs, 8 GiB and 600 seconds. Record
the binary SHA-256, image digest, cgroup values, container exit and OOM
state, and any build failure.

Run `v2-construct-one NEW_DIR 0 public_x_hash 16627 2026100801 3600`
with a 3,900-second outer wall cap. Run `v2-replay-one NEW_DIR 3600` in a
fresh capped container with a 3,900-second outer wall cap. Each phase gets
one CPU, 4 GiB and zero swap. Copy the prior phase's output into the next
phase without modifying object or manifest bytes. Record process wall,
cgroup peak, compressed byte count and all object, plain and point-set hashes.
Do not combine a build from one revision with a replay from another.

If construction and generic replay pass, publish the content-addressed
object and manifest to the versioned S3 prefix declared by the exporter,
download the object and verify the returned bytes and digest. Keep the
compact receipts and retrievable URI in Git. If publication is unavailable,
mark durable reuse incomplete and retain the local bytes only as provisional
evidence. No local absolute path alone establishes durable custody.

## Verdict and follow-on gate

`PASS` requires a complete object, exact point/column counts, successful
generic replay, and an S3 download matching its compressed hash. An exporter,
curve, point, orbit or hash mismatch is `PRODUCER_FAILURE`. A hard wall or
memory cap is `UNKNOWN_budget` or `UNKNOWN_memory`; a Docker, disk or network
error is an infrastructure outcome. Preserve every failed attempt and raw
receipt. Construction timing on the shared host is descriptive L0 only.

The next independent gate is an m=4 full-width relation and useful-rank
screen of this base against a matched comparator, with a degree-83 planted
relation positive control and all cold costs charged. A complete-cost claim
requires the existing current IC baseline, A/A controls, a same-point rho
reference and independent replay. This construction gate alone cannot rank
base quality, choose a winner, or change the index-calculus scoreboard.
