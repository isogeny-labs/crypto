# H116 chiral marked two-isogeny halving-parity protocol

Protocol ID: `H116-CHIRAL-MARKED-TWO-ISOGENY-HALVING-PARITY-001`

Filed after the binary-quartic `I,J` characters were measured null and before
evaluating any marked-kernel halving bit against scalar parity.
[pcc:unverified]

## Related-work and novelty boundary

Division by two on elliptic curves and explicit two-isogeny descent are
standard; see Bekker--Zarhin, *Division by 2 of rational points on elliptic
curves*, St. Petersburg Math. J. 29 (2018), 683--713,
<https://doi.org/10.1090/spmj/1512>.  The resolvent/Jacobian construction for
binary quartics is also classical.  Targeted repository, Crossref, arXiv,
ePrint and journal searches found no generic prime-field ECDLP scalar-parity
decoder from target-tethered marked two-torsion halving classes.  This is a
prior-art screen, not a novelty proof. [pcc:unverified]

Unlike the preceding trace and `I,J` panels, this experiment retains the
factorization itself.  For

```text
q_1=x^2+a*x+b,  q_2=x^2+c*x+d,
```

the partition `q_1*q_2` marks a rational two-torsion point on the auxiliary
quartic Jacobian.  It therefore retains level-four torsion data that is not
an invariant of the unmarked quartic.

## Frozen observable

Use the same four smooth branch-pair quartics, in order

```text
H_0*H_2, H_1*H_3, H_4*H_6, H_5*H_7.
```

For the classical invariants `I,J`, put

```text
E:  Y^2=X^3-27*I*X-27*J,
X_T=3*a*c-6*b-6*d.
```

The factorization marks `T=(X_T,0)`.  Translate it to `(0,0)`:

```text
E_T:  y^2=x^3+A*x^2+B*x,
A=3*X_T,
B=3*X_T^2-27*I=81*Res(q_1,q_2).
```

The two-isogenous curve and its marked dual-kernel point are

```text
E'_T: y^2=x^3-2*A*x^2+(A^2-4*B)*x,
B'=A^2-4*B=81*disc(q_1)*disc(q_2).
```

Retain two total Boolean features per model: whether `T` has a rational half
on `E_T(F_p)`, followed by whether the dual-kernel point has a rational half
on `E'_T(F_p)`.  The eight-bit order is therefore

```text
(model0,T/2), (model0,T'/2), ..., (model3,T/2), (model3,T'/2).
```

Test all 256 feature xors, optionally complemented: 512 affine Boolean rules
predicting canonical scalar parity on the 1,520 nonidentity targets of the
five frozen discovery curves.

## Frozen controls

Retain every inherited branch-cubic, residual-quadratic, resultant,
binary-quartic and genus-one trace control.  For every admitted marked model:

1. require `X_T` to be a root of `X^3-27*I*X-27*J`;
2. require the independent coefficient identities for `B` and `B'` above;
3. enumerate every affine point of both marked Weierstrass models and apply
   the general doubling law, counting preimages of `(0,0)`;
4. independently recount the same preimages through the eliminated equation
   `x(P)^2=B` (respectively `B'`);
5. require both isogenous point counts to equal the prior genus-one order;
6. report the true-feature census and target/negation distinction count.

Require a 1,520/1,520 planted scorer and byte-identical replay.

## Calibration and conditional algorithm

Rank by `(2*M-C)^2/C` when `2*M>C`.  In each of 4,096 deterministic
within-curve parity shuffles, reselect across all 512 rules.  Retain total and
covered exact counts, all controls, and per-curve scores.

A total exact rule with a generic proof would give the public binary
recursion

```text
d=k mod 2,
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P.
```

The fixed branch construction, quartic invariants, resolvent root,
two-isogeny coefficients and bounded halving test cost polynomial time in
`log p` using finite-field arithmetic and division-polynomial/root tests.
Thus a proved generic rule would be polynomial-time and hence
subexponential.  A finite exact fit is only a candidate.  A null closes only
this eight-bit affine marked-halving family. [pcc:unverified]
