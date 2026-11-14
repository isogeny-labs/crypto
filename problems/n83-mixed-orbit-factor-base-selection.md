# N83 mixed-orbit relation yield and total-runtime selection

- **Status:** `investigating`
- **Kind:** empirical bottleneck and implementation gap
- **Affected work:** [N83 factor-base sweep](../research/koblitz_n83_factor_base_sweep_20261008/), [external XOR handoff](../research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/PROTOCOL.md), [full-D18 native continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/PROTOCOL.md)
- **Next action owner:** N83 factor-base research task
- **Last updated:** 2026-11-14 UTC; [native cadence-32 attempt 7](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-7/REPORT.md), [free-choice external CryptoMiniSat probe](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/external-cms-20261114/RESULT.md), [ordered XCNF replay](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/ordered-xcnf-20261114/RESULT.md), and [ordered external solver cap](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/ordered-cms-20261114/RESULT.md)

## Problem

The primary Koblitz GF(2^83) factor-base sweep cannot yet select a base by
fully charged index-calculus runtime because retained explicit-orbit bases
have not produced a measured natural relation yield, closure/rank, linear
algebra, target descent, and same-target rho comparison under one frozen
end-to-end protocol. The current 5+1 and 4+2 mixed-orbit searches expose
bounded, first-identity-pattern SAT instances; a capped search leaves the
relation-yield question open.

There is no complete cold IC baseline with signed PCC standing on this primary
`a=0` 81-bit subgroup [pcc:unverified]. The recorded K1107/root11 cold
comparison is on the separate diagnostic `a=1` 53-bit subgroup and cannot
supply this arm's baseline.

The affected curve is `icv1-f2m83-tm6151469093347-debefd74`, with the
primary-arm K=600 base and cross-stream Gray/hash residual pools. The
requested selection criterion is minimum total index-calculus runtime,
including setup and online phases, over the finite declared design grid.

## Evidence and uncertainty

- **2026-11-14 D18 update:** A source-bound ordered-first-pair, cadence-32
  constructed control completed in 50.414 seconds with independent group,
  row, and assignment replay. Its first public natural query returned
  `UNKNOWN_search_cap` at 100,000 conflicts after 161.878 seconds, with zero
  models or relations. A separately frozen free-first-pair,
  phase-zero-first XCNF export was checked against the exact D18 object;
  CryptoMiniSat 5.14.7 reached its 1,000,000-conflict cap on the constructed
  positive after 420.708 seconds and returned `INDETERMINATE`. The second
  probe withheld its natural solver arm by protocol. These are distinct
  encodings and do not establish a backend speed ratio. Their raw hashes,
  terminal statuses and resource receipts are retained at the links above.
  The external run's sampled peak RSS was 341,072 KiB with unchanged
  swapouts, so its censor is a bounded search outcome under this cap rather
  than a memory-admission failure. No D18 natural relation or useful-rank
  observation was promoted.

- **2026-11-14 ordered external-control update:** The archived native
  cadence-32 positive assignment satisfies every original clause and XOR
  row of the fresh ordered-first-pair XCNF (2,807,999 CNF clauses, 3,901 XOR
  rows). CryptoMiniSat 5.14.7 then returned `INDETERMINATE` on that exact
  satisfiable constructed formula at 1,000,002 reported conflicts after
  431.660 seconds; the natural arm remained withheld. Host load stayed below
  the frozen admission threshold and swapouts were unchanged. This is a
  bounded local execution observation with unverified PCC standing
  [pcc:unverified]; it does not show a natural relation or an end-to-end
  factor-base ranking. The linked ordered reports retain raw hashes, source,
  solver, model replay and process receipts.

