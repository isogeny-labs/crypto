#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set table(stroke: 0.4pt)
#show heading: set block(above: 1.1em, below: 0.6em)

= Two orbit columns under the lazy member index

2026-10-18. Study `research/ecbench_lazy_columns2_20261018/`, protocol `PROTOCOL.md` (frozen in commit `760c672a5` before any claim data). Class: *engineering*. Stacked on PR \#2158 (oracle `mitm-frobenius-lazy-counted`). Sessions ran on commit `760c672a5` (`git_dirty: false`), `ecbench` binary `ae108d54…`, 4-vCPU Xeon VM, isolation L0 (counts stand; wall times descriptive only).

== Result

*Question:* now that the lazy oracle has removed the member-index set-up that grew with the number of columns, is `columns = 2` cheaper than the per-curve column count frozen on 2026-10-14 under the eager oracle?

*Measured (counted `ecbench.gae`, every counter priced, paired by target, 95 % cluster-bootstrap):* on one curve of five. Two columns is cheaper at $n = 17$, $a = 1$ (frozen $c = 1$): 1.176 [1.063, 1.297]. It is dearer on the other four: $n = 13$ (frozen $c = 1$) 0.637 [0.630, 0.645]; $n = 23$ ($c = 3$) 0.778 [0.695, 0.874]; $n = 29$, $a = 1$ ($c = 1$) 0.644 [0.625, 0.665]; $n = 37$ ($c = 8$) 0.246 [0.200, 0.309]. Ratios are $S$(frozen $c$) ÷ $S$($c = 2$), above 1 meaning two columns is cheaper. The frozen column counts, chosen under the eager oracle, stay the better choice on four of the five curves under the lazy oracle.

All 2 880 records verified, none a lower bound, every audit passed with all replays reproduced (`audits/`), and `rho-frob-aa ÷ rho-frob` is 1 [1, 1] on every curve.

#figure(image("figures/fig1-ratio.svg", width: 100%), caption: [Frozen-$c$ ÷ two-column paired cost ratio per curve. Sessions ECBS1h5f7f47653898 (n13), ECBS1had6c4dfa75fc (n17), ECBS1h88a016cfa7c0 (n23), ECBS1h486ae2ee8d06 (n29), ECBS1hbef0622fd5f3 (n37).])

#figure(
  table(columns: 7, align: (left, right, right, right, right, right, right),
    [curve (a, n)], [frozen $c$], [$S$, frozen $c$], [$S$, $c = 2$], [frozen ÷ $c$2 (H1)], [$c$2 ÷ rho-strong], [frozen ÷ rho-strong],
    [0, 13], [1], [4.667], [7.328], [0.637 [0.630, 0.645]], [0.510 [0.503, 0.516]], [0.325 [0.319, 0.330]],
    [1, 17], [1], [2.483], [2.112], [1.176 [1.063, 1.297]], [0.512 [0.490, 0.535]], [0.602 [0.545, 0.665]],
    [0, 23], [3], [1.265], [1.627], [0.778 [0.695, 0.874]], [1.403 [1.236, 1.578]], [1.091 [1.009, 1.176]],
    [1, 29], [1], [1.830], [2.840], [0.644 [0.625, 0.665]], [0.615 [0.602, 0.627]], [0.396 [0.381, 0.411]],
    [0, 37], [8], [1.044], [4.239], [0.246 [0.200, 0.309]], [15.42 [12.28, 18.61]], [3.798 [3.266, 4.403]],
  ),
  caption: [Mean $S$ (`ecbench table`) and paired ratios (`ecbench compare`, `sessions/*/comparisons/`). 96 pairs per curve (32 targets × 3 rounds); n37: 48. Columns 6 and 7 are cost ÷ the matched strong rho, below 1 meaning the IC arm is cheaper.],
)

== What changes (H3)

Two columns need fewer trials than one column (n13, n17, n29) and more than three or eight (n23, n37). Mean trials per solve, frozen $c$ against $c = 2$ (derived: mean of the records' `targets_tried`): n13 12.8 vs 5.8; n17 280.5 vs 89.7; n29 53.5 vs 20.5; n23 802.3 vs 1 415; n37 9 565 vs 53 115. Mean relations (= rank) per solve: 2.0 for $c = 1$, 3.6 for $c = 3$, 7.3 for $c = 8$, 2.9 for $c = 2$. Fewer trials pay for the wider base at n17 only; at n13 and n29 the extra relations cost more than the trials saved.

== Distance to rho (reported, not a gate)

`ic-lazy-c2 ÷ rho-strong` is below 1 at n13, n17 and n29 and above 1 at n23 (1.403) and n37 (15.4). The frozen choice is below 1 at n13, n17 and n29 and above 1 at n23 (1.091 [1.009, 1.176]) and n37 (3.80). No ranking against rho is claimed.

#figure(image("figures/fig2-S.svg", width: 100%), caption: [Mean $S$ per arm, 95 % interval from `ecbench table`.])

== Classification and limits

*Engineering.* One improvement, on one curve ($n = 17$, $a = 1$, frozen $c = 1$ to $c = 2$, 1.18×), among five pre-declared comparisons; four regress. Choosing $c = 2$ at $n = 17$ after this run is a tuning result on these seeds and needs fresh seeds before it is a rule (protocol §6). The exploratory sweep that suggested two columns (sessions not kept) compared $c = 2$ with $c = 3, 4, 5, 6$ and did not try $c = 1$; the claim shows the frozen $c = 1$ beats $c = 2$ at $n = 13$ and $n = 29$. No exponent, ECC2K-130 or deployed-curve statement follows.

*Evidence:* `sessions/` (5 ecbench sessions, loose), `audits/`, `tables/`, `logs/`, `figures/plot_figures.py` (lays out recorded values only).
