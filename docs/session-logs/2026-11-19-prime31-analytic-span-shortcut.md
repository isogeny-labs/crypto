# Prime31 analytic exterior-span shortcut — 2026-11-19 UTC

## 2026-11-19T06:25Z — Full rank by four signed pairs per slot

### What was done (commands, files, features)

- Read `AGENTS.md`, the active queue, #4692 and #4779 lane state, original RR9 exterior4 source and protocol, previous #4728 rank/1024 report, #4776 exhaustive-pair negative, and #4780/#4783 preprocessing improvements.
- Research-KB MCP/CLI unavailable through this connector-only runtime; used the bounded GitHub PR/file audit and prime-field index-calculus references (Petit–Kosters–Messeng 2016; Amadori–Pintore–Sala 2018; Kudo et al. 2018), without claiming literature novelty.
- Derived the exact 8x8 determinant of four positive/negative signed pairs in each RR9 slot:
  `D=16 product_i[(x_i-r)y_i] product_{i<j}(x_j-x_i)^2`.
  Target ordinate cancels.
- Proved that when every slot's eight signed support rows span Fp^8, all four-fold exterior half-spans equal Λ⁴(Fp⁸) and the restricted complementary-minor pairing has rank 70. The proof is independent of the number of right/left half-table words and avoids 65,536-vector rank construction.
- Independently coded Python EC addition/scalar multiplication, canonical base generation and both modular Gaussian 8x8 determinant and closed formula. On pinned Prime31 curve, all 32 N4/P4/N8/P8 slot values match exactly and are nonzero. Base x=0,1,2,3; target residuals G and the planted 8-weight sum; eight frozen residues for N and P recorded in report. All 32 weighted base abscissae are distinct. Therefore all but at most 64 group targets meet this simple full-rank certificate, for any B>=4 containing these four base points.
- Committed mathematical report `research/prime31_ic_20261115/rr9/RR9_SPAN_FULLRANK_SHORTCUT_20261119.md` and an independent *unexecuted* Rust regression example `research/prime31_ic_20261115/native/examples/prime31_rr9_analytic_span.rs` reusing the original `Arithmetic` crate on existing PR #4779 branch. Source tip at this handoff `5a1f80afa2a7864759b89dee21088908cef22c7a`.

### Decisions made (and why)

- Contribute to existing span-rank PR #4779 instead of opening a duplicate PR. Keep the original pre-registration and its independent checker and mutation gates intact.
- Mathematical full rank is established from exact 8x8 sufficient certificates and multilinearity, but the registered **source-specific Rust producer/checker protocol is not complete**. Do not change #4779's disabled automatic workflow trigger or circumvent its checkpoint restriction. Do not label this signed PCC validated.
- Fixed linear Pluecker-message compression is excluded for nearly all original Prime31 target points with these supports. This says nothing about nonlinear indexing, SAT/F4/XL and does not establish a complete non-generic ECDLP speedup.

### Open threads (unfinished, pick up next time)

- Once the #4779 owner resolves its registered checkpoint blocker, run the new Rust example (from pinned original crate) with `cargo +1.100.0 test --locked --release --example prime31_rr9_analytic_span` and `cargo +1.100.0 run --locked --release --example prime31_rr9_analytic_span`; verify exact source, 32 residues, full protocol checker and mutation controls. Do not extrapolate from Python alone.
- Move primary attention to #4692 public-only RR9/F4/remainder-tower unknown-index extractor on frozen natural B8 46+/18- corpus. Only then reuse #4728 rank31/1024 Rust solver, pricing setup, failed queries, LA, and recovery against same-runner #4774 signed shared BSGS, strongest IC, and optimized rho.
- Further math gate if needed: handle the at most 64 exceptional target points by searching alternative four-point support bases; do not assume full rank there without a certificate.

### Errors (what went wrong, how it was handled)

- This runtime has no checked-out Rust workspace, Rust toolchain, authenticated GitHub CLI, or reachable GitHub host from the container (DNS failure). GitHub connector was used for checked-in files; Rust compilation and source-specific CI were **not executed**. This limitation is explicit in the report and PR handoff.
- Shared research-KB tools not loaded; source audit bounded rather than exhaustive. Existing #4779 workflow remains intentionally disarmed due to prior checkpoint restriction; it was not changed.
