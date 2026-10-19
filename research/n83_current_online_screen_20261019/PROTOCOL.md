# Current-source N83 online factor-base screen

This experiment compares three exact high-K factor bases on 8,000 new public
points using the current IC producer. It measures the complete target-online
path after each fresh process builds its base, relation logs and root index.
The result is a secondary selector diagnostic. The separate fresh-process,
one-target same-point IC/rho holdouts provide the primary online speedup rows.

The curve is `EC1N83Ckb1a1haacb69ee9f37`, with binary field modulus terms
`[0,2,4,7,83]`, curve `y²+xy=x³+x²+1`, subgroup order
`8569786107849059`, and cofactor `1128547018`. The exact producer source SHA
is `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
All arms use the same 4-point root decomposition, group verification, target
cancellation, and batched root lookup. The fixed variants are:

| Arm | Usable base points | Folded columns | Exact candidate |
| --- | ---: | ---: | --- |
| K3450 wide3 | 572,700 | 3,450 | `IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hc7264b83bbd1` |
| K3500 root4 | 581,000 | 3,500 | `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea` |
| K3596 root4 | 596,936 | 3,596 | `IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h3da4e8b31c3f` |

Public points come from `SHA256("n83-current-online-screen-fresh-8000-20261019-v1"
|| u32_le(index))`, reduced to nonzero subgroup scalars. The producer receives
only public points. The answer sidecar is used for independent replay. The
public SHA is `c3ff6b9f3f2f5524ab6d3db036f7916e88f9b6d6f554c0245b52fe514e905d41`;
the answer SHA is `329065934512488ac4fbd04b04db54d188bfdc75d6c0f8eb3bc770764d02910a`.
All 8,000 are distinct and were checked against 8,064 pinned public points
from the prior 8,000-point panel and the three current fixtures.

Run six fresh processes, K3450/K3500/K3596/K3596/K3500/K3450, with empty
indexes and relation logs. Each process has a 1,500-second cap and observed
RSS guard of 15.25 GiB. Require at least four exposed CPUs, at least 15.5 GiB
of physical memory, 2 GiB free disk, no cgroup CPU throttling, OOM kills or
observed steal-time jiffies in any arm,
complete subgroup rank, all 8,000 recovered and verified scalars, and
independently replayed factor-base points and four-point relations. Preserve
every failed process and resource receipt, including topology, affinity,
cgroup, CPU-frequency and pressure snapshots. The EC2 Spot instance is Tier 1;
its visible affinity alone does not establish physical-host isolation.

Average the two process times for each candidate and target. Resample 320
contiguous blocks of 25 targets 10,000 times using the fixed pairwise seeds
`2026101651+i*2+j` and repeatability seeds `2026101661+i` in the verifier.
Select the lowest mean only if its upper two-sided 99% paired block-bootstrap
endpoint is below zero against both competitors and every A/B 99% interval
lies within ±1.0 ms. Any missing, capped, unverified, noisy or failed arm
leaves the selection null. Report all paired means, intervals, process setup,
peak RSS, throttling and base/index identities, including a censored outcome.

The cloud envelope is one `g5g.metal` Spot instance in `us-east-1b`, max bid
`$0.50/hour`, 165 minutes from pending time, compute ceiling `$1.375`, with
a boot-time systemd shutdown watchdog, encrypted 50 GiB gp3 root disk,
outbound-only security group, and private versioned S3 evidence archive.
Its measured Spot price must be at or below the bid before launch. The
approved campaign ceiling remains $5 including this allocation and any
separately charged storage. The instance shuts down after archiving or at the
watchdog deadline. The archive retains raw base dumps, per-target rows,
resource receipts, source/build hashes, full replay output and failures.
