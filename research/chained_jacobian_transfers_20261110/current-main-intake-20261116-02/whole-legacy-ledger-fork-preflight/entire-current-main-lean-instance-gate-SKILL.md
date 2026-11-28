---
name: lean-instance-gate
description: Gate for every Lean statement, definition or proof written in this repository, its siblings or crypto-lean. Run it before the first Lean edit of a study, before writing a theorem, structure or certificate family, whenever a proof grows long, times out, needs `maxHeartbeats` or unfolds a definition, and on resume after a context reset. It restates the task as an instance of a structure that Mathlib or crypto-lean already has, names the existing theorem used as stated, type-checks an assembly probe, lists only the missing components (each built generically in its owner first), and writes the inheritance table. Complements AGENTS.md "Mathematical results should include Lean proofs" and `tools/lean/proof-gate.sh`, which check that a proof is sound; this skill checks that it is not a re-derivation.
---

# Lean instance gate: instantiate, never re-prove

Adapted from the Trail of Bits constructive-cryptography skills
(`trailofbits/constructive-cryptography`, `skills/instantiate`,
`skills/state-over-structures`, `skills/prove-from-laws`; MIT License,
Copyright (c) 2026 Trail of Bits). The rules below are restated for
elliptic-curve and ECDLP work over Mathlib and `aburan28/crypto-lean`.

A Lean task here is never "prove X for this curve". It is: "X is an instance
of a structure S that Mathlib or crypto-lean already has; the theorem T about
S is used as stated; the only new mathematics is showing that X is an
instance." A result in group theory is not re-proved for one curve.

Why this exists (measured 2026-11-15, `docs/session-logs/2026-11-15.md`):
696 research `.lean` files sit in 87 separate Lake projects; 20 reference
crypto-lean; 88 set `maxHeartbeats`. The proof gate accepts all of that,
because each file is sound. It is the re-derivation, not the soundness, that
this gate catches.

## The floors

Each statement lives on one floor and is stated over the structure of the
floor above:

1. **Mathlib**: fields, `ZMod`, polynomials, `WeierstrassCurve`, its
   `Affine.Point` group, `orderOf`, finite-group counting.
2. **crypto-lean** (the shared library, revision pinned with the
   toolchain): curve families, isogenies, endomorphism rings, summation
   polynomials, cost models, certificate checkers.
3. **The study** (`research/<topic>/lean/`): instance obligations for the
   study's curves and parameters, and the endpoints that read off theorems.

A study's module states laws only when no floor below can own them; a law
that a second study could use belongs in crypto-lean first (an "upstream
candidate").

## The instance statement (before the first Lean edit)

Write these six fields in the study's proof map (the one AGENTS.md
requires), or in the task description.

1. **Structure above.** S with its module and declaration: a class, a
   structure, a predicate. List every operation and law the task touches.
2. **Theorem used as stated.** The declaration T over S whose conclusion is
   the goal. If none exists, the missing law is the task: state it over S,
   in its owner (Mathlib-style generality in crypto-lean), and build it
   first.
3. **The object as an instance.** X is a value of S, a predicate on S's
   objects, or a morphism into S. A syntax type with an erasure map is
   never the primary object.
4. **Assembly probe.** The target statement with the components as variables
   and their contracts as hypotheses, type-checked (`lake env lean
   probe.lean`) and closed by the floor's laws only. Then the same probe at
   the chosen objects: fields, characteristic, subgroup, cofactor and
   endomorphism-order hypotheses must fit, not just the names.
5. **Missing components only.** Each a component (a definition, an equality
   of two existing objects, a law of the owning floor). None is a
   re-derivation of a bound or law that T already contains.
6. **Inheritance table.** For each operation of S that the report uses, the
   line by which X inherits it: `rfl`, an existing lemma, or an instance. A
   missing line is unfinished work, whatever has been proved.

If a field cannot be filled, rewrite the task (field 2 usually names the
real one) or record why no instance exists. Do not start building to find
out.

## Before writing anything (every declaration)

1. Where is it in the whole: which floor, and what above it is stated against
   its name or hypotheses?
2. What changes, at which floor? One component; its interface stays.
3. What is the sketch? A definition: the composition of existing objects. A
   statement: which line of the proof chain it is. A proof: the chain.
4. What is missing below? List every law the sketch cites that does not
   exist, each with its first consumer. Build each just before that
   consumer, never as an up-front phase and never discovered mid-proof.
5. What does the source say? The paper settles hypotheses and bounds; use
   them as stated. Deviate only where the source is wrong, and record the
   erratum at the declaration.

## The proof chain (the sketch comes first)

Before any Lean for a proof that crosses a floor, write it as a chain of
equalities or inequalities in the proof map, as on paper. Each line either
**restructures** (rewrites by named laws so one component stands alone) or
**replaces one component** (by a law, a hypothesis or a cited theorem). One
line never does both. The Lean proof is this chain: one `calc` step per
line, each closed by citing the laws its line names. If a line does not
close that way, a law is missing (build it at its floor, then return) or the
restructuring is wrong (fix the sketch).

