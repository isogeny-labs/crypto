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
