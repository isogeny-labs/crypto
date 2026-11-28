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

The first invocation on the emitted cohort stopped before comparison because
`parse_integer` did not skip whitespace between a top-level colon and value.
Its stdout was zero bytes. The cohort was not regenerated. The parser now skips
ASCII whitespace and its self-test covers the exact spaced top-level form. A
new source/binary hash will be committed and pushed before retrying the same
cohort bytes.

Corrected strict builds and self-tests exited zero. Retry-gate SHA-256 bindings
are:

```text
3ca55d14d93de1aca4610cab414129ac1a07371a91dd2e93023565d82b3bbbb8  h116_cohort_verify.rs
e90ee939c644edc42e5cf9bdeb49882116e5c7d04a4d3f127877452b29648382  /tmp/h116_cohort_verify.v2.debug
885bb1cc7a54b670d913eab1c5cf32162c43047cca21ea227f3f8e9a6f5f29af  /tmp/h116_cohort_verify.v2.opt
```

Tracked `COHORT_V2.json` is byte-identical to the raw selector output at
SHA-256 `29e2fd5596ce3ed8e93933161e05b92229cd3065a892bc8ee59d6265f6fa2d99`.
