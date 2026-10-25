# Setup observations and retained authoring failures

These setup observations are separate from the frozen research execution. They are not
research results, reference timings, or a successful security analysis.

- Managed Linux x86-64 workspace only, Rust1.98.0; no paid host launched.
  Existing runtime network policy was observed enforced/unrestricted before
  network commands; inherited proxy/CA settings were retained. No credential
  files, environment dump, chat credentials or S3 operation.
- Actual Conductor invocations returned127 (`command not found`), including
  before editing and before expanding scope to CI/report integration.
  Pinned `.conductor/WORKFLOW.md` fetch returned404. Neither is a pass.
- Lean and Lake version probes returned127. No formal proof was run or
  claimed. Finite arithmetic checks retain their narrower scope.
- External solver availability was checked by executable lookup on PATH.
  Magma, msolve and Singular were not found there. Protocol language about
  their absence refers to this availability check, not a filesystem-wide
  proof that no external solver file exists. External comparison is unrun.
- The first CLI compilation failed with eight authoring errors: Option
  conversion, CurveId slug field access and inferred map types. Original
  `unit-initial.log` is retained. Corrected sources were tested and frozen
  before the research command; no failed research result was retried.
- `cargo fmt --bin factor-base-sweep` was refused because this Cargo fmt
  interface does not accept `--bin`. Formatting used direct Rustfmt instead.
  The tool-observed usage error was not separately saved as a full original
  stderr file; this note preserves the exact failed invocation and reason.
- Initial staging checks found one trailing blank line in the NEW
  `.gitignore`; it was fixed before the source lock. No executed historical
  protocol or text-whitespace rule was changed.
- A report-renderer import probe found CairoSVG unavailable and Matplotlib's
  default home cache unwritable. The original tool output was observed but
  not separately redirected as a full original log. Rendering uses a writable
  workspace Matplotlib cache and the existing Typst/PDF renderer instead;
  no research implementation or result is changed by that choice.
- The initial Typst route render rejected `px` as a page-size unit. Its
  failed log and the dependent missing-PDF conversion log are retained.
  The successful retry uses point units; no experiment input changed.
- The mechanical executor refused the initial dispatch because it named
  only "repository root" without the absolute path. The parent supplied
  `/workspace/crypto-pr2134-head819-review`; no native attempt had started.
  The same card, prompt, arguments, budgets and no-retry rule were retained.

The native experiment then ran once. Resource-limited calls are scientific
results in the sealed records and were not retried or given larger limits.

Director assessment was interrupted and completed read-only from saved
records on October25. Canonical `check-relay` exited1: the runner prints
output dictionaries in card order, while serialized receipts sort those
keys and the checker expects the reloaded order. All values and stream
digests agree; reconstructed runner-order output is byte-exact to the
executor relay. The canonical failure and its `executor_deviated` API
classification remain retained; the separate narrower verification is
not called a passing canonical check. No tool/card/receipt edit or research
rerun was used. See cards/assessment.md and its verification helper.

At the October25 publication resume, GitHub connector reads of both PRs
and current AGENTS returned404; one native `git ls-remote` returned403
with `Write access to repository not granted`. Remote heads, current CI
and policy therefore could not be verified. Local completion and commits
do not establish publication or merge. The last successfully read AGENTS
blob is retained and applied locally; it is not asserted to be today's
remote version. No old chat credential was retrieved or tried.

The first unpublished raw-archive preservation metadata used nonexistent
`research/backlog/` prefixes for the three cards, instead of `backlog/`.
This was caught before acceptance. The corrected check requires every
protected path to have tracked entries and confirms an empty diff. Initial
JSON is retained in the accepted raw tree and the first archive is retained
in `checks.tar.xz`. Scientific records and sealed experiment bytes did not
change. Final archive member checks and deterministic repacking pass.

Final staging found Matplotlib-generated trailing spaces in the new SVG
paths. The failed full whitespace-check log is retained. The renderer now
normalizes only its newly generated SVG lines and pins SVG identifiers;
no historical file or whitespace rule changes. Rebuilt PDF page renders
are checked against the inspected pages to verify unchanged visual content.

All successful and failed authoring logs are retained alongside exact
commands/exit records in the final raw validation bundle. Existing studies,
their failed logs and all previously published receipts remain unchanged.
Source-lock commit and execution receipts identify the actual boundaries.
