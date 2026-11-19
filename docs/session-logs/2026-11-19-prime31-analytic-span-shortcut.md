# Prime31 analytic exterior-span shortcut — 2026-11-19 UTC

## 2026-11-19T06:25Z — Full rank by four signed pairs per slot

### What was done (commands, files, features)

- Read `AGENTS.md`, the active queue, #4692 and #4779 lane state, original RR9 exterior4 source and protocol, previous #4728 rank/1024 report, #4776 exhaustive-pair negative, and #4780/#4783 preprocessing improvements.
- Research-KB MCP/CLI unavailable through this connector-only runtime; used the bounded GitHub PR/file audit and prime-field index-calculus references (Petit–Kosters–Messeng 2016; Amadori–Pintore–Sala 2018; Kudo et al. 2018), without claiming literature novelty.
- Derived the exact 8x8 determinant of four positive/negative signed pairs in each RR9 slot:
  `D=16 product_i[(x_i-r)y_i] product_{i<j}(x_j-x_i)^2`.
  Target ordinate cancels.
- Proved that when every slot's eight signed support rows span Fp^8, all four-fold exterior half-spans equal Λ⁴(Fp⁸) and the restricted complementary-minor pairing has rank 70. The proof is independent of the number of right/left half-table words and avoids 65,536-vector rank construction. `[pcc:unverified]`
- Independently coded Python EC addition/scalar multiplication, canonical base generation and both modular Gaussian 8x8 determinant and closed formula. On pinned Prime31 curve, all 32 N4/P4/N8/P8 slot values match exactly and are nonzero. Base x=0,1,2,3; target residuals G and the planted 8-weight sum; eight frozen residues for N and P recorded in report. All 32 weighted base abscissae are distinct. Therefore all but at most 64 group targets meet this simple full-rank certificate, for any B>=4 containing these four base points. `[pcc:unverified]`
- Committed mathematical report `research/prime31_ic_20261115/rr9/RR9_SPAN_FULLRANK_SHORTCUT_20261119.md` and an independent *unexecuted* Rust regression example `research/prime31_ic_20261115/native/examples/prime31_rr9_analytic_span.rs` reusing the original `Arithmetic` crate on existing PR #4779 branch. Source tip at this handoff `5a1f80afa2a7864759b89dee21088908cef22c7a`.

### Decisions made (and why)

- Contribute to existing span-rank PR #4779 instead of opening a duplicate PR. Keep the original pre-registration and its independent checker and mutation gates intact.
- Mathematical full rank is established from exact 8x8 sufficient certificates and multilinearity, but the registered **source-specific Rust producer/checker protocol is not complete**. Do not change #4779's disabled automatic workflow trigger or circumvent its checkpoint restriction. Do not label this signed PCC validated.
- Fixed linear Pluecker-message compression is excluded for nearly all original Prime31 target points with these supports. This says nothing about nonlinear indexing, SAT/F4/XL and does not establish a complete non-generic ECDLP speedup. `[pcc:unverified]`

### Open threads (unfinished, pick up next time)

- Once the #4779 owner resolves its registered checkpoint blocker, run the new Rust example (from pinned original crate) with `cargo +1.100.0 test --locked --release --example prime31_rr9_analytic_span` and `cargo +1.100.0 run --locked --release --example prime31_rr9_analytic_span`; verify exact source, 32 residues, full protocol checker and mutation controls. Do not extrapolate from Python alone.
- Move primary attention to #4692 public-only RR9/F4/remainder-tower unknown-index extractor on frozen natural B8 46+/18- corpus. Only then reuse #4728 rank31/1024 Rust solver, pricing setup, failed queries, LA, and recovery against same-runner #4774 signed shared BSGS, strongest IC, and optimized rho.
- Further math gate if needed: handle the at most 64 exceptional target points by searching alternative four-point support bases; do not assume full rank there without a certificate.

### Errors (what went wrong, how it was handled)

- This runtime has no checked-out Rust workspace, Rust toolchain, authenticated GitHub CLI, or reachable GitHub host from the container (DNS failure). GitHub connector was used for checked-in files; Rust compilation and source-specific CI were **not executed**. This limitation is explicit in the report and PR handoff.
- Shared research-KB tools not loaded; source audit bounded rather than exhaustive. Existing #4779 workflow remains intentionally disarmed due to prior checkpoint restriction; it was not changed.

## 2026-11-19T17:10Z — local Rust certificate validation and repairs

### What was done (commands, files, features)

- Merged the analytic shortcut with the completed owner-waived span-rank evidence on #4779, leaving the hosted workflow manual-only.
- Ran the analytic example against original arithmetic commit `1df714645aa683cf975bc76fcacb9842cb4780fd` with Rust 1.100.0. Attempt 01 stopped at formatting; attempts 02 and 03 exposed two P-case frozen-residue transcription errors. Every failed attempt is retained in `rr9/analytic_span_local_20261119/`. `[pcc:unverified]`
- Corrected only the two constants to the values already present in the analytic report and independently recomputed by the determinant implementation: `1982101171 -> 1982109171` and `1921086591 -> 1921836591`.
- Attempt 04 passed rustfmt, the focused Rust test, release build, two 4-GiB-capped byte-identical executions and SHA-256 replay. It reports all 16 unique N/P slot determinants nonzero, 32 forbidden target abscissae and at most 64 group points unproved by this sufficient witness. Claim `PCC-20261119-a9e2e9` remains unverified. `[pcc:unverified]`

### Decisions made (and why)

- Retain both the direct full-span producer/checker and the shorter analytic certificate: the first checks the preregistered fixture protocol, while the second explains why full rank persists for all but an explicit tiny exceptional target set.
- Do not call the result a nonlinear-search lower bound or an end-to-end index-calculus result.

### Open threads (unfinished, pick up next time)

- Move to the separate public-only unknown-index Boolean/tensor, SAT or F4 discriminator under #4692.
- Obtain signed PCC/source-bound CI only if that administrative evidence is needed; the local artifacts remain explicitly owner-waived and unverified.

### Errors (what went wrong, how it was handled)

- The incoming Rust constants contained two digit transpositions. Independent recomputation made the tests fail closed; both failures and the corrected successful attempt are committed.
- The first uncommitted claim preflight (`PCC-20261119-b60d68`) was created before the theorem note received its required `[pcc:unverified]` tag, so its subject-hash verification failed. No published ledger was rewritten: the uncommitted tail entry was replaced before commit, and final claim `PCC-20261119-a9e2e9` binds the corrected prose bytes. `[pcc:unverified]`
- The focused test harness emitted one blank line at end of `attempt-04/tests.stdout`; its raw bytes are retained under an exact-path `-whitespace` attribute rather than normalized.
