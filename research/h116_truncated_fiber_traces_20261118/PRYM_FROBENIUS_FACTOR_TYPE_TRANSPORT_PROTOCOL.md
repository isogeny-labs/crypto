# H116 Prym Frobenius factor-type transport protocol

Protocol ID: `H116-PRYM-FROBENIUS-FACTOR-TYPE-TRANSPORT-001`

Filed after the full-Weil linear-factor-presence family was measured null and
before evaluating any complete factor-degree tuple against scalar parity.
[pcc:unverified]

## Related-work and novelty boundary

For `ell != p`, the irreducible factor degrees of the Prym Weil polynomial
modulo `ell` give the Frobenius orbit lengths of eigenvalues on the
four-dimensional mod-`ell` representation.  Rybakov, *Finite group subschemes
of abelian varieties over finite fields*, Finite Fields Appl. 29 (2014), DOI
`10.1016/j.ffa.2014.04.001`, relates factorization of Weil polynomials to
finite group subschemes.  This experiment retains strictly more information
than the preceding root-presence bit, but less than a complete Frobenius
conjugacy or Jordan class.

Targeted repository, ePrint, arXiv and journal searches found no prime-field
ECDLP scalar-parity decoder based on cross-curve transport of the ordered
chiral Prym factor-type tuple below.  This is a prior-art screen, not a novelty
proof. [pcc:unverified]

## Observable

For every admitted chiral genus-three double cover and each auxiliary prime
`ell in {5,7}`, factor the reciprocal Prym Weil polynomial

```text
F(T)=T^4-tau*T^3+eta*T^2-p*tau*T+p^2
```

over `F_ell`.  Retain the sorted irreducible-factor degrees, including
multiplicity, using the fixed codebook

```text
0 -> 1+1+1+1
1 -> 1+1+2
2 -> 1+3
3 -> 2+2
4 -> 4.
```

A missing/nonsmooth cover has code `-1`; it is part of the categorical key,
not silently discarded.  The eight-entry tuple order is

```text
(m=1,c=+1,ell=5), (m=1,c=+1,ell=7),
(m=1,c=-1,ell=5), (m=1,c=-1,ell=7),
(m=2,c=+1,ell=5), (m=2,c=+1,ell=7),
(m=2,c=-1,ell=5), (m=2,c=-1,ell=7).
```

## Frozen decoder and controls

The primary decoder is one fixed leave-one-curve-out exact-key transport
rule.  For a target on curve `E`, collect scalar-parity labels of all targets
on the other four curves with exactly the same eight-entry tuple.  Predict the
strict majority; abstain if the tuple is unseen or tied.  No feature, distance,
threshold, or rule is selected after observing labels.

Also report descriptive in-sample tuple counts: number of distinct keys,
keys containing both parities, targets in conflicting keys, and best
within-key majority matches.  These are collision diagnostics, not the
primary algorithm score.

Recompute the complete base/quadratic-extension/Prym oracle and retain every
field, curve, histogram, count, Weil, Newton, rescaling, factor and chirality
control.  For every admitted quartic:

1. factor by exact monic division over `F_ell`;
2. brute-force certify each returned factor has no proper monic divisor;
3. multiply all factors and require exact reconstruction of `F`;
4. at every element of `F_ell`, require a generic polynomial evaluator to
   equal the independently written Prym-Weil Horner evaluator;
5. report the five-bin factor-type histogram and how often the ordered tuple
   distinguishes a target from its negation.

Require the 1,520/1,520 planted scorer and byte-identical replay.

## Calibration and conditional algorithm

Score covered predictions by `(2*M-C)^2/C` when `2*M>C`.  For each of 4,096
deterministic within-curve parity shuffles, rebuild every cross-curve training
dictionary and rescore the fixed transport decoder.  Report plus-one empirical
`p`, coverage, matches, exactness, and per-curve scores.

If a fixed categorical map from these tuples to parity transported without
conflict and admitted a generic proof, it would yield the ordinary recursion

```text
d=k mod 2,
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P.
```

Fixed-genus point counting and fixed-degree factorization over fixed auxiliary
fields are polynomial in `log p` in the theoretical model, so the proved rule
would be polynomial-time and therefore subexponential.  A finite lookup or
cross-validation score is not such a proof.  A null closes only exact-key
transport of the two-prime degree-partition tuple, not Jordan data, actual
eigenvectors, larger auxiliary primes, or the general chiral-Prym route.
[pcc:unverified]
