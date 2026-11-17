#set page(paper: "a4", margin: 20mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)

= Nonbranch Lean import preflight
_Source and receipt reconciliation · 2026-11-11_

This report is a source-only preflight. It records the tracked import closure and existing receipts. It adds no compiler invocation, theorem verdict, experiment attempt, or scientific promotion.

== Pinned source and closure

The source tree is pinned to crypto commit `4f8d8bf375348180cf1518ecf9d913a2ddca7ad2`. The five archived pencil roots and their dependencies are unchanged at gate commit `cce79de90322deb6e598c28a63b0f3cb75e2fb02`.

The static basename resolver finds *41 tracked Lean sources* and *53 internal import edges* from the five roots. Three edges cross into the projective-law formal directory, which contributes 36 sources. It finds no missing or ambiguous tracked import. This is a source-name map, not Lean elaboration.

The 36 projective-law sources have nearest toolchain pin Lean 4.34.1. The archived pencil roots have no nearest pin; retained native commands explicitly used Lean 4.34.1. The projective-law Lake manifest has SHA-256 `ebc6643e694eaa4a60ab2b727c96253f3d88694e7ecb404f2176a548eedbe98d` and Mathlib revision `d13f23b723b8a846827a245b89c10fc7d3f11612`.

== Recorded attempt states

#table(columns: (34%, 66%), inset: 6pt,
  [Record], [Observed status],
  [Original native compiler], [`Nonbranch` exit 0; `IndependentControls` exit 1. Compiler receipt says failed, machine review incomplete, scientific execution false.],
  [Dependency custody], [Separate pre and post receipts each checked 16,736 files and 3,111,892,318 bytes.],
  [Later correction], [Candidate SHA-256 `448d5ab601e955b93a9071d6b5f932b38850f5728c9cf2c3fc3eb5b3742bb35d`; proposed_uncompiled, zero invocations; original ceiling 3 of 3; no new attempt authorized.],
  [Hosted proof gate], [Run 42773450127 reported FAIL_build for five standalone roots without cross-directory imports. This identifies a gate workspace gap.],
)

== Reproduction and next proof boundary

The accompanying `imports.json` records root paths, source SHA-256 values, toolchain pins, and internal edges. Recompute a listed hash from the pinned tree with `git show <commit>:<path> | sha256sum`. The SVG renders the directory-level closure and recorded states.

A prospective proof attempt requires a separately authorized envelope, an immutable source and dependency workspace at the recorded pins, independent source-fidelity review, explicit named-module compilation, transitive axiom readback, and negative controls. The correction remains unverified until those checks exist.

#text(size: 8pt, fill: gray)[Source: repository receipts and pinned tracked sources. No historical source, receipt, deadline, or retry count changed.]
