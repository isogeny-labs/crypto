# TorsionGuard

TorsionGuard is a bounded audit tool for one question: do supplied linear
relations have a nontrivial common kernel on a declared finite abelian primary
module?

This question is useful when reviewing elliptic-curve circuits with hints,
cofactor components, endomorphism decompositions, or incomplete subgroup
binding. The tool does not parse a circuit, derive a curve's group order, or
check that supplied matrices match deployed code. Those facts remain explicit
obligations in every input and certificate.

## Run it

```console
cargo run --release --bin torsionguard -- \
  analyze tests/fixtures/torsionguard/toy_unsound.json

cargo run --release --bin torsionguard -- \
  synthesize tests/fixtures/torsionguard/toy_unsound.json

cargo run --release --bin torsionguard -- \
  validate tests/fixtures/torsionguard/two_relation_sound.json
```

`analyze` emits normalized inputs, every prime-local restricted matrix, RREF,
and pivot columns. A rank defect also includes a nonzero witness lifted into
the original prime-power module and its zero evaluation under every original
relation. `synthesize` exhaustively searches at most 20 declared candidate
relations, minimizing declared cost, then candidate count, then names.

The verdict vocabulary is closed:

- `unsound_in_declared_model`: a nonzero common-kernel witness was constructed;
- `not_established_missing_obligations`: the algebraic kernel is trivial, but
  at least one required external obligation is open;
- `sound_under_declared_assumptions`: the kernel is trivial and every required
  obligation is `verified` or `not_applicable`. [pcc:unverified]

The last verdict is conditional. It is not a statement that a deployed circuit
is sound unless the recorded model and evidence match that circuit.

## Input contract

All integer matrix entries and primes may be JSON integers or base-10 strings;
certificates normalize them to strings. A component represents
`Z/(prime^exponent)`. A relation is a square integer matrix acting on the direct
sum of all components. The parser rejects a matrix entry `a[i][j]` unless
`n_i` divides `a[i][j] n_j`, the well-definedness condition for a map from
`Z/n_j` to `Z/n_i`.

Six obligations are required: `group_structure`, `point_on_curve`,
`input_domain`, `scalar_binding`, `coefficient_bounds`, and `map_actions`.
Each has a nonempty evidence string and a status of `verified`,
`not_applicable`, or `unverified`. These are input labels, not PCC standings.
[pcc:unverified]

Primes through 64 bits receive deterministic Miller–Rabin validation. Larger
values must pass a fixed-base probable-prime screen and include a nonempty
`prime_proof` reference; the certificate preserves that conditional
assumption. This screen is not a primality certificate.

## Scope and evidence

The implementation uses integer and finite-field algebra; it is not a
benchmark and makes no performance or deployed-vulnerability claim. The
deterministic test suite compares its rank criterion with exhaustive
enumeration on 10,000 small generated modules, in addition to fixed positive,
negative, prime-power, map-validation, obligation-gate, and repair fixtures.
Claim record: `PCC-20261117-9fbe8c`. [pcc:unverified]

The shared research KB was unavailable in the implementation environment
(MCP tools absent; repository wrapper could not find its executable). Current
repository searches for the tool name, hinted scalar multiplication,
underconstrained hints, subgroup binding, cofactor torsion, and GLV soundness
found adjacent material but no matching analyzer. That bounded search is not
an absence or novelty proof.

Motivating primary sources reviewed before implementation:

- Eagen et al., “Fast elliptic curve scalar multiplications in SN(T)ARK
  circuits,” IACR ePrint 2025/933: <https://eprint.iacr.org/2025/933>
- gnark, “Soundness issues in gnark/std circuit components,”
  GHSA-3mvx-pp85-pm65 (2026-08):
  <https://github.com/Consensys/gnark/security/advisories/GHSA-3mvx-pp85-pm65>
- gnark, “Missing on-curve check in emulated fake-GLV scalar multiplication,”
  GHSA-7fx8-hmgc-82jp (2026-10):
  <https://github.com/Consensys/gnark/security/advisories/GHSA-7fx8-hmgc-82jp>
- “Practical Security Analysis of Zero-Knowledge Proof Circuits,” IACR ePrint
  2023/190: <https://eprint.iacr.org/2023/190>

These sources motivate the audit boundary. They do not establish that
TorsionGuard finds a new vulnerability or covers every failure mode in those
systems.
