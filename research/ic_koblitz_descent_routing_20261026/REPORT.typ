#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show table: set text(size: 8pt)

= Where should the planner route large Koblitz curves?

2026-10-26 · `research/ic_koblitz_descent_routing_20261026` · protocol frozen in `cd3af7078` · three ecbench sessions,
288 records, all verified, audits OK · reference `rho.signed_frobenius_strong`.

`ic.koblitz_descent` (new in `b561ebca8`) is the ledger §23 one-target Koblitz pipeline, charged in counted group operations.
On the same seeds its logs and native counts are identical to `ic price --single-target`. It ran in one session per curve against the
planner's `ic.pipeline` recipe and the strong rho.

#table(columns: 7, align: (left, right, right, right, left, left, left),
  [*curve*], [*log#sub[2] r*], [*S pipeline*], [*S descent*], [*descent / pipeline*], [*pipeline / rho*], [*descent / rho*],
  [`icv1-f2m47-t22705043-f4e44623`], [36.64], [1.312], [1.864], [1.421 [1.310, 1.541]], [9.46], [13.45],
  [`icv1-f2m41-tm2308219-7f48b14a`], [39.00], [1.966], [3.185], [1.489 [1.433, 1.547]], [16.19], [24.10],
  [`icv1-f2m53-tm56619371-dac20a85`], [44.26], [2.728], [3.845], [1.409 [1.382, 1.438]], [23.99], [33.81],
)

#figure(image("figures/routing.svg", width: 98%), caption: [Counted S per arm, and the paired ratios.])

*The planner's recipe is the cheaper one*: the descent costs 1.41–1.49× it at every size, with every interval above 1, so P1
fails and the routing stays. Rho is 9.5–34× cheaper than either.

Table A's 4.7–11.8× was mixed-unit: rho there is priced in canonical step units, while counted rho folds by 2n. The descent's cold cost
is mostly reusable precomputation. A many-target comparison needs batch rho and is a different question.

The band cannot price the descent above w = 0 until four of its native categories are counted. Engineering class; no leaderboard
row moves. Deviation: no resume checkpoint could be uploaded (no token); the owner said to proceed, and all inputs and outputs are pushed.
