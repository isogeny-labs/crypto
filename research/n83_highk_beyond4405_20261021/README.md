# N83 five-byte index capacity boundary and next factor-base comparison

The current symmetric one-root index reaches the 75% worst-case load boundary
at K4405. Its 2³¹ five-byte slots occupy 10,737,418,240 bytes. At K4406 the
existing load75 sizing rule requests 2³² slots, or 21,474,836,480 bytes.
`capacity.py` reproduces the producer's state bound and slot sizing with exact
integer arithmetic. It also finds K4549 as the largest K whose two-root
upper load is at most 80% of the existing 2³¹-slot table. The K4405 measured
process reached 22,355,877,888 bytes peak RSS in the independently replayed
1,536-point Tier 1 screen; its online selector remained `BOUNDED_UNKNOWN`.

The capacity proof is checked in `Capacity.lean` with the pinned Lean 4.35.0
toolchain. `maxStates` is the producer's symmetric N83 state bound:
`83 * (K * (K - 1) / 2) + 42 * K`. `maxStates_mono` proves this bound is
monotone on natural K. `max_k_load75` proves any K with worst-case two-root
load at most 75% of 2³¹ slots is at most 4405; `load75_4405` proves the
boundary is attained. `max_k_load80` and `load80_4549` give the corresponding
80% boundary. `lean-check.log` records a successful module build and
`#print axioms` reports only `propext` and `Quot.sound`. `lean4checker --fresh`
was unavailable on this host.

The opt-in `KIC_INDEXED5_FIXED31=1` mode in
`examples/koblitz_orbit_dlp_fast.rs` fixes the five-byte table at 2³¹ slots
and rejects a worst-case two-root load above 80% before allocation. It
requires the exact N83 one-root, ID30 and five-byte index modes, and excludes
the existing load75 and fixed30 sizing modes. The slot encoder and exact-key
reconstruction are shared with the checked ID30 path. The new mode has a
separate result flag. The initial comparison includes K4405 under both sizing
flags as a same-base layout control, then K4500 and K4549
under fixed31. Record actual usable base counts, hashes, source and binary
identity, memory, setup, rank, one-target online intervals, same-point rho,
and independent arithmetic replay per run.

