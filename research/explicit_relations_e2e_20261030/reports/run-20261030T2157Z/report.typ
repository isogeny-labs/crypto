#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Both 63-bit targets recovered on one RTX 4090

The smaller explicit bases completed the entire ten-execution V21 panel:
eight 52-bit controls and two 63-bit prime-field targets. Every scalar passed
a fresh CPU certificate replay and the scalar+1 negative control. This is a
completed prescribed rung, not a measurement of the GPU's global maximum.

#table(columns:(0.6fr,1fr,1fr,1fr,1fr),inset:4pt,
 [63-bit seed],[Audit total (s)],[Cold solve (s)],[Relations],[Target queries],
 [101],[2600.976385],[2600.961379],[338],[15,737],
 [202],[1192.904874],[1192.887598],[344],[13,733])
Measured audit totals include fresh replay and negative-control execution.
The relation oracle accounts for2600.586033/1192.429319 seconds respectively;
linear algebra takes40.328/41.349 milliseconds. These L0 shared-host clocks
are descriptive (wall_valid=false); no controlled timing advantage is claimed.

== Same-service 52-bit controls
#table(columns:(0.6fr,1fr,1fr,1fr,1fr),inset:4pt,
 [Seed],[b: 2R+2],[u: ceil(1.5R)+1],[v: ceil(1.4R)+1],[x: repeat v],
 [101],[3.044364],[4.542738],[6.634252],[6.626250],
 [202],[2.723673],[4.689136],[4.806434],[4.829474])
Cold solve seconds from rows.csv. Every arm uses twelve slots and
R=ceil(r^(1/12)). Smaller bases reduce relation counts from251/261 (b) to
185/192 (v), but increase queries from341/306 to5947/4008 and increase the
observed cold time. Same-threshold v/x certificates are byte-identical;
recovered scalars agree across thresholds. Order is b/u/v/x then x/v/u/b.

#image("scaling-v21.svg",width:100%)
All ten recorded executions are plotted individually from results-v21/rows.csv.
The two63-bit points are armv; intermediate sizes53–62 were not prescribed.

#pagebreak()
= Memory admission, validation and provenance

#table(columns:(0.6fr,1fr,1fr,1fr),inset:4pt,
 [63-bit seed],[Left rows],[Complete right-cache rows],[Admission bytes],
 [101],[259,713,792],[786,806,784],[16,008,818,600],
 [202],[549,596,520],[444,762,500],[14,920,341,256])
Admission is a conservative allocation bound, not measured peak VRAM.
Both large cells fit their complete right cache. Partial-cache fallback was
checked separately with forced-fallback fixtures; partial allocations remain
charged even when full caching wins. V19's recorded memory/work refusals remain
unchanged; V20's preflight compilation failure produced no experiment cells.

Hardware: one NVIDIA GeForce RTX4090, 24,564MiB, driver595.91.07,
compute capability8.9; CUDA13.2 / sm_89 and Rust1.88.0. RunPod pod
g2naamm7l8d25t in US-NC-1, provider rate USD0.89/hour. Controller, bootstrap
and validation all exited0. The complete archive was downloaded and hashed;
pod deletion and absence from a fresh full provider inventory were verified.

Command: bash research/explicit_relations_e2e_20261030/run-memory.sh.
Public prime-order fixtures, seeds101/202, p and r below2^63. The driver
checks primality, curve membership and the unique Hasse-interval order witness.
Target coordinate lifts supply no logarithm to the reduction. All arms use
128 times (columns+1) target queries per trial, at most16 trials. The existing API
default stays20. Caps: rows2^32, work2^34, chunk2^20, columns16384,
reserve25%; solves180s/7200s at52/63bits, fresh replay60s, fixture180s,
worker18000s/controller18300s. These explicit thresholds do not assume the
paper's sufficient L>=10m condition.

All29 Rust tests, host/device differential checks, Clippy, nine query variants
under each of memcheck/racecheck/synccheck (27 checks), service certificate
checks and cap headers passed before data. The exact10-cell native coverage
audit passes its fault-injection controls. Schema and file hashes pass.
The canonical rectangular checker preserves five findings:16 assumed rows
versus10, b/u/x cover one size, and arm instance sets differ. No planned native
cell is missing or excluded. Canonical findings remain in canonical-check.log.

Cold solve includes input generation, startup, validation, base construction,
all relation attempts, elimination, extraction, internal replay, serialization
and teardown. Build/preflight and shared curve generation are outside it.
CPU user/system time and peak VRAM were not measured. Periodic archive uploads
add host I/O and do not restore solver state. No external-tool timing comparison
was prescribed; this is a configuration/admission and recovery observation.

Source: 253abb737139e0e0eaac49bc2bb8a296f353f67c, clean source archive.
Protocol SHA256:
#text(size:8pt)[e15407b5acb04f557cfb0476dbe8ae00a4973583f6bf7be3e7435962d125174b]
Archive SHA256:
#text(size:8pt)[08b89e95da968f85bdf4f7d8c1f8d789cb9bc69d912083258fcf8219db65f50a]
Raw records were pushed as ee2a1cb4c613 before this report. Full certificates
and traces are in validation/ecbench-explicit-x/e2e-20261030-v21/results.tgz
in the approved artifact bucket; archive-manifest.json hashes every member.
Literature and threshold rationale: SMALLER-BASES.md and FULL-RIGHT-CACHE.md.
Next: specialize affine subtraction in the dominant query path; the separate
NEXT-AFFINE-QUERY.md records EFD formulas and required exceptional cases.
That candidate is not yet implemented or measured.
