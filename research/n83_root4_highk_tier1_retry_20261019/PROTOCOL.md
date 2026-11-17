# Eight-point N83 high-K root-index selection on one RunPod allocation

This frozen panel compares three exact `a=1`, wide3 N83 factor bases on eight
new public points. It measures one-target online IC time for each base and an
independent same-point rho solve, with full cold index cost retained separately.
It supersedes the earlier cgroup-v1 panel **for selection only**: that panel's
first K3300 pair had 12.585% before/after A/A median drift against its frozen
10% limit, so its status is `NOT_ADMITTED` and all raw receipts remain retained.
No pair or threshold from that allocation enters this selector.

| Arm | Candidate ID | Actual points | Folded columns |
| --- | --- | ---: | ---: |
| K3300 wide3 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` | 547,800 | 3,300 |
| K4300 wide3 | `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77` | 713,800 | 4,300 |
| K4405 wide3 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7` | 731,230 | 4,405 |

The exact curve is `EC1N83Ckb1a1haacb69ee9f37`, subgroup order
`8569786107849059`. Candidate manifests resolve the field representation,
base-point digest, root-table construction and source hashes. The executed IC
source hash is `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
`generate_fresh.py` fixes indices 0–7 from domain
`n83-highk-wide3-tier1-retry8-20261019-v1` using
`1 + LE(SHA256(domain || u32le(index))) mod (r-1)`; its target file SHA-256 is
`2824e714820524e81d7e3cb16821529827d82a18fc1c9de8a0e7315ab940deac`
and answer sidecar SHA-256 is
`10a782568c1bdc09c8ca9d2e5536b243d12c7382e13bb1d72a59458e81cf289c`.
`audit_fresh.py` found zero overlap with 31,552 earlier public points and the
32-point historical finalist copy. These eight points form the **selection**
panel. A selected base subsequently needs a separate preregistered Tier 1
same-point IC/rho holdout of at least five new points.

One new RunPod Community Pod and one machine ID must remain allocated for all
eight points. Accept only a V100 PCIe/SXM2 or RTX A2000 class at no more than
$0.24/hour, at least four visible CPUs and four CPUs of cgroup quota, at least
24 GiB plus 256 MiB of cgroup memory, 3 GiB free disk, and a 24-GiB child RSS
cap. Freeze and pin the lowest four exposed logical CPU IDs for the runner and
all children; bind that exact set into the provider and host receipts. Freeze Pod ID, machine ID, exact GPU/CPU class, image, source commit,
price, allocation times and stop-after before any measured child. The producer
cap is 30,600 seconds; the provider stop-after is at most 32,400 seconds.
With $1.50 reserved for all earlier attempts and a $2.50 new-allocation cap,
the projected combined spend stays below the owner's $5 cap. The controller
stops the Pod before either bound. No API key is placed on the Pod.

For each point, rotate the three arms and alternate IC/rho order. Three
identically seeded rho A/A calibration children run before and after every
pair. Each IC child starts with empty target-independent caches and builds its
base/index; record that cold process time separately. The online clock starts
at first target-dependent IC work and ends after scalar recovery/check.
Every timed result must pass independent base, witness, scalar and telemetry
replay; missing or failed attempts remain explicit. Child time caps are
1,200 seconds for K3300, 1,500 for K4300/K4405, and 120 for rho/calibration.
Require zero cgroup throttling, CPU steal, memory-limit denial and OOM. The
A/A gate is a predeclared at-most-20% relative difference of the two
three-repetition medians, computed against their mean. This wider cloud noise
bound is fixed from the earlier 12.585% pilot observation **before** any new
point is run; it does not change that earlier panel's rejection.

`run.py` fixes 192 events. `runpod_runner.py` binds cgroup-v1 raw counters to
the provider receipt. `verify.py` independently replays the full eight-point
result. `aggregate_runpod.py` binds exact archive bytes to the replayed tree;
`runpod_provider.py` checks the single allocation and budget. All 24 IC/rho
pairs must pass the Tier 1 gates. `select.py` uses exact two-sided sign-flip
p-values on paired log online ratios, with Bonferroni `0.05/3` across the three
pairwise comparisons; cold process differences are separately tested. A
candidate is selected only if it is faster than both alternatives in their
prescribed directions at that threshold. Otherwise `selected_candidate_id`
remains `null` and status `BOUNDED_UNKNOWN`. Report every one-target rho/IC
ratio and its observed cloud-instance envelope. The selection panel alone
cannot serve as the fresh post-selection speedup holdout.
