# LEAD-C-jv-sieve-emit-price: emitted F4, descent and LA additions on the sieve route, p = 1511

Lead preregistered in `research/leads_20261111/lane_C_prime_ext_cover.md` (Lead 3). Local stage only.
Every result below is **[pcc:unverified]**: local L0 runs in a shared container, no pcc claim, no validator re-run.
This is an **accounting correction**. It is not an improvement, and nothing here moves a leaderboard row.

## Verdict

- **Kill falsifier: not triggered.** [pcc:unverified] The kill threshold is A_emit / proxy > 1.20. The measured
  value is 0.816 on seeds 17–24 and 0.815 on all 16 seeds. The proxy was an upper bound, so there is no erratum to
  VAL-20261103-c31c76.
- **Ordering falsifier: not triggered.** [pcc:unverified] No emitted layer-A cell comes near 1. The worst cell is
  0.1196 (U2, w = 1). Parity would take about 94 times the emitted additions.
- **Prediction A_emit / proxy = 0.92 [0.80, 1.05]: met only on the ratio of per-run means.** [pcc:unverified]
  - Ratio of per-run means: 0.816, inside the interval near its lower edge. This is the statistic that matches how
    the proxy was quoted ("1.56e9 averaged over seeds 17–24"). The measured proxy mean is 1.5649e9.
  - Per seed: 0.44 to 0.93. Five of seeds 17–24 fall below 0.80.
  - Geometric mean of the per-seed ratios: 0.715, outside the interval.
  - The lane file does not say which statistic it means, so I count this part as partially met.
- **Layer-A cells in both units: inside every preregistered interval.** [pcc:unverified] Each moves by less than 3 %
  from the validated value, well within the 15 % limit (table below).
- **Memory: about 68 MB peak RSS on the cover arm, below 4 GB.** [pcc:unverified] Peak RSS was not measured for the
  rho arm, because no rho run was made (frozen records).

Overall: **partial**. Both falsifiers survive and the layer-A cells match. The ratio prediction holds only as the
ratio of means, and the isolated-lab stage is blocked.

## Why the ratio is about 0.82, not about 1 (mechanism check)

[pcc:unverified]

- **Descent.** Additions per proxy multiplication are 0.990–0.992 on all 16 seeds. F4 row operations are
  multiply-subtracts, except the pivot normalisations (about 1.9 % of row operations), which are products only. The
  descent's other stages (Weil, linear stage, post-processing, verification, residual stream, F4 monomial merges)
  run at about 1 addition per product.
- **Linear algebra.** The proxy took `la_muls = 16 * la_ops`, the legacy flat price of 16 per Wiedemann operation.
  The emitted count is one mod-ℓ addition per operation (`modr_add` is about `la_ops`), so LA emits exactly 1/16 of
  its proxy.
  - The lead's mechanism counted LA "mults" as about 0.30e9 additions. That was wrong by 16x.
  - LA is a fixed cost of about 0.29e9 proxy per run. That is why seeds with a short descent (seed 18: proxy 0.49e9)
    have a low ratio (0.44).
- **Cross-check of the counters.** [pcc:unverified] The bitop/v1 descent `field_mul` agrees with the legacy
  `descent_muls` to within 0.3 % on every seed (ratio 0.99983–1.00264). The legacy F4 counter delta equals
  `descent_f4_muls` on every seed.

## What was run

### Code change

The change is opt-in, and default behaviour is unchanged. Diff: `results/source.diff`, sha256 `6bc6dcc5…`.

- `src/cryptanalysis/f4_fp.rs`:
  - adds `NORM_OPS` and `norm_ops_total()`;
  - `count_norm(n)` is called beside `count_ops` at the three pivot-normalisation sites (`rref32`, the lazy reducer,
    `rref`);
  - it increments only while `jv_bitops` counting is enabled, which is off by default. Legacy counters are untouched.
- `examples/jv_cover.rs`, `--exp sieve`:
  - with `JV_SIEVE_EMIT=<path>`, the same route runs through `sieve_emit()`;
  - `sieve_emit()` turns on `jv_bitops`, uses a one-thread rayon pool that claims the tally, builds the spec under
    `Phase::Instance`, then calls `run_cover_sieve_dlp_on` (what `run_cover_sieve_dlp` calls);
  - it writes a sidecar `jv_sieve_emit/v1` JSON: per-phase bitop/v1 counts, F4 row and normalisation operations,
    foreign hook calls, `VmHWM`;
  - unset, it calls `run_cover_sieve_dlp` exactly as before.
- Binary: `cargo build --release --example jv_cover` on HEAD `6b7fc4520e` plus the diff. Its sha256 is in
  `results/BINARY.sha256` (`b1ad327c…`). Files were formatted with `rustfmt --edition 2021` on the two changed files
  only.

### Runs

`run.sh`, invoked as `JV_BIN=<binary> research/leads_20261111/runs/C_sieve_emit_price/run.sh`:

