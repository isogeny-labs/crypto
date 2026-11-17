# Does clearing the cofactor from the IC factor base lower counted cost? (pre-registration)

Written 2026-11-03T02:05Z, before any data from the treatment arm exists.
This is a hypothesis test, not a result. Nothing here claims a speedup.

## 1. What was found before designing anything (literature first)

Sources checked, and what they changed:

| source | read how | what it says | effect on this study |
|:--|:--|:--|:--|
| `src/cryptanalysis/ic_framework/plugins.rs`, `PrimeProjectedAbscissaBase` (registered as `prime-projected-abscissa`, tests `projected_prime_tests`) | read at source | scans abscissae `x = 1..scan_cap`, takes `Q = [h]P`, drops identity and duplicate abscissae, keeps `±Q`, stops at `size` columns; checks `[r]Q = O` | **This already is "clearing the cofactor from the factor base" for prime curves.** No new code option is needed. The task's step 2 (add `clear=1`) is therefore replaced by using this existing factor base. |
| `src/cryptanalysis/ic_boundary.rs` `project_factor_base`, `BaseArm::Projected` (LEAD-19 arm B) | read at source | the same projection for the binary two-summand rows | same lever, other family |
| `research/pdp_literature_leads_20261022/leads/LEAD-19.md` | first ~80 lines | desk estimates for the binary ladder; predicts trials per relation fall by about `h` | the mechanism and its expected size |
| lead atlas `LEAD-ICB-cofactor-projected-ladder-base` (crypto-autoresearcher `knowledge/leads/ecdlp/`, reviewed 2026-10-31) | read | measured paired S gain 1.29 (1.12, 1.52), 1.58 (1.35, 1.82), 3.36 (2.80, 4.02) at h = 2, 2, 6 on binary random curves; classed "engineering"; projected base still 6.1x, 11.5x, 14.4x above matched rho | **Prior measurement exists for binary curves. The prime / volcano family (h = 32) has not been measured in ecbench.** The size of the gain grows with h in that data, which is the direction the Python claim relies on. |
| lead atlas `LEAD-ICP-isogeny-transported-relation-yield`, `LEAD-ICB-cofactor-torsion-quotients` | titles and dispositions only | blocked / open neighbours | not relied on |
| `src/cryptanalysis/ic_framework/fiber.rs` | read at source (first 290 lines) | the fiber-aware oracles (`lifts`, `closed`) recover the same lost yield from the oracle side by probing the whole fibre `{T+K}` | **A competing way to get the same yield.** Not tested here; a later comparison should include it. |
| `research/volcano_ecdlp_census_20261017/` (specs, README) and `benchmarks/20261017-volcano-ecdlp-census/VALIDATION.json` | read | the census `ic` arm is `ic.pipeline`, `prime-abscissa:size=64`, `mitm:negation_folded=1`; matched rho is `rho-neg`; IC is far behind rho there (V1 pooled mean priced S 46.7 vs 1.10 at 0 GAE) | defines the baseline arm and the matched rho |
| web search (2 queries, 2026-11-03) on cofactor handling in elliptic-curve index-calculus factor bases | search snippets only (Kosters-Yeo arXiv 1503.08001, Semaev et al., Petit 16PKC, Huang-Kosters-Petit-Yeo-Yun) | none states a cofactor or subgroup projection of the factor base | evidence gap, **not** proof that no prior work exists. LEAD-19 reached the same conclusion from one search. |

