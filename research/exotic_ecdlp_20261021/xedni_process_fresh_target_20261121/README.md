# Fresh Sage processes expose fixed-curve table setup on one public target

Three separate checked Sage processes rebuilt the 28-row rational relation
matrix and anchored lookup table for the frozen public target `T=(10,1)`.
Each table arm and its same-point Floyd-rho arm recovered and independently
verified scalar `22`. The **median process-fresh in-process table setup was
30.539 ms**; median table online lookup plus verification was **0.0605 ms**,
and median same-point rho online was **0.2435 ms**. The median setup-plus-table
online interval was 30.557 ms. Full checked-launcher wall time was recorded
separately at a median of 1,628.295 ms.

| Process | Arm order | Launcher wall ms | Table setup ms | Table online ms | Rho online ms |
| ---: | --- | ---: | ---: | ---: | ---: |
| 1 | table, rho | 1,666.490 | 66.001 | 0.0665 | 0.1901 |
| 2 | rho, table | 1,623.782 | 30.539 | 0.0175 | 0.2435 |
| 3 | table, rho | 1,628.295 | 29.757 | 0.0605 | 0.3081 |

The [protocol](protocol.json) froze a new SHA-256 target law, the one-target
workload, relation prefix, process separation, three arm orders and timing
boundaries before execution. The coordinator starts the repository's checked
Sage launcher anew for each repetition. Each worker rebuilds its rational
curve, 28 word rows, 21-column sign-folded matrix, anchor solve and table.
[`process-rows.jsonl`](process-rows.jsonl) retains launcher wall, worker
stdout/stderr, exclusive setup phases, arm start/stop intervals, statuses,
scalars and rho walk counts. All three processes completed within the
frozen 60-second cap, and all six arms have verified status. The
[`result.json`](result.json) pins source and raw-row hashes.

The [independent audit](audit.py) reconstructs the order-43 finite group
with an integer group law, derives the public target from the frozen seed,
replays all six scalars and checks source hashes, arm order, timing sums,
the 28-row/21-column/anchored-rank metadata and process outcomes. Its
[`audit-result.json`](audit-result.json) is `PASS`. The source panel's
independent modular matrix audit is pinned by hash, and the checked Sage
runtime receipt is retained.

## Accounting boundary and proof map

The target fixture is built before both online arms. Table online starts
at public-point fold and lookup and ends after scalar replay. Rho online
starts at its first target-dependent walk state and ends after scalar
replay. Curve creation, rational relation generation, matrix construction,
final linear algebra and table construction have exclusive setup intervals
whose sum is the table setup. The checked-launcher wall includes process
startup and fixture work; it is reported separately from the in-process
setup. OS-level file and page caches may persist between the three new
processes.

| Result | Formal or independent evidence | Status |
| --- | --- | --- |
| Rational group relation on the selected rank-one curve | `XedniRankOneAnchor.rational_relation` in the [curve packet](../xedni_anchor_rank1_search_20261121/README.md) | Lean checked |
| Endpoint coverage and counting limit | `prefix14_covers_nonzero`, `endpoint_card_le` in the [counting packet](../xedni_endpoint_count_bound_20261121/README.md) | Lean checked |
| Anchored relation matrix and scalar replay | Source [matrix audit](../xedni_fixed_curve_target_accounting_20261121/README.md), fresh-process Sage results, this independent group audit | Verified on the declared order-43 instance |

These are ordinary-host component timings without a Tier 0 or Tier 1
isolation receipt. The online table uses a full prepared scalar-labelled
endpoint table; its setup and launcher costs are recorded above. The next
algorithmic experiment must replace that enumeration with target-driven
relation discovery under a frozen public input law before a general
xedni-calculus cost claim is possible.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
python3 research/exotic_ecdlp_20261021/xedni_process_fresh_target_20261121/run.py
python3 research/exotic_ecdlp_20261021/xedni_process_fresh_target_20261121/audit.py
```
