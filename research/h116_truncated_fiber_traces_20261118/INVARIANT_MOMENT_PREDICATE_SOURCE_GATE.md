# H116 invariant-moment predicate source gate

Protocol `H116-INVARIANT-MOMENT-PREDICATE-001`, both executable sources and
this gate are committed together before any affected full-row moment
generation or predicate scoring. This is an excluded-fixture
hypothesis-generation run, not the frozen H116 out-of-sample campaign.
[pcc:unverified]

The exact pre-execution source identities are:

```text
109111ff15fbcbd8494261ead3500f6981169d5515276be9278fc29571f9b117  INVARIANT_MOMENT_PREDICATE_PROTOCOL.md
4b7238c8f1fc93148c53fbb7bac92d730d66452988efaf4dc7fe33b2e7afad4e  h116_invariant_moment_producer.rs
ccbd0a8c590256617bb683ef55df15c5795047ad366a5f21f19f7990cad65b47  h116_invariant_moment_scorer.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
```

The producer textually includes the already retained `h116_producer.rs`,
which in turn uses the retained field implementation. It evaluates every
target by the jet path and direct fiber enumeration, repeats the jet
evaluation, and refuses output unless all 576 model-target records have zero
candidate/direct and repeat mismatches. The separately compiled scorer does
not import the producer. It reconstructs all sixteen discriminant-normalized
channels, rejects any native/isomorphic-model disagreement, enumerates the
frozen 11,520 predicates, and evaluates the fixed shuffled-label control.
[pcc:unverified]

## Pre-execution checks

Both programs compiled under warnings-as-errors with
`rustc 1.100.0 (0ef1bc386 2026-11-09)`:

```text
/workspace/.h116-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
  --edition=2021 -O -D warnings h116_invariant_moment_producer.rs \
  -o /tmp/h116_invariant_moment_producer
/workspace/.h116-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
  --edition=2021 -O -D warnings h116_invariant_moment_scorer.rs \
  -o /tmp/h116_invariant_moment_scorer
/tmp/h116_invariant_moment_producer --selftest
h116 invariant moment producer selftest: pass
/tmp/h116_invariant_moment_scorer --selftest
h116 invariant moment scorer selftest: pass
```

The scorer self-test enumerates all 720 distinct permutations. A synthetic
channel with bin equal to digit has exactly the identity permutation as an
exact predicate; mutating one label makes every permutation inexact. The
producer self-test checks the two locked fixture descriptors and expected
census without evaluating any target moment. Binary hashes from this check
are recorded only as a diagnostic because the source hashes and compiler
identity are the execution gate:

```text
c20cefde03b7d9c45886713a8ffb2de19e453e817b3871c1c0f14d4b311c8b0d  /tmp/h116_invariant_moment_producer
556e14059c6e30c24c1837fcd87c129bed73b89bf7a2b73e4490ca213af9b759  /tmp/h116_invariant_moment_scorer
```

No affected moment row or predicate score existed when this gate was written.
The first permitted run is the exact launch manifest in the first pushed
commit containing this gate. Every generated row and the complete aggregate
score are retained regardless of outcome. This gate predicts no score and
establishes no ECDLP or complexity theorem. [pcc:unverified]
