# H116 Prym Frobenius-splitting parity protocol

Protocol ID: `H116-PRYM-FROBENIUS-SPLITTING-PARITY-001`

Filed after the raw Prym-discriminant rule failed unchanged OOS and before
evaluating any auxiliary-prime splitting bit. [pcc:unverified]

## Related-work and novelty boundary

Rybakov, *Finite group subschemes of abelian varieties over finite fields*,
Finite Fields Appl. 29 (2014), DOI `10.1016/j.ffa.2014.04.001`, relates the
factorization of abelian-surface Weil polynomials modulo auxiliary primes to
Frobenius actions on torsion.  Honda--Tate theory classifies finite-field
isogeny classes through Weil polynomials.  Targeted repository, ePrint,
arXiv and journal searches found no prime-field ECDLP scalar-parity decoder
from the ordered chiral Prym splitting tuple below.  This is a prior-art
screen, not a novelty proof. [pcc:unverified]

## Observable and conditional algorithm

For each frozen chiral cover, retain the Prym real Weil polynomial

```text
W(X)=X^2-tau*X+(eta-2p)
```

and its integer discriminant `D=tau^2-4*(eta-2p)`.  For auxiliary primes
`ell in {5,7}`, define

```text
b_ell(D)=1  if D is a nonzero square mod ell (W splits with two roots),
b_ell(D)=0  if D is a nonsquare mod ell (W is irreducible),
abstain     if D=0 mod ell (W has a repeated root).
```

The eight ordered bits are

```text
(m=1,c=+1,ell=5), (m=1,c=+1,ell=7),
(m=1,c=-1,ell=5), (m=1,c=-1,ell=7),
(m=2,c=+1,ell=5), (m=2,c=+1,ell=7),
(m=2,c=-1,ell=5), (m=2,c=-1,ell=7).
```

Test every xor of a subset of these bits, optionally complemented: 512
affine Boolean rules.  A rule abstains when any selected bit is repeated-root.
The target is the canonical scalar parity `k mod 2` on all 1,520 nonidentity
targets of the five frozen discovery curves.

A total exact rule with a generic proof would give the ordinary public binary
recursion: recover `d=k mod 2`, then compute

```text
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P
```

and repeat.  Fixed-genus point counting plus constant-size auxiliary-prime
factorization is polynomial in `log p` in the theoretical model, so such a
rule would be a polynomial-time, hence subexponential, ECDLP candidate.  A
finite exact fit without a generic identity is only a candidate.

## Frozen controls and calibration

Recompute the full base/quadratic-extension/Prym oracle.  Retain every field,
curve, histogram, count, Weil, Newton, rescaling, factor and chirality control.
For every admitted Prym and each auxiliary prime, directly count roots of
`W(X)` and require one, zero or two roots exactly as the discriminant bit says.
Report how often the ordered splitting tuple distinguishes a target from its
negation.  Require the 1,520/1,520 planted scorer and byte-identical replay.

Rank by `(2*M-C)^2/C` when `2*M>C`.  In each of 4,096 deterministic
within-curve parity shuffles, reselect across all 512 rules.  Retain total and
covered exact counts, all controls and per-curve scores.  A finite association
only nominates an unchanged OOS rule; a null rejects only this two-prime affine
splitting family. [pcc:unverified]