- **Observed [pcc:unverified]:** The [independently replayed hosted 5+1 split panel](../research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-split-panel-20261025/attempt-1/replay-receipt.json) reached 20,000 native conflicts and zero models in each Gray/hash arm. Worker walls were 1,470,249 and 1,223,792 ms respectively on that Linux host; both statuses are `UNKNOWN_search_cap`. The prior 4+2 panel is likewise capped. The [local external-XOR receipt](../research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/receipt.json) records four first-pattern 5+1/4+2 Gray/hash arms, each ending `INDETERMINATE` at 2,000,001–2,000,002 conflicts with zero models after 119.2–192.5 seconds of L0 process wall. Its [replay](../research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/replay.json) rebuilds all four formulas from exact archive bytes and checks the raw-output hashes and counters.
- **Observed capacity and access gates:** The [full-D18 native preflight](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/PREFLIGHT.md) checked the exact 5,418,738-row S3 object, distinct producer and custody manifests, full-object provider checksums, clean source and release binaries. Its local one-minute load remained 55.95–74.19 against the frozen maximum of 24 while cumulative swapouts increased, so the cadence-32 worker was not launched. A [second source-bound preflight](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-2/REPORT.md) at `45ce91d5` again matched both S3 object hashes but sampled load 26.44–66.14 and 390,448 new swapouts; neither worker arm launched. Configured AWS spot and RunPod SSH endpoints were unavailable at bounded checks. The exact two-object read grant is now applied and read back, while PR #2649 GitHub Actions still stop before runner assignment at the account billing gate. These are resource/access conditions, not evidence about solver yield.
- **Third preflight and runner access:** A [third fresh-GET report](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-3/REPORT.md) again matched the three exact producer/wrapper/object hashes. Its recorded one-minute local load was 114.84, so the worker was not launched. A configured profile reached the deployed `ephemeral` runner in account `104186811093`, but direct HEAD of the original D18 manifest returned `403`. The [exact two-sided access proposal](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-3/ACCESS_PROPOSAL.md) is unapplied pending explicit authorization; it is separate from the already active `rho-gh-snapshot-readonly-20261024` user grant. A no-source On-Demand market smoke then failed at `RunInstances` because the account's `SpotOnlyEC2Launch` v2 identity policy explicitly denies this direct non-Spot launch. The tagged instance readback was empty. No D18 solver job was submitted from this proposal.
- **Fourth source-bound launch:** The [attempt-4 report](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-4/REPORT.md) matched a fresh S3 GET and passed the load/swap/scratch gate, but its worker exited at startup because its compile-time source path pointed to the now-dirty original checkout. The 1.05-second exit yielded no result JSON. Rebuilding from a clean frozen checkout is the exact correction; this is producer provenance failure, not relation-search evidence.
- **Fifth constructed control:** A [clean-source attempt](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-5/REPORT.md) passed fresh full-object custody and the local launch gate. Its 86.603-second worker result reported constructed SAT; separate native group, signed-orbit row, original CNF/XOR and field assignment replay passed. The `/usr/bin/time -l` wrapper nevertheless exited 1 on a denied `kern.clockrate` query, also reproduced with `/usr/bin/true`; the child's exit was not separately captured and the natural arm was held. The [versioned exit amendment](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/EXIT_POLICY_AMENDMENT.md) requires a fresh qualified rerun and explicit terminal exit 0 before natural. These constructed-control timings are not a natural yield or total-runtime comparison.
- **Sixth preflight:** A [fresh D18 GET and corrected-verifier report](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-6/REPORT.md) preserved the exact S3 hashes and a focused terminal-exit regression pass. The patched verifier rejected attempt 5's exit 1. The local launch gate then sampled one-minute load 65.77–69.31 against the frozen maximum 24; no attempt-6 worker arm ran.
- **Hosted Spot capacity:** The [two no-source smokes](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/spot-capacity-smoke-20261026/REPORT.md) completed marker commands with exit 0 on `t3.small` and 15-GiB `c7i.2xlarge` Spot hosts in the deployed runner account; their S3 outputs were checked and both EC2 instances terminated. They did not fetch original D18 input, build the worker, or test Spot endurance. The runner's exact original-object read grant remains unapplied and its direct read was previously `403`.
- **Hosted source-build gate:** A [build-only `c7i.2xlarge` Spot job](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/linux-build-smoke-20261026/REPORT.md) allocated and terminated its EC2 host, but the private GitHub fetch of frozen `45ce91d5` failed for lack of noninteractive credentials before Cargo. The prior approved source archive matches its checksum but freezes the older `54cbef1a` commit and has no portable Git metadata. The same `--ref` job cannot be retried unchanged; exact source delivery and the separate original-D18 role read remain open access gates.
- **No-source alternative and exact package:** A second Spot job with `--no-source --toolchain rust` executed Rust and AWS CLI version checks with exit 0 and verified EC2 termination, without fetching Git. A fresh extraction of the [local exact-source package](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/linux-build-smoke-20261026/SOURCE_DELIVERY_PROPOSAL.md) passed 20 worker and 5 patched-verifier release example tests offline at clean `45ce91d5`. Its 58,011,243-byte archive is not uploaded, and the reviewed three-object cross-account read is unapplied. This resolves the source packaging method locally while hosted source custody and D18 input access remain open.
- **Derived or proved:** None of the capped searches proves relation absence or a cost ordering.
- **Suspected explanation:** The native CDCL backend may be a material solver-stage bottleneck; an XOR-aware backend is a discriminating test, not yet an end-to-end improvement claim.
- **Missing capability:** A verified natural partial producer across enough independent source equations, graph closure, useful rank and log solving for the full-width subgroup, followed by target descent and matched one-target rho with full cost accounting.
- **Access or resource limit:** The frozen full-D18 positive/natural pair needs a host that passes its load, swap and scratch preflight, receives exact clean `45ce91d5` source, and can read the two exact original S3 objects. The user authorized the exact-object IAM change for a separate read-only user and it was applied; the deployed Spot runner's cross-account read remains unauthorized. Spot admission, Rust setup and cleanup are verified, while the actual source fetch failed before Cargo. The [Linux Spot protocol](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/LINUX_SPOT_PROTOCOL.md) is frozen, while the per-job source/binary/host launch lock and original-object GET remain open. The existing checkouts and older source archive are not substitutes for those gates. The temporary user grant must be removed and its absence checked after the hosted attempt or its cancellation.
- **What this evidence does not establish:** A selected best base, relation rate, a complete index-calculus solve, or an end-to-end speedup.

