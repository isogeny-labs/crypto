# One packed stream a partition in the folded pair-table build

- **Status:** measured and kept, 2026-11-14, on this session's container.
- **Why this step:** after the partition-local filter, pushing pairs into
  the partition streams was the build's largest step.

## The change (`src/cryptanalysis/koblitz_index_calculus.rs`, `fold_partitioned`)

- **One `u64` a pair:** each pair goes into its partition's stream as a
  single `u64`, not a hash plus a stored word.
- **Why it fits:** no step reads hash bits 34–37, and a partition's top 8
  bits are its own. Those 12 bits carry the orbit, and the stored word is
  rebuilt from the hash as the pair is placed.
- **When it applies:** a tagged table, no duplicate harvest, 256
  partitions, at most 26 bucket bits and at most 4,096 orbits.
- **Control:** `KIC_FOLD_PACK=0` keeps two streams.
- **Tables:** identical word for word, checked against the two-pass build
  by the existing tests, with packing on and off and under both filter
  layouts.

## A/B ([`ab/PROTOCOL.md`](ab/PROTOCOL.md), frozen before any row; [`ab/report/report.pdf`](ab/report/report.pdf))

42 rounds, one binary, a same-session A/A. Ratios are two streams over
packed.

| row | build | set-up | collect |
|:--|:--|:--|:--|
| k0n53 | **1.244** [1.214, 1.276], established | **1.068** [1.051, 1.080], established | 0.981, no regression |
| k0n61 | **1.252** [1.223, 1.266], established | 1.038 [1.020, 1.050], not established | 1.009, no regression |

- **Mechanism:** stream push 13.28 → 7.11 and 14.83 → 7.90 ns per stored
  pair.
- **Identity:** zero mismatches.
- **Scope:** a component result on this host class. It makes no claim
  against rho and adds no board row (validation gate).

Micro-benchmark source: [`microbench/pb.rs`](microbench/pb.rs) (the
`packed` variant).
