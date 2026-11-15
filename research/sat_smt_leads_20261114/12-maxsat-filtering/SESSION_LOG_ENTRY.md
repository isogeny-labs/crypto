# Session log entry (kept here because this pilot may only write inside its own directory; merge into docs/session-logs/2026-11-14.md)

## 2026-11-14T10:15Z — Idea 12 pilot: MaxSAT relation filtering

### What was done (commands, files, features)
- Wrote PROTOCOL.md first, then filt.py / analyze.py in research/sat_smt_leads_20261114/12-maxsat-filtering/.
- `python3 filt.py validate validation.json`: 20/20 brute-force agreements, planted control RC2 = ANY = 7 vs G2 = 31 (verified by run).
- `python3 filt.py run results_raw.json`: 72 instances, N = 32..4096, excess 5/20/100% (verified by run; numbers [pcc:unverified]).
- Sources checked: KB unavailable: research-kb not installed in this container. docs/session-logs/ holds only README/TEMPLATE.

### Decisions made (and why)
- Random uniform weight-4 model instead of real decompositions (budget); stated in protocol.
- Deviation D1: SGE density stop (avg row weight <= 16) because the unbounded min of N*W is degenerate; recorded before the measured run.

### Open threads (unfinished, pick up next time)
- Real toy relation matrices; merges inside the model; longer budgets. See README "Not covered".

### Errors (what went wrong, how it was handled)
- pysat EncType.cardnetwork is spelled cardnetwrk; fixed before measurement.
- RC2 timed out on every measured instance (5 s), including N = 32; reported as is.
