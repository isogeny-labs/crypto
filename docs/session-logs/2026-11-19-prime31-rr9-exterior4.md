# 2026-11-19 Prime31 public RR9 exterior-4 session

## 2026-11-19T02:50:00Z — registration, code and evidence pipeline

### What was done (commands, files, features)
- Read AGENTS.md, docs/ACTIVE.md, recent Prime31 work and original pinned Prime31 Rust Arithmetic source.
- Reviewed prior #4466, #4692, #4728, #4770, #4772, #4773, #4774 and relevant published prime-field IC sources (PKM 2016 and APS 2018), the negative autoresearcher RR-norm / determinant lanes. Bounded GitHub code search for exterior4 and related terms returned no exact matches. Shared research-KB MCP/CLI is unavailable in this runtime.
- Opened draft PR #4776 with docs/ACTIVE.md initial registration before research changes. Preregistered RR9_EXTERIOR4_PUBLIC_PROTOCOL_20261119.md before code.
- Implemented prime31_rr9_exterior4.rs using original Rust EC arithmetic for public supports and replay. Candidate extraction uses only finite-field RR9 rows, 4x4 minors and exterior 70-component contraction, with no generic group sum table.
- Added separately authored Python BigInt replay and a pinned-original-source GitHub Actions workflow. Committed mathematical report with exact B4/B8 arithmetic work and BSGS comparison.
- Submitted candidate code and workflow to GitHub. Exact-source passing CI, standalone run and signed PCC have not been observed in this entry.

### Decisions made (and why)
- Treat the exterior-4 identity as classical (generalized Laplace), not an original theorem. Treat full B4 scan as a falsifier, not a qualified complete ECDLP solver.
- Do not execute a naive full B8 scan: 4,294,967,296 pair contractions * 70 Fp coordinate products = 300,647,710,720 products per unsuccessful public residual.
- Only advance the 70-dimensional orthogonal-vector representation if subquadratic pairing or a genuine nonenumerative F4/SAT extractor is demonstrated on natural B8 positives and negatives.

### Open threads (unfinished, pick up next time)
- Inspect source-bound CI for Rust build, unit/generalized Laplace identity, first planted positive and complete unplanted B4 run; repair any failure, update report with raw measurements.
- If the B4 source-bound result passes, request independent signed replay and mark draft PR ready; separately notify #4692 that naive B8 exhaustive pairing is an untenable path.
- Integrate only a successful public-only nonenumerative B8 extractor into #4728 rank31 and 1,024 target system; compare on same host against shared signed BSGS and optimized rho.

### Errors (what went wrong, how it was handled)
- Repository research KB server/wrapper not accessible from this connector-only runtime; bounded GitHub research and primary literature substituted, no novelty priority claim.
- Nested mutable borrow in initial Rust canonical_base expression caught by static inspection and corrected before CI.
- An earlier report-writing tool call had a JavaScript string delimiter parsing error; it wrote no file. Retried with properly formed source, committed report.

## 2026-11-19T02:42:00Z — source-specific B4 Rust CI passed and raw artifact recorded

### What was done (commands, files, features)
- Fetched source-specific GitHub Actions run #44445676750 on candidate source ef037d6c641ff69351f2714fee8ad2cbfc54ec7d: status completed, **success**. All steps passed (original 40 Rust regressions, exterior-field/Laplace unit tests, native B4 candidate execution, independent Python BigInt truth replay).
- Downloaded and inspected raw artifact #14493406334. Planted public target=(1991076510,718644930) returned correct all-zero word after 1 algebraic pair and 175.177977 ms including minor construction. Natural public R=G produced no witness after all 16,777,216 pairs, 1,174,405,120 coordinate products and 1,295.599945 ms including minor construction. Independent Python EC grader confirmed both truth labels.
- Frozen exact source-specific summary JSON and updated scientific result, linking raw artifact and pending signed PCC status.

### Decisions made (and why)
- Narrow B4 correctness/cost experiment succeeded. Stop exhaustive B8 architecture: 300,647,710,720 coordinate products per all-negative public residual is an uncompetitive direct-search budget; not a lower bound on subquadratic orthogonal matching or F4.
- Do not interpret the planted first-pair result as a natural positive-yield or full ECDLP benchmark. Keep 31-rank/1024 non-generic replacement work with #4692 owner.

### Open threads (unfinished, pick up next time)
- Develop public-only subquadratic orthogonal-vector match or genuine F4/SAT root extraction and validate on frozen B8 46 positive/18 negative natural corpus before any full-solver comparison.
- Obtain independently signed PCC verification, and execute matched complete native index-calculus/BSGS/rho costs if a nonenumerative extractor succeeds.

### Errors (what went wrong, how it was handled)
- None in final candidate CI. Other unrelated repository workflows may have separate failures.
