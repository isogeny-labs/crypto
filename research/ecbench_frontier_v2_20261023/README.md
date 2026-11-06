# The `docs/bounds` frontier under procedure 2 (2026-10-23)

**Result.** A scoreboard change, not a measurement: no session was run. The
bounds frontier (`docs/bounds/FRONTIER.md`, `frontier.json`) is rebuilt under
frontier procedure 2 from fit-version-2 bounds, because the review in
`DEEP_DIVE.md` §1.3 found the v1 page published more than its data
supported. The rules are `docs/bounds/README.md` §3a and §5. The full report,
with both graphs, is [`report.pdf`](report.pdf) (source
[`report.typ`](report.typ)).

**Nothing published was replaced.** All 46 records on `main` are byte for byte
unchanged and still re-derive under fit version 1 (`ecbench bound check`), so
every id cited by challenges, the IC leaderboard and research notes still
resolves. Each seed fit gained a fit-v2 twin `<name>-v2.json`, and the two
epoch-1 challenge sessions' arms were fitted as `session-*-v2.json`
(`docs/bounds/refit.sh`). The v2 frontier ranks the 48 v2 bounds. The v1
records are listed under each table as historical, not ranked.

## What changed, per domain

| domain | v1 page | v2 page |
|:--|:--|:--|
| prime toy | 8 on the frontier of 17; ops leader `bsgs.negation` (calibration, 1.100 [0.978, 1.225]) named on point estimates | 10 on the frontier of 18 ranked; **no ops leader**: `bsgs.negation` and `rho.negation` (1.453 [1.184, 1.772]) overlap, tie set of six bounds; `rho.plain` is no longer dominated by `rho.negation`, because the order flips at 0.34 to 0.55 GAE per unpriced canonicalisation, inside the §5a band |
| Koblitz toy | 3 on the frontier, all `rho.signed_frobenius`; leader a three-size tier-subset bound (degrees 17, 19, 23) compared with bounds fitted on other curves | three-size and tier-subset bounds not ranked; 5 on the frontier of 11 on the four shared degrees (`rho.signed_frobenius` ×2, `bsgs.negation`, `rho.negation`, `rho.plain`); **no ops leader** (tie set of 3); both `rho.signed_frobenius` bounds have a large tail (max/median 54.2 and 11.7) |
| Koblitz medium | 2 on the frontier of 10; ops leader strong rho, memory leader a three-degree `rho.signed_frobenius` bound | 2 on the frontier of 4 ranked; ops leader strong rho (`pair-claw-koblitz-rho-strong-v2`, 1.089 [0.927, 1.261]), now named because its intervals separate on all six shared degrees; memory leader the pair claw at the cryptanalysis#175 shape (large tail, max/median 10.3) |

The exponent intervals no longer carry degenerate ends. The v1 intervals
ending at -1.0, 0.0 and 2.5, among them `kangaroo.vow`'s [0.179, 2.500] on
which "α agrees with declared" rested, came from one-size resamples. The v2
twins read, for example, [0.401, 0.724] for that bound, and three-size bounds
have no interval at all. The full before/after is figure 2 of the report.

## Files

| file | what |
|:--|:--|
| [`report.pdf`](report.pdf), [`report.typ`](report.typ) | the report (Typst; tables read the JSON below at compile time) |
| [`ops-status.svg`](ops-status.svg) | ops ratio to the floor with intervals for every v2 bound, per domain, coloured by v2 status and labelled with the v1 status |
| [`alpha-intervals.svg`](alpha-intervals.svg) | α intervals, published v1 record against its v2 twin |
| [`plot_frontier.py`](plot_frontier.py) | plots the recorded values; computes nothing |
| [`extract.sh`](extract.sh) | `jq` field copies from `docs/bounds/records/*.json`, `docs/bounds/frontier.json` and the v1 frontier at `3a1ea541d` |
| `records.json`, `frontier-v1.json`, `frontier-v2.json` | those copies |

To rebuild from the repository root:
`git show 3a1ea541d:docs/bounds/frontier.json > /tmp/f1.json && sh research/ecbench_frontier_v2_20261023/extract.sh /tmp/f1.json`,
then in this directory run `python3 plot_frontier.py` and
`python3 -c "import typst; typst.compile('report.typ', output='report.pdf')"`
(Typst and matplotlib from a scratch virtualenv).

## Graphs checked

- `docs/bounds/FRONTIER.md` and `frontier.json`: regenerated.
- `research/ecbench_frontier_20261013/` is a dated synthesis that cites v1
  ids, which still exist; left unchanged.
- No other consumer of `docs/bounds/frontier.json` exists under `docs/`.
- `docs/isobench/` has its own frontier, not built from these records;
  unchanged.
- No `benchmarks/` entry: this is not a speed result.

## Scope

Toy and medium tiers, planted targets, `ecbench.gae`. Nothing here is a
scaling result or says anything about cryptographic sizes.
