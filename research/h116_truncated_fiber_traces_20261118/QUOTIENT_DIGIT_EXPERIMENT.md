# H116 quotient-digit moment experiment

Protocol: `H116-QUOTIENT-DIGIT-MOMENT-001`

Source/launch gate commit: `127d94ff9d`

Disposition: **none of the 32 frozen quotient-digit rules is exact on either
excluded fixture**. [pcc:unverified]

## Conditional algorithm checked

For `Q=[k]P`, each digit candidate forms
`R_d=[6^-1](Q-[d]P)`. At the true `d=k mod 6`, this is exactly
`[(k-d)/6]P`; an exact public rule selecting that candidate would therefore
permit repeated base-six scalar recovery. The experiment asks whether a
unique canonical-representative argmin or argmax in one of sixteen fixed
moment-invariant channels supplies that rule. [pcc:unverified]

## Frozen execution

The source checkpoint was pushed before the first quotient feature was
scored. The self-test passed, the retained 576 model rows had zero recomputed
native/isomorphic invariant mismatches, and all 258 prescribed nontrivial
targets were admitted in six curve/generator rows. The exact scorer output was:

```text
h116 quotient digit scorer selftest: pass
quotient digit scorer: targets=258 best=57/258 exact=0 control_best=47/258 control_exact=0
```

| measurement | result |
| --- | ---: |
| source model records | 576 |
| quotient targets | 258 |
| rows | 6 |
| fixed rules | 32 |
| invariant mismatches | 0 |
| exact rules | 0 |
| p=31 exact rules | 0 |
| p=47 exact rules | 0 |
| cross-curve exact survivors | 0 |
| best target matches | 57/258 (0.220930232558) |
| best target rules | `IX0-argmin`, `IX1-argmin` |
| best shuffled-control matches | 47/258 (0.182170542636) |
| exact shuffled-control rules | 0 |

Ties abstain, so coverage differs by channel. `IX0-argmin` scored 57/246
covered targets (23.17% conditional accuracy); `IX1-argmin` scored 57/204
(27.94%). The largest conditional fraction was 40/136 (29.41%) for
`IY6-argmax`, which did not maximize the preregistered all-target match count.
These descriptive ratios do not change the frozen decision. [pcc:unverified]

The target/control gap is not an exact-oracle result and is not yet a
calibrated statistical signal: the protocol fixed only one shuffled-label
draw while selecting the best among 32 rules with unequal coverage. A
prospective multi-shuffle max-statistic calibration is required before using
the gap to justify a new rule family. A second invocation reproduced the
result JSON byte-for-byte. [pcc:unverified]

## Retained result

```text
441aa9a27264bd87445ddc42f9265b65763a7bb40c0a12b1c8999816b1affbee  QUOTIENT_DIGIT_RESULT.json
```

The result rejects the 32 exact extrema rules on these fixtures. It neither
rejects every quotient-candidate observable nor supplies a digit oracle,
generic ECDLP algorithm, or subexponential runtime theorem. [pcc:unverified]
