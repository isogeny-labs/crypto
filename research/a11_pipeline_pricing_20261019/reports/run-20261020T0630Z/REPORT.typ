#set document(title: "A11 algebraic-oracle paper bound", author: "a11_oracle_paper_bound")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= A11 as an algebraic decomposition oracle: a paper bound (run-20261020T0630Z)

*Kind:* paper bound from committed rows, not a measurement. It prices no pipeline and reports no S. It answers one question before anyone builds the algebraic oracle of #raw("FEASIBILITY.md") (1.5–3 engineer-weeks): could the cheapest presentation measured so far, A11, ever compete per discrete logarithm with the group-law oracle ecbench already runs?

== Inputs (all committed)

- *Group-law arm, measured:* #raw("research/ic_frontier_campaign_20261015/phase1/sessions/PKM20/records.jsonl"): #raw("ic.pipeline"), #raw("prime-kummer:t=8"), #raw("rootfind-mitm:m=3"), p = 946177, r = 473453, 32 verified solves, with the matched #raw("rho.negation") arm (32 verified).
- *Algebraic side:* #raw("research/factor_base_designs_20261015/runs/C1-rescore.jsonl"): F4 row-reduction field multiplications per system for A11 and A0 at m = 3, every system replayed with a matched trace. The trace was identical on p#sub[1] and p#sub[2] (Amendment 2), and A11's t = 4 count differs by 3·10#super[-6] between them, so p#sub[1] counts are used at PKM20's prime.
- *Computation:* native, #raw("examples/a11_oracle_paper_bound.rs") (#raw("cargo run --release --example a11_oracle_paper_bound")). Output: #raw("paper_bound.txt").

== The bound, and why it favours A11

Per relation the algebraic oracle costs F/(e·P) GAE and per discrete logarithm (N + 1)·F/(e·P), with:
- F, the mean F4 row-reduction multiplications per system. Symbolic preprocessing, root finding and lifting are not counted.
- e ∈ {1, 12, 50}, field multiplications per group addition. The comparison uses e = 50, the cheapest for A11.
- P = 1 − exp(−8·C(N, 3)/r), N = 2#super[t−1] abscissae on the curve, and one relation per success. At t = 8 this gives 0.997; PKM20 measured about 0.96 relations per trial with 123 abscissae.
- Only N + 1 relations per logarithm: no linear algebra and no setup.
- For t > 5 (not measured), F is held at its t = 5 value although it rose with every measured bit. This is an extrapolation, drawn open and dashed.

The group-law arm's unpriced counters (square roots, lookups, row operations, Legendre symbols, target-guard probes) are priced at 0, 0.1 and 1 GAE each (AGENTS.md §5a). Rho's unpriced canonicalisations are not emitted as a count, so rho is a lower bound and context only; the verdict does not rank against it.

#figure(image("bound_vs_t.svg", width: 100%),
  caption: [GAE per discrete logarithm against t at m = 3. Filled markers: algebraic lower bounds from measured F4 counts. Open, dashed: A11 with t > 5 held at its t = 5 count (extrapolation). Band: the measured group-law IC arm across w ∈ {0, 0.1, 1}. Dotted: matched rho (lower bound). From #raw("paper_bound.txt"), plotted by #raw("plot.py").])

== Numbers (from #raw("paper_bound.txt"))

#table(columns: 4, align: (left, center, center, center), stroke: 0.4pt + gray,
  [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [group-law IC, GAE per DLP (mean of 32)], [27 015], [29 332], [50 187],
  [group-law IC, GAE per relation], [244.9], [265.9], [454.6],
  [matched rho, GAE per DLP (lower bound)], [892], [892], [892],
  [cheapest algebraic lower bound (A11, t = 3, e = 50)], [7.085·10#super[9]], [7.085·10#super[9]], [7.085·10#super[9]],
  [ratio algebraic bound / group-law IC], [2.62·10#super[5]], [2.42·10#super[5]], [1.41·10#super[5]],
)

The cheapest algebraic point is A11 at t = 3 (F = 4.79·10#super[6], P = 6.8·10#super[-5]). At larger t, P approaches 1 but F grows faster: 8.6·10#super[8] at t = 4 and 5.2·10#super[11] at t = 5. Every A11 and A0 point, measured or held, lies above 7·10#super[9] GAE per logarithm.

== Verdict

Across the whole band, even the most generous lower bound for an A11 algebraic oracle costs at least 1.4·10#super[5] times more per discrete logarithm than the group-law oracle ecbench already runs on the same factor base and curve. A misestimate of P or e by three orders of magnitude would not change the order. On this evidence the 1.5–3-week algebraic oracle is not worth building to price A11. The A11 thread closes at "a constant-factor gain inside F4 that does not reach the pipeline".

== Limits

- A bound, not a measurement. Its terms are stated above, each chosen to favour A11.
- One curve size (p ≈ 2#super[20]), m = 3. The group-law arm is at t = 8 only. Its cost per relation is far below any algebraic point, so a t-matched group-law arm would not change the order.
- Rho is a lower bound (unpriced canonicalisations not emitted) and is not ranked against.