## What would unblock it

- [ ] Independently replay natural partials against the exact S3 base/pool bytes, public target stream, original CNF/XOR/field equations, and original curve group.
- [ ] Collect enough checked rows to measure useful rank and solve full-width column logs; record failed attempts and charged setup cost.
- [ ] Complete and verify a previously unseen public target with all five online phases and run a same-target rho reference under matching resource envelopes.
- [ ] Apply the frozen finite-grid coverage and unsupported-combination dispositions, and compare total cost with uncertainty before filling `selected_best_total_runtime`.

These checks are jointly needed to select a factor base on the requested
criterion. A checked partial relation would resolve only the first gate.

## Next discriminating step

- **Primary `a=0` action:** Freeze one new solver or encoding policy on the exact D18 input. Require a complete positive model and original-XCNF plus group replay before any public natural query. Keep all first-pattern and native cadence-32 caps as separate candidate cells.
- **Inputs and controls:** Bind the exact compressed S3 object and provider checksums, pilot public corpus, target point, positive model, formula bytes, solver binary, source commit, and resource caps before timing.
- **Execution limits:** Retain `INDETERMINATE`, timeout, OOM, and producer failure as terminal rows without assigning zero cost or natural yield. A checked natural relation advances to multi-query yield and rank measurement.
- **Cross-curve action:** The separately identified `a=1` 53-bit-subgroup panel needs its own admissible route, same-point IC/rho replay, and total-cost selection. Do not pool its data with this primary `a=0` subgroup.
- **Recheck trigger:** A changed solver or encoding with a positive pass, a checked natural relation, a completed matched panel, or a qualified cloud route for the `a=1` comparison.

## Resolution and history

Unresolved. The 2026-10-25 XOR bridge and local pilot are a solver-backend
probe. The 2026-10-26 native D18 input preflight passed while the capacity
gate failed before worker launch. Merging this record does not complete the
selection criterion.

### 2026-10-26 full-D18 solver-gate update

The [ordered full-D18 run](../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261025T2339Z-ordered-d18/REPORT.md) used the exact frozen 32,643-column, 5,418,738-point generation. Its constructed positive returned independently checked rank-one rows `[0,166,332,498]`; its single natural public target reached the internal 2,400-second wall cap after checked import/model construction and yielded no terminal solver result. The independent audit status is `PASS_CENSORED_NATURAL`. This is a resource-bounded observation, not evidence of a zero relation rate.

The separate [native XOR cadence-32 attempt](../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261026T0121Z-cadence32/REPORT.md) ended at its 300-second internal cap before `import_checked` on a saturated shared host. The natural arm was skipped. Its independent audit status is `CAPACITY_CENSORED_BEFORE_MODEL`; it does not compare solver cadences. The native kernel's 120-case exhaustive mixed CNF/XOR differential check covers cadences 1, 2, 4 and 32, while a full-D18 constructed control under cadence 32 is still open. The [combined result](../research/koblitz_n83_factor_base_sweep_20261008/reports/d18-solver-gates-20261026/RESULT.md) preserves these separate gates.

At the earlier 2026-10-26 checkpoint, an inspected AWS execution principal
lacked S3 read access to the exact full-D18 generation. The exact-object IAM
proposal was then unapplied; automatic approval review rejected that persistent
grant pending explicit authorization. A separately approved source-archive
upload did not authorize the grant or a model-artifact upload. This was an
access condition, not a mathematical or solver result.

