# Validation record

All experiment inputs are public synthetic known-answer fixtures. Algorithms,
curve construction, result metrics and experiment drivers are native Rust.
PARI fixture verification is native C. Python commands here only assemble or
validate static repository metadata or lay out recorded values.

## Passed experiment checks

- Release example: 3/3 tests passed, including exhaustive small-group
  c=1/2/4 decisions, secret-free recovery and stopping inside a residual
  at a one-pair-query budget. See example-release-tests.log.
- Primary: 12 curves, 24/24 verified recoveries, 11,370 dictionary checks,
  exact A/A counters and decisions. Follow-up: 4 overlapping larger curves,
  12/12 verified recoveries, 18,344 dictionary checks. Zero budget exhaustion
  or reference disagreement. Every successful relation passed group checks.
- Primary replay records byte-identical: replay.txt.
  This includes the eight baseline DLP A/A controls; see
  [control arrangement](../PROTOCOL_NOTES.md). Its 24 successful duplicate
  recoveries are not counted as new curve/coefficient cases.
- Native PARI 2.17.2 checked all primary fixtures and repeated all four
  larger fixtures: run directories' pari_checks.jsonl and pari.log.
- All report PDF pages and source figures were visually inspected. See
  PDF_INSPECTION.md. Benchmark files are exact copied artifacts with hashes.

## Repository checks and retained failures

- Initial `cargo test --release --lib`: 4,396 passed, 9 failed, 118 ignored.
  All nine failures were forbidden loopback sockets in the sandbox. The
  retry used granted network access, including loopback. Initial log retained.
- Full retry: 4,404 passed, 1 failed, 118 ignored. The remaining assertion
  expected 1,324 extension models; registering the new 12 makes 1,336.
  The test now expects exactly 1,336 and retains all field/subgroup checks.
  The focused final rerun is recorded separately. No experimental algorithm
  or mathematical acceptance condition was weakened.
- Strict Clippy with `-D warnings` fails in unchanged
  src/symmetric/modes/pmac.rs:112 under Rust/Clippy 1.99.1, lint
  `mismatched_bit_width_type`. clippy-strict.log preserves the failure.
  A separate diagnostic allowing only that existing lint passed; it is
  not described as a strict pass. No unrelated PMAC change is included.
- Native cover refresh: all 2,584 models verified; invalid=0, unsupported=0.
  The new curves require no cover for their measured solver. This refresh
  only satisfies the repository catalog/browser metadata joins.
- Browser generator freshness, browser unit controls (3 tests), and catalog
  join controls (3 tests) passed after explicitly fetching their missing
  public committed input blobs in the sparse partial clone.
- The separate catalog-wide transfer-assessment check still fails its
  hard-coded 2,100-row guard. The base commit already has 2,572 rows; this
  branch has 2,584. See assessment-known-failure.log and the
  [named metadata gap](../../../problems/catalog-cover-assessment-population-stale.md).
  The older assessment ledger is preserved; this PR makes no new transfer claim.
- The full curve-name linter initially stopped on sparse-excluded
  backlog/README.md. Added/changed materialized files and the generated
  alias map are checked separately. See curve-names-scoped.log.

## Reproduction and provenance

Primary runtime source fc13c52e; protocol 6095144c. Follow-up runtime and
protocol 040b8e1a. Retained snapshots and SOURCE_SHA256SUMS identify the
source, fixture engines and lockfile. Each run has exact argv, seeds,
limits and raw ledgers. The primary binary hash was not captured before
rebuilding; that limitation is explicit in its provenance. The follow-up
binary hash was captured before execution.

Producer: AMD EPYC 9V74 KVM, four-CPU cgroup quota, 16 GiB; L0 concurrent.
No native wall-clock performance is admitted. Conductor was bootstrapped
locally because this workspace had no configured shared endpoint; checks
and the local task claim are workspace coordination, not global exclusion.

## Additional record details

The first example compile had an E0282 inference error for an untyped
`Vec::new`; an explicit `Vec<Value>` fixed it before measurement. The
terminal diagnostic was not saved as a separate artifact. Final release
example tests and successful build logs are retained. Raw measured records
were not altered to address any compiler or repository gate failure.

Native cover and transfer metadata were rebuilt and their native `--check`
/ `check` modes passed. These are catalog certificates/formulas, not
measurements of cover-based DLP solving. No covering curve enters the
weighted solver. Generated SVG path whitespace and raw logs are excluded
from source whitespace checks; their exact evidence bytes and hashes are
preserved.

Final focused release registry test: 1 passed, 0 failed (4,522 filtered
out), after the exact expected-population update to 1,336. The preceding
full retry's other 4,404 tests passed. This record does not call that
preceding full retry an all-green run. See registry-test-final.log.
