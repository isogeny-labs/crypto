# N83 V5 ordered K600 zero-trial preflight

**Run identity:** `run-20261116-v5-ordered-preflight`, attempts 1 and 2.
**Scope:** frozen input and implementation admission for V5 ordinal
`26960859236`; zero relation trials. The complete single-target cold
index-calculus runtime and factor-base ranking remain unset.

The [protocol](../../verification/v5-ordered-preflight-20261116/PROTOCOL.md)
and [original runner](../../verification/v5-ordered-preflight-20261116/run-preflight.sh)
were committed at `28b3d207d0b6` before attempt 1. The
[write-once amendment](../../verification/v5-ordered-preflight-20261116/AMENDMENT-1.md)
and [attempt 2 runner](../../verification/v5-ordered-preflight-20261116/run-preflight-attempt-2.sh)
were committed at `d6e61824a862` before attempt 2. The amendment
records a receipt defect; it does not alter source, input or limits.

## Frozen workload and controls

The exact curve is `icv1-f2m83-tm6151469093347-debefd74`, subgroup
ordinal `26960859236`, primary `a=0` K600 hash base, seed
`2026100801`, 600 signed-Frobenius orbit columns (99,600 signed points),
six summands, zero split bits, signed-Frobenius and ordered-first-pair
symmetry, SAT CDCL/binary, zero large primes, one thread and modular
BigUint linear algebra. The source snapshot is
`cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`; Linux ARM64 binary
SHA-256 is `87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`.
The image is `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`.
The previous [exact binding controls](../../verification/v5-linux-binding-20261116/attempt-1/RESULT.md)
admitted this ordinal and rejected an adjacent ordinal and a bad
manifest; they are a separate run.

Both attempts used a fresh GET of the existing K600 object at
`s3://setec-astronomy-artifacts/store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.
Account `104186811093`, content length 3,025,798 bytes and local GET
SHA-256 `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`
were checked before each worker. This is the physical content-addressed
object key; the factor-base store name is a separate logical index key.
The compressed object is omitted from Git because the same exact object
is durable in S3. Re-extract with
`aws --profile ecc-challenges-new s3api get-object --bucket setec-astronomy-artifacts --key store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf <output>`
and check the byte count and SHA-256 above.

## Observed results

| Attempt | UTC host interval | Worker status / strategy | Trials, relations, SAT calls | Docker exit / OOM | Admission |
|---|---|---|---|---|---|
| [1](../../verification/v5-ordered-preflight-20261116/attempt-1/) | 15:40:18–15:40:22 | `PREFLIGHT_ONLY` / `ordered-orbit-chain-s3` | 0, 0, 0 | worker 0; inspection overwritten | incomplete inspection receipt |
| [2](../../verification/v5-ordered-preflight-20261116/attempt-2/) | 15:43:07–15:43:11 | `PREFLIGHT_ONLY` / `ordered-orbit-chain-s3` | 0, 0, 0 | 0 / false | passed the frozen preflight gate |

These intervals have one-second timestamp resolution and are operational
receipts, not comparative algorithm timings. Attempt 2's preserved
[inspection](../../verification/v5-ordered-preflight-20261116/attempt-2/container.inspect.json)
shows Linux ARM64, one CPU quota, 4,294,967,296-byte memory limit,
zero swap, 128 PID cap, no network, read-only root, exit 0 and
`OOMKilled=false`. The worker
[summary](../../verification/v5-ordered-preflight-20261116/attempt-2/output/run/summary.json)
records a 107,536,384-byte cgroup peak, 600 orbits, 601 matrix
columns, zero rows and rank, and no verified target log. Base import
and worker timings are retained in that raw summary; they exclude
construction, archive custody, build, stopped attempts, relation
collection, rank acquisition, target work and verification. No
end-to-end ratio is computed.

Attempt 1's worker [summary](../../verification/v5-ordered-preflight-20261116/attempt-1/output/run/summary.json)
also passed its internal zero-trial checks. Its shell trap replaced the
successful Docker inspection with `[]` after container removal, so the
requested independent Docker inspection cannot be audited. The
[amendment](../../verification/v5-ordered-preflight-20261116/AMENDMENT-1.md)
preserves that failure and explains the repeat. Both attempts' exact
command, object metadata, identity, disk samples, stdout/stderr,
sidecar, config, event stream, summary and outer receipts are in their
linked write-once record directories. The S3 read used the
`ecc-challenges-new` profile; neither attempt wrote to S3.

## Boundary and next experiment

The existing [K600 orbit-chain preflight](../run-20261116-k600-preflight/attempt-6-zero-trial-pass/REPORT.md)
is an input-staging reference with a different strategy; it is not a
paired runtime baseline for this V5 cell. The paired positive-control
and natural-query relation arms need their own frozen protocol, native
model replay, measured relation/rank/target phases, and matched rho
before any factor-base choice or total-runtime comparison. The
requested complete combination sweep remains open. The current
preflight establishes only exact V5 input/resource admission.

The [flow diagram](FLOW.svg) shows the recorded outcome of each attempt;
[its source](FLOW.dot) and [PDF report](REPORT.pdf) are beside this
report. No canonical IC scoreboard or progress graph changes because
zero relation trials and no end-to-end cost do not add a graphable
factor-base ranking. Checked: `docs/ic/boundary_targets.json`,
`docs/index-calculus-scoreboard.html`, and `docs/ic/progress-timeline.json`.
