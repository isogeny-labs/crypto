# Restored Docker and native retained-base continuation

The packaged Docker service was inactive and was started successfully with
authorized `sudo systemctl start docker.service`. Task-local sudo Docker access
leaves global socket/group permissions unchanged. The frozen native x86-64
worker loaded in the pinned image; a live preflight confirmed one CPU,
4,294,967,296 bytes of memory, zero swap and no network. All six stages retain
filtered live Docker inspection and verified container cleanup.

The exact K4096 archive was reconstructed and all **679,936 points / 4,096
representatives** passed generic multi-limb replay. Its compressed/plain/
point-set hashes and counts match the retained original. It was published to
`setec-astronomy-artifacts` with overwrite protection and AES-256 encryption;
fresh HEAD/GET and download hashes passed. Both search inputs were freshly
downloaded from that bucket. Historical original-bucket provenance sidecars
remain unchanged; the actual current transfer channel is recorded separately.
See [fresh inputs](input/fresh-inputs.json) and
[K4096 publication](k4096-publication/publication.json).

## Observed results

Both construction calls passed the original-group input checks, exact
all-affine model counts and independent custody/resource audits.

| Base | SAT variables | Total clauses | Domain clauses | Cgroup peak bytes |
| --- | ---: | ---: | ---: | ---: |
| K2048 | 111,478 | 1,140,557 | 892,470 | 317,931,520 |
| K4096 | 131,958 | 2,021,197 | 1,773,110 | 635,953,152 |

The primary [protocol](PROTOCOL.md) and separate
[short-control protocol](SHORT_CONTROL_PROTOCOL.json) were committed before
their respective runs. Both use m=5, orbit-phase domains, one configured
public trial and the same variable/domain/model/resource limits.

| Base / conflict cap | Outer cap | Observed supervisor wall | Disposition | Final conflicts / models / relations / rank |
| --- | ---: | ---: | --- | --- |
| K2048 / 100,000 | 600 s | 600.612 s | UNKNOWN_wall_cap | unknown |
| K4096 / 100,000 | 600 s | 601.215 s | UNKNOWN_wall_cap | unknown |
| K2048 / 1,000 | 180 s | 162.630 s | UNKNOWN_solver_cap | 1,000 / 0 / 0 / 0 |
| K4096 / 1,000 | 180 s | 181.095 s | UNKNOWN_wall_cap | unknown |

Outer times include cleanup after the watchdog. The completed K2048 control
made one SAT call; solver-stage wall was 155.640 seconds and cgroup peak
was 484,421,632 bytes. Its independent original-curve replay checks the
recorded public query, which matches the historical reference exactly.
No K4096 terminal attempt record was emitted, so actual paired terminal-query
identity is unverified despite identical frozen public fixtures. See
[query identity](query-identity.json) and [stage summary](summary.json).

Wall-censored attempts supply no final conflict/model/relation/rank count or
worker peak. Their events confirm base readiness and relation collection,
not independently observed SAT entry. They are not zero-relation or UNSAT
measurements. The separate short controls do not replace the retained long
attempts. Repeated use of one fixture is not a disjoint holdout campaign.

## Audit, cost and storage limits

Six stage audits pass. Six final controls reject forged target-success or
SAT-count receipts even with recomputed worker hashes. Actual run evidence
is unchanged. Initial control-harness failures and their corrections are
preserved locally and summarized in [validation](VALIDATION.md).

The declared main and short worker envelopes are 2,040 and 360 seconds.
Observed supervised construction/replay/capacity/search/control intervals
sum to **1,603.999884 seconds**. Docker/image preparation, storage IO, audits
and validation are separate costs; no complete aggregate interval or unused
historical allowance is asserted. See [accounting](accounting.json).
These are shared-host bounded observations, not isolated throughput or
fastest-tool comparisons, and not comparisons with the earlier emulated host.

The 5,771,044-byte, 224-entry raw archive was prepared and locally verified
at SHA-256
`b7afa24e3ca80630344b5e584915227c5e177c66068f6f5749ce72ba4894ba92`.
It contains the frozen worker, source snapshots, public input metadata,
logs and controls; factor-base archives are stored separately. Automatic
approval review rejected this broader S3 transfer because the exact payload
exceeded the authorized factor bases. Its upload process never started.
The archive remains local at the path in
[approval status](raw-artifact-status.json); [contents](raw-artifact-manifest.json)
are retained for review. Compact receipts are committed here; full source
snapshots, logs and binaries from that ZIP are not transferred through Git.

## Requirement status

| Requirement | Status |
| --- | --- |
| Restore Docker and guarded execution | Verified complete for this executor |
| Store exact K2048/K4096 bases in the selected bucket | Verified fresh setec round trips |
| K4096 model construction | Verified complete; source projection confirmed |
| Native bounded query probes | Executed; three wall-censored and one completed solver-cap receipt |
| Broader raw evidence upload | Blocked by automatic approval review; exact ZIP ready locally |
| Full finite grid, WDSat/FES/large-prime combinations | Original declared grid remains mostly unexecuted; no additional adapter coverage claimed |
| Natural useful rank, column logs, individual log, matched references and complete cold runtime | Unresolved; selected_best_total_runtime remains null |

The next discriminating task is
[native K4096 phase/counter observability](../../../../problems/n83-native-k4096-terminal-observability.md).
That task is proposed, not running in the background. These measurements do
not select a best factor base or establish a pipeline improvement.
