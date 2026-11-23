# H116 quotient-digit unchanged-rule OOS protocol

Protocol ID: `H116-QUOTIENT-DIGIT-OOS-001`

Filed before any quotient-digit moment or score was generated on the five
curves below. The rule was selected by the separately frozen excluded-fixture
calibration and is not changed here. [pcc:unverified]

## Frozen cohort

Reuse the already committed `COHORT_V2.json` in its locked order:

| slug | p | r | trace | torsion degree |
| --- | ---: | ---: | ---: | ---: |
| `h116-fp107-a1-b25-r127-d3` | 107 | 127 | -19 | 3 |
| `h116-fp211-a1-b148-r197-d4` | 211 | 197 | 15 | 4 |
| `h116-fp241-a1-b57-r257-d4` | 241 | 257 | -15 | 4 |
| `h116-fp431-a1-b111-r397-d3` | 431 | 397 | 35 | 3 |
| `h116-fp587-a1-b204-r547-d3` | 587 | 547 | 41 | 3 |

Use generator labels 1, 2 and 3 and every scalar `1..r-1`. Produce the fixed
jet moments in both the native and `lambda=2` coordinate models, repeat every
jet evaluation, and retain separate digest/moment and digest/scalar artifacts.
The source census is 4,560 targets and 9,120 model-target records. Scoring
excludes public base cases `k<6`, leaving 4,485 targets in fifteen rows.

The producer uses the already frozen jet formula. It does not perform the
large extension-field direct-fiber census in this OOS run. Prior evidence for
that formula consists of direct agreement on the p=31/p=47 controls and full
formal-map agreement on four separate OOS models; any positive here still
requires direct-map validation on the exact cohort models. [pcc:unverified]

## Unchanged rule

For `Q=[k]P` and each `d=0..5`, form

```text
R_d = [6^-1 mod r](Q-[d]P).
```

Compute only the previously selected invariant

```text
IX1(R_d) = X_1(R_d)^6 / (4*A^3+27*B^2)^2.
```

Predict the unique `d` minimizing its canonical representative in `0..p-1`.
A tie abstains. No other channel, extremum, normalization, tie rule, threshold
or combination may be inspected.

## Gates, controls and decisions

1. Require all 9,120 repeated jet evaluations to agree.
2. Recompute all sixteen invariant channels and require native/`lambda=2`
   equality before the unchanged rule is scored.
3. The scorer self-test must accept a unique synthetic minimum, abstain on a
   tie, and verify `k=6*(k/6)+(k mod 6)` over a fixed integer panel.
4. Report overall and per-curve targets, coverage, correct predictions,
   coverage rate, conditional accuracy and signed excess over `coverage/6`.
5. Generate 4,096 deterministic within-row digit shuffles with the exact seed
   schedule from `H116-QUOTIENT-DIGIT-CALIBRATION-001`, and report the plus-one
   one-rule exceedance probability for the same standardized-excess statistic.

The preregistered dispositions are:

- `exact_decoder_candidate`: coverage and correct predictions both equal
  4,485, invariant/repeat gates pass, and no shuffled replicate is exact;
- `association_replicated`: not exact, positive standardized excess, and
  plus-one permutation probability at most 0.01;
- `not_replicated`: all other valid outcomes;
- `invalid`: any gate or positive control fails.

Association replication is not digit decoding. Even an exact finite-panel
result would require a new growing-family test, exact-model direct validation,
public cost accounting and a proof before any ECDLP or runtime claim.
[pcc:unverified]
