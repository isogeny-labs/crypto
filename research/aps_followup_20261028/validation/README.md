# Validation receipts

These are development correctness checks, not a performance experiment or
an independent replay certificate. No cryptographic-size result is claimed.

## Completed checks

- All 20 APS module tests passed in the final full-suite invocation. They
  include exact-set comparisons with the legacy scan and point addition,
  all streaming routes, a mixed informative/uninformative stream, horizontal
  cases, certified pruning and its independent-label counterexample.
- The touched example passed seven tests, including the shipped certificate
  fingerprint and native check. `example-tests-final.txt` retains the output.
  Its earlier six-test run under default codegen is also preserved.
- Final focused checks passed 63 F4-related and 15 GLV tests. See
  `aps-final.txt`, `f4-final.txt` and `glv-final.txt` for completed invocations.
- Strict library and touched-example Clippy passed with test targets, on
  Rust 1.99.1 and CI's Rust 1.98.0. The extra shipped-certificate test passed
  the Rust 1.98 lint too. Rust 1.98 formatting and actionlint also passed.
- Both pinned Lean gates audited seven theorems, zero failing and zero
  trusted. See `lean-*-audit.txt`, `lean-*-build.txt` and
  `../lean/proof-dependencies.json`.

The final full library invocation exited 0: **5,002 passed, 0 failed,
122 ignored**. `full-library-summary.txt` records its command, source and
raw-log SHA-256. The full stdout remains in the execution workspace; the
committed summary is not a copy of that complete log. Published focused
outputs normalize absolute local paths to `REPO`, `BUILD_DIR` and
`LOCAL_RECEIPTS`, and omit build-lock diagnostics after automatic review
rejected a receipt containing internal workspace/build metadata. Test outcomes,
failures and commands are preserved. Original copies remain local;
`receipt-sha256.json` records both original and published hashes. Commands are
in `commands.txt`; source bytes are pinned in `source-sha256.txt`.

Detailed Clippy dependency/build outputs are withheld after automatic review
rejected their broad toolchain metadata. `lint-summary.txt` records the completed
checks; their original and normalized hashes remain in `receipt-sha256.json`.
The full-suite summary also omits host/resource/path/build metadata and elapsed
duration after publication review rejected those fields.

The final development release build uses optimization level 3 with
`CARGO_PROFILE_RELEASE_CODEGEN_UNITS=256` and `CARGO_BUILD_JOBS=2`. The
focused CI uses the same setting. This changes build partitioning, not the
test workload; no compile-time or algorithm-speed comparison is reported.

## Retained failures and corrections

1. Initial compilation lacked four tracked compile-time fixtures. The
   maintained CI materializer restored 170 tracked inputs before retrying.
2. `draft-aps-failure.txt` records 18 passing tests and one failed assertion:
   its purported positive partition had no relation. Positive fixture
   construction now uses the independent point oracle in the test only.
   It does not select a performance sample or enter the relation solver.
   This draft log is failure history, not final-source validation.
3. `draft-f4-tests.txt` records 63 passing F4-related tests from the draft
   integrated binary. Final-source tests are covered by the final full run.
4. The first full invocation observed seven failures due to absent runtime
   fixtures, then was explicitly terminated as superseded. It is neither a
   completed full-suite pass nor a completed full-suite failure. Restoring
   the tracked C++ emulator, r24 records and four cited planner reports made
   all affected checks pass without changing their source or assertions;
   `restored-device-tests.txt`, `restored-band-test.txt` and
   `restored-planner-test.txt` retain those reruns.
5. A superseded compilation was stopped rather than counted as a check.

The checkout uses custom non-cone sparse patterns, so the profile-managed
`sparse-checkout.sh add` command could not adopt it. `git sparse-checkout
add --stdin` widened the existing patterns for the exact tracked fixtures.
No missing fixture was fabricated or test disabled.

## Boundaries

The point oracle is independent of polynomial construction, and uses the
same native curve arithmetic as the producer. A separate EC implementation
and standalone transcript replay are still owed. Coefficient proofs do not
formalize the full chord law or Rust correspondence. Root/solver counters
are stage diagnostics, not complete cost accounting.

The required checkpoint upload was rejected by automatic approval review
because it packages source and local changes for S3. It was not retried or
bypassed. No performance experiment, external-solver panel, or EKS job was
launched. The earlier complete-cost card remains open.
