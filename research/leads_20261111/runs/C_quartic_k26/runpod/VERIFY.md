# VERIFY: quartic-k26-clean (LEAD-C-quartic-n4v2-k26 on one RunPod pod)

2026-11-12. Independent skeptic pass. I did not use the producer's scripts (`analyze_pod.py`, `compare.py`,
`analyze.py`). I wrote my own reader, M-eq pricing, OLS, two-level bootstrap and rho-model code in the scratchpad.
I read `analyze.py` only to get the frozen definitions: the phase list, M-eq = mul + r_inv inv + r_lmul lmul, and the
rho-model constants from `summary_supp.json`. Every statement below is **[pcc:unverified]**.

## Verdict

**I agree with the producer.** I re-derived every key number from the raw records to the reported precision. All
three frozen falsifiers were kept and none fires. Both numeric predictions are honestly reported as missed. I found
no source change. No `bdr-quartic-k26-clean` pod exists on the account. [pcc:unverified]

## Re-derived from `raw/` with my own code [pcc:unverified]

| quantity | producer | mine | match |
|:--|:--|:--|:--|
| verified records (`ok`, `k_found == k`, `fail == ""`) | 90/90 | 90/90; 1 attempt each, 0 failures | yes |
| calibration records | 120 | 4 rounds x 3 records x 10 sizes, all present | yes |
| DL exponent k = 22..26, M-eq w = 0 | 1.0681 [1.0680, 1.0682], per round 1.0681-1.0682 | 1.0681; per round [1.0681, 1.0682]; my bootstrap (seed 12345, B = 4000) [1.0680, 1.0682] | yes |
| DL k = 22..25 | 1.1107 | 1.1107, bootstrap [1.1106, 1.1109] | yes |
| DL k = 17..26 (prediction row) | 1.0837 [1.0831, 1.0843] | 1.0837 [1.0831, 1.0843] | yes |
| DL k = 23..26 (post hoc) | 0.984 | 0.9843 | yes |
| DL w = 1, k = 22..26; DL1 and D, k = 22..26 | 1.0413; 1.0538; 1.1125 | 1.0413; 1.0538; 1.1125 | yes |
| rho model: priced GAE/step, unpriced ops/step | 1.0354, 3.037 | 1.0354, 3.037 (from the summary_supp k = 22 neg row) | yes |
| DL / rho-neg k = 26, w = 0 / 0.1 / 1 | 0.00162 [0.00129, 0.00199] / 0.00139 [0.00111, 0.00170] / 0.00086 [0.00068, 0.00105] | identical | yes |
| DL / rho-neg k = 22..25 | table in RESULT.md | identical (k = 24 w = 0 upper bound 0.00416 against 0.00415, a rounding difference) | yes |
| DL arm CPU at k = 26 | 193.5 s | 193.7, 193.3, 193.6 s | yes |
| peak RSS at k = 26 | 5.72 GiB | 5.72 GiB, all 3 units | yes |
| measured rho-neg pooled steps/sqrt(ell) | 0.998 (16 runs) | 0.9984; k = 22 mean 0.969, k = 23 mean 1.028; SE 0.12 | yes |
| identity control | 27/27 units, 7938 fields | 27 units, 7938 non-timing fields, 0 differences; k = 26 has no local counterpart | yes |
| binary | sha256 cf55da04..., src last at 1a0957d6c0 | `runpod/bin/v2w2` = `../bin/v2w2` = cf55da04...; `src/*.rs` hashes equal `../raw/SHA256SUMS_v2w2.txt`; `git status` shows src unchanged | yes |
| schedule | interleaved CAL1, s1 asc, CAL2, s2 desc, CAL3, s3 asc, CAL4; then k = 17..21; then rho | the `start_utc` order in the `.res.json` files matches exactly; every rc is 0 and no job timed out | yes |

## Extra checks beyond the producer [pcc:unverified]

