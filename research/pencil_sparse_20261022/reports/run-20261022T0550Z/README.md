# First-target 28-bit pencil price records

The [frozen protocol](../../PROTOCOL.md) fixed the original p=48,383
screen target for identity, reverse and affine column orders. The separately
frozen degree-peeling alternative used the same target and identity order.
Two adjacent archived 28-bit models, p=48,823 and p=49,451, each supplied
their first frozen target under the original online identity solver. All six
candidate and same-process rho scalars were source checked. These are
one-target records; they do not describe the other 59 targets per model.

The five online commands used `PENCIL_MAX_TARGETS=1`,
`PENCIL_NODE_ORDER=identity|reverse|affine` for p=48,383 and
`PENCIL_NODE_ORDER=identity` for p=48,823 and p=49,451, each with
`pencil-sparse --cohort 28 P`. The peel command was
`PENCIL_MAX_TARGETS=1 PENCIL_NODE_ORDER=identity PENCIL_SOLVER=peel
pencil-sparse --cohort 28 48383` from the repository root. Online binary
SHA-256 was
`40820a6179e903b2482c1eefc7c088222e0d1e1b3ce260a72aa6901c220d742b`;
peel binary SHA-256 was
`539144816c83d765da6ea5731f34b5a2d3a8bd75519d02721342f56c797ce6a8`.

The [PDF](RUN_REPORT.pdf), [first-target graph](p28-first-targets.svg), and
[36-cell heatmap](p28-price-band.svg) show complete
representative operation prices against rho and p=48,383 paired costs
against the online identity solver. [Every target and pricing cell](../../results/price-grid.csv),
the [paired peel grid](../../results/p48383-peel-vs-online.csv), and the
`../../results/p*-t0.jsonl` raw records retain the individual costs and
counter evidence. The [native validator](../../results/validation.json)
checked source scalars, archived workload IDs, matched rho controls,
field-equivalent non-wall replays and every price cell. All replay
processes finished. Pre-peel online records omit three zero/default solver
metadata fields that the replay binary emits; the certificate names them.

At p=48,383, the representative peel/online identity ratio is 0.233555;
24 of 36 pricing cells cost less and 12 cost more, with range
0.233492–1.018367. Peel/rho is 22,644.844 at the representative price,
and all 36 cells exceed rho. Wall times are L0 diagnostics because the
runs shared the host with other independent work.
