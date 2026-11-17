# Item 02: pricing binary curves at deployed sizes under the measured degree model

Assessed 2026-10-18 from the on-disk outputs in `raw/`, by the frozen rule in
[`PROTOCOL.md`](PROTOCOL.md). Every number below is **derived** (a closed-form
model evaluation); none is measured on a curve, and wall time is not a metric.

## Verdict

**ADVANCES (derived, model level), conditional on a constant-degree model.
Item-07 gate: OPEN by the frozen rule. Not a measured improvement.**

By the frozen rule, some design arm flips (gap to matched rho <= 0) at `K = 1`,
which needs no unmeasured constant: 100 design cells at `K = 1` have a
non-positive gap at a deployed `n` (31 at `n = 571`; both families counted).
The level is **exponent** (derived): the flipping arities are `m >= 4`, whose
free-oracle floors are `2n/5` and `n/3`, below rho's `n/2`.

Three readings that the rule's one word does not carry, stated here so the
verdict is not read as more than it is:

1. **The flip is carried by the constant-`D` arms (`D = 3, 4, 6, 7`), which are
   hypotheses at deployed `n`.** The measured degree evidence is F4 degree 4
   for `n <= 15`, `l <= 5` and XL 6 to 7 (`RESEARCH_ECC2K130_DESCENT_DEGREE.md`,
   `research/ic_next10_20261014/06-equivariant-f4-m4/README.md`), a toy range.
   The one arm that is the measured *growth* fit, `D = l + 4` at `m = 4`, **never
   flips**: gap `+105.1` bits at `n = 131` rising to `+336.7` at `n = 571`
   (Koblitz, `w = 1`, `K = 1`), and `D = ceil(l/2) + 4` (supplementary) is
   `+73` to `+384` bits over every deployed size, `m` and `w`. Under the measured fit this is a
   **loss at every deployed `n`**; the open gate is conditional on the degree
   staying bounded as `l` grows to 90 to 130, which nothing here measures.
2. **The `m = 3` route does not flip anywhere**: every `m = 3` arm that is not
   the free-oracle floor has gap `+14` to `+72` at `K = 1`; the floor at `m = 3`
   is `+3.8` to `+8.9`. The negative control (`m = 2`) never crosses.
3. **A derived crossing is not an improvement over the index-calculus baseline**
   in the owner's success sense (measured, verified). No claim of a speedup, win
   or break is made. Both sides are lower bounds (unpriced counters below), so
   no gap is a ranking.

Level: exponent (derived model). Gap to rho is given for every row below as the
distance remaining.

## Method

Closed-form evaluation of the committed model (PROTOCOL.md "Model"):
`total = log2(2^(targets + oracle) + 2^algebra)` with
`targets = l + max(0, log2 m! + n - m l)`, `oracle = w log2 C(N, <=D) - log2 K`,
`algebra = log2 m + 2 l`, `N = (m-2) n + m l`, minimised over integer
`l in 1..=n`, against the matched rho `1/2 (log2(pi/2) + log2 r - log2 A)`.
Cells: `n in {131, 163, 233, 283, 409, 571}`, two matched families with each
curve's real order and cofactor (Koblitz `A = 2n`, random `A = 2`),
`m in {2, 3, 4, 5}`, `w in {1, 2}`, `K in {1, 10, 16, 256}`, five degree models
plus the free-oracle floor, 2352 rows in `raw/cells.tsv`. Crossover scan:
`raw/scan.tsv`. Instrument: `examples/blindspots_02_deployed_binary_pricing.rs`
(std only, compiled with `rustc`; deviation 8), cards `card-01.yaml` and
`card-02.yaml`, receipts `raw/receipt-card-0{1,2}.json`.

## Controls (all passed; `raw/control.txt`, sha256 `96633aa8...`)

| control | result |
|---|---|
| positive A: `descent_degree_cost-output-20261016-scale.txt` | 30/30 rows identical |
| positive B: `06-equivariant-f4-m4/cost_d4-output.txt` | 16/16 rows identical |
| named rows | `m=4 D=6 w=1` -> 2^76.31, `m=3 D=6 w=1` -> 2^88.05, both as committed |
| rho reference ECC2K-130 | 60.8090, as committed (`A = 262`) |
| range check `4..70` vs `1..=131` | constant-`D` minimiser unchanged |
| negative control `m = 2` | minimum gap **+24.50** bits (sect131r1 floor `w=1 K=1 l=43`), never crosses |
| monotonicity (`w`, `K`, `D`) | 1392 checks, 0 violations |

