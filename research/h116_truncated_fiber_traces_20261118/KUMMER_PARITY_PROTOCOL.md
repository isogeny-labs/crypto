# H116 quadratic-Kummer parity protocol

Protocol ID: `H116-KUMMER-PARITY-001`

Filed after the fixed-ratio `IX1` ordering failure and before computing any
Kummer-cover signature. This changes both the observable and the candidate
algorithm. [pcc:unverified]

## Related-work boundary

Lauter and Stange, *The Elliptic Curve Discrete Logarithm Problem and
Equivalent Hard Problems for Elliptic Divisibility Sequences*, SAC 2008,
DOI `10.1007/978-3-642-04159-4_20`, gives ECDLP-equivalent EDS problems and
cites the Shipsey--Swart EDS reduction (`IACR 2008/444`). The latter has the
known special-order/extension obstruction already recorded in this branch.
Targeted Crossref and arXiv queries found work on EDS squares, perfect powers,
and pseudorandom elliptic sequences, but no exact generic-prime-field decoder
matching the bounded quadratic-cover subset family below. This search is a
prior-art screen, not a novelty proof. [pcc:unverified]

## Cover family and candidate algorithm

On each of the five retained small prime-order curves, use the first public
base point `P` and anchors `A_j=[j]P`, `1<=j<=8`. For each nonempty subset
`S` of size at most three, define the quadratic cover of the elliptic curve

```text
C_S: z^2 = product_{j in S} (x - x(A_j)).
```

At `Q=[k]P`, its split/inert signature is the Legendre symbol of the displayed
product. A zero value is a branch point and forces abstention. Test both fixed
polarities mapping split/inert to the low base-two digit `k mod 2`. There are

```text
2 * (C(8,1) + C(8,2) + C(8,3)) = 184
```

rules. An exact total rule could be iterated by replacing
`Q` with `[2^-1](Q-[d]P)`, so a genuine total parity decoder would recover the
scalar in logarithmically many public group operations. No abstention may be
silently repaired.

## Discovery panel and controls

The discovery panel is all 1,520 nonidentity scalars on the five retained
prime-order curves, one generator per curve. Rank rules by the one-sided exact
binary statistic `(2M-C)^2/C` when `2M>C`, where `M` is correct covered
predictions and `C` is coverage. Ties break by subset mask then polarity.

Use 4,096 deterministic parity-label shuffles within each curve. Every null
replicate repeats selection over all 184 rules; the primary plus-one p-value
compares its family-wise maximum to the observed maximum. Retain total-exact
and exact-on-covered counts, coverage, per-curve performance, branch-point
counts, the selected divisor subset, and a byte-identical replay. Bitset and
scalar reference scorers must agree in self-test.

## Decision boundary

Only a calibrated discovery association can nominate one unchanged rule for a
separately frozen large-curve OOS test. Exact-on-covered is not a total digit
oracle. A favorable finite test would still require a proof of totality,
genericity, and subexponential end-to-end cost. A null result rejects only this
bounded Kummer-cover family. [pcc:unverified]
