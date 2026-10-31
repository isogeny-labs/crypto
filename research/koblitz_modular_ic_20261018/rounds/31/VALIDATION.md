# Validation boundaries

Seven native attempts pass their exit/terminal-marker/error-line gates:
color83-01, color284-01, cm31-01, reverse83-01, reverse284-01,
frob83-01 and frob284-01. Every frozen case completes within the original
per-invocation4GiB/1200seconds caps; runtime is not an efficiency measurement.

Lean compile succeeds under4.34.1/mathlibd13f23b, -j1 -M4096. The separate
transitive axiom audit must be read from lean31-01/audit.log and results.json;
no native_decide or custom axiom is allowed. Formal scope is PROOF_MAP.md.

Initial rustfmt differences are retained in publication-attempts. Formatting
was corrected and the final check passes. That change is formatting only;
the exact original compiled native source remains in cm31-01/source.
The metadata assembler checks process outcomes and index consistency, not
mathematical validity. Raw source/input hashes and S3 verification establish
artifact integrity, not an independent mathematical replay.

The root Rust library was not changed. Its previously recorded1200-second
cap/inherited test failure and the old standalone study crate's unrelated
binary_ecc imports are not repaired or newly certified here. These focused
native binaries and CAS programs are separate from that root-suite status.
Current-head CI is a further delivery check, not inferred from local evidence.

The generic experiment_protocol.py is unavailable; prospective Git-frozen
Markdown is normative. The JSON protocol index explicitly records post-data
creation and the missing serialization of CM31 base permutations. No generic
protocol lint, independent science review or official ledger promotion.