* **Local re-run of the cheapest unit.** I ran `RAYON_NUM_THREADS=1 V2W_COMPACT=1 ./v2w2 ic 4194271 1 D 3.0 2.0 4 4`
  with the same binary (sha256 cf55da04...) in this container. It took 69 s wall. All 3 arms were ok with
  `k_found == k`. I compared 294 non-timing fields against the pod's `ic_4194271_1_D.jsonl` and found 0 differences.
* **Measured rho against the model's per-step pricing.** I priced the 16 measured rho runs with the pod's pooled
  calibration. Priced work came to 1.0354 GAE per step and unpriced work to 3.036 ops per step, which matches the
  model. DL divided by measured mean rho cost, w = 0, gives 0.00507 at k = 22 and 0.0044 at k = 23. The model gives
  0.00535 and 0.00493, so the model is slightly conservative, in rho's favour.
* **Seed sensitivity.** Dropping any one seed leaves the k = 22..26 exponent at 1.0681.

## Thresholds and preregistration

* The kill rules and predictions in PROTOCOL.md are verbatim from `lane_C_prime_ext_cover.md`, lines 261-278:
  * exponent over k = 22..26, kill if the lower bound is above 1.10;
  * DL / rho-neg at k = 26, kill if the upper bound is above 0.01;
  * any unverified logarithm.

  The test window k = 22..26 is the lane file's, not one chosen after the data. The lane file's `rho_reference`
  allows a calibrated model plus measured rho at k <= 23, and that is exactly what was used.
* Addendum A (k = 17..21 and 16 rho runs) is outside the decision. Its records start after `MAIN DONE`
  (19:21Z onward), as the addendum says.

## Caveats a reader should keep (none overturns the verdict)

1. **The 95% intervals are tiny because the three seeds are almost deterministic in cost.** Dropping a seed changes
   nothing at 4 digits. The intervals therefore do not describe the spread over the instance population with any
   statistical power. The margin to the kill line (0.032) is still much larger than any plausible instance-to-instance
   spread seen here.
2. **The pass depends on the window.** k = 22..25 gives 1.1107, which would have been killed. The extra k = 26 point
   dilutes the one-time r_lmul step at the 64-bit wall (2.569 to 3.701). The producer says this plainly. The step is a
   host-dependent arithmetic constant: the local container measured r_lmul 3.15 at k = 23.
3. **Wording.** RESULT.md calls the factor of about 620 (w = 0) to about 1170 (w = 1) the "distance remaining to rho",
   "in DL's favour". At this toy size DL is already below the rho-neg model, so these are margins, not distance still
   to cover. The rho reference is a model at k = 24..26, not a matched measured rho, and the row is not validated.
4. **Not independently checked.** I took the `kG = Q` check on the logarithms from the binary's own `verify`; I did
   not re-check it with separate curve arithmetic. I checked that `k_found` equals the planted `k`.
5. **Pod cost.** I could not check the cost ($0.56/h x 1.95 h, about $1.09) against the API, because the pod is
   deleted. The start and end times in `pod_start_utc.txt` / `pod_end_utc.txt` (18:11:55Z and 20:08Z) agree with
   1.95 h.

## RunPod

I created no pod. I listed `GET https://rest.runpod.io/v1/pods` at about 22:20Z: 29 pods, none named
`bdr-quartic-k26-clean` and no id `c9gkalzltj9f42`. The one running `bdr-*` pod (`bdr-emin-exact-large-n`) belongs to
another task, and I did not touch it.

## Exact commands

```bash
python3 -I <scratch>/v/indep.py research/leads_20261111/runs/C_quartic_k26/runpod/raw   # own re-derivation
cd <scratch>/v/run && RAYON_NUM_THREADS=1 V2W_COMPACT=1 ./v2w2 ic 4194271 1 D 3.0 2.0 4 4 > ic22_1.jsonl  # local re-run
curl -sS -H 'Authorization: Bearer proxy-injected' https://rest.runpod.io/v1/pods            # pod listing
```

Here `<scratch>` is `/tmp/claude-0/-home-user/5d26ea47-4c79-5b59-bd9c-e47e6411a571/scratchpad`. These are scratch
files and are not committed.
