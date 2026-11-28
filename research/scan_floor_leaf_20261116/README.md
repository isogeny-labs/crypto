# Scan-floor leaf pilot: leads B ("dual leaf") and C ("y-free t") against a port of lin21-ht

Local pilot, 2026-11-16, shared 4-vCPU container, single thread. Every number
below is `[pcc:unverified]`. No benchmark record, leaderboard or
`benchmarks/README.md` row follows from this; wall times are indicative.
Protocol frozen before data: [`PROTOCOL.md`](PROTOCOL.md), commit `d2a43e435e`.
All computation, checks, statistics and tables are Rust: [`scan_leaf.rs`](scan_leaf.rs)
(one file, `rustc -O`, no crates). Generated outputs: [`breakdown.json`](breakdown.json),
[`breakdown_tables.md`](breakdown_tables.md), [`results.json`](results.json), [`tables.md`](tables.md).

## The baseline is a standalone port, not the repository arm

`cargo build --release --example ic_algebraic_floor` (tried once) stopped with
`error[E0308]` at `src/cryptanalysis/koblitz_index_calculus.rs:1646`
(`lift_x_packed` returns `Vec::new()` where a tuple is declared) at HEAD
`5f0dbd38f1`; the library does not compile, so `lin21-ht` itself was not run.
The baseline here is a line-by-line port of its leaf (`decide`,
`pair_half_trace`, `split_root_and_verify`, `verify`, `solve_f2`, blocks of 256,
batched addition then batched inversion of `x(Q)`) on its own one-word
`F_2[z]/(f)` arithmetic with `pclmulqdq`. Every ratio below is against that
port, and says nothing measured about the repository arm. Differences from the
repository arm: curve `y^2+xy=x^3+x^2+1`, `f = z^n + (smallest low part)`,
targets `[2]P` for seeded random `P`, and all arms return every decomposition
instead of stopping at the first.

## Result in one paragraph

- **B fails** under the frozen rule. B1 (the dual leaf as proposed: rows
  `T(z^j/t)` over a basis of `V^perp`, span test for `tau`) costs more than the
  baseline leaf: hw-priced counted operations baseline/B1 = 0.501, 0.443, 0.423
  at n = 35, 41, 47, wall baseline/B1 = 0.529, 0.495, 0.475. B2 (the left-kernel
  verdict on the baseline's own half-trace system, i.e. elimination without
  combination tracking, tracked solve on the 11.7% of leaves that pass) gives
  1.050, 1.056, 1.062 counted and 1.093, 1.060, 1.061 wall: below the 1.1 fail line.
- **C is inconclusive** under the frozen rule. `linear21.rs` does not already do
  it (it uses stored `y`, a full batched affine addition and a second batched
  inversion). On the t stage: hw-priced counted baseline/C = 1.292, 1.273, 1.276
  at n = 35, 41, 47 (threshold 1.3, fail line 1.1); t-stage wall baseline/C =
  2.329, 2.424, 2.282, outside the A/A band at all three. Whole leaf: counted
  1.045, 1.036, 1.031; wall 1.200, 1.171, 1.154.
- All correctness gates pass on all five cells (section "Gates").

## Baseline stage breakdown (reported before B and C were run; `breakdown_tables.md`)

Per leaf of the baseline port, "no" class, hw price (mul 8, sqr 8, inv 8(n+8), lookup 1, wop 1):

| n | l | leaves/target | ns/leaf | hw ops/leaf | t stage | half-trace elimination | root split | membership | wall: t stage | wall: elimination (build + solve) | wall: split + membership |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 23 | 7 | 107 | 227.6 | 315.0 | 28.3% | 69.4% | 2.1% | 0.1% | 31.2% | 67.7% | 1.0% |
| 29 | 9 | 517 | 279.7 | 381.9 | 23.1% | 74.8% | 2.0% | 0.1% | 24.0% | 72.4% | 3.6% |
| 35 | 11 | 2009 | 348.9 | 457.4 | 19.1% | 78.9% | 1.8% | 0.1% | 19.4% | 78.6% | 2.0% |
| 41 | 13 | 8193 | 420.1 | 541.6 | 16.2% | 82.0% | 1.7% | 0.1% | 16.6% | 81.1% | 2.3% |
| 47 | 15 | 32469 | 508.2 | 629.8 | 14.0% | 84.5% | 1.5% | 0.1% | 14.1% | 82.5% | 3.4% |

Amdahl at n = 47: the t stage is 14.0% of the leaf, so C can change the whole
leaf by at most 1/(1-0.140) = 1.16; the elimination stage is 84.5%, so B has
room (ceiling 6.5). Both are above the 10% stop line, so both were run. In the
full run the wall split of the elimination at n = 47 is 18.7% system build and
63.2% solve (`results.json`, `baseline_wall_stage_share`); root split and
membership together are under 4%: nothing acting on them can matter.
Per leaf at n = 47 the baseline executes 25.3 mul, 3.1 sqr, 0.008 inv, 7.6
lookups and 391 word operations; 11.7% of leaves have a consistent system.

