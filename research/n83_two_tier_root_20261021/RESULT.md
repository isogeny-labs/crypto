# Exact split root index on the N83 K128 control

The split five-byte root index constructed 1,359,488 entries from the same
680,000 symmetric S3 states and 21,248-point N83 factor base as the
monolithic index. The two builds have identical base hash
`0d49fb3973e6cde8fc8dedb531c0f48e0bc00d077accfdc674fb4bab45c7e593`
and state digest
`8ff4de15e941ea6d1b6f526338382d06eb9e223d792a3c1cc6503efda1c75d60`.
The exact-root unit control additionally compared every key in a real
six-representative N83 S3 index with a core-first reference map and checked
missing-key probes. All 22 example tests passed.

The split mode uses a K64 primary and K64-to-K128 extension in this stage
control. It requested 2,621,440 more five-byte table bytes than the
monolithic index; Bloom storage is outside this table-byte figure. The
observed index-build times changed with execution order:

| Order | Arm | Index build (ms) | Process wall (s) | Root entries | Table bytes |
| --- | --- | ---: | ---: | ---: | ---: |
| monolithic, split | monolithic | 22,393.262 | 38.954 | 1,359,488 | 10,485,760 |
| monolithic, split | split | 15,918.803 | 26.972 | 1,359,488 | 13,107,200 |
| split, monolithic | split | 15,502.341 | 25.802 | 1,359,488 | 13,107,200 |
| split, monolithic | monolithic | 14,284.693 | 24.227 | 1,359,488 | 10,485,760 |

These are local stage diagnostics on a busy shared macOS host. The first
order favored the split build; the reverse order favored the monolithic
build. The stage timing comparison remains **inconclusive**. Process wall
includes startup and base construction; `index_build_ms` is the source's
internal index interval. The stage-only process exits before rank, target
decomposition, relation replay, and scalar recovery. Its `candidate_id` is
`null`, and it supplies no online speedup row.

## Shared Bloom variant

One Bloom filter now covers the union of the primary and extension tables.
The K128 shared-filter controls retained the same base hash, state digest,
and 1,359,488 exact-table entries. Both opposite-order stage controls passed;
the complete example test set passed again (22/22), including a real N83
root-key control with per-table filters removed.

| Order | Arm | Index build (ms) | Table bytes | Bloom bytes |
| --- | --- | ---: | ---: | ---: |
| monolithic, shared split | monolithic | 18,030.382 | 10,485,760 | 2,097,152 |
| monolithic, shared split | shared split | 28,389.694 | 13,107,200 | 2,097,152 |
| shared split, monolithic | shared split | 14,631.946 | 13,107,200 | 2,097,152 |
| shared split, monolithic | monolithic | 24,861.570 | 10,485,760 | 2,097,152 |

The shared-filter layout saves 524,288 Bloom bytes against two separate
split-table filters at K128. The corresponding frozen K4405 capacity formula
gives a 128 MiB Bloom saving. The table increment remains 671,088,640 bytes
at K4405. The two local timing orders again disagree, so index-build
performance and single-target online performance remain unresolved.

## Frozen receipts

- [First-order raw inputs and results](runs/k128_a/input.json) and
  [result](runs/k128_a/result.json): result SHA-256
  `94b656624a08b3e22a08a3b669cbd464bcc65aaada416b2d3f81cb13070cfe5c`.
- [Reverse-order raw inputs](runs/k128_b/input.json) and
  [result](runs/k128_b/result.json): result SHA-256
  `0d47c7400a2f6ee4cd0c475f6d3fd7689777b9507d304eaf99947e6dcd791a7d`.
- Both receipts embed source SHA-256
  `14834b41b4bb5ee041efc246acd03d5bab32449c90d80818d47af4bb15edbae9`,
  executable SHA-256
  `92601fa8824daf14afbd248c799de897fca6b76eb663aa697f708e443cd565fd`,
  target-file SHA-256
  `1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656`,
  and runner SHA-256
  `ed464d0e405c515e1f5fb63634f64f28ca580aef2516749c7d6c3bd86471ea84`.
  Each run directory retains the runner copy, per-arm input, stdout, stderr,
  and process outcome. The target file was only a fixed stage invocation
  input; the stage did not start target work.
- [Shared-filter first-order result](runs/k128_shared_a/result.json), SHA-256
  `36c61c1e13e803ce935af0e10f5204e0dc9e3fa28685ed5f669c3035ef75ad12`,
  and [reverse-order result](runs/k128_shared_b/result.json), SHA-256
  `36741665b377900b72a026de39f22eb2ef71fe3b8d77f4a95ae0faf0a7f06dda`.
  Both embed source SHA-256
  `c125b638d89e418936895659f1b3bd856b4e18a6a24f1d307dec99a204833cf0`,
  binary SHA-256
  `1e37d0cdf3af6329a94a92ee7101f8e094590b01ec3d3a95aba6b56f7c007083`,
  and frozen runner SHA-256
  `ada510798505a4d460b1c5f1dd25e0930a237d0fbb5acb4ca7449b348d46205b`.

## Next qualification step

Build the exact K4405 base with K4300 split point, then independently replay
the same public target and its relation and scalar for both layouts. Compare
the one-target online interval with alternating Tier 1 same-instance pairs
only after a fresh cloud allocation is authorized. A capped local K4405
load80 monolithic correctness control is already running separately; its
terminal receipt and replay determine whether the full-size path is feasible
on this host.
