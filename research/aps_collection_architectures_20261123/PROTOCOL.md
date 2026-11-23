# Bounded architecture correctness plan

Planning contract frozen with the source before the dedicated CLI call.
This is a custom correctness plan, not `experiment.protocol/v1`, an admitted
performance protocol or a canonical export. Producer standing is unverified.

Request: implement all four November 23 leads. Preserve large-prime and
recursive decomposition priority, the multi-target alternative and the
pilot-frozen partition alternative. Do not substitute this tiny qualification
for the still-open growing-size/exponent goal in PR #4531.

## Fixed workload and acceptance

- Both fixtures from the package README. Disclosed check scalars 7, 11, 16.
- Partials: targets [7] and [7,11,16], each with one/two-large-point routes,
  512 draws, 6-point pool, three known small points. Duplicate pairs are
  intentional controls, not independent random sampling. Retain every match,
  every unmatched draw, dependent row, graph constraint and deficient outcome.
- Recursive: 8 slots with labels [0,1,2,3,4,5,6,r-1], all 28 intermediate
  pairs; compare complete roots for each of three targets with a four-loop
  oracle, cold construction and shared table. Library checks cover every
  group target, duplicate points, opposite pairs and changed replay inputs.
- Partitions: four fixed rules (slot index, x, y, x+3y modulo four); pilot
  scalars [3,5] and seeds [11,13], then freeze. Fresh scalars [7,11,16], seed17.
  Same frozen rule and no retraining. Compare fixed-index rule on identical
  fresh inputs. Retain full/admitted roots, useful and zero-B counts.
- Require source-curve validation of partials, emitted constraints, recovered
  scalars and recursive tuples. Rank deficiency and failures remain explicit.
- Transport-only call FIRST via the actual binary: four planned/attempted,
  zero scientific completions/exclusions, all four deliberate errors retained.
  Qualification requires four planned/attempted/completed, zero exclusions.
- No timing, host capsule, performance profile, cloud experiment or scaling
  run. Counts are diagnostic and unpriced. No success threshold on a speedup.

## Accounting and integration gaps

This packet is a bounded native affine lab. Full production APS/F4 routing,
known-label sampler integration, all phase/bit-operation/memory pricing, canonical
export/replay, stronger matched baselines and prospective performance protocol
remain open. A batch cannot replace the cold single-target endpoint. Independent
signed standing is not supplied by this producer.

All previous ledger/shared-log/raw-build/host/private-history publication
restrictions and merge hold remain. Existing source S3 approval persists but
no future performance run can rely on the old checkpoint: fresh exact source
and protocol capture plus all accounting/replay/export/admission requirements
are still required. Canonical schema access was previously unreadable; this
custom report does not claim schema conformity.

## Sources checked

- `research/pencil_large_prime_20261029/README.md` and `src/lib.rs` at main
  015f50a6526b0ca85ad4556dc62fa20900e4723c: reuse symbolic graph; an unbalanced
  cycle can anchor a root rather than yield a small-only relation.
- `tools/aps-partition-lab/src/curve.rs` at the same pin: bounded ordinary
  affine arithmetic; it is shared by collector and enumeration, not an
  independently implemented field-arithmetic audit.
- Gaudry, Thériault and Thomé, February 2005 preprint,
  [double large-prime variation](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf),
  sections 2–3: partial relation graph mechanism; its genus>=3 analysis does
  not transfer as an elliptic prime-field complexity result.
- Amadori, Pintore and Sala,
  [prime-field elliptic curves](https://iris.unitn.it/handle/11572/202653),
  sections 2–3: known labels and informative dependencies; decomposition cost
  and yield must both be included.
- KB unavailable: no loaded status/search tools and the repository wrapper
  was unavailable in the retained sparse source. Used the exact mechanism
  paths above; no novelty or best-known-method claim.

## Preserved development failures

Initial native library run: 9 passed, 1 failed because the stream did not
exercise the asserted duplicate-row outcome. The prospective stream was
changed to explicitly repeated draw pairs; the assertion was retained.
Initial strict Clippy found an index-only row loop. Replaced it with a cloned
pivot row and zipped mutable iteration; no lint suppression or weakened check.
Original logs remain private. Later checks keep their own source boundaries.
