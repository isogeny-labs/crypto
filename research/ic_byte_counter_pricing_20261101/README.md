# Is `residual_table_bytes_uncharged` priced as an operation?

Audit, 2026-11-01, of finding F1 in
`benchmarks/20261101-ic-residual-dp-eq/validator-report.json` (PR #3119). No
measurement was run; this re-reads committed bands and hydrated raw records.
**Nothing under `src/` or any committed session record was changed.** The
decision on how to price bytes is the owner's (item 2 below).

## Verdict

**Confirmed.** The byte counter is 83-99% of the residual arms' band-priced S
at w = 1 and 40-98% at w = 0.1 in both studies, and it is the whole reason
their w > 0 distances to the matched rho are 17-58x rather than 2.6-5x.

## How a byte count becomes "operations"

| step | where | what happens |
|:--|:--|:--|
| emit | `src/cryptanalysis/ic_framework/residual.rs:297-300` | `rel.count("residual_table_bytes_uncharged", table.len() * (16 + 8 * d_col))`: a byte count of the residual table |
| list | `src/cryptanalysis/ecbench/methods.rs:5135-5141` | every phase-native counter named `*_uncharged` with count > 0 is appended to `cost.unpriced` |
| price | `src/cryptanalysis/ecbench/band.rs:39-72, 136-146` | `unpriced_counts` sums the counts; `priced_gae(w) = G - C + w * (U + n_C)`, so each byte costs `w` GAE |
| contrast | `src/cryptanalysis/ecbench/work.rs:454` | the analogous `native_table_bytes_uncharged` is class `Shape` (not work); `residual_table_bytes_uncharged` has no rule there |

§5a prices "native operations". A byte count is a space measure. The entry
count `residual_table_entries` and the probes (`lookups`) are already
counted separately, so the table's time cost is already in the band.

## Confirmation on the two studies

Hydrated with `tools/artifacts/artifacts.py pull` (24 + 25 files, `verify`:
0 problems). `scripts/byte_share.py` mirrors `band.rs::unpriced_counts`,
reads the published `analysis/band-*.json` mean S, and subtracts only the
byte counter's own `w * bytes / sqrt(r)` (mean over verified, non-warmup
runs). Full per-arm output: `analysis/byte_share.txt` / `byte_share.json`.
Ratios are ratios of mean S, not the paired bootstrap of `ecbench compare`.

The reconstruction reproduces the published numbers: the planner study's
README cells 17.6, 20.5, 23.3, 24.3 (res2 / rho, w = 1) and 31.3, 39.5, 41.9,
47.3 (res4 / rho, w = 1) come out of the published band exactly.

`ic_prime_residual_lp_20261026` (p21-p32, 24 verified runs per arm), res4 / rho:

| session | byte share of S, w = 0.1 | w = 1 | w = 1 as published | w = 1, bytes excluded |
|:--|--:|--:|--:|--:|
| p21 | 63% | 91% | 41.2x | 3.6x |
| p22 | 64% | 91% | 56.0x | 4.8x |
| p24 | 67% | 92% | 50.4x | 4.1x |
| p26 | 67% | 92% | 57.6x | 4.6x |
| p28 | 68% | 92% | 51.4x | 4.0x |
| p32 | 68% | 92% | 51.1x | 4.0x |

Other arms: res8 75-78% / 95%, res16 84-87% / 97%, res32-64 90-96% / 98-99%.
With the counter excluded the distance is flat in w (p24 res4: 4.07x at
w = 1, 4.05x at w = 0.1), as expected of a counter that is not work.

`ic_planner_exhaustive_20261026` (48 verified runs per arm), the two cells
the task suspected:

| cell | arm | published w = 1 | bytes excluded | byte share, w = 1 |
|:--|:--|--:|--:|--:|
| p36 | res2 | 23.3x | 2.6x | 89% |
| p37 | res2 | 24.3x | 2.7x | 89% |
| p16 / p18 | res2 | 17.6x / 20.5x | 3.0x / 3.0x | 83% / 85% |
| p36 / p37 | res4 | 41.9x / 47.3x | 3.2x / 3.6x | 92% |

