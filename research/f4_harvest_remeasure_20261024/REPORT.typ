#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)

= Re-measuring the PR \#1862 harvest default on m = 3
2026-10-24 · `research/f4_harvest_remeasure_20261024/` · counts only (x86-64, isolation L0) · five sessions of 144 verified records, every audit `OK` with 144 of 144 replays identical

== Question and method
PR \#1862 made the root linear-row harvest of `koblitz_groebner` the default for inherited F4. The pricing-jump bisect saw about +7% counted solver work on one workload (K0, n = 13). Here the harvest is a method parameter (`linear_harvest = off | default | gpv`, `pdp3-koblitz`), so it can be switched per arm inside ecbench. Five specs (one per curve, 24 public targets each), arms `rho-strong` (reference), `f4-m3-off` (baseline), `f4-m3-default`, `f4-m3-gpv`, and the m = 2 pair as a negative control. Protocol frozen and committed before any session existed.

== Results
*Control passed:* at m = 2, `default / off` = 1.0000 [1.000, 1.000] on all five curves.

#table(columns: 7, align: (left, right, left, left, left, left, left),
  [*curve (dim)*], [*ratio*], [*95% interval*], [*targets up / down*], [*per-target range*], [*mean S off → default*], [*S / rho off → default*],
  [n=11, K1 (4)], [1.0958], [[1.0940, 1.0980]], [24 / 0], [1.089–1.113], [403.5 → 442.1], [7.17 → 7.86],
  [n=13, K0 (5)], [1.0699], [[1.0646, 1.0759]], [24 / 0], [1.050–1.169], [517.0 → 553.1], [36.9 → 39.5],
  [n=17, K1 (6)], [0.9969], [[0.9955, 0.9985]], [6 / 18], [0.992–1.006], [1678.1 → 1672.9], [424.1 → 422.8],
  [n=19, K1 (7)], [0.9593], [[0.9576, 0.9611]], [0 / 24], [0.953–0.972], [3004.4 → 2882.1], [1248 → 1198],
  [n=19, K0 (7), XORs unpriced], [S 1.0000; band 0.9755], [none (point)], [S flat], [–], [4.859 both (lower bound)], [1.62 both],
)
Totals ratio of `S` over 24 paired targets; interval is the harness's cluster bootstrap. `gpv` equals `default` at n = 11, 13, 19 and equals `off` at n = 17, so the n = 17 gain is PR \#1891's root-rank gate alone (it fires on none of the other four curves).

#figure(image("harvest_ratio.svg", width: 100%), caption: [(a) Per-target ratio of `S`, `default / off` at m = 3 (dots), the harness's totals ratio with its 95% bootstrap interval (red), and the unpriced curve's point ratio at 0.1 GAE per word XOR. (b) The m = 2 control and the two gates. Data: `analysis/*.json` (`ecbench compare --json`); source `plot.py`.])

== Reading
The sign depends on size: a 7–10% cost at n = 11 and 13 on every one of 24 targets; a 4.1% gain at n = 19 (K1) on every target; a 0.3% gain at n = 17 with mixed per-target signs. Against the current default, switching the harvest off at n = 11 and 13 would lower counted `S` by about 8.7% and 6.5%. Every priced F4 arm sits 7 to 1,250 times the strong rho in `S`, and the harvest moves that gap by at most 10%: nothing here brings inherited F4 closer to rho.

== Classification and limits
AGENTS.md §3: regression at n = 11 and 13 (counted `S` and ratio to the floor rise); engineering gain, 4.1%, at n = 19 K1. Not a row of tables A–C, not a speed claim, no wall time; the leaderboard and `benchmarks/` are not updated. Four priced sizes and one unpriced; intervals resample targets within a curve; no exponent fitted. The hypothesis (sign varies with size) was formed after a two-target scratch smoke run; the sessions use different target seeds.

== Process facts
The director assessed `executed_with_deviations`: card 1 stopped at a missing `audits/` directory; card 2 was refused (stale pinned commit); card 3 was killed by the executor's 30-minute background limit and left no receipt; cards 4–6 ran one command each, and their executors returned `refused` on a misread stop rule although every command exited 0 and every audit reads OK. All five sessions and audits are intact and none was edited or repeated. Details in the README.
