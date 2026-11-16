# First 32-bit pencil target under online and peel elimination

The [frozen protocol](../../PROTOCOL.md) fixed p=186,379 target 0,
identity order, the 10,000,000 stored-coefficient cap, and a 3,600-second
external wall budget. Both solver runs returned the explicit
`sparse_matrix_cap` status. Both same-process rho controls recovered the
source scalar. The candidate scalar and candidate/rho ratio are null for
both attempts.

The [PDF](RUN_REPORT.pdf) and [graph](upper-two-caps.svg) compare recorded
peak coefficients and modular row updates. The source records are
`../../results/p186379-identity-t0.jsonl` and
`../../results/p186379-peel-t0.jsonl`; the latter also records
971,612,198 logical index operations. No full-cost victory is inferred from
the stage counts of unsuccessful attempts. The raw replays and native
validator record their independent checks. Wall times have L0 isolation.
