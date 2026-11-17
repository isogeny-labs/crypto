# Native N83 higher-conflict factor-base continuation

Requested: raise conflict caps for K2048/K4096, continue native N83 sweep,
and upload every new raw record to setec-astronomy-artifacts.

The [frozen protocol](../../verification/native-natural-100k-20261019/PROTOCOL.md)
raises each cap10000→100000conflicts, with900seconds per worker and990seconds
per mechanical command, K2048 then K4096. A separate
[setup correction](../../verification/native-natural-100k-20261019/LAUNCH-SETUP-V6C.md)
retains two pre-solver failures and freezes new cards at660f0f4b10. These are
separate observations from prior1000/10000-conflict trials, not repetitions
for a pooled timing estimate. No actual timed solve was retried.

Curve: public synthetic a=0 `icv1-f2m83-tm6151469093347-debefd74`, polynomial
x^83+x^45+x^2+x+1, subgroup order2417851639230796216685689. This is distinct
from the positive-trace a=1 N83 cold leaderboard holdout. EC1 mapping remains
unresolved; no new curve/registry equivalence is inferred. Workload:
public_x_hash seed2026100801, the same retained single public query,
five summands, identity mask0, exact retained K2048/K4096 panels.

## Outcomes

Both new trials reached100000conflicts with zero models, relations and rank.
Native query/custody replay and director receipt/output-digest checks passed.
Neither reached the900-second wall limit or OOM. The exact measurements are
listed in the measured-outcomes table below. Native
solver intervals are derived in Rust as return-phase elapsed minus entry-phase
elapsed. Supervised outer wall includes other costs. Missing terminal data
from a killed prefix remains null. Unknown does not mean UNSAT or absent
decompositions; these bounded trials cannot identify an optimal factor base
without useful relations, rank and complete cold-runtime evidence.

## Setup refusals and exact execution

Original v6 shell redirection failed because the new relay parent did not
exist; neither card runner nor solver executed, and no actual receipt/raw
stdout was produced. Director retained the mechanical failure observation
and an absent-output assessment. After parent creation, v6b entered the
native runner and stopped at command1 with `checkout not clean` before
creating worker output: new JSON relay/receipt files were not ignored.
The initial ignore check used a .log path and was insufficient. No native
solver ran in either attempt. Both plans/failure records/assessments remain.

The committed setup-only correction ignores the entire new exec subtree;
explicit JSON path checks and clean status pass. Separate v6c K2048/K4096
cards preserve the original conflict/wall/input/resource/verification limits
and use unique output paths. Director compiles and assesses; mechanical
executes the exact prompt without retries/computation, with actual stdout
captured directly by noclobber redirection. No relays or absent measurements
are fabricated. The raw archive includes obsolete plans as plans, not runs.

## Conditions and validation

Unchanged native indexed worker SHA256
3a31bd2bedee5519d30b517f39ec27f72561ce898213657688c55b51bbcdf7b6;
immutable runner-preflight-v4 SHA256
da7481808e38a60be7231bc462f13350ca7c74b27f745dafd4362038aed152e1.
No code changes or rebuilds. The frozen source660f0f4b10 differs from the
validated96cf46560d only in this new protocol/setup metadata.

Host managed Linux x86-64 Xeon6975P-C,32vCPU/128GiB. Same immutable
Docker image541096ae9b05…,1CPU quota/4GiB/no extra swap/networknone,
readonly root/source/inputs/worker,capdropALL,no-new-privileges,64PIDs.
Native guard holds exclusive lock, reserves CPU/sibling, evicts movable user
threads with restoration and checks quiet preflight with bounded readiness.
Actual CPU/samples/evictions/inspect/cleanup are retained for each run.
Fixed kernel threads and missing worker context-switch/fault accounting
prevent an automatic L2 claim. No builds/tests/heavy work from this task
runs during timed arms. These are descriptive n=1 bounded observations,
without A/A uncertainty, a timing-rank admission or fastest-tool claim.

Inherited validation on the identical worker/library: full serial release
library4398passed/118ignored/0failed, exporter19tests, model checker2,
final native runner14unit+7card tests. Logs and hashes are preserved as
inherited evidence, not misreported as fresh test reruns. Native audit
checks original field/group queryaG+bQ, ordered phase markers, source/input/
ELF/stream custody, declared caps, exit/OOM and cleanup. Positive relations
stay fail-closed until independent original-group/rank/scalar replay supports
them. Only Rust/native tools run research, replay and derive result values;
Python only renders recorded values and inspects PDFs.

The native summary helper also emits known-witness comparison fields, which
are null/false here because no new A/B/E controls are in this continuation.
Those unused fields do not change the separate per-natural-run audit statuses.
Its generic engineering label comes from the original XOR-component schema;
this higher-cap round is a bounded diagnostic, not an algorithm improvement.
No external-tool timing comparison is attempted for these bounded-search
observations; no algorithm-change or IC/rho speed ratio is claimed.

## Artifacts and graph scope

[Native data](figure-data.json), [plot source](render.py), [Typst source](report.typ),
[inspected PDF](report.pdf), [cap/clock observations](cap-continuation.svg),
[execution diagram](execution.svg); each SVG has a same-source150dpiPNG.
[Benchmark copies](../../../../benchmarks/20261019-n83-natural-100k/).
[Compact records and publication](../../verification/native-natural-100k-20261019/RESULT.md).

The prior XOR component leaderboard, cold scoreboard, progress timeline,
curve-registry diagrams and rho values stay unchanged: this round changes
a search bound and does not establish a new measured algorithm improvement,
complete cold pipeline or curve link. The new cap/clock graph records the
bounded trials separately; it is not a factor-base yield/time ranking.

Next discriminating step depends on actual outcomes: a positive observation
requires full natural original-group/rank/scalar replay before promotion;
bounded unknowns leave useful yield open and motivate a separately frozen
higher-budget or changed-query experiment. No next run is claimed as scheduled.

## Requirement status

| requested requirement | completion evidence |
|---|---|
| Raise both conflict caps | frozen100000conflict/900second specs; actual terminal counts below |
| Continue native sweep | separate K2048/K4096 records and director assessments |
| Upload every new raw record | [publication receipt](../../verification/native-natural-100k-20261019/PUBLICATION.md); complete only after fresh GET, payload checks and native replay |
| Preserve earlier and failed evidence | two pre-solver failures, original plans and prior S3 content addresses retained |
| Useful natural yield/full-cold base selection | unresolved unless new verified positive evidence explicitly establishes it |
## Measured outcomes

Both new trials reached100000conflicts, within their separate900-second wall
limits, with **zero models, relations and rank**. Both native query/custody
replays and director exact-relay/output-digest assessments passed. No OOM,
wall censor or timed retry occurred. These are bounded unknowns, not UNSAT.

| K | conflict cap / actual terminal conflicts | models / relations / rank | native solver interval ms | supervised outer wall ms |
|---:|---:|---|---:|---:|
| 2048 | 100000 /100000 | 0 /0 /0 | 383937.390858 | 393756.253810 |
| 4096 | 100000 /100000 | 0 /0 /0 | 395987.838978 | 407619.165871 |

Values are recorded or derived by the frozen native summary helper. Each
K/cap is n=1; timing uncertainty, A/A spread and a timing-rank comparison are
not established. The earlier1000/10000-conflict rows in the graph are separate
frozen observations, linked through the prior raw S3 publication; they are
not combined into a speedup or mean. No useful relation/rank, recovered
unknown scalar, optimal base, full cold IC time or matched-rho ratio follows.
