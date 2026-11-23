# H116 quotient-digit calibration source gate

Protocol `H116-QUOTIENT-DIGIT-CALIBRATION-001`, its calibration source and
this gate are committed and pushed before any affected multi-shuffle statistic
is computed. The retained observations and raw inputs necessarily predate this
calibration and are bound below. [pcc:unverified]

```text
b89ed14710bedc759015803ca52f380ff891c76ac0f3fc7bdf51d845c363fb55  QUOTIENT_DIGIT_CALIBRATION_PROTOCOL.md
f61f25b5351def91d143646a0aaf00eb416c0b923c7baacce70b0acfe58c0d53  QUOTIENT_DIGIT_CALIBRATION_LAUNCH.json
4404269b5a617b25321d941a37db05c38aaa2ea356ff37fc7a4114bc606e2ff9  h116_quotient_digit_calibrate.rs
52b28350a37483547165cd6b7efc115005aa3453b1fa802ad1cb85f8177207a7  h116_quotient_digit_scorer.rs
8d9791b149a5815c35336c8b7219e71806d6e3376286b7eacb414586d06d2135  INVARIANT_MOMENT_PREDICATE_MOMENTS.tsv
7e892aac8b67704ebb7c78f30ffc5e6c6ae8fafb5f960dc962c2da49dbd9f338  INVARIANT_MOMENT_PREDICATE_LABELS.tsv
441aa9a27264bd87445ddc42f9265b65763a7bb40c0a12b1c8999816b1affbee  QUOTIENT_DIGIT_RESULT.json
```

The calibrator textually includes the exact frozen scorer only to reuse its
field parsing, invariant and extrema definitions. Its new code independently
reconstructs and gates the retained match/coverage arrays, implements exact
rational max-statistic comparison, and generates the fixed 4,096 rowwise
permutations. No candidate rule is added. [pcc:unverified]

The program compiled under warnings-as-errors with
`rustc 1.100.0 (0ef1bc386 2026-11-09)` and its pre-data self-test passed:

```text
/workspace/.h116-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
  --edition=2021 -O -D warnings h116_quotient_digit_calibrate.rs \
  -o /tmp/h116_quotient_digit_calibrate
/tmp/h116_quotient_digit_calibrate --selftest
h116 quotient digit calibration selftest: pass
```

Diagnostic binary hash:

```text
0eda5dafacafa8092f0e33a402cbce8a42933beaff5968caec96d3ad3cbc7a55  /tmp/h116_quotient_digit_calibrate
```

No affected permutation replicate or exceedance count existed when this gate
was written. The first permitted calibration is the exact launch command in
the first pushed commit containing this gate. This predicts no calibration
outcome and establishes no decoder or ECDLP claim. [pcc:unverified]
