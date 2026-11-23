# PR #2560 CI follow-up, 2026-10-25

The delivered head was `b9d5caad068edaf6262ace868f6dd857d406fb8c`.
All 25 pull-request workflow runs for that exact head were enumerated
through the paginated Actions API; the reported total and retrieved count
both equal 25. This is not a completeness inference from a connector's
first page. No reviews, comments or unresolved review threads were present
at the initial check. Parent #2430 remains open at `8e6eb294`.

## Completed replication verification

The [replication workflow](https://github.com/aburan28/crypto/actions/runs/40179357412)
passed on that head. Its complete 1,112,669-byte artifact is retained as
[the original ZIP](../validation/ci-20261025-b9d5caad.zip), with the GitHub
artifact digest checked against the downloaded bytes. The
[readable receipt](../validation/ci-20261025-b9d5caad.json) records 68 native
audits, 43,520 main and 128 excluded calibration records, all exact replays,
43,520 independently checked main scalars and scalar+1 rejections, the full
geometry checks, and 114 passing independent headline comparisons.
This is verification of recorded data, not a new scientific measurement.
The crater result, shared-index dependence, fixed-cohort limits and missing
CM setup/algebraic-oracle coverage remain unchanged.

## Changes caused by this PR

- The dashboard presentation check failed with `Dashboard is stale` in
  [job 121126869177](https://github.com/aburan28/crypto/actions/runs/40179357471/job/121126869177).
  The new replication panel was missing from the canonical renderer's
  retained-panel list. It is now registered there; both integration paths
  call that renderer. The generated page and source/panel hashes are refreshed.
- The [leaderboard job](https://github.com/aburan28/crypto/actions/runs/40179357436/job/121126868809)
  found three stale outputs. Their registry digest still named the exact
  parent registry (`595b7160…`), whereas this PR registers new models.
  The maintained builder refreshed the roster and source digest. All 28
  ranked rows are identical, including numeric tokens; no result or validation status
  is promoted. The dependent lab-browser index is regenerated afterward.
- The native Jacobian companion-table generator was also run. Its result is
  byte-identical to the existing Markdown table; no table change is needed.

## Local validation

Canonical dashboard and leaderboard checks pass. The complete regenerated
browser index and site are checked after all dependent outputs are refreshed.
The first site attempt started too early, encountered the old browser-index
source digest and spent time formatting the large assertion diff; that
attempt was interrupted and retained, rather than labelled a pass.

The local browser's managed policy blocks file URLs. Its native SUID sandbox
helper is also unavailable in this container. The same desktop, mobile,
dark-theme and JavaScript-disabled assertions were therefore run on the
unchanged page bytes through a loopback-only HTTP server and a temporary
Chrome launch wrapper. Only that document and the browser's automatic
favicon request are exempted from the HTTP transport check; every other
HTTP request remains forbidden. The replication fragment link was checked
explicitly. The transport adapter and receipt are retained with screenshots;
the new-panel, mobile and dark screenshots were inspected. The repository's
file-URL browser check and its assertions are unchanged for CI.

No Rust solver, measured fixture, raw session, preflight, report number or
frozen archive is edited by this repair. Prior native validation continues
to cover the unchanged native code. These repairs receive a new commit;
all applicable checks must be confirmed again on that head before merging.

## Unresolved gates on the delivered head

- [IC framework job 121126868766](https://github.com/aburan28/crypto/actions/runs/40179357316/job/121126868766)
  ran out of disk while building `libcrypto_lib` (`os error 28`), before its
  parameter/pipeline checks. The workflow is unchanged by this PR. The same
  workflow on the parent was cancelled, so it does not establish a passing
  baseline or prove attribution. No unrelated workflow alteration is made.
- [Jacobian certificate job 121126869339](https://github.com/aburan28/crypto/actions/runs/40179357503/job/121126869339)
  stopped before native validation; its first log retrieval returned
  `BlobNotFound`. Its cause is unestablished and its validation is not counted
  as passed. Retrieve the failure evidence at the next scheduled check.
- Other workflows were still queued or running. Neither a queued job nor a
  cancelled or failed run satisfies the merge gate. Exact-head CI and review
  remain required; the PR stays open.

The [CI capacity/log problem record](../../../problems/conductor-gap-ci-capacity-and-certificate-logs.md)
keeps the remaining gates and their checkable exit conditions explicit.

Final local results: all 49 site tests and all 49 validation-gate tests pass;
the validation gate, global curve-name check, canonical dashboard/leaderboard
checks and 306-panel index check pass. The original frozen archive hash
still matches. The browser-index change is its leaderboard source digest;
all 28 ranked rows, including their exact numeric tokens, remain identical.