Reused, not new: the `n = 131` Koblitz cells reproduce the committed ECC2K-130
tables (for example `m=4 D=6 w=1 K=1`: `+15.50` = 76.31 - 60.81).

## Numbers: gap to matched rho, log2 group operations (`w = 1`, `m = 4`)

`K = 1` is 1 GAE per column touch, `K = 10` is 0.1 GAE, the floor is 0 GAE
(AGENTS.md section 5a band). Negative is at or below rho.

Koblitz family (rho: 60.81, 77.15, 111.39, 136.25, 198.99, 279.75):

| arm | n=131 | 163 | 233 | 283 | 409 | 571 |
|---|--:|--:|--:|--:|--:|--:|
| floor (0 GAE) | -4.35 | -7.83 | -14.07 | -18.93 | -30.99 | -47.29 |
| D=4, K=1 | +7.62 | +4.89 | -0.92 | -5.38 | -16.70 | -31.67 |
| D=4, K=10 | +6.24 | +3.24 | -2.24 | -6.41 | -18.23 | -33.35 |
| D=6, K=1 | +15.50 | +9.94 | +4.87 | +0.44 | -10.58 | -25.38 |
| D=6, K=10 | +12.34 | +8.69 | +3.31 | -0.71 | -11.97 | -26.82 |
| D=7, K=1 | +21.30 | +15.17 | +7.33 | +3.49 | -7.52 | -22.10 |
| D=l+4, K=1 | +105.1 | +122.3 | +160.7 | +187.2 | +253.1 | +336.7 |

Random family (rho: 64.83, 80.83, 115.83, 140.83, 203.83, 284.83):

| arm | n=131 | 163 | 233 | 283 | 409 | 571 |
|---|--:|--:|--:|--:|--:|--:|
| floor (0 GAE) | -8.37 | -11.50 | -18.50 | -23.50 | -35.83 | -52.37 |
| D=4, K=1 | +3.61 | +1.22 | -5.35 | -9.96 | -21.54 | -36.75 |
| D=4, K=10 | +2.23 | -0.43 | -6.67 | -10.98 | -23.07 | -38.42 |
| D=6, K=1 | +11.49 | +6.26 | +0.44 | -4.13 | -15.42 | -30.46 |
| D=6, K=10 | +8.32 | +5.01 | -1.12 | -5.28 | -16.81 | -31.90 |
| D=7, K=1 | +17.28 | +11.49 | +2.90 | -1.08 | -12.36 | -27.18 |
| D=l+4, K=1 | +101.1 | +118.7 | +156.2 | +182.6 | +248.3 | +331.7 |

Other rows (full set in `raw/cells.md`): `m = 5` flips earlier and deeper
(Koblitz `D=4 w=1 K=1`: `-0.68` at 131, `-71.19` at 571); `w = 2` delays the
flip (Koblitz `D=6 m=4 w=2 K=1`: first flip at 571, gap `-3.37`; `D=7 w=2`
`+2.92` at 571, no flip at `K = 1`, `-0.07` at `K = 256` (n = 571)).
Intervals: the model is exact arithmetic on integers `l`; there is no sampling
interval. The `K` band moves a `m=4 w=1` gap by 1 to 6 bits (`K = 1` to
`K = 256`), less than the `D`-model spread (about 8 to 14 bits between
`D = 4` and `D = 7`), and the `D` constant versus `l + 4` spread (over 100 bits).

Where the sign is unresolved inside the GAE band: of 480 design cells (`(family,
n, model, m, w)`), 5 change sign between `K = 1` and `K = 10`
(Koblitz `n=283 D=6 m4 w1`: `+0.44` / `-0.71`; random `n=131 D=3 m4 w1`,
`n=163 D=4 m4 w1`, `n=233 D=6 m4 w1`, `n=233 D=4 m5 w2`), and 16 between
`K = 1` and `K = 256`. Those are reported as unresolved at that boundary, not
as wins. The floor flips (`m >= 4`) are the weight-0 end of the band.

