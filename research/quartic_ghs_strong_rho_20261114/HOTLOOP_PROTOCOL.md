# Prospective protocol: strong-rho hot-loop bookkeeping cleanup

Status: frozen on 2026-11-18 before candidate source changes or candidate
output. Study ID: `ghs-strong-rho-hotloop-r1`.

## Question and scope

Can the merged Round B strong-rho implementation remove transient allocation,
redundant point hashing, and runtime power-of-two partition division without
changing a single mathematical walk transition, collision/table decision, or
priced abstract operation?

This is an implementation-engineering study on the public synthetic
known-answer GHS control. It does not change the curve family, target policy,
32 partitions, batch width, look-ahead rule, distinguished-point rule, cycle
escape, collision recovery, or cost model. It does not amend or execute the
pending complete frontier grid.

## Pinned baseline and sources

- Baseline revision:
  `ad2a014c0022cf000845014499a9649b6c2cef2d` (`origin/main`).
- Baseline `strong_rho.rs` SHA-256:
  `0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`.
- Frozen controls: [`PROTOCOL.md`](PROTOCOL.md), [`ROUND_B.md`](ROUND_B.md),
  and [`round-b/RESULTS.md`](round-b/RESULTS.md).

Sources checked before choosing the candidate were the files above and
`src/cryptanalysis/jv_rho_strong.rs`, whose counted/uncounted arithmetic and
batched-inversion walk show the same separation between mathematical work and
implementation hooks. The research-KB wrapper reported that no executable was
available, and Conductor was not installed. No novelty or absence claim is
based on those failures.

## Candidate changes

The coherent candidate may include only these semantics-preserving changes:

1. Reuse preallocated scratch vectors for look-ahead inputs, retry indices,
   batch-add active indices, denominators, prefix products, inverses, and
   outputs.
2. Cache the hash of each walker's current affine x-coordinate. Reuse the
   candidate hash already computed by the look-ahead partition test for the
   distinguished-point test and the next transition.
3. Compute `h mod r` with `h & (r-1)` only when `r` is a power of two, with
   the general modulo fallback retained and cross-checked.
4. Reserve the distinguished-point table's expected small capacity before the
   walk. The implementation never iterates this table, so capacity may not
   affect collision order.

Rejected or incomplete subchanges remain in the result note. No changed walk
schedule, altered retry rule, new hash function, custom table hasher, or field
arithmetic change is in scope.

## Requirements-to-evidence matrix

| Requirement | Evidence | Pass condition |
|:--|:--|:--|
| Field and curve correctness | standalone self-test | all frozen field, batch-inverse, curve-add, and planted-log controls pass |
| Exact walk identity | baseline/candidate deterministic record or a source-bound regression control | same instance, target, found scalar, steps, DPs, restarts, look-ahead retries, cycles, cycle additions, field counts, scalar counts, and table operations |
| Abstract accounting identity | native count comparison | `band_ops` and every component except raw `hashes` are identical |
| Hash reuse | raw diagnostic counts | any reduction is reported only as fewer executed `hx` calls; it is not re-priced or called a walk reduction |
| Partition identity | deterministic helper tests | mask and modulo agree for power-of-two `r`; fallback agrees for non-powers of two |
| Scratch reuse | code inspection plus correctness controls | buffers retain capacity across rounds and field/native counted work is unchanged |
| Wall-time result | checkpointed profile plus separate A/A and at least five ABAB pairs | absent unless all repository profiling, isolation, benchmark-record, and output-identity gates pass |

## Controls, stop rules, and reporting

The bounded development control is the existing `q=223` public synthetic
known-answer self-test at widths 1, 8, and 32. A full driver record, profile,
Callgrind run, or timing panel is an experiment and may run only after
`scripts/checkpoint.sh push` and `gate` pass. Checkpoint refusal is retained
and leaves those rows unmeasured; it does not authorize a smaller timing
substitute.

Stop the candidate on a wrong scalar or point, any transition/count divergence
outside the prospectively allowed raw-hash delta, a non-power-of-two partition
mismatch, missing raw output, or checkpoint refusal before an experiment.
Preserve failures. A passing source/self-test is an implemented correctness
result only; it is not a throughput result or a complete frontier update.
