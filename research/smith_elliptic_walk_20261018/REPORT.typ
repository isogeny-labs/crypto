#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")
#show table: set text(size: 7.5pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[W3a: the Smith walk via the elliptic 2-isogeny class]\
  #text(size: 10pt)[Frozen-protocol run · protocol frozen at `eba0a48fbf` · runs 2026-10-26 UTC
  (run-20261026T0212Z, run-20261026T0244Z)]\
  #text(size: 8.5pt)[Synthetic curves over $FF_(p^3)$ only. No claim about any standardized curve, no
  sub-rho or key-recovery claim.]
]

= Question

The problem record `problems/smith-kernel-isogeny-class-walk.md` asks what an isogeny walk costs
when it moves a genus-3 hyperelliptic curve $H$ to a member of its class where Smith's
$(2,2,2)$-construction succeeds. Hyperelliptic-to-hyperelliptic $(2,2,2)$-isogenies generically do
not exist (see `LITERATURE.md`). W3a therefore walks on the elliptic side.

- Start from $E slash K$, $K = FF_(p^3)$, admitting a Joux–Vitse / Momose–Chao $(m, r) = (3, 5)$
  descent to $H slash FF_p$.
- Move $E$ by rational 2-isogenies (Vélu).
- Re-descend, and run Smith's full test (`smith_prop24::analyze_curve`, `full_success`).

Q1 is exhaustive at small $p$. Q2 is sampled walks from $p approx 2^10$ to $2^20$, priced in $FF_p$
multiplications. Q3, the literature, is in `LITERATURE.md`.

= Q1: exhaustive at $p in {7, 11, 13, 17, 19, 23, 29, 31}$ (measured)

Every reduced weak model ($2p^2$ per $p$) was descended and tested. Every class was closed under
rational 2- and 3-isogenies. All controls had zero failures at every $p$:

- the $L$-polynomial of every $H$;
- trace equality and a dual edge on every edge;
- JV-model signatures;
- no `analyze_curve` failure.

The unreduced $p = 7$ enumeration (98,784 models) gives the same classes and signature sets. A
lost 2026-10-20 run of the same cells printed identical summaries.

#figure(image("figures/q1_shares.svg", width: 92%), caption: [Shares against $p$: $D_H$ (descended
$H$ passing), $D_E$ (weak $E$ passing), $C_2$, $C_23$, $C_t$ (weak $E$ whose 2-component,
2/3-component or same-trace class holds a passing $E$). The reference lines are Smith's 0.1857
and 0.5016 for random $H$, and the record's class share 0.43–0.475 at $p = 7, 11$. Populations:
91 to 1,886 weak $E$ per $p$; exhaustive, so there is no sampling error.])

*Findings.*

+ *Every descended $H$ has a rational tractable subgroup* (share 1.000). Only two cycle types
  occur, `1+1+3+3` and `2+6`.
  - Prop. 2 holds for 0.78–0.88 of $H$, and Prop. 4 given Prop. 2 for 0.62–0.72.
  - $D_H$ = 0.53–0.61, which is 3× Smith's 0.1857 for random curves.
  - Pre-registered E3 is falsified: it is a structural departure of the descended locus, not a
    bug.
  - The record's ceiling (0.43–0.50, measured on random $H$) does not apply to this locus.
+ *Closure is large.*
  - $C_2$ = 0.74–0.98, $C_23$ = 0.76–0.99, $C_t$ = 0.78–1.00.
  - $r_2$ = 0.68–1.00 and $r_23$ = 0.83–1.00.
  - Passing is not constant on components: 38–75% of the components with at least 2 weak curves
    are mixed. E2 holds, and the walk is not killed by invariance.
+ *Distances are short* (@dist). A failing weak $E$ is a mean 1.5–3.6 rational 2-isogenies from
  the nearest passing weak $E$, with a maximum of 3–11. With 3-isogenies added it is 1.4–2.9.
+ *The residue of $p$ mod 4 matters.* The share of failing $E$ with no passing $E$ in their
  component is 56–59% at $p = 13, 17, 29$, against 4–25% at $p equiv 3 med (mod 4)$.

