# Continuation of the in-progress five-coefficient task

Continue the existing docs/ACTIVE.md five-coefficient task, originally tracked
by800a1d651c/PR4984 and still explicitly in progress in publishedbe76dbad.
The unknown-coefficient compiler and extension/projection obligation were not
completed by the supplied-section predicate. This continuation uses the same
Curve25519 lane and existing draft PR5022, not a duplicate scientific PR.
[pcc:unverified]

Author branch: codex/curve25519-coeff-circuit-20261123. The PR5022 maintenance
checker is paused during source authoring. ACTIVE currently conflicts and its
single Copilot request is pending, so that shared file is not edited; its
existing in-progress task remains the tracking authority. Publish this
continuation record before further investigation or source work.

Planned work: a public Spec/R-conditioned arithmetic circuit with five unknown
normalized section coefficients, explicit inverse/nonidentity guards and
original indexed witness replay. The compiler receives no known coefficients,
labels, signs, scalar tags or original witness. A separate control may extend
supplied known coefficients into circuit wires. Root discovery is a separate
algorithm and cannot be inferred from a small semantic input count.

First audit the classical rational two-primary Montgomery structure and
whether it supports a fixed-degree nonidentity guard for the declared fields.
Then compare a guarded acyclic arithmetic representation with the frozen
numeric predicate, preserving every accepted original-row obligation.
Algebraic-closure assignments and Fp root extraction must remain distinct.

Before controls, record sources, derivation/proof map, prospective cases,
failure/refusal categories, resource limits and exact source qualification.
Use Rust; preserve every prior source/report/raw/claim/criterion byte. No new
paid job, benchmark, solver campaign or broad root-library rerun is authorized
by this tracking record. No solving-cost or exponent claim is established.
[pcc:unverified]
