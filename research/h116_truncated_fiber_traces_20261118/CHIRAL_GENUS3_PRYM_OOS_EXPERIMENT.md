# H116 fixed Prym-discriminant out-of-sample experiment

Protocol: `H116-CHIRAL-GENUS3-PRYM-OOS-001`

Source/launch gate commit: `8113960084`

Disposition: **the unchanged discovery-selected rule does not replicate on
the fresh panel**. [pcc:unverified]

The recomputed oracle passes all controls: 1,147,299 extension square-table
checks, three extension curve and Frobenius-fixed censuses, six histograms,
4,596,152 extension coordinate-rescaling identities, 3,708 base traces,
3,708 extension traces, 3,708 Newton checks, 3,708 Prym factor/real-Weil
checks, 3,680 chirality swaps, and the 1,840/1,840 planted scorer.  The replay
is byte-identical.

The sole frozen rule `[0,2,0,2]`, offset two, covers 1,836/1,840 targets and
matches 598.  Its ternary excess is `3*598-1836=-42`, so the one-sided
statistic is zero.  Every one of 4,096 fixed-rule shuffles meets that statistic
and plus-one `p=1.000000`.  The rule is not exact and meets none of the three
association-replication criteria.

| subgroup order | correct / covered | ternary excess |
| ---: | ---: | ---: |
| 577 | 176 / 576 | -48 |
| 647 | 221 / 646 | 17 |
| 619 | 201 / 614 | -11 |

The geometric oracle remains nonconstant: the Prym discriminant vector
distinguishes 1,640/1,840 targets from their negations.  This result closes the
fixed raw-residue rule, not Prym Frobenius data in general.  The next lead
changes the observable to the splitting type of the Prym real-Weil polynomial
modulo auxiliary primes. [pcc:unverified]
