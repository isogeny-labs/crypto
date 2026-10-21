# Prepared next comparison: K5086/K4549 dual-Bloom factor base

The next factor-base candidate is the largest K admitted by the current
30-bit state-ID format: K5086 with a K4549 primary split and separate
`wide3` Blooms. [CAPACITY.md](CAPACITY.md) proves the numeric bound and
records the 26.00 GiB three-array allocation calculation. The actual base,
rank, relation yield, online time, and memory peak remain unmeasured.
`candidate_id` and measured costs remain `null` until the base has been
enumerated, subgroup filtered, hashed, and independently replayed.

## Fixed source, controls, and resource gate

- Curve: `EC1N83Ckb1a1haacb69ee9f37`, exact N83 `a=1` binary model,
  subgroup order `8569786107849059`, with the generator and field encoding
  from the existing K4300/K4405 manifests. The field size `2^83` does not
  identify the subgroup bit length.
- IC source: [koblitz_orbit_dlp_fast.rs](../../examples/koblitz_orbit_dlp_fast.rs),
  SHA-256 `c125b638d89e418936895659f1b3bd856b4e18a6a24f1d307dec99a204833cf0`.
  Rebuild native code on the chosen Pod and record the Pod executable hash.
  The existing K4300 and K4405 exact manifests are the controls.
- K5086 flags: `KIC_ROOT_ONLY11=1`, `KIC_SYMMETRIC_S3=1`,
  `KIC_INDEXED_ROOT5=1`, `KIC_INDEXED5_ID30=1`,
  `KIC_INDEXED5_LOAD80=1`, `KIC_PACKED_ROOT_TABLE=1`,
  `KIC_INDEXED5_SPLIT_AT=4549`, `KIC_ROOT_BLOOM=1`,
  `KIC_ROOT_BLOOM_PROFILE=wide3`,
  `KIC_INDEXED5_SPLIT_SHARED_BLOOM=0`, with the frozen guided-rank,
  batch-root, target-cancel, and target-thread policies from the K4405
  controls. Record every flag in the candidate manifest.
- Tier 1 requires one frozen Pod allocation with at least four exposed CPUs,
  four CPUs of quota, and 48 GiB cgroup memory. Pin four CPUs and cap each
  child at 40 GiB sampled RSS. Cap K4300 and K4405 screen children at
  1,800 seconds, the K5086 screen child and each one-target IC holdout at
  3,600 seconds, each rho child at 120 seconds, and each calibration child
  at 30 seconds. Record Pod and
  machine IDs, image, architecture, CPU model, affinity, cgroup limits,
  throttling, steal time, memory pressure, and before/after same-seed
  calibration. Retain noisy, failed, timeout, and OOM rows. The provider
  watchdog must verify Pod absence independently after termination.

## Stage gate before candidate selection

Construct K5086 with the split settings above, dump the ordered base, and
freeze its hash and exact point count. Independently check subgroup
membership, distinctness, the 166-point-orbit expansion assumption, the
1,073,499,477-state count, all exact-root insertion counts, the 2^31 and
2^29 slot allocations, the two Bloom allocations, and the full-rank log
system. If any count, base hash, relation sum, or scalar replay differs,
record the failure and stop promotion. A capacity calculation alone cannot
set `fb844276` or form an `IC1` candidate ID.

## Paired online selection and one-target holdout

The 192 public screen targets are sealed under
`n83-maxk-screen192-20261021-v1` (target SHA-256
`b99b196b43e69414c101e948ca74ea32703d1ea87ad71e0109e01b37f8cafe38`)
and the eight disjoint holdout targets under
`n83-maxk-holdout8-20261021-v1` (target SHA-256
`96375c178e8315c136f190927dc3470fc84f1b7096c2e563387808928aa56d6a`).
[generate_maxk_fresh.py](generate_maxk_fresh.py) derives each scalar as
`1 + LE(SHA256(domain || u32le(index))) mod (r-1)` outside timed intervals;
it checks the public point and stores scalar sidecars separately from the
executable's public-point inputs. The [fixture manifest](maxk_fresh_manifest.json)
pins their hashes. [audit_maxk_fresh.py](audit_maxk_fresh.py) verified zero
overlap with 33,336 prior distinct public points, including the frozen
1,536-point high-K archive and the local control target; the
[exclusion receipt](maxk_exclusion_audit.json) pins every input digest.

The secondary screen compares three complete candidates: frozen K4300
wide3, frozen K4405 wide3, and verified K5086/K4549 dual wide3. Use the
same 192 points and one Pod resource envelope, with two 96-point blocks and
reverse candidate order in block two. Retain each target's exclusive online
phase costs, relation indices, verified scalar, failures, and cold setup
costs. Use an exact two-sided sign test for each of the three candidate
pairs with Bonferroni threshold `0.05/3`. Select one candidate only if it
wins more often than each other arm below that threshold and has the
smallest geometric-mean online time. Otherwise set
`selected_candidate_id: null` and `BOUNDED_UNKNOWN`; the screen does not
substitute for a single-target online result.
The frozen [selector](select_maxk.py) consumes an independently replayed
192-point Tier 1 matrix, requires all three exact candidate manifests and
both reverse-order blocks, and applies this rule. A missing point, failed
scalar replay, noisy pair, or failed exact-base stage gate leaves the
selection `BOUNDED_UNKNOWN`.

Only after a unique selection, run eight fresh one-target IC children, each
with empty setup caches, and independently solve the same public point by
rho. Alternate IC/rho order, freeze all seeds and limits, run before/after
same-seed A/A calibrations, and preserve every failure or noisy pair. The
headline is each verified one-target `rho_online_ms / IC_online_ms` and the
paired aggregate under the repository Tier 1 gate. The IC online clock
starts with target-dependent query work after reusable preparation and
stops after scalar verification; cold index/base/log costs are reported
separately. The rho clock starts with target-dependent walk work and stops
after independent scalar verification.

The previous RunPod combined cap was exhausted. A fresh spending ceiling,
provider rate check, binary build receipt, and exact candidate manifest are
required before allocation; the cost cap and Pod identity remain unfilled.
No Pod is allocated by this protocol. The current local K4405 split run is a correctness control
and has its own 7,200-second wall, 26 GiB RSS, and 8 GiB internal-disk
free-space guards; its result is recorded separately from Tier 1 selection.

The [RunPod GPU Models price page](https://www.runpod.io/gpu-models), checked
2026-10-21, lists a Community Cloud RTX 3090 with 125 GB host RAM, 16 vCPUs,
and a $0.22/hour displayed rate. This is a capacity and price lead, not an
allocation receipt. The dispatch gate will require an eligible live Pod at
no more than $0.35/hour, verify at least 48 GiB actual cgroup memory and
four CPUs of quota, and cap provider lifetime at 12 hours. At that ceiling,
the projected 12-hour provider charge is at most $4.20; the independent
watchdog in [runpod_guard.py](runpod_guard.py) will request termination
10 minutes before the earlier provider-lifetime or approved-spend deadline
and verify provider absence through both direct lookup and Pod inventory.
Its API key is entered without echo and is not included in its receipt.
The guard has a local mock-provider deletion/absence control; no Pod has
been allocated with it. If the eligible Pod is unavailable, the
dispatch stays pending without substituting a weaker memory envelope.
