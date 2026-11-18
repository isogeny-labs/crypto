#set page(paper: "us-letter", margin: (x: 18mm, y: 15mm))
#set text(font: "New Computer Modern", size: 9pt)
#set par(justify: true)

= Reused-setup dual-isogeny fibers on 64 public targets

*Measured 2026-11-18 17:01:06–17:01:11 UTC.* The frozen V1 protocol and
pre-run V2 scheduling correction produced 18/18 complete batch children,
18/18 independent replays, and 1,152 verified target results. The native
pair audit matched 192 dual/inverse cells and 76,711 relation witnesses.
The SHA-256 manifest verified after the run.

The source curve is $y^2 = x^3 - 21x + 37$ and the different-$j$ destination
is $y^2 = x^3 - 81x + 179$ over $F_(211^2)$. The held-out subgroup has prime
order 229. The dual candidate received a witness-free destination base;
the 64-target public file contains neither scalar answers nor the selection
seed. The checker held those separately. The first four public targets match
the prior held-out points; the other 60 are distinct.

== One-unit whole-batch comparison

$S_"group"$ is all counted setup, query, and final-verification additions
plus doublings in a child divided by $64 sqrt(229)$. Entries are means
across three completed rounds. It is a partial operation view. The
prespecified generic two-class *single-target* floor is
$sqrt(pi / 4) = 0.886227$ in its corresponding single-target unit.

#table(
  columns: (2.2fr, 1fr, 1.2fr),
  inset: 4pt,
  [*Arm*], [*Targets*], [*Mean $S_"group"$*],
  [Direct dual], [192/192], [1,714.111],
  [Direct inverse Vélu], [192/192], [1,714.442],
  [Destination direct], [192/192], [1,712.437],
  [Source BSGS], [192/192], [1.659],
  [Signed rho], [192/192], [6.654],
  [Signed rho A/A], [192/192], [6.654],
)

#figure(
  image("S_GROUP_GRAPH_V1.svg", width: 75%),
  caption: [Mean whole-batch counted group work per target, logarithmic axis;
  three rounds on the same 64 targets. Full precision is in
  `RUN_V1/SUMMARY_V1.json`.],
)

== Phase observation and limits

The mean whole-batch group counts were 1,660,109 dual, 1,660,430 inverse,
1,658,488 destination direct, 1,607 BSGS, and 6,444 signed rho. Both
source relation arms used exactly 1,646,809 query operations and 1,120
verification operations. Dual setup used 12,180; inverse setup used 12,501.
Thus dual saved 321 counted setup operations per batch. The measured
inverse/dual whole-batch counted ratio is 1.000193; dual/rho is 257.621.
These are partial count ratios. The factor-base columns, relation rows,
ranks, and recovered answers matched in paired controls.

The host was a shared Darwin M4 Pro at L0. Mean child thread CPU was
370.987 ms dual, 378.721 ms inverse, and 2.921 ms rho; no isolated wall
ratio is admitted. Complete portable $S$ is unset because field and
square-root internals, table and polynomial work, allocation, startup,
and I/O lack disjoint calibration. Larger-order dual/inverse trend and
the independent primitive-counter gate remain open. The required release
library test hit a duplicate test declaration already in the current base
commit; touched example tests passed. The Markdown result and run reports
link the raw children, replay, host, hashes, and validation receipts.
