#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session B (`ECBS1h90cd7a0dc213`, re-run)

*Spec* `specs/B-16k.json` (`ECS1ha361677676e8`), repository head `855c0bc1f` (clean), binary `c1378033…41c5`
(built from `e59331c96`), host env class `ECBENV2h6639ae09b95e`, `ecbench run --wait --cpus auto`, run CPU 3.
Started 2026-10-20 08:58 UTC, finished 10:13 UTC.

*Workloads* the first 4 targets of session A's law and seed. *Rounds* 2 measured + 1 warm-up, alternate order.
*Arms* `rho-neg`; `scalar-16384`, `batch-16384`, `blocked-16384`.

*Runs* 48 planned, 48 written, 48 verified. *Isolation earned*: IC runs L1 (all 8 per arm);
`rho-neg` L2 (7) and L1 (1). Fewer than 5 rounds: wall times descriptive by the protocol.
*Audit* OK, 6 exact replays (`sessions/B-16k-r2.audit.json`).

*Interrupted first attempt* `ECBS1h64e9d03d695a` (`sessions/B-16k`), 08:14–08:57 UTC: 26 runs
verified, then run 27 (`scalar-16384`) recorded `crashed` by SIGTERM from the operator's tool time
limit. Kept unchanged; not analysed.

== Measured values (means of 8 measured runs per arm)

#table(columns: 6,
  [*arm*], [*S (measured)*], [*set-up wall*], [*relations wall*], [*LA wall*], [*peak RSS*],
  [rho-neg], [0.802 [0.415, 1.214]], [—], [—], [—], [—],
  [scalar-16384], [4097.226 [4096.986, 4097.446]], [224.1 s], [103.7 ms], [3.63 s], [9.15 GiB],
  [batch-16384], [4097.226], [116.5 s], [102.4 ms], [3.74 s], [9.15 GiB],
  [blocked-16384], [4097.226], [28.20 s], [96.4 ms], [1.61 s], [4.65 GiB],
)
S intervals from `ecbench table` (`analysis/table-B.md`). 24 of 24 executions identical across
the three oracles; blocked/batched set-up ratio 0.24 (pair range 0.22–0.26), blocked/scalar 0.13
(0.12–0.14). Derived values: `analysis/B-16k.json`.

#figure(image("../../figures/phase_wall.svg", width: 85%), caption: [Each phase's share of a run's wall time per arm, sessions A and B.])

Records: `sessions/B-16k-r2/records.jsonl`. Observations only; the interpretation is in the study README.
