# Validation context

The publication branch starts at the actual parent #2430 source-branch head
`2efb7830fb732928a5da8451648dc1fa40adaef7`. The initial checks used the
stale PR ref `f416adefe9fc43f3a68c5658b88e832eaedaf9ea`; the unpublished
change was reapplied onto the actual branch before publication. The two
parent commits have identical AGENTS.md and library source (`src/` tree
`2d7ba8b11b1414648c9f3ced61546fab002bd31e`); the intervening 198 changed
paths are a storage migration. This change adds one Rust example and does
not modify library code, validators, leaderboard rows, historical
certificates or measured records. Earlier a720770 results are not
substituted for these checks.

## Verified checks

- `cargo test --release --example conductor_gap_confirmation --offline`:
  **12 passed, zero failed**, including a repeat on the actual current
  parent plus this change at 7c841ed31. See [current-base output](checks/example-tests-current-base.md)
  and the preserved [initial output](checks/example-tests.md).
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

A second full release-library run has **no confirmed final summary or exit
status** after the execution environment reattached. No process remains.
The retained log reports terminal status for 5,015 distinct names out of
5,016 declared tests, including five failures. The unconfirmed case is
`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
No test was manually disabled or skipped, and no full-suite pass is claimed.

Focused reruns reproduce all five observed failures. Materializing the
tracked `research/ecbench_bounds_challenges_20261006/` fixture then clears
the bounds failure. **Four inherited failures remain:**

- Weighted comparison: `assertion failed: zero.legacy_comparison_id.is_none()`.
- Three hyperelliptic tests: `method ic.hyperelliptic has no parameter branches`.

See [complete focused traces and the passing bounds rerun](checks/library-focused-failures.md).
The full local log's hash and publication limit are recorded there; it is
not claimed durably uploaded. These failures are in the unchanged parent
library; no parent-code patch is made here.

## Site and validation-gate limits

The site suite stops before running tests on the unhydrated parent input
`docs/backend-leaderboard/leaderboard.json`. Its current manifest pins
**8,451,778 bytes**, SHA-256
`91d30a0f5d72dd191b5361f5c5a2699d8b0e6ac074b2f21957d8155daa75cab9`.
The documented artifact pull first hit a read-only default cache, then an
unavailable AWS CLI. A local development virtual environment now supplies
AWS CLI 1.46.2, but a normal STS identity probe returns **Unable to locate
credentials**; no outbound-identity selector manifest is configured. No
profile, key, role, billing or cloud capacity was created or changed.
Remote read-back is not confirmed. See
[site output](checks/site-tests-blocked.log) and
[initial artifact-pull output](checks/site-artifact-pull.log), followed by
the [current identity-setup failure](checks/aws-identity-unconfigured.log).

After materializing the tracked benchmark index, validation certificates
and historical sweep data, `python3 scripts/test_validation_gate.py` runs
**62 tests: 61 pass, one errors**. The committed-tree test stops with:

```
research/validation_sweep_20261019/B-benchmarks/verdicts.jsonl:1 names no row (needs `row` or `row_id`)
```

See [initial output](checks/gate-tests-baseline-schema.log) and
[current-base repeat](checks/gate-tests-current-base.log). That first record
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
