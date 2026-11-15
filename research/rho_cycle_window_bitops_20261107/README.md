# Rho cycle windows in bitop/v1, and the rho Ir/gate spread

**Report:** [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)).
**Run report:** [`reports/run-20261107T1947Z/`](reports/run-20261107T1947Z/).
**Class:** accounting. No algorithm changed and no walk takes a different step;
this is not an improvement of index calculus or of rho. Every figure below is an
unverified assertion until a verifier checks it [pcc:unverified].

## 1. The cycle windows are now charged

The default hyperelliptic rho reference
(`RhoVariant::DistinguishedPointsNegationFullWidthCollideFirst`, round fifteen)
keeps a 32-entry recent-hash window. Its work was not in the `bitop/v1` tally.
ecbench's tuned rho walker (`ic_boundary::TunedWalker`, its lanes, and Brent's
detector under `WalkPolicy::v2`) had the same gap with its 16-entry ring.

What is charged now (rules in `docs/units/BITOPS.md`, "2026-11-07"):

| where | work | primitive |
|---|---|---|
| `hyperelliptic_ic_bench::rho_dp`, active window | hash compares of the scan (newest first, to the match or all 32) | `word_op` each |
| same | the eviction length test of each push | `word_op` |
| same, escape step | RNG draw, branch index, two coefficient additions mod N (as the gap-limit escape) | `rng_word`, `word_op`, 2 `modr_add` |
| same, collide-first repeat | `solve_collision` and the `k·D₁ = D₂` check | moved from `oracle` to `linear_algebra` and `verification` (already counted; the phase was wrong) |
| `ic_boundary` tuned walk and lanes | ring scan (every entry), the push's two index tests, on a hit the re-scan, least-key search and repeat test | `cycle_window_word_ops`, one `word_op` each |
| same, `WalkPolicy::v2` | Brent's least-key update, checkpoint test, power test; repeat test on a detection | `brent_word_ops`, one `word_op` each |

Not charged, by the unit's rules: holding the window (memory), clearing it
(control), and the passive window of `DistinguishedPointsNegationFullWidth`,
an instrument the walk never consults.

The ecbench counters go into the search phase's `work_native`, never into
`counters`: committed rho records carry none, so they replay and price exactly
as before (`audit` compares `work_native` only when the committed record has
it). A rho record written before 2026-11-07 is therefore short of its window's
word operations, a lower bound on the walk it records. The GAE unit is
unchanged.

## 2. Re-pricing: which published ratios used the default walk

None. Searched: `research/hec_bitop_20261020/raw/*.jsonl` (all 18 rows
`"rho_variant": "DistinguishedPointsFullWidth"`, no window),
`docs/ic/leaderboard.json` (its hyperelliptic section sources that study),
rounds fourteen and fifteen (`research/notes/index-calculus/round1{4,5}`, rho
against rho in walk group operations, where each lap is already a counted
group operation), round sixteen (`research/hyperelliptic_ecbench_20261024`,
ecbench `rho.plain`/`rho.negation` in GAE, which the change leaves unchanged),
and every committed file naming the variant. No leaderboard or dashboard
figure changes, so neither was regenerated.

To size the change on the rows a default-walk reference would give, the rho
arm of all 18 hec_bitop rows was re-derived with
`examples/hec_rho_window_reprice.rs` (same instances, `k = N/3 + 7`, 40 seeds
from `0xC0FFEE`, counts only) and divided into the committed IC means.

- **Control:** with `--variant full-width` all 18 rho means equal the
  committed ones exactly (`raw/full-width.txt`, column `control`).
