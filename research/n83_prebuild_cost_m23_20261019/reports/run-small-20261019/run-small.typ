#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 K2048 and D12 finite-query panel
_Run report · small-base-v1 · 2026-10-19 UTC_

Two separately identified signed-Frobenius bases were queried on eight frozen public N83 points. K2048 has 2,048 columns and 339,968 points; D12 has 2,001 columns and 332,166 points. Each m=2 arm exhausted first-summand complements; each m=3 arm drew `2^20` deterministic pairs. Target zero had two A/A repetitions. Each completed query had a fresh replay. Workers used one pinned CPU, hard 4 GiB, zero swap and read-only inputs on a shared L0 EC2 Spot host.

#image("small.svg", width: 100%)

Nine arms per base and m completed. K2048 made 3,059,712 m=2 and 9,437,184 m=3 finite attempts; D12 made 2,989,494 and 9,437,184. All 36 public arms returned zero natural relation rows and useful rank zero, and all 36 fresh replays passed. Five constructed controls, including one after a Spot reboot, passed separate general-field replay with changed-target and changed-row rejection.

The original D12 target-7 m=3 attempt lost its worker and exit receipt when the Spot instance stopped. Its partial directory remains `UNKNOWN_spot_interruption`. The same instance and root EBS volume resumed; a frozen continuation reran a D12 m=3 control, then the missing public arm and replay. All 77 completed Docker arms exited zero. The chart shows process-wall means, but host timing is L0 and split by the stop. The base point sets and rank dimensions differ, so these bounded zero-yield samples do not select a base or establish a whole-pipeline speed result.

== Sources

`research/n83_prebuild_cost_m23_20261019/SMALL_BASE_PROTOCOL.md`, `SMALL_BASE_SPOT_RESUME.md`, `runs/small-base-v1/`, `runs/small-base-v1.sha256`, and `reports/run-small-20261019/small-data.json`. Worker source commit `d4badc176820cb8bad63246ad8db02e47ae4cde1`; binary SHA-256 `3f2572663880fc010f3fb2aee33114d5be4f4192e60f0ead4bc71f1b5e3e895f`.
