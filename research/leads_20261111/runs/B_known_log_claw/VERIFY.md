# VERIFY: LEAD-ICB-tableB-binary-known-log-claw (independent skeptic, 2026-11-12)

All statements are [pcc:unverified]. BOUNDARY (BSGS-class) context unchanged. Nothing here was
edited except this file. Scratch outputs went to the session scratchpad, not to the repository.

## What I re-ran

- Binary: `target/release/ecbench`, sha256 `85318bc71e0b41071dae2ffabad0c502e6e407f418b25939afec6645b843f8ac`,
  which matches the hash in RESULT.md. HEAD is `6b7fc4520e`. `git diff --stat HEAD -- src/cryptanalysis/ecbench src/bin/ecbench.rs`
  is empty. The dirty tree files (`f4_fp.rs`, `gaudry_quartic_bin.rs`, `examples/jv_cover.rs`) belong to other lanes and are
  not on the ecbench path.
- `sha256sum -c specs/SHA256SUMS`: all 7 specs OK.
- Curve check: the f2m15 spec (modulus 0x8003, a=1, b=0x524b, r=16319, G=(0x7b33,0x6f56)) matches the
  `docs/curves/registry.json` entry for `icv1-f2m15-t131-ed7256f0`. Records report `registered=true`.
- `RAYON_NUM_THREADS=1 ecbench run --spec specs/main-f2m15.json` and `specs/main-f2m27.json` into scratch
  (`--cpus none --allow-busy --lock <private>`). Each run took about 2 s.
- **Result: bit-exact.** All 240 records per session match the committed ones in arm, round, target,
  recovered log, total_gae and every counter. Ratios of totals are KL/rho 1.4921 (f2m15) and 1.8155 (f2m27).
  A/A/KL is 0.9900 and 0.9330. My own band recomputation (all `*_uncharged` counters at weight w) gives
  1.492 / 1.512 / 1.616 (f2m15) and 1.815 / 1.866 / 2.102 (f2m27), the same as `ecbench compare`.

So the reported key numbers are confirmed as **ecbench's charged accounting**.

## Where the verdict departs from the preregistration

The lane file `lane_B_binary_ic.md` §1 `cost_band` says: "IC arm: counted group additions (build T,
queries r/(2T)); **unpriced: base generation**, table inserts, table lookups (emit counts; priced at 0,
0.1, 1 GAE each)". The `mechanism` and the [1.4, 2.4] prediction are derived from table + queries only.

ecbench charges base generation in full, as the `factor_base` phase: s scalar multiplications at about
15 to 32 GAE each. RESULT.md mentions this ("which the lead's mechanism left out"), but it keeps the
harness accounting as the w=0 figure and does not flag that this departs from the preregistered band.
Here is the recomputation from the committed records under the preregistered band: base-generation GAE
removed and the s base scalar multiplications priced at w per operation, all other `*_uncharged` at w.

| curve | harness w=0 / 0.1 / 1 | prereg band w=0 / 0.1 / 1 (known-log) | prereg band (A/A seed) |
|---|---|---|---|
| f2m15 | 1.492 / 1.512 / 1.616 | **0.883** / 0.947 / 1.283 | 0.848 / 0.907 / 1.222 |
| f2m18 | 1.619 / 1.654 / 1.832 | **1.030** / 1.109 / 1.516 | 0.939 / 1.007 / 1.359 |
| f2m21 | 1.740 / 1.766 / 1.891 | **1.185** / 1.257 / 1.605 | 1.152 / 1.220 / 1.551 |
| f2m24 | 1.763 / 1.798 / 1.964 | **1.271** / 1.348 / 1.715 | 1.249 / 1.325 / 1.681 |
| f2m27 | 1.815 / 1.866 / 2.102 | 1.440 / 1.523 / 1.913 | 1.320 / 1.392 / 1.726 |

Under the preregistered band:

- **Main prediction** ([1.4, 2.4] at w=0 on every instance): fails on 4 of 5 instances, and on 5 of 5
  for the A/A seed.
- **Falsifier** "ratio < 1.4 that survives an audit of the counted additions": in its trigger region on
  4 of 5 instances. A counter audit does not show a counting error. Table plus queries at about
  1.5 to 1.7 S is near the 1.414 floor. The low ratio comes from rho-neg's own S at small r (1.97 at
  f2m15, against its 0.886 asymptote). The prediction's "cannot go below ~1.6" assumed rho at its
  asymptote. In other words, the preregistered derivation is wrong at these sizes. That is a prediction
  failure, not support for "held".
- **±15% band sub-prediction**: fails badly (f2m15 +45%, f2m18 +47%), not just once at +15.8%.
- **Band order** (§5a): flips. At f2m15, known-log < rho-neg at w=0 and w=0.1 but > at w=1, so the two
  arms cannot be ranked there. RESULT.md's "rho-neg cheaper at w=0, 0.1 and 1 on every instance" holds
  only under the harness accounting.

Charging base generation is arguably the more honest cost: a known-log arm really must compute [k]G.
The harness numbers themselves are correct. But the verdict "HELD, main prediction held, one
sub-prediction missed" depends on an accounting choice the preregistration did not make, and the
report does not label that as a deviation. A correct statement would be: held under ecbench charged
accounting; under the preregistered cost band the main prediction fails on 4 of 5 instances and the
band order is not stable on f2m15. Either way the cause is rho-neg's small-r overhead, not a defect.

## Other checks

- **Thresholds not moved**: 3.0x, 10% A/A, 1.4 and ±15% are quoted correctly from the lane file.
- **Table size**: T = s^2 is about sqrt(r/2) (f2m15: 100 vs 90.3; f2m27: 3364 vs 3344), as specified.
- **A/A**: the point gap is 6.7% at f2m27 (< 10%), but its 95% CI [0.83, 1.05] does not exclude a gap
  greater than 10%. With 16 targets x 4 rounds, the A/A falsifier has little power at f2m27.
- **Pricing**: the w>0 band prices `scalar_additions_uncharged` (integer additions, about 231k at f2m27)
  and other counters beyond the lead's list. This inflates the w=1 drift. That is conservative and was
  reported, not hidden.
- **Controls**: the positive and negative control sessions exist with the stated outcomes. Not re-run
  (f2m15 main reproduced bit-exact on the same binary).
- **Code changes**: none. Confirmed.
- **Still blocked**: L2/EKS, an independent second-host replay, VALIDATION.json/benchrec/S3 and a pcc
  claim. My rerun is on the same host class and the same binary, so it is not an independent certificate.

## Verdict

The numbers reproduce bit for bit. The "held" verdict does not follow from the preregistered cost band.
It holds only under the harness's charged base generation, an undeclared accounting deviation. Under the
literal preregistered band, the main prediction fails on 4 of 5 instances. agrees = false.