### 2026-10-26 exact-read authorization and capacity continuation

After the rejection above, the user explicitly authorized the prepared temporary
inline policy `N83D18ExactInputRead20261025` for IAM user
`rho-gh-snapshot-readonly-20261024`. A live `aws iam get-user-policy` readback
confirmed that its sole statement allows `s3:GetObject` for exactly the D18
`manifest.json` and `objects/c295da88…jsonl.gz` keys under the frozen
`d18-full-direct-subgroup-v1/a0` prefix. It does not alter the factor-base
object or establish that a worker ran. The current [PR handoff](https://github.com/isogeny-labs/crypto/pull/2649)
and [timestamped IAM readback](evidence/n83-d18-exact-read-20261026.json)
record the applied state and cleanup obligation. The readback policy document
matches the reviewed proposal after canonical JSON sorting. The latest applicable
GitHub Actions annotation says the job was not started because recent account
payments failed or the spending limit needs an increase. This is a hosted
admission failure before source checkout, object GET, or solver execution.

The source-bound second local preflight used commit `45ce91d5`, release
worker SHA-256 `11220d8b838bf5cac2985dbcba4ea96e69a64b70dc54044d180cf6a3e8970e49`,
and verifier SHA-256 `e89578703888568d67f79e5b2645e47b911a30a9d7a8a3d57e795be997296b71`.
Fresh provider GETs and local SHA-256 checks matched the 160,747,529-byte
full-D18 object and custody wrapper. Six selected one-minute load samples
were 26.44–66.14, all above 24; cumulative swapouts rose from 851,611 to
1,242,059, a delta of 390,448; scratch availability was 75 GiB. The
constructed worker was not launched. The [write-once attempt-2 report](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/attempt-2/REPORT.md)
has the Markdown, SVG, PNG, inspected PDF, host samples, and provider GET
metadata beside it. This preflight is a capacity result, not a search result.

A read-only check of the repository's `ephemeral` runner returned
`ParameterNotFound` for `/ephemeral/config` under the default AWS identity
in both `us-east-1` and `us-west-2`. A subsequent check with the configured
`ecc-challenges-new` profile reached the deployed runner in account
`104186811093`; direct access to the original source bucket returned `403`.
The third preflight above preserves the raw access receipts and exact
cross-account policy proposal. The runner defaults to Spot capacity; this PR
adds an explicit On-Demand request path. The frozen native solver has no
application checkpoint, and its wall time would be invalid after an
interruption. The On-Demand path passed local fake-AWS market checks and
standalone Rust CLI unit tests. A live market smoke was rejected by the
account's explicit Spot-only launch policy before instance allocation; an
authorized non-Spot launch path remains open. The
new main merge also introduced a parser
failure in `ecbench/methods.rs`; PR #2649's N83 delimiter repair is pushed,
but full post-merge Cargo validation remains open. These backend and source
conditions do not change the frozen positive-then-natural stop rule.

**2026-10-26 hosted exit criterion (historical route):** either the account owner authorizes an exact
non-Spot launch exception or supplies another non-Spot runner, then a
self-terminating no-source smoke must show an allocated instance, uploaded
output and termination. The deployed runner also needs the separate exact
two-object cross-account S3 read grant, with fresh full-object GET receipts.
The attempted smoke's explicit deny is an access policy, not a solver or
capacity observation; do not retry the same launch parameters unchanged.

**2026-10-26 next action (superseded for the local D18 probes):** obtain a resource-qualified execution host, verify fresh
full-object S3 GETs and the exact frozen source/binary identities there, run
one constructed positive at its 300-second internal cap, and launch at most
one 2,400-second public natural query only after separate group and retained
assignment replay passes. Retain every censor or infrastructure failure.
When the hosted attempt ends or is cancelled, remove the temporary inline
read policy and verify `NoSuchEntity`; then continue the remaining yield,
rank/log, unseen-target, matched-rho and finite-grid gates before selection.

**Next discriminating step:** preserve the completed D18 runs as separate
candidate cells. The ordered XCNF assignment replay is complete, while both
free-choice and ordered CryptoMiniSat positive controls are capped under the
declared one-million-conflict budget. Freeze one changed solver or encoding
policy with its exact formula, source and resources, and require a complete
positive model plus original-XCNF and group replay before a public natural
arm. Retain every censored or failed arm. A checked natural relation advances
only the first gate: independent multi-equation yield, closure, rank/logs,
unseen-target descent, matched rho and finite-grid coverage remain required
for total-runtime selection on both N83 curves.
