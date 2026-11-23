# H116 selected quotient-channel proof map

Status: written before the corresponding Lean source.

## Claim 1: the selected channel cancels the higher jet

For

```text
F_u(w) = 1-u*w+h1*w^2+h2*w^3+...
```

and its reciprocal `G_u=1/F_u`, the reciprocal recurrence gives

```text
[w^0]G_u = 1,
[w^1]G_u = u.
```

Therefore

```text
[w^1](F'_u G_u) = 2*h1-u^2.
```

Subtracting target from reference with the same higher jet cancels `2*h1`
exactly:

```text
[w^1](F'_(u0)G_(u0)-F'_u G_u) = u^2-u0^2.
```

This is the producer's `X_1` coordinate. The selected `IX1` rule is thus a
sixth power of a scaled x-coordinate square difference divided by the fixed
curve discriminant square; it does not use `h2,...,h7` and does not depend on
a deep high-moment identity.

## Claim 2: the public quotient step is exact given the true digit

For a point `P`, natural scalar `k`, quotient `a=k/6` and digit `d=k%6`, prove

```text
k • P = 6 • (a • P) + d • P,
(k • P) - d • P = 6 • (a • P).
```

Also prove `a<k` for `k>=6`. These are the unconditional arithmetic/group
facts behind recursive digit recovery. Inverting multiplication by six on a
prime-order subgroup is a separate existing Bezout/group step.

## Controls and nonclaims

- The Lean theorem must quantify over an arbitrary field and arbitrary shared
  higher-jet function; no finite experiment is imported.
- A mutation with unequal `h1` terms should retain the uncancelled correction,
  showing why the shared-jet premise matters.
- The quotient theorem is conditional on the true digit. Neither theorem
  proves that canonical representative argmin is exact, and the OOS experiment
  shows that it is not. [pcc:unverified]
