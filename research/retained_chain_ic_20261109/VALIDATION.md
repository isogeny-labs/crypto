# Development checks and current limits

All observations below are producer development checks `[pcc:unverified]`.
Claim record: `PCC-20261109-fae4fa` (statement, no independent proof standing).
None is a cloud scaling or timing result.

| Check | Observed outcome |
|---|---|
| `cargo check --bin ecbench --no-default-features` | Completed successfully, including the study module |
| `cargo test --release --lib retained_ --no-default-features` | 15 passed, 1 existing ignored, 0 failed; includes six new retained-chain/pipeline tests |
| Final-source default-feature library binary, `retained_ --nocapture` | 15 passed, 1 existing ignored, 0 failed |
| `cargo test --bin ecbench --no-default-features retained_study_plan` | 1 passed; both registered L2 JSON specs parsed and planned |
| `cargo test --release --lib --no-default-features` | Started 6,280 tests; interrupted after environment failures identified; not green and not complete |
| Device-test retry after restoring `gpu/certicom/` | All 6 passed |
| Historical bounds-test retry after restoring its session | Passed |
| `cargo test --release --lib` with default features | Built successfully; full run interrupted after 7 known environmental failures; not green and not complete |
| `git diff --check`; PCC prose lint | Passed at preparation time |
| PCC ledger check | 484 entries; hash chain accepted locally |
| PCC canaries | 59/59 classes behaved as expected locally |

The final default-feature binary includes the help-text correction and separate
projective/guessed-branch counters. The tested implementation is source commit
`1e6184a7935f016fa1fa28cba5864ef964ada387`; subsequent handoff changes only
documentation/metadata. Final focused confirmation completed at 2026-11-09
16:45 UTC. A duplicate historical PKM-only run was interrupted while slow
existing tests were still running; no pass is claimed for that whole filter.

Local default-feature test binary SHA-256:
`e0fc3c3b6239e707aaf2bd650bda5c9af880cf2a11dac2e9c12b9053321432b5`.
Focused stdout/stderr SHA-256:
`9071a17d369d0cf1cbe69399a2958ccc71f583f4f8e059209b01305041ea77fc`.
Raw development logs remain in the working environment; they have not been
archived to object storage and are not an independently attested receipt.

## Full-suite environment failures

The initial full attempt reported these categories before interruption:

- Five device tests could not find tracked `gpu/certicom` sources in the
  sparse checkout. Restoring that directory fixed the focused device suite.
- Four band tests and one comparison test require the archived
  `research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`.
  The tracked manifest names SHA-256
  `8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738`
  and 2,293,097 bytes. Restoring the tracked session does not hydrate this
  object. The maintained artifact helper reports no AWS CLI; the unsigned
  read fallback returns HTTP403. No substitute data was generated.
- The historical bounds test initially lacked its tracked session. Restoring
  `research/ecbench_bounds_challenges_20261006/sessions/prime-toy-reference-e1`
  made the focused check pass.
- Two existing `diagnostic_reap_publication_regression_tests` fail in
  `DiagnosticSessionClock::begin`: `/proc/self/ns/time` and
  `/proc/self/timens_offsets` are absent in this runtime. No runner semantics
  or assertions were weakened to make these tests pass.

This is not an exhaustive clean full-suite result. Its failures are retained,
and the normal CI/review gates remain required. The separate cloud admission
gaps are recorded in [admission.json](admission.json).

After installing the workflow-pinned AWS CLI in a local tool environment,
the configured-client STS request and artifact retry both report `Unable to
locate credentials`. The named `ecc-challenges-new` profile cannot be found.
The checkpoint gate, with that client on PATH, still exits 1. Installing the
client therefore did not resolve identity or upload access. No cloud build,
paid job, credential export, or permission change was attempted.
