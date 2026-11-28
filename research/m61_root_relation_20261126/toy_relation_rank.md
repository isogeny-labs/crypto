# Actual seven-term relation rank on a toy prime field

The selected M61 census predicts roughly 20,078 seven-term homogeneous
rows after cofactor projection, but their rank is not known. The
[dependency-free Rust probe](native/examples/toy_relation_rank.rs) tests
the **rank step** on the previously certified toy curve
`y²=x³−3x+40` over `p=131071`, with `#E=37·3541`. It projects every
base point by `[37]` into the prime-order `r=3541` subgroup.

For each base of size `B`, the program enumerates all signed distinct
three-column and four-column sums, matches opposite projected points,
rejects overlapping indices, and deduplicates the resulting signed
seven-column rows under global negation. Every unique row is replayed by
elliptic addition, then inserted into incremental Gaussian elimination
modulo `3541`. A row appears 70 times in the raw meet-in-the-middle
matches: 35 choices of its three-column partition and two global signs.
This factor matches every nonzero measured case.

| B | Root unique rows | Root rank / `B−1` | 16 random-x control rank range | 53 nonzero root-window rank range |
|---:|---:|---:|---:|---:|
| 10 | 0 | 0/9 | 0–4 | 0–7 |
| 12 | 12 | 11/11 | 7–11 | 6–11 |
| 14 | 52 | 13/13 | 13–13 | 13–13 |
| 20 | 1380 | 19/19 | 19–19 | not scanned |

The 53 cyclic windows at `B=10,12,14` use the root base after removing
its one `[37]`-killed column; these windows overlap heavily. Their
mean unique-row counts are 2.23, 13.42, and 61.62, close to the 16
random controls' means 2.38, 13.31, and 61.00. At `B=12`, 30 of 53
windows have full rank; at `B=14`, all 53 do. The first `B=10` root
window's zero-row result is not representative of every root window.

For comparison, a uniform prime-subgroup model gives
`2^6·C(B,7)/3541` expected global-sign-folded seven-term rows:
approximately 2.17, 14.31, 62.03, and 1401.10 at the four sizes. The
observations are compatible with that rough scale, but the finite,
overlapping controls do not prove uniformity or M61 rank. They do show
that actual toy rows can reach rank `B−1` once enough independent
relations exist, while relation count alone is inadequate at small `B`.

At `B=20`, the collector builds 9,120 signed triples and streams 77,520
signed quartets, performing 261,000 counted group additions for the root
case. Its general time is `Θ(B⁴)` and triple-table memory `Θ(B³)`.
Thus this is a **rank diagnostic**, not a cube-root-scale M61 relation
generator or target descent. The complete deterministic output is
[frozen here](toy_relation_rank.out).

## Trace-base rank control on the same toy curve

The same executable now forms complete `x=t+t^−1` trace bases for
`μ_51` and `μ_102`, retaining only orbits whose points survive `[37]`
projection. It verifies each free horizontal chord row and then uses
the same exhaustive 3+4 collector, group-law replay, and elimination
over `F_3541`.

| Torus order | Columns | Free orbit rank | Seven-row count | Seven-row rank | Combined rank | Seven-distinct-orbit rows / rank |
|---:|---:|---:|---:|---:|---:|---:|
| 51 | 12 | 4 | 7 | 6 | 6 | 0 / 0 |
| 102 | 21 | 7 | 2,037 | 20 | 20 | 44 / 20 |

For `μ_102`, the seven orbits each have three columns. The exact
signed seven-distinct-orbit tuple count is `2^7·3^7`; dividing by
`2·3541` gives a uniform estimate of 39.53 zero rows, versus 44
actually found. These 44 verified rows **alone** reach rank `B−1`.
This is direct toy evidence that orbit-distinct rows can exist and span
the log relation space. For `μ_51`, only four valid orbits exist, so a
seven-distinct-orbit row is impossible, and its seven-row rank stops at
six. Sixteen equal-size random-x controls per trace case reach rank
5–11 at `B=12` and 20–20 at `B=21`; the trace result is not presented
as a unique statistical advantage over random bases.

This control does **not** locate a seven-point M61 relation. The toy
collector still enumerates `Θ(B⁴)` quartets and stores `Θ(B³)` triples;
for `B=21` it streams 95,760 signed quartets. The asymptotic relation
locator and both target descents remain missing.

The example now also completes a toy ECDLP replay using **only the 44
orbit-distinct rows** for its rank-20 nullspace. It chooses a generator
`G=[123][37]P_0` and target `Q=[1777]G`, with neither `G` nor `Q`
equal to a signed factor-base point. A signed triple-sum table finds a
six-column, six-distinct-orbit decomposition of each point. Both sums
are checked by group addition. The relation nullspace gives
`d=1777 mod 3541`, and `[d]G=Q` is checked independently. The scalar
1777 is used only to construct and finally verify the synthetic target;
descent and recovery take its point coordinates. This is an end-to-end
small-field **functional control**, not an M61 attack: the toy six-term
descent costs `Θ(B³)` time and memory in addition to the quartic row
collector.

An exhaustive support check further enumerates all
`7·6^6=326,592` signed six-column tuples taking one column and sign
from each of six of the seven trace orbits. It verifies coverage against
every point of the order-3,541 subgroup: **all 3,541 points are
covered**, with 67–117 decompositions each. This removes target
cherry-picking on the toy instance. Its tuple enumeration is finite
evidence; the per-target triple-hash witness search remains cubic in
`B`, and no corresponding M61 coverage or runtime is established.

Reproduce:

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example toy_relation_rank
```
