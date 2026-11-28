# CT74 scoped research receipt — 2026-11-13T15:46Z

## What was done (commands, files, features)
- Inspected current #3877 / #4015 / parent #4012 metadata, exact CT73 session and reports, AGENTS.md for crypto and cryptanalysis, concurrent cryptanalysis #1523, and successful CT73 CI. Reviewed Snowden's good-reduction torsion specialization and the elliptic-sieve formal-filtration account. KB MCP and checked-out wrapper are absent; no Conductor executable is present in this connector runtime.
- Established the finite local ring source-section formula [p^(N-1)u]R for u inverse of p^(N-1) modulo ell, using the formal reduction kernel. Implemented the 11a1 F29 ell=3 instance by independent Python division-polynomial Hensel and JS per-digit group-law search.
- Ran Python normal/-O and JS normal/--no-opt in local sandbox; all six JSONL records agree exactly, SHA256 b459b8be46bf3b7e9cc8af7ecc85d732e4e5fc5d51dfcb27773fae0a22d799e4. Negative control rejects the curve-only naive lift at N>=2. [pcc:unverified]

## Decisions made (and why)
- This solves only source-local point lifting; do not conflate it with a global 29-to-2 point transfer. Keep code distinct from concurrent native Jacobian and cryptanalysis work. Add bounded CI, no EKS spend. No general [m] finite-ring evaluator is claimed yet.

## Open threads (unfinished, pick up next time)
- Check exact-head CT74 CI/PCC, parent review and signing. Implement general finite-ring [m] group arithmetic (including formal infinity chart), and investigate algorithmic global point-field recognition and binary-place coherent orientation; then measure cold complete solving costs.

## Errors (what went wrong, how it was handled)
- The public GitHub clone URL did not resolve in the local container; used authenticated connector for the repository. The shared KB MCP, its checkout fallback and Conductor are unavailable. No arithmetic mismatch or optimization-mode skipped assertions; no signed proof was produced. [pcc:unverified]
