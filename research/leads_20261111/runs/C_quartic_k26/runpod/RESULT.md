# LEAD-C-quartic-n4v2-k26: clean decision on one dedicated RunPod pod

2026-11-12. Task `quartic-k26-clean`. This follows up `../RESULT.md` and `../VERIFY.md`. The skeptic
found that the earlier verdict rested on mixed binaries (`v2w` for k <= 24, `v2w2` for k = 25), on
calibrations taken under shared-host load of 1-6, and on an interval that left out calibration
noise.

This run uses one binary on one dedicated pod, with four calibration rounds interleaved with the
runs. The protocol was frozen before any data was taken: `PROTOCOL.md`, with Addendum A written
before any pod record was read. Every result below is **[pcc:unverified]**: there is no replay
certificate, no validation file, and no isolab host. This is an L0 result taken on a rented CPU pod.

## Verdict

**The lead is not killed on any of its three falsifiers.** Both of the lead's numeric predictions
miss, so the outcome is "survives, prediction missed". [pcc:unverified]

| falsifier / prediction (lane file, frozen, not moved) | measured on the pod | status |
|:--|:--|:--|
| Kill if the DL exponent over k = 22..26 has a lower 95% bound above 1.10 | **1.0681 [1.0680, 1.0682]** (M-eq, w = 0). The interval covers units and per-size calibration rounds. Per-round range 1.0681-1.0682. | **not killed**, lower bound 0.032 below the line |
| Kill if DL / rho-neg at k = 26 has an upper bound above 0.01 | 0.00162 [0.00129, 0.00199] at w = 0; 0.00139 [0.00111, 0.00170] at w = 0.1; 0.00086 [0.00068, 0.00105] at w = 1 | **not killed** (upper bound 5x below the line) |
| Kill on any unverified logarithm | 90/90 IC records (30 units at k = 17..26 x arms D, DL1, DL): `ok` and `k_found == k` | **not killed** |
| Prediction: exponent over k = 17..26 of 1.00 [0.95, 1.06] | **1.0837 [1.0831, 1.0843]** | **missed** |
| Prediction: DL / rho-neg at k = 26 of 0.00085 [0.0006, 0.0014] | 0.00162 [0.00129, 0.00199] | **missed** (1.9x higher; the bands overlap only in 0.00129-0.0014) |
| Prediction: DL wall at k = 26 of about 190-400 s | DL arm phase CPU 193.5 s (mean of 3; whole D-family unit 491-623 s) | inside, at the low edge, on a faster host than the M4 |

### How to read the exponent

The k = 22..26 fit (1.068) passes. The k = 22..25 fit on the same records and the same calibration
is **1.1107 [1.1106, 1.1109]**, which is above the line. That range is not the frozen test range,
and it is reported here only so the result is not over-read.

The gap between the two fits comes from one step at k = 22 -> 23. There `ell` crosses 2^64, the
arithmetic moves from the u64 path to 128-bit Barrett, and the calibrated `r_lmul` jumps from 2.569
to 3.701. On a 4-point window that step dominates the slope. Adding k = 26 dilutes it.

Local slopes of DL M-eq between neighbouring sizes (post hoc, descriptive only):

| step | k 22->23 | 23->24 | 24->25 | 25->26 | k = 23..26 fit |
|:--|--:|--:|--:|--:|--:|
| DL | 1.404 | 0.980 | 0.991 | 0.979 | 0.984 |
| DL1 | 1.269 | 0.985 | 1.009 | 1.010 | 1.002 |

Past the wall, DL scales as about q^0.98, which is in line with Diem's O~(q). The exponent over
any window that contains the wall carries a one-time constant-factor step of about 1.44x, from
`r_lmul` 3.70 / 2.57. That step is arithmetic, not algorithm. A window-independent statement is
"q^0.98 past the wall, plus a one-time step of about 1.4x at the wall". The decision itself stays on
the frozen k = 22..26 window. [pcc:unverified]

### What changed relative to the shared-host run

* **Calibration noise on the dedicated pod is small.** Across the 4 rounds, the round medians of
  r_inv and r_lmul stay within 0.1% of the pooled value for k >= 20, and within 0.8% at worst
  (k = 17-19) (`fig_pod.png`, right panel).
* On the shared container, the skeptic found r_lmul at k = 24 varying from 3.04 to 3.27 between
  runs. That noise is what put the earlier k = 22..25 lower bound on the line.
