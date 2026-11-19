# 2026-11-19 Prime31 W8 signed-weight collision audit (PR #4805)

## 2026-11-19 11:20 UTC — what was done (commands, files, features)

- Read repository `AGENTS.md`, `docs/ACTIVE.md` and recent Prime31 PRs [#4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4728](https://github.com/isogeny-labs/crypto/pull/4728), [#4803](https://github.com/isogeny-labs/crypto/pull/4803), [#4793](https://github.com/isogeny-labs/crypto/pull/4793). Shared research KB MCP/wrapper not available through this connector runtime, so review is bounded by these primary GitHub records, autoresearcher search, and PKM 2016 / APS 2018 published summaries.
- Created branch `automation/prime31-w8-weight-signature-audit-20261119`, recorded task in `docs/ACTIVE.md` before research write, created draft [#4805](https://github.com/isogeny-labs/crypto/pull/4805).
- Preregistered `research/prime31_ic_20261115/rr9/W8_WEIGHT_SIGNATURE_AUDIT_PROTOCOL_20261119.md`.
- Independently enumerated the four-slot signed **integer** coefficient vectors in Python stdlib (not an ECDLP solver): original [1,3,5,7] and [11,13,17,19] each had 64,737 distinct vectors, 904 unordered word-collision pairs, max fiber 16; comparison [1,2,4,8] and [16,32,64,128] each had 65,536 distinct vectors, no pairs, max fiber 1. The historical 64,737 actual half-sum support in #4728 exactly matches the formal count, suggesting a structural explanation for that source fixture.
- Implemented original-crate Rust example `prime31_w8_weight_signature_audit.rs` using `prime31_algebraic_ic::{check_curve,Arithmetic,Point,PRIME31}`. It performs exact formal-vector census, actual EC half-support measurements and a matched panel of four windows × 64 natural public targets per arm; the generic MITM is *only* an out-of-band grader. No natural target scalars or witness words are passed to the membership test.
- Added the pinned original-source workflow `prime31-w8-weight-signatures.yml` (Rust 1.100.0). Workflow-run completion and measured data must be checked before any empirical claim.

## Decisions made (and why)

- This is a distinct weight-geometry and yield-control experiment, **not** a duplicate of the genuine unknown-index F4/remainder-tower extractor owned by #4692.
- No speculative whole-solver or non-generic performance claims. Deliberately compare identical public residual points to isolate weight choice. Report accidental EC collisions rather than assuming their absence, because the support upper bound is mathematical, not guaranteed equality.
- Preserve original curve, 31 base points and four windows to avoid changing curve security or rank requirements. Alternative powers-of-two are a *candidate control*, not assumed better.
- Literature receipt: Petit–Kosters–Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009). Both give prior index-calculus context, not a novelty claim.

## Open threads (unfinished, pick up next time)

- Inspect GitHub Actions [candidate source workflow](https://github.com/isogeny-labs/crypto/actions/runs/44515210069) to confirm compilation, formal counts, actual EC half-support and 256-target paired hits.
- If Rust/CI fails, correct original-crate E0499/type mistakes and rerun. Report actual paired old/new success and memory/operations; do not conflate group-sum collision removal with algorithmic extraction or whole-solver improvement.
- Publish source-linked frozen JSON summary and mathematical result, mark task completed only after passing source-specific execution. Coordinate with owner #4692 if meaningful.
- New mechanism remains efficient public-only B8 weighted RR9 unknown-index extraction, then rank31/1024 complete solver and same-runner strongest-BSGS/IC/rho evaluation.

## Errors (what went wrong, how it was handled)

- Container has no GitHub DNS or Rust toolchain; used GitHub connector for source reads/writes and source-specific GitHub Actions, with pure-integer local independent precomputation only.
- Avoided nested mutable borrow in first-x lift; loosened brittle assertions equating formal and actual EC cardinalities so unforeseen group collisions appear as measurements rather than a false test failure.
- Shared research-KB tooling unavailable in this runtime; bounded fallback explicitly documented.
