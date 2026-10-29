# TR-04 development validation, 2026-10-29

The requested four-part finite experiment is implemented locally. **The primary
research census has not run, the protocol is draft, and no solving advantage
is established.** This record covers development checks only.

| Check | Observed status |
| --- | --- |
| Standalone native tests | 40 passed: 35 library, 2 legacy binary, 3 CLI; zero failed/ignored/filtered |
| Strict all-target Clippy and formatting | Both exit 0 |
| Fixed known-answer preflight and artifact-directory replay | Both exit 0; development M=3 calibration, separate from primary M=8 |
| Draft protocol JSON Schema | Valid under the canonical v1 schema using maintained Ajv |
| Workflow | Actionlint, ShellCheck, Bash syntax, six wrapper and six checkpoint lifecycle stubs pass; actual remote transport/runner readiness unverified |
| Partial report/export | Synthetic inputs only: one failed logical attempt, 26 absent rows, expected CLI exit 1; canonical export schema valid |
| PDF inspection | Both pages of the draft plan and both pages of the synthetic report visually inspected; no clipping observed |
| Root-library validation on base `24def3ae4dfe94bbbee5f30245847b372d02fa70` | **Incomplete**, with no final summary or exit receipt |

The root-library attempt first failed on a tracked sparse path dependency. That
failure is retained. The subsequent tool session disappeared at the environment
handoff; its log ends while compiling `isogeny_algos`. No matching process remains.
The interruption receipt records a null exit code and signal because neither
was observed. No clean pass or ongoing run is claimed. The check-in explicitly
forbids a repeat broad-library run, so none was started. Repository validation
required before pushing remains unsatisfied for this new branch.

The earlier completed 5,143/0/123 run on source `0044aaf9` belongs to PR #2857's
separate immutable evidence. It does not validate this later base or TR-04.
The old `4c871` SIGKILL and earlier new-compilation SIGKILL records remain separate.

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

Remaining prerequisites are completed root-library validation, final review,
source publication, measured runtime estimation and source/analysis protocol
freeze, then the authorized supported checkpoint-gated execution and collection.
Nothing in these development checks establishes all 531 primary dispositions,
27 completed export rows, a new scalar-preserving map, or a runtime gain.
