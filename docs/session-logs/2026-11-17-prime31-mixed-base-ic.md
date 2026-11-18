# Prime31 mixed-base complete IC attempt — 2026-11-17 UTC

## 2026-11-17 ~04:30Z — literature and prior lead audit; freeze before new code [pcc:unverified]

### What was done (commands, files, features)
- Read repository `AGENTS.md` and `docs/ACTIVE.md`, 2026-11-17 shared session-log entries, previous cross-base 4+4 multiplicity and orientation protocols, prior APS complete cold result, RR9 signed-norm protocol and previous non-generic lead triage.
- Read exact merged PR statuses: #4466 RR9 norm, #4468 WZ build unblocking, #4486 linear-chord null test, #4489 encoding; reviewed cross-repo pending leads #3301/#3305 and cryptanalysis #1536 from existing reports. Verified #4486 was merged, not an open PR needing editing.
- External literature search (2026-11-17 UTC): prior source citations APS 2018 DOI 10.1016/j.ffa.2018.01.009, PKM 2016 DOI 10.1007/978-3-662-49387-8_1 and Shanks BSGS (classical). This mixed 4+4 table is a generic BSGS-derived method; no novelty claim.
- Determined a **potentially material cross-base column alias**: first 31 rational abscissae and Dickson-depth-six base may share an actual affine point, so rank must be on the deduplicated point union plus Q, not 63 naive columns. The Rust run tests that overlap rather than presuming disjointness.
- Created branch `research/prime31-mixed-base-ic-20261117`, committed prospective `MIXED_4P4_IC_PROTOCOL_20261117.md` at 89a9dab before implementation. Implemented an index-calculus-derived original-curve mixed 4+4 half-sum collector and incremental modular RREF recovery in native Rust; committed at 26c515, early borrow-separation patch at f9fbfd, source-pinned Actions workflow at 684655.
- No access to local Cargo/Rust or EKS credentials in this connected-only runtime. Research-KB MCP/wrapper and Conductor are not exposed. Fallback: GitHub source & scientific literature. No paid compute requested.

### Decisions made (and why)
- Reuse `PRIME31`, `Arithmetic`, and `Base::build` from existing native Rust instead of introducing a new curve, programming language or DLP engine. Freeze exactly 2 caller-known Q and 160 relation attempts/Q, with all failed probes and full cold costs recorded.
- Deduplicate base points before assigning columns; all relations are from public R=aG+bQ with original-curve replay, matrix rank and recovered individual base logs independently checked. No scalar leakage into `solve_public_q`.
- Method is **generic MITM-equivalent index calculus**, intended as a rigorous complete-cost control for cross-base S9; it is not the pending genuinely nonenumerative target-conditioned RR9 extractor. No speedup, Rust test success, or PCC signature is claimed before CI evidence.

### Open threads (unfinished, pick up next time)
- Inspect source-specific Rust CI for compiler, unit, full unknown-target log or timeout errors. Preserve *all* failures, fix only actual defects without rerolling the seed/gate.
- If complete, record raw two-target full-cold cost and compare only as historical context with #4486 APS, not as a same-host rho victory. A real advantage requires fresh ≥96 same-Q isolated matched tuned WZ rho and external IC plus signed independent evidence.
- Still priority: original-ideal-correct target-conditioned RR9/S9 `U+yV` extraction from public residual without half-sum support enumeration, B4/B8 exact fibre controls, then B31 full IC.

### Errors (what went wrong, how it was handled)
- Rust nested mutable `Arithmetic` borrows in initial code were separated before CI submission, with no benchmark observations.
- Local toolchain and KB/conductor unavailable; report limited execution scope, do not invent performance claims.