#figure(image("figures/q1_distance.svg", width: 100%), caption: [Distance from each failing weak
$E$ to the nearest passing weak $E$, as shares of the failing $E$. The $n$, mean and max on the
right are over reachable ones. Measured, exhaustive.]) <dist>

#figure(image("figures/volcano_p11.svg", width: 100%), caption: [The largest mixed 2-isogeny
component at $p = 11$ (60 classes, 15 weak, 9 passing; $t = 36$), drawn with
`tools/volcano-diagram`. Levels come from the graph distance to degree-1 vertices. The label to
class-key map is in `figures/volcano_p11_labels.json`. Here the weak crater curves are JV-weak and
pass, and the weak floor curves fail.])

= Q2: sampled walks (measured; Smith cost derived)

Frozen primes are the first prime above $2^k$, $k = 10..16, 18, 20$. There are 40, 24, 12 or 6 walks
per arm and prime: 500 walks in all, from 2026-10-26T02:44 to 04:01 UTC.

- *Arm A*: from a random failing JV-split weak $E$, a non-backtracking 2-walk through
  full-2-torsion nodes until a JV-weak node's model passes.
- *Arm B*: from a random full-2-torsion curve until the first JV-weak node.

Caps: $2 dot 10^7$ tested nodes and 3,300 s per walk, plus the amended step cap and trap rule
(@amend).

#figure(image("figures/q2_walks.svg", width: 100%), caption: [Left: tested nodes per walk.
Successes are dots, censored walks are crosses. Lines show the frozen estimator (all tested nodes
over successes). Middle: cost per success in $FF_p$ multiplications (frozen, and with trapped walks
left out). Right: censoring share by cause. Data: `data/walks/walk_*.jsonl`, `fit.json`.])

*Unit costs* (`fit.json`; multiplications are instrumented):
- one step: 1.8–3.0 k $FF_p$ multiplications, growing like $log p$ because of the square root in
  $K$;
- one JV-weak test: 114;
- one descent: 2.0–4.0 k;
- one Smith call: 25–61 k (derived from wall time at 5.1 ns per multiplication; inversion = 7.6
  multiplications; shared host).

*The parity of admissible steps* [derived, and tested by
`admissible_forward_steps_follow_the_sign_of_minus_one`]. For the roots $r_0, r_1, r_2$ of a node,
$product_i (r_i - r_j)(r_i - r_l) = -product_(i<j) (r_i - r_j)^2$. So the number of squares among
the three is

- 2 or 0 when $chi(-1) = -1$, that is $p equiv 3 med (mod 4)$. Every non-start node then has
  exactly one forward edge (a non-branching path), and a start can be isolated.
- 1 or 3 when $p equiv 1 med (mod 4)$, so each node is a leaf or branches into three.

The data agree. At $p equiv 3 med (mod 4)$ there are no reflections, and arm A never revisits a
class on a successful walk. At $p = 2053, 8209, 65537$, arm A walks are trapped in closed sets of
6–30 classes: 7/40, 37/40 and 20/24 walks. Arm B has 0/24 successes at 65537.

*Scaling.*

- *The frozen rule outputs `o(p)`.* The arm A cost exponent is $-0.14$ $[-0.80, 0.95]$, with primes
  and walks resampled.
- *We do not read that output as evidence.* In the frozen estimator, a censored walk contributes
  what it had accrued when a cap stopped it. Up to $2 dot 10^7$ nodes per walk dominate the means
  at the small primes. At $p = 1031$, for example, the mean is $5.95 dot 10^5$ nodes while the
  median successful walk is 291. This flattens the slope.