## Statements

Each new statement is one of three kinds, recorded in a table (statement,
kind, owner module, structure quantified over, hypotheses):

- **Law**: generic, over S, in S's owning floor.
- **Instance obligation**: X meets one of S's contracts.
- **Endpoint**: T applied to the instance obligations.

Forbidden: a law of S restated for X (a study's `card_points_eq` that is
Hasse plus a computation), or a bound stated for one curve when the
argument is generic. A local `have` inside a proof is fine; a top-level
copy is not.

**Certificate families.** A finite computation that really is per-instance
(a factorization, a point count, a residue table) is an instance obligation
discharged by a checker. Write the checker once, generically, with its
soundness theorem (`check c = true → P c`) in crypto-lean or the study's
owner module; each instance is then `checker_sound _ (by decide)` or
`rfl`. Five hundred generated theorems with copied proof scripts are a
missing checker.

## Proof rules

1. Use theorems as stated. Do not copy a proof, re-run an argument or
   re-derive a bound for an instance.
2. Do not unfold composites by default. A definition's facts come from its
   components' lemmas.
3. Study proofs are short: an instance obligation is `rfl`, a lemma or a
   few rewrites; an endpoint is one application. Over about 15 lines in a
   study module means a missing law: look for it in its owner first.
4. Prefer `rw` with the named law over `simp` on structural goals; under
   binders use `congrArg`/`funext` ending in the law.
5. Do not raise limits. A new `set_option maxHeartbeats` or `maxRecDepth`
   has three fixes instead: supply the missing law, make an implicit
   argument explicit, or stop unfolding. Where a limit is kept, record the
   argument for it in the proof map, as for opening a composite below.
6. `native_decide` stays under AGENTS.md: allowed only when listed in
   `tools/lean/trusted-native.txt` with its reason. Prefer a checker with a
   kernel-checked soundness theorem.
7. One `lake` process at a time. Build focused targets while working; run
   `tools/lean/proof-gate.sh <target>` and `#print axioms` on every reported
   theorem once, at the end.

## Opening a composite

Unfolding a definition into its internals is allowed only with a short
argument in the proof map: the goal and its consumer; each existing law that
could give it and why it fails; the attempt to state the missing generic law
and why it does not hold; and the one named lemma that does the opening
(no other proof opens the same definition).

## Stop signs

| Sign | Move |
| --- | --- |
| A new `inductive` or syntax is the primary object | a predicate or subtype over the existing semantic objects |
| A bound or group fact is proved for one curve | find T over S; the bound is T's, the curve an instance |
| A conclusion mentions one instance's internals (a coordinate, a table entry) | the conclusion is T applied to instance equalities |
| Two presentations of one object kept in agreement by hand | one object; the other is one lemma about it |
| The same rewriting sequence in two proofs | a law in its owner |
| Hundreds of theorems differing only in constants | a generic checker with a soundness theorem |
| A new `maxHeartbeats` or `maxRecDepth` | the missing law, an explicit argument, or no unfolding |
| A study-local copy of something in crypto-lean or Mathlib | import it; delete the copy |
| A theorem's statement changed to make the instance fit | generalize where the theorem lives, same proof, and say so |
| A law proved in a study module that a second study needs | move it to crypto-lean, stated over S |

## Acceptance

The endpoint's proof is T applied to the instance equalities. If it re-runs
a bound, the gate was skipped. If a lemma restates a law of S for X, an
operation was redefined instead of inherited. If the inheritance table has
a missing line, the step is not finished. The proof-gate receipt and
`#print axioms` output are still required (AGENTS.md); this gate adds to
them and replaces neither.

Record in the PR: the six fields, the statement table, the inheritance
table, the upstream candidates for crypto-lean, and any limit or composite
opening with its argument.

## Continuous loop: learn, adapt, act on your own

AGENTS.md "Continuous learning loop"; ledger `tools/lessons.py`.

1. **Before acting**: `python3 tools/lessons.py brief --scope lean-instance-gate`. Apply the
   adopted lessons; treat candidates as hypotheses this run can test.
2. **While acting**: resolve a blocker yourself within your authority
   (diagnose the cause, try the documented remedy, then a materially different
   approach) before escalating; AGENTS.md "Persistent scientific research loop".
   Persistence grants no extra execution or spending authority.
3. **After acting**: record the outcome with evidence,
   `python3 tools/lessons.py outcome --scope lean-instance-gate --task "..." --result success|partial|fail|blocked --evidence <receipt|commit|job> [--applied L-...]`,
   and `lessons.py test --lesson L-... --outcome confirm|refute --evidence ...`
   for each lesson you applied. A surprise that would change what the next run
   does becomes `lessons.py add --scope lean-instance-gate --trigger "..." --action "..." --evidence ...`.
4. **Adapt**: when a lesson scoped to `lean-instance-gate` is adopted (two distinct
   confirmations), propose the edit to this file in a PR that cites the lesson
   id; when one is refuted twice more than confirmed, it retires itself.
