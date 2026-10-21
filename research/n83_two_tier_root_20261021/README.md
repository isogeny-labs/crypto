# N83 K4300-prefix root index for the K4405 factor base

The opt-in `KIC_INDEXED5_SPLIT_AT=4300` mode builds an exact two-table root
index for the existing K4405 base. The first table contains states whose two
representative indices are below 4300; the extension table contains the
remaining states. A lookup searches the first table before the extension,
preserving a deterministic core-first witness for duplicate roots. Both
tables resolve five-byte entries against the complete one-root state array,
so a fingerprint match alone cannot produce a relation.
`KIC_INDEXED5_SPLIT_SHARED_BLOOM=1` is a second opt-in on the split mode: one
Bloom filter covers both tables and can reject a missing key before either
exact-table probe. The per-table filters are disabled in this mode. The
shared filter is sized for the full K4405 key request, so its negative answer
is valid for the union.

## Question and fixed comparison

Compare the split index with the monolithic indexed-five-byte load80 index
using the **same K4405 points, one previously unseen public target, rank/log
preparation, source revision, and resource envelope**. The primary metric is
verified single-target online wall time, with the interval and phase costs in
`AGENTS.md`. Record setup, index build, resident memory, negative/positive
lookup counts, verified relations, and rank as separate diagnostics. Alternate
execution order for a Tier 1 cloud-instance comparison and retain every
timeout, failed replay, and noisy pair. Pair the eventual selected IC result
with same-point rho before reporting an IC speedup.

The frozen K4300 base from the prior RunPod comparison is an exact prefix of
the frozen K4405 base: 4,300 representative records and 713,800 point labels
match. The archive SHA-256 is
`352766bd9610615b10b0aa8f6a10d28a4880f0cb8851c33ee300a199bb9e6ece`.
This experiment changes the index layout and first-witness priority, not the
factor-base point set. It is a distinct implementation variant; its
`candidate_id` remains `null` until a complete canonical manifest is frozen.

## Capacity and controls

For symmetric N83 states, `S(K) = 83*K*(K-1)/2 + 42*K`. Thus
`S(4300) = 767337150`, `S(4405) = 805269240`, and the extension contains
`37932090` states. At load80, the requested primary and extension tables
round to `2^31` and `2^27` five-byte slots. The extension adds 671,088,640
table bytes beyond the monolithic K4405 table, plus its Bloom filter and
allocation overhead. Memory and query cost must therefore be measured rather
than inferred from the smaller extension.
With the shared-filter mode, the two-table Bloom request is replaced by one
full-base request. At K4405 and the `wide3` profile, the separate filters
would allocate 2 GiB plus 128 MiB; the shared filter allocates 2 GiB. The
table-byte increment remains 671,088,640.

The same opt-in split path also permits a calculated K5086/K4549 layout at
the current 30-bit state-ID boundary. [CAPACITY.md](CAPACITY.md) gives the
exact sizing formulas, Lean-checked bound and split implication, memory
diagram, and missing execution checks. The projected 844,276-point count is
not an enumerated factor base; `candidate_id` stays `null`.
The [prepared max-K comparison](MAXK_PROTOCOL.md) freezes 192 fresh screen
points and eight disjoint holdout points with a zero-overlap audit against
33,336 earlier public points. Cloud spending, exact base identity, and the
new executable receipt remain gates before dispatch.

1. Run the exact-root unit control: real N83 S3 roots are partitioned at a
   small prefix, compared with a core-first reference map, and probed for
   missing keys and duplicate witnesses.
2. Build the same K4405 base with split mode off and on; compare state count,
   base hash, exact-root coverage, memory, and index-stage digests.
3. Independently replay every emitted relation and the recovered scalar for
   a frozen public target. A root or scalar mismatch rejects the variant.
4. Run matched Tier 1 one-target pairs only under a fresh cloud budget and
   instance allocation. The previous RunPod allocation is absent and the
   earlier capped campaign exceeded its $5 upper bound, so no cloud result is
   implied by these local correctness controls.

Status: **K128 index-stage controls passed; full-size target qualification in
progress**. No measured online row or candidate selection has been entered
for either split variant.

The bounded small-prefix stage control uses the same executable in both arms:

```sh
python3 research/n83_two_tier_root_20261021/run_index_smoke.py \
  --binary /absolute/path/to/koblitz_orbit_dlp_fast \
  --target /absolute/path/to/one_public_target.jsonl \
  --output /absolute/path/to/new_receipt_directory \
  --k 64 --split 32 --timeout-seconds 900
```

Add `--shared-bloom` to control the shared-filter variant, and
`--split-first` to reverse the execution order.

Its `PASS_INDEX_STAGE_CONTROL` status checks the identical base hash, state
digest, and split allocation receipts. It is an index-construction control;
the full K4405 target and Tier 1 online gates above remain separate.

The local full-size correctness control is prepared as a capped successor to
the monolithic load80 run. It takes the same public target point and records
the independently known fixture scalar outside the executable's input. It
monitors sampled RSS and writes a terminal process receipt on success,
failure, timeout, or a memory-cap stop:

```sh
python3 research/n83_two_tier_root_20261021/run_full_control.py \
  --binary /absolute/path/to/release/koblitz_orbit_dlp_fast \
  --target /absolute/path/to/one_public_target.jsonl \
  --expected-scalar 8171303969165027 \
  --output /absolute/path/to/new_receipt_directory \
  --variant shared --cap-seconds 7200 --rss-cap-gib 26
```

The monolithic control terminated at its 7,200-second cap; its receipt is
linked from [RESULT.md](RESULT.md). Run the split control only when no other
large index is active on the host. `PASS_PROCESS` is a process
outcome; an independent group, relation, and scalar replay must then pass
before entering a correctness row.

After a terminal process receipt, run:

```sh
python3 research/n83_two_tier_root_20261021/verify_full_control.py \
  /absolute/path/to/new_receipt_directory
```

The verifier uses the versioned N83 group implementation and K4405 candidate
manifest to check the exact base dump, relation sum, recovered scalar,
single-target phase accounting, and split-table allocation. A capped process
returns `BOUNDED_UNKNOWN`; only `PASS_INDEPENDENT_REPLAY` qualifies the local
correctness control.
