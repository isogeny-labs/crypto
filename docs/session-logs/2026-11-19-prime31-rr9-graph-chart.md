# 2026-11-19 Prime31 RR9 Grassmann chart session

## 2026-11-19T03:28:00Z — preregistration and prior evidence

### What was done (commands, files, features)
- Read root AGENTS.md, docs/ACTIVE.md and the PR #4776 2026-11-19 session log; reviewed prior PR #4466, #4692, #4728, #4772, #4773, #4774, #4776 and concurrent #4779.
- Checked the previous native exterior-4 Rust code and source-specific workflow. Inspected published prime-field index-calculus prior art PKM 2016, APS 2018 and Kudo et al. hybrid elimination summary. Bounded GitHub searches for Grassmann chart and Schur-complement RR9 found no exact prior implementation.
- Created task branch and draft PR #4780, recorded task in docs/ACTIVE.md and preregistered the prospective experiment before code/measurement.

### Decisions made (and why)
- Test a fixed four-plane graph-chart Schur complement: normalized 4x4 blocks need 16 coordinates per half, and six 2x2-minor Laplace terms need 30 field multiplications per pair, versus 70-term exterior-4 contractions.
- Keep exact all-pairs exhaustive B4 negative fixture and identify this as a relation-predicate component, not a nonenumerative unknown-index solver.
- Avoid duplicating the separate #4779 fixed-linear Plucker span audit and #4692 unknown-index extractor.

### Open threads (unfinished, pick up next time)
- Implement native Rust chart computation, synthetic and real elliptic-curve identities, B4 positive/negative extraction, independent Python BigInt verification and source-pinned GitHub Actions.
- Inspect CI; record actual operation counts, elapsed and failures, then update PR and research protocol outcome.
- Signed independent PCC remains pending.

### Errors (what went wrong, how it was handled)
- Shared research-KB connector/CLI not present, so used bounded repo search and primary literature.
- Main did not have docs/session-logs/2026-11-19.md; recorded this task in a dedicated dated session log to avoid overwriting other branches.
