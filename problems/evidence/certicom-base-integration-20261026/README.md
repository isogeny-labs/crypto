# Certicom base-integration review, 2026-10-26

This is source integration and host correctness evidence for existing crypto
PR #2250. It is not a physical GPU receipt, benchmark, public-challenge solve,
or physical-device qualification.

The normal merge combines task parent
`4f0a505dc69d15989e341332cfb03f5e28f55037` with observed main
`e8fcadf62cd0b674c40ebc4e76dfe88e26e71f5c`. After more than eight hours
without a response to Copilot request #6412766526, the author explained the
current AGENTS fallback in [comment #6423377885](https://github.com/isogeny-labs/crypto/pull/2250#issuecomment-6423377885)
before editing conflicted files. Both problem-index additions and all three
Certicom module declarations survive. No shared arithmetic, solver, worker,
CLI assertion, method, curve parameter, correctness budget or test ignore was
changed. The two merged cryptanalysis deliverables are outside monitoring
and publication scope.

The receipts record these distinct observations:

- `merge-audit.json` binds all 21 shared files and normalized CLI assertions
  to the merged suite source. Incoming research, benchmark and paper trees
  match main; all incoming problem-index rows survive.
- `source-pin-lineage.json` lists the nine generated SHA-256 changes. Two
  bind task integration sources; seven bind files identical to main. Every
  other assessment JSON value is unchanged. The documented Node generator's
  byte-exact check passes; that is not semantic validation.
- `verifier-results.json` retains the official independent Node verifier's
  six merged-source errors and 14 exact-base errors. All six catalogue,
  model-membership and PDF-marker errors also occur on unmodified main.
  The remaining eight base errors are stale source-artifact hashes. No
  frozen record, catalogue, report, graph, PDF or gate was edited to clear
  these failures. `base-verifier-control.json` identifies the isolated
  exact-base checkout and its own tracked inputs.
- `restored-inputs.json` binds all 105 materialized sparse-checkout inputs
  to their exact merged-index Git blobs. No historical input bytes changed.
- `validation-status.json` is a timestamped snapshot: 14 focused host
  solver/device-policy checks, 14 real CPU/emulator `icx` CLI executions,
  and seven actual release NIST integration cases pass. Eighteen distinct
  fixture controls pass across the recorded replays. At that earlier snapshot
  the complete library run was still running, not passed. Its compile-input and session
  fixture failures remain disclosed; nested child-test result lines are
  not a complete full-library result.
- `host-validation-completed-20261026.json` supersedes the earlier running
  state with the complete clean-input result against code commit
  `c3f3c1a2b4314e42c728c517c4b9e4dda176fcea`: 4,878 passed, 0 failed,
  121 existing ignored, no filtering, exit 0. All 14 task checks passed.
  It records the full local log's byte count and SHA-256 without claiming a
  durable remote raw-log upload. The earlier full result of 4,870 passed,
  eight missing-fixture failures and 121 ignored remains disclosed.

The official independent checker was invoked identically in the merged
checkout and exact-base control:

```sh
node research/cover_transfer_assessment_20261013/verify_assessment.mjs
```

The original full-suite command is `cargo test --release --lib`, with
network/loopback enabled. Its command receipt and complete stdout/stderr
remain under the explicitly named `/tmp` paths in the snapshot. Those paths
are ephemeral local observations, not durable uploaded artifacts. The
formerly long-running test's module is byte-identical to the pinned base; this establishes
source identity, not a separate completed execution on main. No timeout,
cancellation, ignore or narrower substitute was applied. The complete rerun
finished normally at 2026-10-26T18:58:03.931Z; its host-suite pass is distinct
from the still-failed semantic gate and original physical requirements.

Current-head applicable CI/review, the failed independent semantic gate,
and original physical GPU requirements remain open. Complete host-library
validation is now established for the recorded code commit. Keep PR #2250
open with auto-merge off. No new PR, paid resource,
billing change or merged-branch update is authorized by these receipts.