The relayed Python re-implementation (gain 2.7x, 6.5x, 7.7x on three volcanoes; reviewers 2.65x, 6.0x, 6.8x
after adding back ecbench's larger fixed cost) was **not seen by me** and is cited as relayed. It was post hoc.

## 2. What the treatment changes, and what must stay identical

Treatment arm `ic-clear` = `ic.pipeline` with `factor_base = prime-projected-abscissa:size=64,scan_cap=4096`.
Baseline arm `ic-raw` = the census arm, `factor_base = prime-abscissa:size=64`.

Changes (only):
- which points enter the base: `[h]P` for the raw point `P` above each scanned abscissa, instead of `P`;
- therefore the abscissae of the base are no longer the smallest ones;
- the `h` scalar multiplications (here `h = 32`) are charged to base construction, as group operations.

Identical: oracle (`mitm:negation_folded=1`), 64 columns, linear algebra (`incremental-gauss`), curves, targets
(`planted_shared`, same `target_seed` for every arm), `max_trials`, warm-up and round structure, host, binary,
accounting unit (S = total GAE over the session floor).

Mechanism check (secondary): trials per relation should fall by about `h` (LEAD-19). It is recorded, not decisive.

## 3. Predictions, pass line, falsifier (frozen before data)

Gain `G_w = sum S_w(ic-raw) / sum S_w(ic-clear)`, a ratio of totals over the matched `(workload, round)` pairs of
the V2 population (14 curves x 8 targets = 112 workloads, 4 measured rounds after 1 warm-up), with every unpriced
native operation priced at `w` GAE for `w` in {0, 0.1, 1}, both arms alike (AGENTS.md section 5a). This is exactly
`ecbench compare --a ic-clear --b ic-raw --weight w` (`src/cryptanalysis/ecbench/compare.rs`: B/A, workloads as
clusters). The primary 95% interval is its linearised (delta-method) interval; the percentile bootstrap it reports
beside it is quoted too. The frozen commands are in `analysis.sh`.

- **Prediction.** `G_0` between 3 and 32; reviewers' estimate for V2 is about 6.
- **Pass line.** The lower confidence limit of `G_w` is above 1 at every `w` in the band (an order that does not hold
  across the band is not ranked).
- **Falsifier (the stated 3x claim).** The point estimate of `G_0` is below 3. Whether the upper limit is also below 3
  is reported separately.
- **Controls, all must hold or the verdict is "invalid controls":**
  - A/A (`ic-raw` vs `ic-raw-aa`): the 95% interval of the ratio contains 1.
  - Negative control (14 prime-order 26-bit curves, h = 1, its own session under the companion protocol
    `protocol-h1-control.json`, because `ecbench compare` cannot filter one session by population): the 95% interval
    of `G_0` lies inside [0.95, 1.05]. Failing it makes the main verdict `invalid_controls`.
  - Positive control (`rho-plain` vs `rho-neg`, a known counted-operation effect of about sqrt 2): lower limit above 1.2.
- **Distance remaining** (reported, not a gate): `S_w(arm) / S_w(rho-neg)` for `ic-raw` and `ic-clear`.

## 4. Scope and what this cannot show

- Counted metrics only. The host is a cloud VM graded L1 by ecbench: no timing claim.
- One prime field (p = 42,310,123), one r (1,322,303), h = 32, 14 curves of one volcano, 8 planted targets each.
  No statement about other sizes, other h, other families.
- Population overlaps the census curves (partially in-sample for the mechanism); targets and seeds are new.
- No leaderboard or `benchmarks/README.md` row is written. The validation gate (AGENTS.md; `docs/VALIDATION_GATE.md`)
  still needs, for any row: an independent replay certificate from a second host, a validator report, and the
  matched-rho band named by a `VALIDATION.json`. This study provides a same-host `ecbench verify --replay-all`
  only.
- Instrument identity: binary sha256 `b38cffb4f64c4ec7289ee4ef19bf60560d2d99684d4f996114eb403560081ec4`, built from
  `11173ecd0230a9ceedc4d833736402934fdcb95e` (no change to source). That binary does not embed its commit, so ecbench
  records the worktree HEAD at run time; sessions are therefore run at that exact HEAD with this directory untracked,
  and the evidence that the protocol preceded the data is the freeze timestamp and the S3 checkpoint pushed by the
  repository's run gate, not a git commit.
