# Production `ZSMul` source overlay

This directory carries the five Mathlib sources needed for the integer
division-polynomial multiplication formula used by the production Lean
project. The source is open Mathlib PR
[`#41197`](https://github.com/leanprover-community/mathlib4/pull/41197), head
commit `f46c499f84960839eac3fdbc517a930f02b3b252`, rebased onto the production
Mathlib commit `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.

The only compatibility adaptation is in
`NumberTheory/EllipticDivisibilitySequence.lean`: the rebased file imports the
production `Mathlib.Data.Nat.DvdSequence` API and uses its canonical
`IsDvdSequence` name instead of reintroducing the older `IsDivSequence`
definition. The division-polynomial `Basic`, `Universal`, `Omega`, and
`ZSMul` sources otherwise retain the rebased PR contents.

Files are stored with a `.lean.overlay` suffix so the repository-wide proof
gate does not treat them as standalone Lean projects. The main
[`lakefile.lean`](../lean/lakefile.lean) copies them to the corresponding
`.lean` paths in the pinned Mathlib checkout after dependency updates and
before Lake actions. They remain an explicitly pinned local integration of an
open upstream PR, not an assertion that Mathlib has merged that PR.
