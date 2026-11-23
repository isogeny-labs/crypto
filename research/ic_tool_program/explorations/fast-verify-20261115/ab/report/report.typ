#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Single-word relation check in the log solver: same-session A/B with its own A/A

One `ic` binary (`836c601b…`, built at `d1290ca405`, default features) and three labels a round:
- *a* and *q*: `KIC_FAST_VERIFY=0`, each relation summed with the generic `point_add`, which inverts by Fermat's little theorem over multi-word field elements;
- *f*: the default, which sums the same points in the same order with the single-word `FastCurve::add`.

Verdicts and counted additions, doublings and scalar multiplications are the same on both routes. The test `fast_relation_checks_match_the_generic_verdicts_and_counts` checks this. Rows are suite `M1-T01` at `k0n53` and `k0n61`. There are 24 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container. The busy gate refused 80 launches, which were retried. `PROTOCOL.md` was committed before any row.

== Identity
Zero mismatches, and no round dropped: 24 rounds at each size.

== Result (median of per-round ratios a/f, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (generic / fast)], [A/A (generic / generic)], [verdict; prediction],
[k0n53], [*verify*], [*2.099* [2.047, 2.177]], [1.006 [0.945, 1.031]], [*established*; [1.4, 2.2]: held],
[k0n53], [set-up], [0.980 [0.952, 0.999]], [0.989 [0.947, 1.016]], [not established, no regression by the rule; [1.00, 1.03]: missed (below)],
[k0n61], [*verify*], [*2.268* [2.206, 2.309]], [0.998 [0.981, 1.016]], [*established*; [1.3, 2.0]: missed (above)],
[k0n61], [set-up], [1.008 [0.978, 1.023]], [1.003 [0.968, 1.025]], [not established; [1.00, 1.01]: held])

Verify, median over rounds: 3.34 → 1.61 ms (k0n53) and 5.24 → 2.31 ms (k0n61).

#image("ab.svg", width: 100%)

== Decision (frozen rule)
The verify gain is established at both sizes, and no set-up regression is established, so the fast check stays the default.

At k0n53 the whole set-up interval ends just below 1, at 0.999. It does not fall below the A/A's lower end, 0.947, so the rule does not call it a regression. The verify saving is about 1.7 ms of roughly 300 ms of set-up, under this host's resolution. Read the set-up ratio as noise of the A/A's width, not as an effect of the change.

== Scope
This is a component result for the verify phase, on this host class only. It makes no claim against rho and adds no board row (validation gate). Answers and counted work are unchanged.