* Here the decision interval is dominated by unit sampling. It is still narrow, because the 3 seeds
  have low spread.
* The pod's constants are higher than the local container's: r_lmul 3.70 against 3.15 at k = 23,
  and r_inv 26.3 against 24.0 at k = 22. This is a host (CPU) effect. The counts are bit-identical.

## Results

### Run identity [pcc:unverified]

| item | value |
|:--|:--|
| binary | `v2w2`, sha256 `cf55da044d09afae946ca09b5057663f7f6e8e9237e13779feed090d37e4245e`. Rebuilt here with `rustc 1.97.0 --edition 2021 -O src/v2w.rs`; bit-identical to the local `../bin/v2w2`. Uploaded with `rp.sh put` (sha256 checked on the pod) |
| source | `research/leads_20261111/runs/C_quartic_k26/src/*.rs`, last commit touching it `1a0957d6c0`. Hashes match `../raw/SHA256SUMS_v2w2.txt`. Checkout HEAD `2003c4bc85` |
| host | RunPod pod `c9gkalzltj9f42` (`bdr-quartic-k26-clean`), cpu5c, 16 vCPU, 32 GB quota, AMD EPYC 4565P (the physical host has 32 CPUs), Linux 6.8.0, Python 3.12.3 |
| schedule | main sequence 18:11-19:21Z, strictly sequential; Addendum A k = 17..21 19:21-19:24Z; rho 19:24-20:07Z (16 in parallel, after every timed IC job) |
| identity control | 27/27 pod units (k = 17..25) have non-timing fields identical to the local records (`../raw/wide`, `../raw/v2w2/k25`): 7938 fields. k = 26 has no local counterpart |
| resources at k = 26 | D-family unit 491-623 s wall, peak RSS **5.72 GiB** (the projection was 5.7 GiB); `V2W_COMPACT=1` |

### Calibration (median of 3 `calib` records per size and round) [pcc:unverified]

| k | round 1 | round 2 | round 3 | round 4 | pooled (12 records), r_inv / r_lmul |
|--:|:--|:--|:--|:--|:--|
| 22 | 26.32 / 2.569 | 26.33 / 2.568 | 26.31 / 2.568 | 26.31 / 2.569 | 26.31 / 2.569 |
| 23 | 27.63 / 3.700 | 27.63 / 3.701 | 27.61 / 3.701 | 27.62 / 3.700 | 27.63 / 3.701 |
| 24 | 28.77 / 3.640 | 28.79 / 3.640 | 28.78 / 3.643 | 28.74 / 3.640 | 28.78 / 3.640 |
| 25 | 29.80 / 3.580 | 29.82 / 3.580 | 29.82 / 3.579 | 29.80 / 3.580 | 29.81 / 3.580 |
| 26 | 30.84 / 3.465 | 30.81 / 3.465 | 30.83 / 3.466 | 30.83 / 3.465 | 30.83 / 3.465 |

k = 17..21 are in `analysis_main.md`.

### DL per size (pooled calibration; 3 seeds each, all verified) [pcc:unverified]

| k | q | log2 ell | DL M-eq w=0 | w=0.1 | w=1 | DL arm CPU s | D-family unit CPU s (s1, s2, s3) | peak RSS GiB |
|--:|--:|--:|--:|--:|--:|--:|:--|--:|
| 22 | 4194271 | 64.0 | 1.798e9 | 2.033e9 | 4.146e9 | 7.6 | 31, 20, 20 | 0.41 |
| 23 | 8388571 | 67.0 | 4.760e9 | 5.237e9 | 9.532e9 | 21.1 | 52, 55, 61 | 0.74 |
| 24 | 16777183 | 70.0 | 9.390e9 | 1.036e10 | 1.906e10 | 44.6 | 124, 129, 107 | 1.45 |
| 25 | 33554383 | 73.0 | 1.866e10 | 2.062e10 | 3.824e10 | 92.0 | 239, 217, 252 | 2.87 |
| 26 | 67108819 | 76.0 | 3.680e10 | 4.076e10 | 7.642e10 | 193.5 | 623, 491, 499 | 5.72 |

### Exponents (OLS of ln mean cost on ln q; 95% two-level bootstrap, B = 4000) [pcc:unverified]

The bootstrap resamples units within each size and, independently for each size, one of the four
calibration rounds.

