# Follow-up replay review, 2026-10-29

Reviewed runtime: `6542d87e00902c0a4219db7c4a3812ccc9c02bf3`. Read-only follow-up by the original implementation author; not an independent implementation review. Findings were identified from source control flow, before executable mutation tests. The existing frozen full-library process continues on this source; its result cannot validate later corrections.

## Findings requiring correction

1. Separately saved native prime-root call and selected state artifacts receive hash verification but are not compared with the corresponding records in the verified aggregate certificate. Rehashing a mutated auxiliary root or arithmetic state can therefore preserve certificate validity while misrepresenting a retained intermediate. Bind unique expected paths, roles, encodings and payloads to each certificate query and acknowledged selected event. Add rehashed mutation, omission, role and duplication rejection tests.
2. The producer enforces the serialized-event byte cap and records `trace_byte_cap_reached`, but replay only recomputes outcome byte totals. It neither enforces the cumulative ceiling nor validates that flag against the report stop, rejected event envelope and byte-cap cause. A completed query with a toggled flag can pass. Enforce the same cap and terminal metadata semantics, including rejecting a cap flag on a completed query, without claiming complete execution or total operation counts.

The reviewer found no additional arithmetic, startup or prime-chain regression in the inspected changes. The native module remains byte-identical to the retained donor. This is limited source-review evidence, not proof of absence of other defects.

Both findings are assigned to the existing root integration scopes (`solvers.rs`, `mod.rs`). Binary integration remains design-only until the prime corrections and their fresh validation are complete. Earlier focused, CLI and S3 restore passes remain valid evidence for the cases they exercised; they do not cover these new tamper cases.
