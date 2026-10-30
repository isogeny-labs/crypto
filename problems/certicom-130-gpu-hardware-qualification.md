# Physical CUDA and Metal qualification for the two Certicom 130 aliases

- **Status:** `open`
- **Kind:** hardware access limit; device correctness unverified
- **Affected work:** [crypto #2250](https://github.com/isogeny-labs/crypto/pull/2250), the merged [cryptanalysis follow-up #1138](https://github.com/isogeny-labs/cryptanalysis/pull/1138), the historical merged [cryptanalysis #968](https://github.com/isogeny-labs/cryptanalysis/pull/968), and [the shared native workers](../gpu/certicom/README.md)
- **Next action owner:** the PR author for hosted CI; the owner for existing physical GPU access
- **Last updated:** 2026-10-26

## Problem

Physical execution of rho, BSGS and index calculus on both NVIDIA CUDA and
Apple Metal cannot yet be fully qualified. This workspace has neither device
nor an authorized remote GPU connection. Hosted macOS CI now supplies partial
native Metal execution evidence, described below, but not complete CLI or
physical GPU provenance receipts.

The exact instances are the published ECC2-131 and ECCp-131 fields, models,
generators, subgroup orders, cofactors and targets, exposed under the requested
ECC2-130/ECCp-130 aliases in both repositories. ECC2K-130 remains distinct.
CPU support, native GPU code and compilation checks do not discharge the
requested physical-device validation. The separate [nine-binary-curve rho
campaign problem](certicom-binary-gpu-rho-integration.md) has broader curve,
durable-store and performance requirements; it is not resolved by this work.

## Evidence and uncertainty

- **Observed:** at published heads `8ea1c141c6f66e41e78abb50b85de7ff8f100caf`
  and `6552da5ed5f3715bfe5c3b9c8a9fa23f1e1356fe`, the full CPU library suites
  passed 4,420 and 3,150 tests with loopback enabled. The original restricted
  runs' nine/eight localhost failures and the passing reruns remain disclosed
  in the PRs. These are earlier-head results, not qualification of later fixes.
- **Observed:** complete CUDA workers compiled locally with nvcc 13.3.73 for
  sm_80, sm_90 and sm_120. A native device probe exited 3 with an insufficient
  driver error. Explicit CUDA CLI selection returned an error, no scalar and
  no CPU fallback. This workspace has no NVIDIA device or Metal runtime.
- **Observed:** macOS 14 arm64 compiled the Objective-C++ worker, Metal 3.0
  shader and metallib in [crypto CI](https://github.com/aburan28/crypto/actions/runs/39197392278/job/117882993610)
  and [cryptanalysis CI](https://github.com/aburan28/cryptanalysis/actions/runs/39197404995/job/117883031024).
  Those jobs compile only. The optional hardware job was legitimately skipped.
- **Observed access limit:** GitHub self-hosted runner enumeration returned
  HTTP 403. The task runtime reports no GPU capability, secret binding or
  outbound identity. The suite's `cloud/fleet.json` describes `rp-gpu-1`,
  but a configuration name does not establish a live authorized connection.
- **Observed on 2026-10-21:** [crypto's hosted Metal job](https://github.com/aburan28/crypto/actions/runs/39518361538/job/118955884773)
  compiled the worker/shader, opened `Apple Paravirtual device`, and passed all
  six device library tests: independent full-width arithmetic, all three solver
  fixtures on both exact curves, bounded public targets and worker/deadline
  controls. The run tested PR head `917e2214b132e76c509575095269652c66ff907c`
  via GitHub's merge revision `8376e7898de541a82647d3978bda1a9ea0107309`.
  Rust's release library-test build took 20m45s. The 25-minute job was cancelled
  during the subsequent release CLI build, so no CLI pass is claimed. Its
  [retained logs](https://github.com/aburan28/crypto/actions/runs/39518361538/artifacts/12312995140)
  include the probe, device/toolchain identity, native hashes and six passing
  library tests; the [complete decoded job log is archived here](evidence/certicom-metal-20261021/README.md)
  with SHA-256 hashes, including the cancellation. This is execution through
  the native Metal API, distinct from
  host emulation; the exposed paravirtual identity does not establish the
  physical GPU behind it.
- **Derived or proved:** no mathematical obstruction is established.
- **Suspected explanation:** no device correctness failure has been observed;
  actual device behavior remains unknown until execution is possible.
- **What this evidence does not establish:** emulation and compilation do not
  establish physical GPU use, throughput, natural IC relation yield, or a
  solution to either public challenge. Constructed rho `Q=-G` and prepared
  full-rank IC inputs are correctness controls.

## What would unblock it

| Requested component in both repositories | Current evidence |
| --- | --- |
| CPU rho, BSGS and IC on both exact curves | Verified constructed correctness controls and bounded public-Q runs. The crypto base integration passes the complete release library (4,878 passed, 0 failed, 121 existing ignored), all 14 focused solver/device-policy checks, and real CPU/emulator CLI checks. Suite results remain tied to its recorded merged source. |
| NVIDIA CUDA rho, BSGS and IC on both exact curves | Implemented; native compilation and device-source host emulation pass. Physical execution remains unverified. |
| Apple Metal rho, BSGS and IC on both exact curves | Six native library checks passed through hosted `Apple Paravirtual device`; CLI qualification timed out during compilation. Physical GPU provenance and complete receipts in both repositories remain open. |

- [ ] Attach existing authorized NVIDIA and Metal-capable Mac connections,
  or usable GitHub runner labels, with the native compiler/runtime available.
- [ ] On both backends and in both repositories, run the checked-in device
  tests and release CLI tests on both exact curves and all three methods.
  Retain source/binary hashes, OS/architecture, compiler/runtime and device
  identity, explicit backend reports, launch counts and complete pass/failure
  logs. Every returned scalar and IC relation/log must pass independent CPU
  replay. Include bounded incomplete runs on both published targets.
- [ ] Resolve any device defects without fallback or curve substitution,
  preserve shared-source parity, and satisfy current-head CI/review gates.

Passing these checks establishes device correctness for the tested workloads
and hardware. It does not establish a speedup or practical public-target solve.

## Next discriminating step

- **Action:** build/probe the selected native worker and run the hardware
  job in `.github/workflows/certicom-devices.yml` on an existing authorized
  runner. It executes `bash gpu/certicom/qualify.sh`, including device and
  release CLI tests. The repair separates hosted library and CLI phases
  into independent 45-minute jobs. Both phases must pass; a partial receipt
  cannot qualify the whole backend. The user explicitly approved the required
  cryptanalysis follow-up on 2026-10-25, resolving the earlier publication-scope
  rejection. Follow-up #1138 merged on 2026-10-26 as
  `ac9146a587620c0b86196349b5999996452f75ad`; its branch and the earlier
  merged #968 remain untouched and outside PR monitoring. Inspect device
  identity and retain physical provenance separately from API execution.
  Neither merge closes this record. The remaining crypto PR must stay open
  until qualification and current-head CI/review gates pass, with auto-merge
  disabled under the user's explicit hold.
- **Inputs and controls:** the exact published instances; full-width random
  arithmetic against independent Rust; constructed known answers; public Q
  with small search budgets; worker identity, malformed-input and timeout
  controls. `gpu-emulator` stays explicitly labelled host execution.
- **Execution limits:** checked-in per-walk/query/table/matrix budgets and
  whole-session deadlines; serial hardware tests; 45 minutes per hosted phase
  and 45 minutes for a manually dispatched complete hardware job.
  Stop on any mismatch, worker error or deadline. No benchmark campaign,
  paid provisioning or public-challenge solve is authorized by this record.
- **Outcomes:** success supplies the missing physical receipts; a mismatch
  requires a bounded repair and repeat validation; missing hardware or a
  launch error remains infrastructure evidence. Bounded no-solve results do
  not prove impossibility.
- **Recheck trigger:** an attached existing GPU connection or runner label,
  a physical execution receipt, or new current-head CI/review feedback.

## Resolution and history

- 2026-10-26: observed follow-up #1138 merged at 05:22:45 UTC from head
  `87df3734cc1345548fdcb174a97a541b4c36cd4c`, merge commit
  `ac9146a587620c0b86196349b5999996452f75ad`. Monitoring and pushes to that
  deliverable stopped immediately. The merge supplies no new physical GPU
  or complete Metal CLI evidence. Auto-merge on crypto #2250 was subsequently
  re-enabled outside this task at 10:53:26 and 14:48:26 UTC; this task disabled
  it at each permitted cadence and preserved the qualification hold. Those
  API events do not identify the initiating process.

- 2026-10-26: GitHub now resolves both repositories under `isogeny-labs`;
  the preceding 404 access observation is resolved. At cryptanalysis main
  `379faa0f1e38facd9c8fdfc527f0b89518cf2e70`, the Certicom modules, native
  workers and workflow previously merged in #968 were absent. Follow-up
  [#1138](https://github.com/isogeny-labs/cryptanalysis/pull/1138), head
  `87df3734cc1345548fdcb174a97a541b4c36cd4c`, restores those task files and
  their CLI/catalogue integration while preserving unrelated main changes.
  Its full release library passes 3,042 tests (0 failed, 90 existing ignored),
  including all 14 solver/device-policy tests. The real `ca-icx` and `crax icx`
  pass all seven checked-in CLI cases on CPU and explicit host emulation:
  28 executions. Initial sparse-checkout missing-header/corpus failures and
  their unchanged passing controls are retained in the PR history. Current
  main has a different overall library test count from the earlier df69 tree.
  Three restored C/CUDA files now satisfy the current formatting gate;
  comment/whitespace-normalized tokens are identical to the preceding worker
  source. All 21 shared files match the prepared crypto repair. No new CUDA
  device execution, Metal CLI result or physical GPU provenance is claimed;
  earlier native device observations remain tied to their earlier revisions.

- 2026-10-19: recorded the cross-repository physical qualification gap and
  successful compilation controls. Added the CLI checks to the hardware job;
  scheduling that job is not a completed hardware experiment.
- 2026-10-19: reproduced the session-deadline defect with both a native-source
  worker and the actual release CLI: expired sessions could return verified
  scalar zero. The monotonic-deadline repair passes both regressions, all 14
  focused solver/device tests per repository, and the complete release library
  suites (4,421/0/118 existing ignored; 3,151/0/92 existing ignored). Seven CLI
  cases pass through each of `icx`, `ca-icx` and `crax icx` on CPU and explicit
  host emulation: 42 test executions. These remain host correctness evidence.
- 2026-10-21: cryptanalysis #968 was observed merged at `9b5cf83d4ac58bddb19fefb50bf59388c4f98713`;
  merging did not close this qualification requirement. Preserve its branch.
  Prepared a shared hosted Metal workflow split into library/CLI phases with
  a combined failing gate and separate artifacts. Default manual qualification
  still runs both phases. The cryptanalysis candidate is local commit
  `0f42395555661a7878066f1f1f7bcbb9aed2f6d5`; its full release library suite
  passed 3,151 tests (0 failed, 92 existing ignored), and its seven CLI cases
  passed through both real entrypoints on CPU and host emulation (28 executions).
  Nine launcher routing/error controls use stubs and establish no GPU evidence.
  Source parity, Bash syntax and actionlint passed. Automatic approval review
  rejected the candidate branch's publication pending direct approval for a new
  follow-up PR; no alternate publication route was attempted. The merged
  deliverable is not amended or overwritten.
- 2026-10-21: on the follow-up Linux working trees based on crypto
  `917e2214b132e76c509575095269652c66ff907c` and cryptanalysis
  `df69c0c`, complete release library suites passed **4,480/0/119 ignored**
  and **3,151/0/92 ignored**. All 14 focused solver/device checks passed in
  each suite. Seven checked-in release CLI cases passed through `icx`, `ca-icx`
  and the actual `crax icx` on CPU and explicit host emulation: **42 executions**.
  Rust/native arithmetic and CLI source are unchanged by this follow-up.
  These results establish host correctness; they do not close physical gates.

## Other current-head CI observations

### Base integration control on 2026-10-26

The normal base merge into #2250 uses main
`e8fcadf62cd0b674c40ebc4e76dfe88e26e71f5c`. Its generated ledger conflicts
were implementation hashes, not assessment rows. Regeneration changes nine
source SHA-256 fields compared with that main tree; every other JSON value
remains unchanged. Two pins bind this task's curve aliases and module exports;
the other seven source files are byte-identical to incoming main. All 51
materialized generator inputs match the staged Git blobs. Incoming research,
benchmark and paper trees remain identical to main, and all 21 shared Certicom
files still match the merged cryptanalysis source.

The independent verifier fails on the merged sources with six problems:
registry row count, report registry digest, report model-hash membership,
graph model-hash membership, the registry's declared catalogue-commit binding,
and the PDF's embedded v2 coverage marker. An isolated checkout of the exact
main revision, with its own unmodified ledger and exact tracked inputs,
reproduces all six and eight additional stale source-artifact digests
(14 problems total). The verifier and frozen input trees are unchanged from
that base. No catalogue row, model membership, report, graph, PDF or validation
gate was changed to clear these failures. The generator's byte-exact freshness
check passes after regeneration, while this independent semantic gate remains
failed. This is an inherited gate obstruction, not a verifier pass or a device
qualification result.

The current merged-source checks pass all 14 focused solver/device-policy
tests, seven real release `icx` CLI cases on CPU and the same seven through
explicit host emulation, and all seven real release `ecdlp-nist` integration
cases. These are host correctness observations. The complete clean-input
release-library rerun against code commit
`c3f3c1a2b4314e42c728c517c4b9e4dda176fcea` ended at
2026-10-26T18:58:03.931Z with 4,878 passed, 0 failed and 121 existing ignored
(all 4,999 registered tests accounted for; none filtered out). The earlier
six missing-include compile errors and full 4,870-pass/eight-missing-fixture
failure run remain retained separately. All 105 restored inputs match the
merged index, and 18 distinct unchanged fixture controls passed across the
retained replays before the complete rerun. No test, assertion, budget or
ignored set changed; the long imported isogeny-walk test completed normally.
The [integration receipts](evidence/certicom-base-integration-20261026/README.md)
preserve earlier partial and failed snapshots alongside the completed host
result. This library pass does not discharge the failed independent semantic
gate, current-head CI/review or physical GPU requirements.

At crypto head `917e2214b132e76c509575095269652c66ff907c`, three additional
failures and two other cancellations were retained as separate gates:

- [IC autolab integration](https://github.com/aburan28/crypto/actions/runs/39518361616/job/118955884854)
  reports `FileNotFoundError: zstd` and downstream fixture failures. Its
  workflow and tournament source are byte-identical to base `74842092891b87e69aeb7a980f78426717c595cb`.
- [The n37 replay](https://github.com/aburan28/crypto/actions/runs/39518361644/job/118955884858)
  reports missing `rustfmt` for Rust 1.98. Its workflow and frozen producer/replay
  source are unchanged from that base. No source or gate was weakened here.
- [Cover transfer assessment](https://github.com/aburan28/crypto/actions/runs/39518361534/job/118955884393)
  reports a stale generated ledger. `node scripts/build_cover_transfer_assessments.mjs --check`
  independently exits 1 with the same stale-ledger diagnostic on the exact head
  and the exact base, after materializing their tracked inputs in isolated
  checkouts. No unrelated generated ledger was changed in this follow-up.
- [S3 smoke and custody](https://github.com/aburan28/crypto/actions/runs/39518361733/job/118955885553)
  and [Windows isogeny](https://github.com/aburan28/crypto/actions/runs/39518361711/job/118969808523)
  were cancelled. Cancellation does not count as success; both remain separate
  CI observations. The shared-phase repair addresses the demonstrated Metal
  job budget, without patching these workflows or disabling gates.

## Sources checked for this follow-up

The existing worker README and this record define the retained controls and
scope. [GitHub's matrix documentation](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/run-job-variations)
supports independent phase jobs; `fail-fast: false` retains both observations
while the combined gate requires success. The [hosted runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
describes the macOS VM environment, so its reported device name is retained
without inferring physical provenance. Apple's Paravirtualized Graphics page
was checked, but its JavaScript page and linked Markdown were inaccessible to
the research tool; no hardware claim depends on that unavailable source. No
mathematical method, challenge parameter or performance claim changes here.

Unresolved. Merging these PRs or completing this record does not establish
physical-device correctness.
