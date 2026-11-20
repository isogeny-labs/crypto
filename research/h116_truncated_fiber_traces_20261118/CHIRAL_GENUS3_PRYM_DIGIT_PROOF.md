# Elliptic--Prym low-coefficient proof

For the elliptic factor

```text
P_E(T)=1-t*T+p*T^2
```

and Prym-surface factor

```text
P_Pr(T)=1-tau*T+eta*T^2-p*tau*T^3+p^2*T^4,
```

the first two product coefficients are

```text
-(t+tau),
p+t*tau+eta.
```

Therefore, if the genus-three coefficients are `-s1` and `e2`, coefficient
comparison gives exactly

```text
tau=s1-t,
eta=e2-p-t*(s1-t).
```

Lean proves both convolution identities, the recovery formulas over an
arbitrary field, and

```text
tau^2-4*(eta-2p)=tau^2-4*eta+8p.
```

The first two proof attempts are retained because a tactic-generated factor
of two would have silently restricted the theorem in characteristic two.  The
final proof instead uses additive negation and cancellation, with no
characteristic assumption.

The theorem consumes the two displayed product-coefficient equations.  It
does not construct the Prym variety or prove the Jacobian isogeny; those stay
explicit geometric endpoint obligations.

```text
Build completed successfully (3164 jobs).
audited 231 theorems in [H116FiberTrace]; 0 failing, 0 trusted (native_decide)
```

[pcc:unverified]
