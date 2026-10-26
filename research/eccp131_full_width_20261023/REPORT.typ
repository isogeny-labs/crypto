#set page(paper: "us-letter", margin: 0.65in)
#set text(size: 9pt)
#set par(leading: 0.5em)
#text(size: 16pt, weight: "bold")[ECCp-131 auxiliary tower]
#linebreak()
_23 October 2026 | bounded construction and relation screen_

An eight-step rational 2-isogeny tower was built on an auxiliary curve over the exact published ECCp-131 field. Its 256 coset abscissae were checked independently, and 115 lifted onto the published curve. The frozen 256 setup and 256 published-target candidates yielded zero natural rows and rank zero. The full relation, rank and target-descent chain remains open.

== Checked construction

The protocol scanned $E'_b: y^2 = x^3 + x + b$ for $b=1 dots 512$. Candidate $b=301$ yielded a point $Q$ of exact order 256. Independent Python integer EC arithmetic checked $128Q != O$, $256Q = O$, all coset points, eight Vélu kernels and codomains, and one common terminal x-image. PARI/GP checked the published curve order, generator and target, and separately replayed all eight planted point equations.

#table(columns: (2.8in, 2.0in),
  [Auxiliary orders computed], [301],
  [Orders divisible by 256], [2 (candidates 278, 301)],
  [Distinct coset abscissae / map checks], [256 / 256],
  [Published-curve lifted columns], [115],
  [Distinct signed pair points], [26,450],
  [Planted controls], [8 / 8],
  [Setup rows / rank], [0 / 0],
  [Published-target rows / rank], [0 / 0],
)

#figure(image("order_valuations.svg", width: 5.9in), caption: [Two-adic valuations of the 301 auxiliary orders in `runs/search.stdout`. Dashed line: divisibility by 256.])

== Bounded execution and limit

The GP phase took 83.927 s wall with 15,584 KiB maximum sampled RSS. The independent replay and screen took 8.276 s wall with 35,280 KiB maximum sampled RSS. Both exited 0 below the frozen 120/60 s wall and 2 GiB RSS limits. Mac16,7, PARI/GP 2.17.3, Python 3.13.1. Receipts sample RSS every 0.2 s.

The 230 signed lifted points and 26,450 distinct nonidentity pair points imply at most 6,083,500 sums for the implemented oracle. Including an identity pair gives a general three-point bound of 6,083,730 points. Under uniform sampling the per-point upper bound is $3.925 times 10^(-33)$, and the 512-point union bound is $2.010 times 10^(-30)$. The next natural-row experiment needs a larger certified base and arity or a different relation mechanism, followed by independent row replay, rank, published-target descent and complete cold cost.

== Provenance

`PROTOCOL.md`, `search.gp`, `replay_and_screen.py` and `run_bounded.py` are source. `runs/search.stdout`, `runs/replay.json`, GP replays, stderr files and receipts are raw evidence; `audit.py` cross-checks them. Local release library tests: 4,477 passed, zero failed; gate tests: 49 passed. `REPORT.md` states commands and requirement status. Source base: `c813aea6044bdc88ec4a1af3a659e8e538798a00`. An earlier T7 worktree disconnected before its uncommitted output could be retained; every number here is from the preserved rerun.
