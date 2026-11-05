#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Twelve slots reduce 52-bit recovery work on RTX 4090

All eight prescribed 52-bit executions recovered and independently replayed
the public target scalar. On the same filtered meet-in-the-middle service,
12 slots used 251 / 261 relations, compared with 743 / 709 for 8 slots.
Observed cold solve time fell from about 9.7–9.9 seconds to 2.7–3.0 seconds.
The two 63-bit targets reached explicit limits with both tested arities.

== Matched configurations
#table(columns:(0.6fr,1fr,1fr,1fr,1fr),inset:4pt,
 [Seed],[b: 8 slots],[u: 10 slots],[v: 12 slots],[x: repeat v],
 [101],[9.725032],[4.967727],[3.032026],[3.048896],
 [202],[9.889092],[4.508011],[2.749641],[2.739248])
Cold solve seconds, rounded from solver_seconds in results-v19/rows.csv.
The full audit totals include fresh replay and the corrupted-scalar rejection.
Order is b/u/v/x for seed101 and x/v/u/b for seed202. Same-arity repeat
certificates match byte for byte; scalars agree across all configurations.
Shared-host clocks are L0 with wall_valid=false; these are descriptive timings.

#table(columns:(0.6fr,1fr,1fr,1fr),inset:4pt,
 [Seed],[8 slots relations / queries],[10 slots relations / queries],[12 slots relations / queries],
 [101],[743 / 1050],[383 / 510],[251 / 341],
 [202],[709 / 1289],[368 / 618],[261 / 306])
The 12-slot repeat has identical counts. Linear-algebra stage times were
159.268 / 141.308 ms for 8 slots and 7.643 / 11.975 ms for 12 slots.
Each successful 52-bit cell used a complete right cache and a nonzero filter.

#image("scaling-v19.svg",width:100%)
All twelve executions are shown, including the four fast limit exits.
Red points are limit outcomes, not completed recoveries.

#pagebreak()
= Memory and work limits, validation and custody

#table(columns:(0.6fr,0.6fr,1fr,1fr),inset:4pt,
 [Seed],[Slots],[Admission bytes],[Outcome],
 [101],[10],[57,354,429,359],[GPU-memory limit],
 [101],[12],[64,960,046,743],[GPU-memory limit],
 [202],[12],[3,373,077,439],[Right-side work limit],
 [202],[10],[67,478,106,239],[GPU-memory limit])
The 12-slot seed202 split has 139,110,048 left rows and 143,721,864,000
right combinations, exceeding the 2^34 decomposition-work cap. All four
63-bit cells have zero right-cache rows. Admission bytes are conservative
allocation bounds, not measured peak VRAM. No intermediate field sizes were
prescribed: 53–62-bit behavior on this GPU was not measured by this panel.

One NVIDIA GeForce RTX4090, telemetry 24,564 MiB, driver595.91.07,
compute capability8.9. CUDA13.2 / sm_89, Rust1.88.0. Pod25f10of8l9nl6i
started2026-10-30T21:17:08.633UTC inUS-NC-1 atUSD0.89/hour (provider rate).
Controller, bootstrap and validation exits are0. Final archive integrity,
pod deletion and a fresh complete provider inventory confirmed cleanup.

Command: bash research/explicit_relations_e2e_20261030/run-arity.sh.
Seeds101/202; tau=2ceil(r^(1/m))+2; rows2^32, work2^34, chunk2^20,
columns16384, trials16. Solve caps180s at52bits and3600s at63bits;
fresh replay60s, fixture180s, worker18000s/controller18300s. Finish every
prescribed cell at the current rung, then stop larger rungs after any limit.
The explicit threshold does not invoke the paper's sufficient L>=10m guarantee.

Source: e8e1c819df8901100db0254a849271130e6cca90. Prime-order public fixtures
are reproduced in curves.txt, with p,r below2^63. Targets are coordinate lifts;
no target logarithm is supplied to the reduction. The driver checks primality,
curve membership and the unique Hasse-interval order witness.
All28 Rust tests, host/device differential checks, Clippy, eight query variants
under memcheck/racecheck/synccheck, service checks and maximum-cap headers passed.
Every recovery passed a fresh CPU certificate replay and rejected scalar+1.

Cold solve includes per-target point generation, startup, validation, base
construction, all relation attempts, elimination, extraction, internal replay,
serialization and teardown. Build/preflight and shared curve generation are
outside that interval. Separate audit totals include the fresh replay and
negative control. CPU user/system time and measured peak VRAM were not captured.
Periodic partial uploads add host I/O; they restore no solver state.

Protocol SHA256:
#text(size:8pt)[b58d0dfe47bc039a6160ea7912374d37ff30f595e2daea262490beef58ac5422]
Archive SHA256:
#text(size:8pt)[d6c403d2eb5ccd4bc39a98432d38c70567efd9fd3f51445f47c4471305459237]
The archive is under validation/ecbench-explicit-x/e2e-20261030-v19/results.tgz
in the approved artifact bucket. Raw records were pushed in e1db39a661f3
before this report. Schema and file hashes pass. The strict native audit passes
exact coverage and nine negative controls. The canonical rectangular checker
retains four findings:16 assumed rows versus12, b/x each cover one rather than
two sizes, and arm instance sets differ. canonical-check.log preserves these.
Raw telemetry whitespace is preserved. A missing newer-version branch in the
post-run audit's negative-control generator was repaired before audit passed.

Next: reduce factor-base sizes to fit24GB, with an explicit bounded target-query
budget to allow complete relation collection from smaller bases.
