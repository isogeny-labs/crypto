# Frozen Linux full-D18 phase-first access gate

This is a separate source-bound capacity gate following the audited L0
[constructed/natural pair](../phase-first-natural-20261025/RESULT.md). It uses
the same primary Koblitz `a=0` GF(2^83) curve, public fixture `(a=0,0)`,
complete S3-custodied D18 base (32,643 signed-Frobenius columns and 5,418,738
point rows), `m=4`, identity mask zero, shared-root-bit circuit, all selector
and phase choices free, and phase-zero-first search policy. The public target
is `(1834913121178197242856238,8797964190732846213981377)`; the natural
worker reads only the point, never its known-answer scalar. The local 300-s
natural arm was censored after a verified import and model build.

The host must check out one clean source commit and build one release binary
from it. Freshly fetch the exact S3 object and hash-bound manifest; reject
any compressed SHA-256, provider checksum, byte length, producer-manifest or
curve identity mismatch before running a worker. Retain the complete GET
metadata and the source/binary/image digests. The worker itself replays every
stored point and signed-Frobenius orbit map before modeling.

Run the constructed `positive` arm first, then the public `natural` arm only
if the constructed arm returns `PASS_CONSTRUCTED_CONTROL` and an independent
Python auditor reproduces its original-field four-point sum, signed-orbit
coefficient row, changed-target rejection and rank one. Both arms use the
same binary and fetched base, each with an exact 8 GiB hard memory cgroup,
zero swap, one CPU, read-only source/base/binary mounts and no worker network.
This 8 GiB limit is a distinct capacity profile from the 32 GiB
per-representative protocol; compare their measurements only with explicit
source, encoding, host, memory and clock context. The worker has a 1,200-s
internal whole-process cap, at most 100,000 SAT conflicts and at most one SAT
model. A separate supervisor enforces a 1,210-s outer cap. The workflow may
fail before launching if the host cannot provide the declared cgroup.

Retain UTC start/end times, exact argv, source and binary SHA-256, full
stdout/stderr, exit status, phase/shape/result/cap files, import/model/search
and total process wall, cgroup peak and host measurements, S3 GETs, and an
inventory of every raw file's bytes and SHA-256. On a natural SAT result,
require a separate original-field replay of the four stored points, row,
target sum and tamper control before recording a checked relation. A cap,
missing solver answer, incomplete model, group rejection or malformed
receipt is censored or invalid, never a measured zero relation rate. This
single public query cannot select a base: natural relation yield on disjoint
targets, rank growth, factor-base logs, unseen-target descent, complete cold
one-target cost and matched same-target rho remain required.
`selected_best_total_runtime` remains `null`.
