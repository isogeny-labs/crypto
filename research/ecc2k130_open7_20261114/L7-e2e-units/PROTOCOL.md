# L7 protocol: end-to-end unit coverage of binary / Koblitz index calculus (frozen before the table)

Lane L7 of `research/ecc2k130_open7_20261114/` (rules: `../COMMON.md`). Frozen 2026-11-14
before any number of the consolidated table was computed. Amendments are appended, dated,
never edited in place. Every statement this lane makes is `[pcc:unverified]`.

## 0. Item and residual question

Owner's item: "papers report solver seconds at toy n; none normalise to (cost per relation x
relations needed + linear algebra) against rho group operations, with large-prime variations
included."

The lab's harness (ecbench `S = GAE/sqrt(r)`, matched rho, AGENTS.md §5a band, validation
gate) exists for exactly that normalisation, so the item is read as a question about this
lab's **coverage**: which complete (DLP solved and verified) binary/Koblitz IC rows exist in
that normalisation, with which matched rho, at which band weights, validated or not, with or
without large primes, and what they jointly say about scaling and the dominant phase.

Residual question (to be answered or declared closed): after prior work, is there (a) a
consolidated, single-table view of all complete binary/Koblitz IC rows in counted units
against matched rho across the band; (b) a fitted exponent pair (IC vs rho) with an honest
crossing estimate; (c) a measured large-prime row; (d) a phase decomposition of the
nearest-to-rho arm? Any of (a)-(d) that already exists is cited `repo:path`, not recomputed.

## 1. Sources (closed list, fixed now)

Rows are read from committed JSON/JSONL only, never retyped from prose where a JSON exists.

| id | path (`crypto:`) | what is read |
|:--|:--|:--|
| LB | `docs/ic/leaderboard.json` `.board[]` | families A (Koblitz one-target, rule v9, timed units), B (char2 random binary, GAE), C (Koblitz 32-target batch, timed); `validation.status` |
| FC | `research/ic_frontier_campaign_20261015/analysis/band-koblitz.json`, `phase1/sessions/K*/records.jsonl` | Koblitz n=13..47, arms ic-compact, ic-negfold, ic-f4, ic-shared; rho-frob, rho-strong; band 0/0.1/1; per-phase GAE from records |
| FO | `research/ecbench_frobenius_orbit_vs_rho_20261014/` (sessions `*/records.jsonl`, comparisons) | Koblitz n=13..41 Frobenius-orbit IC, full-priced reading |
| LZ | `research/ecbench_lazy_orbit_members_20261018/figures/summary.json`, `figures/ratios.json` | Koblitz n=13..41, eager and lazy orbit IC; rho-frob, rho-strong; phase means |
| DT | `research/ic_koblitz_dlp_tuning_20261023/bands/holdout-n*.band.json` | Koblitz n=37,41,43,47: base-frozen (ic-compact) and dlp-selected (double large prime); rho-frob, rho-strong; band |
| DR | `research/ic_koblitz_descent_routing_20261026/analysis/band-k*.json` | Koblitz n=41,47,53: pipeline, descent; rho |
| PX | `research/ic_planner_exhaustive_20261026/analysis/summary.json` | Koblitz K-k41..k61 and random-binary B-hb19..hb27: pipeline / plan arms, descent; rho; band |
| FB | `research/fes_then_batched_20261112/results.json` | char2 n=13..23 and Koblitz n=23 algebraic-descent arms; matched rho |
| GK | `research/gaudry_k4_char2_crossover_20261015/raw/summary.json` | k=4 char-2 (composite degree 4m, subfield-dependent) IC vs rho, and Part F large primes |
| SY | `research/koblitz_symmetrised_e2e_20260927/results/*.json` | torsion-symmetrised arms n=17,23 (supplementary) |

Validation status per row: `validated` only where `python3 scripts/validation_gate.py evidence
<dir>` prints `valid` for the row's evidence directory (or the leaderboard's own
`validation.status` says `validated`); otherwise `unvalidated (pre-gate)` for directories dated
before 2026-10-19 and `unvalidated (post-gate research record)` after.

Inclusion: an arm-on-curve row is included when every run of the arm recovered and verified the
logarithm and a rho arm was run in the same session/process (or, for LB, the board's matched
reference). Arms with incomplete runs (e.g. ic-shared 2/32) or uncounted unpriced work are
listed as excluded with the reason. Stage-only diagnostics and models are excluded.

## 2. Columns extracted

n, curve slug, log2 r, regime, method/arm, base (columns / points), m, relations, trials per
relation, cost per relation (relation-phase GAE / relations), LA GAE (or row ops), total S (the
source's unit), total cost `S*sqrt(r)` and its log2, matched rho S and kind (signed Frobenius
+ negation, A = 2n, or negation, A = 2), ratio IC/rho at w = 0, 0.1, 1 where the source
supplies a band, validation status, source path. Fields a source does not carry are left null
and the missing field is named; nothing is imputed.

