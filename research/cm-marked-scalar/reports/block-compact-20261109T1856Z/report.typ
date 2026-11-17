#set page(paper:"a4",margin:14mm)
#set text(font:"Libertinus Serif",size:9pt)
#set par(leading:0.35em)
#align(center)[#text(size:18pt,weight:"bold")[Inline planner and complete cold integration] #linebreak() P-192 primary; P-224/P-256/P-384/P-521 retained #linebreak() Software qualification · 2026-11-09 · PR 3674]
= Completed component
The bounded inline planner emits the exact original block programs, including first-encountered ties, cost vectors, signs, table metadata and transition counts on the supplied qualification cases. It executes the same original scalar point action. All results below are producer-side unverified assertions pending independent keyring-signed PCC standing. [pcc:unverified]

#image("layout.svg",width:92%)

#table(columns:(3fr,1fr),align:(left,right),stroke:0.5pt,inset:4pt,
 [Qualification check],[Passed cases],
 [Exact original-program and GMP reconstruction],[4,950],
 [Independent small exhaustive minima],[1,800],
 [Original-scalar point / full-count cases],[3,150],
 [Independent retained-bank point labels],[35,760],
 [Maximum accepted scalar, both signs],[2],
 [Invalid domain / inline overflow rejected],[7],
 [Shared complete cold paths, untimed],[625],
 [Wrong expected-output controls rejected],[5])

These are finite deterministic software checks, not timing samples; no uncertainty or speed ratio is inferred. Both builds and both qualifiers exited 0, without compiler warnings. Planned 4 invocations, attempted/completed 4, failed 0, unchanged retries 0. [pcc:unverified]

= Representation and full-cost boundary
A fallback column needs at most two signed terms; a folded block needs at most one. Steps, transitions and DP nodes now hold an inline two-term array. The label set becomes a bounded bitset: maximum radix 32 times largest odd multiplier 63 gives maximum label 2016. The original accepted recoded length 2048 and block span 16 remain. The output reserves its selected step count once. Big/digit vectors, DP state/result vectors and prefix allocations still exist; no total-allocation claim is made.

The cold wrapper substitutes only the block types/planner/executor in the existing worker. Request parsing and fresh curve/point setup, recoding, bitset initialization, DP allocations/transitions/reconstruction, point-bank construction/normalization, arithmetic, symmetric output verification and required destruction remain inside its clock. Original points/scalars and all regular/binary reference arms remain. No input-dependent plan or point bank is reused. The five exclusive phases and nested arithmetic partition are unchanged.

= Publication, limits and remaining measurement
Core source/launch 1a1c09f3703c was hosted and PR updated before its build/qualifier. Shared cold wrapper/source 084524a3d150 was hosted and PR updated before its separate build/qualifier. Exact commands, source snapshots/hashes, publication receipts, binaries, outputs, stderr and exits remain in the two qualification directories. Raw records were pushed before this report. [pcc:unverified]

Each of four native invocations was capped at 120 seconds and 2 GiB address space, with one process/thread. These are ceilings, not measured CPU durations. No census, allocator measurement, timing worker or repeated preflight ran. No cloud job, resource/role/budget change or generator/controller restart. No Rust code changed; no local root Cargo suite is claimed. [pcc:unverified]

The original e2e objective still needs an admitted complete comparison. The user's quiet-window start/readiness question is pending; V4's 4.24-second other-CPU refusal remains unchanged. A later timing amendment must bind the new binary/source, preserve all references and charge every planning/setup cost. This qualification neither declares a speedup nor silently changes the original frozen protocol. Signed PCC registration/review also remains a separate obligation. [pcc:unverified]

#text(size:8pt)[Evidence: ../../qualification/block-compact-v1/ and block-compact-cold-v1/, source.sha256, publication.json, binaries.sha256, qualifier.json and exits; ../../qualification/block-compact-session.json; ../../BLOCK_COMPACT.md. Literature basis: existing block/cold contracts and cited Bernstein-Lange precomputation background; KB tools/wrapper unavailable. Same recurrence, no novelty/global-optimality claim.]
