# Signed Kummer v2 source review preread

Date: 2026-11-11 UTC

## Subject

This is an independent, source-only review of the corrected packets:

- `signed-kummer-combined-runner-v2-20261111`
- `signed-kummer-lock-gate-v2-20261111`

The producer-reported packet-manifest SHA-256 values are respectively
`6467e061c332c1f8ebeb251d4a3c8a643b94bfe2577353c3345a49ff996e7a51`
and `74dd88670a3635736684af62c6470f3cfa67c8ea8d9d6c3c25e41bc56469dbfb`.

## Prior and known attacks

The prior review identified three concrete defects. The review therefore starts
skeptical that the revision closes them and checks each correction directly:

1. F1: the staged driver imported `driver-controls.rs`, but the old staged-input
   closure contained only 49 inputs and omitted that file. Attack: enumerate the
   current input set and trace every source/destination/hash binding used by the
   staging code.
2. F2: scalar-output agreement did not prove the trace header identified the
   intended source revision and staged source hash. Attack: inspect the parser,
   every applicable trace header, expected identities, mutation controls, and the
   intentional pre-header empty-output fault.
3. F3: adapter lock generation was ambiguous through a nested workspace.
   Attack: inspect the standalone adapter manifest/workspace boundary, the
   generated lock destinations, and both independent package selections.

Additional attacks are: dropped or duplicated panel members; deadline resets
between phases; a per-command cap exceeding the shared attempt cap; a campaign
ceiling/deadline correction that overwrites its initial binding; and prose or
machine state that implies execution, locks, authorization, or remaining budget
despite null/unadmitted receipts.

## Required static checks

- Pin and verify both packet manifests, every listed file, and the exact 50
  staged inputs including driver controls.
- Verify the full planned panel is 28 signed fixtures, 7 label fixtures, 11
  driver fixtures, 2 probes, and 4 negative controls, without a success-only
  subset.
- Verify trace identity is checked against both source revision and staged-source
  hash, while the intentional empty-header fault remains an explicitly expected
  negative case.
- Verify the lock gate uses one shared 45-second resolver deadline inside a
  60-second total cap, and the combined attempt uses bounded build/execution
  phases inside a 2,400-second total cap.
- Verify the current campaign ceiling is 129,600 worker-seconds with deadline
  `2026-11-11T12:00:00Z`, while the initial binding remains preserved and
  actual prior use, remaining budget, lock receipts, and admission remain null
  or false.

## Boundaries

No compiler, dependency resolver, test, fixture execution, source application,
Git index mutation, or canonical-journal write is authorized. This review can
establish only static source and custody properties; runtime properties remain
unchecked until a separately admitted execution produces receipts.
