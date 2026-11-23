# Signed norm branch: exact non-full-rank block

**Date:** 2026-11-13  
**Status:** exact desk certificate and prospective experiment; no native run or retained benchmark.  
**Standing:** every interpretation is `[pcc:unverified]`. This note makes no speed,
complexity, security, or novelty claim.

## Result

The universal full-rank statement proposed in
[`CONTINUATION_PROTOCOL.md`](CONTINUATION_PROTOCOL.md) is false as written
`[pcc:unverified]`: distinct abscissae and a large characteristic do not force
every fixed-cardinality coefficient block to have rectangular full rank.

For one block, let the variable indices be split into `L` and `R), let
`A subset L` and `B subset R` have fixed cardinalities, and fix whether the
target index belongs to the Laplace subset. After removing nonzero row and
column factors from the Vandermonde products, Laplace signs, target factors,
and ordinates, the coefficient block is rank-equivalent to

```
K[A,B] = product_{i in A, j in B} (x_j - x_i)^2.
```

This identity is useful independently of the counterexample: it reduces the
rank census to a source-bound product kernel while retaining an independent
full-coefficient reconstruction as a semantic check.

## Exact certificate

Take `d=5`, hence `v=2`. Keep the target out of the Laplace subset, use the
`a=b=1` block, and set

```
x_L      = (0, 3)
x_R      = (6, 2)
x_target = 9.
```

With ordinates set to one, the unsigned Vandermonde coefficient block
`det(U[S^c]) det(W[S])`, with rows indexed by the singleton chosen from
`L` and columns by the singleton chosen from `R`, is

```
[ -252   108 ]
[  378  -162 ].
```

Every entry is nonzero, while

```
(-252)(-162) - (108)(378) = 40824 - 40824 = 0.
```

The second row is `-3/2` times the first, so the block has rank one rather
than two over `Q`. Laplace signs and arbitrary nonzero ordinates apply only
invertible row/column sign and scale factors and do not change that rank.

Reducing the same certificate modulo any prime `p>7` preserves distinctness,
nonzero entries, and rank one. At `p=11` the displayed block is

```
[ 1  9 ]
[ 4  3 ],
```

whose determinant is zero and whose second row is four times the first.

## What this does and does not decide

This certificate clears the theorem-or-counterexample subgate in B1. It does
**not** advance the SAT/proof-size stage:

- it is a fixed `d=5` algebraic family, not a growing-rank family;
- it does not establish that these abscissae arise under the frozen natural
  curve/subgroup/support sampler;
- a one-dimensional defect in one block supplies neither the preregistered
  half-null rank gap nor a lower `log2(rank)` slope;
- it gives no positive branch finder and no end-to-end relation-collection
  improvement.

The earlier consecutive-node development check and this certificate are
consistent: rectangular full rank is a generic possibility, not a consequence
of distinctness alone. No inference about the frequency of rank defects is
made without a retained, replayable census.

## Next falsification panel

Build one native generator and an independent checker with the following
ordered gates.

1. **Identity replay.** Reconstruct every coefficient both from the original
   Laplace minors and from the scaled product kernel. Require identical ranks
   at the integer certificate and modulo frozen primes. Flip one factor,
   duplicate one abscissa, change target inclusion, and change block
   cardinality as mutation controls.
2. **Natural incidence.** On frozen public curve/subgroup/support instances,
   measure whether the certificate's cross-ratio condition or any larger
   singular minor occurs more often than in a support- and model-count-matched
   shuffled-abscissa arm. Ordinates remain an invariance control, not a null.
3. **Growing defect.** Search a preregistered ladder of `k` for an explicit
   public-input family whose natural balanced block ranks have both (a) at
   most half the independently-randomized-coefficient null rank on the two
   largest held-out sizes and (b) a lower `log2(rank)` slope by the frozen
   margin. Preserve every censored and negative cell.
4. **Positive use.** Only if gate 3 passes, construct the separator-message
   algorithm, extract positive branch witnesses, and charge setup, failed
   targets, output rank, norm/group replay, and final scalar recovery.
5. **SAT is conditional.** Export canonical byte-identical SAT/UNSAT instances
   only after a growing source-bound defect survives. Trimmed proofs alone do
   not count as positive relation finding.

An isolated singular minor, including the certificate above, is a diagnostic
and not a tensor breakthrough.
