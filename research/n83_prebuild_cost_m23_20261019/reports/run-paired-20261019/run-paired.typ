#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 matched D finite-query run
_Run report · paired-mode-v1 · 2026-10-19 06:01:45–06:18:33 UTC_

One x86_64 EC2 Spot host ran all D prebuilt and on-demand arms under the same native binary, frozen eight public points, 1 CPU, 4 GiB hard memory, zero swap and no network. Other jobs shared the host (L0). Both D preparation paths had identical point-set and labeled-row digests. The target-zero order was prebuilt/on-demand/on-demand/prebuilt at each m, followed by seven paired public targets.

#image("../result-20261019/cost.svg", width: 100%)

Each m=2 query completed 2,760,082 first-summand complement checks; each m=3 query completed `2^20` deterministic pair draws. Nine arms per m and mode completed with zero natural relation rows and rank zero. The 36 probe receipts all say `COMPLETE`; 36 separate replays say `PASS`; two setup and two constructed control arms passed. Both controls passed separate general-field replay with changed-target and changed-row rejection. All 76 Docker runs exited zero. The full card elapsed 16 min 48 s, including container launches, setup, controls and replays.

Mean worker process wall was 9.848 versus 35.131 s at m=2 and 8.828 versus 34.601 s at m=3 for prebuilt versus on-demand. The figure separately charges 27.914 s census, 39.373 s export and 20.702 s full-object replay; its one/eight-query sums are derived L0 finite-event projections, not a complete IC comparison. Prebuilt maximum worker RSS was 651,149,312 bytes versus 563,486,720 bytes on demand.

== Sources

`research/n83_prebuild_cost_m23_20261019/PAIRED_MODE_PROTOCOL.md`, `PAIRED_MODE_SPOT_ADDENDUM.md`, `runs/paired-mode-v1/`, `runs/paired-mode-v1.sha256`, and `reports/result-20261019/cost-data.json`. Source commit `49e5f5f12324ee2f750136f4114fb9cd15388e4d`; binary SHA-256 `80a53099ff4db99519de3d3700bbde3948dc0b435e6a3c7ac800164ac7450a24`. Natural rank growth, linear algebra, individual logarithm, verification and matched rho remain open.
