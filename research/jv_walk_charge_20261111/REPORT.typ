#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= The isogeny walk into the weak class, charged
_Accounting item on the validated row jv_sieve_line_vs_rho_20261103; protocol frozen before the runs. Every number is an unverified assertion [pcc:unverified]._
#figure(image("walk.svg", width: 100%), caption: [Left: reach (walk + transport + model) over the route cost, per instance and start distance M. Right: pooled cover/rho at M = 512 with and without the walk.])
- *Runs:* 96 runs (24 validated instances × M ∈ {8, 32, 128, 512}), with 0 failures. Every logarithm was verified on the non-weak start curve.
- *Verdict:* walk charged, still below rho. Pooled cover/rho at M = 512 is 0.0155 / 0.0858 / 0.0900 at w = 0 / 0.1 / 1, against 0.0141 / 0.0857 / 0.0900 uncharged.
- *Tail:* the median walk costs 0.1–4 % of the route. The maximum grows with M, up to 1.48× the route.
- *Arithmetic means:* the walk adds 21 % / 1.6 % / 6.2 % at p = 1009 / 1511 / 1823.
- *Not covered:* classes without a weak curve, which include all depth-1 2-volcanoes (half of all classes, census at toy sizes). Hash-set probes are unpriced.
