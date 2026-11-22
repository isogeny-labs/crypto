# H116 fixed division-polynomial character protocol

Protocol ID: `H116-DIVPOLY-CHARACTER-PARITY-001`

Filed after the doubling Miller family calibrated null and before evaluating
any target-point division-polynomial character. [pcc:unverified]

## Related-work boundary

Lauter and Stange, *The Elliptic Curve Discrete Logarithm Problem and
Equivalent Hard Problems for Elliptic Divisibility Sequences*, SAC 2008,
DOI `10.1007/978-3-642-04159-4_20`, study the EDS-residue problem of deciding
the character of the unknown-index value `psi_k(P)` from `Q=[k]P`.  Shipsey
and Swart, IACR `2008/444`, use EDS symmetry for special group orders.  The
present observable instead evaluates the public fixed-index functions
`psi_m(Q)` for small `m`; it is not either cited oracle.  Repository search,
targeted Crossref queries for division-polynomial quadratic characters, and
an OpenAlex EDS-residue search found those adjacent works but no paper
asserting this exact bounded decoder.  This is a prior-art screen, not a
novelty or absence proof. [pcc:unverified]

## Candidate family

For every target `Q=[k]P`, compute the eight public signatures

```text
s_m(Q) = Legendre(psi_m(Q)),  2 <= m <= 9.
```

The division polynomials satisfy

```text
div(psi_m) = sum(T in E[m]) [T] - m^2[O]
```

with the usual multiplicities/convention, and
`x([m]Q)=x(Q)-psi_(m-1)(Q)psi_(m+1)(Q)/psi_m(Q)^2` away from torsion.  The
implementation checks the latter identity for every target and every
`m=2..9`.  Odd-index functions are x-only; even-index functions contain the
signed y-coordinate, so the family is not wholly subject to the earlier
negation-invariance obstruction.

Test products over all nonempty subsets of the eight signatures of size at
most three and both parity polarities: 184 rules.  A zero character abstains.
Use the same 1,520 targets, exact statistic, deterministic tie-break and 4,096
within-curve family-wise label shuffles as the three preceding parity tests.
Retain per-curve scores, total and covered exactness, identity checks and a
byte-identical replay.

Only a calibrated association may nominate an unchanged OOS rule.  Even an
exact finite rule would still need totality, a generic proof, and complete
recursive cost accounting before it became an ECDLP algorithm.
[pcc:unverified]
