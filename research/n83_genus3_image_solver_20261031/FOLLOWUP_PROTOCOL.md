# Follow-up protocol: add one untouched family size after the n=7 reference failure

Frozen after auditing the preregistered four-size session and before planning or
executing the extension. The original session is retained unchanged. It found
that both destination solvers verified every target, while
`rho.signed_frobenius_strong` and its A/A copy exhausted on every `n=7`
attempt. All measured verified records replayed exactly; the failure makes the
source/destination comparison incomplete at that size.

This follow-up does not tune the failed reference, replace a seed, delete a
target, or reinterpret `n=7`. It adds the next registered `a=0`, `b=1`
same-field cubic-pullback family member above the frozen panel:

```text
n = 23
curve = icv1-f2m23-t5197-69e76b73
EC1 = EC1N23Ce0h27a979171a13
r = 2095853, h = 4
```

The arms, method parameters, target count and laws, target seed, algorithm
seed, rounds, warmup, order, timeout, L2 request, and accounting are identical
to the original protocol. The extension succeeds only if all three substantive
arms and the A/A control verify every measured target and replay exactly. The
combined successful-size panel is then `n in {9,13,19,23}`; the failed `n=7`
rows remain in every table and report. Because Jacobian operations still lack
a source-GAE conversion and this host already failed the PSI/L2 preflight, no
operation-count or wall-time speedup may be promoted.
