# Native all-input model adapter — continuation protocol

This continues the existing in-progress Curve25519 task and its explicit NEXT
step in docs/ACTIVE.md/PR5071. The shared tracking/journal conflicts have one
pending Copilot request; do not edit those conflicted files. Independent owned
paths are tools/torus-spectrum-solver and this versioned study. Keep the v1
compiler and every76-input source binding byte-identical. No new claim of a
novel solver or changed exponent is made.

Public solve inputs: Spec, R, Q=[8]R, backend identity and declared limits only.
No coefficient tuple, known index/sign, scalar, witness or point list enters
the query. Reuse the frozen compiler's complete SMT equations. Request every
semantic input with get-value only after a SAT status; never ask for a model
after UNSAT/unknown. Strictly parse the pinned compact field-value dialect,
exact binding set, requested modulus, canonical range and unique wire names.
Re-evaluate/check all gates and constraints. Derive coefficients/labels from
that checked assignment and run the unchanged numeric predicate and original
indexed/group extraction. Invalid models are retained and rejected.

Backend child bounds: native cvc5 GPL/CoCoA1.4.2 with its prior exact executable
SHA754bc9abb49bcd2ae095bf07a9ba2f68759bf0f232e71f601ac642f802a59fc8;
Linux address-space cap2GiB; at most5000ms backend limit plus1000ms watchdog
grace for a fixed integration control; stdout <=1MiB, stderr <=64KiB. Capture
status/output/refusal separately. These clocks enforce resource limits and
are not benchmark evidence. UNSAT without a checked certificate is only a
NoRootObservation; caps, unsupported cases and parser/model failures are not
absence or route-disproof certificates.

Development correctness controls: strict parser adversaries; two-input
quadratic and inconsistent small-field systems; the12 old supplied models
encoded as **synthetic model text**, followed by full gate/original replay.
Synthetic responses establish parser/replay correctness, not root discovery.
One fixed known-consistent p101/q3/k8 public-target integration may submit the
complete system without supplied unknowns, cap5000ms. A checked root or bounded
refusal is recorded as observed; a cap is not hardness evidence. No cohort,
parameter sweep, timing comparison, paid job or full actual-base search.
These are local development/unit-integration checks under AGENTS.md §9a.
Any comparative solver campaign requires a separate frozen full-cost EKS
protocol and existing authorization/budget. No such campaign is scheduled.

Final qualification: source commit before the single final bounded native
control call, immutable executable and pre-call qualifier with compiled-in
new-source hashes, distinct write-once BUILD/PRECALL/POSTCALL, shell transport
success/collision tests, full raw transcripts and source snapshot roundtrip.
At most2GiB/600seconds, zero paid compute, no automatic retry of the curve query.
Relevant release tests, rustfmt and strict Clippy; preserve initial failures.

Prior-art review: frozen compiler source/ir.rs, model-solver NEXT contract,
unchanged torus-quotient-query support/extract and original point replay;
Ozdemir et al., CAV2024 §§4–5,
https://theory.stanford.edu/~barrett/pubs/OPB%2B24.pdf;
https://cvc5.github.io/docs/cvc5-1.4.2/theories/finite_field.html and options
tlimit/tlimit-per/get-value interface. Split GB can refine to a full basis;
its verification gains supply no decomposition-cost bound. Observe native
model spelling rather than assuming the documented input spelling.

KB unavailable (no loaded connector, wrapper127). Intended retrieval:
“finite-field full model replay original elliptic relation”, “Split GB SAT
model extraction bitsum”, “resource-limited UNSAT absence certificate”,
“public coefficient compiler solution projection”. Repository and primary
sources supply this bounded review; no exhaustive novelty claim.
[pcc:unverified]