| arm | range | measure | e | 95% (units + calibration) | per-round e |
|:--|:--|:--|--:|:--|:--|
| **DL** | **k = 22..26** | **M-eq w=0 (decision)** | **1.0681** | **[1.0680, 1.0682]** | 1.0681-1.0682 |
| DL | k = 22..26 | M-eq w=1 | 1.0413 | [1.0412, 1.0413] | |
| DL | k = 22..26 | CPU s | 1.1446 | [1.1398, 1.1496] (units only) | |
| DL | k = 22..25 | M-eq w=0 (not the test range) | 1.1107 | [1.1106, 1.1109] | |
| DL | k = 17..26 | M-eq w=0 (prediction row) | 1.0837 | [1.0831, 1.0843] | 1.0835-1.0838 |
| DL | k = 17..22 | M-eq w=0 | 1.0443 | [1.0427, 1.0458] | |
| DL1 | k = 22..26 | M-eq w=0 | 1.0538 | [1.0536, 1.0539] | |
| D | k = 22..26 | M-eq w=0 | 1.1125 | [1.1105, 1.1145] | |

### DL / rho-neg (calibrated model, as the lead's `rho_reference` allows) [pcc:unverified]

The model is the frozen rule of `../analyze.py` section 4:

* **Steps:** 0.9175 sqrt(ell), the committed pooled constant.
* **Priced work:** 1.0354 GAE per step.
* **Unpriced work:** 3.037 ops per step, charged at w.
* **GAE:** taken from this pod's calibration.
* **Band:** c over [0.75, 1.15], crossed with the seed range and the calibration-round range.

| k | w = 0 | w = 0.1 | w = 1 |
|--:|:--|:--|:--|
| 22 | 0.00535 [0.00427, 0.00655] | 0.00468 [0.00373, 0.00573] | 0.00314 [0.00250, 0.00384] |
| 23 | 0.00493 [0.00393, 0.00604] | 0.00420 [0.00335, 0.00514] | 0.00251 [0.00200, 0.00307] |
| 24 | 0.00339 [0.00271, 0.00415] | 0.00289 [0.00231, 0.00354] | 0.00175 [0.00140, 0.00214] |
| 25 | 0.00236 [0.00188, 0.00288] | 0.00201 [0.00161, 0.00246] | 0.00123 [0.00098, 0.00150] |
| **26** | **0.00162 [0.00129, 0.00199]** | **0.00139 [0.00111, 0.00170]** | **0.00086 [0.00068, 0.00105]** |

* **Order:** DL is below rho-neg at all three weights and at every size, so the order holds across
  the §5a band.
* **Distance remaining to rho at k = 26:** a factor of about 620 at w = 0 and about 1170 at w = 1,
  in DL's favour.

**Measured negation rho (Addendum A).** This is a cross-check of the model constant only. It was
not used in the decision. [pcc:unverified]

* **k = 22:** 8 runs with `v2w2 rho 4194271 1 i i+1 neg`, all ok. steps/sqrt(ell) mean 0.969,
  range [0.204, 1.540]. About 590 CPU-s per run.
* **k = 23:** 8 runs, all ok. steps/sqrt(ell) mean 1.028, range [0.440, 1.818]. About 1531 CPU-s
  per run.
* **Pooled c:** 0.998 over 16 runs. That is inside the [0.75, 1.15] admissibility range and about
  9% above the model's 0.9175.
* **Effect on the ratio:** with the measured c, the k = 22, 23 ratios would be about 8% lower.
* **Limit:** 16 runs give a standard error on c of about 0.1, so this only confirms that the model
  is in range.
* **Run conditions:** the 16 runs ran in parallel on the 16-vCPU pod, so their CPU-s are not
  comparable with the sequential IC timings.

## Exact commands

