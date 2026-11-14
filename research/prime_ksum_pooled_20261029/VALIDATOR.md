# Validator report: prime_ksum_pooled_20261029 (VAL-20261112-ad875d)

**Disposition: validated** (status `pass`, independent recompute, Python 3 stdlib, no ecbench run). Results commit `a0ff77014f42a1c96106a833f1e99d21f8e85e15`; repository HEAD at start `14cbbecc383012699e79728d5d993de8d7697277` (moved to `55713d2001bfbced31461f7b94e96992e75b7177` by other sessions; the study directory and the gate script are unchanged across). All numbers are `[pcc:unverified]` until a pcc claim records them.

Scope of the validation: the counted-unit claim (ecbench GAE, §5a band) under the study's own pricing reading. The study's UNQUALIFIED status (one host, no second-host replay of this holdout, L0 isolation, no external tool) is unchanged by this report.

## Raw data binding

| session | records.jsonl sha256 | git blob at a0ff77014f | records | binds |
|:--|:--|:--|--:|:--|
| train-P20 | `f6b450327de721c5…` | `feccf56d405362ef8eed6b21e980a62ff407ad39` | 1224 | yes |
| train-P24 | `8f94b334389b90b5…` | `dcfa73e9f96f9258b216f018d7505dac10c47e04` | 1224 | yes |
| train-P28 | `71b56035fd166590…` | `c1f398a3254da5c25573d692ba816cd0b7b64c12` | 1224 | yes |
| train-P32 | `d4e5539588c3f5a5…` | `3522a798f398b37cda70b73655a1a750e1934d4b` | 1224 | yes |
| holdout-P20 | `4d0406a9f46f8e43…` | `61bc1eede160a16586de7bf60b4aaf1335f1a55c` | 2016 | yes |
| holdout-P24 | `72408ce5bc266ddd…` | `20f7ac96e5e2118410f5fdbc1e0d2feb34ebaf73` | 2304 | yes |
| holdout-P28 | `d1b005cbf9ec2dac…` | `012acebe991ea2e912e74a6392201af3d155297a` | 2304 | yes |
| holdout-P32 | `fc4c325893dd52ff…` | `eaab476a2706e1701b051c2b20692eeba11317b1` | 2304 | yes |

Each recovered file hashes to its `.artifacts.json`, to `records_sha256` in `session.json`, and to the git blob content at a0ff77014f; the git blob id was recomputed from the bytes.

## 1. Certificates ([d]G = Q, own affine arithmetic)

- Measured verified holdout records: **5952 / 5952** satisfy [d]G = Q; failures 0.
- Also: holdout warm-ups 2976/2976, train measured 3264/3264, train warm-ups 1632/1632. 480 workloads, 0 with conflicting logs across arms.
- Curves: a, b are not in the record. They were solved from G and one target, then required to hold for G and every other target of the curve. j(a, b) equals the ICV1 j; p and r are prime (Miller–Rabin); r = p + 1 − t; Hasse holds; cofactor 1; [r]G = O and [r]Q = O; (a, b, G, r) equal the curve registry entry, on all 24 curves. j is never 0 or 1728, so negation is the only fold, and rho-neg used it (automorphisms_used 2) on all 1,152 holdout records.

## 2. Pooled paired ratios (Σ S_B / Σ S_A over 192 (workload, round) pairs)

Pricing: S(w) = (total_gae + w·U)/√r. C = n_C = 0 (no pinned calibration). U = the counts behind `cost.unpriced`. **The study used the pre-2026-11-08 reading** (`table_insert_attempts` not priced). Every committed comparison equals this reading to 1.6e-15 and differs from the 2026-11-08 reading by up to 25%, and `band.json` has no `priced_table_work` key.

