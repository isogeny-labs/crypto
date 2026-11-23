# Rational construction cost at the exact coverage radii

The two coefficient boxes that first reach complete reduced-point coverage
require 16,510 and 10,522 exact rational point evaluations in the frozen
height order. At `(p,q,B)=(1531,1553,65)`, the prefix reaches all 1,552
nonidentity points and a 145,007-bit maximum rational coordinate. At
`(2287,2251,54)`, it reaches all 2,250 and an 85,546-bit maximum. The
first reduction relations occur at evaluations 35 and 1,505, respectively.

## Frozen experiment and result

The [protocol](protocol.json) and [checked Sage runner](run_sage.py) were
pushed in `8a73ae6ceb6` before this measurement. The cells and radii come
from the prior [exact threshold sweep](../xedni_radius_threshold_20261120/README.md).
For each cell, the runner freshly evaluates every nonzero `(a,b)` in
`[-B,B]²` in the same 128-bit height-pairing order as the original panel,
stopping at the first prefix covering all nonidentity reduced points, the
complete box, or the frozen 300-second per-cell wall cap. Every constructed
`aQ+bR` is exact over `ℚ`; its reduction is checked against `aQ̄+bR̄`.

| p | q | B | Candidate cap | Evaluations to coverage | First relation | Peak coordinate bits | Construction and reduction wall seconds | Status |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1531 | 1553 | 65 | 17,160 | 16,510 | 35 | 145,007 | 111.926 | `full_coverage` |
| 2287 | 2251 | 54 | 11,880 | 10,522 | 1,505 | 85,546 | 35.052 | `full_coverage` |

The [raw rows](rows.jsonl) retain exact counts, relation labels, point
coverage, cumulative process maximum RSS, and full-precision diagnostic
intervals. [Result digests](result.json), [runtime receipt](sage-runtime-info.json),
and the [per-cell progress record](progress.jsonl) preserve the execution.
The wall interval begins immediately before candidate construction and ends
at coverage or cap. It includes rational arithmetic, reduction, and
per-candidate group-law verification; it excludes curve setup, pairing
calculation, label sorting, process startup, and the independent audit. This
ordinary-host measurement is a construction-cost diagnostic, not a Tier 0/1
single-target timing comparison.

The larger radius does not force a larger observed prefix peak in every
cell. The `p=2287` threshold box covers the group before reaching its
highest-score labels; its 85,546-bit peak is below the 98,113-bit peak from
the earlier radius-49 capped scan. Prefix identity and stop rule determine
the measured cost.

## Independent check and next step

The [Fraction replay](audit.py) independently builds every completed
`aQ+bR` from precomputed exact multiples, compares its finite-field
reduction with a separate group-law sum, and recomputes the first relation,
coverage, and coordinate peak. Its [result](audit-result.json) records the
verified point count and raw-row digest. The earlier pinned
[Lean threshold theorem](../xedni_radius_threshold_20261120/formal/RadiusThreshold.lean)
proves complete scalar coverage at radius 65 for the first cell; the present
experiment measures the exact rational work needed to reach that coverage in
the frozen order.

The next cost question is target-conditioned search at these radii: freeze
previously unseen reduced points, run independent one-target prefixes, and
pair each with a same-point rho run under a calibrated resource envelope.
The full-coverage prefix here is shared setup and does not itself give an
online one-target interval.

## Replay

From `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119`:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_threshold_rational_cost_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_threshold_rational_cost_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_threshold_rational_cost_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_threshold_rational_cost_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_threshold_rational_cost_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_threshold_rational_cost_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_threshold_rational_cost_20261120/audit.stderr
```
