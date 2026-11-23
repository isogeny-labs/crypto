# H116 quotient-digit scorer source gate

Protocol `H116-QUOTIENT-DIGIT-MOMENT-001`, the conditional proof map, the
scorer and this gate are committed and pushed before the first affected
six-candidate quotient score. The retained input rows predate the protocol,
but their exact identities are sealed below. [pcc:unverified]

```text
f87d49a743b718eb99dcece17dd1506aa2e75ef3258c686133d08a1d24b917a7  QUOTIENT_DIGIT_PROTOCOL.md
b906c56c93d68d301bb3210e41abebd2467cd1d24d3016aff46f4ae535cca6c5  QUOTIENT_DIGIT_PROOF_MAP.md
53405b060a733636f7edac8d2d32bd9f9d1764e201f48d28cc9ded29769f2468  QUOTIENT_DIGIT_LAUNCH.json
52b28350a37483547165cd6b7efc115005aa3453b1fa802ad1cb85f8177207a7  h116_quotient_digit_scorer.rs
8d9791b149a5815c35336c8b7219e71806d6e3376286b7eacb414586d06d2135  INVARIANT_MOMENT_PREDICATE_MOMENTS.tsv
7e892aac8b67704ebb7c78f30ffc5e6c6ae8fafb5f960dc962c2da49dbd9f338  INVARIANT_MOMENT_PREDICATE_LABELS.tsv
```

The scorer imports no producer source. It reconstructs native/isomorphic
model invariants, indexes the complete nonidentity census by scalar only after
that check, constructs all six quotient candidates, applies the locked 32
argmin/argmax rules, and evaluates a deterministic within-row shuffled-label
control. It refuses an unexpected input or target census, an identity
quotient, a missing quotient, or any invariant mismatch. [pcc:unverified]

## Pre-score check

The source compiled under warnings-as-errors with
`rustc 1.100.0 (0ef1bc386 2026-11-09)` and its synthetic self-test passed:

```text
/workspace/.h116-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
  --edition=2021 -O -D warnings h116_quotient_digit_scorer.rs \
  -o /tmp/h116_quotient_digit_scorer
/tmp/h116_quotient_digit_scorer --selftest
h116 quotient digit scorer selftest: pass
```

The diagnostic binary hash is:

```text
c6c1beffe344216e52d59b907ef2359ca25de30cced1d3f1e2a02939c0fc4e0f  /tmp/h116_quotient_digit_scorer
```

No affected quotient feature, prediction or score was computed before this
gate. The first permitted score is the exact launch manifest in the first
pushed commit containing this document. Every aggregate is retained. This
gate predicts no outcome and proves no digit oracle or ECDLP claim.
[pcc:unverified]
