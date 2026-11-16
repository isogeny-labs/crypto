# Blobless Cargo checkout follow-up

Observed head: `28bb1f67f51f17eefb24053b689311317b360c29`.
Base: `c6d64436a33359320ad9d5b42c4acebee2968b9a`.

## Change-caused failure

[Leaderboard job 117525528405](https://github.com/aburan28/crypto/actions/runs/39076802361/job/117525528405)
passed input materialization after the preceding `--no-cone` correction, but
Cargo then exited 101 before compiling the package or running its tests:

```text
failed to determine package fingerprint for build script for backend-leaderboard
Failed to update the excludes stack to see if a path is excluded
```

The initial sparse patterns excluded the root `.gitignore`. In a shallow,
blobless checkout that blob was also absent from the object store. Cargo's
Git-backed file scanner could not read it. The fix adds `/.gitignore` to the
workflow's initial sparse selection. No producer, dependency, reproduction
lock, generated report, historical source, session or receipt changes.

## Local negative and positive controls

A new Git repository fetched this exact head with `--depth=1 --filter=blob:none`
from GitHub, using the workflow's original non-cone sparse patterns. It had
25 initially materialized tracked files and no object-store alternate.
Executing all three actual workflow `run` blocks reproduced exit 101 and the
same excludes-stack error. `GIT_NO_LAZY_FETCH=1 git cat-file -e` confirmed the
root ignore blob `0198f3d40b94809928c46e3ecc72a2a7304bb0d5` was absent (exit 1).
The trace identifies Cargo's `list_files_gix` path.

Then only `/.gitignore` was added to the sparse selection. Its blob became
available, and the same three actual workflow blocks exited 0:

- Rust 1.99.1 format, all 20 native tests and Clippy with warnings denied;
- committed-view freshness;
- locked report reproduction: all 115 input/build files and five outputs.

`workflow-run-blocks.sh` contains the exact commands extracted from the
corrected YAML, prefixed by `set -euo pipefail`. `before.log.gz`,
`trace-before.log.gz`, `after.log.gz`, `reproduction.json` and
`local-validation.json` retain the observations and content hashes.
Cargo dependencies were cached and `CARGO_NET_OFFLINE=true`; repository input
materialization used GitHub. This validation used local Git 2.52.0; the hosted
job used Git 2.55.0. The focused workflow actionlint check also passed.

The earlier fixture in [ci-20261017](../ci-20261017/README.md) used an object
alternate. Its passing test did not expose this missing-blob condition.
That evidence remains unchanged; this stricter checkout control supplements it.

To reproduce the checkout condition with Rust 1.99.1 available, initialize a
fresh repository, configure its origin as `https://github.com/aburan28/crypto`,
set `core.sparseCheckout=true`, `remote.origin.promisor=true` and
`remote.origin.partialclonefilter=blob:none`, and place
`initial-sparse-checkout.txt` at `.git/info/sparse-checkout`. Then:

```sh
git fetch --no-tags --no-recurse-submodules --filter=blob:none --depth=1 origin \
  28bb1f67f51f17eefb24053b689311317b360c29
git checkout --detach FETCH_HEAD
# Run the retained workflow-run-blocks.sh: expected exit 101.
# Assert the root ignore object is absent without lazy fetching.
GIT_NO_LAZY_FETCH=1 git cat-file -e 0198f3d40b94809928c46e3ecc72a2a7304bb0d5
# Expected exit 1 above. Add exactly the missing selection:
printf '/.gitignore\n' | git sparse-checkout add --stdin
# Run the same workflow-run-blocks.sh: expected exit 0.
```

Keep the expected negative exits separate from a fail-fast positive run.
A server that supplies extra blobs can defeat the negative control; do not
count a checkout with the ignore blob already present as this reproduction.

## Current-head merge gates and limits

`check-runs.json` records the single check-in snapshot for the observed head.
The all-targets guard was still queued; successful `replay` and `review`
service jobs were not a complete green CI result or a submitted head review.
The only submitted approval was on `a86219ad`; current Bugbot and Approval
Agent checks were neutral after usage-limit failures. No inline finding was
reported. No merge was attempted.

Seven new decoded hosted logs are bound by `job-logs.json`.
`base-comparison.json` confirms the same 19 base/head Git objects as the
previous triage. The inherited scope syntax, duplicate workflow keys,
historical registry/name mismatch, fail-closed harness guard and three site
failures remain. See the [open baseline gate problem](../../../../../problems/backend-frontier-baseline-ci-gates.md).
The root library's separately observed 26 compiler errors remain open; the
hosted scope guards did not compile the library. Unchanged root and site
suites were not repeated locally for this checkout-only correction.

The report lock remains
`0871cabc1de47a7b47f972e312295a181e8be0e23de9d71a8cd16ddbeabba8ee`.
These are workflow/software checks on the same local host, not fresh solver,
independent-host, arithmetic or timing replication. Historical binary/source
provenance and missing field-size measurements remain unresolved. No solver
campaign, measurement or promotion was run, so canonical performance figures
and the research report PDF remain unchanged. Hosted checks and applicable
reviews must pass on the correction's final head before merging.

Conductor was attempted before editing and was unavailable (exit 127).
