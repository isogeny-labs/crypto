# H116 quotient-digit moment protocol

Protocol ID: `H116-QUOTIENT-DIGIT-MOMENT-001`

Filed before any quotient-digit score was computed. This protocol reuses the
already retained moment/label census from
`H116-INVARIANT-MOMENT-PREDICATE-001`; those raw values predate this protocol,
but no six-candidate quotient score has been inspected. The candidate family
is fixed from the prior sixteen invariant definitions, not selected from the
prior single-point best channels. [pcc:unverified]

## Conditional algorithm

Let `P` generate a prime-order subgroup of order `r>3`, let `Q=[k]P` with
canonical `0<=k<r`, and put `mu=6^-1 mod r`. For every `d in {0,...,5}` form
the public point

```text
R_d = [mu](Q-[d]P).
```

For the unique integer digit `d*=k mod 6`, writing `k=6a+d*` gives
`R_(d*)=[a]P` without a modular wrap. If a public rule identifies `d*`, replace
`Q` by `R_(d*)` and repeat. Scalars `k<6` are detected first by the six public
equalities `Q=[d]P`; all scored rows therefore have `k>=6` and every candidate
`R_d` is nonidentity.

An exact rule on a growing generic family would give complete base-six scalar
recovery after at most `ceil(log_6 r)` rule calls, plus ordinary public group
operations. The experiment tests a small fixed rule family only; it does not
assume that such a rule exists. [pcc:unverified]

## Frozen rule family

Use exactly the sixteen discriminant-normalized channels from
`H116-INVARIANT-MOMENT-PREDICATE-001`:

```text
IX_j = X_j^6 / D^(j+1),       j=0..7,
IY_j = Y_j^12 / D^(2*j+3),    j=0..7,
D = 4*A^3+27*B^2.
```

For each channel, evaluate its canonical base-field representative on the six
candidate points. The two rules are:

```text
argmin: predict the unique d with minimum representative;
argmax: predict the unique d with maximum representative.
```

A tie abstains and counts as a mismatch. This gives exactly 32 rules. No
weights, thresholds, combinations, channel selection, tie breaker or post-run
rule may be added.

## Frozen census and decisions

Use both retained curves, all three generator labels, and scalars `6..r-1`:
31 targets per label at `r=37` and 55 per label at `r=61`, hence 258 targets
in six rows. Recompute and require all native/`lambda=2` invariants to agree
before scoring.

Report for target and control labels: best match count and rate, coverage,
all best rule identifiers, exact-rule count, per-channel argmin/argmax match
counts, and per-curve exact-rule counts. A uniform positive requires the same
rule to cover and correctly classify all 258 targets. The p=31-only diagnostic
cannot promote a claim without unchanged p=47 survival.

Within each curve/generator row, deterministically shuffle the true digit with
a SplitMix64 Fisher--Yates permutation while retaining every quotient feature.
No rule should be exact on the shuffled control. The scorer self-test must use
synthetic six-candidate scores to show that unique argmin and argmax recover a
locked digit, while a tied extremum abstains.

All outcomes are retained. Even an exact small-fixture rule would be only a
candidate for a separately frozen out-of-sample cohort and a public-cost
audit. No decoder, generic-prime-field result, ECDLP algorithm, or
subexponential runtime is established by this protocol. [pcc:unverified]
