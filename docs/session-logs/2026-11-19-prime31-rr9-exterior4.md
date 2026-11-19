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
