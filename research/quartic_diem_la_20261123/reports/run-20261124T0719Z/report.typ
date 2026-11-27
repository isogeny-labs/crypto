#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt, inset: 3.5pt)
= Quartic Diem 2LP IC: SGE + coordinate Wiedemann on a square subsystem vs scalar Wiedemann

Run report for study `research/quartic_diem_la_20261123`, runs on 2026-11-24 from 07:19 to 07:39 UTC. *Every number is \[pcc:unverified\]*: one shared host, planted targets, and no replay certificate or validator report yet.

*Binding.* PROTOCOL.md was frozen in 612f8dee20e before any grid-prime run. Its parameters (W = 8, FILL = 1.5, EXCESS = 64) were fixed on development curves; the rule amendment made from development data is recorded in section 4. The binary `genus_ladder.qla` has sha256 `adddfbfa…d6d1f6` and was built from 4faca82a6f0.

*What ran.* 4 primes (4099, 16411, 65537, 131101), with the parent ladder's 5 prime-order quartics per prime and solver seeds 1 and 2. Both arms ran on every instance, 80 runs in total. All runs certified, both arms recovered the same logarithm, and C0 holds: all 40 `2lp-sfb` rows are identical to the ladder's.

*Reference.* Ideal negation rho, $sqrt(pi r slash 4)$ group additions. GAE#sub[w] = group_ops + w · (all counted native operations), for w ∈ {0, 0.1, 1}.

== Decision (frozen rule): IMPROVEMENT, a component speed-up
#table(columns: 3,
[w], [pooled paired GAE ratio sge/sfb], [95% (cluster bootstrap)],
[1], [0.371], [\[0.340, 0.406\]],
[0.1], [0.372], [\[0.340, 0.407\]],
[0], [1.0028], [\[1.0010, 1.0047\] (tolerance ≤ 1.01)],
)

#figure(image("fig_la.svg", width: 100%), caption: [Left: IC / ideal negation rho per arm and weight, geometric mean over 5 curves with boot95 intervals. Right: paired ratio sge/sfb on the same instances and seeds.])

== IC / rho
#table(columns: 5,
[p], [sfb w=0.1], [sge w=0.1], [sfb w=1], [sge w=1],
[4099], [2.38], [0.896], [23.8], [8.94],
[16411], [1.76], [0.760], [17.6], [7.59],
[65537], [0.896], [0.294], [8.96], [2.93],
[131101], [0.775], [0.278 \[0.202, 0.382\]], [7.75], [2.78 \[2.02, 3.80\]],
)
Every prime is UNRESOLVED for both arms (below rho at w = 0, above at w = 1). The distance remaining at 131101, w = 1, is 2.78×, down from 7.75×.

== Where the cost is now
- Runs that needed a second attempt carry 46–83% of the sge arm's linear algebra at the three largest primes. In those runs the fully counted fallback repeats a whole-core solve that also fails.
- Single-attempt pairs have `la` ratios of 0.17–0.20.
- The next lead needs a new protocol: detect failure from the square subsystem and skip the fallback.
