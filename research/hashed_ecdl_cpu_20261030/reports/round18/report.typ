#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= CPU hash enumeration: six-arm result
All 180 planned executions completed: 162 measured and 18 warmups, with byte-identical full certificates across arms for every fixture and repetition. The native analyzer accepted all 18 complete cells.

Job `hashed-ecdl-cpu-round18-20261030`, UID `68845702-fd9d-4940-b64c-67be6a25260c`, completed 2026-10-30 20:59:14 UTC. AMD EPYC 9R45, one software thread, 2 vCPU / 4 GiB allocation. L0 shared-host diagnostic; recorded times do not establish an isolated performance claim.

Arms u/x: original baseline `2bd4588fba60`; v/w: hash enumeration `0d6e3840edb6`; y: Curve context `23ff0e6bda63`; z: inverse cache `5d7dcb67616c`. Batch size 64 except w (1). Each cell has one warmup and nine measured repetitions. All comparisons below are within this panel.

#table(columns: (0.5fr,0.5fr,1fr,1fr,1fr), inset: 3pt,
 [Arm], [Case], [Min ms], [Median ms], [Max ms],
 [u], [0], [0.283021], [0.284861], [0.287391],
 [u], [1], [3.929944], [3.942654], [3.953055],
 [u], [2], [4.148525], [4.160665], [4.218955],
 [v], [0], [0.100051], [0.101051], [0.10173],
 [v], [1], [0.603192], [0.607232], [0.614392],
 [v], [2], [0.744722], [0.750332], [0.753313],
 [w], [0], [0.099111], [0.099961], [0.108761],
 [w], [1], [0.802062], [0.808853], [0.812043],
 [w], [2], [1.058473], [1.065143], [1.068504],
 [x], [0], [0.284721], [0.285671], [0.286531],
 [x], [1], [3.932034], [3.945584], [3.974184],
 [x], [2], [4.146975], [4.168635], [4.189905],
 [y], [0], [0.10118], [0.102471], [0.105311],
 [y], [1], [0.608482], [0.611262], [0.623702],
 [y], [2], [0.749822], [0.754313], [0.768362],
 [z], [0], [0.10125], [0.102581], [0.11167],
 [z], [1], [0.610672], [0.614282], [0.625222],
 [z], [2], [0.755213], [0.759202], [0.765742],
)

Complete time includes curve and known-target construction, all trial setup and discarded trials, factor bases, query generation, reduction, internal verification, second independent replay and certificate serialization. Process launch, archive I/O and corrupted-scalar negative controls are outside this timer. The external process timer is retained separately in rows.csv.

#pagebreak()
= Recorded complete latency
#image("../../round18/results/latency.svg", width: 100%)
Source: frozen `round18/analyze.rs`, recorded `round18/results/summary.csv`; nine measured values per cell. The graph shows medians, with the complete observed ranges in the preceding table.

Case 0: p=211, a=0, b=7, subgroup order 199, P=(3,33), m=3, tau=40, scalar=42, seed=111. Cases 1/2: p=65521, a=65518, b=3, subgroup order 65563, P=(0,64894); case 1 m=3, tau=80, scalar=45678, seed=222; case 2 m=4, tau=64, scalar=54321, seed=333. Limits: 16 trials and 1,000,000 work units. Known scalars construct fixtures; the solver receives public points.

Command: frozen `round18/run.sh`, executed with `EPHEMERAL_OUT=/work/results`. Baseline duplicate x provides the A/A control; all positive scalar checks, full replays and corrupted-scalar rejection completed. A/A ranges overlap in all three fixtures. No runs failed, timed out, resumed or were excluded.

Source, binary and returned-manifest identities are retained in `round18/results/`. Protocol SHA-256: `259ecd8eec7d7c17e99fa3f15c52196aa6b07cf33ac1bf0d60ab3c36f98e26a6`. Result archive SHA-256: `787fc9dd46c6a228d1622706e5834e7ad4f5cdec0a3b23afe396fa2c484bf13f`.

The cached-affine implementation in PR 2980 was implemented after the four revisions pinned in this panel and is not measured here. Its next measurement needs a separately frozen comparison. Reference material: the protocol's literature records and the retained round16 report informed this panel; no new literature or novelty claim is introduced.
