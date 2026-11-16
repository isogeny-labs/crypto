# Sparse paged binary pair index: source-bound screen

Recorded 2026-11-16. This is a producer correctness check and a derivation of
fixed requested backing bytes. It has no complete-call timing or process-RSS
measurement. The standalone index is not wired into the elliptic-curve
collector. Protocol commit `d80f3e97f1`; prototype source commit
`3007d15058`. A later [editorial protocol amendment](AMENDMENT-20261116.md)
adds only a prose standing marker. [pcc:unverified]

## Mechanism, sources and exact scope

The current torsion-harvest builder retains one first witness per binary
pair-sum key and reports later equal-key pairs as collision candidates. Its
opt-in flat direct layout builds the hash map first, then initializes
`2^n + 1` eight-byte words. The proposed 16-word paged layout would insert
into a four-byte directory and preallocated pages directly. Its lookup uses
one directory read and, when present, one page read. The caller still must
verify full group equalities and maintain the same relation and rank path.
The [editable diagram](diagram.dot) and [SVG](diagram.svg) show the proposed
data path. [pcc:unverified]

Sources checked: current `src/cryptanalysis/ic_boundary.rs` at
`7a3c3f2153`; [the three collision-census records](../rootfind_pair_collision_census_20261116/reports/run-20261116T0711Z/REPORT.md)
and their K13/K19/K23 JSON; the internal [rootfinding program paper](../../papers/ecdlp-index-calculus-program/sections/rootfinding.tex),
KB source `source:e9db8cc2be204429e8fb1daa12a3324c`, which already covers
binary pair tables and dense-index engineering; and the prior
[N83 dense-root protocol](../n83_dense_root_20261013/PROTOCOL.md), which uses
a different table. The shared KB was ready at revision
`9141e386254340a9bdba3036f2bb706a` but last refreshed on
2026-11-14, before the current source and census. No novelty claim follows
from this screen. [pcc:unverified]

The fixed cells are Koblitz E0 with subgroup orders 2,003 (K13), 130,873
(K19), and 2,095,853 (K23), cofactor four, factor-base abscissa counts 17,
70, and 138. Full curve IDs and base digests are copied into
[`storage.json`](storage.json), with SHA-256 hashes of each input JSON.
That file is derived from the source-bound census, not a new curve run.
[pcc:unverified]

## Correctness observations

`rustc 1.93.1` on macOS arm64 compiled the dependency-free
[`paged_index.rs`](paged_index.rs) test executable from committed source.
All four tests passed: an ordered deterministic 16,384-pair stream matched a
hash map's first-witness and collision sequence, and every possible K13 key
was checked against that map. The other tests covered key 1 and `2^13`,
`i=0`, `j=u32::MAX`, missing keys, insufficient page capacity and invalid
degree, key and witness inputs. This tests the storage semantics only; it
does not check actual Koblitz pair sums or target answers. Complete output is
[`test.stdout`](test.stdout); build and test stderr were empty.
[pcc:unverified]

The Python [derivation](derive_storage.py) and a separate shell/`jq`
[cross-check](verify_storage.sh) returned exit zero and the same integers for
all three cells. The cross-check is a second implementation by the same
producer, not an independent review. [pcc:unverified]

## Derived backing requests

For degree `n`, the flat layout requests `8(2^n+1)` bytes. The paged
prototype requests `4(floor(2^n/16)+1) + 128 min(floor(2^n/16)+1, a^2+1)`
bytes for `a` base abscissae. The second term preallocates enough 16-word
pages for the current folded builder's insertion bound. It does not depend
on a favorable distribution of pair-sum keys. [pcc:unverified]

| Cell | Retained keys | Flat initialized words | Flat requested bytes | Paged requested bytes | Flat / paged bytes |
|:--|--:|--:|--:|--:|--:|
| K13 | 261 | 8,193 | 65,544 | 39,172 | 1.673 |
| K19 | 4,800 | 524,289 | 4,194,312 | 758,404 | 5.530 |
| K23 | 18,842 | 8,388,609 | 67,108,872 | 4,534,916 | 14.798 |

At K23 the flat layout initializes 8,388,609 words for 18,842 retained
keys, a derived 445.21 words per key. The table and
[quantitative SVG](requested_backing.svg) describe requested backing only.
They exclude `Vec` headers, allocator overhead, collision rows, factor-base
storage, curve arithmetic, rank, verification, RSS and wall time. No
conclusion about actual memory peak or speed is supported. [pcc:unverified]

## Decision boundary

`dominated_by`: the current hash table and opt-in flat direct table are the
reference layouts. The paged prototype requests less fixed backing on the
three cells and skips the intermediate hash build by construction, but it
adds a directory read, page allocation bookkeeping and a different memory
access pattern. Their complete-call ordering is unknown. The directory
still grows with `2^n/16`; this bounded layout does not extend to N83.
[pcc:unverified]

`sota_delta`: zero change to relation-search exponent, factor-base content,
relation yield or rank in this screen; no demonstrated complete-call cost
change. After T-6708 releases the collector files, the next check is a
guarded production integration with exact first-witness, collision-stream,
signed group re-add and K13/K19/K23 answer parity. A separately frozen
same-target timing panel then decides whether fewer initialized bytes and
direct insertion compensate for the added lookup. The branch-specific S3
checkpoint remains reserved by T-6342, and the frozen EKS L2 F4/F5/F6 panel
has its own admission gate. Canonical performance graphs were checked for
scope and not changed because this screen has no timing result.
[pcc:unverified]
