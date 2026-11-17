#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 D18 same-host construction cost
_Run report · census-cost-v1 and export-cost-v3 · 2026-10-19 UTC_

The native D18 census, byte-identical archive exporter and generic full-object replay ran as separate frozen one-time phase cards on the same EC2 m8i.8xlarge Spot host. Each used one pinned logical CPU, hard 4 GiB memory and zero swap; unrelated research jobs shared the host (L0). The census checked 262,144 x values, found 32,643 full signed-Frobenius direct-subgroup classes and selected 16,627. The exporter retained exactly 2,760,082 labeled point rows. Generic replay checked the full exported object.

#image("../result-20261019/cost.svg", width: 100%)

Native process walls were 27.913872792 s census, 39.372869997 s exact export and 20.702345662 s generic replay, totaling 87.989088451 s. Their GNU outer walls were 27.92, 39.38 and 21.67 s; peak RSS values were 6,100, 97,528 and 233,568 KiB. The exact selected CSV SHA-256 is `534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4`; compressed archive SHA-256 is `347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7`.

The first exporter cost attempt failed in its cgroup wrapper. The second exporter completed but produced a different provenance header, and its exact-object check failed. Those raw attempts are retained. The accepted third exporter used the original pinned source and reproduced the stored bytes. The chart's query-process data came from a separate later native revision and is shown as derived reuse accounting. Upload/storage, useful rank, linear algebra, target verification and matched rho remain unmeasured.

== Sources

`research/n83_prebuild_cost_m23_20261019/CENSUS_COST_PROTOCOL.md`, `runs/census-cost-v1/`, `runs/census-cost-v1.sha256`, `runs/export-cost/`, and `reports/result-20261019/cost-data.json`.
