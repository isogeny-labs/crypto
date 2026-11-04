# 4 November source-bound development recheck

Native source: `507829726fbc731543f4527348b55c5e479242c9`, an external merge
already published on this PR. This check-in fast-forwarded to it; it did not
create another main merge. The earlier parser failure is repaired in that source.
`source.sha256` pins the native feature files and Cargo.lock. No native source
was edited during this check-in.

Local cache12/12 and ordered-lift1/1 rechecks passed; their exact logs are
cached.txt and ordered.txt. E2E and full-library rechecks remain pending. The
full suite has two reproduced failures in diagnostic resource publication,
whose source matches current main. `reap-reproduce.txt` is the exact isolated
module result: two passed, two failed. This host lacks `/proc/self/ns/time`
and `/proc/self/timens_offsets`, required by DiagnosticSessionClock::begin.
The full parent suite is still running; it cannot be reported as passed. The
historical passing suite and earlier parser failures remain in their original
records. `ci-inspection.json` and the CI log excerpts identify exact-head
size/gate/artifacts outcomes and the unrelated benchmark-row failure.

The workflow change gives the metadata-only checkpoint checkout a dedicated
path and working directory. It adds no artifact credentials to that job and
preserves all build/experiment conditions. `wiring-transform.txt` uses the
existing administrative generator with the three paths reported by CI: the
published head reproduces the mismatch, upstream main and the repair match.
This focused check does not claim complete artifact-store discovery. Two
full-store scans were interrupted (exit143) while hydrating unrelated promisor
manifests. Their empty outputs establish no admission result. Actionlint1.7.12
and formatting passed; the actionlint process emitted no output.

The first detached full-library attempt left only a Cargo-lock wait message
and no live driver or terminal receipt. Its log is retained separately and it
is not a completed suite. The second attempt runs in retained exec session34529;
the parent result must be collected before reporting the full suite as passed.

Sources checked: repository artifact-wiring and checkpoint implementations,
the pinned actions/checkout README's `path` contract, the preserved SEC1 point
recovery contract and native tests, current AGENTS.md and the P1 requirement.
No scientific checkpoint retry or experiment was attempted: no approved access
change was observed. The frozen protocol and spec remain unchanged.
