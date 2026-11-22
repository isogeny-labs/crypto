# H116 quotient-digit max-statistic calibration

Protocol: `H116-QUOTIENT-DIGIT-CALIBRATION-001`

Source/launch gate commit: `a9302285a7`

Disposition: **the unchanged `IX1-argmin` discovery statistic exceeds 4,090
of 4,096 row-preserving max-statistic null replicates; this admits an unchanged
out-of-sample test, not an ECDLP claim**. [pcc:unverified]

## Frozen result

The source checkpoint was pushed before the null distribution was generated.
The calibrator reconstructed the exact retained 32-rule match and coverage
arrays, passed its rational-comparison and deterministic-shuffle controls, and
completed all 4,096 replicates:

```text
h116 quotient digit calibration selftest: pass
quotient calibration: observed_z=4.320948 exceed=6/4096 p=0.001709 raw_exceed=32/4096
```

| measurement | result |
| --- | ---: |
| targets | 258 |
| rules included in each maximum | 32 |
| row-preserving null replicates | 4,096 |
| selected standardized rule | `IX1-argmin` |
| exact rational statistic | 19044/1020 |
| statistic value | 18.670588235294 |
| positive standardized root | 4.320947608488 |
| standardized exceedances | 6/4,096 |
| plus-one standardized empirical p | 0.001708567244 |
| observed raw maximum | 57 |
| raw-maximum exceedances | 32/4,096 |
| plus-one raw empirical p | 0.008054674152 |
| exact-rule null replicates | 0 |

The primary null-statistic quantiles were 4.319101 at 50%, 8.052941 at 90%,
9.508911 at 95%, and 13.338462 at 99%. The observed 18.670588 lies above the
99th percentile of this fixed family-wise null. The raw-maximum result is
weaker, as expected because it does not normalize differing tie coverage.
[pcc:unverified]

The plus-one probability is a randomization calibration conditional on these
six excluded-fixture rows. It is not a population p-value for prime-field
curves, and the selected rule is not exact: it correctly classifies 57 of 204
covered targets and abstains on 54 others. The only admitted next action is an
unchanged `IX1-argmin` test on curves not used for this discovery; no channel,
extremum, tie rule or statistic may be retuned. [pcc:unverified]

A second complete calibration invocation reproduced the JSON byte-for-byte.

## Retained result

```text
5c6c8a803c73cc78554169ff7eb92b4ce02903049ccd6faea1df5aa8ef419a72  QUOTIENT_DIGIT_CALIBRATION_RESULT.json
```

This evidence justifies an OOS association test only. It does not supply an
exact digit oracle, a generic ECDLP algorithm, or subexponential runtime.
[pcc:unverified]
