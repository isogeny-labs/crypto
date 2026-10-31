# KUMMER-REF-20261022 run record

This deterministic algebraic gate used the 21-model, 1,260-target frozen
public cohort and the [pre-run protocol](../../PROTOCOL.md). It checked 8,820
target cases and 630 exceptional cases. All source scalars, transfers,
differential quotient outputs, and signed-anchor recoveries passed. The
[graph](kummer-reference-gate.svg), [PNG](kummer-reference-gate.png), and
[PDF](REPORT.pdf) render the recorded counts. The complete result and proof
scope are in the [study report](../../REPORT.md).

The [raw JSONL and standardized summary](../../results/) contain one target
row per public target, one model row per source model, and all missing price
cells. A second release-binary run into `/var/tmp/kummer-replay` matched all
14 JSONL files byte for byte. The command list, source revision, input archive
digest and exact population are in the [manifest](../../results/manifest.json).

This was a Jacobian-oracle correctness check (L0, no timed comparison).
Fast coordinate formulas and a full DLP collector remain unmeasured; their
candidate/rho ratios are null. The graph and PDF were rendered from
[plot.py](plot.py), which reads only the native summary values.
