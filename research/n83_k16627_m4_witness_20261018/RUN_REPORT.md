# N83 K=16,627 phase-observable complete-witness control

**Run:** 2026-10-18 UTC. **Frozen protocol:** [PROTOCOL.md](PROTOCOL.md),
committed before worker implementation. **Outcome:** `UNKNOWN_wall_cap` for
the positive arm. The dependent unpinned arm was not attempted. This is a
bounded oracle diagnostic, not a natural relation-yield or full index-calculus
measurement.

## Exact object and source

The primary curve is `icv1-f2m83-tm6151469093347-debefd74` over
`GF(2^83)`, with the 81-bit prime subgroup. The retained
`public_x_hash`, seed-`2026100801` base has **16,627 signed-Frobenius
columns** and **2,760,082 points**. Its compressed object BLAKE3 is
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`;
its point-set BLAKE3 is
`564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`.
The earlier [construction](../n83_k16627_prebuild_20261017/RUN_REPORT.md)
and [model-capacity](../n83_k16627_m4_capacity_20261017/RUN_REPORT.md)
studies are separate runs; their timings are not added to this arm.

The exact worker source was the clean pre-stack commit
`f68b9a94ada669360f88be4d8c8a5288cca55d73`, with five
[source-file hashes](source-files.sha256), a read-only
[snapshot attestation](source-attestation.json) and five checked
[input hashes](inputs-before-run.sha256). The native arm64 release binary
SHA-256 was
`5345802d80a1ec7bfee04be5913a15536ec355a388debfde3d543db1ca4b1318`.
See the [build receipt](BUILD_RECEIPT.md) and [executed commands](COMMANDS.md).
The three touched Rust files match the stacked review branch byte for byte;
that branch's inherited ecbench/Gaudry compilation failure is reported
separately in the build receipt. This result is bound to the buildable
pre-stack source, not a passing current-head build.

## Observed execution

The positive arm fixed four stored signed-orbit representatives, computed
their two intermediate group sums and final target with the native primary
group law, then installed the **full** K=16,627 Frobenius-orbit domain.
The first four summand x coordinates were distinct, all intermediate sums
were affine, the directly checked row raised a fresh wide rank tracker to
one, and the same row was rejected against `R+G`. These are the worker's
group-preflight checks; no SAT-produced relation was returned.

| Durable event | Elapsed wall from worker start | Receipt |
| --- | ---: | --- |
| Exact base import done | 39.982 s | object and point-set digests checked |
| Group preflight done | 39.999 s | rank after insert 1; `R+G` tamper rejected |
| Full orbit domain installed | 40.252 s | 201,598 variables; 5,915,238 clauses; 3,901 XOR rows |
| Seven x blocks checked/pinned | 40.259 s | four summands, two internal sums, previously fixed target |
| SAT solver entered | 40.267 s | 100,000-conflict cap; one-model cap |
| SAT solver returned | **no durable record** | result, conflicts and model count unknown |

All five records are in the append-only, flushed
[phase JSONL](positive/worker.json.phases.jsonl). The outer container ran
from **03:16:27 to 03:46:27 UTC**, one CPU, hard **4 GiB** memory, zero
swap, no network, read-only root. Its exit was **124**, `OOMKilled=false`,
with cgroup peak **2,661,351,424 bytes**. There was no worker result JSON;
stdout and stderr are empty. The [outer outcome](positive/outcome.json),
[container state](positive/container-state.json),
[host configuration](positive/host-config.json), UTC timestamps and cgroup
peak are retained. The interval from the synced `sat_entered` record to
outer stop is 1,759.73 seconds; it includes the solver call and termination,
and is not a measured conflict count or a separate solver-completion time.

## Interpretation and next control

The prior pinned arm's unlocalized timeout is now narrowed to the solver
call **after** exact-base import, valid group preflight, complete orbit-domain
installation and seven-block pinning. The receipt does not distinguish the
solver's initial propagation from later conflict search. Its SAT status,
conflicts, model count, field-checked model, group lift and post-SAT rank
remain unknown. Exit 124 is a wall cap, not UNSAT or a measured zero yield.
The preregistered unpinned arm required a passing positive control, so it
was **not attempted**. No natural-query yield, useful-rank comparison,
linear-algebra cost, target extraction or complete IC total is produced;
the canonical IC scoreboard and matched-rho gap are unchanged. These L0
walls were observed on a shared host and are not a speed claim or an
external-tool comparison.

The next bounded control should keep the exact object and full model but
emit durable checkpoints around root propagation and then completed
conflict intervals. That separates clause/XOR propagation from CDCL search
without guessing from wall time. A constructed full assignment to the SAT
clauses is another independently checkable positive-control route if the
solver remains capped. Either path needs its own frozen protocol, source
and verification receipts. Natural-query and useful-rank comparisons for
the near-size-matched K=2,048 explicit base and 2,001-column structured
base still determine factor-base quality. The [diagram](witness-phases.svg)
shows completed stages, the censored solver call and the untouched natural
comparison; canonical IC graphs are unchanged because no relation/rank
panel or end-to-end row was added.
