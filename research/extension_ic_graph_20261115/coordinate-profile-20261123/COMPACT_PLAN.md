# Compact coordinate profile follow-up

The first two profiles include loading/hashing the full105MiB saved report.
Keep those original profiles unchanged. This follow-up uses the same first32
systems and34roots, extracted by native jv_sieve_unit_profile_fixture.rs and
bound to the original parent SHA plus the published compact fixture hash.
No curve, target, seed, selected system or expected root changes.

One native binary, degree24/staircase64, original F4 flags and one thread.
Profile six-variable, old unit, normalized unit, normalized-unit A/A repeat,
and normalized-unit trace paths in fixed order. Callgrind instruction counts
only; each has a600second cap and no automatic retry. Every call checks all
captured roots/original equations. The trace arm clears scope5once before
its first system and charges recording plus every checked shape fallback.
The A/A repeat bounds nondeterministic allocation/hash/setup effects; no small
instruction difference is automatically a time improvement. Profiles cannot
admit throughput or rho parity. Input compaction removes unrelated diagnostic
loading rather than changing the coordinate workload. Native all3415system
traced correctness replay independently matched3419roots,3348replays and66
shape fallbacks with zero failures; no new natural-yield call was made.
