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

## 2026-11-19T16:56:00Z — exterior-four span rank gate run locally

### What was done (commands, files, features)
- Stacked draft PR #4779 on #4776 and froze `RR9_PLUCKER_SPAN_PROTOCOL_20261119.md` before implementation. The producer computes exact streaming spans and restricted complementary-minor pairing rank; a separate Rust checker uses permutation-expanded minors and u128 modular arithmetic.
- The S3 resume checkpoint gate failed because this runtime has no upload token or AWS credentials. The owner authorized one local run with Git-resident artifacts; the deviation and each retry were added to the PCC bypass ledger before execution.
- Attempt 01 exited 128 during temporary checkout because the first local runner placed the 4 GiB virtual-memory limit around Git. Attempt 02 passed formatting, original library tests, focused producer/checker tests and release compilation, then exited 127 before producer execution because `/usr/bin/time` was absent. Both failures are retained byte-for-byte. `[pcc:unverified]`
- Attempt 03 used locally built GNU Time 1.9, retained the 30-minute outer cap, and applied 4 GiB limits to producer/checker processes as in the frozen workflow. Two producer outputs and two checker outputs were byte-identical; all recorded exit codes are zero and six mutations were rejected. `[pcc:unverified]`
- Exact local result: N4, P4, N8 and P8 each report left/right/pairing rank 70/70/70 with nonzero determinant certificates. Under the frozen rule the verdict is `KILL_FIXED_LINEAR_COMPRESSION`. Claim record `PCC-20261119-50ffe2` is unverified. `[pcc:unverified]`
- Raw attempts, `SUMMARY.json`, `RESULT.md`, `rank_gate.svg`, PNG and inspected three-page `REPORT.pdf` are under `research/prime31_ic_20261115/rr9/plucker_span_local_20261119/`. `[pcc:unverified]`
- Merged the companion exterior-multilinearity proof and ran its corrected Rust one-slot certificate locally. It strengthens the four-case observation to every finite target except at most 64 points for this fixed support witness; claim `PCC-20261119-a9e2e9` is unverified. `[pcc:unverified]`

### Decisions made (and why)
- Close only fixed linear compression of the registered 70-coordinate exterior-four message. Full span and restricted-pairing rank supply no smaller lossless fixed linear message on the frozen B8 fixtures. `[pcc:unverified]`
- Do not generalize this result to nonlinear charting, Boolean/tensor SAT, F4, XL, orthogonal-vector indexing, relation yield, scalar recovery or end-to-end ECDLP cost.
- Use a canonical Boolean/tensor encoding-growth gate as the next discriminator for #4692 before paying for a SAT solve.

### Open threads (unfinished, pick up next time)
- Preregister a SAT encoding experiment that emits one typed predicate IR, independently checks tiny B2/B3 truth tables, compares bit-blasted and finite-field encodings, and measures held-out clause/monomial growth through B4/B6/B8.
- Advance SAT only if extraction and verification included, held-out scaling improves over exhaustive `16^8` pairwork; otherwise preserve the negative and move to F4/XL degree-of-regularity or orthogonal-vector indexing.
- A source-bound GitHub rerun remains useful for execution attestation if checkpoint access is restored; it is not required to retain this owner-waived local result.

### Errors (what went wrong, how it was handled)
- Attempts 01 and 02 are preserved rather than overwritten; amendments A1 and A2 change only local cap placement and GNU Time path, respectively.
- Headless Chromium lingered without creating a PDF. It was stopped, and the report was rendered with Pandoc/pdfTeX after converting the SVG figure to PNG; the first page and extracted text were inspected.
