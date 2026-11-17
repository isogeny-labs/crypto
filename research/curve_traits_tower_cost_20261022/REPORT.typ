#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Point tests over GF(p^k) extensions: cost model recalibrated

_curve\_traits, branch `claude/curve-traits-tower-cost` (stacked on PR \#2417), 2026-10-22. Data: `calibration.json`; graph: `plot.py`._

== What changed
The point tests of `points.rs` budget their work in modelled word products. For curves over $"GF"(p^k)$ the extension $"GF"(q^m)$ is `Ext` over the word field `FpK`, and the model charged it 64× the word arithmetic of `PrimeExt`, the overhead measured for the generic extension over a multi-word $"GF"(p)$. That factor was never measured for `FpK` towers. Measured at the same degree over $"GF"(p)$ (26-bit $p$, 256-bit scalar), the tower is 1.9× to 2.8× slower, rising with the degree. The factor for `Model::Extension` is now 3. Nothing else changed: the tests, their certificates and the budgets in modelled units are as before.

#figure(image("tower_overhead.png", width: 100%), caption: [Left: seconds per scalar multiplication, word arithmetic vs. the tower, same total degree. Right: their ratio against the factor now charged.])

#table(columns: 3, [degree $D$ over GF(p)], [word (s)], [tower (s)],
  [20], [0.0124], [0.0236], [40], [0.0500], [0.0912], [80], [0.191], [0.406], [160], [0.598], [1.478], [320], [2.227], [6.178])

== Results (counts from the two files, no sampling)
`traits.json` on \#2417's head against this branch: 13 levels change, all from open to decided, and no decided level changes. The 13 are the prime-power levels at $ell = 2, 3$ that the tests had only bounded: 7 on the crater, 5 at a level between, 1 one above the floor. Every one agrees with the same test run with no budget at all (the debug replay before the change, verdicts listed in `verdicts.txt`). Open levels: 34 → 21.

Build cost: `curve_traits build` 23.1 CPU-minutes, 10.9 minutes wall on the 4-core container, measured with another single-threaded job on one core (\#2417's head measured 22.0 CPU-minutes alone).

== What stays open, and the limits of the calibration
The 19 depth-1 levels over $"GF"(p^k)$ stay open: the twist degree reaches 131 over $"GF"(p^10)$, and even at the measured overhead one multiplication there is far beyond the budget (the ℓ = 31 level at degree 15 takes about 40 s unbudgeted). The ratio rises with the degree (2.8 at $D = 320$); above that, 3 may undercharge by a small fraction. Stored certificates (points searched offline, only checked in the build) are the next step for those levels.
