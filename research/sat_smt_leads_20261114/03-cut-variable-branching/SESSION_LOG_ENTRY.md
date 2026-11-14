# Session log entry for docs/session-logs/2026-11-14.md (kept here: this pilot may write only inside its own directory)

## 2026-11-14 pilot 03 cut-variable branching
### What was done
Wrote PROTOCOL.md (+2 amendments), pdp.py, arms.py, validate.py, run.py, analyze.py; ran 92 rows
(n=11 l=1..3 five arms; n=13 l=2..4 arms D,C); results.json, README.md, raw_logs/. All numbers [pcc:unverified].
### Decisions made
n=19 dropped for n=11/13 after calibration (default arm did not finish); conflict cap replaces wall cap
(interrupt() ineffective for Cadical195); planted targets replaced by oracle-accepted relaxed-decomposable targets.
### Open threads
2l-bit cut ((s,p) form), hard learnt cap, Gauss-capable solver, l>=4 cells: not run.
### Errors
First validate.py run hung on n=19 and was killed; n=11 l=3 planted generator stalled and was killed (no rows lost);
n=11 l=2 decomposable generator found no target and was stopped. docs/session-logs/ not written (write restriction);
research-kb, conductor, ecdb unavailable in this container, so the knowledge-base lookup was not done.
