# Five fresh same-point n35 IC/rho pairs

The combined triangular-index and sparse-target candidate recovered five
distinct, preregistered public points in five alternating-order local IC/rho
pairs. Every IC and rho scalar passed point replay, and an independent
column-major index reconstruction verified each target's first witness,
signed relation, workload identity, phase sum, and scalar. The median paired
rho/IC online ratio was 4,509.50 in this ordinary-host rehearsal.

## Frozen workload and result

The candidate is
`IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h21421ad7a3b3`
on `EC1N35Ch4randha95af81414ea`. The [fixture generator](make_fixtures.py)
and its five exact [point records](fixtures.json) were pushed before timing,
as were the [protocol](protocol.json) and [runner](run.py). Each point is a
separate one-target workload; the panel is secondary to the earlier
[complete one-target run](../n35_triangular_sparse_pipeline_20261120/README.md).

| Point index | Exact public point | Arm order | IC online | Same-point rho online | Paired ratio | Calibration before/after |
| ---: | --- | --- | ---: | ---: | ---: | ---: |
| 0 | `(25950354521, 31081087090)` | IC, rho | 0.400 ms | 3,407.031 ms | 8,508.71 | 1.008 |
| 1 | `(24936013292, 4989641402)` | rho, IC | 0.729 ms | 3,407.188 ms | 4,671.91 | 1.046 |
| 2 | `(22553331361, 20243506662)` | IC, rho | 0.617 ms | 2,515.752 ms | 4,078.77 | 1.023 |
| 3 | `(22140771620, 18288140739)` | rho, IC | 0.590 ms | 2,661.735 ms | 4,509.50 | 1.015 |
| 4 | `(21067271462, 16218291137)` | IC, rho | 0.475 ms | 560.935 ms | 1,179.67 | 1.037 |

Median IC and rho online intervals were 0.590 ms and 2,661.735 ms.
Arithmetic means were 0.562 ms and 2,510.528 ms. The paired-ratio range
was 1,179.67–8,508.71. The [raw result](result.json) retains every target,
status, timing phase, scalar certificate, calibration interval, and resource
observation, including the full precision behind these rounded table values.

## Setup and claim boundary

The runner rebuilt the exact 4,096-point base and triangular index once and
replayed all 2,048 factor logs from the frozen complete run before starting
the target intervals. Shared setup took 74.184 s: 9.184 s base construction,
64.827 s index construction, 0.096 s log replay, and 0.077 s curve context.
Peak process RSS was 1,591,525,376 bytes. No relation collection or final
linear algebra was rerun for this secondary panel, so the shared setup is
not an empty-cache full-pipeline cost.

Each IC interval starts with target-dependent work and ends after original
point scalar replay; the paired rho interval uses the same public point and
includes its scalar replay. The fixed fixture scalars were generated outside
both intervals. The 5,000-addition before/after calibration is a diagnostic
on an unpinned macOS host; it does not establish Tier 0 or Tier 1 isolation.
The five ratios describe this local target law and process. A new five-point
panel on one qualified isolated allocation remains the performance gate.

The [independent verifier](verify.py) rebuilt the pair index in column-major
order with explicit minimum-payload updates, replayed all factor logs and
all five target witnesses and signed rows, and checked both scalar answers
per point. Its [validation receipt](validation.json) passed. The checked
[Sage runtime receipt](sage-runtime-info.json) was saved before fixture and
panel execution.
