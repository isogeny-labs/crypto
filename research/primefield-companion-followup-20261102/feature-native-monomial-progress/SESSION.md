## 2026-11-07 — Exact progress and failure flow (T-1752/T-1753)

### What was done (commands, files, features)

Added deterministic stderr progress markers outside counter scopes to the matched producer. Native selected correctness test passes;414 inherited tests filtered. Actual development trace parses18 markers and six started/verified calls. Four synthetic methods pass, covering all128 prefixes of127 planned markers, partial tails, bad sequences and receipt mismatches. Release binary compiles and describe executes no kernel. [pcc:unverified]

### Decisions made (and why)

Failure reporting must identify actual calls rather than invent60 attempts after a process failure. The separate monomial.failure-flow/v1 record distinguishes observed starts, verified returns, incomplete calls and not-started targets. CLI pins the progress-enabled binary. Claim PCC-20261107-b96ce1 remains unsigned. Sources checked: matched producer, capture wrapper and preserved export schema/linter; no new mechanism or novelty claim.

### Open threads (unfinished, pick up next time)

Amend/bind the new binary in the still-unfrozen protocol. Resolve canonical export support for unstarted cells without fabricating attempts. Preflight, exact launch publication and authoritative budget/capacity admission remain. No full20-target comparison; original4563-case cohort unchanged. PR3599 remains open at inspection.

### Errors (what went wrong, how it was handled)

No native failure. The preserved canonical schema requires attempts>=1 and linter demands all planned cells, exposing an honest unstarted-cell representation gap. Retain custom failure custody rather than misstate execution. taskq cost-control paths are sparse/absent in this checkout; no cloud mutation attempted.
