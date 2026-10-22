# First-target 28-bit pencil price records

The [frozen protocol](../../PROTOCOL.md) fixed the original p=48,383
screen target for identity, reverse and affine column orders. The separately
frozen degree-peeling alternative used the same target and identity order.
Two adjacent archived 28-bit models, p=48,823 and p=49,451, each supplied
their first frozen target under the original online identity solver. All six
candidate and same-process rho scalars were source checked. These are
one-target records; they do not describe the other 59 targets per model.

The [PDF](RUN_REPORT.pdf) and [graph](p28-first-targets.svg) show complete
representative operation prices against rho and p=48,383 paired costs
against the online identity solver. [Every target and pricing cell](../../results/price-grid.csv),
the [paired peel grid](../../results/p48383-peel-vs-online.csv), and the
`../../results/p*-t0.jsonl` raw records retain the individual costs and
counter evidence. The [native validator](../../results/validation.json)
checks source scalars, archived workload IDs, matched rho controls,
non-wall replays and every price cell after the replay processes finish.

At p=48,383, the representative peel/online identity ratio is 0.233555;
24 of 36 pricing cells cost less and 12 cost more, with range
0.233492–1.018367. Peel/rho is 22,644.844 at the representative price,
and all 36 cells exceed rho. Wall times are L0 diagnostics because the
runs shared the host with other independent work.
