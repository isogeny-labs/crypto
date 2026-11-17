# TR-04 development validation, 2026-10-29

The requested four-part finite experiment is implemented locally. **The primary
research census has not run, the protocol is now frozen, and no solving advantage
is established.** This record covers development checks only.

| Check | Observed status |
| --- | --- |
| Standalone native tests | 40 passed: 35 library, 2 legacy binary, 3 CLI; zero failed/ignored/filtered |
| Strict all-target Clippy and formatting | Both exit 0 |
| Fixed known-answer preflight and artifact-directory replay | Both exit 0; development M=3 calibration, separate from primary M=8 |
| Frozen protocol JSON Schema and native metadata/hash | Version3: both pass; code `f270363df`; earlier attempts retained |
| Workflow | Actionlint, ShellCheck, Bash syntax, six wrapper and six checkpoint lifecycle stubs pass; actual remote transport/runner readiness unverified |
| Partial report/export | Synthetic inputs only: one failed logical attempt, 26 absent rows, expected CLI exit 1; canonical export schema valid |
| PDF inspection | Both pages of the draft plan and both pages of the synthetic report visually inspected; no clipping observed |
| Root-library validation on base `24def3ae4dfe94bbbee5f30245847b372d02fa70` | **Incomplete**, with no final summary or exit receipt |
| Final root-library validation on `445d84bf7f57750e0e34bd7a65992c36c0da92ec` | **Completed: 5,389 passed, 0 failed, 123 ignored, 0 filtered; Cargo exit 0** |

The root-library attempt first failed on a tracked sparse path dependency. That
failure is retained. The subsequent tool session disappeared at the environment
handoff; its log ends while compiling `isogeny_algos`. No matching process remains.
The interruption receipt records a null exit code and signal because neither
was observed. No clean pass or ongoing run is claimed for that old attempt.
The PR #2857 check-in excludes a repeat broad-library run for that evidence/docs
PR; the later clarification preserves the independently requested TR-04 work.
A separate final root run on `445d84bf7` completed in an isolated worktree
after synchronization with the selected main snapshot. It ran from
22:11:17 to 23:23:56 UTC with 5,389 passed, zero failed, 123 ignored and
zero filtered; Cargo exit 0. The outer suite contained 5,512 tests. Two earlier
one-test summaries are nested child runs, not the terminal full result.
The [collected receipt](root-validation-445d84bf7.json) binds the exact source,
tree, raw-log hash, exit receipt and terminal summary. A superseded `fe0e1dac6` compilation
was intentionally stopped before tests with compiler SIGTERM 15 and Cargo exit
101 to select the intended source. Every attempt remains distinct.

The earlier completed 5,143/0/123 run on source `0044aaf9` belongs to PR #2857's
separate immutable evidence. It does not validate this later base or TR-04.
The old `4c871` SIGKILL and earlier new-compilation SIGKILL records remain separate.

The complete root-development archive, including distinct earlier attempts,
is preserved locally as `/workspace/tr04-root-validation-445d84bf7.tar.xz`.
Its bytes/hash are in [development-status.json](development-status.json).
At 80,316 bytes it exceeds the repository's 64 KiB policy, so it is not added
to Git. Configured artifact-store publication and independent download remain
pending; neither is implied by the local validation or small tracked receipt.
No raw log was split or truncated to evade that policy.

Independent development review found and corrected artifact relocation during
analysis-source validation, primary source-pin verification, and report receipt
crossbinding. Compiled source bytes are checked against the declared Git commit;
primary runs require a recorded clean tracked checkout. Tests reject edited
compiled bytes even after the worktree is restored, missing/changed receipt
bindings, fabricated attempts and changed portable evidence. Primitive work
counters are not independently recomputed; structural arithmetic, rank, census
and digest replay have their explicitly narrower scope.

[development-evidence.tar.xz](development-evidence.tar.xz) preserves successful
and failed development logs, actual exit receipts, tested source fingerprints,
the known-answer preflight records and the clearly labelled synthetic partial
report. It is 40,960 bytes, SHA-256
`630a6b558984d0dae5a1973b59b072f6bb0f8215c71aedb56c6f401025133d71`.
The archive passed `xz -t`. It contains no primary research results. The synthetic
PDF predates the final receipt-crossbinding hardening; its old analysis source
is intentionally retained, while final native tests cover the current code.

The later [transport development archive](development-transport-evidence.tar.xz)
retains seven synthetic transport cases, deterministic repacking, updated static
checks, native-source reuse proof and the intentional superseded compilation.
It establishes no actual remote upload or download.

Runtime planning and protocol freeze are now complete. Remaining prerequisites
include checkpoint transport and primary execution. Source publication and
collection of the first failed workflow attempt are complete; its
[failure report](reports/run-20261029T233325Z/README.md) records checkpoint
exit 1 before any field starts. A separate reviewed artifact-pattern defect
is corrected and regression-checked without a rerun. Primary coverage and
durable experiment delivery remain unverified.
Nothing in these development checks establishes all 531 primary dispositions,
27 completed export rows, a new scalar-preserving map, or a runtime gain.

## Later main integration and publication

After the Copilot conflict request remained unanswered for nearly four hours,
the documented fallback merged the single fetched main snapshot
`f81b1c6c562d10251eae883c8084affddc060f8b` with two parents. The fallback
explanation was posted before editing. Main's complete daily-log bytes and
the feature-only appends are preserved verbatim; only the two logs differ
from Git's automatic combined tree. The original failed-run report, frozen
protocols and native code are byte-identical to the prepared `ca40b6841` tree.

All 40 standalone native tests, strict Clippy/formatting, Actionlint with
ShellCheck, the routing regression and native frozen-protocol check pass.
The inherited main changes 89 root-runtime paths relative to validated
`445d84bf7`. No new full root run was authorized or launched, and that older
pass is not extended to this merge. A broad whitespace scan reported
pre-existing main-figure whitespace and was stopped; affected-path whitespace
checks pass. No unrelated figure was patched. Raw validation receipts are in
`postmerge-validation-20261030.tar.xz`, indexed by
[postmerge-validation-20261030.json](postmerge-validation-20261030.json).
