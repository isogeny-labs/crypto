# E-222 / M-221 analog ladder: matched rho reference and distance (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, no replay certificate, no validator report, no §5a band. Not a leaderboard row.

**Run.** `ic rho --prime-bits 10,12,14,16 --runs 64 --json` (seed default; all walks `ok`), output `results/rho1/rho_matched.json`. Matched reference = negation-map walk (A = 2). Mean group-addition equivalents (GAE) per solve at log2 r = 9.68 / 11.93 / 13.99 / 15.99: 103.6 / 162.0 / 252.5 / 441.4. These are the repository's generated prime-order curves of those sizes, not the E/M/W instances; rho cost depends on r, not on the curve family, so the reference is applied by interpolation in log2 r.

**Distance (lower bound).** IC cost is the baseline's median `trials_total` (`results/baseline1/summary.json`) counted at one GAE per trial; a trial does at least one group addition, so the ratio is a floor on IC/rho. Full table: `results/rho1/distance_table.txt`. Ranges over rungs 10 to 17 bits: W (control) 4.8x to 81.6x, E 18.5x to 359.9x, M 30.6x to 655.3x; every row grows with size (baseline cost about doubles per bit, rho about 2^(0.33 bits), fitted α in the report).

**Caveats.** (1) Trials are not priced in GAE; the true ratio is higher. The band (0 / 0.1 / 1 GAE per unpriced native operation) was not applied to either arm. (2) The sawtooth between odd and even rungs comes from using r's bit length against an interpolated log2 r. (3) The E and M rows pay the field-size ratio p/r (baseline 1); at equal p the gap to the control is about 1.