The base-only path produced exact first-K signed-Frobenius bases on the local
correctness host without allocating the root table. K4405 reproduced the
archived BLAKE3 base hash `d64fcee7ba4975873859d256302872681c82b99fbe687ff4fba0bf3ba3a4c8d2`.
K4500 contains 747,000 usable points in 4,500 folded columns, with base hash
`6f3305328b265b70f7a2d53e97be2c4b70c3da291ef4cca4693dac6dae468f95`.
K4549 contains 755,134 usable points in 4,549 folded columns, with base hash
`65bcc74cd45d33881bcaad17216fe724d2666072230230ed6868eb7187b968af`.
`verify_base_stage.py` independently replayed every representative, subgroup
check, signed Frobenius point and label for all three bases, checked their
ordered K4405 prefixes, and matched the SHA-256 set hashes in the candidate
manifests. All three arms are `PASS_INDEPENDENT_BASE_REPLAY` in
`base_stage_validation.json` (SHA-256
`fe7667fa4b0eeb2ca0604fc06615d2cd109c6c54a22e0b37cfd4235bb4486f8d`).
The full ordered dumps and receipts for the merged source are in the
53,531,559-byte
`base_stage.tar.gz` at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_fixed31_base_stage_merged_20261021/`.
Its SHA-256 is `5b13e49152b0f120400930fa181a5e8cd3e8e45640bc0134389b10945b3ce333`,
confirmed by a fresh S3 download in `base_stage_custody.json`.
`source_binary_custody.json` separately pins the executed macOS ARM64 source
and executable to fresh S3 downloads.
`identities/catalog.json` records the four source-bound IC1 candidate
IDs for the K4405 load75/fixed31 controls and the two larger bases. These are
exact candidate identities, with run costs still unknown.
The separate split-index campaign has already enumerated and independently
replayed a K5500 base with 913,000 usable points. This fixed31 study tests the
largest monolithic ID30 table configurations within its 2³¹-slot capacity;
K5500's index, rank and target measurements remain a separate frontier gate.
The [previously Lean-checked K5500 split layout](../n83_two_tier_root_20261021/CAPACITY.md)
calculates 33,136,508,082
bytes (30.86 GiB) for its state, two exact tables and two Bloom filters
alone, above this run's 26 GiB child RSS cap. Its full comparison needs the
separately prepared larger-memory resource envelope.
The current source now reproduces the exact archived K1107 and K5500 ordered
base dumps. K1107 received fresh independent arithmetic replay and an exact
K4405 prefix check; K5500's current output is byte-identical to its prior
independently replayed dump. Their receipts are in
`K1107_CURRENT_SOURCE.md` and `K5500_CURRENT_SOURCE.md`.
K1107 has now also completed a current-source full-process control on an
earlier public point: final rank 1,107, independently replayed four-point
relation and scalar, 128.914-second local process wall time, 334.808 ms
exclusive target interval, and 1.933 GB peak child RSS. This is a correctness
and source-bridge result; its Tier 1 sealed same-point comparison remains
pending. The current-source K1107 IC1 candidate identity is
`IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hf1851768e85a`.
The raw and replay receipts are in `k1107_current_source_full_control/`.
The current source also reproduced the historical K3596 root4 ordered base
byte for byte, with fresh independent arithmetic replay of all 596,936 usable
points; see `K3596_CURRENT_SOURCE.md`. Its full current-source index and target
run remains a separate gate.
The earlier source snapshot, its candidate identities and custody receipt are
retained under `identities_source_f9_snapshot`,
`base_stage_receipts_source_f9_snapshot` and
`base_stage_custody_source_f9_snapshot.json`.

`generate_fresh.py` sealed 1,536 new public screen points and 16 one-target
holdout points. The respective public-stream SHA-256 values are
`e62e8ea5b77dbcbd80217b8f994ebb571fa03647355bfb5593077c58aedfdf11`
and `a0e1f76ddec6ae901d5e766a942ad0b35299e10f175243fe7abd0fc679040a96`.
`audit_fresh.py` found zero overlap within these streams or against 33,602
distinct earlier public points, including the prior high-K screen, both
current 2026-10-21 cold-base fixtures and the merged branch's K5500 screen.
The initial 33,368-point audit remains versioned; `fresh_exclusion_audit_v2.json`
is the expanded current inventory. Answer sidecars are excluded from timed
inputs. [PROTOCOL.md](PROTOCOL.md) fixes the comparison and promotion gates.
`seal_dispatch.py` checks the source, candidate identities, independent base
replay and fixture exclusion before writing the immutable `dispatch.json`.
The dispatch pins the four arms, alternating screen blocks, one-target holdout,
Tier 1 resource limits and proposed allocation ceiling. Its current status is
`AWAITING_OWNER_BUDGET_APPROVAL`; it is a prepared experiment, not an allocation.
`seal_workloads.py` records two exact secondary block workloads and 16
separate one-target workloads. The preregistered point-0 workload ID is
`f3f41cbb7544`; verifier rows use candidate/workload/run IDs under the
repository naming contract.
`run.py` verifies all 1,552 public points against the sealed scalar sidecars,
then schedules eight cold IC children with three rho A/A calibrations on each
side. It keeps the sidecars out of child inputs, records every capped child and
stops before the ten-hour producer limit. Its fixture preflight passed with
1,536 screen points, 16 holdout points and 56 screen events.
`runpod_runner.py` binds the producer to a quoted Pod, four pinned CPUs,
the recorded cgroup envelope, source commit and ten-hour cost limit. It keeps
the allocation receipt with the raw results.
`watch_runpod.py` is the separate provider controller: it checks the live Pod,
emits a readiness receipt required by the Pod launcher, transfers the archive,
terminates the Pod with five minutes of budget margin, and confirms absence.
Its tested failure paths still issue termination when transfer fails or the
watchdog starts late. A live provider absence receipt is required for
measured Tier 1 admission. After termination, it writes
`provider_custody_validation.json` through `verify_provider_custody.py`.
That local gate matches the allocation, watchdog chronology, archive SHA-256,
embedded allocation and confirmed Pod absence; it returns `BOUNDED_UNKNOWN`
when any receipt is missing or inconsistent. The Pod-side scientific selector
and this post-termination custody gate must both pass before a Tier 1 result
is promoted. Its `final_primary_status` is read from the immutable archive and
remains `BOUNDED_UNKNOWN` unless the archived screen and point-0 cross-family
replay both passed. `final_cold_cost_status` separately requires the archived
16-pair setup-inclusive selector, which the controller recomputes from the
archived one-target rows before promotion. The custody check can be rerun
independently:

```sh
python3 -m research.n83_highk_beyond4405_20261021.verify_provider_custody \
  --allocation /absolute/path/to/runpod_allocation.json \
  --archive-dir /absolute/path/to/local_archive