```
env -u JV_FB_LINE -u JV_FB_LINE_MODE -u JV_SIEVE_HILBERT -u JV_SIEVE_EMIT RAYON_NUM_THREADS=1 JV_SIEVE_ROUND_THREADS=4 JV_LIN=sparse \
  [JV_SIEVE_EMIT=results/emit_sN.side.json] $B --exp sieve --stop 64 --sizes 1511 --first-seed N --seeds 1 --rho-runs 0 --json results/<label>.json
```

- **Emit arm:** seeds 17–32.
- **Plain arm (default-unchanged control):** seeds 17, 24, 27, 32. Same binary, `JV_SIEVE_EMIT` unset.
- Each run takes 63–163 s single-threaded. Logs and wall times are in `results/`.
- `JV_SIEVE_ROUND_THREADS=4` keeps the round plan of the committed 4-thread runs. Without it, `jv_sieve.rs` plans
  rounds by `rayon::current_num_threads`.

### Analysis

- `python3 research/leads_20261111/runs/C_sieve_emit_price/analyze.py` writes `results/analyze.out`,
  `results/summary.json`, `results/emit_a_primary.json` and `results/emit_a_upper.json`.
- **Rescore with the validator's own check.**
  - Source: `check_emit/` is a copy of `research/jv_sieve_negrho_20261102/review/validator/check`.
  - Patch: one change in `cov_of`. With `EMIT_A_JSON=<file>`, the per-seed layer-A proxy is replaced by A_emit.
    Unset, the computation is unchanged.
  - Control: unset, it reproduces the committed `check_output.txt` and `recompute.json` byte for byte (`diff` and
    `cmp`). Outputs: `rescore/control_*`.
  - Emitted rescore: `rescore/emit_primary_*`, with the sensitivity variant in `rescore/emit_upper_*`.
  - The single `[FAIL]` line in the emitted rescore is expected. It compares the cells against the producer's
    `fit.json`, which used the proxy.

### A_emit definition

The scope matches the proxy, which is `total_muls − sieve_muls − enum_muls`. A_emit is the sum of:

- **Descent:** `field_add` minus F4 normalisation operations. This covers F4 multiply-subtracts and monomial merges,
  plus the Weil, linear, post, verify and stream stages.
- **Linear algebra:** `modr_add` (+ `field_add`, which is 0).
- **Small phases:** `field_add` of the factor base, model, verification and unattributed phases.
- **Carried at k = 1:** `setup + table + extract + verify` multiplications (1.14e7 per run, 0.73 % of the proxy).
  These sit inside the Relations phase, and the bitop tally cannot separate them from the sieve and enumeration
  additions.

Sensitivity variant "upper": the F4 normalisations are counted as additions too, which is the bitop/v1 row-operation
convention.

## Results

### C1: counted multiplications reproduce the committed records

[pcc:unverified]

- **Records compared:** all 16 emit rows and all 4 plain rows.
- **Result:** every key of `research/jv_sieve_negrho_20261102/results/inputs/ic_1511_sparse.json` except `wall_ms`
  is identical: `total_muls`, `total_plus`, every `*_muls`, `descent_digest`, relations, solved and correct.
- **Keys added since the V2 build:** `descent_hilbert` = 0 and the other Hilbert, shared-array and incidence fields.
  These keys are absent from the committed records.
- **Counting checks:** foreign hook calls are 0 and other-modulus operations are 0 on every run.

### Per seed (proxy = unemitted multiplications; ratio = A_emit / proxy)

| seed | proxy | A_emit | ratio | F4 adds | F4 norm ops | other descent adds | LA adds | descent adds/mul | peak RSS MB |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 17 | 1.7528e9 | 1.4560e9 | 0.831 | 1.0300e9 | 1.95e7 | 3.95e8 | 1.88e7 | 0.990 | 68.0 |
| 18 | 4.9093e8 | 2.1754e8 | 0.443 | 1.3438e8 | 2.55e6 | 5.32e7 | 1.81e7 | 0.992 | 68.2 |
| 19 | 1.0682e9 | 7.7464e8 | 0.725 | 5.3739e8 | 1.02e7 | 2.06e8 | 1.91e7 | 0.991 | 68.0 |
| 20 | 4.4501e9 | 4.1196e9 | 0.926 | 2.9563e9 | 5.61e7 | 1.13e9 | 1.93e7 | 0.990 | 68.0 |
| 21 | 6.0878e8 | 3.3925e8 | 0.557 | 2.2397e8 | 4.25e6 | 8.60e7 | 1.78e7 | 0.992 | 68.0 |
| 22 | 1.3539e9 | 1.0818e9 | 0.799 | 7.6154e8 | 1.45e7 | 2.91e8 | 1.74e7 | 0.990 | 68.1 |
| 23 | 1.1235e9 | 8.3593e8 | 0.744 | 5.8238e8 | 1.11e7 | 2.23e8 | 1.86e7 | 0.991 | 67.9 |
| 24 | 1.6711e9 | 1.3909e9 | 0.832 | 9.8521e8 | 1.87e7 | 3.76e8 | 1.78e7 | 0.990 | 68.0 |
| 25–32 | | | 0.680–0.876 | | | | | 0.990–0.991 | 67.9–68.1 |

