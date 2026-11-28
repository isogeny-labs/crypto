# H116 chiral genus-three second-zeta digit protocol

Protocol ID: `H116-CHIRAL-GENUS3-ZETA2-DIGIT-001`

Filed after the first-trace chiral family was calibrated null and before
enumerating any extension-field point or second Frobenius power sum.
[pcc:unverified]

## Mathematical candidate

Keep the frozen chiral covers

```text
C_(c,m)(Q): z^2=(x+c*y)-(x([m]Q)+c*y([m]Q)),
c in {1,-1}, m in {1,2}.
```

For each smooth cover let

```text
s1 = p+1-#C(F_p),
s2 = p^2+1-#C(F_(p^2)),
e2 = (s1^2-s2)/2.
```

Here `s1,s2` are the first two Frobenius power sums and `e2` is the second
elementary symmetric coefficient of the genus-three zeta numerator.  The
first-trace experiment tested only `s1`.  This experiment separately tests
the ordered four-vectors of `s2 mod 3` and `e2 mod 3`.

For each vector exhaust all 243 affine maps `F_3^4 -> F_3`; family-wise
selection is over all 486 rules.  The target remains the canonical digit
`k mod 3`.  A total exact rule with a proved generic identity would feed the
same public ternary recursion frozen in the preceding protocol and would be
polynomial-time in `log p` under fixed-genus Schoof--Pila point counting.

This is not index calculus: no smoothness relation, factor base or linear
algebra is used.  The native `F_(p^2)` census is only a small-field exact
oracle, not the proposed large-field implementation or a timing claim.

## Frozen exact controls

Use all five curves and all 1,520 nonidentity targets.

1. Construct `F_(p^2)` from the first irreducible monic quadratic returned by
   the shared field layer.  Enumerate its square table and require exactly
   `(p^2-1)/2` nonzero quadratic residues.
2. Enumerate `E(F_(p^2))` by x-coordinate and square roots.  Require its count
   to equal `p^2+1-(t_E^2-2p)`, and require exactly `#E(F_p)` Frobenius-fixed
   points.
3. Require each chiral value histogram to contain exactly
   `#E(F_(p^2))-1` affine points.
4. Retain the base discriminant gate.  For every admitted cover verify the
   exact count identities for `s1` and `s2`, `s1^2-s2` even, the bounds
   `s1^2<=36p`, `s2^2<=36p^2`, and `e2^2<=225p^2`.
5. Under `(x,y)->(u^2*x,u^3*y)` and `c->c/u`, require the extension-field
   function value to equal `u^2*(x+c*y)` pointwise for `u=2,3`.
6. Require the exact `c <-> -c` swap for the full integer triple
   `(s1,s2,e2)` at both multipliers and every target.  Report target counts
   distinguished by each new residue vector.
7. Require a 1,520/1,520 planted scorer and a byte-identical replay.

Use the same one-sided ternary statistic `(3*M-C)^2/C` when `3*M>C`.  In
4,096 deterministic within-curve digit shuffles, reselect across both
observables and all 486 rules.  Retain total and covered exact-rule counts,
all census/control counts and per-curve scores.

A finite association nominates only an unchanged out-of-sample rule.  A null
rejects only affine ternary decoding from these two second-zeta vectors.
[pcc:unverified]
