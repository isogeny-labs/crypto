# H116 quotient-digit max-statistic calibration protocol

Protocol ID: `H116-QUOTIENT-DIGIT-CALIBRATION-001`

Filed after the frozen quotient-digit result and before any multi-shuffle
statistic was computed. This is a calibration of the already fixed 32 rules;
it does not add or tune a decoder. [pcc:unverified]

## Inputs and reconstruction gate

Use the exact retained moment and label files plus
`QUOTIENT_DIGIT_RESULT.json`. Reconstruct all 258 targets and 32 predictions
from source data. Before calibration, require exact equality with the retained
per-rule match and coverage arrays:

```text
matches  = [57,24,57,16,52,39,51,12,56,11,45,18,47,39,45,37,
            30,19,37,30,48,29,34,47,18,26,20,38,16,40,29,30]
coverage = [246,192,204,196,196,186,206,156,218,178,188,208,202,202,184,188,
            214,164,200,136,204,184,194,200,172,176,204,168,198,136,176,208]
```

Any mismatch invalidates the run.

## Primary max statistic

For rule `j` with matches `M_j` and fixed coverage `C_j`, define its one-sided
standardized-excess square exactly as the rational

```text
S_j = max(0, 6*M_j-C_j)^2 / (5*C_j).
```

This equals the square of the binomial-standardized excess over accuracy
`1/6`, while avoiding floating-point comparisons. The primary statistic is
`T=max_j S_j`, thereby including selection over all 32 rules and accounting
for unequal tie coverage. Report all observed maximizers. The unstandardized
`max_j M_j` is a secondary statistic only.

## Frozen randomization control

Generate exactly 4,096 replicates. For replicate `q=0..4095`, independently
shuffle the true digits within each of the six curve/generator rows using
Fisher--Yates driven by SplitMix64 with seed

```text
0x116003 XOR ((q+1)*0x9e3779b97f4a7c15 mod 2^64)
         XOR generator_label XOR sum(bytes(curve_slug)).
```

This preserves each row's digit multiset, every prediction and every coverage
pattern. For each replicate compute the same primary maximum and secondary raw
maximum. The plus-one empirical exceedance probabilities are

```text
p = (1 + number(null statistic >= observed statistic)) / 4097.
```

Also report null median, 90th, 95th and 99th percentiles for both statistics,
and whether any shuffled replicate produces an exact rule.

The self-test must verify exact rational comparison, a known standardized
score, deterministic shuffle replay and label-multiset preservation. All
outcomes are retained. No p-value threshold promotes an ECDLP claim; a small
calibration value could only justify a new, separately frozen out-of-sample
test of the unchanged rule. [pcc:unverified]
