# Native validation diagnostic capture

The collected hosted run 42529489383 hit its declared 55-minute test-step limit
after an `ic_wide` failure, before libtest emitted its captured assertion
diagnostic or Cargo produced a parent exit receipt. Its old incomplete result
and all original receipts remain unchanged.

The existing optional full-library workflow now passes `-- --nocapture` to
libtest, so panic text is emitted while the parent is still running. Its
always-run retention step writes `completion.json` with the test step's
outcome, whether `parent-exit.json` exists, and the observed Cargo/tee exit
codes or null. A parent receipt from another source is rejected. This metadata
does not certify a parent summary; that summary still needs separate collection
and verification.

Ordinary jobs, triggers, concurrency and permissions are unchanged. The optional
job still has the same 90-minute deadline,5/25/55-minute step caps, pinned
Rust 1.99.1,2 build jobs/4 test threads/1 Rayon thread, no test filter, unchanged
ignored-test policy and zero automatic retries. No second hosted dispatch,
scientific preparation, index-calculus execution or local timing was performed.

## Verification

`verify.sh` extracts and executes the actual always-run shell body, rather
than a copy. Five cases cover missing parent status, real failure 101,
success 0, a skipped test step, and rejection of a different source. Missing
exit observations stay null, and both 0 and 101 are preserved.

The small native Rust `interrupted_harness.rs` fixture intentionally panics
in one test while another waits for external termination. With libtest's
`--nocapture`, the panic text is retained before the synthetic timeout ends
the fixture, without a parent summary. The timeout wrapper's 124 is a fixture
supervision status; it is not a Cargo exit code, library result or measurement.

To reproduce the helper checks, pass a new empty output directory:

```sh
research/koblitz_halftrace_20261030/development/diagnostic-capture-20261112/verify.sh \
  /tmp/native-validation-diagnostic-fixtures
```

Actionlint 1.7.12, Bash syntax, fixture formatting, all 4 existing workflow-parser
tests and native-source manifest verification pass. The linter's official
release archive digest was checked against the pinned repository installer and
upstream checksum. Parsed ordinary workflow configuration equals the parent
commit's configuration after excluding the opt-in full-library job.

Raw synthetic outputs, the exact extracted shell body and input hashes are in
the deterministic `runs.tar.xz`; extract with `tar -xJf runs.tar.xz`.
`FILES.sha256` verifies the extracted tree and `runs.tar.xz.sha256` verifies
the archive. Repack using AGENTS section 13a, with mtime 2026-11-12 00:00:00Z and xz -6 -T1.

These are administrative workflow checks using a native libtest fixture.
They do not establish current-source full-library success or address the
unresolved P1 saved native comparison. Checkpoint access remains blocked;
the scientific source 7c5866193, protocol/spec, old evidence and dated
November 6 PDF are unchanged. No speedup is claimed.

Sources reviewed: the existing manual full-library workflow; the original
hosted timeout/API/receipt snapshot in
`../hosted-validation-20261108/outcome-20261109T0027Z/`; repository
`.github/scripts/run-actionlint.sh`, `scripts/test_workflows_parse.py`,
AGENTS workflow/evidence rules and actual Rust 1.99.1 libtest behavior.
