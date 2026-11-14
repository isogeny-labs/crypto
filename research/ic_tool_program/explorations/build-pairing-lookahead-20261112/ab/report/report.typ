#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Build-side pairing and filter lookahead 64: same-session A/B with its own A/A

One `ic` binary a block, built from `62f1f0a778` with `--features scan-probes`: `7e44ec2a…` for rounds 1–38 (k0n53) and 1–37 (k0n61), and `e7ffd03a…`, rebuilt from the same commit after the container was replaced, for the rest (amendment 1, declared before those rows).

Four labels a round:
- *a* and *q*: `KIC_BUILD_PAIRED=0`, lookahead 32 (today's path);
- *b*: build pairing on;
- *l*: `KIC_BUILD_PAIRED=0`, `KIC_LOOKAHEAD=64`.

Rows are suite `M1-T01` at `k0n53` (2^44.3) and `k0n61` (2^47.2). There are 44 rounds, with the order in a cyclic Latin square. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container (Intel Xeon, AVX-512). The busy gate refused 213 launches, which were retried and are not measurements. `PROTOCOL.md` was committed before any row. Class: engineering. Every output is identical by construction and by check.

== Identity
Zero mismatches: every process of a row gave the same certificates and counts. No round was dropped.

== Result (median of per-round ratios, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [ratio], [A/B], [A/A (a/q)], [verdict; prediction],
[k0n53], [build a/b], [*1.044* [1.028, 1.064]], [1.005 [0.992, 1.029]], [not established (lower end 0.001 below the A/A's upper end); [1.00, 1.03]: missed high],
[k0n53], [set-up a/b], [1.016 [0.989, 1.028]], [1.005 [0.997, 1.014]], [not established; [1.00, 1.01]],
[k0n53], [collect a/l], [0.997 [0.978, 1.015]], [1.002 [0.978, 1.010]], [not established; [0.98, 1.04]: held],
[k0n53], [set-up a/l], [1.002 [0.973, 1.013]], [1.005 [0.997, 1.014]], [not established; [0.98, 1.03]: held],
[k0n61], [build a/b], [1.036 [0.991, 1.068]], [1.007 [0.993, 1.019]], [not established; [1.00, 1.04]: held],
[k0n61], [set-up a/b], [0.996 [0.968, 1.022]], [1.003 [0.987, 1.018]], [not established; [1.00, 1.01]],
[k0n61], [collect a/l], [0.997 [0.966, 1.055]], [1.001 [0.983, 1.010]], [not established; [0.98, 1.05]: held],
[k0n61], [set-up a/l], [0.993 [0.972, 1.045]], [1.003 [0.987, 1.018]], [not established; [0.98, 1.04]: held])

*Sensitivity (amendment 1; not used for the decision):* the rounds before the restart only (38 and 37) give the same verdicts. Build a/b is 1.047 [1.028, 1.066] at k0n53, against an A/A upper end of 1.035.

#image("ab.svg", width: 100%)

== Reading
- *Pairing in the build:* the build is about 4% faster at both sizes (median 1.044 and 1.036), more than the arithmetic predicted (about 2%).
  - It is not established at either size: at k0n53 the interval misses the A/A bound by 0.001, and at k0n61 it touches 1.
  - On the whole set-up it is invisible, because the build is about 30% of set-up at k0n53 and about 15% at k0n61.
- *Lookahead 64:* no effect on collect or set-up at either size. The medians are within 0.3% of 1.
- *Decision (frozen rule):* neither lever's primary metric is established on any row, so both are reverted from code, with their switches. The patch stays in the exploration directory.
- *Scope:* a component result on this host class. It is not a cold-time claim against rho, and no board row follows from it.

== Deviations, disclosed
- The container was replaced mid-run. Rows on disk survived and were committed before any further row.
- The binary was rebuilt from the same commit with a different hash, on a node reporting a different nominal clock.
- Two old-binary processes of k0n61 round 38 were set aside, so that no round mixes binaries.
