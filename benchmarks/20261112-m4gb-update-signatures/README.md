# M4GB divisibility signatures in update(), 2026-11-12

Stage diagnostic (one decomposition call through M4GB), not a leaderboard
row: no table row in `benchmarks/README.md`, no VALIDATION.json. Study,
profile, protocol and the pre-stated expectation:
[`research/m4gb_update_signatures_20261112/`](../../research/m4gb_update_signatures_20261112/).
Record `benchmarks/records/20261112T0517Z-m4gb-update-signatures.json`
(measured; raw `corpus_ab.json`).

M4GB patch 0004 decides divisibility and lcm exactly from 64-bit monomial
signatures instead of decoding degrevlex-ranked monomials. Paired corpus of
90 captured descent systems, old build (0001–0003) against new (0001–0004)
plus an A/A arm, 5 repeats, through `--serve`, one thread, local container
`[pcc:unverified]`:

- 0 disagreements in reply or output bytes over 1,350 requests.
- Per call 0.57 to 0.75× of the old build from 14 to 24 unknowns (0.57 to
  0.64× at 22 to 24), below the old build on 65 of 66 systems; the A/A cell
  medians stay at 0.93 to 1.11×.
- No effect at 12 unknowns; 0.88 to 0.89× on the degree-6 m = 3 systems.

Files: `REPORT.pdf`, `results.svg`, `results.png` (copies of the study's).