```
`launch_on_pod.sh` checks the checkout and fixture, builds both release
examples on the Pod, runs the screen under the pinned affinity and provider
deadline, then seals an archive with the raw child receipts and build logs.
`verify_screen.py` replays every recovered scalar and four-point relation
with the independent N83 group model, checks exact base/rank/index and phase
accounting, preserves failed child rows, checks the Pod envelope and rho A/A
noise, and emits the validated matrix consumed by `select.py`.
`holdout.py` schedules the selected base and current-source K1107 against
separate same-point rho controls on each of the 16 individual sealed points.
It places preregistered point 0 first and alternates candidate and IC/rho
order. `verify_holdout.py` independently replays every answer and produces
the point-0 cross-family online wall-time ratio only when both Tier 1,
resource, noise and correctness gates pass. Its 16-point paired selector
retains `BOUNDED_UNKNOWN` for incomplete or conflicting evidence. The
[cross-family addendum](K1107_CROSS_FAMILY_ADDENDUM.md) freezes its event
order and selection rule. A separate 16-pair cold-process selector ranks
setup-inclusive one-target IC cost, retaining setup, rank, target and residual
phase accounting. The Pod launcher runs this phase only after the
screen selector admits a unique fixed31 base and time remains before the
provider deadline.
`select.py` admits a fixed31 winner only from a complete independently replayed
Tier 1 matrix, a passing same-base layout control, and all three pairwise
exact sign tests at the Bonferroni threshold of 1/60.

The merged source release example suite passed 24/24 tests with Rust 1.93.1;
its command and output are retained in `rust-example-tests-merged-source.log`.
The merged upstream source contained an incomplete fixed31 assertion; the
restored separator compiled in the same release build used for the new base
stage.
The earlier 22/22 suite is retained in `rust-example-tests.log`. The capacity
model, Lean module, `rustfmt --check`, and `actionlint` passed. The branch
workflow repeats the capacity, Lean, and Rust example checks with Rust 1.99.1
on its exact head. A full K4500 or K4549 process has not yet been measured.
The first broad parallel release library attempt recorded three time-limited
`jv_cover` failures before interruption in `rust-lib-tests-parallel-failed.log`.
Each of those tests passed on isolated replay and in the subsequent serial
run. That serial full-suite run finished with 4,157 passed, 117 ignored and
five `cryptopro_b_chain_set` failures because this sparse worktree had omitted
its tracked frozen input file. The raw result is in
`rust-lib-tests-serial.log`. After materializing the exact committed fixture,
all five failed tests passed in a focused serial rerun. The full serial binary
rerun with the fixture present passed **4,162 tests, zero failures, 117
ignored** in 1,310.28 seconds. Its raw output is
`rust-lib-tests-serial-with-fixture.log.gz` (raw SHA-256
`92d148a7a4c827431f2f8d56a342ac544cf16ead33205476b7671621310f59ca`).
The release library command gate is
recorded separately. A rebase removed that tracked fixture from the earlier
ad hoc sparse worktree materialization. The exact Cargo command then recorded
4,157 passes and the same five missing-fixture failures in
`rust-lib-tests-cargo-serial.log.gz` (raw SHA-256
`6a6acd8efe85265c9e4dc3b6a981fdc0a0ec9a968b8c0fe753ffca08681b581e`).
The sparse profile now includes
`research/cryptopro_b_chain_sweep_20261006/frozen_inputs`; all five tests
passed through Cargo in `rust-lib-tests-cryptopro-fixture-focused.log.gz`.
The touched release example suite passed 24/24 again in
`rust-example-tests-current-head.log.gz`. The stable-profile exact
`cargo test --release --lib -- --test-threads=1` rerun passed **4,162 tests,
zero failures, 117 ignored** in 3,970.39 seconds on the contended local
host. Its raw output is retained in `rust-lib-tests-cargo-serial-stable.log.gz`
(raw SHA-256
`b94c87a2ac20e3e2862f3849b7f6a73e78e9abe08a9c0a0991a6b85e88e8f360`).
The previous RunPod campaign's conservative upper charge was $5.710216
against its $5 ceiling, so this preparation launches no paid instance. A new
bounded allocation needs its own reviewed budget and protocol.
