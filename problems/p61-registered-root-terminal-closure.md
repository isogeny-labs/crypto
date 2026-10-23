# p61 registered-root terminal closure mismatch

- **Status:** `investigating`
- **Kind:** implementation gap
- **Affected work:** [p61 terminal closure protocol](../research/p61_terminal_closure_20261023/PROTOCOL.md)
- **Next action owner:** Conductor `T-4942`
- **Last updated:** 2026-10-23

## Problem

The `p61-i0025` vertex census cannot admit its final segment because the
terminal degree-13 closure compares the generic canonical representative with
the registered root model coefficient-for-coefficient, although those models
represent the same `F_p`-isomorphism class.

The affected range is `[19890496,19894128)` in the 18-lane class lattice. This
blocks the contiguous `358,094,304`-vertex census gate and every downstream
global-uniqueness, full-adjacency, dense-index, and M7 source-admission gate.

## Evidence and uncertainty

- **Observed:** two preserved attempts produced an `8,304,984`-byte terminal
  partial with SHA-256
  `6b29341fd85d046decfcc4830b2b793d049f19895b712ec9923399f35edbaa5d`.
  Each advanced through `x=19894128` and then reported that degree-13 lane
  `(0,0)` did not close.
- **Derived or proved:** the registered root is distinct from the generic
  canonical coefficient pair, has the same j-invariant, and is
  `F_p`-isomorphic. The predecessor code canonicalizes an isogeny codomain and
  then compares that model directly with the registered seed.
- **Suspected explanation:** this representative mismatch is the only terminal
  defect. The full terminal generation and replay are required to distinguish
  it from an independent row-action problem.
- **Missing capability:** an admitted successor producer/verifier that links
  the terminal codomain directly to each stored lane seed while preserving the
  immutable predecessor checkpoint.
- **What this evidence does not establish:** it does not yet establish that all
  18 rows close, that the terminal segment replays, or that the complete class
  is globally unique.

## What would unblock it

- [ ] A committed and hash-pinned native successor passes the focused
  regressions and the repository's required test gates.
- [ ] The exact terminal segment generates and independently replays with
  `65,376` nodes, `18` closures, and `next_x=19894128`.
- [ ] An immutable admission manifest binds the predecessor checkpoint,
  successor sources and binaries, generated artifacts, replay receipt, and
  resource envelope.

These conditions resolve this implementation gap only. Global uniqueness,
complete adjacency, endpoint resolution, and hardness-model training remain
separate obligations.

## Next discriminating step

- **Action or proof obligation:** execute the frozen terminal-closure protocol.
- **Inputs and controls:** the exact `x=19890496` checkpoint, certified class
  number receipt, old failed partial hash, root-representative regression, and
  full native replay.
- **Execution limits and stop condition:** the protocol's 8-thread, load-12,
  and 1.5 TB free-space gates; stop and preserve evidence on any mismatch.
- **Outcomes and next actions:** a full pass admits only the terminal segment;
  a j mismatch reopens the row-order construction; a link/kernel mismatch
  reopens the certificate implementation; an operational interruption remains
  resumable but inconclusive.
- **Recheck trigger:** a completed terminal generation or a new preserved
  failure receipt.

## Resolution and history

2026-10-23: root cause localized and successor implementation started under
Conductor `T-4942`. Unresolved pending exact terminal generation and replay.
