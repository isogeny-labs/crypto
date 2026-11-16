# Independent factor-base / graph replication protocol

**Protocol ID:** `factor-base-graph-replication/20261024/v1`

**Freeze time:** 2026-10-24 UTC, before any replication target was generated
or candidate arm was run.

## Question and fixed cohort

Replicate the five-arm factor-base / large-prime graph experiment from
`research/four_followups_20261021` on a second, independently derived cohort.
The cohort uses the same 21 public exact-order Scholten models, seven size
slices (16, 20, 24, 28, 32, 36 and 40 bits), and 60 targets per model: 1,260
public known-answer targets in total. Each target is a separate cold solve.

No deployed curve, secret target or unknown scalar is used. Every recovered
scalar is checked by recomputing `k*P` on the source curve and by comparing it
with the planted public scalar.

## Independent deterministic derivation

The input identity for a target is the UTF-8, NUL-separated tuple

```
factor-base-graph-replication/20261024/v1
bits
curve ICV1 identity
target_index
```

For each identity, compute SHA-256 with the two distinct domain prefixes below.
Interpret the first eight digest bytes as a big-endian unsigned integer.

* target scalar domain: `target-scalar/v1`; set `k = 1 + value mod (r-1)`;
* algorithm seed domain: `algorithm-seed/v1`; use the value as the `u64` seed.

The replication workload digest is SHA-256 over the target identity followed
by the big-endian encodings of `k`, target `x` and target `y`. Its workload ID
is `W` followed by the first twelve hexadecimal digest characters. Target and
algorithm domains are disjoint. The original cohort's planted scalar,
algorithm seed and target coordinates are not inputs to either derivation.

## Frozen arms and execution

The implementation, candidate cap, transfer, signed elimination and complete
cost accounting match the original experiment. The five arms are:

| Arm | Factor-base denominator | Jump table entries |
| --- | ---: | ---: |
| `full16` | 1 | 16 |
| `half16` | 2 | 16 |
| `quarter16` | 4 | 16 |
| `full8` | 1 | 8 |
| `full32` | 1 | 32 |

Each arm runs all 1,260 targets. The source negation-rho comparator runs in the
same binary and process on every target, using the same target and algorithm
seed. Candidate and rho costs include construction, transfer or initialization,
search/relation work, elimination and independent source verification. Failures,
exhaustions and incorrect recoveries remain explicit records.

## Frozen pricing and summaries

Price every target in the same 36 cells as the original experiment:

* `S/M` in `{0, 0.5, 1}`;
* `I/M` in `{1, 10, 100, 1000}`;
* logical-operation price in `{0, 0.1, 1}` source-addition equivalents.

For arithmetic counts `C` and measured source-addition unit `U`, the price is

```
(C.M + (S/M)*C.S + (I/M)*C.I)
-------------------------------- + logical_price * logical_operations
(U.M + (S/M)*U.S + (I/M)*U.I)
```

where logical operations include recorded native counters, field additions and
bit operations. Aggregate ratios are ratios of cohort cost sums, never means of
per-target ratios. Report wins (`candidate/rho < 1`), losses (`> 1`) and exact
ties for every arm over all 36 cells. The representative cell is
`S/M=0.5`, `I/M=10`, logical price `0.1`.

## Acceptance and reporting

Completion requires 6,300 arm-target records, 6,300 verified candidate answers,
6,300 verified same-process rho answers, unique workload IDs within each arm,
identical target sets across arms, all 36 price cells, a native fail-closed
validator, SHA-256/byte manifests, a quantitative graph and an inspected PDF.
One deterministic seed per target supplies no sampling confidence interval.
Operation counts are the measurement; wall time is diagnostic on this shared
host and cannot support a speed claim. The result is kept separate from the
leaderboard because it has no independent-host replay or matched isolated wall
comparison.
