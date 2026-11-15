#set page(paper: "a4", margin: 17mm)
#set text(size: 9.5pt)
#set heading(numbering: none)
= Both 62-bit targets recovered on one GPU

Frozen protocol-v11 completed all 12 prescribed executions on one RTX PRO 6000
Blackwell Server Edition. Every result passed fresh CPU certificate replay and
scalar-corruption rejection. The largest complete prescribed rung is 62 field
bits. This is a bounded implementation result, not an absolute hardware maximum,
certified speedup, matched-rho result or security conclusion. The relation oracle
is generic meet-in-the-middle; it is not the paper's conditional tensor backend.

== Matched 52-bit controls
#table(columns:(0.5fr,0.5fr,1fr,1fr,1fr,1fr),inset:4pt,
 [Bits],[Seed],[b: 16-byte Thrust],[u: 12-byte Thrust],[v: 12-byte CUB],[x: identical v],
 [52],[101],[17.700446],[15.116574],[14.998561],[15.019326],
 [52],[202],[22.070081],[18.789266],[18.797276],[18.710886],
)
Cold solve seconds, rounded from `results-v11/rows.csv`. Arms run b/u/v/x for
seed 101 and x/v/u/b for seed 202. Certificates match byte for byte across arms.
All clocks are shared-host L0 diagnostics, with wall_valid=false; no inferential
interval or certified timing ratio is available.

== Larger fixed targets: dense CUB arm v
#table(columns:(0.5fr,0.6fr,1fr,1fr,1fr,1fr),inset:4pt,
 [Bits],[Seed],[Cold solve (s)],[With audit (s)],[Relations],[Attempts],
 [61],[101],[1123.435000],[1123.454534],[1617],[2292],
 [61],[202],[1405.348145],[1405.366104],[1591],[2432],
 [62],[101],[2139.880952],[2139.908483],[1724],[2675],
 [62],[202],[2187.942595],[2187.971582],[1771],[2485],
)
At 62 bits, oracle time was 2137.956289 / 2185.870989 seconds and linear
elimination 1.618892 / 1.757169 seconds. Table admission estimates were
48,798,250,775 / 49,539,778,479 bytes, including queried sort scratch; they are
not measured peak VRAM. No prescribed cell timed out, refused admission or failed.

#image("scaling-v11.svg",width:100%)
Every recorded execution is shown. Colors denote arms, not independent hardware.

#pagebreak()
= Contract, validation and custody

Measured source: `d44b59149f468357e86186f956ecb7e323f38915`.
Pod: `jees49ew9dkfd7`, created 2026-10-30T15:50:53 UTC. NVIDIA telemetry reports
97,887 MiB, driver 595.91.07 and compute capability 12.0. The build used CUDA 13.2,
sm_120 and Rust 1.88.0. Provider rate was USD 2.49/hour; this is not a final bill.
The controller verified archive integrity and pod deletion after completion.
A subsequent complete provider listing independently confirmed absence.

Command: `bash research/explicit_relations_e2e_20261030/run-sort.sh`, with
EPHEMERAL_OUT=/results. Seeds 101/202; m=8; tau=2ceil(r^(1/8))+2;
`rows <= 2^32`, `work <= 2^34`, chunk 2^20, columns 16384, trials 16.
Cold solve caps: 180 seconds at 52 bits, 2700 at 61, 5400 at 62. Fresh replay:
60 seconds; PARI fixture generation: 180; worker: 18000; controller: 18300.
The plan finishes both targets at a size, then stops larger sizes if either
is incomplete. Its complete maximum is 12 executions, all recorded here.

The source accepts certified prime-order curves with `p < 2^62` and `r < 2^63`.
Public targets are coordinate lifts; no target scalar is generated or supplied.
The driver checks the curve, primality and the unique Hasse-interval order witness.
All 19 Rust tests, host/device differential checks, Clippy, maximum-cap protocol
headers and recorded query/service sanitizer checks passed. Every successful
certificate passed fresh CPU replay and rejected scalar+1. There is no root
crypto plug-in build on the constrained local workspace.

Cold solve includes process startup, validation, all base construction and
relation attempts, elimination, extraction, in-process replay, serialization
and teardown. The separate audit total also includes target generation, fresh
replay and negative control. Build/preflight and curve fixture generation are
outside cold solve. CPU user/system time and measured peak VRAM were not captured.

Frozen protocol hash:
#text(size:8pt)[`0f916e48bea43b0b7dabb85bc8cb50fe0d31c2a268aa89c016e1df6b46a0a09e`]
Raw archive SHA256:
#text(size:8pt)[`e80cf3fb2c64fd2510e1eac7f15d0477b3a3df9408dfd88d73df0b6e3a888372`]
Archive: `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-v11/results.tgz`.
Raw records were committed and pushed in `099dff2edd81` before this report.
The archive manifest hashes every member; certificate-audit.json retains
native replay and rejection outputs. These manifests do not replace replay.

Schema, flow and file hashes pass. Native analysis and the strict coverage
audit pass; the latter rejects eight deliberate coverage/status/input mutations.
The canonical checker retains five rectangular-coverage findings: 24 assumed
rows versus 12, b/u/x each cover one rather than three sizes, and arm instance
sets differ. The frozen plan prescribes four arms only at 52 bits and v alone
at larger sizes. This is not a full canonical-checker pass or independent
scientific validation. See `results-v11/CHECKER-NOTE.md`; no row was excluded.

== Implementation context and next steps
Exact 12-byte rows and queried CUB scratch are described in DENSE-ROWS.md,
which cites NVIDIA alignment guidance and the CCCL DeviceMergeSort source.
V10 was never executed. The separately frozen v13 adds persistent queries and
full right-sum caching with distinct controls; its results are not included here.
A separate 63-bit arithmetic extension remains pending GPU validation. Neither
that source change nor this complete 62-bit panel establishes a hardware maximum.
