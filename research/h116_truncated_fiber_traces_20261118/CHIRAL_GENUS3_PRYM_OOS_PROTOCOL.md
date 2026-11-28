# H116 fixed Prym-discriminant out-of-sample protocol

Protocol ID: `H116-CHIRAL-GENUS3-PRYM-OOS-001`

Filed after the in-sample Prym result and its Lean coefficient proof were
committed and pushed at `1605610ad3`, and before enumerating any point or
computing any cover invariant on the three curves below. [pcc:unverified]

## Frozen candidate

Retain exactly the discovery-selected rule from
`H116-CHIRAL-GENUS3-PRYM-DIGIT-001`:

```text
observable = Prym real-Weil discriminant mod 3
feature order = (c=1,m=1), (c=-1,m=1), (c=1,m=2), (c=-1,m=2)
coefficients = [0,2,0,2]
offset = 2
prediction = 2 + 2*D_(-1,1) + 2*D_(-1,2) mod 3.
```

The selected discovery result has SHA-256
`a282c3c4e6d09de37213285d1813ee17c93fad751580b76871b3611afc0a4351`.
No coefficient, offset, multiplier, chirality, modulus, abstention rule or
observable may be changed in this experiment.  Score exactly one rule; do not
reselect a family.

## Fresh curve panel

Use all nonidentity targets on these three ordinary prime-field,
prime-order curves:

| slug | `p` | `A` | `B` | order `r` | trace |
| --- | ---: | ---: | ---: | ---: | ---: |
| `h116-prym-oos-fp607-a1-b7-r577` | 607 | 1 | 7 | 577 | 31 |
| `h116-prym-oos-fp617-a1-b32-r647` | 617 | 1 | 32 | 647 | -29 |
| `h116-prym-oos-fp631-a1-b5-r619` | 631 | 1 | 5 | 619 | 13 |

These contribute 1,840 targets.  The fields, curves and target scalars do not
occur in the five-curve discovery panel.  Selection used only public curve
order data; anomalous curves were excluded.  No Prym feature or digit score
was available at selection time.

## Controls and decision rule

Recompute every quadratic-extension square table, extension curve census,
Frobenius-fixed census, chiral histogram, coordinate-rescaling identity,
base/extension cover trace, Newton identity, Prym factor identity, real-Weil
bound and chirality swap from the frozen source.  Require a planted scorer to
match all 1,840 labels and require a byte-identical replay.

Use the frozen one-sided ternary statistic `(3*M-C)^2/C` when `3*M>C`.
Generate 4,096 deterministic within-curve label shuffles and score only the
fixed rule.  Association replication requires all of:

1. positive pooled ternary excess;
2. plus-one empirical `p <= 0.05` for the fixed rule;
3. positive ternary excess on at least two of the three curves.

Exact decoding is reported separately and requires 1,840/1,840 with no
abstention.  Association replication would still be only a lead, not a
generic identity or ECDLP algorithm.  Failure rejects this fixed rule on the
fresh panel, not every Prym invariant. [pcc:unverified]
