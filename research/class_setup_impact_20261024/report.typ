#set page(paper: "a4", margin: 18mm)
#set text(font: ("Liberation Sans", "DejaVu Sans"), size: 9.5pt)
#set par(justify: false)
#show heading: set text(size: 11pt)
#show table: set text(size: 8.5pt)

#text(size: 15pt, weight: "bold")[Impact audit: the pre-2026-10-18 `rho.quartic` / `rho.cubic` class set-up undercharge]

2026-10-24 · `research/class_setup_impact_20261024/` · derived from committed records with ecbench `compare` and `table`; nothing re-measured, no committed session changed · class: *accounting* (no method improves)

= What was wrong

`rho.quartic` and `rho.cubic` begin with a class set-up: the scalar multiplication that fixes ι (j = 1728) or ζ₃ (j = 0) on the subgroup. Binaries built before `9ead78cf0` (2026-10-18) charged it to the `setup` phase but not to `total_gae` or `S`. Each total fell short of the phase sum by exactly the set-up: 16–58 GAE at r ≈ 2^14–2^24, and 111–125 GAE at r ≈ 2^39.5. The phase sum is the true total. Every other arm, `rho.negation` included, was charged correctly.

`ecbench verify` reconstructs the old figure from a replay so that old records still replay exactly. It does this only for records from the five pre-fix producer binaries, with no `detail.class_setup`, on these two methods. A record the current harness writes, undercharged and resealed, fails its replay (`docs/ecbench/CLASS_SETUP_AUDIT.md`).

= Census (every session, every storage form, four repositories)

#table(columns: 5, align: (left, right, right, left, left),
  [*study*], [*sessions*], [*records*], [*binary*], [*commit*],
  [prime_rho_automorphism_20261015], [2], [192], [`0a8a48d7…`], [`c9db37d01`],
  [drastic_prime_conductor_gap_round2], [2], [128], [`1a87842e…`], [`466035c18`],
  [drastic_prime_conductor_gap_round3], [3], [320], [`d24c2247…`], [`7fa17f92e`],
  [volcano_ecdlp_census_20261017 (V3, `.tar.xz`)], [3], [4,032], [`ac4bc5b2…`], [`d13f5259c`],
  [volcano_ecdlp_census_followup_20261023 (V5, `.tar.xz`)], [66], [7,600], [`ac4bc5b2…`], [`d13f5259c`],
  [isogeny-volcano-cost_20261017 (gzip)], [2], [256], [`7da27ec3…`], [`3d191d806`],
)

= Recorded against corrected

Both columns come from the same `ecbench compare`: operations, ratio of mean S, linearised-t 95% interval. One is run on the session as recorded, the other on a scratch copy whose totals include the set-up.

#figure(image("figures/impact.svg", width: 100%), caption: [Recorded (open) and corrected (filled) ratios. Red ticks: asymptotic theory. Data: `compare/*.json`.])

#table(columns: 5, align: (left, right, left, left, right),
  [*automorphism rho / rho.negation*], [*pairs*], [*recorded*], [*corrected*], [*theory*],
  [prime-aut quartic], [72], [0.791 [0.696, 0.899]], [*0.819 [0.722, 0.929]*], [0.707],
  [prime-aut cubic], [72], [0.611 [0.512, 0.729]], [*0.647 [0.543, 0.770]*], [0.577],
  [iso-cost j = 0 crater], [64], [0.697 [0.580, 0.837]], [*0.716 [0.598, 0.857]*], [0.577],
  [iso-cost j = 1728 crater], [64], [0.707 [0.606, 0.826]], [*0.723 [0.621, 0.843]*], [0.707],
  [census V3 crater D], [1,000], [0.572 [0.546, 0.598]], [*0.599 [0.573, 0.627]*], [0.577],
  [census V3 crater N], [1,000], [0.584 [0.557, 0.612]], [*0.611 [0.584, 0.640]*], [0.577],
  [census V3 crater ctl], [2,000], [0.569 [0.557, 0.582]], [*0.596 [0.583, 0.609]*], [0.577],
  [follow-up V5 crater D], [1,000], [0.614 [0.588, 0.641]], [*0.620 [0.594, 0.648]*], [0.577],
  [follow-up V5 crater N], [1,000], [0.604 [0.583, 0.625]], [*0.610 [0.590, 0.632]*], [0.577],
  [follow-up V5 crater ctl], [2,000], [0.614 [0.598, 0.630]], [*0.621 [0.605, 0.636]*], [0.577],
)

#table(columns: 4, align: (left, right, left, left),
  [*IC / matched rho.cubic, 0 GAE*], [*pairs*], [*recorded*], [*corrected*],
  [census V3 crater D], [1,000], [85.0× [81.8, 88.3]], [*81.1× [78.1, 84.2]*],
  [census V3 crater N], [1,000], [84.7× [79.5, 90.4]], [*80.9× [76.0, 86.2]*],
  [census V3 crater ctl], [2,000], [102.6× [98.8, 106.5]], [*97.9× [94.4, 101.6]*],
  [follow-up V5 crater D], [1,000], [88.7× [83.9, 93.7]], [*87.7× [83.0, 92.7]*],
  [follow-up V5 crater N], [1,000], [90.0× [87.6, 92.4]], [*89.0× [86.7, 91.4]*],
  [follow-up V5 crater ctl], [2,000], [91.2× [88.7, 93.9]], [*90.2× [87.7, 92.8]*],
)

The correction raises only the rho arm's charged cost, so IC/rho falls by the same factor at 0, 0.1 and 1 GAE. Rho stays ahead on every row, by about 81× or more. This is accounting, not an IC improvement, and no IC baseline moves. The 40-bit conductor-gap rows move by 0.012–0.026% (`tables/`), which is invisible at the precision they report.

= What changes

+ *prime_rho_automorphism_20261015.* The quartic control now excludes its prediction 0.707, so finding 1 holds for cubic only. The set-up is a fixed cost of about one scalar multiplication, and the asymptotic √ saving leaves it out. The round-2 factors become about 1.55× and 1.22×, and round 2's conclusion stands.
+ *volcano_ecdlp_census_20261017.* The positive control still passes its gate (upper bound below 0.85). It sits 3–6% above 1/√3, and the N and ctl intervals exclude 0.577. The crater IC / `rho-cubic` gap is about 81×, not 85×. Erratum E-CS in `REPORT.md` supersedes 0.5733 and 84.9×, including in the validated `benchmarks/README.md` row, which is digest-pinned and not rewritten here.
+ *volcano_ecdlp_census_followup_20261023 (in progress).* It runs the same frozen binary, so its preregistered analysis will read `rho-cubic` about 1% low: the positive control, the crater gap and the H-T composition. It should add the set-up to every `rho-cubic` total, or amend its plan.
+ *isogeny-volcano-cost_20261017.* Both controls remain detected. j = 1728 still covers √(1/2).
+ *conductor-gap rounds 2 and 3.* Checked and unchanged.
