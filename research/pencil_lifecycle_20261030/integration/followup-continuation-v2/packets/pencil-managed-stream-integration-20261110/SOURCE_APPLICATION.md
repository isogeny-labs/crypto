# Exact source application

`original/` contains exact bytes from repository HEAD
`f3c1b8cc7663ca7d15374ae5e9a16c86839c9d18`. `overlay/` contains the proposed
source. `integration.patch` is their generated unified diff.

The source-only compatibility check was:

```text
git apply -p1 --check --directory=<exact-copy-of-original> integration.patch
apply_check_ok
```

No patch was applied to the canonical worktree. No compiler, test, formatter,
payload, or target was run. An eventual canonical application must first verify
that each original hash in `overlay-manifest.json` still matches, then use
`git apply -p1 integration.patch`. Drift is a stop, not a fuzzy-apply permission.

The managed run now has a concrete path:

```text
DiagnosticLaunch.source_receipt.source_revision
  -> record::with_diagnostic_lifecycle
  -> ScopedPencilProgress::install
  -> DurableWriter + relation_stream::Trace
  -> three_leads::solve
  -> progress_native::solve_one_with_progress_and_trace
  -> exact ordered JSONL + PencilEvidenceTotals
```

The postrun attempt exporter has a concrete path:

```text
existing retained native-session exporter and external custody checker
  -> ecbench.pencil-attempt-export-request/v1
  -> ecbench_retained_attempt_export
  -> retained_attempt_join::validate_full_cohort
  -> create-new ecbench.pencil-attempt-cohort/v1 + FilePin
```

True external input still required: the seven frozen launch documents must carry
the exact lowercase 40-hex Git revision in `source_receipt.source_revision`.
Existing launch documents carry only the source-manifest SHA-256, so the revision
cannot be derived honestly inside the child. This packet adds a backward-compatible
optional field globally and requires it only when installing managed pencil trace.
