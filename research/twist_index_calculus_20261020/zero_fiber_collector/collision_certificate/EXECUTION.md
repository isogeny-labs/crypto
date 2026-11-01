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

## Corrected gate v3, identity certificate and preparation v2

Gate v3 passes all five Rust tests, the build and all three Lean lemmas,
with terminal exit 0 and unchanged source seals. The new regression check
requires the actual completed Phase 1 schema and rejects censors/timeouts.
Executable SHA-256:
`4c49d77faa9b5ef3fd54459bd0ac0cf69ee077eef43ec84361753f7dbc3555f5`.

The separate native IDC wrapper builds and issues/checks
`IDC1h10810f17e0f6c8f6`, with 64 derived plus 64 extra points and a digest
pointer to the already checked Lean source. Its false-identity control is
refused. All source seals and phase exits pass. The initial dependency
inspection found cached serde 1.0.229; the wrapper pins that exact version.

Preparation v2 passes and produces all 42 equation-only input packs.
Its workload SHA-256 is
`17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10`.
The first two rows are p37 seed 0, phi then doubling. The actual driver
controls pass: outer reservation 64 with no output; duplicate phase 65 with
an unchanged marker and no candidate directory. The exact tested driver is
unchanged before registration. No producer case has started at this freeze.

The two actual pages of layout v1 were inspected: page 1 had labels extending
outside diagram boxes. The original PDF/PNG and sources are retained.
Layout v2 corrects the labels, and both actual pages are inspected with no
observed clipping. PDF SHA-256:
`be0b9802e20659424dad22381266edba708ef50a419e5187c03e62e4fbc7356d`.
Matching PDF/SVG/150 dpi PNG copies remain unlisted in benchmarks.
