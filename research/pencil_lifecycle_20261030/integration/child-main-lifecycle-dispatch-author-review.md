# Unapplied child_main lifecycle dispatch patch

Prepared 2026-10-30. This proposal changes private copies only. It was not applied to the working tree, built, tested, or executed. Rustfmt completed on the private copies. An initial patch-construction script stopped on an overbroad assertion because method_call_wall_ns also exists in another type; it wrote only a private before-copy. The corrected script anchors insertion within ChildOutput. No repository files were affected by either script.

Patch: `child-main-lifecycle-dispatch.patch`
SHA-256: `cd60428346749925830d0d6f53bb132b8de4f3633c7c810f398e8bec38e076e5`.

Exact bases:

- `src/cryptanalysis/ecbench/record.rs`: `44e10045bda3e5496aeaf8918275c6c1a73b8a36eea8b2c229f67948be882e32`
- `src/cryptanalysis/ecbench/runner.rs`: `73c20bae3415b0ca741ecd519552e0d29cbc2bea67efa1560256bed97849cd84`

Private before/after files and base manifest are under `child-dispatch-proposal/`. Root should compare these base hashes before applying or rebase the exact small change if its source has moved.

## What changes

The existing child solve/PMU/probe/complete block becomes one FnOnce callback. Without diagnostic context the same callback is called directly, retaining the original statement order, method clock and mathematical result processing. With diagnostic context the actual child_main source invokes `with_diagnostic_lifecycle`, which already installs common lifecycle ownership on every arm and the separate prefix sink only on pencil. This is actual call-path plumbing, rather than another helper with no caller.

ChildOutput gains two optional fields, omitted when absent:

- `diagnostic_lifecycle`: the separately serialized DiagnosticCallbackSummary as JSON Value. This carries diagnostic writer observations; it cannot supply SolveReport, scalar or algorithm cost.
- `diagnostic_call_wall_ns`: an enclosing child interval measured immediately before helper validation/scope installation and after explicit finish returns. It excludes child-output serialization and parent audit/publication; it is not total active-worker cost.

The two existing runner synthetic ChildOutput literals add None for compatibility. No other runner behavior changes.

Both the early v2 child rejection and the run_session admission gate remain unchanged. The new branch therefore stays unreachable through the current child entry point. This patch does not authorize or enable any experiment.

## Error and panic behavior

An ordinary solve/complete error is captured by the helper, which explicitly finishes prefix and lifecycle scopes, then returns the error with diagnostic summary. The child stores that summary before returning the callback error. Its final result match still leaves report absent on error.

An installation/finish error returns no accepted result. The enclosing child diagnostic interval is recorded before propagating a helper error; earlier partial lifecycle/prefix files remain separate custody evidence. The callback/helper does not synthesize a completed summary from missing IO.

Panic behavior is preserved: no new catch_unwind is inserted in the real algorithm path. Scoped guard Drop clears ownership without IO, so an unwind cannot manufacture a finished lifecycle file. Process abort/kill cannot run Drop; the parent’s existing reaped/timeout handling remains responsible for excluding late results. No changes to that acceptance policy are proposed.

## Clock boundary details

The existing inner method-call clock still starts immediately before solve_measured, includes nested instrumentation close and complete, and ends at the original point. The PMU/probe windows remain inside the callback, excluding common lifecycle installation/finish. Candidate prefix writes occurring inside the algorithm naturally occur inside its measured windows; no equal-overhead claim is made. The new enclosing diagnostic clock covers the helper and its installed lifecycle, but not workload rebuild, initial child metadata validation, output serialization or parent final publication. The already-specified outer runner resource interval must cover those separate boundaries; this proposal does not redefine old Timing fields.

## Validation still owned by root

No tests were run. Root can apply after current validation, compile the changed child and runner literals, then run the already-scoped software fixtures. The existing child rejection fixture should still reject before invalid workload construction. The existing fake callback fixtures cover the helper’s success/error/panic ownership. There is no new solver fixture, cohort generation, trial, protocol amendment, or resource admission in this patch.