## Counted operations per target (baseline / arm, "no" class)

| n | arm | mul | sqr | inv | lookup | wop | hw total | base/arm hw | clmul-min | bit-serial |
|---|---|---|---|---|---|---|---|---|---|---|
| 35 | base | 42765 | 6277 | 16 | 12794 | 508199 | 918835 | 1.000 | 1.000 | 1.000 |
| 35 | B1 | 19702 | 2729 | 16 | 224914 | 1425507 | 1835373 | 0.501 | 0.390 | 1.192 |
| 35 | B2 | 42765 | 6277 | 16 | 12794 | 464670 | 875306 | 1.050 | 1.069 | 1.012 |
| 35 | C | 36760 | 7281 | 11 | 18818 | 504180 | 879112 | 1.045 | 1.021 | 1.118 |
| 41 | base | 190807 | 25605 | 66 | 62500 | 2617284 | 4436955 | 1.000 | 1.000 | 1.000 |
| 41 | B1 | 82355 | 11145 | 66 | 1297228 | 7939260 | 10010359 | 0.443 | 0.350 | 1.175 |
| 41 | B2 | 190807 | 25605 | 66 | 62500 | 2381930 | 4201600 | 1.056 | 1.076 | 1.012 |
| 41 | C | 166328 | 29701 | 35 | 91172 | 2609089 | 4282216 | 1.036 | 1.014 | 1.110 |
| 47 | base | 821103 | 101462 | 254 | 247526 | 12709736 | 20449541 | 1.000 | 1.000 | 1.000 |
| 47 | B1 | 333887 | 44142 | 254 | 5919987 | 39233778 | 48289755 | 0.423 | 0.340 | 1.204 |
| 47 | B2 | 821103 | 101462 | 254 | 247526 | 11508498 | 19248303 | 1.062 | 1.082 | 1.013 |
| 47 | C | 724075 | 117696 | 130 | 361164 | 12677265 | 19829796 | 1.031 | 1.012 | 1.102 |

n = 23 and 29 are in `tables.md` (B1 0.705, 0.583; B2 1.035, 1.042; C 1.058, 1.056 at hw).
The last two columns are the pre-registered sensitivity prices and decide nothing.
B1 trades 16 field multiplications per leaf for 2l byte-table maps and a
(2l-1)-row elimination; only with a bit-serial multiplier price does its counted
ratio exceed 1 (1.18 to 1.24), and it stays under 1.5 there too.

## Wall time, baseline A1 / arm (sum over the "no" class, min of 3, paired bootstrap 95%)

| n | A/A band (A1/A2) | B1 | B2 | C, whole leaf |
|---|---|---|---|---|
| 23 | 1.117 [1.106, 1.126] | 0.682 [0.678, 0.686] | 1.078 [1.072, 1.083] | 1.174 [1.168, 1.180] |
| 29 | 1.028 [1.018, 1.038] | 0.592 [0.585, 0.599] | 1.074 [1.064, 1.084] | 1.182 [1.169, 1.195] |
| 35 | 1.012 [1.006, 1.018] | 0.529 [0.527, 0.532] | 1.093 [1.088, 1.099] | 1.200 [1.193, 1.208] |
| 41 | 1.005 [1.002, 1.007] | 0.495 [0.493, 0.497] | 1.060 [1.055, 1.064] | 1.171 [1.165, 1.177] |
| 47 | 1.009 [1.005, 1.012] | 0.475 [0.474, 0.477] | 1.061 [1.057, 1.065] | 1.154 [1.149, 1.158] |

The A/A arm is not centred on 1: the same code timed in two rotation slots
differs by 12% at n = 23 (a 25 µs scan) and by 0.5 to 1.2% at n >= 35. Read
every wall ratio at n = 23 and 29 with that in mind.

## C on the t stage

| n | hw ops/leaf, base | hw ops/leaf, C | counted base/C (hw) | clmul-min | bit-serial | wall A/A (L1a/L1b) | wall L1a/C1 | whole-leaf counted (hw) |
|---|---|---|---|---|---|---|---|---|
| 23 | 89.2 | 71.9 | 1.241 | 1.229 | 1.311 | 1.016 [1.012, 1.020] | 1.552 [1.541, 1.563] | 1.058 |
| 29 | 88.2 | 67.8 | 1.301 | 1.240 | 1.431 | 1.022 [1.006, 1.034] | 1.794 [1.772, 1.816] | 1.056 |
| 35 | 87.5 | 67.8 | 1.292 | 1.199 | 1.459 | 1.000 [0.991, 1.011] | 2.329 [2.294, 2.365] | 1.045 |
| 41 | 88.0 | 69.1 | 1.273 | 1.155 | 1.471 | 1.009 [1.004, 1.015] | 2.424 [2.399, 2.448] | 1.036 |
| 47 | 88.3 | 69.2 | 1.276 | 1.157 | 1.477 | 1.033 [1.029, 1.037] | 2.282 [2.263, 2.301] | 1.031 |

