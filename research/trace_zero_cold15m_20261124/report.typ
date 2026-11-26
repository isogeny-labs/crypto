#set page(paper:"a4",margin:16mm)
#set text(size:9pt)
#set table(inset:5pt,stroke:0.4pt+luma(180))
= ECC2K-130 index calculus: below 15M cold instructions

The unchanged n131 d12 uniform query uses *14,688,697 total simulated instructions* in each of five repetitions, below the user's strict15,000,000 target. Matched baseline:*20,779,444Ir*; *29.31% fewer*, *1.414655x*. Cold setup:4,578,829; complete oracle:10,024,047; post/original replay/output/exit:85,821Ir. These phases sum to the whole cold count. PCC-20261124-807b8e. [pcc:unverified]

== Fixed workload and complete boundary
All process parsing, field/context construction, factor-base admission, index allocation, projected systems, root checks, relation replay, output and exit stay charged. Same frozen inputs, order and semantic outputs; no warmed state or cross-query reuse. Instruction counts are Callgrind Ir, not native instructions per second or a native runtime measurement. The final negative query certifies no decomposition in its supplied factor base; it is not a full discrete-log recovery.

#table(columns:(2fr,1fr,1fr,.8fr,.8fr),[*Case*],[*Baseline cold*],[*Candidate cold*],[*Cold saved*],[*Oracle saved*],
[n83-positive], [13,428,874], [10,305,377], [23.26%], [12.62%],
[n83-uniform], [18,511,226], [14,515,756], [21.58%], [14.30%],
[n83-cancellation], [9,497,932], [6,880,997], [27.55%], [12.32%],
[n131-positive], [14,172,188], [10,174,756], [28.21%], [19.86%],
[n131-uniform], [20,779,444], [14,688,697], [29.31%], [25.44%],
[n131-cancellation], [10,905,013], [7,447,568], [31.71%], [23.04%],
)

All six controls pass cold/oracle no-regression limits against both matched controls; A/A uses a source-identical baseline under another arm name. Five repetitions per cell; exact min/max in results-table.md. Deterministic repeats establish repeatability on this fixed grid, not a population confidence interval. Ratios come from each panel's own matched controls, never compounded across compiler settings.

#figure(image("instructions.svg",width:100%),caption:[All three candidate panels retained. First panel uses the earlier compiler profile; second and third use equal full LTO/codegen-units1 on every arm.])

#pagebreak()
= Implementation, checks and custody

== Qualified changes
Defer unused symbolic polynomial-reduction tables with a shared OnceLock; symbolic preparation remains available. Share the existing exact trace/half-trace table with compact arithmetic. Evaluate projected row maps in word-local chunks. Validate ASCII hexadecimal chunks in SIMD registers, with the original extended parser as fallback. Use runtime-guarded carryless canonical131 square/multiply, cache feature selection, and reuse consecutive equal nonzero denominators inside the original bounded simultaneous-inversion batches. Every point still undergoes its original admission, ordinate/group checks and replay. The prior direct-address dictionary and original order remain intact.

This is an implementation bundle and equal compiler configuration, without isolated per-change attribution. Portable paths remain; the measured hardware class is x86-64 with pclmulqdq/SSE2. Other classes are unmeasured.

== Exact checks and flow
62 focused release tests pass, including every131-bit basis product, separate square checks, all256 byte values at16 parser positions, exhaustive small-field symbolic evaluation, and repeated/exceptional denominator cases.65 retained cases match all three executable variants (195 native rows). Full primary: planned120, attempted120, completed120, excluded0, retries0;240 fresh measurement children (phased/unphased). Separate schoolbook polynomial arithmetic replays80 saved relation tuples and checks the retained negative cases. All three strict exports have zero findings. Current frozen source inventory is unchanged. Producer checks are unsigned, not independent admission. [pcc:unverified]

== Retained misses and launch corrections
First full panel:17,287,437Ir on the uniform n131 query, missing15M and failing one n83 positive oracle guard. Second:16,033,438Ir, all guards pass but target missed. No panel is removed. Two third-candidate preflight launch errors happened before any measurement child: stale study-directory text, then an extra absolute-path prefix. Erroneous bytes and invalid records are retained; affected earlier metadata restored exactly. Final runner derives its directory and refuses overwrites. The third candidate's original epoch1795491725 was never reset; primary completed2026-11-24T03:51:45Z within the original600seconds. A three-instruction baseline typo in an initial claim is corrected by PCC-20261124-807b8e; frozen summary is authoritative. [pcc:unverified]

== Source identities and reproduction
Final numerical source1861f64e14; corrected runner binding781e12c34c; protocol publication9e2d87e27f before primary data. Protocol SHA6ffbb8617211d163d4d24f7fd9b2d7af13ca1aa0ad96816287180226e5d00060. Candidate ELF SHA0c95b811c80e1669b645480a2c52b7635f2f78435f7d54748b4ab5da78d50d23. Inputs and all executables are archived locally with per-member hashes, extraction byte checks and strict exports. Large new objects remain local pending original-policy store upload; pending-artifacts.json enumerates their exact hashes and sizes. No object is marked uploaded. Native execution/harness/replay are Rust; Python handles metadata/custody/layout only.

Local CPU0,2GiB child address space,600seconds per panel, zero paid resources and zero infrastructure retries. This report closes the15M rung. Earlier50x goal remains future work. Broad root-library test execution remains blocked at dependency compilation in retained attempts; focused actual-source workspace checks pass. Remote artifact upload/checkpoint custody retains its prior403 blocker, original missing leaderboard source prevents regeneration, and signed independent admission is pending. No admitted leaderboard row or gate is changed. KB wrapper unavailable127; no claim of fastest external solver or mathematical trace-zero breakthrough. [pcc:unverified]