The full table is in `results/analyze.out`.

### Aggregates

[pcc:unverified]

| statistic | seeds 17–24 | seeds 17–32 | preregistered |
|:--|--:|--:|:--|
| mean proxy per run | 1.5649e9 | 1.5534e9 | "1.56e9" |
| mean A_emit per run | 1.2770e9 | 1.2660e9 | |
| A_emit / proxy, ratio of means | **0.816** | 0.815 | 0.92 [0.80, 1.05]; kill > 1.20 |
| A_emit / proxy, geometric mean of per-seed ratios | 0.715 | | |
| A_emit / proxy, per-seed range | 0.443–0.926 | 0.443–0.926 | |
| "upper" (normalisations as adds), ratio of means | 0.827 | 0.826 | |

### Layer A (cover seeds 17–24, pooled 142-run negation rho, validator check, 20,000-draw joint bootstrap)

[pcc:unverified]

| cell | validated (proxy) | emitted | preregistered | inside | distance to rho, proxy → emitted |
|:--|--:|--:|:--|:--|:--|
| U1 w=0 | 0.00647 | 0.00647 [0.00546, 0.00802] | 0.0065 | yes | x154.5 → x154.5 |
| U1 w=0.1 | 0.1071 | **0.10456** [0.09429, 0.11625] | 0.098 [0.090, 0.112] | yes | x9.3 → x9.6 [8.6, 10.6] |
| U1 w=1 | 0.1214 | **0.11853** [0.10693, 0.13171] | 0.112 [0.102, 0.128] | yes | x8.2 → x8.4 [7.6, 9.4] |
| U2 w=0.1 | 0.1165 | **0.11377** [0.10260, 0.12650] | 0.107 [0.097, 0.122] | yes | x8.6 → x8.8 [7.9, 9.7] |
| U2 w=1 | 0.1226 | **0.11963** [0.10792, 0.13294] | 0.113 [0.103, 0.128] | yes | x8.2 → x8.4 [7.5, 9.3] |

- **Change from the validated values:** emitted/validated is 0.976 in every w > 0 cell, well within the 15 % limit.
- **Order:** below rho at all three weights in both units.
- **All 16 seeds:** U1 w=1 is 0.11829 and U2 w=1 is 0.11940.
- **Parity (w = 1):** the breakeven multiplier on the emitted additions is k = 94.4 (U1) and 93.4 (U2). On the proxy
  it was 69.
- **Layer B:** unchanged, because it does not use the proxy.
- **Reading:** the priced distance to the pooled negation rho moves by about 2.4 % in the cover's favour. This is a
  re-pricing, not an improvement.

## Not covered, or blocked

- **Iso Lab job blocked.** The isolab-eks task card needs AWS, and the AWS identity is rejected in this container.
  Without it there is no isolated-lab run, no full (non-sampled) replay of the 16 cover rows on a second host, and no
  S3 raw data or bench record. Validator limitations 1 to 4 therefore stay open on the lab side. Locally, all 16
  cover rows were replayed bit for bit (C1).
- **No rho-arm memory.** Peak RSS was not measured for the rho arm, because no rho run was made. Every cover peak RSS
  figure comes from the emit (counting) runs.
- **Small slice not emitted.** Setup, table, extract and verify (0.73 % of the proxy) are carried at k = 1, not
  emitted.
- **F4 monomial-ordering work not counted.** F4 additions outside `count_ops` and `normalize` merges, such as pair
  bookkeeping, are not additions in F_p and stay in the native counter.
- **Statistic ambiguity.** The lane file does not say whether the A_emit / proxy interval applies to the
  ratio of means or per seed. Both are reported above. Only the ratio of means is inside.
- **Not done:** no pcc claim, no `VALIDATION.json` change, no leaderboard, `benchmarks/` or `docs/` edit (per task
  rules).

## Next step

1. Freeze the A_emit definition above. Have the validator re-run the gate with `EMIT_A_JSON`, or better, with
   `jv_sieve_emit/v1` read directly. The row's layer A then becomes "priced, emitted" (U1 w = 1:
   0.1185 [0.1069, 0.1317]).
2. Run the isolab card when AWS is available: the 16 emit rows on the isolated lab, a full replay, and memory on
   both arms.
3. Emit setup, extract and verify additions with their own phase split. This closes the last 0.73 %.
4. Separately: the legacy LA price of 16 F_p multiplications per Wiedemann operation is itself a convention. A
   bitop/v1 pricing of `modr_mul` at bits(ℓ) = 62 would re-price layer S. That is out of scope here.