Crossover `n` from the scan (`raw/scan.tsv`; first grid `n` with gap <= 0, `m=4`,
`w=1`, `K=1`; values above 571 are extrapolation of the closed form):

| model | Koblitz | random |
|---|--:|--:|
| floor | 86 | 64 (grid edge) |
| D=3 | 191 | 143 |
| D=4 | 223 | 175 |
| D=6 | 292 | 238 |
| D=7 | 324 | 270 |
| D=6, w=2 | 533 | 475 |
| D=7, w=2 | 605 (extrapolated) | 547 |
| D=l+4, D=ceil(l/2)+4 | none to 4096 grid (min gap +54 to +66 at n=64, `m=4 w=1`) | none |

`m = 3` has no crossing for any non-floor arm to 4096. Where the flipping
arms sit: at the best `l` of the `D=6 m=4 w=1` flip at `n = 571`,
`l = 126`, `N = 1646`, memory `2^128` words (rho: O(1), not priced); no flagged
cell (`l` at a search edge, targets above the subgroup, total above the
subgroup) is among the flips.

## Unit and pricing (AGENTS.md section 5a)

Unit: log2 of group operations (GAE); the oracle term is column touches
divided by `K`. Unpriced counters, not emitted by the model:

* attack arm: Macaulay-matrix construction and memory traffic, verification and
  root extraction of decomposed targets, factor-base membership tests;
* rho reference: orbit canonicalisation per step, partition hash,
  distinguished-point inserts and lookups.

Both sides are lower bounds, priced at 0 (floor), 0.1 (`K = 10`) and 1
(`K = 1`) GAE per oracle touch only; the other counters are not priced at all
and are the largest open gap in the comparison (the attack-side memory, `2^95` and `2^128` words for the `D=6 m=4 w=1`
flips at `n = 409` and `571`, is not charged as time).

## Deviations and caveats

* Deviations 1 to 9 of PROTOCOL.md stand; in particular the instrument is
  std-only and built with `rustc` because the `crypto` library does not compile
  at the branch base (25 errors in other lanes' code), and `conductor` is not
  installed (hooks warn). The broken library is not fixed here.
* **Relay check deviated, reported not repaired.** `python3 tools/task_card.py
  check-relay` returned `relay_verbatim: false` for both cards
  (`raw/relay-1.txt`, `raw/relay-2.txt`): card 1 reports the line
  `output .../cells.tsv: e99f40c9...` as missing, card 2 reports `status:
  completed` and `output .../scan.tsv: 12407af1...` as missing; `verify_findings`
  is empty for both. On disk, the sha256 of `cells.tsv`, `cells.md`,
  `control.txt`, `scan.tsv` and `scan.md` equal the digests the executors
  relayed, and both receipts say `completed`. The numbers above were read from
  the files, not from the relay.
* **Host L0.** The host is a shared 4-vCPU container with no isolation
  tier; irrelevant to a closed-form evaluation (no timing is used), and said so
  for completeness.
* **Toy scale.** The degree evidence behind `D = 4/6/7` is measured only to
  `n <= 15`, `l <= 5`; the deployed sizes need `l` of 90 to 140. Extrapolation
  to there is the model's assumption, not a measurement. The `n = 131` rows are
  reused results.
* The model prices only a column-count oracle and a `2l` linear algebra term. It
  gives the free-oracle floor a crossing already at `n = 86` (Koblitz), which
  says the model is optimistic for `m >= 4`; it is not evidence that rho loses.
* `K` (column touches per group operation) is not measured; `m = 5`'s extra
  decomposition probability is as the committed formula has it.
* Python is used only to plot recorded values (`plot_gap.py`), never to compute
  a result.

## Graph, PDF, evidence

* `gap_vs_n.svg`, `gap_vs_n.png` (source `plot_gap.py`, data `raw/cells.tsv`);
  `REPORT.pdf` (source `REPORT.typ`).
* Graphs checked and left unchanged: `docs/ic/leaderboard*` and
  `docs/index-calculus-scoreboard.html` (no measured, complete-solve or
  phase-speedup row is produced; a derived model crossing is not a leaderboard
  entry).
* Copies for `benchmarks/20261018-blindspots-deployed-binary-pricing/`; the
  `benchmarks/README.md` row is left to the integration step.
