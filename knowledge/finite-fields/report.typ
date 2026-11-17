#set page(paper: "a4", margin: 18mm)
#set text(font: "Libertinus Serif", size: 10.5pt)
#set par(justify: true)
#set heading(numbering: none)
= Handbook of Finite Fields
== Ingestion, checked references, and remaining coverage
2026-10-31 · source HFF2013 · Mullen–Panario (eds.), 2013

The supplied textbook now has a source-addressed inventory and a working reference tool. The current Lean library has *58 audited theorems*, mapped to *35 numbered source items*. The request to validate and formalize the entire book remains partial.

#image("pipeline.svg", width: 100%)

== What is recorded
The inventory contains 5,850 numbered items: 1,181 theorems, 169 propositions, 119 corollaries, 86 lemmas, 986 definitions, 2,808 remarks, 279 examples, 78 algorithms, 50 constructions, 49 conjectures, and 45 problems. These are extraction labels, not a count of proved mathematical claims. Items can contain multiple subclaims.

Every item has a stable ID, line and byte coordinates, and a span hash. The source has 2,907,339 bytes and 67,837 logical lines. Its digest is recorded in #raw("source.json"). The original and full catalogue are retained locally; Git holds the formalizations and reproduction metadata. Catalogue publication to the required artifact store is blocked by access denial, recorded in #raw("PUBLICATION.json").

== What has been checked
The proof set includes finite-field characteristic and cardinality, Wedderburn's theorem, Fermat and inverse powers, the field polynomial and its factorization, irreducibility divisibility, interpolation, extension towers, trace and norm, Frobenius, Galois groups, normal-basis existence, and linearized-polynomial linearity.

The editors' errata contain substantive changes to hypotheses. The corrected splitting statement in Remark 2.1.118 is formalized with irreducibility explicit. The catalogue also records the added restrictions in Theorems 3.1.12 and 3.1.32. A checked counterexample in characteristic four documents why the Frobenius sum rule needs prime characteristic.

#pagebreak()
== Verification and reusable citations
Lean 4.35.0 and Mathlib commit #raw("cfc55fc58714a59dc06e6e79edc6d1d95140d3c2") match the existing shared CryptoLean pin. All structures are actual Mathlib objects. The audit checked all 58 named theorems, with zero failing and zero compiler-trusted proofs. The only accepted transitive foundations are #raw("propext"), #raw("Classical.choice"), and #raw("Quot.sound").

Run #raw("knowledge/finite-fields/verify.sh") to build, audit axioms, print every type, check consumer imports, export dependencies, and record bound file hashes. Supplying the original text as an argument also checks exact source regeneration. The installed toolchain has no #raw("lean4checker"); its secondary check remains unavailable.

The reference tool supports lexical search, inspection, citation generation, and verification. For example:

#raw("knowledge/finite-fields/kb search 'trace'\nknowledge/finite-fields/kb show FF.fermat\nknowledge/finite-fields/kb cite FF.irreducible_splits_degree_extension", block: true)

A citation returns the statement, book number, declaration, assumptions, source coordinates, and scope. Research proof code imports #raw("FiniteFieldHandbook") so Lean checks applicability at the call site. The dependency export records direct constants from actual elaborated proofs. Changed rules, proofs, pins, verifier inputs, or receipts invalidate verified lookup.

== Remaining obligations
Whole-book semantic extraction is open, including unnumbered claims, chapter-one prose, tables, exercises, and individual subclaims. The 49 conjectures have unchecked present-day status. Many formal mappings cover only part of a book item. Independent review of the translation from source text to exact formal statement is pending; local receipts are unsigned records.

The root Rust library suite was attempted and failed on an existing unclosed delimiter in #raw("src/cryptanalysis/binary_semaev_chain_sat.rs"), reported at line 1648. The file is unchanged by this intake. Standalone catalogue tests and the Lean library have their own successful checks. Failed translations and the root-test failure are retained in #raw("receipts/").

The next batch is the remaining section 2.1: irreducible counts, subfield uniqueness, primitive-element criteria and counts, and remaining trace/norm subclaims. Continue chapter by chapter with the same provenance and proof checks. General reusable lemmas can be promoted to CryptoLean in a reviewed change while retaining stable textbook IDs here.

== Sources and evidence
Primary sources checked: the supplied textbook spans; the pinned Mathlib implementations; repository Lean auditor and CryptoLean design; the editors' book page and errata; Lean's proposition reference.

#link("https://people.math.carleton.ca/~daniel/hff/")[Editors' book page] · #link("https://people.math.carleton.ca/~daniel/hff/errata.html")[Official errata] · #link("https://leanprover-community.github.io/mathlib4_docs/Mathlib/FieldTheory/Finite/Basic.html")[Mathlib finite fields]

Canonical detailed report: #raw("knowledge/finite-fields/REPORT.md"). Evidence: #raw("inventory.json"), #raw("rules.json"), #raw("receipts/audit.log"), #raw("receipts/axioms.log"), #raw("receipts/dependencies.json"), #raw("receipts/verification.json"). Open coverage obligation: #raw("problems/finite-field-handbook-formalization.md").
