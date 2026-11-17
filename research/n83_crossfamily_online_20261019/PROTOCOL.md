# N83 cross-family eight-point Tier 1 online holdout

This holdout measures the online cost of one exact high-K wide3 factor base
selected by the sealed high-K panel against the K3072 root11 and K3500 root4
contenders. All three use the same `a=1` N83 curve
`EC1N83Ckb1a1haacb69ee9f37`, polynomial-basis `GF(2^83)` with modulus
terms `[0,2,4,7,83]`, subgroup order `8569786107849059`, and the exact
current-source implementation whose IC file SHA-256 is
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The implementation is built from campaign commit
`e7634a81c22b60ad37e5d6bdb817a3b84867ae86`; the later opt-in parallel
index change is excluded from this comparison.

The two fixed arms are K3072 root11,
`IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e`,
and K3500 root4,
`IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea`.
The third arm is the `online.selected_candidate_id` from the independently
replayed `n83_root4_highk_tier1_retry_20261019` eight-point selector, but
only if its full Tier 1 aggregate passes and its status is `SELECTED`. It
must be exactly one of the frozen K3300, K4300 or K4405 candidate IDs in that
protocol. If the high-K selector is `BOUNDED_UNKNOWN` or any gate fails, this
holdout is `NOT_DISPATCHED`; no descriptive fastest arm substitutes for it.
Pin the selected ID and the selector artifact SHA-256 in dispatch before any
holdout child runs.

[`fresh_manifest.json`](fresh_manifest.json) fixes eight new public subgroup
points by `1 + LE(SHA256(domain || u32le(index))) mod (r-1)` with domain
`n83-crossfamily-online8-20261019-v1`. The target SHA-256 is
`36353c7fa20b31d50e0ca17ecad90608544d9f0350e6655f5f8d4cba4319a6db`;
the answer sidecar SHA-256 is
`3dd214eb570c534ead7bc94fe99499be85761f9ff7d8589fd5c1ac2441082068`.
The [exclusion audit](fresh_exclusion_audit.json) found zero overlap with
31,560 earlier public N83 points, including the active high-K selection
fixture, and zero overlap with the 32-point historical finalist copy. The
answer sidecar remains inaccessible to timed children and is used for
independent scalar replay.

Use one new RunPod Community allocation, one machine ID and one frozen source
snapshot for all eight points. Accept a V100 PCIe/SXM2 or RTX A2000 class at
no more than $0.20/hour, at least four exposed CPUs and four CPUs of cgroup
quota, at least 24 GiB plus 256 MiB of cgroup memory, and 3 GiB free disk.
Pin the lowest four exposed logical CPUs. Cap the provider allocation at
seven hours and the producer at six hours 40 minutes. With $1.50 reserved
for prior attempts and $2.07 reserved for the running high-K selection Pod,
this adds at most $1.40 and keeps the combined reservation below $5. The
controller stops the Pod on a hash-verified local archive or the prefrozen
deadline, then terminates it after the archive is retained. Record provider
creation, price, stop/termination, allocation ID, machine ID, image and
topology. No API key is placed on the Pod.

For every point, rotate arm order and alternate IC/rho order. Each arm starts
an empty-cache IC process to build its own base, index and logs, then times
one previously unseen target after reusable preparation is ready. Pair it
with an independent rho solve on that exact public point. Three fixed-point
rho calibration children run before and after each pair. Record one-target
online wall time and its exclusive query, PDP, relation-check, descent and
recovery-check phases; record cold setup, rank and index time separately.
The IC child caps are 900 seconds for K3072, 1200 for K3500/K3300, and 1500
for K4300/K4405. The rho and calibration caps are 120 seconds. Every child
has a 24 GiB RSS cap. Preserve all 192 scheduled events, including failures
and timeouts, with actual process and cgroup resources.

Independently replay each base, row/witness, subgroup scalar, same-point rho
scalar, phase sum, event order, source/binary digest and provider envelope.
Require zero CPU throttling, steal time, memory-limit denial and OOM, plus
at most 20% before/after A/A median spread for every pair. The 20% Tier 1
noise limit was fixed before these points were generated, based on the earlier
12.585% pilot observation; it never admits that earlier rejected panel.
Any missing or noisy pair leaves the aggregate `BOUNDED_UNKNOWN`, with a null
selected ID.

For complete admitted runs, compute all three paired log online contrasts
across the eight points. Enumerate exact two-sided sign flips and apply
Bonferroni `0.05/3` to the three comparisons. Select the fastest geometric
mean candidate only if its direction favors and passes both alternatives;
otherwise retain `selected_candidate_id: null`. Report each point's verified
same-point `rho_online_ms / IC_online_ms`, the paired IC contrasts and
uncertainty, and the observed Tier 1 resource envelope. Cold process costs
are supplementary and selected separately. This fresh holdout, not the
earlier high-K selection points, decides the cross-family online result.
