# Versioned policy compatibility

This companion consumes saved `crypto.ml-dataset.plan/v2` runs and
`ml-ecdlp-factor-base/v2` artifacts from the native producer. Plan/v1 remains
accepted without a policy field. Its frozen 720-cell pilot's canonical digest
and every cell mapping have a separate regression. Present-but-empty, null,
duplicate, unknown or malformed policy axes fail closed.

The source contracts are the producer's `DatasetPlan`, `FactorBasePolicy`,
`verify_factor_base_config` and `factor_bases::sampling_seed`, plus
`docs/ml-dataset/FACTOR-BASE-POLICIES.md` and
`docs/ml-dataset/PRIME-ROOT-RECORDS.md` on the policy producer branch. The
consumer implements these bindings independently using its own JSON parsing
and validation; its standalone crate does not link the producer library.

## Bindings and claim boundaries

Plan hashing reconstructs the producer's field order, including canonical
recipe fields. Recipe order in the policy array is significant; JSON object
key order is not. The mixed-radix index varies solver fastest, policy next,
then twist and the original problem axes. A saved base must match the selected
recipe, policy rule, requested size, actual point count, declared caps and
restricted control seed derived from the frozen problem configuration.

The policy parser accepts exactly four recipe shapes: legacy, sampled-x
projected control, binary subspace intersection and binary Frobenius-union
intersection. It independently rejects unknown fields, invalid integer types,
out-of-field or dependent binary basis masks and oversized limits. These are
structural checks, not reconstruction of random samples or subgroup tests.

Complete, capped and unsupported bases have separate consistency rules. A
literal coordinate-domain intersection may be empty or smaller than the
requested point count even when complete. Prime curves with binary recipes
are unsupported. Unsupported populations retain labels and provenance but
have null exported features, with `unsupported_factor_base_policy` recorded.
Closure indices are checked for bounds and consistency with declared closure;
this does not independently verify their algebraic action.

Optional independent positive PDP arithmetic accepts the saved v2 point
domain and checks the same public group equation as for v1. It continues to
report construction, subgroup order, solver independence, UNSAT,
exhaustiveness, instruction cost and fall degree as unverified. Root artifact
hashes are preserved, but the consumer adds no independent root-set or
execution-trace verifier. An explicitly requested producer replay checks those
producer contracts through the pinned executable and is labelled separately.

## Separate static training view

Policy cells export `ml-dataset.static-routing-view/v2` with the consumer
policy name `consumer-static-policy-recipes-v2`. Legacy cells retain v1.
Only the typed public recipe is added to model features. Sampling seeds,
construction candidates, counters, status and solver answers remain outside
features; reported construction counters are restricted labels.

This view is deliberately distinct from producer
`strict-structural-allowlist-v4-policy-recipes`. It retains the consumer's
manifest-derived record IDs, solver/resource features and existing whole-field
split algorithm. The producer uses mathematical feature identities, separate
target rows, a different split hash and explicit transformed PDP-domain
identities. `producer_training_export_equivalence_verified` remains false.
Do not join these views by record ID, combine their partitions, or treat their
PKM/large-prime features as matched ordinary PDP domains. Independent validation
of the producer's complete training-export bundle is still outstanding.

## Reproducible software interoperability

`native/tests/producer_policies.rs` contains an opt-in test requiring a pinned
producer binary and a fresh retained directory. It files a software-only
protocol before generation, then requests 72 small cells: three curve
families, both twists, m=2, six recipes (four kinds plus capped controls),
and enumeration/root adapters. Each cell has a five-second deadline, 512 MiB
memory and 8 MiB trace cap. There are no S3 publication flags or cloud jobs.
Each subprocess also has an independent ten-minute/output bound.

The test audits hashes/config bindings, requests independent positive
arithmetic and producer replay, exports all rows, checks unsupported exclusion
and feature leakage, and rejects rehashed recipe/seed/status substitutions in
a separate copy. Original producer cells are retained unchanged. A passing
test establishes local software interoperability only; it is not the intended
large-size training corpus, a cost comparison, a worker S3 permission test or
a fall-degree result.

Run from the repository root, with a distinct consumer target directory:

```sh
cargo test --release --locked --manifest-path research/ml_dataset_independent_validation_20261028/native/Cargo.toml --all-targets
cargo clippy --release --locked --manifest-path research/ml_dataset_independent_validation_20261028/native/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path research/ml_dataset_independent_validation_20261028/native/Cargo.toml -- --check
```

For the opt-in test, set `ML_DATASET_INTEROP_PRODUCER` to the exact binary,
`ML_DATASET_INTEROP_DIR` to a nonexistent output directory and
`ML_DATASET_INTEROP_FROZEN_AT` to the current UTC timestamp. Invoke
`cargo test ... --test producer_policies -- --ignored --nocapture`.
The binary hash is pinned in the test source. Generation requires a producer
build with its full source commit and `CRYPTO_BUILD_GIT_DIRTY=false` provenance;
the test does not bypass that admission rule. Failed attempts remain retained
separately. Validation evidence distinguishes completed checks from pending
interoperability and cloud work.
