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
exhaustiveness, instruction cost and fall degree as unverified. The subsequent
[root-set extension](ROOT_CERTIFICATE_DESIGN.md) independently checks recognized
completed prime and binary root certificates; its per-call scope does not
certify a complete execution trace or total cost. An explicitly requested
producer replay remains separately labelled. The validation records below are
the frozen pre-extension results and are not rewritten as root-set evidence.

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

## Recorded validation, 2026-10-28

Consumer source `8d55079e37f2c5f2f506edf20670f018354edd56` passed 87 native
release tests (80 library, 7 integration), strict all-target Clippy, formatting
and release build. The opt-in interoperability test separately passed against
producer source `d71ec2ef5ab9b7f2a038f61c941af10297a775ad` and binary SHA256
`63c95dd788f267d1c4cc6323a33b67f607390dc669730c13e99eb156a0004c07`.

| Software check | Observed result |
| --- | --- |
| Planned, sealed and audited cells | 72 each |
| Producer-reported outcomes | 41 verified, 19 partial, 12 unsupported |
| Independently checked positive certificates | 22 |
| Exported records / unsupported rows with null features | 72 / 12 |
| Root-artifact cells retained without independent root-set claims | 6 |
| Rehashed recipe, seed and status substitutions | Rejected |
| Producer replay | Passed over the same 72 sealed cells |

All cells use n=5 and m=2. The 41 producer-reported verified outcomes are not
41 independently proved mathematical results; independent arithmetic coverage
is the 22 positive certificates listed separately. The test makes no solver
timing, cost or fall-degree comparison.

The source, consumer executable, raw logs, first failed provenance admission
and all sealed cells are archived at
`s3://setec-astronomy-artifacts/research/ml-dataset/validation/20261028/consumer-policy-compatibility/f6e4f49cb829728dd8d2688f97415a4e4d526cbfce6e155a1353cbeb273119e9/validation.tar.gz`.
The 1,471,427-byte archive was conditionally created, freshly downloaded with
its ETag, compared byte-for-byte and SHA256-checked. All 2,139 inner file hashes
passed. The consumer then audited the restored 72 cells and rechecked the 22
positive certificates. This proves a controller archive roundtrip; the worker
publication and completion-receipt fetch paths remain outside this check.
Exact receipts and raw local checks are in
[evidence/policy-compatibility-20261028/](evidence/policy-compatibility-20261028/).
The parent-owned root-library gate also passed: 4,987 passed, zero failed and
122 ignored, with the log reporting 1340.44 seconds. The release test executable
was built from runtime source `8d55079e37f2c5f2f506edf20670f018354edd56`;
its SHA256 is `c833e37af8a54d1a92dadbc6385fb071b24e15fdef31e3463cdab46e3a88a563`.
Commit `377319ecf33d561be7bec3de0de7e40642ca3ea5` and this publication follow-up
change only documentation and evidence. The native subtree remains
`ffc2c5727e8902add679db8ac875677d2f8397de`, and the root source/build/test files
are unchanged from the tested runtime commit. No test suite was rerun for
this evidence-only update. These are software checks, not timing comparisons.

The full root build/test logs and hash metadata, excluding the 268,484,192-byte
test executable, are archived at
`s3://setec-astronomy-artifacts/research/ml-dataset/validation/20261028/consumer-policy-root-gate/ec3a9621a0d5e58a145a8c3a5bfb2f15fbe80deb2c3a1ccd8962453d11f820fe/root-validation.tar.gz`.
The 83,828-byte archive passed conditional creation, ETag-bound fresh GET,
SHA256 and byte comparison, extraction, and all six internal file hashes.
Exact metadata is in [root-validation.json](evidence/policy-compatibility-20261028/root-validation.json)
and [root-durability.json](evidence/policy-compatibility-20261028/root-durability.json).
Current-head remote CI, review and merger remain separate delivery gates.
