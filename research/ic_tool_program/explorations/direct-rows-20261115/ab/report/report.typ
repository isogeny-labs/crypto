#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Sparse relation rows built straight from their summands: same-session A/B with its own A/A

One `ic` binary (`c54e32a0…`, built at `66e84494c8`, default features) and three labels a round:
- *a* and *q*: `KIC_DIRECT_ROWS=0`, each row built densely over every column with a `modpow` a summand, then scanned back into sparse form;
- *d*: the default, the `(column, ±λ^k mod r)` pairs pushed directly, with `λ^k` from a table built once.

The rows are the same in both routes. Debug builds assert this, and the test `direct_sparse_rows_equal_the_dense_route` checks it. Rows are suite `M1-T01` at `k0n53` and `k0n61`. There are 24 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container. The busy gate refused 87 launches, which were retried. `PROTOCOL.md` was committed before any row.

== Identity
Zero mismatches, and no round dropped: 24 rounds at each size.

== Result (median of per-round ratios a/d, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (dense / direct)], [A/A (dense / dense)], [verdict; prediction],
[k0n53], [*verify*], [*2.682* [2.585, 2.821]], [1.020 [1.007, 1.047]], [*established*; [1.9, 3.0]: held],
[k0n53], [set-up], [1.047 [0.999, 1.082]], [1.022 [0.970, 1.054]], [not established; [1.00, 1.04]: median above it],
[k0n61], [*verify*], [*2.437* [2.373, 2.499]], [0.994 [0.961, 1.034]], [*established*; [1.8, 2.8]: held],
[k0n61], [set-up], [1.011 [0.970, 1.064]], [0.996 [0.956, 1.034]], [not established; [1.00, 1.02]: held])

Verify, median over rounds: 8.76 → 3.29 ms (k0n53) and 12.68 → 5.19 ms (k0n61).

#image("ab.svg", width: 100%)

== Decision (frozen rule)
The verify gain is established at both sizes, and no set-up regression is established, so direct rows stay the default. The whole set-up gains about 5–7 ms, 1–5%, which is under this host's resolution.

== What is left in verify
Temporary timers put the group check of each collected relation at about 5 ms a repetition at k0n53 before this change. That check is now most of the verify phase.

== Scope
This is a component result for the verify phase, on this host class only. It makes no claim against rho and adds no board row (validation gate). Answers and counted work are unchanged, because row building is not counted work.
