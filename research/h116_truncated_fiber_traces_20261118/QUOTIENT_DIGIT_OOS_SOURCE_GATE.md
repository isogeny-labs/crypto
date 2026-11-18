# H116 quotient-digit OOS source gate

Protocol `H116-QUOTIENT-DIGIT-OOS-001`, both executables and this gate are
committed and pushed before any affected moment or score is generated on the
five-curve cohort. [pcc:unverified]

```text
ec1617091c97fed4f64a4352dedd3b038515160ed2671e47e6bfb3635c8a1756  QUOTIENT_DIGIT_OOS_PROTOCOL.md
79a71a004bd17d8fb6b6151ccdad6ffdacb230cfd25559a37943d90e6ca9fefc  QUOTIENT_DIGIT_OOS_LAUNCH.json
28a7d9644de6a3cffcbaa8548610adef5d611cb3c9b3baa426712da4369c935a  h116_quotient_digit_oos_producer.rs
bdf71550707c15046077e050099cbd26f53e6351f3a7ceb7971aabdfde2a619e  h116_quotient_digit_oos_scorer.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
52b28350a37483547165cd6b7efc115005aa3453b1fa802ad1cb85f8177207a7  h116_quotient_digit_scorer.rs
29e2fd5596ce3ed8e93933161e05b92229cd3065a892bc8ee59d6265f6fa2d99  COHORT_V2.json
```

The producer textually includes the frozen H116 jet implementation, emits
separate digest/moment and digest/scalar files, repeats every jet evaluation,
and enforces the exact 4,560-target/9,120-model census. It does not execute the
extension-field direct-fiber path. The separate scorer includes the earlier
invariant parser but applies only `IX1-argmin`; it recomputes all sixteen
coordinate invariants, constructs the six quotient candidates, and performs
the fixed 4,096 one-rule shuffles. [pcc:unverified]

Both sources compiled under warnings-as-errors with
`rustc 1.100.0 (0ef1bc386 2026-11-09)`, and their source-only controls passed:

```text
h116 quotient digit OOS producer selftest: pass
h116 quotient digit OOS scorer selftest: pass
```

Diagnostic binary hashes:

```text
1ce399bd982a1f988d379b2578d31afa7978f69015e162276cebd237852c2c14  /tmp/h116_quotient_digit_oos_producer
4587dfc24f718a5977f571441cae8a363445f60c6db1e86ace514e2063ec7a7a  /tmp/h116_quotient_digit_oos_scorer
```

No affected OOS moment, prediction or permutation score existed when this
gate was written. The first permitted run is the exact launch manifest in the
first pushed commit containing this gate. Every artifact and result is
retained. This predicts no outcome and establishes no decoder or ECDLP claim.
[pcc:unverified]
