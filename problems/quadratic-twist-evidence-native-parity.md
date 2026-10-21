# Quadratic-twist full native replay and formal-proof parity

- **Status:** open
- **Kind:** implementation gap; tool/access limits
- **Affected work:** [quadratic-twist evidence import](../research/quadratic_twist_evidence_20261021/README.md)
- **Next action owner:** explicitly unassigned
- **Last updated:** 2026-10-21

## Problem

The imported historical correctness suite has not been rerun fully in native
code, and its general written theorems have not been checked in Lean. The native
auditor verifies the saved finite certificates only. It is not a full port of
the historical experiment harness.

The historical workload is every nonsingular short Weierstrass model over
primes 5, 7, 11, 13, 17, 19: 942 models, all 4,188,162 ordered S3 triples,
113,342 ordered S4 quadruples over primes 5 and 7, the retained subfield and
trace-zero examples, and 81 toy IC recoveries after pairing reduction.

## Evidence and uncertainty

- **Observed:** the historical source, logs, metadata and results are preserved
  byte-for-byte in the study's hash-bound archive. The native audit verifies
  three saved IC certificates, 81 scalar-pairing identities, 3,219 bilinearity
  identities and the two twist point-count examples. Controls reject source
  corruption and a false relation even after its checksum is changed.
- **Derived or proved:** the general readable arguments remain in the archived
  report, with their original hypotheses. No new formal theorem is claimed.
- **Missing implementation:** native full-census, higher-degree/subfield and
  relation-collection parity; faithful checked Lean proofs and registered
  algebraic-identity certificates.
- **Tool limits:** Conductor and Cargo commands are unavailable (exit 127);
  Lean is absent. `.conductor/WORKFLOW.md` is absent at the import's base revision.
  No Conductor reservation or full Rust-library test pass is claimed.
- **Not established:** these gaps imply neither a mathematical obstruction to
  a native implementation nor general ECDLP hardness. No speedup, benchmark
  qualification or deployed-curve result is established.

## What would unblock it

- [ ] Implement and replay every historical workload natively, preserving
  its exact fields, point sets, controls and acceptance conditions. Bind the
  native inputs/source and compare the complete mathematical outputs.
- [ ] Complete a faithful Lean proof map for the central general statements;
  build the theorem modules and audit their transitive axioms under the proof gate.
- [ ] Run the required repository tests and actual shared Conductor check in an
  environment with those tools and configuration. Do not substitute a fresh local
  coordination instance for the shared task state.

These conditions would support full native/formal parity, not an asymptotic
hardness or speedup conclusion. Those are separate obligations.

## Next discriminating step

- **Action:** port the exhaustive finite-field/census path without adding a
  performance claim, then compare each original per-prime count and relation set.
- **Inputs and controls:** use the archived parameters/source; retain both
  original and native transcripts and introduce deliberate false relations.
- **Limits:** stop each finite workload at its originally specified domain;
  preserve every error or mismatch rather than dropping a field or control.
- **Outcomes:** exact agreement validates that finite workload's parity;
  a mismatch requires investigation; a missing tool is an infrastructure gap.
- **Recheck trigger:** native implementation, Lean dependency/toolchain, or
  repository validation access becomes available.

## Resolution and history

Unresolved. The 2026-10-21 import adds a native finite-certificate audit and
preserves the historical suite. Creating or merging that PR does not resolve
this full-parity problem.
