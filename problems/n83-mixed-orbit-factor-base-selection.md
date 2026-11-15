# N83 mixed-orbit relation yield and total-runtime selection

- **Status:** `investigating`
- **Kind:** empirical bottleneck and implementation gap
- **Affected work:** [N83 factor-base sweep](../research/koblitz_n83_factor_base_sweep_20261008/), [K2048 native XCNF gate](../research/koblitz_n83_factor_base_sweep_20261008/verification/k2048-m5-xcnf-20261114/RESULT.md), [full-D18 native continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/PROTOCOL.md), [separate a=1 cross-family panel](../research/n83_crossfamily_current_20261025/native/README.md)
- **Next action owner:** N83 factor-base research task
- **Last updated:** 2026-11-15 UTC; the exact rank-query census, native K600 model verifier, v3 base producer, bounded v3 trial bridge, and full K600 construction/S3 artifact custody control plus native read-only S3 adoption are in open [PR #4295](https://github.com/isogeny-labs/crypto/pull/4295). The same-public-point D18/K2048 solver pair and independent raw audit merged in [PR #4131](https://github.com/isogeny-labs/crypto/pull/4131). The earlier K2048 native [PR #4126](https://github.com/isogeny-labs/crypto/pull/4126) and [D18 ordered solver result](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/ordered-cms-20261114/RESULT.md) remain separate cells.

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

- **2026-11-15 exact screen and verifier, open PR [pcc:unverified]:** The [native rank-query census](../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261115T0854Z/REPORT.md) covers 78 retained objects and 390 `m=2..6` rows with exact combinatorial necessary counts. It reproduces all 30 frozen historical cases but measures no natural relation yield or cold runtime. The [K600 native replay amendment](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-full-smooth-m6-20261114/NATIVE_REPLAY_AMENDMENT_20261115.md) adds original-XCNF, point-lift, subgroup, and relation-row verification; the touched release example suite passed 25/25, including a constructed six-point witness and tamper rejection. No full K600 solver model has passed that verifier. The frozen K600 m6 search still needs a source-locked Linux ARM worker image, exact inputs, and a new budget allowance. Native v3 factor-base publication to `s3://setec-astronomy-artifacts/store` still needs the owner's writer/destination choice. These are execution and access gaps, not a negative relation-yield result.

- **2026-11-15 storage access update:** A [fresh read-only check](../research/koblitz_n83_factor_base_sweep_20261008/STORAGE_TRANSITION_20261027.md#read-only-access-update--2026-11-15-utc) used `ecc-challenges-new` to GET and hash a 14,861,500-byte known store object from `s3://setec-astronomy-artifacts/store`; the SHA-256 matched its prior publication receipt. IAM simulation allows object GET and PUT on a placeholder store key, but no new N83 write/fresh-pull has been attempted. Owner confirmation of the exact new-bucket writer and historical-mirror policy remains pending. The old-bucket D18 input keys stay frozen.

- **2026-11-15 full K600 construction and S3 artifact custody, open PR [pcc:unverified]:** The [precommitted full-size control](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-local-custody-20261115/RESULT.md) constructed 600 signed-Frobenius columns and independently imported all 99,600 points under a temporary file store. Its 3,025,798-byte gzip SHA-256 `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf` exactly matches the historical K600 object. The PR artifact bot then archived that object to `s3://setec-astronomy-artifacts/store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`; the [versioned addendum](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-local-custody-20261115/S3_CUSTODY_ADDENDUM_20261115.md) records a fresh S3 GET with matching SHA-256 and gzip integrity. This resolves archive custody for this exact object, while native v3 S3 producer/import acceptance and the future writer choice remain open. The construction's one-sample L0 timings do not price natural relations or complete IC runtime. Docker Desktop now exposes Linux arm64 and cgroup v2 but has zero local images; the frozen m6 search has no source-locked worker image or new charged-time allowance.

- **2026-11-15 successor import gate:** The [v3 native store reader](../research/koblitz_n83_factor_base_sweep_20261008/STORAGE_TRANSITION_20261027.md#native-successor-reader--2026-11-15-utc) checks an indexed content-addressed object, exact compressed and plain hashes, bound replay/fresh-download receipts, and every primary point and orbit label. Its local `file://` control and rejection tests pass. This was a reader-only checkpoint; later implementation status follows.

- **2026-11-15 successor producer and trial bridge, open PR [pcc:unverified]:** The [native v3 producer](../research/koblitz_n83_factor_base_sweep_20261008/STORAGE_TRANSITION_20261027.md#native-successor-producer--2026-11-15-utc) constructs a primary public-X base, replays every point and orbit with the generic curve implementation, publishes through the content-addressed store, and verifies an independent fresh pull. Its two-orbit, 332-point local `file://` control produced a 12,601-byte gzip object with SHA-256 `3805cf6e445967f1f867352368acddfd0e484318860bd94a8d3c0e63d86e6154`; the touched release example passed 26/26. The [v3 trial bridge](../research/koblitz_n83_factor_base_sweep_20261008/STORAGE_TRANSITION_20261027.md#native-successor-trial-bridge--2026-11-15-utc) imports that checked base into the bounded native enumerate path for `m=2..6`. Its zero-trial local control ended `PREFLIGHT_ONLY` with zero queries, relations and rank; total runtime stayed null. The control does not measure natural yield. A native v3 S3 publish/fresh-import receipt, larger-base solver resource behavior, SAT/WDSat/FES combinations, useful rank, target descent, and fully charged same-target selection remain open.

- **2026-11-14 same-point D18/K2048 pair [pcc:unverified]:** The
  [frozen result](../research/koblitz_n83_factor_base_sweep_20261008/verification/a0-matched-public-solver-20261114/RESULT.md)
  binds both original XCNFs to sealed public fixture 1 on the primary `a=0`
  subgroup. The D18 ordered m4 arm returned `INDETERMINATE` at 1,000,001
  conflicts after 427.082 seconds; the K2048 m5 wide-XOR arm returned
  `INDETERMINATE` at 1,000,002 conflicts after 156.853 seconds. Both
  original solver transcripts, commands, host samples, and terminal receipts
  passed the separate raw audit. The arms produced no checked ordinary row
  at their caps, so natural yield, novel rank, and complete runtime remain
  unmeasured. These local solver-stage walls had different background loads
  and do not rank the complete candidates.

- **2026-11-14 K2048 m5 native gate [pcc:unverified]:** The retained
  339,968-point, 2,048-column `a=0` base has a source-bound five-summand
  positive XCNF that returned SAT in 839 conflicts and passed independent
  original CNF/XOR, archive, subgroup, group-sum and relation-row replay.
  One fixed public natural query on the same formula returned
  `INDETERMINATE` at 1,000,001 conflicts after 137.753 seconds of local L0
  process wall. The [merged report](../research/koblitz_n83_factor_base_sweep_20261008/verification/k2048-m5-xcnf-20261114/RESULT.md)
  preserves the native receipts and earlier Python-orchestrated provenance.
  The separately frozen `--maxmatrixcols 25000` [follow-up](../research/koblitz_n83_factor_base_sweep_20261008/verification/k2048-m5-wide-xcnf-20261114/RESULT.md)
  retained a first `HOST_REJECTED_LOAD` preflight at loads 82.0376, 76.6616,
  74.8535 with stable swapouts. The second locked positive returned SAT at
  839 conflicts and passed independent original-XCNF, group, archive, and
  relation-row replay. The locked public natural arm admitted three
  332-by-20,916 Gaussian matrices, then returned `INDETERMINATE` at the
  1,000,001-conflict cap after 151.8622 seconds. It produced no model or
  verified natural relation on that query; all raw transcripts are retained.

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
- **2026-11-15 native read-only S3 adoption, open PR [pcc:unverified]:** The [frozen K600 adoption run](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-s3-adoption-20261115/RUN_REPORT.md) used the archived 3,025,798-byte object under `s3://setec-astronomy-artifacts/store`. Two separate empty-cache native `Store::ensure` runs matched its pinned SHA-256/length and produced S3-bound v3 import `PASS` receipts, each with generic replay of all 99,600 points and 600 orbit representatives. The second run captured process exit 0; the first application's completed receipts survived a post-run zsh wrapper exit-capture error. Both receipts say `uploaded=false`. This closes the native v3 **read-only import** gate for these exact bytes, but does not test `Store::push`, relation yield, rank or total IC cost.
- **Derived or proved:** None of the capped searches proves relation absence or a cost ordering.
- **Suspected explanation:** The native CDCL backend may be a material solver-stage bottleneck; an XOR-aware backend is a discriminating test, not yet an end-to-end improvement claim.
- **Missing capability:** A verified natural partial producer across enough independent source equations, graph closure, useful rank and log solving for the full-width subgroup, followed by target descent and matched one-target rho with full cost accounting.
- **Access or resource limit:** The frozen full-D18 positive/natural pair needs a host that passes its load, swap and scratch preflight, receives exact clean `45ce91d5` source, and can read the two exact original S3 objects. The user authorized the exact-object IAM change for a separate read-only user and it was applied; the deployed Spot runner's cross-account read remains unauthorized. Spot admission, Rust setup and cleanup are verified, while the actual source fetch failed before Cargo. The [Linux Spot protocol](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/LINUX_SPOT_PROTOCOL.md) is frozen, while the per-job source/binary/host launch lock and original-object GET remain open. The existing checkouts and older source archive are not substitutes for those gates. The temporary user grant must be removed and its absence checked after the hosted attempt or its cancellation.
- **2026-11-15 implementation gate for the distinct K600 m6 pair:** The frozen protocol's outer `chain_search_supervisor.py` performs material source/input locking, Docker cap and cleanup enforcement, and receipt auditing. It cannot be the execution path for new research under the current no-Python rule. The [native-supervisor amendment](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-full-smooth-m6-20261114/NATIVE_SUPERVISOR_AMENDMENT_20261115.md) lists the exact replacement and control obligations. A [native zero-trial admission component](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-supervisor-20261115/IMPLEMENTATION_STATUS.md) now implements commit-bound preparation, fresh S3 input fetch, hard Docker-cap arguments, timeout/cleanup handling and receipt validation; its release tests passed 6/6, including a committed-source tamper rejection before Docker launch. The v3 worker was corrected to report real cgroup peak for a source-bound zero-trial run. A separate [native arm-input staging component](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/IMPLEMENTATION_STATUS.md) prepares the exact legacy panel layout from a read-only fresh astronomy-store GET; metadata tests passed 2/2 and the touched exporter suite 27/27. Neither component has performed its actual K600 Linux/S3 execution path. No Linux image or hard-cgroup preflight was available; the full native positive/natural outer path and distinct proposed 3,600-second allowance remain open. No relation-yield conclusion follows.
- **2026-11-15 Linux ARM64 build and current-head validation, open PR [pcc:unverified]:** After a source-bound build failed on three tracked files omitted by the sparse checkout, the [write-once retry](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/LINUX_ARM64_BUILD_SMOKE_ATTEMPT4.md) compiled all five staged examples as executable ARM64 ELF files under the pinned Rust 1.99.1 build image, with Docker exit 0 and `OOMKilled=false`. This reused a partial Cargo target and did not build the solver or runtime image; it cannot price cold setup. The [focused preflight repair](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/CLIPPY_CI_VALIDATION_20261115.md) passed 6/6 release example tests. The current-head [full-library attempt](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/ROOT_LIB_RESOURCE_FAILURE_20261115.md) stopped before tests when LLVM linking exhausted the macOS boot volume; its status is a resource failure, not a test pass. A fresh fully charged Linux build with the sixth solver output, runtime image, committed build plan, and verified zero-trial hard-cgroup preflight is still required before any constructed/natural arm. At this checkpoint the local boot and SSD volumes had about 1.3 and 2.5 GiB free; do not infer that a further cold build is admitted without a new capacity check.
- **What this evidence does not establish:** A selected best base, relation rate, a complete index-calculus solve, or an end-to-end speedup.

- **2026-11-15 later current-head library receipt, open PR [pcc:unverified]:** A second release-library attempt compiled and started tests on clean `8e7a65eeaa71`, but its [preserved partial log and report](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/ROOT_LIB_INCOMPLETE_20261115.md) have no full-suite summary or child exit receipt. The wrapper used zsh's read-only `status` variable. Its 39 observed `FAILED` lines include one checkpoint test beyond the earlier 59-name failure set; failure details were not captured. Boot storage reached 115 MiB free at inspection, without a proved termination cause. The library gate remains open. A corrected-wrapper isolated checkpoint replay is running; a complete rerun requires stable free storage. Docker was unresponsive at this checkpoint, so the frozen charged build did not begin.

## What would unblock it

- [x] Archive the exact primary K600 object to the repository-configured S3 artifact store and verify a fresh GET by SHA-256 and gzip integrity; [artifact-bot receipt and readback](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-local-custody-20261115/S3_CUSTODY_ADDENDUM_20261115.md). This is artifact custody, not native v3 producer acceptance.
- [x] Adopt the exact bot-archived K600 bytes through the native v3 reader with fresh S3 GET, pinned manifest/pointer/index and complete generic curve replay; [read-only adoption report](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-s3-adoption-20261115/RUN_REPORT.md). This does not establish native writer acceptance.
- [ ] Finish the native admission/runner path with a source-bound, fully charged Linux ARM64 build of all six required outputs, solver binary and labelled runtime image from a committed plan; then lock the exact source and staged inputs and pass one zero-trial hard-cgroup preflight before the constructed positive. The warm five-example build is only a feasibility control. Preserve the original caps, arm order and terminal statuses. The distinct 3,600-second proposed execution budget remains a separate gate.
- [ ] Confirm the exact authorized new S3 bucket/prefix and writer identity for future native publications; exercise native v3 `Store::push`, independent fresh GET/import and logical store index/manifest with write-once receipts. Keep the frozen old-bucket D18 keys unchanged unless an explicit mirror decision is recorded.
- [ ] Independently replay natural partials against the exact S3 base/pool bytes, public target stream, original CNF/XOR/field equations, and original curve group.
- [ ] Collect enough checked rows to measure useful rank and solve full-width column logs; record failed attempts and charged setup cost.
- [ ] Complete and verify a previously unseen public target with all five online phases and run a same-target rho reference under matching resource envelopes.
- [ ] Execute or explicitly disposition the frozen finite grid, including the SAT/WDSat/FES, splitting, symmetry, Gray and double-large-prime combinations, and compare fully charged total cost with uncertainty before filling `selected_best_total_runtime`.

These checks are jointly needed to select a factor base on the requested
criterion. A checked partial relation would resolve only the first gate.

## Next discriminating step

- **Primary `a=0` action:** The second K2048 wide-XOR positive and its public natural query are complete, as is the same-point D18/K2048 pair. For the [frozen K600 m6 pair](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-full-smooth-m6-20261114/PROTOCOL.md), first free or provision enough isolated build storage, freeze a committed three-step Linux ARM64 build plan, produce all six ELF outputs including the solver and a source-labelled runtime image with cold build time charged, and pass the zero-trial cgroup/input preflight. Then stage exact inputs under the distinct proposed 3,600-second ceiling and run the constructed positive before at most one natural public query. An invalid, capped, or unverified positive stops the natural arm. If it passes, independently check any natural row and then measure common-query useful yield and novel rank across retained bases. Keep K600, K2048, and full-D18 encodings, arities, bases, and capped outcomes as separate candidate cells.
- **Inputs and controls:** Bind the exact compressed S3 object and provider checksums, pilot public corpus, target point, positive model, formula bytes, solver binary, source commit, and resource caps before timing.
- **Execution limits:** Retain `INDETERMINATE`, timeout, OOM, and producer failure as terminal rows without assigning zero cost or natural yield. A checked natural relation advances to multi-query yield and rank measurement.
- **Cross-curve action:** The distinct `a=1` 53-bit-subgroup [current-source panel](../research/n83_crossfamily_current_20261025/native/README.md) freezes K3300, K3450, K3500 and K3596 on 16 unseen public points with paired same-point IC/rho and separate online/cold selectors. Its owner-approved `us-east-1a` two-instance Spot route with security group `sg-030ffadf6a62d2ce2` remains price-gated: the latest 2026-11-14 read-only quote was $0.396900/hour against the $0.27 bid, though placement scored 8 for two units. No allocation has occurred. Keep its results separate from the primary `a=0` subgroup.
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
