#set page(paper: "us-letter", margin: 0.64in)
#set text(size: 9pt)
#set par(leading: 0.5em)
#text(size: 16pt, weight: "bold")[ECCp-131 relation-screen benchmark]
#linebreak()
_23 October 2026 | exact published curve and target | nine fixed cells_

The certified auxiliary tower's 115 lifted columns and a 115-column small-abscissa control were screened with identical scalar candidates. The small base was repeated twice per repetition to expose host timing variation. Each process screened 256 setup and 256 published-target points with the same complete three-point lookup, eight planted controls and 30 s / 2 GiB watchdog. The protocol was frozen before execution.

#figure(image("bench_screen.svg", width: 6.6in), caption: [Screen-stage wall time. Each dot is one repetition; horizontal bars are medians. The two small-base arms are identical inputs.])

#table(columns: (1.45in, 1.55in, 1.55in, 1.35in),
  [Arm], [Screen median (range), s], [Process median (range), s], [EC additions],
  [Tower], [2.343 (2.100–2.906)], [2.421 (2.231–3.049)], [270,564],
  [Small A], [1.795 (1.752–2.165)], [2.007 (1.978–2.406)], [270,564],
  [Small A repeat], [2.160 (1.730–2.203)], [2.228 (1.817–2.451)], [270,564],
)

All nine cells exited normally. The candidate scalar digest matched across cells; source and base digests, 115 columns, 26,450 signed pair points, and eight planted equations were checked. Natural setup rows, published-target rows, useful rank and target scalar were zero/null in every cell. Sampled RSS ranged from 32,720 to 33,520 KiB. The repeated identical base varied substantially, and operation counts were equal, so these runs do not establish a base-dependent speedup.

This is a screen-stage diagnostic. The previous 301-curve auxiliary search and tower construction are separate costs. No matched rho run or full cold IC accounting was made. At arity three, the 115-column complete support contains at most 6,083,730 points including the identity pair. An unplanted full-parameter relation still requires a larger base/arity or another oracle; independent replay, useful rank and published-target descent remain the gates.

*Evidence:* `BENCH_PROTOCOL.md`, `BENCH_REPORT.md`, `runs/bench-r*.json`, corresponding receipts and stderr, `runs/bench-audit.json`, and `plot_bench.py`. Source revision `1b869e62197782465c53b05dd875d504c58e951a`; Python 3.13.1, Darwin 25.6.0 arm64 T6041. The prior `REPORT.md` contains the exact-curve construction and independent GP replay.
