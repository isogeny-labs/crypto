# H116 quotient-digit rank and search-cost protocol

Protocol ID: `H116-QUOTIENT-DIGIT-RANK-001`

Filed after the unchanged OOS result and exact selected-channel derivation,
but before any full rank histogram or recursive path statistic was computed.
This analyzes the existing `IX1` ordering; it adds no candidate rule.
[pcc:unverified]

## Deterministic rank

For each of the 4,485 OOS targets and its six quotient candidates, sort the
pairs `(canonical IX1 representative, digit)` lexicographically ascending.
The public digit tie-break makes the order total. Record the true digit's rank
in `1..6`, whether any values tied, and cumulative top-`t` inclusion for every
`t=1..6`.

As a reconstruction gate, unique-minimum coverage and matches must equal the
retained OOS values 4,328 and 845. All sixteen native/`lambda=2` invariant
channels must again agree.

## Recursive path survival

For each canonical scalar `k>=6`, repeatedly replace `k` by `floor(k/6)` until
the public base case `k<6`. At each non-base step use the rank already measured
for that curve/generator/scalar. Report:

- path depth histogram and maximum;
- maximum true rank along each path;
- for each `t=1..6`, number and rate of complete true paths whose every rank is
  at most `t`;
- the same complete-path counts per curve.

A top-`t` exhaustive recursion has at most `t^L` leaves at depth `L`. Since
`6^L` is the scalar scale, report the fixed exponent `log(t)/log(6)` for each
`t`. Any fixed `t>1` is `r^c` with constant `c>0`, not `r^o(1)`; `t=1` is the
only fixed-width polynomial case, and it requires exact rank one at every
step. No empirical miss may be silently repaired.

## Frozen rank control

Use the same 4,096 deterministic within-row digit shuffles as the OOS test.
For each replicate assign the shuffled digit's rank at every target and compute
the exact integer reciprocal-rank score

```text
MRR60 = sum(60/rank),   rank in 1..6.
```

The primary plus-one probability compares shuffled `MRR60` at least as large
as observed. Also report shuffled top-one and top-two exceedance counts. The
self-test must verify lexicographic ties, the exact reciprocal weights
`60,30,20,15,12,10`, and a synthetic recursive path whose required branching
width is known.

All outputs are retained. A favorable constant-rank distribution is a search
ordering heuristic, not an exact digit oracle or a subexponential ECDLP result.
[pcc:unverified]
