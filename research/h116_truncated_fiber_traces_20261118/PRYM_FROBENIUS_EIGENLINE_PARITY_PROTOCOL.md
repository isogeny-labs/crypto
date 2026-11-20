# H116 Prym Frobenius-eigenline parity protocol

Protocol ID: `H116-PRYM-FROBENIUS-EIGENLINE-PARITY-001`

Filed after the real-Weil splitting-bit family was measured null and before
evaluating any full-Weil eigenline bit against a scalar label.
[pcc:unverified]

## Related-work and novelty boundary

For a genus-two Prym over `F_p`, Frobenius acts on the four-dimensional
`ell`-torsion representation with characteristic polynomial equal to the Prym
Weil polynomial modulo `ell`.  Thus a linear factor gives a Frobenius
eigenvalue and a nonzero eigenline over `F_ell`.  Rybakov, *Finite group
subschemes of abelian varieties over finite fields*, Finite Fields Appl. 29
(2014), DOI `10.1016/j.ffa.2014.04.001`, relates factorization of Weil
polynomials modulo auxiliary primes to finite group subschemes.  Honda--Tate
theory supplies the surrounding isogeny-class interpretation.

Targeted repository, ePrint, arXiv and journal searches found no prime-field
ECDLP scalar-parity decoder from the ordered chiral Prym eigenline tuple below.
This is a prior-art screen, not a novelty proof. [pcc:unverified]

## Observable

For every admitted chiral genus-three double cover, retain the degree-four
Prym Weil polynomial

```text
F(T)=T^4-tau*T^3+eta*T^2-p*tau*T+p^2.
```

For auxiliary primes `ell in {5,7}`, define

```text
e_ell(F)=1  if F has a root in F_ell,
e_ell(F)=0  otherwise.
```

There is no repeated-root abstention.  A missing/nonsmooth cover remains
absent.  A root is exactly a Frobenius eigenvalue on the mod-`ell` Prym Tate
module; equivalently the Frobenius matrix has a nonzero eigenvector over
`F_ell`.

The eight ordered bits are

```text
(m=1,c=+1,ell=5), (m=1,c=+1,ell=7),
(m=1,c=-1,ell=5), (m=1,c=-1,ell=7),
(m=2,c=+1,ell=5), (m=2,c=+1,ell=7),
(m=2,c=-1,ell=5), (m=2,c=-1,ell=7).
```

Test every xor of a subset of these bits, optionally complemented: 512
affine Boolean rules.  A rule abstains only when a selected cover is absent.
The target is canonical scalar parity `k mod 2` on all 1,520 nonidentity
targets of the five frozen discovery curves.

## Conditional algorithm and cost obligation

A total exact rule with a generic proof would yield a public binary recursion.
After recovering `d=k mod 2`, compute

```text
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P
```

and repeat.  Fixed-genus point counting and factorization of a fixed-degree
polynomial over fixed auxiliary fields are polynomial in `log p` in the
theoretical model.  Therefore a proved generic rule would give a
polynomial-time, hence subexponential, ECDLP algorithm.  A finite exact fit is
only a candidate until the generic identity, constructibility, and full cost
proof are supplied.

## Frozen controls

Recompute the complete base/quadratic-extension/Prym oracle and retain every
field, curve, histogram, count, Weil, Newton, rescaling, factor and chirality
control.  For each admitted Prym and auxiliary prime:

1. enumerate every element of `F_ell` and directly count roots of `F`;
2. require the stored bit to equal the predicate that this count is nonzero;
3. for every nonzero `lambda in F_ell`, independently verify

```text
F(lambda)=lambda^2*W(lambda+p/lambda),
W(X)=X^2-tau*X+(eta-2p),
```

where inversion is performed directly in `F_ell`;
4. report how often the ordered eigenline tuple distinguishes `Q` from `-Q`.

Require the 1,520/1,520 planted scorer and byte-identical replay.

## Calibration and disposition rule

Rank rules by `(2*M-C)^2/C` when `2*M>C`.  In each of 4,096 deterministic
within-curve parity shuffles, reselect across all 512 rules.  Retain total and
covered exact counts, all control counts, and per-curve scores.

A finite calibrated association only nominates the selected rule for an
unchanged out-of-sample test.  A null result rejects only this two-prime affine
Boolean family of full-Weil linear-factor bits; it does not reject higher
factorization types, eigenvalue identities, larger auxiliary primes, or the
general chiral-Prym route. [pcc:unverified]
