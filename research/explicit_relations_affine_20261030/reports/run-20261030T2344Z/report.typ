#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Affine-query recovery comparison on one RTX 4090

The opt-in affine-input subtraction was compared with the existing mixed
formula on identical public targets, explicit bases, cache layouts and limits.
Recorded executions: 12; verified recoveries: 12.
Each successful result passed fresh CPU certificate replay and scalar+1
rejection. Complete certificates match byte for byte across formulas and repeats.
Affine cold time was lower for both 52-bit targets and the 63-bit seed 202
target, and slightly higher for the 63-bit seed 101 target. The option remains
off by default because the large-target effect was not uniform.

#table(columns:(0.4fr,0.5fr,1.2fr,1.2fr,1.2fr),inset:4pt,
 [Bits],[Seed],[b: mixed],[v: affine],[x: mixed repeat],
 [52],[101],[7.213224 / succeeded],[6.979429 / succeeded],[7.177102 / succeeded],
 [52],[202],[5.238853 / succeeded],[5.163135 / succeeded],[5.241295 / succeeded],
 [63],[101],[2636.991553 / succeeded],[2644.849780 / succeeded],[2636.255738 / succeeded],
 [63],[202],[1207.318937 / succeeded],[1190.595553 / succeeded],[1206.938904 / succeeded])
Cold solve seconds and native status, rounded from rows.csv. All rows are
retained. The solve interval includes target generation, process startup,
validation, base/cache construction, relation collection, elimination,
extraction, internal replay, serialization and teardown. Fresh replay and
negative-control time are included separately in wall_seconds.

#table(columns:(0.5fr,0.5fr,1.5fr,1.5fr),inset:4pt,
 [Bits],[Seed],[Candidate / mean(b,x)],[AA spread / mean(b,x)],
 [52],[101],[0.970017],[0.005020],
 [52],[202],[0.985317],[0.000466],
 [63],[101],[1.003120],[0.000279],
 [63],[202],[0.986304],[0.000315])
Ratios are the frozen analyzer's outputs: candidate_seconds / mean(b,x)
and abs(b-x) / mean(b,x), separately per fixed target. Values below 1 in the
candidate column mean lower observed cold time. These L0 shared-host times
are descriptive; the repeat spread is not a confidence interval. No population
or cross-panel timing inference is made. Ratios require all three verified results.

#image("scaling.svg",width:100%)
Every execution from rows.csv is plotted separately, including any explicit
limit or timeout. Two fixed targets at each prescribed field size.

#pagebreak()
= Formula scope, checks and custody

The source uses the established EFD mmadd-2007-bl affine-input formula:
4M+2S instead of 7M+4S in the nonexceptional addition formula. These are source
formula operation counts, excluding conversions, normalization, lookup and
memory traffic. Ordinary affine cache storage is unchanged. Infinity, inverses,
doubling and zero-y doubling have explicit branches. Partial-cache and direct
sum fallbacks retain mixed arithmetic; all three search entry points share
one residual helper and preserve exact minimum witnesses.

Before data: 29 Rust tests, host/device differentials, Clippy, the original 27 query
sanitizer checks and 12 affine-specific checks passed. The affine arithmetic
fixture checked 37,584 cases on both CPU and GPU against an independent ordinary
coordinate affine law, including exhaustive small curves and 63-bit cases.
The affine packed/unpacked/persistent query paths passed memcheck, racecheck
and synccheck; full service, certificate and cap-header checks also passed.

Command: bash research/explicit_relations_affine_20261030/run.sh.
Fields: 52 and 63 bits; seeds: 101 and 202; 12 slots; R=ceil(r^(1/12)),
tau=ceil(7R/5)+1. Query budget: 128 times (columns+1); maximum 16 trials.
Caps: rows 2^32, work 2^34, chunk 2^20, columns 16384; reserve 25%.
Order: b/v/x for seed 101, x/v/b for seed 202 at each size.
Baseline repeat uses the exact baseline binary.
Every successful cell must have a complete right cache and nonzero filter.
No sufficient paper threshold guarantee is assumed for these explicit bases.

One NVIDIA GeForce RTX 4090, 24,564 MiB, driver 595.91.07, CC 8.9;
CUDA 13.2/sm_89, Rust 1.88.0. Pod hnvmwjamem9ox9; provider rate USD 0.89/hour.
Per-solve caps: 180/7200 s; replay: 60 s; fixture: 180 s;
worker: 18000 s; controller: 18300 s.
Periodic artifact copies every 60 s preserve evidence, not process restart. CPU
user/system time and peak VRAM were not measured. Admission bytes are bounds,
not observed peak usage. Build/preflight and shared curve generation are
outside solve intervals. The code uses generic decomposition; no external-tool
timing comparison was prescribed for this formula-only diagnostic.

Source: 333712634ae2fa0694236a530b7a5efd1be3efc5.
Protocol SHA256:
#text(size:8pt)[2cfc1be77efbdc0d65031c1a42a19a72aca407aa69459e4440731993a92dad66]
Archive SHA256:
#text(size:8pt)[4758f67ba19c0c12fe119f28404fd3c94b6c3dae481c630700d7ac0b179b24ba]
Raw records pushed as c79bd38363f9de485087a0e206032cf5938dbdae before this report. archive-manifest.json
hashes every raw certificate/log in the approved S3 archive under
validation/ecbench-explicit-x/affine-20261030-v1/results.tgz.
Controller, bootstrap and validation exited 0; final archive integrity and
pod deletion with fresh provider-inventory absence were verified.
The native analyzer checks coverage, outcomes and equal relation/query counts;
its malformed-input tests are documented in README.md. Canonical validation
results are preserved with the raw export. Neither software checks nor this
report promote an independent scientific-review verdict.

Primary formula source: https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html.
Implementation assumptions and fixtures: gpu/explicit-relations/AFFINE-QUERY.md.
