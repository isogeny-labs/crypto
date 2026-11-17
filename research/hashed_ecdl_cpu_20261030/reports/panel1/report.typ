#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU complete recovery: initial cache
Observations only; no hypothesis promotion or certified latency claim.
Run: `hashed-ecdl-cpu-panel1-v3-20261030`, October 29, 2026 PDT.

The original CPU oracle (u), reusable tables with batch 64 (v), batch 1 ablation
(w), and identical reference repeat (x) ran on one software thread in an EKS
job reserving two vCPUs and 4 GiB on AMD EPYC 9R45. The main-built runtime image
is distinct from the pinned standalone CPU instrument. All clocks are L0.

#table(columns: (1fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Original u, ms], [Cached v, ms], [A/A x, ms],
 [8-bit, 3 slots], [0.380702], [0.352192], [0.380672],
 [16-bit, 3 slots], [53.021651], [46.852160], [53.035951],
 [16-bit, 4 slots], [89.756843], [34.865870], [89.855764])
Medians of nine measured repetitions; one warmup per arm and fixture. Exact
curves, public known-answer targets and seeds 111/222/333 are in the protocol.

#image("../../results/latency.svg", width: 100%)

All 120 requested executions completed (108 measured, 12 warmups), with
byte-identical complete certificates across arms. Scalar recovery, checked group
replay and deliberately corrupted scalar rejection passed. The interval includes
curve and target construction, all setup and trials, relation collection,
elimination, internal replay, second replay and serialization. Process launch,
archive I/O and the negative control are excluded. Original zero table/inversion
counters are uninstrumented/inapplicable, not zero arithmetic.

Two staging jobs failed before any measurement: bucket HeadObject 403, then
missing tar. Both receipts are retained; protocol versions 1–3 document the
transport corrections. Final artifacts used scoped presigned URLs and standard
archive handling, with the native benchmark and analyzer unchanged.

Source `b02a2f2dd130`; full source/binary hashes: `results/build.json`.
Protocol hash: `83ad81c97c9919b34f918bdfa0d3d1a6d645512f4a6dfa4f97f16a16804bfcea`.
Raw rows, full certificates, stage ranges and checked export: `results/`.
Matched rho and fastest external comparisons are absent; their gaps are unknown.
This generic decomposition result is not Boolean tensor contraction or evidence
of asymptotic improvement. A later panel separately tests a parameter-only key.
