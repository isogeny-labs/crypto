# H116 growing-radix quotient-rank protocol

Protocol ID: `H116-QUOTIENT-MULTIRADIX-001`

Filed after the frozen radix-six rank experiment, before computing any rank at
a new radix. This changes the candidate mechanism's arity and tests a necessary
scaling condition for a subexponential quotient recursion. It does not add a
fitted selector. [pcc:unverified]

## Candidate and frozen panel

Use the retained five-curve OOS moment and label files without regeneration.
For every radix

```text
B in {2,3,4,5,6,8,12,16,24,32,48,64}
```

and every canonical scalar `k` with `B <= k < r`, form the `B` public points

```text
R_d = [B^-1] (Q - [d]P),  0 <= d < B.
```

All subgroup orders in the panel are prime and exceed 64, so the inverse
exists. For the true digit `d=k mod B`, the group identity gives
`R_d=[floor(k/B)]P`. Rank all candidates lexicographically by
`(canonical IX1 representative, d)` ascending. The digit tie-break is public
and total. No result at a new radix may be inspected before the exact source
and input hashes are pushed.

## Measurements

For every radix retain:

- the complete true-rank histogram and cumulative top-width counts;
- mean and normalized mean rank, median, 90th and 99th percentiles;
- all-value and minimum tie counts;
- per-curve target count, rank sum and normalized mean rank;
- for each starting scalar, the maximum true rank along the exact recursion
  `k -> floor(k/B)`, and complete-path survival for every width `1..B`;
- maximum recursion depth and maximum required width.

The cross-radix primary scaling statistic is the ordinary least-squares slope
of `log(mean true rank)` on `log(B)`. Linear rank growth gives a slope near
one; bounded or polylogarithmic rank would have asymptotic slope zero. This
small fixed panel can only measure a finite trend, not establish an asymptotic.

## Frozen controls and reconstruction

Before admitting new values, recompute zero mismatches across all sixteen
native/isomorphic invariant channels. At `B=6`, exactly reconstruct the prior
rank histogram `870,727,738,763,729,658` and complete-path counts
`355,744,1460,2345,3437,4485`.

For each radix and each of 4,096 replicates, shuffle the true digit labels
within each fixed curve/generator row using the preregistered deterministic
seed family. Count null rank sums no larger than observed and null
top-`ceil(log2 B)` counts at least as large as observed. Compute the same
cross-radix slope for every replicate and count null slopes no larger than the
observed slope. Use plus-one empirical probabilities throughout.

## Decision boundary

A finite sublinear slope is only a lead. A complete fixed-width rule requires
the maximum path rank to stay within that width; any observed miss is retained.
For a growing-radix recursion with `B=exp(sqrt(log r))`, a polylogarithmic
complete width would yield a conditional subexponential tree, while width
`B^c` for fixed `c>0` would not. No rank association, finite slope or shuffled
control by itself proves completeness, an asymptotic bound, or an ECDLP
speedup. [pcc:unverified]
