# Concrete F17 semantic/degree bridge

## Checked progress

### What was done
- Fresh-main worktree at 2741765c9, preserving the old shared worktree.
- The exact published generic group project rebuilt successfully; all three
  modules pass warnings-as-errors. The unchanged auditor checked 22
  declarations, zero failures, zero native trust.
- Original pre-outage group source is accessible again and its two SHA-256
  hashes match the published recovered source exactly. Prior sealed files
  remain unchanged.
- New FixtureGroupDegree.lean proves the concrete CompleteSplit polynomial
  identity, nonzero actual discriminant, evaluation bridge for every original
  generator, raw roots iff actual signed base relations, and one combined
  canonical-degree 5-to-3 theorem. Build attempt08 passed. All lifts and
  identity inverse pairs are included.

### Decisions made
- Require both prior Lake packages by repository-relative paths. The shared
  SecantIdentity source is identical in both. The dependency-layout build
  succeeded without source duplication or changes to either sealed package.
- Reuse curve icv1-fp5-tm1-a77cc6af and base FB1hed5b19904cc8 from the
  immutable fixture. Its existing registered=false status stays unchanged.
- The invariant is first vanishing homogeneous degree, not affine F4 degree.
  Dense comparator degree is inherited; no new dense-to-group equivalence
  theorem or solver benchmark is claimed in this continuation.

### Open threads
- Publish the focused regular PR, inspect current-head CI and follow the
  standing merge workflow. No proof or cryptographic run remains active.

### Errors
- Seven development builds failed before attempt08. They exposed finite-set
  numeral elaboration, unavailable lemma names, a reassociation mismatch and
  characteristic-reduction tactics not unfolding a field abbreviation.
  The final polynomial identity explicitly unfolds F before coefficient
  normalization. Failed source versions and full logs are retained.
- Conductor absent (127), documented fail-open. Fetch updated main but an
  unrelated uninitialized submodule returned exit1; the proof checkout and
  ancestor checks succeeded.

## Final verification, 2026-10-31

### What was done
- `env PATH=/workspace/prime-degree-lean/bin:$PATH LEAN_NUM_THREADS=4 bash
  verify.sh verification/final` completed with status `complete`, exit0.
- Thirty source pins passed. All nine Git dependency revisions match the
  lockfile. The new module passes warnings-as-errors, and the unchanged
  fourteen-module auditor reports 159 declarations, zero failures, zero
  native trust. The final source has no placeholders or custom axioms.
- Typst0.15.0 rendered a two-page A4 PDF. Both 100dpi page renders were opened
  and inspected: diagram labels, theorem names, source links and scope limits
  are legible, with no clipping. PDF inspection reports no suspects.
- Existing prior group SHA256SUMS still passes. Original group source hashes
  exactly match the published recovered version. Recovered raw prior logs
  and all seven failed new builds are retained in the development archive.

### Decisions made
- Publish only the new fixture bridge and parent README pointer. Existing
  sealed records, quantitative figures and performance dashboards are unchanged.
- No Mathlib cache-download pass, unavailable lean4checker result, new
  benchmark or independent dense-to-group theorem is asserted.

### Open threads
- PR publication, current-head required checks and merge follow-up.

### Errors
- Node child-process metadata capture returned EPERM despite captured git
  exit0; direct shell git reads succeeded and checked all pinned revisions.
- One receipt read used the generic-project cwd and missed the new receipt
  paths; reran in the correct project and confirmed the completed audit.
