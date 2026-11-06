#set page(paper: "a4", margin: 17mm)
#set text(font: "Arial", size: 10pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
#set table(inset: 5pt, stroke: 0.4pt + rgb("ccd4dc"))

= N83 complete adjacent-K cold screen
*Run 38978215742 · 16 October 2026 · one exposed public target*

Twenty-one fresh IC processes cover all 19 integer K values from 1102 through
1120. The original analysis records K1107 as the descriptive cold-process
minimum at 116.222 seconds, followed by K1112 at 117.262 seconds. This is an
exposed-point screen with a null promoted candidate. It nominates a later
comparison; it does not establish a fresh-target selection.

#figure(image("observations.svg", width: 100%), caption: [All 21 recorded IC
observations. Orange dots are the K1104 and K1120 A/B bookends, with two
processes each; blue dots have one process each. The cold-time axis is cropped.
The lower panel is a separately recorded stage, not the complete process.
No confidence intervals were estimated.])

#table(columns: (0.6fr, 1fr, 1fr, 1fr),
  [*K*], [*Cold process, s*], [*Online stage, ms*], [*Processes*],
  [1107], [116.222], [354.511], [1],
  [1112], [117.262], [59.898], [1],
  [1111], [117.598], [488.007], [1],
  [1103], [117.637], [43.962], [1],
  [1104], [119.440], [51.613], [2; A/B mean],
  [1120], [119.632], [23.300], [2; A/B mean],
)

The table reproduces the original result's selected rows; the graph includes
every process. Measured milliseconds are divided by 1,000 for seconds.
The two control means are derived summaries already recorded by the verifier.
K1104 and K1120 have recorded relative A/B spreads of 0.336% and 1.039%,
respectively, both below the frozen 5% threshold. Their repeated base and
root-index identities match. The cold minimum and the smallest target-stage
measurement occur at different K values; those accounting intervals stay separate.

#pagebreak()
== Workload, schedule and host
The curve is `EC1N83Ckb1a1haacb69ee9f37`, workload `eebf98ee1031`.
The dynamic root11 layout is fixed throughout. Each process constructs an
empty base, index and log cache for one previously exposed public target.
The usable base contains 166 K points and K folded columns. Rank seed 29,
four rank threads and one target thread are fixed. The schedule begins with
K1104 A and K1120 A, runs the other 17 K values in ascending order, and ends
with K1120 B and K1104 B. One same-point rho process follows.

The IC cap is 360 seconds per arm and the rho cap is 120 seconds. All 21 IC
arms and the rho arm completed. The archived host exposes four AMD EPYC 7763
CPUs and 15.61 GiB memory. The verifier records
`cloud_instance_exploratory`; topology, affinity, cgroups, telemetry and
resource receipts are retained. No stronger host-isolation tier is claimed.

== Accounting and evidence boundary
Cold process time includes setup, relation-rank work and target processing.
The five exclusive target-dependent online phases are retained separately in
the exact IC1 rows. Same-point rho recorded 720.959 milliseconds online.
The original study classifies the online values as stage diagnostics from
this cold experiment; a primary online comparison requires its own matched
fresh-point panel. No cold/online ratio is presented as a speedup. This screen
also has no fastest-external-tool comparison.

== Source and custody
The exact cloud execution commit is
`77a6090773b49ce45c7377d82853924722898bcd`.
The IC example SHA-256 is
`8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12`.
The source and both executable hashes are copied unchanged in `raw/source.json`.
The provider ZIP SHA-256 is
`b07cef5779312fed6fb49c2af767d5d2c8cbdc1eca6c14de85ddd6753fb6290f`.
The archived analysis SHA-256 is
`d4ad13279592ae82086c5ea04cf97348a98b692d559f1847af270b8caed065f0`.

The original independent replay exited zero, had empty stderr and reproduced
the archived analysis. During report completion, the materializer rechecked
the ZIP digest, all 180 result members, source/host/target identity, the
retained replay logs, 19 candidate manifests and 21 run rows. This is a fresh
custody check using the recorded replay; it adds no new timing measurement.
The raw receipt copies and `provenance.json` retain exact bytes, hashes and
the archive location.

== Interpretation and reproducibility
The frozen result is `SCREEN_COMPLETE` with `selected_candidate_id: null`.
The original next gate compares root11 and root22 at K1107 and K1112, before
the separately sealed sixteen-point holdout. The primary leaderboard,
progress timeline and performance-gains figures receive no new admitted
point from this exposed-point screen. The benchmark entry records the
observed grid and its limitations.

Protocol and result: `../../PROTOCOL.md`, `../../RESULT.md`.
All 21 observations and exact candidate identities:
`../../ic1_rows_38978215742.json`.
Provider run: #link("https://github.com/aburan28/crypto/actions/runs/38978215742")[38978215742].
Render with `python plot.py`, then
`typst compile --root ../.. report.typ report.pdf` from this directory.

