# N83 six-base online finalist panel

This panel measures six complete index-calculus pipelines on 32 fresh public
points of `EC1N83Ckb1a1haacb69ee9f37`. The field is polynomial-basis
`GF(2^83)` with modulus terms `[0,2,4,7,83]`; the curve is
`y² + xy = x³ + x² + 1`, with prime subgroup order `8569786107849059` and
cofactor `1128547018`. The primary measure is verified single-target online
wall time after reusable factor-base, index, and log setup. Fresh-process
internal cold time is a separate supplementary measure. Every candidate
solves the same public point as its paired rho arm on the same Tier 1 cloud
allocation.

The exact executed IC source has SHA-256
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
All six arms use two target workers, cooperative cancellation, and
`KIC_INDEX_ROOT_BATCH=1`, which batches up to 83 relative S3 roots per
representative pair. The batched index preserves complete state, root-key,
and first-witness digests in its separately replayed K2816 stage panel.
Candidate manifests in this directory pin the exact ordered bases, layouts,
source and flags:

| Arm | Usable base points | Folded columns | Candidate ID |
| --- | ---: | ---: | --- |
| K1107 root11 | 183,762 | 1,107 | `IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0he423fcbc5668` |
| K2304 root11 | 382,464 | 2,304 | `IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h00a0a52d463c` |
| K2816 root11 | 467,456 | 2,816 | `IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h3a9440ac9544` |
| K3072 root11 | 509,952 | 3,072 | `IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e` |
| K3450 wide3 | 572,700 | 3,450 | `IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hc7264b83bbd1` |
| K3500 root4 | 581,000 | 3,500 | `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea` |

The [fixture](fresh_manifest.json) fixes 32 targets by
`1 + little_endian(SHA256(domain || u32_le(i))) mod (r-1)`, with domain
`n83-online-finalists32-batched-roots-20261018-v1`. Only public points enter
the producers; the scalar sidecar is for independent replay. The
[exclusion audit](fresh_exclusion_audit.json) found zero overlap with 31,456
distinct prior N83 public points and with the separate 32-point finalist
fixture. Each frozen candidate ID is SHA-256 over canonical sorted compact
JSON excluding the ID, with the actual usable base size in its readable tag.

Use four distinct overlapping GitHub Actions `ubuntu-24.04` allocations,
eight points per stratum. Rotate candidate order by point index; alternate
IC/rho order within each candidate-point pair. For each pair, run three
independent fixed-point rho calibrations before and three after. The paired
calibration medians may differ by at most 10% of their mean. The complete
schedule has 192 IC solves, 192 same-point rho solves, and 1,152
calibrations. Each IC child starts with empty process caches. Rho's seed is
`2026111800 + 6*point_index + fixed_candidate_index`.

Cap K1107 IC children at 600 seconds, K2304/K2816/K3072 at 900 seconds,
K3450/K3500 at 1,200 seconds, and each rho/calibration child at 120
seconds. Cap every child at 15.25 GiB peak RSS, each stratum producer at 330
minutes, and each cloud job at 360 minutes. The runner records exact
commands, source and binary hashes, event order, process peaks, host
topology, visible affinity, cgroup CPU and memory limits, throttling, OOM,
steal time, and calibration. The independent verifier replays every base
dump, relation, recovered scalar, paired rho scalar, five exclusive target
phases, resource event, and calibration result. The aggregate requires four
distinct overlapping provider runner IDs and binds every ZIP member to its
SHA-256. Failed, capped, noisy, and incomplete events remain in the raw
artifacts; either selector keeps its selected ID null until all strata pass.

For the primary online selector, calculate all 15 unordered paired
contrasts over the 32 points as `log(second_ms / first_ms)`. Select the
lowest geometric-mean candidate only when it favors every other candidate
and all five of its exact two-sided `2^32` sign-flip p values are below
`0.05/15`. The selector counts assignments through two different 16/16
partitions and rejects a numerical boundary disagreement. Apply the same
15-comparison gate to cold-process millisecond
differences as a separate measure. Report each paired contrast, point-win
count, arithmetic and geometric summaries, descriptive 95% paired interval
using `t(0.975,31)=2.0395134463964077`, and paired rho/IC ratios. The
candidate catalog and prior bridge establish why these six bases enter the
panel; this selection is scoped to these six exact pipelines and this curve.

The [dispatch](dispatch.json) binds fixture, source, candidate, protocol,
runner, verifier, selector, tests, and workflow SHA-256 values before a cloud
run. Only a push containing that file to
`codex/n83-sixarm-finalist-20261018` launches the panel.
