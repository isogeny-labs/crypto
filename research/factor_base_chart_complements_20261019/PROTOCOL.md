# Frozen chart-complement screen — 2026-10-19

User request: “We need new leads.” Deliver a new bounded native structural
screen and actionable hypotheses, preserving the completed shape study and
its executable bindings. This is a new post-hoc diagnostic, not a rerun of
either draft backlog card or a held-out confirmation campaign.

## Inputs and provenance

Use exactly the 60 inventories indexed in `inputs/index.json`, copied without
changing their bytes from the primary, secondary, confirmation and
representation panels of the completed 2026-10-18 study. The source archive
SHA256 is recorded in the index. Retain the empty failed inventory and all
repeated configurations. Deduplicate nonempty inventories by exact curve slug
and sorted signed point set before screening. Do not count duplicates as
independent bases. `inputs/models.json` contains the exact four registry rows
from source 819d6b172c8a58983bf1b2463fe24576f3f58094. Preserve the original
ICV1, EC1, generator and subgroup metadata in the copied base files.

## Decided construction and limits

For each exact curve, measure every distinct nonempty singleton and every
unordered pair of those bases. A pair denotes the set union, with no trimming,
projection, point generation or new scalar-log search. Retain and report any
union exceeding 256 signed points as capped; do not partially enumerate it.
Repeated union point sets remain explicit cases, not independent replicates.

Enumerate every unordered pair of signed points, including repetition. Compute
the actual group sum. Classify identity separately; accept a nonidentity
subgroup sum only when direct multiplication by recorded prime r gives the
identity. For every unique point pair also compare that result with the
cofactor-image test [r]P + [r]Q = O. Check all points, signs, generator order,
group-order metadata and prime r. Reproduce every completed singleton's
previous independent exact coverage in `inputs/previous-coverage.json`.

For each union report the exact extra target points beyond the union of the
two singleton supports. Retain every target and every unordered witness pair.
Record column count, signed-point count, compatible pairs, cofactor-image
fibre occupancy, active/unused columns and active pair-graph components.

Columns identify inverse point pairs, with the lexicographically smaller
point assigned +1. For alternative witnesses of the SAME target, subtract
the first witness vector from each other vector and compute rank modulo r.
Report both full rank and rank after removing columns whose [h] projection
is zero, where group order is h*r. These are homogeneous collision relations
on projected factor-base points, not training rank, an acquired set of
factor-base logarithms, or a guarantee of usable end-to-end linear algebra.
No target logarithms are searched for or recovered.

## Execution

Native standalone Rust crate `screen/`; arithmetic is adapted from the
published independent audit, with explicit provenance in the report. Unit
tests, strict all-target Clippy and release build precede freezing this
protocol/source/input commit. Bind every input, both Rust source files,
Cargo manifest/lock, this protocol and the built executable in the director
card. Keep the executable outside Git and record its hash.

One mechanical command, no retries:

```
./research/factor_base_chart_complements_20261019/screen/screen screen research/factor_base_chart_complements_20261019 research/factor_base_chart_complements_20261019/runs/census.json
```

Timeout: 900 seconds. Output must not exist. stdout and stderr go to
`cards/screen.stdout` and `cards/screen.stderr`; receipt path
`cards/CARD-chart-complements-20261019-1.receipt.json`.
Card ID `CARD-chart-complements-20261019-1`; experiment ID
`factor_base_chart_complements_20261019`; bus address
`executor-mechanical-chart-complements`; background false. Write scope only
`research/factor_base_chart_complements_20261019/`. Stop on any nonzero exit;
success exit code only 0. Parent creates the empty ignored `runs/` directory
before dispatch. Director compiles/lints; parent relays unchanged to mechanical;
director assesses against the receipt. All original evidence remains read-only.

## Interpretation and selection, fixed before execution

Report every singleton and union, not only selected examples. For each curve
highlight (1) the union with most new cross targets; (2) the union with the
fewest columns attaining any new cross target, ties by more new targets;
(3) the highest useful same-target-difference rank, ties by fewer columns.
Break remaining ties by case ID. Compare sizes explicitly. Mark nested or
identical unions. These are selection heuristics on exhaustive finite support,
not held-out statistical tests or equal-cost comparisons.

The three leads being tested are complementary chart unions, fibre-aware
shape selection, and witness-diversity screening before online relation
collection. A two-chart solver must charge all chart-pair branches (up to four
ordered branches or three symmetry-reduced cases), all failures and setup;
this census implements no new F4 chart solver and measures no successful
solver work. Uniform random-target coverage on these finite fixtures is an
exact combinatorial count, not a practical sampling method or a speed claim.

## Acceptance and nonclaims

Execution is valid only with checked native tests, exact bindings, all-pair
scalar checks, prior coverage agreement and director assessment. Failed runs
remain records. Counts only: no wall/native timing, certified regularity,
refutation-degree improvement, statistical significance, asymptotic scaling,
83/130-bit transfer, full native pricing, rho comparison or end-to-end winner.
No new theorem is proposed; the familiar group-homomorphism criterion is
checked on every enumerated pair, not presented as a novel proved result.

Publish source-linked Markdown, quantitative SVG/PNG, a visually inspected
PDF, all cases and witnesses in a deterministic raw archive with member
hashes and verified extracted-tree repacking. Preserve old study archives,
protocols, draft cards, registry/alias/store containers and installed binaries.
No S3 or paid cloud operation. Literature context identifies related prior
art and does not assert global novelty. Root native inputs remain unchanged;
run the required release-library command before pushing, with failures retained.
