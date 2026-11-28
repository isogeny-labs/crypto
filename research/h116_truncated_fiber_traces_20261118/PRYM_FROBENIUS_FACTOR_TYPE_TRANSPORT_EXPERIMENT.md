# H116 Prym Frobenius factor-type transport experiment

Protocol: `H116-PRYM-FROBENIUS-FACTOR-TYPE-TRANSPORT-001`

Source/launch gate commit: `6b6cc18a15`

Disposition: **the complete two-prime degree-partition tuple is highly
target-specific, is not parity-consistent where it repeats, and does not
transport parity across curves**. [pcc:unverified]

The recomputed oracle passes every inherited control, 6,296 complete quartic
factorization/reconstruction checks, 15,240 brute irreducibility checks, and
37,776 comparisons between generic and independently written polynomial
evaluators.  The planted scorer is 1,520/1,520 and replay from the frozen
binary is byte-identical.

The five-bin factor-type histogram `(1111,112,13,22,4)` is
`(1292,1512,0,2044,1448)`.  The absence of type `1+3` is compatible with the
reciprocal root pairing `lambda -> p/lambda`; it is retained as observed
structure, not treated as a scalar signal.  The ordered tuple distinguishes
1,506/1,520 targets from their negations.

There are 1,492 distinct tuple keys among 1,520 targets.  Fourteen keys contain
both parities, involving 28 targets, so even the in-sample exact lookup is not
a parity function.  Its best descriptive within-key majority is 1,506/1,520.

The preregistered leave-one-curve-out exact-key decoder covers only 32 targets
and matches 16.  No total or covered exact decoder exists.  Every one of the
4,096 deterministic within-curve shuffles has statistic at least as large,
giving plus-one `p=1.0`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 2 / 2 |
| 197 | 0 / 2 |
| 257 | 8 / 8 |
| 397 | 2 / 10 |
| 547 | 4 / 10 |

This closes exact-key transport of the two-prime degree-partition tuple.  The
next lead leaves characteristic-polynomial summaries and uses target-tethered
ramification geometry directly: for each chiral line through `Q`, factor the
residual quadratic containing the other two branch points and test its signed
split character under controlled scalar multiples. [pcc:unverified]
