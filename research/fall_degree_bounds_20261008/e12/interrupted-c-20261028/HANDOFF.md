# Scoped handoff: interrupted E12c evidence

Conductor: T-711. Branch: `codex/e12c-interrupted-observations-20261028`.
Scope: this directory and `../../RESULTS.md`; the shared daily log is not edited.

## What was done

- Reconciled the clean worktree and merged PR #2758 before branching from current main.
- Downloaded the original terminal receipt, snapshot, final output files and logs.
- Verified archive SHA-256, exact latest/immutable manifest equality, and all 24 final-output files against the archive.
- Retained the one raw N=84 window record unchanged; missing records are not measurements.
- Added the root session's canonical S3 PUT/GET checksum and byte-comparison receipt.
- Prepared an observational report, completion diagram and native Typst source.

## Decisions and boundaries

- One of four requested completed records exists; zero window certifications.
- Spot interruption and exit 143 ended the attempt. The generic resume message supplies neither solver-memory recovery nor new-attempt authorization.
- Recorded gates are preserved; no solver, gate, solution-count or independent closure replay was performed.
- No formal experiment-export/review schema validation or official research decision is claimed. Legacy Python research validation tools were not run.
- Existing unrestricted degree KPIs and plots remain unchanged: a null window result cannot count as a censored unrestricted degree.
- No new experiment, new attempt, IAM modification or legacy-bucket write occurred.

## Validation and delivery

- Native hash, byte-comparison, JSON and record-count checks passed.
- Typst 0.15.1 rendered a one-page PDF. Native Poppler 24.08 decoded its text and image; visual inspection found no clipping. `pdfinfo` exited 0 with a `Suspects object is wrong type (boolean)` metadata warning.
- Copied-evidence SHA-256 checks, terminal/draw JSON consistency and whitespace checks passed. Raw evidence commit: `cbb6b15251177cb7eb057173b54fcc8ce7b0b9e3`.
- A second reader in this shared session repeated the seven hashes and raw identity/null-field checks and found no report overclaim. This was a record-consistency review with shared context, not formal independent experiment review or solver replay.
- The initial required Cargo build exited 101 because the sparse checkout omitted 36 unique tracked include files. Those were restored without source changes. The complete serial library run then recorded **4,967 passed, 20 failed, 122 ignored**; all 20 failures identified additional omitted runtime fixtures.
- After restoring those tracked fixtures, **all 20 exact failed tests passed** on the same compiled binary (SHA-256 `038ac60841136d626ee61348a6fc1d36e60e4dd5b58ef97a954cbb4a2a154b2c`). The test-name list matched the failure list exactly. No implementation or test code changed. The initial full run is not described as green, and the full suite was not repeated after fixture-only restoration.
- All three validation logs (failed initial compile, complete serial run, successful exact reruns) were PUT to content-addressed canonical S3 keys, downloaded again, byte-compared and hashed. [delivery-checks.json](delivery-checks.json) retains the exact counts, source tree, binary identity, test names, URIs and readback receipts. These are software delivery checks, not experiment replay.
- PR/CI state will be recorded after publication; this handoff does not claim overall E12 completion.

## Open work

- Three E12c completed records are missing. A new attempt requires authorization and a frozen amendment.
- Durable output snapshots preserve rows, logs and provenance, but omit the roughly 30 GB closure basis and full in-flight state. Complete intermediate persistence and exact solver recovery remain unsupported.
- Independent algorithm validation and formal experiment review remain unperformed.
- E12a and the broader first-/last-fall questions are separate unfinished work.
