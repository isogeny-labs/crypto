# Complete-solve instruction attribution diagnostic

2026-10-25, frozen before these profiles. Follow the repository profile
workflow and ecbench's documented Callgrind solve/target markers. This is
instruction attribution under Valgrind, not physical performance or a new
complete-price comparison. Valgrind replaces the native allocator; its Ir
omits the real allocator's work. All existing cost obligations stay open.

Profile all ten executions of the sealed n=13 structured smoke, in sequence
order 0--9, using its unchanged spec and the frozen ecbench binary
efa25f209e57dd30fe191ddbd3a3c0d8465952cbe0a23b38a9dc7450f8051c9c
(adapter build 0a5137c2). This preserves both independent public one-target
workloads and all five arms: strong signed-Frobenius rho and its identical
repeat, direct S4, symmetric S4, generic three-summand MITM. No target or
arm is chosen by the profiling outcome.

Export the exact child input using profile-input. Enable the complete-solve
and target-only markers. Run Callgrind 3.24.0 with cache/branch simulation
off, one Rayon/OMP thread, 360 seconds per profiled child, retaining timeout
and missing-marker outcomes. The local shared host earns L0. Profiles run
sequentially and may share the host with the registered native queue; their
wall/CPU times are diagnostics only. No address-space limit is placed around
Valgrind because its shadow mappings reserve large virtual ranges; these
small frozen inputs and the timeout bound execution. Record actual RSS.

Use the frozen native callgrind-ir and callgrind-online-ir parsers. Require
the exact solve/online marker bounds and sum identity. Retain every raw
part, child output, process exit, command and source/input/binary/package
hash. Independently check recovered scalars, phase/counter/GAE equality
against the sealed native records; a dispatch or count mismatch stays
visible and prevents an equal-work comparison. Missing output is unknown.

Report instruction attribution for counted and previously uncounted work,
including construction and splitting where the profile resolves it. It
does not supply missing native field/word counters, physical calibration,
isolation, or a runtime speedup. Do not promote any leaderboard or benchmark
row. Produce a source-linked profile report, graph and inspected PDF; raw
profile archives follow the artifact-store policy without modifying records.