- Every run of every variant verified.
- "Window uncharged" is the charged total minus the window's own primitives
  (`RhoResult::window_charges`, priced at the row's widths): the walk is the
  same walk, so one run gives both.

Default walk, IC/rho in `bitop/v1` (committed IC means; `raw/collide-first.jsonl`):

| g | p | N | committed (full-width rho) | default walk, window uncharged | default walk, window charged | change | window share of rho |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 41 | 1321 | 0.2071 | 0.2100 | 0.2099 | −0.039% | 0.039% |
| 2 | 61 | 1399 | 0.2213 | 0.2236 | 0.2235 | −0.037% | 0.037% |
| 2 | 101 | 4663 | 0.2987 | 0.3059 | 0.3057 | −0.045% | 0.045% |
| 2 | 151 | 7949 | 0.4299 | 0.4490 | 0.4488 | −0.042% | 0.042% |
| 2 | 211 | 11813 | 0.5558 | 0.6088 | 0.6085 | −0.055% | 0.055% |
| 2 | 251 | 61667 | 0.5238 | 0.5294 | 0.5287 | −0.125% | 0.125% |
| 3 | 23 | 6299 | 0.1995 | 0.1999 | 0.1997 | −0.094% | 0.094% |
| 3 | 31 | 7333 | 0.2567 | 0.2705 | 0.2703 | −0.064% | 0.064% |
| 3 | 41 | 6679 | 0.3052 | 0.3184 | 0.3182 | −0.046% | 0.046% |
| 3 | 61 | 124459 | 0.3014 | 0.3316 | 0.3311 | −0.158% | 0.158% |
| 3 | 101 | 364747 | 0.3572 | 0.3939 | 0.3933 | −0.165% | 0.165% |
| 3 | 151 | 1180351 | 0.4309 | 0.5215 | 0.5207 | −0.154% | 0.154% |
| 3 | 211 | 4620611 | 0.4540 | 0.4278 | 0.4269 | −0.211% | 0.211% |
| 4 | 17 | 5639 | 0.7667 | 0.7901 | 0.7897 | −0.047% | 0.047% |
| 4 | 23 | 30223 | 0.5355 | 0.5627 | 0.5622 | −0.089% | 0.089% |
| 4 | 31 | 239753 | 0.3849 | 0.3899 | 0.3892 | −0.200% | 0.200% |
| 4 | 41 | 333041 | 0.4370 | 0.4713 | 0.4706 | −0.140% | 0.140% |
| 4 | 61 | 16790591 | 0.1144 | 0.14367 | 0.14327 | −0.277% | 0.277% |

The window costs 18–31 word operations per walk group operation, but a
genus-2 to genus-4 Jacobian step costs 10⁵–10⁶ gates, so the window is
0.04–0.28% of the walk and moves no ratio past its third decimal. The share
grows with `N` because the scan runs every step while the fixed branch
precomputation does not.

Beside the question asked, and not acted on here: the default walk is
cheaper than the full-width walk the published rows used on most rows (up to
20% at genus 4, p = 61), so against it the published 0.114 for that row reads
0.143. The published row is pre-gate and names its reference; changing its
reference is a protocol decision, not an accounting fix.
`cycle-escape.jsonl` and `negation-full-width.jsonl` (passive window, charges
zero) are recorded for completeness.

## 3. The rho Ir/gate spread of ic-acct-7

`research/ic_acct7_symmetrised_20261021` reported 1.395 and 0.442 Callgrind
instructions per counted gate for two `rho.signed_frobenius_full` records
(seq 9 and 11) against a steady 0.30–0.32 for index calculus.

Hypothesis (fixed per-solve work not in `bitop/v1` dominates short walks):
**holds**.

- From the committed audit-ir records (session binary `bf8196c2`, two points):
  `Ir = 2.335e6 + 0.0869·gates`.
- A local ladder: `ecbench audit-ir` on all 12 rho records of that spec with
  the existing binary `4ff2214e…` (the binary the study's repricing used),
  Callgrind only. `Ir = 1.441e6 (±1.9e3) + 0.08704 (±0.0007)·gates`,
  R² = 0.9993, n = 12 (`ir_spread/fit-ladder.json`, `examples/ir_gate_fit.rs`).
- `callgrind_annotate --inclusive=yes` on the parts names the fixed work, the
  same in every record (`ir_spread/fixed-parts.jsonl`):
  - `SignedFrobeniusClasses::new`, **≈ 605 300 Ir**:
    - `FrobeniusPowers::new`, 493 698;
    - `FrobeniusCanon::new`, ≈ 110 800.

    The per-solve build of the Frobenius power maps and canonicalisation
    tables. `bitop/v1` prices `canon` and `as_solve` assuming these tables
    exist, and nothing charges building them.
  - `full_price::reprice`, **≈ 778 000–786 000 Ir**. The `*_full` method
    re-prices its own report inside the bracketed solve: it parses
    `full_prices.json` (serde_json) and hashes it (SHA-256). This is harness
    work, not algorithm work.
- Removing those two parts leaves `Ir = 5.7e4 + 0.0859·gates` (R² = 0.9997).
  Per-record Ir per gate falls from 0.31–1.22 to 0.095–0.133.

**Recorded as known constants, not charged.** The re-pricing is
instrumentation and should leave the bracket, or be subtracted by `audit-ir`.
The classes build is real per-curve set-up that `bitop/v1` omits for every
Frobenius-folded rho (about 6e5 Ir at n = 17). Charging it needs a counter for
the table builds and a rule. That changes every Koblitz rho record's price, so
it is left as the next step and not folded into this accounting change.

What remains open: the marginal rate, 0.086 Ir per gate for this rho, sits
below the index-calculus arm's 0.30–0.32. Per counted gate, the IC arm runs
about 3.5× the instructions the rho walk runs. That is the comparison the
instruction audit is for. Either the IC phases still do work their counters
miss, or the binary-field `canon`/`as_solve` weights over-price the rho walk's
table lookups. Two IC records do not separate the two.

## Files

- `raw/`: re-derived rho rows per variant (`*.jsonl`, `*.txt`), example
  binary hash, base commit.
- `ir_spread/`: spec (from `origin/claude/ic-acct-7`), ladder audit-ir lines,
  fixed-part attribution, annotate excerpts, fits, binary hash.
- `figures/`: `ratio_and_window_share.svg`, `ir_vs_gates.svg`, PNG renders,
  `plots.py` (plots recorded values only).

## Limits

- Counts and Callgrind only, on a shared L0 host. No wall time is used. No AWS
  identity on this host (`aws` is not installed), so nothing ran on isolab.
- The re-pricing is descriptive. It promotes no row and changes no published
  figure. No `VALIDATION.json`, so there is no leaderboard or `benchmarks/`
  row.
- The resume checkpoint could not be pushed: no upload token on this host.