```bash
cd research/leads_20261111/runs/C_quartic_k26
rustc --edition 2021 -O src/v2w.rs -o runpod/bin/v2w2      # sha256 cf55da04... (= ../bin/v2w2)
export RP_STATE=/tmp/claude-0/rp_quartic-k26-clean; K=research/runpod_kit_20261112/rp.sh  # (absolute path)
$K create bdr-quartic-k26-clean 16 cpu5c 30 && $K wait bdr-quartic-k26-clean              # pod c9gkalzltj9f42
$K put bdr-quartic-k26-clean runpod/bin/v2w2 v2w2         # also runjob.py, pod_run.sh, pod_low.sh (sha256 verified)
$K exec bdr-quartic-k26-clean 'bash pod_run.sh'           # main sequence (PROTOCOL.md order)
$K exec bdr-quartic-k26-clean 'bash pod_low.sh'           # Addendum A: waits for MAIN DONE; k = 17..21; then rho
runpod/fetch.sh                                           # copies out/ and rho/ to runpod/raw/
python3 runpod/analyze_pod.py > runpod/analysis_main.md   # writes runpod/results_pod.json
python3 runpod/plot_pod.py                                # fig_pod.{png,svg}
$K delete bdr-quartic-k26-clean                           # pod listing afterwards: no bdr-quartic-k26-clean
```

In `pod_run.sh`, every unit is `V2W_COMPACT=1 python3 runjob.py out/ic_<q>_<s>_D 3600 ./v2w2 ic <q> <s>
D 3.0 2.0 4 4`, and every calibration is `python3 runjob.py out/cal<r>_<q>_<c> 1200 ./v2w2 calib <q> <c>`.

## RunPod usage

| pod id | name | flavor | vCPU | memory quota | created / deleted (UTC) | hours | costPerHr | approx. cost |
|:--|:--|:--|--:|--:|:--|--:|--:|--:|
| c9gkalzltj9f42 | bdr-quartic-k26-clean | cpu5c | 16 | 32 GB | 18:11 / 20:08 | 1.95 | $0.56 | about $1.09 |

* **Deleted:** the pod was deleted at 20:08Z, and a pod listing afterwards contains no
  `bdr-quartic-k26-clean`.
* **Pod count:** one pod was used, and no other pod was touched.

## Deviations and caveats

1. **The pod is a 16-vCPU slice of a 32-CPU host.** I cannot see co-tenant load. The host load
   average was 2-14 at pod start, but that is the host's figure, not the pod's. The four
   calibration rounds agree to within 0.1% for k >= 20, which bounds the effect of any interference
   on the M-eq constants.
2. **The rho-neg reference is the calibrated model, not a matched measured rho at k = 24..26.** The
   16 measured runs at k = 22-23 only confirm that the constant is in range.
3. **The exponent depends on the window, because of the arithmetic step at the 64-bit wall**
   (see "How to read the exponent"). The frozen k = 22..26 window passes, and k = 22..25 would not.
   This was not chosen after seeing data: the window is the lane file's.
4. **Only 3 seeds per size.** The decision interval is narrow because the counts have little spread
   between seeds, not because there are many units.
5. **k = 17..21 (Addendum A) ran after the main sequence.** They used the same calibration pattern.
   They enter the prediction row only.
6. **Not done by this subagent:** the repository's resume checkpoint, `benchrec` record, S3 upload,
   research-kb receipts and validation gate. The coordinator owns those. No leaderboard, benchmark
   or docs files were touched.

## What is blocked

* **A validated row.** That needs a replay certificate, an isolated host (isolab) and a matched
  measured rho reference. RunPod gives a dedicated pod, not an isolab-certified host, and the AWS
  identity is rejected in this container (per `../RESULT.md`).
* **Measured rho-neg at k >= 24.** One run takes about 1.2 h of CPU at k = 24 and about 10 h at
  k = 26 on this CPU. It was not attempted.

## Next step

* **Arithmetic engineering at the wall.** A Montgomery or 2-limb `mulmod` for 64 < log2 ell < 80
  should cut the k = 22 -> 23 step (`r_lmul` 2.57 -> 3.70). It would be measured as an arithmetic
  constant-factor improvement with its own A/B.
* **Validation.** If the row should become validated, run the isolab job the lead names: k = 25, 26
  with a replay certificate, plus measured rho-neg at k = 23, 24, with this same binary.

## Files

* `PROTOCOL.md`: the frozen protocol, plus Addendum A.
* `pod_run.sh`, `pod_low.sh`, `runjob.py`, `fetch.sh`: the run.
* `analyze_pod.py`, `plot_pod.py`, `analysis_main.md`, `results_pod.json`, `fig_pod.{png,svg}`: the
  analysis.
* `raw/out/`: 30 IC units and 120 calibration records, each with `.jsonl`, `.err` and `.res.json`,
  plus `host.txt` and `progress.txt`.
* `raw/rho/`: 16 measured negation-rho runs.
* `bin/` is git-ignored. Rebuild it with the command above.