Band: where a source supplies a committed band file, it is used as is and labelled with its
reading date (pre-2026-11-01 byte pricing; pre-2026-11-08 table-work pricing). Where records are
in Git and the native `ecbench` builds within the lane budget, `ecbench band` is re-run on the
committed sessions for the current reading and both readings are reported. Timed sources
(LB family A, C) have one figure (both arms in one process: nothing unpriced), stated as such.

## 3. Fits and crossing (task 2)

- Series: every IC arm with >= 4 sizes in one source, and that source's matched rho.
- Model: OLS of log2(total cost) on log2 r (primary: rho's generic axis; r << 2^n on these toy
  curves) and on n (secondary, as asked), with t-based 95% intervals on slope and intercept.
- Crossing: solve log2 C_IC(x) = log2 C_rho(x); interval from 10,000 parametric draws of both
  fits (coefficients from their OLS covariance, t-scaled), reported as median and 2.5-97.5%
  percentiles, with the fraction of draws that cross at all at x > 0 and before x = 129
  (log2 r of ECC2K-130's prime subgroup) / n = 131.
- Also fit log2(IC/rho ratio) on log2 r directly (paired by size), since the ratio's slope is
  the quantity that decides a crossing.
- Pre-registered prediction: for every Koblitz arm on prime n the IC slope >= rho slope (no
  crossing before log2 r = 129, or a crossing interval that is uninformative), with the ratio
  slope positive (about +0.1 to +0.17 per ln r, as FO and LB already report). The k=4 char-2
  (GK) series is the exception the repo already reports (a crossing near m ~ 17), and is
  subfield-dependent (AGENTS.md §8b), so it does not transfer to n = 131.
- Falsifier: a Koblitz prime-n arm whose ratio slope interval lies wholly below 0.

## 4. Large primes (task 3)

List every complete binary row with a 1LP/2LP variant (expected: DT double-LP n=37..47, GK
Part F m=10..14). If such rows exist, report their measured effect against the same-session
baseline and do **not** add a model projection as if it were missing data. A standard 1LP/2LP
yield model is given only as a labelled projection of where the variant could start to pay
(LA share reaching the relation share), using measured phase shares.

## 5. Dominant component (task 4)

For the nearest-to-rho Koblitz arm(s) (lowest IC/rho at w = 0 among complete prime-n rows), the
per-phase GAE share (factor base, oracle setup/pair table, relations, linear algebra, verify)
read from committed records or the source's phase means. Prediction: relation search plus
oracle setup > 90% of S; linear algebra < 2%.

## 6. Decision rule for the item

- **CLOSED-BY-PRIOR**: (a)-(d) of §0 each have a committed source, the consolidated table is
  only a re-reading (this lane adds no new measurement).
- **PARTIALLY OPEN**: some of (a)-(d) exist; the gaps are named as residual items with the
  cheapest experiment that would close each.
- **OPEN**: none exist.
This lane is analysis; COMMON.md's IMPROVES / NO-CHANGE / NEGATIVE / INCONCLUSIVE apply only
to a measured run, and none is planned. A re-run is made only if a needed number is missing and
costs seconds to minutes locally in counted units (§2 band recompute is the only one foreseen).

## 7. Controls

- Positive control of the extraction: recompute from records the FC band at w = 0 and w = 1 for
  ic-compact and rho-frob at every size and require agreement with `band-koblitz.json` within
  1e-9 relative (the instrument reads what ecbench read).
- Negative control: the FC A/A arm (`rho-strong-aa`) must give ratio 1.000 to `rho-strong`.
- Determinism: the table builder run twice gives byte-identical `results.json`.

## 8. Language

AGENTS.md "no Python": the analysis tool is Rust (`tool/`), shell and `jq` only for thin
orchestration; the native `ecbench band` for any band recompute.

## Amendment 1 (2026-11-14, after the band re-read; appended, nothing above edited)

- Positive control, as observed: the native `ecbench band` re-read of the committed FC sessions
  reproduces the frozen `analysis/band-koblitz.json` at w = 0 exactly (relative difference 0 on all
  16 IC cells). At w > 0 it differs only on ic-compact. The cause is the 2026-11-01 rule that leaves
  `factor_base_vec_bytes_floor_uncharged` (a byte counter) unpriced. For example, n = 37 w = 1 reads
  6.394 now and 8.266 frozen. The `--no-price-table-work` reading is identical to the default reading
  on every FC cell, so table work does not enter. The §7 clause "w = 1 within 1e-9" was wrong for a
  file written before 2026-11-01. The control is read as "w = 0 exact, w > 0 differences explained by
  the dated rule".
- Reports: the harness refuses Markdown report files from this subagent, so the RESULTS/REPORT/
  SUMMARY narrative is returned in the lane's final message, for the main session to file.
  Machine outputs (`results.json`, `table.csv`, SVGs, `raw/`) are in this directory.
