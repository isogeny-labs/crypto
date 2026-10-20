# N83 five-base online tie-break, conditional on the sealed 192-point screen

This extension measures the five exact N83 factor bases on 1,536 new public
points if the 192-point Tier 1 screen is fully admitted but its corrected
selector returns `BOUNDED_UNKNOWN`. It increases paired precision while
preserving the current screen and its archived decision. The extension is a
secondary factor-base ranking study; a fresh one-target IC/rho holdout is the
next primary gate for a selected candidate.

## Trigger and identity

Dispatch only after independent local aggregation of
`research/n83_highk_online_distribution_20261020` reports
`admission_status: PASS`, `status: BOUNDED_UNKNOWN`, and
`selected_candidate_id: null`. Bind the extension dispatch to the SHA-256 of
that immutable aggregate and its archive. A failed/noisy/capped screen does
not trigger this tie-break; retain its disposition and repair that gate first.
Use `check_trigger.py` on the locally replayed aggregate and raw archive to
seal their hashes in `n83.online-tiebreak-trigger/v1`. The dispatch must
recheck those bytes and the trigger receipt before any timed child.

Use `EC1N83Ckb1a1haacb69ee9f37`: `y² + xy = x³ + x² + 1` over
`F₂[z]/(z⁸³ + z⁷ + z⁴ + z² + 1)`, subgroup order
`8569786107849059`. The executed IC source is commit
`e7634a81c22b60ad37e5d6bdb817a3b84867ae86`, file SHA-256
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
Retain the exact five candidate manifests, base dumps, layout flags and IDs
from the sealed screen, in its table order:

| Arm | Usable points | Columns | Candidate ID |
| --- | ---: | ---: | --- |
| K3072 root11 | 509,952 | 3,072 | `IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e` |
| K3500 root4 | 581,000 | 3,500 | `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea` |
| K3300 wide3 | 547,800 | 3,300 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` |
| K4300 wide3 | 713,800 | 4,300 | `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77` |
| K4405 wide3 | 731,230 | 4,405 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7` |

## Fresh secondary screen

Freeze 1,536 distinct public points using the domain
`n83-online-tiebreak-screen1536-20261020-v1` and the scalar rule
`1 + LE(SHA256(domain || u32le(index))) mod (r - 1)` for indices 0–1,535.
Save public-point and answer streams, their byte hashes, and a point-level
exclusion audit against all prior N83 target streams, including the running
screen's 192 points and reserved eight-point holdout. Answer sidecars stay
outside timed input. Split the new points into ordered blocks 0–767 and
768–1,535.

The frozen public stream SHA-256 is
`41b160dc56fadb0e8beec4c86bfee47457c6ade8290dd17cfa30c728787d5582`;
its answer sidecar SHA-256 is
`f17dcdfbf14b06c1cc708c8d0f362bf45c3b299156221be6eb9c1e281fb71bc7`.
The [exclusion audit](fresh_exclusion_audit.json) found zero shared points
against 31,800 distinct earlier public targets. The new stream itself has
1,536 distinct subgroup points.

On one frozen Tier 1 cloud allocation, run each of the five IC arms once per
block with empty setup caches. Use table order in block 0 and reverse table
order in block 1. Each child builds the exact base, index and logs, then
processes its block's public points. Preserve every per-point online
interval, exclusive target phase costs, recovered scalar, four-point
relation, base hash, failed attempt, and resource receipt. Shared preparation
and the 1,536-target count must be reported separately; do not divide a
process wall time by 1,536 to represent a single-target solve.

Run three identically seeded rho A/A calibration children before and after
each IC child, using the same frozen calibration point and seed as the
192-point screen. Require the before/after median spread, divided by their
mean, to be at most 20% for every child. The extension compares IC methods
on the same points; it does not run rho on every secondary-screen point.
The later primary holdout must solve its one public point with rho under the
same resource envelope.

