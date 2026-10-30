# RTX PRO 6000 query transfer diagnostics

This is an observation report for a software panel. It makes no scientific
promotion, certified timing, fastest-tool or full ECDLP performance claim.
The GPU service remains generic decomposition (enumerated group sums), not
an algebraic relation solver or a general implementation of the paper.

RunPod pod `bbrl08j1o8gizz` ran source
`68be1081e696c232881b2da8d864d93b297e5092` on one RTX PRO 6000 Blackwell Server
Edition, UUID `GPU-6f4be1b4-e646-c053-0f84-110a43b0e103`, 97,887 MiB, compute
capability 12.0, driver 595.91.07, CUDA 13.2.51 and Rust 1.88.0. Creation was
2026-10-30T02:16:09.784Z. Controller, bootstrap and validation exited zero;
pod deletion was verified. No continuing GPU workload belongs to this run.

The new [protocol](protocol.json), hash
`5b519ba744fbe7c14a1ab32296959d322bb467f64044536fcc2c5523253ce020`, was frozen
before measurement. The previous normalization panel informed the design but
is not pooled with this panel. All arms use product-tree normalization.

- `u`: previous separate match buffer and host reset upload.
- `v`: packed winner plus device-side asynchronous reset (new default).
- `w`: packing alone, retaining the old host reset upload.
- `x`: second invocation of the exact `u` binary (A/A).

All 120 planned rows completed: three cases, four arms, one warmup and nine
measured repetitions. No rows failed, timed out or were excluded. The native
analyzer checked unique cells, witness/visited digests, normalization digests
and payload identities. Inputs, targets, caps and order are specified in
[PROTOCOL.md](PROTOCOL.md); the exact command is
`EPHEMERAL_OUT=/results bash research/explicit_relations_query_20261030/run.sh`.

## Recorded query timings

Milliseconds for four queries, median [minimum, maximum], n=9. Each workload
has four bases of the listed size, two planted hits and two independently
exhausted misses. These are L0 shared-host diagnostics; CPU exclusivity and
stable clocks were not certified. Every export `wall_valid` flag is false.

| Points/base | Previous u | New default v | Packing only w | A/A x |
|---|---|---|---|---|
| 16 | 0.161201 [0.157776, 0.162794] | 0.138247 [0.137145, 0.140079] | 0.146458 [0.142333, 0.154852] | 0.163915 [0.163324, 0.167430] |
| 64 | 0.174400 [0.160831, 0.176574] | 0.142262 [0.139719, 0.156494] | 0.157365 [0.152608, 0.158326] | 0.173539 [0.164385, 0.188442] |
| 256 | 1.854872 [1.776946, 1.866299] | 1.747181 [1.693391, 1.757738] | 1.840191 [1.769996, 1.853690] | 1.841893 [1.775373, 1.866630] |

Derived descriptive differences `(u_median-v_median)/u_median` are 14.24%,
18.43% and 5.81%. They describe this panel, not a certified speedup. The A/A
case-0 observed ranges also do not overlap; retain that warning about host
variability rather than treating disjoint ranges as statistical proof.

Cold medians u/v were 148.872484/148.824673, 150.270249/149.864362 and
153.363963/151.367678 ms. Candidate case-2 preparation reached 9.680058 ms
and cold cost reached 241.054600 ms; both remain in the data. Full stage
ranges, including all A/A observations, are in [summary.csv](results/summary.csv).
No cold end-to-end speedup is established. Query timing includes transfers,
synchronization and native witness verification; prepare includes allocation,
table construction, sort and synchronization; cold includes input generation,
GPU context creation, prepare and queries. The independent host comparison
is validation overhead outside these intervals. The separate normalization
interval excludes readback/verification and is unchanged across arms.

## Exact API payload accounting and correctness

On the encodable fixtures, u/x use 16 bytes per launched batch plus 8 per
hit; w uses 16 per batch; v uses 8 per batch. The observed batch/hit accounting
gives u/v/w payloads 80/32/64 bytes for each small case and 800/392/784 bytes
for the largest. These counts exclude preparation, device writes and bus
framing. The packed path removes the `8*4096 = 32768` byte match buffer.
It preserves explicit chunk limits, early stopping, visited counts and
right-then-left witness order. Oversized radix products use the old array
path; the sentinel cannot collide with an admitted key.

All three query variants passed boundary/reset checks. Host and CUDA service
tests, independent affine replay, bounded paper reduction/certificate tests,
10 Rust tests and release Clippy passed. Compute Sanitizer reported zero
memcheck errors, zero racecheck hazards/errors/warnings, and zero synccheck
errors. The new regression checks are wired into `make test-cuda`.
Root crypto/ecbench integration was not compiled locally under the user's
disk constraint. The paper driver remains bounded to p<=65521; the GPU
tensor backend and general paper algorithm remain outside this change.

## Evidence and next work

[export.json](results/export.json) passes the canonical export checker with
file verification and zero findings. The frozen native analyzer consumes
neutral labels; no human blinding is claimed and no unmasking time was
recorded. Planning runtime was an allocation estimate; round zero retains
the actual preflight. The raw `passed` label is mapped to schema `verified`
only in export metadata; raw rows are unchanged.

Raw archive, complete logs, provider receipt and deletion receipt:
`s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/runpod-bbrl08j1o8gizz/`.
The downloaded `results.tgz` matched SHA-256
`7de347dd22d60b6989e2efc9c6f26090973d6f82dd50834e6ab01aa5569e9afd`.
No API credential is in the archive. Source/binary hashes are retained.
The chart is generated by the frozen [analyzer](analyze.rs); the inspected
[PDF](reports/run-20261030T0216Z/report.pdf) embeds it.

The next implementation target is repeated right-side tuple construction,
with any caching charged to preparation and admitted under explicit memory
limits. That needs a separate frozen comparison; no new run is scheduled.
