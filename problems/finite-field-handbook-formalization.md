# Whole-book finite-field statement fidelity and formalization

- **Status:** `investigating`
- **Kind:** semantic coverage gap; proof implementation gap; OCR/source ambiguity; unchecked present-day conjecture status
- **Affected work:** [Handbook knowledge base](../knowledge/finite-fields/README.md), [requirement report](../knowledge/finite-fields/REPORT.md)
- **Next action owner:** current research workstream; no unattended worker is scheduled
- **Last updated:** 2026-10-31

## Problem

The user's request to extract, validate and formalize all mathematical content
of the supplied Handbook of Finite Fields is not yet established because a
complete semantic claim inventory, source-fidelity review and proof
implementation are still missing beyond the checked foundation.

The exact supplied text is identified by SHA-256
`860cd68078087e791edce1e8a7f3a1d9315848cfd99e0c9ca2b673527efe8bf7`.
The requested scope includes laws and rules inside prose, definitions,
algorithms, tables and exercises, as well as numbered results. Extracting
numbered labels alone cannot discharge that scope. Some items are explicitly
conjectures, whose current truth/proof status must be researched rather than
inferred from their appearance in a reference book.

## Evidence and uncertainty

- **Observed:** the Rust intake records 5,850 numbered items, including 49
  conjectures, in `knowledge/finite-fields/inventory.json`. It retains exact
  source coordinates and hashes; its scope note excludes a claim of exhaustive
  semantic extraction.
- **Derived or proved:** 58 actual Lean theorems mapped to 35 source items
  pass the recorded build and transitive axiom audit. Many mappings are
  explicitly partial. See `rules.json`, `receipts/axioms.log` and
  `receipts/audit.log` in the study directory.
- **Observed corrections:** the [editors' errata](https://people.math.carleton.ca/~daniel/hff/errata.html)
  change assumptions and formulas. The corrected first sentence of 2.1.118
  has been formalized; other corrected results still require their own proofs.
  A characteristic-four counterexample clarifies the prime-characteristic
  requirement in the Frobenius sum formula.
- **Suspected explanation:** none asserted for unresolved statements. OCR
  errors, implicit notation and genuine mathematical gaps must be
  distinguished item by item.
- **Missing capability:** complete claim decomposition, formal definitions
  and proofs for the remaining content, current literature checks for the
  conjectures, and independent source-to-statement review. The installed Lean
  toolchain has no `lean4checker` executable.
- **Publication access limit:** the required artifact store returned 403 for
  the tool's access check and AccessDenied for an immutable PutObject. The
  complete derived catalogue remains intact locally, with its hash in
  `PUBLICATION.json`; upload/readback and a real manifest are still owed.
- **What this evidence does not establish:** whole-book truth, exhaustive
  extraction, full coverage of any item merely because one subclaim has a
  proof, or independent semantic validation. No bounded failure is treated
  as a mathematical impossibility.

## What would unblock it

- [ ] Review all source regions, including chapter 1, unnumbered claims,
  tables and exercises; record explicit semantic coverage and unresolved
  OCR ambiguities, preserving the original text version.
- [ ] Decompose each mathematical item into claims with exact quantifiers,
  domains, exceptions and inherited hypotheses. Record conjectures,
  algorithm contracts and historical statements with their appropriate kind.
- [ ] For each established mathematical claim, provide a faithful checked
  Lean declaration and a source-fidelity verdict, or a specifically named
  missing definition/dependency/proof obligation. Definitions need definitions
  and verified consequences; they cannot all be treated as theorem assertions.
- [ ] Resolve corrections and conjecture status through primary sources.
  A claim lacking proof keeps its original unresolved requirement visible.
- [ ] Obtain independent fidelity review and rerun the pinned proof and axiom
  checks. Run a fresh secondary proof checker if it becomes available.
- [ ] Restore authorized artifact-store access, upload the unchanged full
  catalogue, download and hash its entire bytes, and commit the real manifest.

These conditions establish a reviewed formal reference corpus for the
mathematical claims covered. They do not turn historical assertions,
unproved conjectures or heuristic performance discussions into unconditional
theorems. No wholesale truth assumption can replace these obligations.

## Next discriminating step

- **Action or proof obligation:** finish the remaining foundation in section
  2.1, starting with 2.1.24 (irreducible counts), 2.1.35 (subfield uniqueness),
  2.1.42/45 (primitive-power criterion and count), and the uncovered subclaims
  in 2.1.83/84/88. Read each source span and cited reference before translating.
- **Inputs and controls:** the hashed supplied text, correction overlay,
  pinned Mathlib source, existing exact type/axiom export, and consumer
  examples. Use real field and polynomial structures with map compatibility.
- **Execution limits and stop condition:** this is proof/implementation work,
  with no new cloud or experiment spending authorization inferred. Each claim
  stops at a checked theorem plus fidelity review, a checked correction or
  refutation, or an explicit missing proof obligation.
- **Outcomes and next actions:** a checked faithful proof extends the mapping;
  an erroneous statement receives a correction or counterexample with its
  original form retained; an unresolved lemma or source ambiguity gets its
  own concrete obligation; infrastructure errors remain verification gaps.
- **Recheck trigger:** completion of a new claim batch, a source clarification,
  independent review, or a compatible proof-checker installation.

## Resolution and history

Unresolved. On 2026-10-31 the intake, reference tool and first 58 checked
theorems were implemented. The full requested book scope remains open.
The required root Rust test was also attempted and failed on an unchanged
syntax error in `binary_semaev_chain_sat.rs`; the new standalone tool and
Lean library have separate recorded checks. See the study report for the
failure log and the precise distinction between those checks.
