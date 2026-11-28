# Draft matched native-launcher comparison — not frozen or run

The original 15× goal remains open. Repeated comparison 200/211 produced six primary median strongest-control/candidate ratios of 1.0027–1.1244, inside measured A/A spread. Parent trace 216/223 motivates testing fewer process launches, but its wait interval includes worker computation and does not measure pure wrapper overhead. The native supervisor passes its recorded functional qualification; that qualification has no parent-performance statistic. [pcc:unverified]

## Hypothesis and controls

Test whether the native supervisor changes complete application-call cost while retaining a fresh worker and the complete proof. Offer that same supervisor to every eligible baseline. A wrapper-only comparison would confound implementation choice with transport, so it cannot be the primary comparison. This is an implementation hypothesis, with no predicted ratio or claim that the mechanism can reach 15×.

The draft keeps all 12 original control algorithms: frozen-85 sparse, frozen-96 DAG, current sparse and current DAG, each with frontier-103, products-231 and batch-487 kernels. Each receives wrapped and native launchers, giving **24 controls**. The six packed-constructor sparse/DAG variants also receive both launchers, giving **12 candidates**. Primary is fixed as `packed-dag-487-native`; the other 11 candidates are secondary and cannot replace it after inspection.

Only the existing six originals are eligible: primes 2147483647 and 9007199254740881; n = 3, 4, 6; seed 7 LAST-star. No new or reserved instance seeds are available for selection. Both launchers preserve complete original-ideal membership, input containment, reduced monicity, strict-standard/checked-product S-pairs, complete wire/proof writes, worker independent replay and controller second whole-proof replay. No F4 solve runs; observed solving degree remains explicitly null.

## Fixed proposed observation roster

AA stage: five rounds × six inputs × 48 calls, with an original/copy pair for each of 24 controls. AB stage: five rounds × six inputs × 36 calls, with all 24 controls and 12 candidates. Total: **2520 calls** (1440 AA + 1080 AB). The same shared Rust roster constructs both stages. Stage/round/prime/n loop order and cyclic arm rotation by round are fixed; no warmup removal, trimming, replacement, retry or exclusion is proposed. A failed/censored call remains and makes the complete comparison ineligible.

|Variable|Treatment|
|---|---|
|Launcher|Wrapped taskset/timeout/prlimit or native pidfd/resource-limit supervisor, for every arm|
|Constructor/proof/kernel|Original fixed control/candidate arms; no post-hoc winner replaces the primary|
|Instances|Six original files and frozen input hashes|
|Primary observation|Complete parent application-call nanoseconds|
|Diagnostics|Worker constructor/wire, replay and worker/proof-write clocks; no standalone speed admission|
|Resources|LocalCPU0; one numerical thread; no cloud; child 30 s / 2 GiB / 16 MiB; case 180 s / global 600 s|
|Confounders retained|Order, host/storage variability, contention monitor and every raw outcome|

The full measured interval remains the agreed 96/200 boundary: raw capture/input open, command/limit setup and launch, construction/internal checks, complete wire, worker whole-proof replay and proof writes, process receipt write, stdout parse, exact input binding, controller full replay, basis/stat/cross-arm comparison and digests. Fixed input and worker hashing, build, output-directory preparation and analysis remain outside that per-call interval. In particular, the functional qualifier's per-call ELF hashing must not be copied into the timing interval. Both launcher paths in the draft collector use prehashed worker identities. A native timeout kills at 30 seconds; the old wrapper retains TERM at 30 / KILL at 31 seconds. Every nonzero, signal or timeout rejects admission.

## Proposed analysis and qualifications still required

For each case and round, divide the **minimum parent time across all 24 eligible AB controls** by the primary candidate time. Keep all five paired ratios. Retain the prior fixed xorshift bootstrap seed 0xc159ea72b084d36f, 20,000 resamples and simultaneous lower quantile 0.05/6. Require all six lower bounds strictly greater than 15, all calls complete, every full proof and binding checked, unchanged prospective source/tool/input pins, zero monitor-flagged contention, and each primary median beyond the largest symmetric AA ratio over every control pair. No new field-instance seeds or statistical selection are introduced.

The Rust draft analyzer binds every raw/summary cell, the process receipt clock, diagnostic phase clocks, fixed input/worker hashes, wire/basis/statistics, all-stage checked bases and the resource/launcher metadata. Its synthetic tests cover a faster native control defeating a misleading wrapper comparison, strict thresholds, A/A noise, contention, missing/zero controls, changed/null receipt clocks and a missing/failing sixth case. Tests 249 pass 8 analyzer + 3 collector checks (8 distinct definitions; the 3 shared roster checks run in both binaries). These are synthetic logic tests with zero performance observations. [pcc:unverified]

**This draft cannot admit a performance claim.** Its report distinguishes threshold arithmetic from admission and always leaves admission false. Before an actual run, reconcile and qualify a separate final source version against the canonical protocol/export schema and checker; complete the masked analysis mapping, measured positive calibration and unchanged-source/whole-proof sabotage qualifications; freeze every required source/tool/input pin; publish and remotely verify them before collecting observations. Preserve these draft sources and test outputs as their own context. Do not change the final analysis after seeing data.

The current experiment-protocol skill requires reading and following the canonical SKILL.md. That canonical file, schemas and checker are authenticated ciphertext in crypto-autoresearcher. The documented unlock has no configured KEY binding; an environment settings question is pending. This draft is not an `experiment.protocol/v1` or `experiment.export/v1` validation result and substitutes for neither. No comparative native-launcher performance calls have been run. Local checkpoint/S3 waiver and all original proof and measurement obligations persist.

References: frozen PACKED_COMPARE_PROTOCOL_200.md, PACKED_ANALYSIS_211.json, PARENT_TRACE_ANALYSIS_223.json, NATIVE_SUPERVISOR_PINS_239.json, SUPERVISOR_FINAL_OUTCOME_240.json, SUPERVISOR_FINAL_AUDIT_241.json and SUPERVISOR_ARCHIVE_AUDIT_245.json. No novelty claim; prior primary Linux/Rust source review is recorded in the qualification report.
