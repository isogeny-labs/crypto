# Validation context

This follow-up starts at parent #2430 head
`f416adefe9fc43f3a68c5658b88e832eaedaf9ea`. Its only Rust change is the new
example. It does not modify library code, validators, leaderboard rows,
historical certificates or measured records. Earlier a720770 validation
results are not substituted for this worktree's checks.

## Verified checks

- `cargo test --release --example conductor_gap_confirmation --offline`:
  **12 passed, zero failed**. See [captured output](checks/example-tests.md).
- `cargo build --release --example conductor_gap_confirmation --offline`:
  succeeds. Complete-cohort `plan` and `audit` succeed; `preflight` returns
  **3 with status blocked**, as required. Native audit and separate `jq`
  counts agree on 128 classes, 4,096 class/target pairs, 32,768 distinct
  nonzero seeds/primary slots and 512 blocks. Zero solver calls occurred.
- The old native parameter verifier accepts all **128 native and six upper
  classes / 22 cells**; the new receipt is byte-identical to the frozen
  receipt, SHA-256
  `8a554733a867352643c14e2438ce05071c1d5cb10b9606149d9cfc21d3d775fb`.
- `target/release/backlog validate XP-20261026-conductor-gap-size-confirmation`:
  **ok**; card remains blocked. Its schema was first materialized from Git.
- `rustfmt --check --edition 2021 examples/conductor_gap_confirmation.rs`,
  `git diff --check` and the new `INPUTS.sha256` check pass.

## Full-library validation

The first `cargo test --release --lib --offline` attempt failed to compile
because six tracked `include_str!` inputs were excluded by the inherited
sparse checkout. See [initial output](checks/library-initial-sparse.log).
These were not missing from Git or established artifact-store failures.
The repository's minimal sparse profile materialized all six.

A second complete release-library run is in progress. It has already
reported failures in inherited bounds, weighted comparison and hyperelliptic
tests. This suite is not claimed passing. Its final result and failure
evidence will be appended when it finishes; no test is disabled or skipped.

## Site and validation-gate limits

The site suite stops before running tests on the unhydrated parent input
`docs/backend-leaderboard/leaderboard.json`. Its current manifest pins
**8,451,778 bytes**, SHA-256
`91d30a0f5d72dd191b5361f5c5a2699d8b0e6ac074b2f21957d8155daa75cab9`.
The documented artifact pull first hit a read-only default cache; retrying
with `ARTIFACTS_CACHE=/tmp/conductor-gap-artifact-cache` establishes that
the **AWS CLI is unavailable**. Remote read-back is not confirmed. No new
credentials or paid resources were provisioned. See
[site output](checks/site-tests-blocked.log) and
[artifact-pull output](checks/site-artifact-pull.log).

After materializing the tracked benchmark index, validation certificates
and historical sweep data, `python3 scripts/test_validation_gate.py` runs
**62 tests: 61 pass, one errors**. The committed-tree test stops with:

```
research/validation_sweep_20261019/B-benchmarks/verdicts.jsonl:1 names no row (needs `row` or `row_id`)
```

See [raw output](checks/gate-tests-baseline-schema.log). That first record
has `row_no` and `slug`, while the inherited parser requires `row` or
`row_id`. Both the parser and this input are unchanged from the parent
(`git diff --quiet HEAD --` on these and the leaderboard/benchmark indexes
passes). Exact parent hashes:

- `scripts/validation_gate.py`:
  `377bdb5e3b7f840f508ebfc171d1e5ca1dee0f871d122e8d315f5f1c273cf723`
- `research/validation_sweep_20261019/B-benchmarks/verdicts.jsonl`:
  `25d9230d8ce3752afeef95b83110f09c0a5e3fc3423cd226311e9e4f98fe1573`

Earlier sparse-only gate failures are not presented as a clean-checkout
source regression. The inherited schema disagreement and unconfirmed
artifact hydration are reported separately, with no parent-integration
patch in this change. A complete gate pass is not claimed.

## Publication and scientific limits

Conductor preflight exits 127 because the CLI is absent; no shared scope
lease is claimed. Exact-head CI, review and merge authorization must be
checked after publication; local example checks do not establish them.
No new scientific searches, curve constructions, solver campaigns, native
timings, cloud jobs, billing changes or unrelated merges were performed.
The measured crater conclusion and the open CM/algebraic-oracle obligations
are unchanged.
