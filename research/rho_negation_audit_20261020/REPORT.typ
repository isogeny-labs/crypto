#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set table(stroke: 0.4pt)
= Rho negation-map audit (2026-10-20)

*Question.* Does every Pollard rho we use as a reference walk on the classes ${P, -P}$ (the negation map, $A = 2$), and does each such walk deliver the $sqrt(2)$ saving in counted operations?

*Answer.* Before this change, two walks lacked the map: `volcano-rho`'s native walk (crypto) and crypto-autoresearcher's index-calculus baseline `index_calculus/rho.py` (audit X02/X03). Both now walk on ${R, -R}$ by default, with the old walk kept behind `--negation off` and `--rho-walk plain`. The matched reference `ecbench rho.negation` already had the map, and it delivers the saving.

#figure(image("figures/negation_vs_plain.svg", width: 100%), caption: [Left: ecbench session ECBS1h83b97eccdcda, mean $S$ with 95% interval, 64 targets per curve and arm; dotted lines are the floors $sqrt(pi/4)$ and $sqrt(pi/2)$. Right: `volcano-rho --instance-study`, 36-bit prime-order curve, 256 solves per arm, mean iterations over each walk's own floor.])

== 1. ecbench `rho.negation` vs `rho.plain`
Spec ECS1h7a0b49db98d3 at main `6e35e1f42` (clean). Four fresh prime-order curves (28–40 bits; $\#E$ counted natively, prime, cofactor 1). 512 records, all verified; 16 exact replays. Host: Intel Xeon \@ 2.10 GHz, 4 vCPU shared VM, L0.

#table(columns: 7, align: (left, right, right, right, right, right, right),
  [curve], [log2 r], [neg S [95%]], [/floor], [plain S [95%]], [/floor], [plain/neg],
  [fp28-tm24531], [27.95], [0.958 [0.846, 1.076]], [1.081], [1.391 [1.229, 1.556]], [1.109], [1.452],
  [fp32-t93403], [31.76], [0.993 [0.884, 1.108]], [1.120], [1.280 [1.122, 1.439]], [1.021], [1.289],
  [fp36-t360541], [35.21], [1.016 [0.895, 1.140]], [1.146], [1.235 [1.069, 1.406]], [0.986], [1.216],
  [fp40-tm62443], [39.73], [0.953 [0.821, 1.096]], [1.075], [1.468 [1.314, 1.631]], [1.171], [1.540],
)
- Pooled ops (cluster bootstrap, 256 pairs): plain/negation = *1.371 [1.257, 1.491]*; $sqrt(2)$ = 1.414 is inside. ecbench: "diagnostic only (unpriced work)".
- §5a band: the order holds at 0 and 0.1 GAE per canonicalisation. At 1 GAE the negation $S$ is 1.89–2.03, above plain's. Verdict: unresolved inside the band on all four curves.
- Wall (L0, descriptive): median paired ratio 1.140.

== 2. `volcano-rho` (changed)
36-bit curve $p = 64371108127$, $n = 64371143701$; `--negation on` vs `off`, same seed, 256 solves per arm, 2,560/2,560 correct.

#table(columns: 6, align: (left, right, right, right, right, right),
  [arm], [on / $sqrt(pi n slash 4)$], [off / $sqrt(pi n slash 2)$], [off/on], [ns/iter on], [ns/iter off],
  [walk], [1.011], [0.998], [1.396], [363], [314],
  [k], [1.026], [1.020], [1.405], [362], [310],
  [generator], [0.971], [0.995], [1.449], [362], [306],
  [all], [1.027], [1.036], [1.428], [336], [308],
  [small_k], [1.068], [0.980], [1.298], [353], [314],
)
Iterations include the look-ahead's discarded additions and the escapes' doublings. Cost per iteration rises 9–16%, so the wall gain is about 1.2× (descriptive). Native and pull-back lenses: 176/176 correct each on a 30-bit depth-2 volcano.

== 3. crypto-autoresearcher `index_calculus/rho.py` (changed)
640 paired solves at 16–32 bits: plain/negation walk_ops = 1.353 [1.281, 1.425]. The negation walk is at 1.19× its floor; plain at 1.14×. With `--rho-walk plain` the output is bit-identical to the walk before the change.

== Scope
This corrects a reference and makes two rho baselines stronger. It is not an index-calculus improvement and moves no leaderboard row; crypto's prime rows already use `rho.negation`. Rows built on the autoresearcher's plain rho are $sqrt(2)$ too generous to IC. Not changed: crypto-autoresearcher `harness/rho.py` and `harness/walk.py` (follow-up). Sources: `research/rho_negation_audit_20261020/` (crypto) and `research/rho_negation_map_20261020/` (crypto-autoresearcher).