C per leaf: 5 mul + 1.5 sqr + half of one half-trace table map, against
8 mul + 1 sqr for the baseline's addition and second inversion. The t-stage
wall ratio (about 2.3) is far above the counted ratio (about 1.28): the counter
scheme does not price what the wall clock sees there (two serial Montgomery
chains and 24-byte point records in the baseline against one chain and two
word arrays in C). At n = 47 the whole-leaf wall difference A1 - C is 2.20 ms
per target while the t-stage difference L1a - C1 is 1.32 ms, so about 0.9 ms of
the whole-leaf wall ratio is not explained by the t stage alone (layout or
code generation of the leaf loop); it was not investigated.

## Decisions under the frozen rules (from `tables.md`, section 5)

- **B (B1, primary): fails.** Counted ratio 0.423 at n = 47 (< 1.1). `[pcc:unverified]`
- **B2 (secondary): fails.** Counted ratio 1.062 at n = 47 (< 1.1); its wall ratio
  is outside the A/A band (1.06 to 1.09) but the rule is on counted operations. `[pcc:unverified]`
- **C: inconclusive.** t-stage counted ratio 1.292, 1.273, 1.276 at the three largest
  n: under the 1.3 threshold at all three, above the 1.1 fail line; t-stage wall
  ratio outside the A/A band at all three. Whole-leaf effect: counted 1.03 to 1.05,
  wall 1.15 to 1.20, capped by Amdahl at 1.16 to 1.24 for a stage of 14 to 19%. `[pcc:unverified]`

## Gates (all pass; counts per cell in `tables.md`)

- every arm (B1, B2, C, and each timed build) returned the same set of
  decompositions as the baseline on all 742 targets (640 "no", 22 natural yes, 80 planted);
- n = 23, 29, 35: that set equals exhaustive pair enumeration on all 443 targets;
- all 80 planted triples found;
- per-leaf verdicts of B1 and B2 equal the baseline's "consistent" flag on every
  leaf of every target; C's multiset of `(t, 1/t)` equals the baseline's on every target.

## Deviations and what was not covered

1. Repository `lin21-ht` did not build; baseline is the standalone port (stated in the protocol before data).
2. One off-protocol debugging run after the protocol commit and before the measured
   runs: cells 17:5, 19:6, 21:6, 24 + 6 targets, seed 7, to exercise the gates. Its
   output included ratio lines, which I saw (B1 0.72 to 0.82, B2 1.01 to 1.04,
   C t-stage counted 1.09 to 1.20). No source change followed: the measured runs
   used `scan_leaf.rs` sha256 `816d0831…c9196e`, the only version ever compiled.
3. A/A is biased at the two smallest cells (above).
4. Wall breakdown gives split + membership together, not separately (counted
   operations give them separately).
5. The counter scheme omits the per-leaf zeroing of the pivot array (1 KB in the
   tracked solve, 512 B in the untracked one) and sequential loads of constants;
   the same omission in every arm.
6. Not covered: the repository's 4-lane batched inversion and its `add_many`
   (the port uses a plain Montgomery chain, which may overstate the baseline's
   t-stage wall time and hence C's wall ratio); other curves, `b != 1`, even
   `n`, `l` other than `(n-2)/3`, n > 47; a table-driven build of the leaf
   system (the system matrix is linear in `t`), which this pilot did not test;
   Koblitz prime-subgroup targets; any end-to-end relation or ECDLP cost.
7. `research-kb` and the lead atlas were not queried in this pilot; no novelty statement is made.
8. No figure was made.

## Arms run

`base` (A1, A2), `B1`, `B2`, `C`, stage arms `L1a`, `L1b`, `L2`, `L3`, `C1`, on
cells 23:7, 29:9, 35:11, 41:13, 47:15: one `breakdown` run (44 s) and one
`full` run (110 s). Plus the debugging run in deviation 2. Nothing else.

## Rebuild and rerun the smallest cell (about 2 s)

```sh
cd research/scan_floor_leaf_20261116 && D=$(mktemp -d) && rustc --edition 2021 -O -C target-cpu=native -o $D/scan_leaf scan_leaf.rs && $D/scan_leaf --cells 23:7 --out $D
```

Needs an x86-64 host with `pclmulqdq`. Counted rows reproduce exactly (checked:
the rebuilt binary had the same sha256 `21056edc…04e102` and printed the same
n = 23 counted rows); wall rows vary. All cells: drop `--cells 23:7`
(`--mode breakdown` for the baseline-only run).
