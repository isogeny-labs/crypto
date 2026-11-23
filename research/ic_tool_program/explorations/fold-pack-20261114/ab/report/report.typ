#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= One packed stream a partition in the folded build: same-session A/B with its own A/A

One `ic` binary (`332b380f…`, built at `9ffb571245` with `--features scan-probes`) and three labels a round:
- *a* and *q*: `KIC_FOLD_PACK=0`, two streams a partition (a hash and a stored word);
- *p*: the default, one `u64` a pair, with the orbit in hash bits 34–37 and the partition's top eight.

Rows are suite `M1-T01` at `k0n53` (2^44.3) and `k0n61` (2^47.2). There are 42 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container (Intel Xeon @ 2.1 GHz, AVX-512). The busy gate refused 132 launches, which were retried and are not measurements. `PROTOCOL.md` was committed before any row. Tables are identical word for word, by construction and by test.

== Identity
Zero mismatches. One k0n53 round was dropped as contended, leaving 41 and 42 rounds.

== Result (median of per-round ratios a/p, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (two / packed)], [A/A (two / two)], [verdict; prediction],
[k0n53], [*build*], [*1.244* [1.214, 1.276]], [0.994 [0.971, 1.017]], [*established*; [1.08, 1.25]: held],
[k0n53], [*set-up*], [*1.068* [1.051, 1.080]], [1.005 [0.981, 1.016]], [*established*; [1.00, 1.07]: held],
[k0n53], [collect], [0.981 [0.972, 1.004]], [0.985 [0.973, 1.012]], [no regression; [0.97, 1.03]: held],
[k0n61], [*build*], [*1.252* [1.223, 1.266]], [0.999 [0.990, 1.015]], [*established*; [1.08, 1.25]: missed high],
[k0n61], [set-up], [1.038 [1.020, 1.050]], [1.013 [0.994, 1.025]], [not established (1.020 < 1.025); [1.00, 1.04]: held],
[k0n61], [collect], [1.009 [0.987, 1.019]], [1.007 [0.992, 1.023]], [no regression; [0.97, 1.03]: held])

#image("ab.svg", width: 100%)

== Mechanism: build steps, ns per stored pair (median over rounds)
#table(columns: 5, align: (left, right, right, right, right),
[step], [k0n53 two], [k0n53 packed], [k0n61 two], [k0n61 packed],
[stream push], [13.28], [*7.11*], [14.83], [*7.90*],
[counting sort + filter fill], [9.04], [9.39], [9.43], [10.00],
[additions + keys], [7.33], [7.29], [7.95], [8.03],
[whole], [31.25], [24.70], [33.08], [26.89])
The push halves. The fill pays about 0.4–0.6 ns to rebuild each stored word from its hash.

== Decision (frozen rule)
The build gain is established at both sizes, and no collect regression is established. So packing stays the default. The whole set-up is established faster at k0n53 (6.8%), the first set-up gain in this series to clear its A/A. At k0n61 its lower end (1.020) misses the A/A's upper end (1.025).

== Scope
A component result on this host class. It is not a cold-time claim against rho, and no board row follows from it (validation gate). Tables, answers and counted work are unchanged, so no recorded session's replay changes.
