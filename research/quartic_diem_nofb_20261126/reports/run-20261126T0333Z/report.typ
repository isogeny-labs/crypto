#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt, inset: 3.5pt)
= Quartic Diem 2LP IC: skipping the doomed full-core fallback in `2lp-sfb-sge`

Run report for study `research/quartic_diem_nofb_20261126`, runs on 2026-11-26 from 03:33 to 04:03 UTC. *Every number is \[pcc:unverified\]*: one shared host, planted targets, and no validator report yet.

*Binding.* PROTOCOL.md, `seeds.json` (seed policy v1), `analyze.py` and the run scripts were frozen in 8297532db36 before any grid run. The binary `genus_ladder.nofb` has sha256 `55ebb440…31602`.

*What ran.*
- Arms: `2lp-sfb-sge` (baseline, W = 8, FILL = 1.5, EXCESS = 64) and `2lp-sfb-sge-nofb`, which skips the full-core Wiedemann fallback after a failed square solve. The fallback draws only from a cloned RNG, so the pairing is exact.
- Instances: the parent's 20 prime-order plane quartics at p = 4099, 16411, 65537 and 131101. Solver seeds 1–2 are the parent's; seeds 3–4 are held-out and form the primary panel.
- 160 runs, all certified with the same log in both arms. C0 matches 40 of 40 rows to the parent, with no pair-identity violation.

*Rescues.* The baseline fell back 16 times and certified 0 of them, so skipping the fallback gave up nothing.

== Decision (frozen rule): IMPROVEMENT, with the order holding across the band
#table(columns: 3,
[w], [pooled paired GAE ratio nofb/sge, held-out seeds 3–4], [all seeds 1–4],
[1], [0.833 \[0.758, 0.907\]], [0.826 \[0.773, 0.881\]],
[0.1], [0.833 \[0.761, 0.908\]], [0.826 \[0.771, 0.880\]],
[0], [0.9971 \[0.9959, 0.9984\]], [0.9972 \[0.9962, 0.9980\]],
)
All four preregistered predictions held. On fallback pairs, the new arm's linear algebra is 20–27% of the baseline's.

#figure(image("fig_nofb.svg", width: 100%), caption: [Left: paired GAE_w ratio nofb/sge per prime, held-out panel (solid) and all seeds (dashed), cluster boot95. Right: IC / ideal negation rho per arm and w.])

== IC / rho at w = 1
#table(columns: 5,
[p], [4099], [16411], [65537], [131101],
[`2lp-sfb-sge`], [8.83], [7.55], [2.98], [2.63],
[`2lp-sfb-sge-nofb`], [8.83], [5.16], [2.69], [1.98 \[1.84, 2.16\]],
)
Every prime is UNRESOLVED (below rho at w = 0, above it at w = 1). The distance remaining at 131101, w = 1, is 1.98×.

*Next lever.* Second attempts now cost about double a single attempt. They come from first attempts in which t is undetermined. Avoiding those needs a new protocol.
