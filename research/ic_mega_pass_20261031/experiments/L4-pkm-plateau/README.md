# L4-pkm-plateau: interim (chains still running under nohup at write time)
See PROTOCOL.md (written before any row). Raw: raw/*.jsonl, raw/msolve_logs/*.gz.
Interim, Kummer m=2, P1=2013265921, 4 targets/t, no decomposable draws so far:
- own closure d_F: t=1..6 -> 3,4,4,4,4,5 (all draws agree) = registered medians (reproduction, not a C-step test). ffd=4 at every t>=2.
- msolve max step degree D: t=1..6 -> 3,4,4,4,4,5; t=7 seed1 D=5. So far D = d_F (gate G2 holds).
- Gates: summand d_F=2 (t=2..7), planted never reaches 1 (t=2..7, t=7 time-censored at c=6).
- Pending: closure t=7..9, null arm, P2=998244353 arm, msolve t=8..11. RESULTS.json not yet written.
C-step verdict: UNDECIDED, blocked. Its only blind cells (t=12..14) need 56-112 GiB and hours-days per system; this lane has 2 threads, ~2 CPU-h, ~5 GB free disk, low free RAM. The own closure is dense-ish sparse rows, feasible to t~9 (c=5); t=10 at c=6 needs ~60k columns (infeasible here).
Deviation: msolve run with -t 1 (not -t 2) so the two chains total 2 threads. Rust pilot not used (disk).
