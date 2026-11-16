# Native canonical polynomial interface

This is a bounded format adapter and external-library integration test, not an unknown-coefficient solver or performance result. The original fixed exports remain unchanged. The adapter was completed and verified in the parent session after its parallel tooling task was interrupted by automatic content filtering; that interruption is not a mathematical negative.

The current official msolve README requires a single collected occurrence of each monomial. It documents expanded sum-of-monomial syntax, not the prior prototype's nested expression-tree grammar. `canonicalize.cpp` parses arithmetic expressions, expands products, reduces coefficients modulo the declared prime, collects like monomials and emits canonical terms. It rejects unsupported syntax, duplicate/invalid variables, bad fields and bounded-resource violations. Its supported contract is limited to inputs below 1 MiB, at most 1,024 variables, degree at most 32 and at most 100,000 terms per polynomial; it is not a general CAS parser. Rational coefficients and arbitrary scientific workloads are not supported.

The strict C++17 build passes 21 adapter checks. All six original fixed systems round-trip to identical coefficient maps, retain maximum input degree two, and vanish at their known assignments. A separately implemented arithmetic evaluator rechecks the product-format outputs. The collected term counts for Graeffe/remainder are 73/102 at m=3, 179/273 at m=5 and 291/486 at m=7. These are representation counts, not measured solving work.

## Actual external importer check

An unprivileged local install extracted Debian msolve **0.7.5-1**, libmsolve0 and FLINT into `/workspace/audit/backend/prefix`, without modifying system libraries or identities. `backend/packages.json` binds package filenames, versions and SHA-256 hashes checked against Debian's package index. The msolve 0.11.0 source archive was inspected but was not built or benchmarked. No fastest-backend claim is made.

The version-0.7.5 native executable correctly parsed all six canonical systems after **every variable was explicitly bound to its known assignment**. Each reduced basis equals the expected linear assignment ideal. This verifies format and fixed-input consistency, not unknown solving or scaling. No timings were collected as measurements.

The first bound-witness check unexpectedly returned the unit ideal. Isolating the failure showed that this executable interprets the unnormalized input `x+0` as `x+1`, while `x` gives the expected zero assignment. The small reproducer and initial failed bound system/output are retained under `backend/`. Re-normalizing appended assignment constraints removes zero terms; then every expected assignment ideal matches. This does not establish a defect in every msolve version, nor invalidate any historical run. It shows why a rejected known answer must first be checked for frontend/format errors.

The original nested AST exports were not submitted as unknown research systems to this backend. No conclusion about their direct acceptance is inferred from the canonical route.

## Reproduce

From the repository root:

```sh
bash research/prime_one_shot_20261029/continuation/canonical/check.sh /tmp/norm-canonical-check
```

To include the installed external integration check:

```sh
LD_LIBRARY_PATH=/workspace/audit/backend/prefix/usr/lib/x86_64-linux-gnu \
  bash research/prime_one_shot_20261029/continuation/canonical/check.sh \
  /tmp/norm-canonical-check /workspace/audit/backend/prefix/usr/bin/msolve
```

The second command is a fixed known-answer unit test. New unknown-solving comparisons need a prospective scientific protocol and the repository's execution/measurement route. The separate actual unknown solver and direct DLP development baseline live in `../solver/`.

## Separate unbound fixed unit

The later [single unbound check](unbound-check/README.md) passes all33 m3
fixture variables as unknowns. Its printed leading staircase matches the28
expected models. This separate bounded development result does not change
the six known-bound checks into an unknown-system scaling experiment.
