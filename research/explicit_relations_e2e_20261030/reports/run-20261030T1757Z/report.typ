#set page(paper: "a4", margin: 17mm)
#set text(size: 9.5pt)
#set heading(numbering: none)
= Both 62-bit targets recovered on one GPU

Observations only: frozen protocol-v13 completed all 12 prescribed executions on one RTX PRO 6000
Blackwell Server Edition. Every result passed fresh CPU certificate replay and
scalar-corruption rejection. The largest complete prescribed rung is 62 field
bits. This is a bounded implementation result, not an absolute hardware maximum,
certified speedup, matched-rho result or security conclusion. The relation oracle
is generic meet-in-the-middle; it is not the paper's conditional tensor backend.

== Matched 52-bit controls
#table(columns:(0.5fr,0.5fr,1fr,1fr,1fr,1fr),inset:4pt,
 [Bits],[Seed],[b: chunked CUB],[u: persistent],[v: full cache],[x: identical v],
 [52],[101],[15.435474],[15.941507],[15.093022],[15.072991],
 [52],[202],[18.999574],[18.808671],[17.910145],[17.910142],
)
Cold solve seconds, rounded from `results-v13/rows.csv`. Arms run b/u/v/x for
seed 101 and x/v/u/b for seed 202. Certificates match byte for byte across arms.
All clocks are shared-host L0 diagnostics, with wall_valid=false; no inferential
interval or certified timing ratio is available.

== Larger fixed targets: full-cache arm v
#table(columns:(0.5fr,0.6fr,1fr,1fr,1fr,1fr),inset:4pt,
 [Bits],[Seed],[Cold solve (s)],[With audit (s)],[Relations],[Attempts],
 [61],[101],[1149.580269],[1149.605554],[1617],[2292],
 [61],[202],[1398.287720],[1398.337550],[1591],[2432],
 [62],[101],[2152.177507],[2152.226223],[1724],[2675],
 [62],[202],[2274.229212],[2274.281053],[1771],[2485],
)
At 62 bits, oracle time was 2150.131104 / 2272.074842 seconds and linear
elimination 1.654154 / 1.750970 seconds. Table admission estimates were
60,880,922,152 / 68,978,885,440 bytes, including cache storage; they are
not measured peak VRAM. Both targets cached every right row (2,276,312,640 /
2,759,268,600). No prescribed cell timed out, refused admission or failed.

#image("scaling-v13.svg",width:100%)
Every recorded execution is shown. Colors denote arms, not independent hardware.

#pagebreak()
= Contract, validation and custody

Measured source: `e9e87ffb83807d7505c29fa04358d104e72d4b20`.
Pod: `7tugqyoxyvnikm`, created 2026-10-30T17:57:20 UTC. NVIDIA telemetry reports
97,887 MiB, driver 595.91.07 and compute capability 12.0. The build used CUDA 13.2,
sm_120 and Rust 1.88.0. Provider rate was USD 2.49/hour; this is not a final bill.
The controller verified archive integrity and pod deletion after completion.
A subsequent complete provider listing independently confirmed absence.

Command: `bash research/explicit_relations_e2e_20261030/run-cachefull.sh`, with
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

Cold solve includes per-target GP generation, process startup, validation, all base construction and
relation attempts, elimination, extraction, in-process replay, serialization
and teardown. The separate audit total adds fresh
replay and negative control. Build/preflight and curve fixture generation are
outside cold solve. CPU user/system time and measured peak VRAM were not captured.

Frozen protocol hash:
#text(size:8pt)[`9ab5b955afbc2ba7f9878793a8012b0b740b29fa5107c40b5d8adb23fb012413`]
Raw archive SHA256:
#text(size:8pt)[`f00ea3e1ef9d32ebbf2063cb43b99939a42a19f31085c0f1ccb1d32d8e083338`]
Archive: `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-v13/results.tgz`.
Raw records were committed and pushed in `84d5159e3635` before this report.
The archive manifest hashes every member; certificate-audit.json retains
native replay and rejection outputs. These manifests do not replace replay.

Schema, flow and file hashes pass. Native analysis and the strict coverage
audit pass; the latter accepts a valid first-rung stop and rejects nine deliberate
coverage/status/input mutations.
The canonical checker retains five rectangular-coverage findings: 24 assumed
rows versus 12, b/u/x each cover one rather than three sizes, and arm instance
sets differ. The frozen plan prescribes four arms only at 52 bits and v alone
at larger sizes. This is not a full canonical-checker pass or independent
scientific validation. See `results-v13/CHECKER-NOTE.md`; no row was excluded.

== Implementation context and next steps
PERSISTENT-QUERY.md and FULL-RIGHT-CACHE.md describe the two distinct changes,
with NVIDIA CUDA/CUB references. Both 61-bit cells also used complete right
caches. These per-run observations do not establish a cross-version speedup.
V15 failed source checkout before compilation or data; its archive and deletion
are preserved separately. A new, separately frozen infrastructure attempt will
validate 63-bit arithmetic and the optional negative filter. The largest verified
rung remains 62 bits. No timing result establishes a global hardware maximum.
