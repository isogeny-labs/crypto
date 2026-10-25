#set page(paper: "a4", margin: 15mm)
#set text(size: 9pt)
= Can twist symmetry reuse peeled S4 systems?
23 October 2026. Six complete frozen p=73/97 one-target streams. Protocol commit 8801c22aaa; source and raw record in the study directory.
For each residual and signed L2 factor, canonicalize R−T under the verified 24-element Frobenius/cubic action. Equal canonical targets have symmetry-equivalent S4 solution sets in principle. This counts possible calls eliminated before charging the cache.
For a nonzero residual R, corresponding factors T and λT in one L2 orbit cannot give λ-equivalent peeled targets: R−λT=λ(R−T) would imply (1−λ)R=0, impossible for λ≠1 in the prime-order group. All 1,856 tested residuals are nonzero.
#image("k1_peeled_orbit_reuse.svg", width: 100%)
#table(columns: (1fr, 1fr, 1fr, 1fr, 1fr), inset: 3pt, [*p/seed*], [*peels*], [*local dup*], [*cross dup*], [*max cache ratio*],
[73/0], [11,232], [0], [66], [1.005911],
[73/1], [11,520], [0], [65], [1.005674],
[73/2], [11,712], [2], [64], [1.005667],
[97/0], [40,176], [0], [236], [1.005909],
[97/1], [32,976], [0], [184], [1.005611],
[97/2], [34,632], [0], [173], [1.005020],
)
Across separate streams: 142,248 finite peels, two within-residual and 788 additional within-stream exact-orbit collisions. The ideal maximum call-count ratio is 1.005585. Group-action closure, all-image canonical equality and a perturbed-eigenvalue rejection were checked. The physical logs came from the prior independently audited BSGS records.
#pagebreak()
== Subgroup ablation
Follow-up protocol commit 433df0f52d. Identity, cubic order 3, transported Frobenius order 8 and combined order 24 are compared on the same six streams. The full-24 counts reproduce the first record.
#image("k1_peeled_subgroup_reuse.svg", width: 88%)
#table(columns: (1fr, .7fr, .8fr, 1fr, 1fr), inset: 3pt, [*action*], [*order*], [*local*], [*cross query*], [*ideal ratio*],
[identity], [1], [0], [20], [1.000141],
[cubic], [3], [0], [107], [1.000753],
[frob], [8], [0], [243], [1.001711],
[full24], [24], [2], [788], [1.005585],
)
Combined action identifies 770 more targets than identity-only caching among 142,248 finite peels. It identifies 683 more than cubic alone. These counts exclude cache work.
This is an exact orbit count, not a measured polynomial cache or whole-method speed result. Other factor bases and curves remain open.
