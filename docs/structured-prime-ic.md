# Structured prime-field relation queries

`tensor-structured` exposes the structured fibre predicate, exact counting,
prefix-count witness recovery and reusable local membership message through a
versioned JSON interface. The public fixture lives in
[`structured-prime.json`](../crates/hashed-ecdl-tensor/examples/structured-prime.json).

From the repository root:

```sh
cargo run --release --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin tensor-structured -- crates/hashed-ecdl-tensor/examples/structured-prime.json > receipt.json
cargo run --release --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin tensor-structured -- replay receipt.json
```

These commands use a public regression fixture. A request/receipt path can be
`-` to read standard input. The library entry points are
`hashed_ecdl_tensor::structured_runner::{execute_json,replay_json}`.

## Request contract

The required schema is `structured-prime-ic-request/v1`. Field/curve/subgroup
parameters, generator coordinates, `kernel_x` and every `image_x` entry use
unsigned decimal strings. A target is either `["x","y"]` or `null` for identity.
All fields in the example are required. Missing, duplicate and unknown fields
are rejected, including duplicates inside a receipt's nested request or answers.
JSON input is capped at one MiB, decimal strings at 200 characters, image sets
at 1024 entries, targets at 64 and arity at 2–8.

The `membership` option selects `circuit` or `fibre_message`. Both use the same
rational two-isogeny fibre definition and actual subgroup restriction. The second
builds one local membership decision diagram from quadratic roots and rational
lifts, sharing it across slots and queries. `fixture_seed` makes root extraction
reproducible for public development fixtures. The arithmetic network is prepared
once and the target boundary is pinned separately for each query.

Relations are ordered signed tuples with repetitions and inverse pairs allowed.
The archive factory's unordered multiset receipts have a different policy.
The source family requires rational two-torsion. Complete addition charts cover
identity intermediates; the chosen factor base explicitly excludes the kernel.

`steps`, `vector` and `bits` are explicit caller limits. A single meter spans
input validation, shared setup and every count-plus-witness query. The command
returns a receipt only when the complete request finishes. A later query's
resource failure emits no partial success JSON; a completed zero count has a
null witness. Setup and per-query charged steps are reported separately, with
their sum, graph sizes and optional local-message size. These are software
diagnostics rather than calibrated operation costs or timing.

## Receipt and replay contract

`structured-prime-ic-receipt/v1` binds the normalized request, relation policy,
counts and signed witnesses in `semantic_sha256`. Selected source pins cover
the predicate, runner, encoder, counter, curve and arithmetic source files.
They are partial source provenance; dependencies and the full build are not
included. Diagnostic counters are outside the semantic digest.

Replay checks the schema, policy, digest and selected source pins, then
recomputes all answers. It compares semantic answers rather than diagnostic
counters. A matching self-reported digest is insufficient: answer mutation with
a recomputed digest is also subjected to recomputation. This replay uses the
same native implementation, and receipts have `standing: producer_assertion`.
Independent claim standing comes from the repository PCC ledger.

The implementation report and retained development checks are in
[`research/structured_ic_integration_20261104/`](../research/structured_ic_integration_20261104/README.md).
The next algorithmic component is a compact message for the full constrained
summation boundary; the current shared message represents local membership.
