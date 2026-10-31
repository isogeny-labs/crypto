# Correctness validation record

These are software/proof checks, not a controlled performance experiment.
There is no `experiment.export/v1`, speed comparison or independent DLP replay
certificate for this study. The complete-cost panel remains blocked.

## Completed checks

| Check | Observed result | Receipt |
| --- | --- | --- |
| Exact module tests in isolated native review build | 15 passed, 0 failed | [isolated-tests.log](isolated-tests.log) |
| Same public example source's tests in that build | 4 passed, 0 failed, including random-label paper baseline for m=3 and split m=4 | [isolated-tests.log](isolated-tests.log) |
| Locked repository release example tests | 4 passed, 0 failed | [release-example-tests.log](release-example-tests.log) |
| Final locked repository release module tests | 15 passed, 0 failed | [release-module-tests.log](release-module-tests.log) |
| Strict lint of the changed module, its tests and example in isolated build | Passed; no lint suppression for the reported defects | [isolated-clippy.log](isolated-clippy.log) |
| Changed-file rustfmt, repository CI toolchain 1.98.0, `skip_children=true` | Passed | [rustfmt.log](rustfmt.log) (empty success log) |
| Native S3 identity issue plus independent extra seed `(9001,32)` | Accepted, `IDC1h32f0b3e300bbf951` | [identity.json](../identity.json), [generation log](certificate-generation.log) |
| Pinned Lean repository proof gate | PASS; 16 audited theorems, 0 failing, 0 trusted | [build](lean/build.log), [audit](lean/audit.log), [status](lean/status) |
| Focused workflow syntax/expressions | `actionlint` passed | Command: `actionlint .github/workflows/aps-single-relation.yml` |
| Deferred experiment card | Native `backlog validate XP-20261026-aps-complete-cost-panel` passed | [backlog-validation.log](backlog-validation.log) |
| Report PDF | Built with Graphviz/pdfLaTeX; all three pages rendered and inspected | [report.pdf](../report.pdf), editable `.dot`/`.tex` beside it |

The review build used Rust 1.99.1, a scratch crate depending on this Crypto
checkout, and an exact copy of the new module. Its `cryptanalysis` namespace
reexported `f4_fp`, `identity_certificate`, `prime_fast`, and `residual_walk`
from Crypto. The example source was imported directly. The scratch dependency
lock is distinct from Crypto's lock, so this supplements rather than replaces
locked repository integration checks. The module copy was compared byte for
byte after the last code edit. The source hashes are in
[source-sha256.txt](source-sha256.txt).

Preferred integrated reproduction:

```sh
cargo test --release --locked --lib aps_single_relation -- --test-threads=2
cargo test --release --locked --example aps_single_relation_check
tools/lean/proof-gate.sh research/aps_extensions_20261026/lean/Coefficients.lean
```

The tests' point oracle is independent of polynomial construction but uses
the same native curve arithmetic. It is not an independently implemented EC
backend. Fixed known-answer fixtures are outside the solver's arguments.

## Repository integration status

`cargo test --release --locked --lib -- --test-threads=2` was launched against
the implementation. The new module's 15 tests passed in that release binary;
the full suite is still running at this entry. The final targeted locked
release checks have completed: 15 module tests and four example tests pass.
The full run has reached the end of its other test output with the unchanged
`jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`
still active; no failed test is reported in that implementation run so far.
No completed full-suite pass is claimed here.

The full run is the real process still running in this workspace, not a
scheduled future experiment: command above, test executable PID 25856,
log `/workspace/crypto-setup/final-library-tests.log`, Conductor wrapper
log `/workspace/crypto-setup/conductor-validation-final.log`. The wrapper's
following example-test command is harmlessly redundant with the completed
example receipt. Collect the final exit status and append a receipt before
calling the full suite passed. The old pre-change run was independently
still waiting on this same untouched test when terminated.

Strict root lint on the repository's CI Rust 1.98 toolchain was attempted:

```sh
cargo +1.98.0 clippy --locked --lib --example aps_single_relation_check -- -D warnings
```

It fails with **seven errors in untouched files**:
`jv_bitops.rs` (doc-comment blank line), `certicom131_symlp/gf2w.rs`
(two divisibility lints and a bool comparison), `certicom131_symlp/relgraph.rs`
(map entry), `ghs_smooth.rs` (doc indentation), and `koblitz_groebner.rs`
(thread-local const initializer). See [root-clippy-1.98.log](root-clippy-1.98.log).
These files match the base commit; they were not edited or suppressed to
make this PR green. The isolated lint check covers the changed source while
this integration gate remains open.

## Development failures and corrections

- The first scratch run passed 7/12 tests. The point-oracle/divisor checks
  caught the use of F4's **monic** normalizer inside expression arithmetic.
  It changed relative coefficients and caused missing or false relations.
  Expressions now merge terms without rescaling; whole equations alone are
  made monic. A separate arithmetic regression test was added. The later
  expanded 15-test battery passes.
- Additional linting caught two new divisibility-style issues and a cloned
  reference in a test. They were fixed. The final isolated strict check
  passes; unrelated root lint failures remain recorded above.
- A full pre-change baseline test run found a missing sparse-checkout fixture
  for `ecbench::bounds::committed_version_1_bounds_re_derive_under_their_recorded_version`.
  `docs/bounds/` and `research/ecbench_bounds_challenges_20261006/` were
  materialized without changing tracked bytes. The duplicate old run was
  terminated after the corrected implementation's full rerun was in flight;
  it is not counted as a passed full suite. Its long-running untouched
  `jv_isogeny_walk` test was still active at termination.
- Initial standalone Lean names used two unavailable cast lemmas. Those
  names were removed before the passing proof-gate receipt. No unchecked
  declarations were accepted.

Conductor prerequisites were resolved with the repository-documented local
setup and scoped claims before integration. The S3 credential is absent.
No benchmark experiment, cloud worker, funded objective or timing panel was
launched, and no benchmark checkpoint bypass was used.
