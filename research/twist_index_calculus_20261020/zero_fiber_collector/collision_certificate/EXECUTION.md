# Execution history

## Native/proof gate v1

Source committed in `7758998be9`, with proof capture added in `8b317f051f`,
before the gate starts. Raw output is
`../runs/collision-certificate-native-gate-v1`.

- Lockfile generation exit 0; native tests: 4 passed, 0 failed.
- Production build exit 0.
- Lean exit 1: `power_zero`'s zero-exponent branch failed to derive the
  zero-ring consequence using `grind`. The compiler's axiom output contains
  `sorryAx` for that failed declaration and its dependent rejection theorem.
  This is a failed check, not an accepted proof or a permitted research axiom.
- No actual retained S4 system was searched; the Rust tests use small
  synthetic polynomial systems. No new F4 or target recovery was run.
- No terminal success marker is inferred from the successful Rust phases.

The versioned correction replaces that auxiliary branch with explicit
multiplication identities. Gate v2 must check the actual corrected file,
its transitive axioms, all native tests and source seals before production.
The first gate and its original source snapshots remain unchanged.

Workload preparation, bounded driver controls, production source/workload
freeze and all 42 matched case-arms remain pending at this checkpoint.

## Native/proof gate v2 and preparation v1

Gate v2 uses committed source `47d1a8ea39`. All four Rust tests and the
production build pass. Lean 4.34.1 (commit
`5045d0056413266e57c625dcd7c365b10e377c52`) checks all three lemmas: the
cubic and rejection theorems use only `propext`, `Classical.choice` and
`Quot.sound`; `power_zero` has no axioms. No accepted declaration depends
on `sorryAx`. The terminal gate exit is 0 and all source seals pass.
The native executable SHA-256 is
`2736b40cdd59987d08366231aad19426b75ad517ad167b9110a2953333b0fcd3`.

Workload preparation v1 exits 2 before writing a case input. Its parser
incorrectly expected the Phase 2 `pass` status in a complete Phase 1 record.
The actual retained schema is `twist-zero-five-polynomial/v1` with status
`complete-polynomial-enumeration`, zero undetermined systems and no
cooperative timeout. The old records remain valid and unchanged; this is a
preparation implementation error. A regression control accepts that exact
completed schema and rejects a pipeline status, an undetermined system or
a timeout. Gate v3 and fresh preparation v2 will check the corrected parser.
No actual case certificate search or new F4 solve has occurred yet.
