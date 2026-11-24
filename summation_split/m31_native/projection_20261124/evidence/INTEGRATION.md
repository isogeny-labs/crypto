# Integration and custody state at this continuation

Read-only observations, 2026-11-24. Branch source parent:
`f80a6afef699299b9c3cee5bd4ddf54517280dfa`; inspected main:
`ad634b4aa2e6d99e129e9ed3f5ede9c20446b901`.

## Broader repository gate

The root `cargo test --release --lib` gate was not run in this fresh runtime.
Earlier 45-second and 180-second compilation caps remain historical evidence
in the previous supplement. Main's build repair PR #5078 is merged; those
repairs are not an assertion about this older PR branch.

The current checkout is sparse. Root `src/` and the eight root path
dependencies are omitted: `crates/certified-relations`,
`crates/relation-replay-checker`, `crates/hashed-ecdl-tensor`,
`crates/ecdl-tensor-ic`, `research/pencil_large_prime_20261029`,
`tools/factor-base`, `tools/volcano-cli`, and `isogeny_algos`.
The nested optional path `research/five_leads_e2e_20261019/native` is also
needed for dependency resolution. Compile-time include files extend beyond
those directories into documentation, GPU headers, test fixtures and
research fixture directories. Merely hydrating the root `src/` would create
artificial missing-input failures.

The restored Rust toolchain has an empty external dependency cache. Native
C/C++ tools and OpenSSL are available, but cmake is absent. First reconcile
the intended source baseline, hydrate its complete compile-time inputs and
obtain the locked dependencies, then run the actual root gate:

```sh
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked --release --lib
```

The changed standalone crate has its own empty Cargo workspace and no
external dependencies or root-library linkage. Its passing local tests do
not substitute for the root gate. [pcc:unverified]

## Shared control plane and checkpoint custody

No Conductor executable/endpoint/token or loaded control-plane MCP was
available. `scripts/research-kb status` returned 127 because no executable or
`RESEARCH_KB_BIN` was present. Published source and primary mathematical
references supplied the documented fallback. Live queue ownership remains
unknown without the shared control plane.

Presence-only inspection found no `CHECKPOINT_UPLOAD_TOKEN`, token-file
override, default checkpoint upload-token file, AWS environment credentials,
AWS profile/shared credential files or AWS CLI. No credential value was
printed. The documented checkpoint transport consequently has no available
authentication path here. No gate/upload or remote job was attempted, and no
admission was invented. Workspace PR #109 owns connection restoration.

## Evidence standing and ledger

`PCC-20261124-0c52ec` is the unsigned interpretive statement.
`PCC-20261124-3de82e` is the unsigned producer-local data statement.
Canonical `verify` returns `not_covered` and `standing` returns `unverified`
for these statements; no signing key, keyring or verifier was changed.
[pcc:unverified]

The canonical ledger checks are retained as `PCC_LEDGER_BRANCH.txt` and
`PCC_LEDGER_MAIN.txt`, with separate exit files. Append-only history is
preserved. Main-prefix reconciliation is an integration task; rewriting or
rehashing previous entries is not an acceptable substitute.

## Observed GitHub artifact migration

On source commit `9ce6aa69200ad155e5512d11e04b27c628f8fc56`, GitHub's
`migrate` check failed in the step `Upload changed data and replay remote
bytes`. The subsequent pointer-commit step was skipped. The log reports an
S3 `HeadObject` 403 for content hash
`a61cf6887d76c675608f5d4f5b2e71aebf9cbb556c6ce7abd797adbde81cd800`, the
earlier preserved prepared-input archive. That response alone does not
distinguish an inaccessible object from an absent key without list access.
The workflow exited 2. Artifact-store access must be reconciled before this
integration gate can complete.

[Observed job](https://github.com/isogeny-labs/crypto/actions/runs/45441435994/job/138613342662).
The minimal relevant log excerpt and step results are retained in
`CI_ARTIFACT_MIGRATION.json`. The same source had a skipped review check;
it did not receive a passing full CI verdict.