The 23-24x are the artifact. The leaderboard row for this study
(`docs/ic/LEADERBOARD.md`, 2.59x) is the w = 0 value and is unaffected.

**A second byte counter.** `factor_base_vec_bytes_floor_uncharged`
(`ic_framework/plugins.rs:1313`) is priced the same way in the Koblitz
sessions (K-k41..k61): 0.4-12% of S at w = 1, at most a 12% shift in the
w = 1 distance (k47: 18.3x to 16.1x). It is small, but not zero. The binary
`B-hb*` sessions carry no byte counter. `full_price.rs:161` already prices
this one correctly under the `*_full` methods: 0.125 `word_xor` per byte.

**Not audited** (raw records not hydrated): `ic_planner_weight_rule_binkob_20261026`,
`ic_koblitz_descent_routing_20261026` and the embedded record in
`docs/backend-leaderboard` also name `factor_base_vec_bytes_floor_uncharged`.
`research/ic_residual_scale_20261101/` and `scripts/band_ex_bytes.py` (PR
#3119) are not on this branch, so the F1 figures (about 2.5x at B = 1,
3.7-4.1x at B = 4) were not re-derived here; they agree with res2 2.6-3.0x
and res4 3.2-4.8x above.

## Proposal for the owner: how should bytes be priced?

| | A. keep (bytes are operations) | B. exempt like Shape (recommended) | C. per-byte weight |
|:--|:--|:--|:--|
| change | none | classify the byte counters as non-work in one place (`band.rs` exempt list, or drop the suffix at the emitter) | give `unpriced_counts` a per-counter weight, e.g. 1/8 op per byte |
| consistent with `work.rs` | no | yes | yes, if the weight is stated |
| w = 0 rows | unchanged | unchanged | unchanged |
| residual res2 / rho, w = 1 | 17-24x | 2.6-3.0x | 4.8-5.4x |
| residual res4 / rho, w = 1 | 31-58x | 3.2-4.8x | 7.2-11.2x |
| residual arm vs `plan`, byte-bearing, mean S | 36 of 50 "unresolved inside the band" | 0 of 50 unresolved: the residual arm is cheaper at every weight | 22 of 50 unresolved |
| Koblitz factor-base counter | priced at w per byte | exempt (up to 12% off the w = 1 distance) | priced at 1/8 w per byte |
| risk | keeps a 15-20x artifact in published distances | a real memory cost is no longer in S | the weight is a new constant; at 1/8 op per byte and w = 1 it is still about 400x the measured word-store cost (0.0025 GAE per word is 0.0003 GAE per byte; `full_prices.json` has binary rows only, none for prime curves) |

Recommendation: **B**, as a classification in `band.rs` that names both
counters, rather than renaming the emitter. A rename changes counters in
newly produced records only, and would break replay audits and the
`factor_base_vec_bytes_floor_uncharged` rule of `full_price.rs`; a
`band.rs` list applies to old and new records alike and leaves every raw
record, replay digest and `VALIDATION.json` as committed. If memory should
be priced, do it as C with a weight measured on a prime curve first, and
price a rho arm's own storage the same way (§5a "every arm"). Report the
table size as a separate Shape column either way.

Effect of B on published rows, once decided (item 3, not done): the w > 0
columns of the bands of the two studies, the w = 0.1 and w = 1 distances in
their READMEs, and every verdict that depends on residual-versus-plan order
at w = 1. 36 of the 50 residual-vs-plan comparisons (resN0 aside) change
from "unresolved" to "residual arm cheaper at every w" on means; the paired
`comparisons/*.json` verdicts must be regenerated with `ecbench compare`
rather than inferred from this table. Frozen evidence is not rewritten; each
README gets a dated note.

## Reproduce

```bash
export ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store
python3 tools/artifacts/artifacts.py pull research/ic_prime_residual_lp_20261026 research/ic_planner_exhaustive_20261026
python3 -I research/ic_byte_counter_pricing_20261101/scripts/byte_share.py \
  research/ic_prime_residual_lp_20261026 research/ic_planner_exhaustive_20261026 \
  --out research/ic_byte_counter_pricing_20261101/analysis
```

Not produced: a graph or PDF (`research-visuals`), because no result lands
in `benchmarks/` or the leaderboard; add them with item 3.
