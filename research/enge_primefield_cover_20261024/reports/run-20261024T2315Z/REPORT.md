# Native replay record: 2026-10-24T23:15Z

Run ID: `enge-primefield-cover-20261024-native-2315Z`.
Parent study: [REPORT.md](../../REPORT.md).

## Execution

- Requested and completed: one exhaustive census for each of five cover configurations `(d,s)=(2,0),(3,0),(5,0),(7,0),(2,6)`; one deterministic rho control. No failed native census.
- Command: `cargo run --release --offline --manifest-path research/enge_primefield_cover_20261024/Cargo.toml --bin enge-primefield-cover-toy > research/enge_primefield_cover_20261024/reports/run-20261024T2315Z/results.json` from the repository root.
- Input: hard-coded public curve `E/F_101: y^2=x^3+x+46`, `G=(2,37)`, `Q=(64,19)`, and exhaustive coefficient loops. No random seed; rho uses deterministic salts `0..20`.
- Host: `ip-172-31-3-58.us-east-2.compute.internal`, x86-64, Intel Xeon 6975P-C, 32 logical CPUs. Shared-host execution, with no CPU pinning or isolation claim.
- Source base revision: `5cb553810e5bbfbd8691ec614ffcaacefc625757` (`origin/main` at branch creation). The collector was formatted and linted after this run with no logic change. The current source digest is `4d5ed2e5ee973bbadffc69c4fb2286ccd5c2b6fad9ac7f8d77b258ea259409c1`; a fresh run from that source reproduced this JSON byte for byte.
- Raw record: [results.json](results.json). Root copy: [toy_cover_sieve_result.json](../../toy_cover_sieve_result.json).
- Result SHA-256: `4ec26659815293875c81529b02e22e8693c5eb45ad042fb0907ac23064a31864`.

## Observations

The five public-section full split counts are `30,0,0,0,8` out of
`10,201` sections each. The two degree-two target-section native full split
counts are `446` and `202` out of `9,999` target candidates each.
All `202` accepted `(2,6)` native target rows had matching direct elliptic
norm zero sets. Target-only recovery in `(2,6)` occurred after `1,070`
candidate sections, `108,070` cover-point tests, and `22` accepted rows;
the signed rank was `21` on `22` columns and the verified scalar was `37`.
The rho control found the same scalar after `72` elliptic group operations.

These are deterministic counts from one complete census per cover, so there
is no sampling interval. Cover-point tests, rank solves and elliptic group
operations are different units. Wall time, memory, shift-selection work,
setup and failed-section factorization costs were not measured. This record
does not claim a speed ratio.

![Relation counts](../../relation_counts.svg)

The root [REPORT.pdf](../../REPORT.pdf) contains the graph and the full study
summary. The figure source and tabular data are in the study root. The
canonical IC progress and leaderboard graphs were checked conceptually
against their scope and left unchanged: this run supplies no calibrated
IC/rho ratio, pipeline speed, or new best regime.
