# VERIFY: LEAD-C-jv-sieve-emit-price (independent skeptic, 2026-11-12)

Every statement here is **[pcc:unverified]**: a local re-check in the shared container, with no pcc claim.

## What I re-ran

I used the patched binary `target/release/examples/jv_cover`. Its sha256 `b1ad327c…e8f1a0` equals
`results/BINARY.sha256`. I copied it to the scratchpad and ran seed 18 single-threaded with the exact `run.sh` command:

```
env -u JV_FB_LINE -u JV_FB_LINE_MODE -u JV_SIEVE_HILBERT -u JV_SIEVE_EMIT RAYON_NUM_THREADS=1 JV_SIEVE_ROUND_THREADS=4 JV_LIN=sparse \
  [JV_SIEVE_EMIT=<side.json>] <bin> --exp sieve --stop 64 --sizes 1511 --first-seed 18 --seeds 1 --rho-runs 0 --json <out>
```

- **Emit arm:** 148 s.
  - All 83 keys of the committed record `research/jv_sieve_negrho_20261102/results/inputs/ic_1511_sparse.json`
    (seed 18) except `wall_ms` are identical (`total_muls` 2610714441).
  - My sidecar `jv_sieve_emit/v1` is identical to the committed `results/emit_s18.side.json` except `vm_hwm_kb`.
- **Plain arm** (`JV_SIEVE_EMIT` unset): 72 s. All 83 committed keys except `wall_ms` are identical. The producer's
  plain controls were seeds 17, 24, 27 and 32, so this is an extra, independent default-unchanged control.
- **Key number recomputed** from `results/summary.json` and the committed record:
  - A_emit / proxy over seeds 17–24, ratio of means: 0.81599. The geometric mean is 0.7148 and the per-seed range is
    0.443–0.926. These match the report.
  - The legacy `la_muls / la_ops` equals 16.0 exactly on all 16 seeds, and the emitted `modr_add / la_ops` equals 1.000.
    That confirms the 1/16 LA mechanism.

## Source changes

- **`f4_fp.rs`:**
  - `NORM_OPS` is incremented only under `jv_bitops::enabled()`, at the three pivot-normalisation sites, beside the
    existing `count_ops`.
  - Legacy counters are untouched. In `rref_ckpt`, `ops` there counts only the normalised nonzeros, so it is correctly
    a normalisation count.
- **`examples/jv_cover.rs`:** the new path is taken only if `JV_SIEVE_EMIT` is set and non-empty. Otherwise it is the
  original `run_cover_sieve_dlp` call.
- **Validator copy `check_emit`:**
  - Its diff against the validator's `main.rs` is the single opt-in `EMIT_A_JSON` branch in `cov_of`.
  - The stored `rescore/control_check_output.txt` and `control_recompute.json` are byte-identical to the committed
    validator outputs. I compared the stored files with `cmp`. I did not rebuild and re-run the check myself.

## Falsifiers versus the lane file

The thresholds are as preregistered and none was moved:

- The kill threshold is A_emit / proxy > 1.20. The measured value is 0.816, so the lead is not killed.
- The ordering kill fires if a layer-A cell reaches 1. The largest cell is 0.1196, so it is not killed.
- The prediction interval was [0.80, 1.05]. The ratio of means sits inside it, close to the lower edge. The per-seed
  values and the geometric mean are not inside.
- The producer reported this honestly as "partial" and did not claim that the prediction was fully met.

## Caveats (they do not overturn the verdict)

1. **The LA pricing convention drives most of the gap below 1.**
   - The proxy priced each mod-ℓ Wiedemann product as 16 F_p multiplications. A_emit prices each mod-ℓ addition as
     one native operation.
   - Under the per-operation band ("prices operations, not bytes") that is defensible. Still, it is a convention, not
     a measurement.
   - If an ℓ-addition were scaled like the ℓ-multiplication (×16), A_emit / proxy would be about
     (1.277e9 + 0.278e9) / 1.565e9 ≈ 0.99. That is still inside [0.80, 1.05], and both falsifiers still survive.
   - So the verdict is robust to this choice, but "0.816" and "x8.4" should be quoted together with the convention.
2. **Setup, table, extract and verify work is carried at k = 1.** This is 0.73 % of the proxy and is negligible.
3. **The rescore control equality rests on stored files.** I verified those files against the committed outputs, but
   I did not re-run the check binary.
4. **The Iso Lab, full replay and rho-side RSS stages are blocked, as stated** (AWS identity rejected).

## Verdict

The key number reproduces. I re-ran seed 18 on both arms with bit-identical output, and the aggregates recompute
exactly. The verdict follows the preregistered falsifiers, and the opt-in changes leave default behaviour unchanged.
I **agree** with "partial" and with the accounting-only reading: this is not an improvement.