- *Post hoc, labelled:* for successful walks at $p equiv 3 med (mod 4)$:
  - tested nodes scale with exponent 1.07 $[0.95, 1.19]$, and cost with 1.13 $[1.00, 1.26]$.
    The rule's label for the cost is `omega(p)`; with the $log p$ growth of the step cost this is
    $p log p$, that is $tilde(Theta)(p)$;
  - medians are $0.1$–$0.6 p$;
  - $h(p) approx (3 "to" 10) slash p$;
  - at $p = 262147$ and $1048583$, $pi$ = 0.80 (12/15 hits) and 0.55 (6/11 hits). The second
    is consistent with Q1's pass given JV (0.78–0.85) within sampling at 11 hits;
  - a non-circular check (derived): the walk's $h$ with Q1's $pi approx 0.8$ predicts
    $1 slash (0.8 h)$ = $1.19 dot 10^5$ and $3.24 dot 10^5$ nodes at $p = 262147$ and $1048583$,
    against $1.19 dot 10^5$ and $4.75 dot 10^5$ observed (ratios 1.00 and 0.68).
- Arm B, successful walks at $p equiv 3 med (mod 4)$: exponent 0.93 $[0.75, 1.10]$.

*Verdict.* The walk costs $Theta(p)$ nodes at about 2.5 k multiplications each. That is not
$o(p)$, so it does not vanish against $tilde(O)(q)$ index calculus. At $p equiv 1 med (mod 4)$
the walk as specified usually never finishes. E1 is supported.

= Amendments and deviations <amend>

`PROTOCOL.md` is unchanged. The full list is in `README.md`. In brief:

+ A step cap of $4 times$ the node cap, for reflection loops that never retest a node.
+ An isolated start (no full-2-torsion neighbour) is censored. Before this, the arm B pilot
  panicked: a control failure, fixed before data were collected.
+ A distinct-$j$ diagnostic, outside the cost counters, and a trap rule: a walk is censored once
  it has tested $>= 2 dot 10^5$ nodes with $<= 1000$ distinct $j$.
+ The fit records censoring by cause. It bootstraps over primes and then walks, as frozen; the
  old driver resampled walks only. A prime with no success gives "undetermined". Post hoc
  variants are labelled.
+ Host: an Apple M4 Pro with 14 cores, shared and without pinning (cores 29 and 30 do not exist),
  running $<= 4$ processes at nice 10. Wall time and the derived Smith cost are noisy.
+ Calibration: the $p = 65537$ row prices every $p$.
+ "Nodes visited" means tested nodes, counting retests.
+ Q1 was extended from the brief's $\{7, 11, 13\}$ to the protocol's full list.
+ All data come from the 2026-10-25/26 rerun. The 2026-10-20 outputs were lost.

= What this establishes and what not

*Established on this locus and range:*

- the pass rates;
- isogeny closure and distances (exhaustive, $p <= 31$);
- per-operation costs;
- $Theta(p)$ walk length for the walks that finish (post hoc, at six primes $equiv 3 med
  (mod 4)$ up to $2^20$);
- frequent trapping at $p equiv 1 med (mod 4)$.

*Not established:*

- whether some other walk policy avoids the traps, for example admitting steps through
  non-full-2-torsion nodes;
- whether weak or passing curves cluster beyond density $1 slash p$, which is what a cheaper walk
  would need;
- why the descended locus passes Smith at $tilde 0.55$;
- any statement beyond $p approx 2^20$.

*Next step:* measure clustering of weak and passing curves in the isogeny graph. General
$(3,5)$-weak curves are sparse too: $2p^2$ models against about $2p^3$ $K$-classes, so their
density is about $1 slash p$. At that density the nearest weak node is about $log_2 p$ steps away,
and a ball of that radius holds about $p$ nodes. A general weakness test plus a short search would
therefore beat $Theta(p)$ only if weak or passing curves cluster beyond $1 slash p$.

The Q1 data give a first, derived reading. The weak share of the explored components falls from
0.54 at $p = 7$ to 0.13 at $p = 31$. The global share is $91 slash 686 approx 0.13$ and
$1886 slash 59582 approx 0.03$ (with $2p^3$ classes), so the explored components are about 4×
enriched at $p = 31$. The mean distance rises from 1.5 to 3.6.

The discriminating step is to test whether that enrichment persists as $p$ grows, and whether
$d_2$ grows like $log p$, on larger exhaustive graphs or sampled components. Only then is a
general weakness test worth building.
