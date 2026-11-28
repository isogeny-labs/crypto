#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt, inset: 3.5pt)
= Quartic Diem 2LP IC: lines through P0 so that the first attempt determines t

Run report for study `research/quartic_diem_p0lines_20261128`, runs on 2026-11-28 from 03:45 to 04:21 UTC. *Every number is \[pcc:unverified\]*: one shared host, planted targets, and no validator report yet.

*Binding.* PROTOCOL.md, `seeds.json` (seed policy v1), `analyze.py` and the run scripts were frozen in 7f67f384ece before any grid run. The binary `genus_ladder.p0` (sha256 `283c1510…23723cd`) was built from that commit.

*Diagnosis behind the change* (PR \#5269; 57 cores dumped on dev curves 4093 and 8191, exact elimination mod r):
- The line rows leave 1–2 dense free directions (the scale and the base-point shift) plus 0–2 columns that occur only in decomposition rows.
- t is determined exactly when the linked decomposition rows add at least that many + 1 to the rank.
- The base-point shift is free in 45 of 57 cores, because no line through P0 reaches the core.

*What ran.*
- Arms:
  - `nofb`, the baseline;
  - `p0`, which also seeds the lines through P0 and each factor-base point;
  - `p0i3`, as `p0` but with 3 decomposition rows instead of 4.
- The parent's 20 prime-order plane quartics at p = 4099, 16411, 65537 and 131101, with solver seeds 1–6. Seeds 5–6 are held-out and form the primary panel.
- 360 runs, all certified with the same log in all arms. C0: 80 of 80 `nofb` rows reproduce the parent exactly.

== Decision (frozen rule): INCONCLUSIVE
#table(columns: 3,
[w], [pooled paired GAE ratio p0/nofb, held-out seeds 5–6], [all seeds 1–6 (secondary)],
[1], [0.990 \[0.934, 1.045\]], [0.951 \[0.921, 0.981\]],
[0.1], [0.990 \[0.933, 1.045\]], [0.951 \[0.921, 0.982\]],
[0], [1.018 \[0.861, 1.181\]], [0.937 \[0.873, 1.002\]],
)
- Runs needing a second attempt: `nofb` 10/40 → `p0` 8/40 on seeds 5–6, and 25/120 → 12/120 on all seeds.
- The held-out seeds gave `nofb` few failures to remove (16411: 1/10 against 8/20 on seeds 1–4).
- Predictions 1 and 2 failed; prediction 3 (`p0i3`) held.

#figure(image("fig_p0lines.svg", width: 100%), caption: [Left: paired GAE_w ratio per prime, held-out seeds 5–6, cluster boot95. Right: IC / ideal negation rho per arm and w (all seeds).])

== Mechanism (exploratory, descriptive)
GAE₁ ratio p0/nofb by attempt pattern, all seeds:
- both arms one attempt: 0.99–1.03;
- rescued runs (nofb 2 → p0 1, 15 of 25): 0.62;
- unrescued failures (10): about 1.0;
- two new p0 failures: 1.56.

The unrescued share grows with p (5 of 9 at 131101). That fits the second mechanism: factor-base columns beyond the pair-enumeration frontier (about 55% of the factor base when the first attempt fires) that occur only in decomposition rows.

== IC / rho at w = 1 (all seeds)
#table(columns: 5,
[p], [4099], [16411], [65537], [131101],
[`nofb`], [8.76], [4.96], [2.84], [2.02 \[1.85, 2.20\]],
[`p0`], [8.88], [4.45], [2.70], [1.91 \[1.82, 2.00\]],
)
Every prime is UNRESOLVED (below rho at w = 0 and w = 0.1, above it at w = 1). The distance remaining at 131101, w = 1, is about 1.9–2.0×.

*Next lever.* Cover the frontier columns, either by an enumeration order that reaches the whole factor base early or by accepting only covered decompositions. This needs a new protocol with a larger held-out panel.