Require identical source and binary hashes, exact candidate/base hashes,
target-stream hashes, CPU affinity and cgroup envelope in both blocks. Pin
four visible CPUs and at least four CPUs of quota, a cgroup memory limit of
at least 24 GiB plus 256 MiB, and a 24 GiB child RSS cap. Retain exposed
topology, provider machine ID, memory peak, calibration times, CPU steal,
throttling, memory-limit denials, OOM and every child exit. Require zero
steal, throttling, denial and OOM for admission. Child caps are 900 seconds
for K3072, 1,200 for K3500/K3300, and 1,500 for K4300/K4405. A capped or
unverified child leaves the entire selector `BOUNDED_UNKNOWN`.

For each of the ten unordered candidate pairs, count per-point wins and
losses from their 1,536 online intervals; drop exact ties from that pair's
binomial count. Use an exact two-sided sign test with Bonferroni threshold
`0.05/10 = 0.005`. Select a unique arm only if it wins more often than every
other arm with each corrected p-value below 0.005 and has the smallest
geometric mean interval. Record all pairwise counts, p-values, geometric
means, medians, paired log differences and cold-process costs. Otherwise
record `selected_candidate_id: null` and `BOUNDED_UNKNOWN`.

## Cost, archive and next gate

Use a RunPod class costing no more than $0.20/hour. Before allocation,
combine the provider-confirmed charge for all earlier campaign Pods with a
conservative reservation for this Pod; dispatch only when the sum is at
most the owner's $5 campaign ceiling. Freeze its provider stop time and
producer cap in the dispatch, leaving time for archive transfer and stop.
The allocation must not overlap another timed N83 run. Store the raw tree,
archive SHA-256, provider snapshot and termination receipt. Independently
replay every base, relation and scalar and recompute the selector from the
archived bytes before promoting a result.

The conditional Pod implementation is `runpod_runner.py` and
`launch_on_pod.sh`. The controller must run `check_trigger.py` against the
locally replayed aggregate and archive immediately before allocation and
again immediately before dispatch. Its `n83.online-tiebreak-runpod-snapshot/v1`
records the provider's single Pod, UTC creation and stop times, price,
machine and CPU allocation, the conservative earlier campaign spend, the $5
combined cap, pinned source HEAD, and trigger receipt/aggregate/archive
SHA-256 values. The matching `n83.online-tiebreak-runpod-allocation/v1`
contains the same Pod fields, snapshot SHA-256, and trigger hashes. Copy the
frozen tie-break package to the pinned `e7634a8` checkout, validate the
fixture there, then run the launcher with the allocation and trigger under
the recorded four-CPU affinity. The runner rejects a changed trigger,
checkout, price, affinity, or cgroup envelope before it starts a timed child.
The snapshot must confirm the preceding distribution Pod is terminated.
The budget verifier charges at least the frozen $3.57 earlier upper bound plus
that Pod's full allocation-to-snapshot interval at $0.19/hour, then the new
Pod's entire created-to-stop reservation. This deliberately overcharges any
idle gap after the preceding Pod terminated.

After `DONE`, transfer `results/` and `raw_results.tar.gz`, verify the
Pod-side SHA-256, stop and terminate the Pod, and retain the provider charge
and termination receipt. Run `aggregate_runpod.py` with the snapshot, raw
archive, extracted results, pinned source HEAD, and the same prior aggregate
and archive used for dispatch. It checks archive bytes before and after
independent replay, provider identity and spend, exact trigger bytes, each
target's scalar and four-point sum, resource receipts, and the selector.
Keep `BOUNDED_UNKNOWN` if any replay or Tier 1 admission gate fails.

If the extension selects an arm, freeze fresh one-target public points and
run an independently replayed Tier 1 IC/rho holdout in alternating order.
Include complete cold and exclusive online costs, same-point scalar checks,
A/A controls and every failed run. The screen alone selects a candidate for
that holdout; a verified one-target result and comparison scope are needed
before reporting a factor-base winner.
