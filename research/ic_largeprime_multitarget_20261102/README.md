# Large primes, multi-target collection, byte-counter decision (exploratory)

Exploratory study, not preregistered; no leaderboard or `benchmarks/` row. Goal (single-target IC near rho parity) not met.

- Triple/quadruple large primes: a generic k-sum search, bounded by Shoup and already measured in-repo (k=4,5: exponents 0.565/0.547, S 11.56/8.51 at 40 bits, from the lead atlas); not built.
- Multi-target shared-table collection (`scripts/mt.py`, 20-bit curve, 12 trials/cell, all logs verified): the B=1 overhead is 1.37x at T=1 and about 1.0 for T >= 8; per-target cost at T=64 is 4.4x below T=1. This is multi-target rho reproduced, not IC beating rho.
- Coordinate-leakage battery (`scripts/leak.py`, p36, N=60,000, 272 pairs): null (max |z| 3.26 vs 3.74 Bonferroni; planted 2% bias reads |z| 8.9).
- Byte counter: option B (AGENTS.md section 5a, band.rs) stays; bytes are operations only at word-store price, immaterial (<= 0.15%); memory reported on its own axis. See `REPORT.pdf` section 4.

Files: `REPORT.html`/`.pdf`, `multitarget.svg`, `multitarget-summary.json`, `scripts/`, `logs/`.
Reproduce: `python3 scripts/mt.py 20 12` (about 2 minutes), `python3 scripts/leak.py`.
