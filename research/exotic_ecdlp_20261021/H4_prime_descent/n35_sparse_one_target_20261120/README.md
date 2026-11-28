# H4 n=35 sparse candidate: one fresh public target

The sparse target-check candidate recovered scalar **22,627,365** for public point `(4111367897, 4655813480)` in a target-online interval of **2.633 ms**. Installed Sage rho independently recovered the same scalar from that exact point in **1,098.946 ms**. The observed same-point rho/IC online ratio was **417.36**. The one-target workload is `Wf6cb4a13a4a3`; the candidate is `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0he546b5410907`.

| One-target exploratory local run | Value |
| --- | ---: |
| Field / subgroup order | `2^35` / `520604191` |
| Actual factor-base points / signed columns | 4,096 / 2,048 |
| Distinct pair-index sums | 8,321,091 |
| Target third-point scans / shifts | 202 / 1 |
| IC target-online interval | 2.633 ms |
| Same-point rho target-online interval | 1,098.946 ms |
| Observed rho / IC online ratio | 417.36 |
| Rebuild and factor-log replay setup | 140.704 s |
| Peak process RSS | 1,590,771,712 bytes |

The new target was fixed by SHA-256 label and checked against the earlier n35 primary point and all ten panel points before measurement. Its fixture scalar was used for construction and independent replay, outside both online clocks. The candidate rebuilt the exact base and pair index, loaded the previously solved 2,048 representative factor logs, and verified every log against the generator before starting the IC online clock. That replay setup is recorded separately. The original relation collection and matrix solve are preserved in the [complete n35 pipeline](../n35_full_pipeline_20261120/README.md); they were not rerun as cold work for this sparse candidate. The run therefore measures the specified one-target online objective and a reproducible precomputation replay cost, while a fresh empty-cache cold sparse run remains a separate experiment.

The IC clock includes target query, all 202 third-point scans, the original three-point relation check, the sparse signed-column reconstruction, log recovery, and scalar replay. Its phase costs were 0.004 ms query, 2.339 ms point decomposition, 0.184 ms relation check, 0.057 ms descent, and 0.049 ms recovery check. The first unshifted target succeeded. Rho used the same public point and one worker, starting with target-dependent walk work and ending after scalar replay.

The [fixture](fixture.json), [protocol](protocol.json), [fixture generator](make_fixture.py), and [runner](run.py) were pushed in `7f6830693a8` before measurement. The [checked Sage runtime](sage-runtime-info.json), [result](result.json), and [independent verifier](verify.py) preserve source identity, phases, target, scalar certificates, and resource use. The verifier rebuilds the pair index in a different traversal order, checks the exact first witness and signed relation, replays every factor log, and verifies both recovered scalars on the target. Its [validation](validation.json) records the outcome. The [sparse candidate proof map](../n35_sparse_check_20261120/formal/PROOF_MAP.md) identifies the checked structural results and the concrete Sage bridge.

This is one exploratory ordinary-host observation. A paired, alternating-order Tier 0 or Tier 1 panel with topology, affinity, throttling, steal-time, and noise receipts is the next performance test. The prior ten-point sparse/dense panel supplies a separate implementation comparison on frozen points; neither result alone estimates an isolated-host speedup.
