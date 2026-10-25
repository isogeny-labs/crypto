# Complete primary N83 solver-setting panel, run 40193751812

The [frozen eight-stratum protocol](../solver-setting-panel-20261025/PROTOCOL.md)
ran in [GitHub Actions 40193751812, attempt 1](https://github.com/aburan28/crypto/actions/runs/40193751812)
at head `a5af8d5023a644e9e3b347af4d8495540c8dea99`. Its eight inputs are
SHA-256-pinned, S3-backed natural-query XCNFs on the primary `a=0` Koblitz
GF(2^83) curve. It crossed two branch strategies and two Gaussian-matrix
limits, then repeated the baseline on each input. Every one of the 40 arms
returned `INDETERMINATE` at the 600-second solver cap. The panel is complete;
its outcomes are all censored.

[`summary.json`](summary.json) independently replays all eight retained
provider ZIPs, their metadata and terminal jobs, the complete provider-run
and artifact inventories, and the saved results of a second full collection.
The [collector](../solver-setting-collector-20261025/README.md) re-downloaded
the exact prior source ZIPs, pinned solver ZIP and every result ZIP from the
provider; all provider SHA-256 digests and byte counts matched. Its eight
audits match the separately saved per-stratum audits. The complete collector
summary is in `run-40193751812-attempt-1/collection.json`; the complete
provider snapshots and both sets of audits are retained beside it. The
summed 24,119.042 solver process-wall seconds represent eight parallel
provider hosts and capped searches. They are neither elapsed wall time for
one solver nor a complete one-target cold index-calculus runtime.

| Stratum | Provider artifact ID | ZIP SHA-256 |
| --- | ---: | --- |
| `gray_4_2_t1_m0` | 12599809244 | `f85714e38b1375dd9cdeb8c7a272277e766cb87b99a19c9446e9c1baaa38441f` |
| `gray_4_2_t2_m0` | 12599694537 | `e0a8d720ad878bfb31cc67bc53d743b5e0533a7ce38ad4eda67fd10b798abd1d` |
| `gray_5_1_t1_m0` | 12599749878 | `239114d768819ae3869043e23b8f7a0362e309bb8528a4f4db0569ac43a85cbd` |
| `gray_5_1_t2_m0` | 12601020945 | `851c650eb6241baab31c69f0f3d5ea2eb81a31a60c3bf33f3709cb57ec7110e0` |
| `hash_4_2_t1_m0` | 12600483557 | `ee7163aeb4d99f5f348d2d0d5dc72350e1e36fb5a776fa366d688baf81fceb93` |
| `hash_4_2_t2_m0` | 12601641853 | `cbdcbc3a8db9134426cdb0ca56d7f3195519ace284ee205279f07dd37a18974e` |
| `hash_5_1_t1_m0` | 12601070492 | `ef7bffd4979dcff49d997e4a277554ba10353fb32d522de2bdc3a74a6ab2a32f` |
| `hash_5_1_t2_m0` | 12600024758 | `316567d5bb2f3a6062766146d076cb49fb00b6f0cf3865278a38f2febda7e0ac` |

[`retain.py`](retain.py) checks exact provider digests, bytes, run/head/attempt,
the eight successful terminal jobs, frozen arm order, complete collector
inventory and summary, and agreement of both audit passes. With the source
and solver ZIPs it reruns the [raw-stream auditor](../solver-setting-panel-20261025/audit_artifact.py)
and requires exact agreement before returning
`PASS_RETAINED_ALL_EIGHT_CENSORED`. Without them, its status is
`BINDING_ONLY_SOURCE_ZIPS_REQUIRED`, even though the committed provider
inventory still reconciles. Its tamper controls reject a changed ZIP, forged
audit, changed provider head and disagreeing second audit. To repeat the full
replay, use the exact-run collector into a new local directory, provide its
source trial ZIPs and metadata under the `t1_m0`/`t2_m0` names expected by
`retain.py`, and pass its `solver.zip`.

This screen found no checked natural relation and cannot rank the factor
bases by total runtime. Conflict throughput under censoring is a diagnostic,
not a time-to-relation estimate. `selected_best_total_runtime` remains
`null`. Selection still requires observed natural relation yield, graph
closure and rank, verified factor-base logarithms, unseen-target descent,
fully charged one-target cold accounting, and matched same-target rho. The
diagnostic `a=1` curve has a different subgroup and is outside this ranking.
