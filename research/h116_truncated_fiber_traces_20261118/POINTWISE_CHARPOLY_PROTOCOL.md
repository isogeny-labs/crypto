# H116 pointwise Frobenius characteristic-polynomial protocol

Protocol `H116-POINTWISE-CHARPOLY-001` is frozen before either implementation
is written or any cell is executed. It tests the last algebraic premise in the
current affine-point Lean endpoint on every point of eight small extension
fields. [pcc:unverified]

## Question

For `E: y^2=x^3+a*x+b` over `F_p`, set
`t=p+1-#E(F_p)` and let `pi(x,y)=(x^p,y^p)`. The primary equations are

```text
pi^2 - [t]pi + [p] = 0,
V = [p]pi^-1 = [t] - pi.
```

The first equation is the premise still exposed by the 105-theorem Lean
checkpoint. The second compares the newly constructed pointwise `V` against
the standard trace expression without assuming a rational-map formula.
[pcc:unverified]

## Frozen panel

The cells use `(p,a,b)=(5,2,1)` at degrees two through four,
`(7,1,1)` at degrees two through four, and `(11,1,1)` at degrees two and
three. Each extension is represented by the lexicographically first monic
irreducible polynomial selected by the existing deterministic Rabin checker.
The largest field has 2,401 elements. The machine-readable cell list and exact
selection rule are in `POINTWISE_CHARPOLY_PROTOCOL.json`. [pcc:unverified]

## Positive checks

The producer must enumerate all extension-field points, not a sample. The
curve discriminant and extension modulus must pass their exact checks. The
enumerated point count must match the independent trace recurrence
`N_d=p^d+1-s_d`, where `s_0=2`, `s_1=t`, and
`s_d=t*s_(d-1)-p*s_(d-2)`. Every point must pass `pi^d=1`, the characteristic
equation, and the pointwise `V` equality. [pcc:unverified]

## Controls and independent replay

Three mutations are frozen: use `t+1`, use `[p+1]pi^-1`, or replace
`pi^-1` by the identity in `V`. Each applicable mutation must disagree on at
least one enumerated point. The verifier will be a separate Python
polynomial-basis field and affine group implementation; it will reconstruct
the cells from the protocol and replay the result rather than consume the Rust
point operations. Source hashes for both implementations must be sealed in
`POINTWISE_CHARPOLY_SOURCE_GATE.md` before execution. [pcc:unverified]

All exact point counts, failure counts, mutation rejection counts and producer
operation counters will be retained. Elapsed nanoseconds are diagnostic only.
No failed cell may be replaced. This finite panel is not a generic
characteristic-polynomial proof, a rational/scheme-map identification, an
ECDLP algorithm, or an asymptotic claim. [pcc:unverified]
