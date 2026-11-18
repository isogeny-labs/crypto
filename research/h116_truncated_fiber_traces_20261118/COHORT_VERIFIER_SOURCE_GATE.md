# H116 cohort verifier source gate

The independent verifier recomputes the v2 cohort from the public rule. Its
curve point counter uses Euler's criterion for each right-hand side, whereas
the selector uses a precomputed square-root multiplicity table. It separately
checks primality, nonsingularity, the Hasse interval, multiplicative order,
extension size, global distinctness, first-model ordering, counters, protocol
hash, slugs, generator labels and coordinate-isomorphism parameter.

Strict debug and optimized builds plus both self-tests exited zero before any
cohort file existed. Pre-cohort SHA-256 bindings are:

```text
1e9633158c97c2fbd005f556894486e4b2973e923f5de3dcdfd7c38664ed6255  h116_cohort_verify.rs
e336a5fb1b4ee309bb2084e37366ec369e1fa95e5afe74fdea25f8b7d5a556f7  /tmp/h116_cohort_verify.debug
08c30ed453b6e68e01524222f40b8f0852d7ab8cef6e6adf796017f2813cfade  /tmp/h116_cohort_verify.opt
```

The verifier source and binary binding must be committed and pushed before
`COHORT_V2.json` is emitted or checked. A later receipt will bind the cohort
and verifier stdout hashes. Passing this structural verifier does not validate
any fiber, moment, scalar predicate, cost or timing result.
