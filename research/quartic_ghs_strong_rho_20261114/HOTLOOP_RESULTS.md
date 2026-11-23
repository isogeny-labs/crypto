# Strong-rho hot-loop result: implemented bookkeeping cleanup

Status: deterministic development controls passed; checkpointed profiling,
allocation instrumentation, isolated timing, and the complete frontier grid
were not run. These producer results are not independently admitted
`[pcc:unverified]`.

## Outcome

All four prospectively allowed subchanges landed:

1. The batched look-ahead path now owns one preallocated reusable scratch object. Its
   first partitions, chosen jumps, candidates, retry lists, batch-add active
   indices, denominators, prefix products, inverses, and outputs are cleared
   and refilled without discarding retained capacity. The public standalone
   batch APIs remain available as allocating wrappers.
2. Each walker caches its current x-coordinate hash after first use, and the
   final candidate hash computed by the look-ahead test is reused for the
   distinguished-point test and next partition. A restarted walker uses a
   lazy cache so an unused restart does not execute or charge an extra hash.
3. Partition selection uses a mask when `r` is a power of two (including the
   frozen `r=32`) and modulo otherwise. The self-test checks representative
   hashes against modulo for `r=1,2,4,8,16,32` and `r=3,5,31,33`.
4. The distinguished-point table reserves 128 entries. The algorithm never
   iterates the table, so this does not alter lookup, insertion, or collision
   order.

The release and debug standalone self-tests both passed. Their source-bound
regression oracle fixes every field count and every walk/native count for
widths 1, 8, and 32. The candidate recovered the same planted logarithms and
preserved the same transitions and abstract work:

| width | found | steps | DPs | restarts | look-ahead | cycles | cycle adds | `band_ops` | raw hashes, baseline -> candidate |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 1 | 17 | 1,177 | 63 | 0 | 33 | 1 | 5 | 4,625 | 3,573 -> 1,220 |
| 8 | 118 | 1,917 | 119 | 0 | 65 | 1 | 5 | 6,852 | 5,831 -> 2,002 |
| 32 | 219 | 1,421 | 76 | 0 | 45 | 0 | 0 | 4,480 | 4,346 -> 1,517 |

For each row, setup/walk/verification field counts, RNG draws,
canonicalisations, look-ahead tests, distinguished-point tests, table
operations, cycle comparisons, scalar additions, scalar multiplications, and
scalar inversions are exactly equal to the baseline. Only `NativeCounts::hashes`
changes. That field records executed `hx` calls; hashes are already the
implementation of charged partition/DP/table work and are excluded from
`band_ops`, so the reduction is not re-priced or described as less ECDLP
work `[pcc:unverified]`.

The reusable-buffer conclusion is a source and correctness result, not a
measured allocation result. The repeated batch-add control confirms retained
capacities on a same-size second call, but no allocator counter or profile was
collected. No throughput claim follows from this result.

## Provenance and controls

- Baseline revision: `ad2a014c0022cf000845014499a9649b6c2cef2d`.
- Baseline source SHA-256:
  `0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`.
- Candidate implementation commits:
  `3bb159455fd116507bfcaed0ae5167a226bfee16` and
  `d4f7fa7f0a67706b6f283dbde002684e1a622049`.
- Candidate source SHA-256:
  `fd515a7f54ba92359fafdf065b5404682903922031f2de20b0b39e48b6cb4fc3`.
- Baseline release binary SHA-256:
  `6174b4e04e473e06e6cd839e43b70b47df3a785d3a8c6f819e08405c4e7c9b9e`.
- Candidate release binary SHA-256:
  `95a4536538968107400bce7a57945650ae107d65354a32495630f06421ebbd86`.
- Candidate debug binary SHA-256:
  `58c9318ee32570556be620dbb00fb10bd9507c782c732689ba38de41260e82a6`.

Both candidate binaries were built from the same source with
`rustc --edition 2021 main.rs`, adding `-O` for the release control, and run
with `selftest`. `rustfmt --check` and `git diff --check` also passed. The
baseline oracle was collected with the same compiler invocation before the
candidate edit; wall-time fields were excluded.

## Blocker and retained alternatives

`scripts/checkpoint.sh gate` exited 2 because `.resume/RESUME.md` was absent.
Per the frozen protocol, no Callgrind/profile invocation, A/A screen, ABAB
panel, full driver record, or timing substitute was run. A measured
throughput or frontier change remains unestablished `[pcc:unverified]`.

The following variants were considered and deliberately not included:

- Eagerly hashing in `new_walker` would move hashes across the setup/walk
  boundary and could hash a restarted walker that is never consumed; the lazy
  cache avoids both diagnostic changes.
- Replacing the table hasher was outside scope and would add a separate
  security/performance choice; only capacity was reserved.
- Reworking cycle traversal to pass hashes between scalar escape steps was
  deferred because cycles are rare in these controls and that larger API
  change was unnecessary for the coherent hot-loop cleanup.
- Removing modulo unconditionally was rejected; the non-power-of-two fallback
  remains covered by deterministic identity checks.

The research-KB wrapper had no executable and Conductor was not installed.
Those availability failures support no novelty, performance, or frontier
claim. The next discriminating step, once a resume checkpoint can pass, is a
source-bound profile followed by the protocol's separate A/A and at least
five-pair ABAB screen.
