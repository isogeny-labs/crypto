# H116 cohort v2 gate

Selector v4 emitted `COHORT_V2.json` once after protocol v2 and selector source
were frozen. The tracked file is byte-identical to the raw output:

```text
29e2fd5596ce3ed8e93933161e05b92229cd3065a892bc8ee59d6265f6fa2d99  COHORT_V2.json
```

The first independent-verifier invocation stopped at a whitespace-parser bug
and produced zero stdout. The corrected verifier was committed and pushed as
part of crypto revision `4240c50565` before retrying the same cohort bytes.

Corrected verifier identity:

```text
3ca55d14d93de1aca4610cab414129ac1a07371a91dd2e93023565d82b3bbbb8  h116_cohort_verify.rs
885bb1cc7a54b670d913eab1c5cf32162c43047cca21ea227f3f8e9a6f5f29af  h116_cohort_verify
```

It recomputed the deterministic selection with Legendre-symbol point counts
and exited zero in 0.028s user CPU. Exact stdout and SHA-256:

```text
{"schema":"h116.cohort-verification/v1","status":"pass","curves":5,"primes_scanned":92,"models_scanned":2127}
771cfe129384685e4d5695187feecb8060eef3c402cfe77fdb32e7da0f9f2319
```

This gate establishes only that the public five-curve cohort matches the
frozen v2 structural rule. It is not evidence for a fiber identity, digit
predicate, speed comparison or ECDLP algorithm. [pcc:unverified]
