# Validation — 2026-10-19

The native package passed 17 active tests, with the explicit fixture-generation
utility left ignored during the ordinary suite. `cargo clippy --all-targets --
-D warnings` and `cargo fmt --check` passed. The host was Darwin arm64 with
`rustc 1.93.1 (01f6ddf75 2026-02-11)`.

Coverage includes exhaustive lifting and group-law checks over three finite
fields, inverses checked against exponentiation, all recipe families, invalid
moduli and generators, deterministic archives, corruption and rehashed-metadata
rejection, relation arities 2 through 8, independent direct pair enumeration,
Semaev chain identities, exact leading-form examples, variable/state cap
semantics, and S3 stub tests for create, duplicate, checksum conflict, and fetch.
The native archive replayer is the same implementation as the producer; this
does not establish independent-host or independent-implementation certification.

## Live S3 result

All four public acceptance archives were created in
`s3://setec-astronomy-artifacts/factor-bases/factory/v1/`, downloaded into fresh
directories, and replayed successfully. The matching configured profile was
`ecc-challenges-new`, region `us-east-2`. The default profile's initial
GetBucketLocation and PutObject attempts returned AccessDenied; the accessible
profile was identified before proceeding. No IAM or bucket configuration changed.

| Example input | Points | Manifest SHA-256 |
|---|---:|---|
| `examples/binary.json` | 4 | `e17f8c9cba841c0af017a66ec04859885d24a5eb7519f05c5090a0f77fbf1c62` |
| `examples/prime.json` | 2 | `fba4f779ee63386cc4f9fbf20f093fe8cb3cdaad39cb709767c4c3338bacc3f6` |
| `examples/binary-prime-degree83.json` | 12 | `b67500c942f38ecb001fbcf8a95bdde3e249d5050e3368b83c8eee55e34f3ec6` |
| `examples/secp256k1.json` | 18 | `d9884da9823b75120479edf9d47141ed2ff8dce7ee56d26eaadc8e38534f8200` |

Each manifest key is `factor-bases/factory/v1/manifests/<digest>.json`.
Publication receipts are `validation/local/publish-*.json`; fresh download
receipts are `validation/local/fetch-*.json`. A repeated publication of the
binary fixture returned `already_present_checksum_verified` for both shards
and the manifest, recorded in `publish-binary-duplicate.json`. S3 did not return
VersionId values for these objects; the receipts preserve null rather than
claiming versioned-object retention. SHA-256 and ETag checks were available.

## CLI acceptance diagnostics

The frozen example grid retained five rows: four built archives and one failed
geometric recipe with an empty subgroup base. `validation/local/batch.json`
preserves the failure. The example comparison accepts identical curve/targets
and leaves `candidate_id` and `selected_best_total_runtime` null.

The binary fixture's two-point relation panel checked 12 sampled queries and
retained six verified hits. Two configured algebraic targets had measured
homogeneous leading-form regularity 4 with six variables. These values exercise
the API and replay contract. They are not a high-yield selection result or a
scaling claim. The degree-83 and secp256k1 examples were construction/storage
checks; no matched large-field yield or regularity improvement is asserted.

All recorded wall times are unqualified implementation diagnostics. They were
not measured on an isolated benchmark host and are not used for comparisons.

## Frozen producer source

The four S3 archives record construction source digest
`160dc6381095685772a6cf699d1da73459b467676c6bbe31fea63a6fbec7609d`.
The exact Rust sources, tests, Cargo manifest, and lockfile are retained in
`validation/source-160dc6381095685772a6cf699d1da73459b467676c6bbe31fea63a6fbec7609d.tar.gz`,
whose archive SHA-256 is
`1726d36761e3d42febf4446f850cbeda20a8fe07de125d25dfcb10d1ccbce098`.
The final working package removes an extra blank line and adds early archive-total
and malformed-comparison checks. Its source digest differs while the construction
and measurement algorithms are unchanged. The original producer snapshot and
published artifacts remain intact; replay with the final source is recorded
separately.

The source bundle and `validation/catalog.json` remain local pending explicit
approval for the additional S3 upload. Automatic approval review rejected that
upload as additional source/payload disclosure beyond the factor-base data.
The catalog's source key is a proposed destination, not an uploaded-object claim.

## Separate repository publication gate

The enclosing repository's required `cargo test --release --lib --offline` was
started on the unchanged library sources at base commit
`f125e2315d2fafc44b93c415bdeb540371e373b2`. Its initial build target on the mounted
workspace progressed slowly and was interrupted, then restarted with a local
SSD target. The final result is recorded below once available. This gate is
separate from the passing standalone package tests.

GitHub CLI authentication was invalid in this session; the connected GitHub app
could read repository metadata. No PR or merge is claimed without a recorded
successful publication result.
