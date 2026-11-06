# p61 registered-root terminal closure mismatch

- **Status:** `resolved`
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
- **Established explanation:** the representative mismatch was the terminal
  defect. The complete successor run closed all 18 lanes, and both native
  replay and a separate full-record structural audit passed.
- **Delivered capability:** the admitted successor producer/verifier links the
  terminal codomain directly to each stored lane seed while preserving the
  immutable predecessor checkpoint and source-transition identity.
- **What this evidence does not establish:** terminal closure does not by
  itself establish that the complete class is globally unique or that all
  selected-degree adjacency endpoints have been generated and resolved.

## What would unblock it

- [x] A committed and hash-pinned native successor passes the focused
  regressions and the repository's required test gates.
- [x] The exact terminal segment generates and independently replays with
  `65,376` nodes, `18` closures, and `next_x=19894128`.
- [x] An immutable admission manifest binds the predecessor checkpoint,
  successor sources and binaries, generated artifacts, replay receipt, and
  resource envelope.

These conditions resolve this implementation gap only. Global uniqueness,
complete adjacency, endpoint resolution, and hardness-model training remain
separate obligations.

## Next discriminating step

- **Action or proof obligation:** complete the external uniqueness index and
  selected-degree endpoint accounting over the now complete census.
- **Inputs and controls:** the gap-free 358,094,304-vertex census, immutable
  segment manifests, exact directed-edge target, and terminal audit-chain tip.
- **Execution limits and stop condition:** retain the campaign's load and free
  space gates; stop and preserve evidence on any digest, endpoint, or
  multiplicity mismatch.
- **Outcomes and next actions:** exact uniqueness and resolved reciprocal
  adjacency admit the graph-derived learning table; any collision or missing
  endpoint opens a separate data-integrity problem rather than reopening this
  terminal implementation gap.
- **Recheck trigger:** a completed uniqueness pass or a preserved downstream
  integrity failure.

## Resolution and history

2026-10-23: resolved under Conductor `T-4942`. Successor source
`3b66e5cae94ab709f2ba2169e181eb0d3ac1c9e2` generated and replayed the exact
terminal range. The admitted segment SHA-256 is
`1b576b189b7354bb9c025f17a14409cb1a4118c8c84c7bfbfb7229f5505fee32`;
the terminal structural-audit receipt SHA-256 is
`a165b95e5c26c9cad590a427fbc8b556e096f7dca7eea78e7cd24d75ba86d5f1`.
The complete source-head serialized release-library suite passed with 4,081
tests passed, zero failed, and 114 ignored. The rebased current-`main` code head
also passed all six focused regressions and the complete serialized suite with
4,479 passed, zero failed, and 119 ignored. Global uniqueness and graph
completion continue as separate campaign gates.
