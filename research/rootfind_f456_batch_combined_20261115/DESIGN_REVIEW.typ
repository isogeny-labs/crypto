#set page(paper: "us-letter", margin: (x: 0.78in, y: 0.68in))
#set text(font: "Helvetica Neue", size: 10pt)
#set heading(numbering: none)

= K13 combined panel: before-data design

_Frozen protocol; no execution or outcome row. 2026-11-15 PST._

The earlier fixed panels answer two separate questions: scalar rootfinding
against F4/F5/F6, and batched against scalar rootfinding. Their results
cannot be pooled into a same-source batch-versus-F4 comparison. This ten-arm
panel places all methods on the same eight registered public targets, binary,
four-CPU host, and interleaved rounds.

#figure(
  image("panel.svg", width: 100%),
  caption: [Matched arms, primary wall gate, secondary counted-cost audit.],
)

== Decision and uncertainty

Each accepted process must verify its scalar, contain complete cold phase
timings, and earn L2 or higher isolation. Batched m3 must have lower wall
time on every matched target and round against each of four F4/F5/F6 arms.
Five declared comparisons use target-cluster bootstrap 99% intervals; the
batch m3 A/A 95% interval must contain one. A decisive interval must finish
below the A/A lower endpoint. Failures and censored rows remain in the export.

== Cost and admission boundary

Bitop/v1 and GAE bands stay visible, with unpriced native work reported;
incomplete counted costs cannot support a counted ranking. The monthly AWS
limit is already exceeded, so the fixed EKS L2 job has not started. The
exact-source binary and publication receipt are still preparation gates.

Internal sources: `PROTOCOL.md`, `protocol.json`, `SPEC-K13.json`,
`analyze.py`, and the prior K13 session named in `RUN_STATUS.md`.
