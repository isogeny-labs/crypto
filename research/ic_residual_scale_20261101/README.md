# Scale of the priced DP residual collectors (p36, p40, p42): result

Protocol [PROTOCOL.md](PROTOCOL.md) (frozen in commit fb2a3c1c34, before data; one wording fix to the curve-check sentence afterwards). Report [REPORT.pdf](REPORT.pdf).
Reproduce: `scripts/gen_curve.py 40 20261701` (and 42), `scripts/make_specs.sh; scripts/run_all.sh; scripts/analyse.sh; python3 scripts/summarise.py`.
Sessions: 3 x 288 records; 863 verified, one baseline (res4, p42) exhausted at 1e8 trials (baseline table cap 2^22); audits OK; A/A 1.0000.

Distance to matched rho (w = 0 / 1) p36 -> p40 -> p42: res1eq 1.32/2.27 -> 1.72/2.99 -> 1.39/2.42; res1dp 2.15/2.25 -> 2.27/2.28 -> 1.92/1.93; baseline res4 3.70/48.1 -> 4.10/53.3 -> 3.47/42.4. No growth with r over this range.
C1 (p40, p42; 8 cells): not met, 6 of 8; both failures at w = 0 (p40 res1dp [0.683, 1.182]; p42 res4dp [0.148, 1.295], widened by the censored baseline run). All w = 0.1 and w = 1 cells below 1.
Class: engineering of a generic collision search; rho stays ahead. No VALIDATION.json: no leaderboard / benchmarks row.


> **Pricing caveat and errata (added after the independent validator's report, `benchmarks/20261101-ic-residual-dp-eq/validator-report.json`).** (1) The band as built prices `residual_table_bytes_uncharged`, a byte count, as operations at w > 0; the large w > 0 gains and the 48-53x baseline distances at w = 1 are mostly that artifact. With that counter excluded: dp/baseline is a constant 0.76-0.94 at every weight, eq/baseline 0.54-0.68 at w = 0 and 0.9-1.18 at w = 1, and the baseline's distance to rho is 2.5-4.1x at every weight (res1 about 2.5, res4 3.7-4.1). w = 0 results are unaffected. See REPORT.pdf. (2) The C1 tally is 9 of 12 (curve, B, candidate) cells (eq 6/6, dp 3/6), not "12/12 and 9/12 of 24"; the protocol text's "24 cells" was an arithmetic slip.

> **Note, 2026-11-01: the band now prices operations, not bytes (owner decision, option B; `band.rs` `NON_OPERATION_COUNTERS`).** The text above is frozen as written. [`analysis/exbytes/`](analysis/exbytes/) holds the three bands and the 42 weighted comparisons re-run with the current binary (same arms, weights and bootstrap seeds; `sessions/` untouched); [`C1.md`](analysis/exbytes/C1.md) lists every cell next to the as-built value. What changes at w > 0 (w = 0 is identical):
> - **C1 is met in 2 of 8 cells, not 6 of 8.** `eq` fails all four cells at w = 1, where its interval reaches or crosses 1: p40 res1eq 1.181 [0.859, 1.625], res4eq 1.089 [0.924, 1.285]; p42 res1eq 0.955 [0.774, 1.177], res4eq 1.084 [0.964, 1.219]. `dp` is unchanged at 2 of 4 (p40 res4dp and p42 res1dp pass; p40 res1dp and p42 res4dp fail at w = 0 as before) and on p40 and p42 is a constant 0.76-0.90 of its baseline at every weight above 0. "All w = 0.1 and w = 1 cells below 1" no longer holds for `eq`.
> - **Distance to matched rho (w = 0 / 1), baseline:** res1 2.5 / 2.5 and res4 3.4-4.1 / 3.4-4.1 on p36, p40, p42 (was 17-18 and 42-53 at w = 1). `dp`: res1dp 1.9-2.3, res4dp 3.0-3.5 at every weight. `eq`: res1eq 1.3-1.7 / 2.2-3.0 and res4eq 2.1-2.6 / 3.7-4.5, nearly unchanged because `eq` carries no byte counter that matters at w = 1.
> - What remains true: no growth of the distance with r; rho stays ahead; all verification, audit and A/A results are untouched.
> - Not from this note: errata (2) above says the C1 tally over 12 cells is 9 of 12 (dp 3/6); the study's own `analysis/summary.json` gives dp 2/6 and eq 6/6, which is 8 of 12 as built. The 8-cell C1 of 6 of 8 agrees. Nobody has reconciled the 9; this note does not change that text.
> Audit and rule: [`research/ic_byte_counter_pricing_20261101`](../ic_byte_counter_pricing_20261101/README.md).
