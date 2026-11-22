# H116 chiral genus-three Prym digit protocol

Protocol ID: `H116-CHIRAL-GENUS3-PRYM-DIGIT-001`

Filed after both mixed second-zeta panels were calibrated null and before
scoring any elliptic-factor-removed Prym observable. [pcc:unverified]

## Geometric factor and prior-art boundary

For a ramified double cover `C -> E`, the Prym is the connected kernel of the
norm map and `Jac(C)` is isogenous to `E x Prym(C/E)`.  Laga and Shnidman,
*Arithmetic statistics of Prym surfaces*, arXiv:`2101.07658`, study abelian
surfaces arising from double covers of genus-one curves by genus-three curves.
Lombardo, Lorenzo García, Ritzenthaler and Sijsling, *Decomposing Jacobians
via Galois covers*, DOI `10.1080/10586458.2021.1926008`, give explicit tools
for identifying Prym factors up to isogeny.  Targeted repository and web
searches found no ECDLP digit decoder from the middle Prym Weil coefficient or
the real-Weil discriminant below.  This is a prior-art screen, not a novelty
proof. [pcc:unverified]

This experiment uses the same fixed-genus point-counting complexity boundary
and exact small-field oracle as the preceding second-zeta panel.  It uses no
factor base, smoothness relation or index-calculus solver.

## Exact factor removal

Write the elliptic factor as

```text
P_E(T)=1-t*T+p*T^2
```

and the abelian-surface Prym factor as

```text
P_Pr(T)=1-tau*T+eta*T^2-p*tau*T^3+p^2*T^4.
```

If `s1` is the first Frobenius power sum of `C` and `e2` its second elementary
coefficient, coefficient comparison in `P_C=P_E*P_Pr` gives

```text
tau=s1-t,
eta=e2-p-t*tau.
```

The Prym real Weil polynomial is

```text
X^2-tau*X+(eta-2p),
```

with discriminant

```text
D=tau^2-4*(eta-2p).
```

For each of the four ordered chiral covers, test separately `eta mod 3` and
`D mod 3`.  Exhaust all 243 affine maps for each observable and select
family-wise over 486 rules.  The target is again `k mod 3`; an exact generic
rule would feed the already-frozen public ternary recursion and yield a
polynomial-time candidate in the fixed-genus point-counting model.

## Frozen controls

Recompute the full base and quadratic-extension oracle from the exact source
bytes.  Retain every preceding field, curve, histogram, count, Weil, Newton,
rescaling, chirality, planted and replay control.  Additionally require, for
every admitted cover,

```text
s1=t+tau,
e2=p+t*tau+eta,
tau^2 <= 16p,
eta^2 <= 36p^2,
0 <= D <= 16p.
```

Report how many targets are distinguished from their negations by each Prym
residue vector.  Use the same one-sided ternary statistic and 4,096
deterministic within-curve family-wise shuffles.

A finite association nominates only an unchanged out-of-sample rule.  A null
rejects only these two affine Prym residue families. [pcc:unverified]
