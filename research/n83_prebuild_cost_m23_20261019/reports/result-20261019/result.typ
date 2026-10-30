#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 9.5pt)
#set par(leading: 0.5em)
= N83 prebuilt D18: finite-query cost result
_Results report · 2026-10-19 · shared EC2 m8i.8xlarge Spot host, L0_

The D18 census checked 262,144 source x values, found 32,643 full signed-Frobenius direct-subgroup classes, and selected 16,627. The content-addressed export holds 2,760,082 labeled point rows. Fresh general-field replay checked every representative, phase, sign, subgroup point, label and duplicate rule. The same-host census was 27.914 s; byte-identical archive export was 39.373 s; fresh full-object replay was 20.702 s. The three one-time phases total 87.989 s.

#image("cost.svg", width: 100%)

The figure uses the nine-arm mean per m and mode from the frozen paired D card. The same native worker prepared identical accepted point sets and labeled-row digests by import or fresh generation. At m=2, mean worker wall was 9.848 s prebuilt and 35.131 s on demand; at m=3, 8.828 and 34.601 s. Target 0 had A/A repetitions. All 36 finite public probes completed with zero natural rows and useful rank zero; all 36 fresh replays passed. Two constructed controls passed and are excluded from natural yield.

The one-query prebuilt cold sums are 97.838 s (m=2) and 96.818 s (m=3); eight-query derived sums are 166.777 s and 158.617 s. On-demand sums are 35.131 / 281.045 s at m=2 and 34.601 / 276.812 s at m=3 for one/eight queries. Prebuilt worker peak RSS reached 651,149,312 bytes, while on-demand reached 563,486,720 bytes. These are L0 finite-query cost projections; the one-time exporter and paired worker use different pinned native revisions. Remote custody, linear algebra, natural rank growth, target verification and matched rho are outside the measured cost.

The separate K2048/D12 panel completed nine public arms per base and m. K2048 mean worker walls were 1.283 s (m=2) and 6.072 s (m=3); D12 means were 1.258 s and 5.761 s. All 36 finite public arms returned zero natural rows/rank and passed fresh replay. A Spot stop interrupted an initial D12 m=3 attempt; the same instance resumed and the frozen retry completed after a new control. These bases have different point sets and rank dimensions, so this panel does not select one.

== Evidence and disposition

Protocols: `research/n83_prebuild_cost_m23_20261019/PAIRED_MODE_PROTOCOL.md` and `SMALL_BASE_PROTOCOL.md`; raw receipts and SHA-256 manifests: `runs/paired-mode-v1/`, `runs/census-cost-v1/`, `runs/export-cost/`, `runs/small-base-v1/`; data: `reports/result-20261019/cost-data.json` and `reports/run-small-20261019/small-data.json`. The separate U/D m=4 SAT card had three `UNKNOWN_budget` arms and a Spot interruption before U2, so the same-host base ranking remains incomplete. Candidate and selected complete runtime remain null.
