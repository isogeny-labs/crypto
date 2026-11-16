# Wide-field constructor preflight

2026-10-25, before output. Use source-c46089aed's frozen after-exclusion
constructor probe, with exact inputs `wide 16`, `wide 17`, `wide 18`, in
that order. No RNG seed or curve instance is involved in field construction.
The constructor verifies a full primitive cycle for the base field and
the quartic rank/unit irreducibility criterion, returning exact coefficients.

Record every process and output in a new directory, with command, binary
hash and source pin, 12-GiB address-space and 900-second limits per input.
Retain any panic, timeout or unsupported result and continue to later m.
A successful constructor establishes a field representation only. Point,
group-order, high-scalar, serialization and full IC integration checks
remain separate; no DLP, native price, throughput or crossover follows.

This complements the frozen exhaustive/independent arithmetic controls;
it does not substitute for those tests. Record the native output/statuses
and provide an inspected descriptive report with the other core controls.
