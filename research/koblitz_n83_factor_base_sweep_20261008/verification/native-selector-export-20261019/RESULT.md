# Native free-selector witness gate: verified

The original fixed-coordinate/free-selector known-witness workload now
passes native model/domain/group audit within its unchanged 5,000-conflict,
180-second, one-CPU/4 GiB envelope. A separate exporter reads the actual
model's representative indices and phases only after solving; it adds no
selector assumptions. Independent polynomial-word arithmetic replays the
exact domain x-coordinates and original five-point group equation.

| Observation | Fully pinned | Original free | Free + post-solve export |
| --- | ---: | ---: | ---: |
| Conflicts | 0 | 787 | 787 |
| Decisions | 23 | 35,307 | 35,307 |
| Initial propagation complete, ms from solve entry | 163.684271 | 188.398658 | 192.850022 |
| SAT return, ms from solve entry | 312.984122 | 167735.758707 | 167259.951266 |
| Worker wall, ms | 433.663717 | 167817.248441 | 167341.433194 |
| Outer wall, ms | 1001.912152 | 168213.674931 | 167835.628938 |
| Worker exit / wall censoring | 0 / false | 0 / false | 0 / false |
| Model/domain/group audit | PASS | stopped: no selector export | PASS |

Each is one L0 observation without a confidence interval. The free controls
install the same 101,636 variables, 714,736 clauses and 4,980 XORs with eight
fixed x blocks. Pinned selectors add a different assumption set. Matching
search counters do not establish a performance comparison. Earlier 180 s
censoring remains preserved with unknown final counters; no speedup is
inferred from its different outcome.

The verified export trial records 167102070169 ns XOR propagation,
70147211 ns clause propagation, 16224291 ns analysis and 3634380 ns
minimization. Initial propagation finishes before CDCL entry at 193.136020 ms.
This localizes this control's cost to XOR work during search. The counters
are existing cumulative solver timers, not a full IC cost decomposition.

Boundary source: 5e9ee5d4b1aa6ca22368213e5bc05b343e00bfb4;
[protocol](../native-sat-boundary-20261019/PROTOCOL.md).
Export source: cbe8e44da5ac0a792922d746368882a3b8f40170;
[follow-up protocol](PROTOCOL.md). See [records](records/),
[transport](transport/), [validation](VALIDATION.md),
[forgery rejection](forgery-controls.json), [raw archive](RAW.md),
and [visual report / inspected PDF](../../reports/run-20261019T0525Z/REPORT.md).

The original free card stops at its explicitly unsupported positive
terminal; its missing audit and raw SAT remain unchanged. The audited
export is a separate run. The pinned relay checker initially fails because
JSON float reserialization changes 1.1790702610000001 to 1.179070261. The
relay equals the original receipt byte-for-byte. A native byte-preserving
correction and regression test verify it and every output digest; the
original failure and impeded assessment are retained. A coherently rehashed
model-phase forgery is rejected with independent selector/phase replay
mismatch. All three runs have verified cleanup, no OOM, and exact relays
assessed before HEAD changes.

Added container allocation is360+180=540seconds. Observed outer walls sum
to337051.216021 ms (derived by adding the three receipts). Build, test,
audit, render and storage work is separate. Earlier rounds keep their own
accounting; no global spent total is reconstructed.

| Requirement | Status |
| --- | --- |
| Exact approved49MB bundle | [Verified S3 publication](../native-phase-publication-20261019/RESULT.md), fresh download and308 payload checks pass |
| Docker/native continuation | Verified: three bounded observations, resources and cleanup |
| Original free-selector witness | Verified for this fixture/cap, actual model selectors and original group sum |
| Native validation | Identical-library full suite4397 pass/118 ignored; exporter 19, pinned selector 1, decoder1, native runner 15 pass |
| Broad sweep / best full-cold base | Partial: natural yield/rank, positive relation/rank/scalar replay and complete cold comparisons remain open |

Primary curve: icv1-f2m83-tm6151469093347-debefd74, a=0,
polynomial x^83+x^45+x^2+x+1, subgroup order2417851639230796216685689.
Fixture SHA256 fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538.
No unknown scalar is recovered. Natural relations, rank gain and selected
best total runtime remain null for these controls. Prior natural K2048 /
K4096 trials retain zero models/relations/rank at 1000 conflicts. The peer
ABBA receipt retains its separate source/host and Bash/Python audit
provenance; this continuation does not recertify it.

Next work: frozen native XOR-engine profiling/candidate comparison on this
verified baseline, and complete positive natural-relation replay before
yield promotion. Neither is launched or scheduled here. Full-cold
leaderboard, scoreboard and timeline remain unchanged.
