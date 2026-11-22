# H116 unchanged quotient-digit OOS experiment

Protocol: `H116-QUOTIENT-DIGIT-OOS-001`

Source/launch gate commit: `a777aa19b2`

Disposition: **the unchanged `IX1-argmin` association replicated, but the rule
is not an exact digit decoder**. [pcc:unverified]

## Frozen execution

The protocol, sources and launch gate were pushed before any affected cohort
moment was generated. The producer retained 4,560 targets in both native and
`lambda=2` models and reported:

```text
quotient OOS producer: targets=4560 records=9120 repeat_mismatches=0
```

The separate scorer admitted all 4,485 non-base-case targets, found zero
invariant mismatches across all sixteen native/isomorphic-model channels, and
applied only the discovery-selected `IX1-argmin` rule:

```text
quotient OOS scorer: targets=4485 coverage=4328 matches=845 p=0.000244 status=association_replicated
```

| measurement | result |
| --- | ---: |
| source targets | 4,560 |
| source model-target records | 9,120 |
| scored targets | 4,485 |
| curve/generator rows | 15 |
| repeated-jet mismatches | 0 |
| invariant mismatches | 0 |
| rule | `IX1-argmin` |
| covered targets | 4,328/4,485 (0.964994425864) |
| correct covered predictions | 845/4,328 (0.195240295749) |
| chance baseline | 1/6 (0.166666666667) |
| signed excess `6M-C` | 742 |
| standardized root | 5.044002313101 |
| null exceedances | 0/4,096 |
| plus-one empirical p | 0.000244081035 |
| exact decoder | no |

## Curve-level heterogeneity

| curve | coverage | matches | conditional accuracy | `6M-C` |
| --- | ---: | ---: | ---: | ---: |
| p=107, r=127 | 354/363 | 84 | 0.237288 | 150 |
| p=211, r=197 | 522/573 | 107 | 0.204981 | 120 |
| p=241, r=257 | 690/753 | 138 | 0.200000 | 138 |
| p=431, r=397 | 1,150/1,173 | 176 | 0.153043 | -94 |
| p=587, r=547 | 1,612/1,623 | 340 | 0.210918 | 428 |

Four curves have positive signed excess, while the p=431 curve is below the
one-sixth baseline. The combined replication is therefore not a uniform
per-curve law and is partly dominated by the largest p=587 cell. The fixed
row-preserving randomization nevertheless calibrates the exact pooled test
that was preregistered. [pcc:unverified]

The OOS producer evaluates the frozen formal jet rather than the large direct
extension-field fibers on these exact models. The zero coordinate-invariance
and repeat mismatches establish internal execution consistency, not the
remaining geometric identification. Prior direct and full-map controls are
supporting evidence only; an algorithmic claim would require exact-model
direct validation. [pcc:unverified]

## Retained artifacts

```text
c69de26d62be755c6cb27e793c2ed7bab20e7da066b77ed5abb027598ebd3016  QUOTIENT_DIGIT_OOS_MOMENTS.tsv
11386d5840d4404554d709423d7aba930937489ac86ac4412c8cdaa813538e25  QUOTIENT_DIGIT_OOS_LABELS.tsv
7c13c6d0da2f2828422fa48ce6cb0d18e05f08822e1f6a33a01cf6fd22f40cf8  QUOTIENT_DIGIT_OOS_RESULT.json
```

A second scorer invocation reproduced the result JSON byte-for-byte.

## What follows and what does not

This is a replicated ordering bias in one fixed public quotient feature. It is
not exact and therefore cannot be iterated as the conditional base-six
algorithm. A fixed prediction advantage can at most reorder a search tree; by
itself it does not make the number of surviving scalar paths subexponential.
The next obligations are to derive the selected channel algebraically, measure
the full correct-digit rank distribution and account for the resulting search
exponent. No generic-prime-field ECDLP or subexponential-runtime claim follows.
[pcc:unverified]
