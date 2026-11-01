# Scale of the priced DP residual collectors (p36, p40, p42): result

Protocol [PROTOCOL.md](PROTOCOL.md) (frozen in commit fb2a3c1c34, before data; one wording fix to the curve-check sentence afterwards). Report [REPORT.pdf](REPORT.pdf).
Reproduce: `scripts/gen_curve.py 40 20261701` (and 42), `scripts/make_specs.sh; scripts/run_all.sh; scripts/analyse.sh; python3 scripts/summarise.py`.
Sessions: 3 x 288 records; 863 verified, one baseline (res4, p42) exhausted at 1e8 trials (baseline table cap 2^22); audits OK; A/A 1.0000.

Distance to matched rho (w = 0 / 1) p36 -> p40 -> p42: res1eq 1.32/2.27 -> 1.72/2.99 -> 1.39/2.42; res1dp 2.15/2.25 -> 2.27/2.28 -> 1.92/1.93; baseline res4 3.70/48.1 -> 4.10/53.3 -> 3.47/42.4. No growth with r over this range.
C1 (p40, p42; 8 cells): not met, 6 of 8; both failures at w = 0 (p40 res1dp [0.683, 1.182]; p42 res4dp [0.148, 1.295], widened by the censored baseline run). All w = 0.1 and w = 1 cells below 1.
Class: engineering of a generic collision search; rho stays ahead. No VALIDATION.json: no leaderboard / benchmarks row.


> **Pricing caveat and errata (added after the independent validator's report, `benchmarks/20261101-ic-residual-dp-eq/validator-report.json`).** (1) The band as built prices `residual_table_bytes_uncharged`, a byte count, as operations at w > 0; the large w > 0 gains and the 48-53x baseline distances at w = 1 are mostly that artifact. With that counter excluded: dp/baseline is a constant 0.76-0.94 at every weight, eq/baseline 0.54-0.68 at w = 0 and 0.9-1.18 at w = 1, and the baseline's distance to rho is 2.5-4.1x at every weight. w = 0 results are unaffected. See REPORT.pdf. (2) The C1 tally is 9 of 12 (curve, B, candidate) cells (eq 6/6, dp 3/6), not "12/12 and 9/12 of 24"; the protocol text's "24 cells" was an arithmetic slip.
