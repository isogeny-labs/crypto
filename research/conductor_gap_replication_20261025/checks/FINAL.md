# Final verification

The final library run exercised all 4,613 registered tests: 4,485 passed,
119 retained their existing ignored status, and nine local-socket tests
failed with sandbox `PermissionDenied`. The exact compiled test executable
was rerun with networking enabled for `cryptanalysis::pollard_collab`:
44/44 passed, including all nine initially denied tests. Thus every one
of the 4,494 active library tests passed across the full run and that
focused environment-corrected rerun. Both original output and rerun are
preserved; the initial run is not relabelled a single-command pass.

Other completed checks:

- Five touched preparer/registry/volcano tests pass; touched native examples
  and volcano binary pass release clippy with warnings denied.
- The lossless registry check preserves 15,791 prior raw objects and all
  2,976 experiment models. Alias content matches the maintained canonical map.
- All 43,520 main and 128 excluded calibration attempts replay exactly after
  the registry repair; complete fresh receipts are in the separate validation archive.
- Current native summaries and CSV exports reproduce the recorded outputs
  byte-for-byte. The separate C++ implementation agrees on 114 headline values,
  validates all main scalars, rejects all scalar+1 mutations, and verifies the
  full exported geometry. These local checks use the original physical host.
- Native cover checks pass for all 18,767 registered models; the generated
  cover graph, Jacobian transfers and browser are refreshed. Jacobian and
  traits freshness computations pass. The traits format change preserves every
  parsed value, is idempotent, and retains all 18,943 records and factor budget.
  Native formatter build and clippy pass; the whitespace-independent CI
  mutation still changes the intended conductor value.
- Site suite: 49/49 pass. Validation-gate suite: 49/49 pass. The gate check
  against the parent passes; no row or pre-gate freeze is promoted or expanded.
- Curve identity self-test and global name check pass. Dashboard panel index
  is current. All 70 PDFs (76 pages) were rendered and visually reviewed;
  final changed main-report pages were rechecked and all hashes match.
- The added replay workflow passes the routine PR-budget check. The global
  check reports three budget violations in the unchanged parent
  `weak-curve-lean.yml`; that file is byte-identical to the parent revision.

Initial validation-gate failures came from off-disk sparse-checkout evidence;
materializing the existing pinned inputs resolved them. The earlier cover
failure exposed the legacy-integer bug documented in REGISTRY_CORRECTION.md.
Neither failed check nor the failed v1 fixture preflight was discarded.

The traits catalogue was previously a 2,748-record snapshot; regeneration
covers the complete current 18,943-record input. Native compact encoding
reduces its pretty output from 156,045,766 to 87,793,168 bytes without dropping
values or lowering a budget. The old native freshness comparison passed
before this lossless format change; CI repeats it using the compact writer.

Per-command logs are under final/. Raw measured sessions remain immutable.
Second-host CI, exact-head review and the parent PR's merge gates remain pending.

After compact encoding, the test executable was rebuilt with the final
traits fixture: all 44 collaboration tests pass again with networking
and all three traits-query tests pass. Authored source whitespace checks
are clean. Generated Matplotlib SVG paths and verbatim test logs retain
their original whitespace; those artifact-only warnings are not repaired
by editing recorded output.