**pre-2026-11-08 reading (the study's)**

| bits | ksum5-sel/inc-sel w=0 / 0.1 / 1 | ksum4-sel/inc-sel w=0 / 0.1 / 1 | ksum5-sel/rho-neg w=0 / 0.1 / 1 |
|--:|:--|:--|:--|
| 20 | 0.6998 / 0.6854 / 0.6151 | 0.6324 / 0.6333 / 0.6373 | 5.9929 / 5.8677 / 5.2565 |
| 24 | 0.4464 / 0.4477 / 0.4536 | 0.4521 / 0.4580 / 0.4858 | 5.8162 / 5.8351 / 5.9244 |
| 28 | 0.3243 / 0.3191 / 0.2934 | 0.3364 / 0.3372 / 0.3413 | 7.3320 / 7.1215 / 6.1597 |
| 32 | 0.2171 / 0.2177 / 0.2204 | 0.2529 / 0.2559 / 0.2708 | 7.3391 / 7.2227 / 6.6965 |

**2026-11-08 default (table work priced)**

| bits | ksum5-sel/inc-sel w=0 / 0.1 / 1 | ksum4-sel/inc-sel w=0 / 0.1 / 1 | ksum5-sel/rho-neg w=0 / 0.1 / 1 |
|--:|:--|:--|:--|
| 20 | 0.6998 / 0.6776 / 0.5906 | 0.6324 / 0.6243 / 0.5922 | 5.9929 / 6.0451 / 6.2999 |
| 24 | 0.4464 / 0.4402 / 0.4171 | 0.4521 / 0.4511 / 0.4474 | 5.8162 / 6.0085 / 6.9165 |
| 28 | 0.3243 / 0.3176 / 0.2926 | 0.3364 / 0.3347 / 0.3287 | 7.3320 / 7.4688 / 8.0936 |
| 32 | 0.2171 / 0.2155 / 0.2095 | 0.2529 / 0.2534 / 0.2552 | 7.3391 / 7.5525 / 8.5164 |

Matches the README at w = 0: 0.700/0.446/0.324/0.217 and 0.632/0.452/0.336/0.253; distance to rho 6.0/5.8/7.3/7.3 and to BSGS 7.6/6.5/7.2/7.1. The selection recomputed from train is identical under both readings: inc 64/162/407/1024, inc-w0 64/115/289/727, ksum4 10/14/21/30, ksum5 9/11/15/19, k* = 5.

## 3. §5a band per curve (mean S, ic = ksum5-sel, rho = rho-neg)

**pre_2026_11_08_study_reading**

| curve | w=0 ic / rho / inc-sel | w=0.1 ic / rho / inc-sel | w=1 ic / rho / inc-sel | rho < ksum5 < inc every w |
|:--|:--|:--|:--|:--|
| icv1-fp20-t375-aef80152 | 7.145 / 1.128 / 10.579 | 7.658 / 1.224 / 11.587 | 12.277 / 2.085 / 20.657 | yes |
| icv1-fp20-t457-f60a16c1 | 7.590 / 1.271 / 10.741 | 8.036 / 1.378 / 11.611 | 12.047 / 2.341 / 19.444 | yes |
| icv1-fp20-tm1079-bf1bb5fd | 7.689 / 1.347 / 10.706 | 8.103 / 1.460 / 11.506 | 11.826 / 2.478 / 18.712 | yes |
| icv1-fp20-tm1141-c50333f6 | 7.412 / 1.233 / 10.610 | 7.878 / 1.336 / 11.509 | 12.074 / 2.271 / 19.594 | yes |
| icv1-fp24-t5097-1f48e65c | 6.557 / 1.079 / 14.487 | 7.200 / 1.177 / 15.851 | 12.989 / 2.056 / 28.129 | yes |
| icv1-fp24-tm1215-254f2c18 | 6.156 / 1.064 / 13.681 | 6.768 / 1.161 / 14.987 | 12.270 / 2.034 / 26.737 | yes |
| icv1-fp24-tm2615-5e182037 | 6.343 / 1.152 / 14.929 | 6.849 / 1.255 / 16.150 | 11.406 / 2.186 / 27.138 | yes |
| icv1-fp24-tm3365-ba8f1013 | 6.415 / 1.084 / 13.965 | 7.051 / 1.183 / 15.268 | 12.776 / 2.070 / 26.991 | yes |
| icv1-fp28-t12791-9e92c6c2 | 6.773 / 0.915 / 21.048 | 7.275 / 1.004 / 22.974 | 11.798 / 1.803 / 40.303 | yes |
| icv1-fp28-t20259-8fb841e7 | 7.065 / 0.883 / 21.850 | 7.482 / 0.968 / 23.547 | 11.233 / 1.735 / 38.814 | yes |
| icv1-fp28-t24271-da5b2687 | 6.760 / 1.016 / 21.285 | 7.242 / 1.115 / 23.209 | 11.581 / 2.006 / 40.525 | yes |
| icv1-fp28-tm5475-0f58a032 | 7.231 / 0.982 / 21.621 | 7.651 / 1.077 / 23.190 | 11.437 / 1.932 / 37.310 | yes |
| icv1-fp32-t8395-a5210489 | 6.908 / 0.972 / 32.311 | 7.475 / 1.068 / 34.932 | 12.572 / 1.936 / 58.520 | yes |
| icv1-fp32-tm14249-af783d1f | 6.972 / 0.937 / 31.755 | 7.573 / 1.030 / 34.369 | 12.982 / 1.866 / 57.893 | yes |
| icv1-fp32-tm30557-158f1cf7 | 6.911 / 0.983 / 32.347 | 7.493 / 1.080 / 35.051 | 12.737 / 1.959 / 59.387 | yes |
| icv1-fp32-tm70799-e5f3e6f2 | 7.158 / 0.917 / 32.322 | 7.694 / 1.008 / 34.565 | 12.516 / 1.826 / 54.756 | yes |

**table_work_priced_2026_11_08_default**

| curve | w=0 ic / rho / inc-sel | w=0.1 ic / rho / inc-sel | w=1 ic / rho / inc-sel | rho < ksum5 < inc every w |
|:--|:--|:--|:--|:--|
| icv1-fp20-t375-aef80152 | 7.145 / 1.128 / 10.579 | 7.868 / 1.224 / 12.013 | 14.375 / 2.085 / 24.919 | yes |
| icv1-fp20-t457-f60a16c1 | 7.590 / 1.271 / 10.741 | 8.284 / 1.378 / 12.116 | 14.533 / 2.341 / 24.495 | yes |
| icv1-fp20-tm1079-bf1bb5fd | 7.689 / 1.347 / 10.706 | 8.366 / 1.460 / 12.041 | 14.460 / 2.478 / 24.063 | yes |
| icv1-fp20-tm1141-c50333f6 | 7.412 / 1.233 / 10.610 | 8.114 / 1.336 / 11.987 | 14.429 / 2.271 / 24.377 | yes |
| icv1-fp24-t5097-1f48e65c | 6.557 / 1.079 / 14.487 | 7.404 / 1.177 / 16.576 | 15.030 / 2.056 / 35.377 | yes |
| icv1-fp24-tm1215-254f2c18 | 6.156 / 1.064 / 13.681 | 6.958 / 1.161 / 15.661 | 14.169 / 2.034 / 33.478 | yes |
| icv1-fp24-tm2615-5e182037 | 6.343 / 1.152 / 14.929 | 7.085 / 1.255 / 16.986 | 13.761 / 2.186 / 35.501 | yes |
| icv1-fp24-tm3365-ba8f1013 | 6.415 / 1.084 / 13.965 | 7.250 / 1.183 / 15.972 | 14.759 / 2.070 / 34.034 | yes |
| icv1-fp28-t12791-9e92c6c2 | 6.773 / 0.915 / 21.048 | 7.604 / 1.004 / 24.107 | 15.085 / 1.803 / 51.634 | yes |
| icv1-fp28-t20259-8fb841e7 | 7.065 / 0.883 / 21.850 | 7.867 / 0.968 / 24.873 | 15.082 / 1.735 / 52.081 | yes |
| icv1-fp28-t24271-da5b2687 | 6.760 / 1.016 / 21.285 | 7.577 / 1.115 / 24.366 | 14.938 / 2.006 / 52.097 | yes |
| icv1-fp28-tm5475-0f58a032 | 7.231 / 0.982 / 21.621 | 8.048 / 1.077 / 24.557 | 15.403 / 1.932 / 50.982 | yes |
| icv1-fp32-t8395-a5210489 | 6.908 / 0.972 / 32.311 | 7.815 / 1.068 / 36.852 | 15.980 / 1.936 / 77.725 | yes |
| icv1-fp32-tm14249-af783d1f | 6.972 / 0.937 / 31.755 | 7.905 / 1.030 / 36.237 | 16.297 / 1.866 / 76.579 | yes |
| icv1-fp32-tm30557-158f1cf7 | 6.911 / 0.983 / 32.347 | 7.827 / 1.080 / 36.934 | 16.078 / 1.959 / 78.216 | yes |
| icv1-fp32-tm70799-e5f3e6f2 | 7.158 / 0.917 / 32.322 | 8.069 / 1.008 / 36.676 | 16.260 / 1.826 / 75.859 | yes |

Order holds on 16/16 curves at every weight under both readings: rho-neg < ksum5-sel < inc-sel at w = 0, 0.1, 1 on all 16 holdout curves under both readings; also ksum4-sel < inc-sel, ksum5-sel and ksum4-sel < inc-w0 (24-32 bits), and bsgs-neg < ksum5-sel, ksum4-sel everywhere. rho is ahead: distance remaining, not a beat.

## 4. Agreement with the committed outputs and the preflight (opened after the recompute)

- 150 `analysis/compare-*.json`: max relative difference 1.6e-15 (ratios), 1.6e-15 (band triplets), 2.0e-15 (linearised-t ci95). Against the 2026-11-08 reading: 0.252.
- `analysis/band.json`: 124 rows, max rel 6.5e-16, charged/native sums equal in integers; 420 ranks, 0 mismatches.
- `preflight.json` max rel 0.0, `preflight-tablework.json` 0.0, `gae_band_per_curve.json` 0.0 (exact float equality).
- README brackets equal the committed ci95_bootstrap (ecbench two-stage cluster bootstrap); the validator's two-stage bootstrap with its own RNG lands within 0.002 of each. ecbench's primary ci95 (linearised t) is narrower and matches the validator's linearised interval to 2.0e-15.

## Findings

- Pricing reading: the study's committed numbers are the pre-2026-11-08 reading. Under the current default (table_insert_attempts priced) no rank or registered verdict changes, but every w > 0 figure moves; the w = 1 distance to rho grows to 6.30x-8.52x. Cite with the reading label.
- Binding: session.json records git_commit = null and git_dirty = null; the measurement binary (fc86ee11, matches BINARY.sha256 and every record) is pinned by hash only. The replay certificate attributes it to 26249b4 plus an uncommitted diff; counts replay identically with a clean build of 6edb2cb36d.
- A/A control: rho-neg-aa equals rho-neg in every counted field on 768 of 768 holdout pairs (deterministic method, same seeds), so in the counted unit it is a duplicate, not a noise estimate; band.json's 'unresolved inside the band' for that pair is a tie. It only informs wall time, which the study does not admit.
- Intervals: README brackets are ecbench's two-stage cluster bootstrap (labelled correctly); the primary linearised ci95 is narrower. No registered verdict depends on which is used (primary upper bounds at w = 0 are at most 0.465 at 24-32 bits; test (d) at 20 bits is unresolved under both).
- The README's 'derived, not a direct comparison' inc-w0/inc-rec values (0.92, 0.88, 0.88) are algebraically identical to the direct pooled ratio of totals (0.9191, 0.8804, 0.8737), because every pair is complete; they lack only an interval.
- Scope disclosed by the study and unchanged: one host (UNQUALIFIED), isolation L0 on every record (counted unit only), no second-host replay of this holdout, no external ECDLP tool, 20-bit incumbent pick on the grid edge, interval scope = workloads (between-curve variance carried only by the per-curve test).
- m = 83 / ECC2K-130 disclosure rules: not applicable (prime-field curves, 20-32 bits).

## Discrepancy

- README.md section 4 (c), third bullet (and the same figures behind section 1 Q3 '8-13%'): claimed 0.92, 0.88 and 0.88 at 24, 28 and 32 bits (inc-w0 relative to inc-rec, derived from the k-sum ratios); records give [0.9191, 0.8804, 0.8737]. At 32 bits the records give 0.8737 (prints as 0.87), not 0.88; the README value matches dividing the rounded ratios 0.217 / 0.248 = 0.875. Off by 0.006 absolute in a secondary, explicitly derived remark; no registered test, verdict or headline number uses it, and the '8-13%' range in section 1 is consistent with 0.874. Correction owed in the README: 0.87 (or 0.874) at 32 bits.

## Gate checker

```
check_validator_report(base=/home/user/crypto/research/prime_ksum_pooled_20261029): OK, no errors
raw_data entries: 16 headline entries: 76 status: pass independent: True
```

`scripts/validation_gate.py:check_validator_report` was run with base = `research/prime_ksum_pooled_20261029`. The `raw_data` paths are the `.artifacts.json` manifests and `session.json` files, because `records.jsonl` is not in the work tree; each entry nests the records' repo path, sha256 and git blob at a0ff77014f.

## Files

`/tmp/claude-0/-home-user/89d98138-21ce-5c29-8de4-5fe8ac9e919c/scratchpad/ksum/validator/`: check_hashes.py, recompute.py, integrity.py, compare_committed.py, bootstrap2.py, sensitivity.py, build_report.py, write_md.py, gate_check.py and their outputs (hashes.json, recompute.json, integrity.json, compare_committed.json, bootstrap2.json, sensitivity.json, freshness_grep.txt, gate_check.txt), validator-report.json (sha256 `00043afd39bbfbe99e0bdcdb6e2d48f3b679709fbb5f243f8d710b26125e4778`).
