# Round-one evidence and formal boundary

The requested research goal remains open and active. These are development
correctness checks and a concrete rejected shortcut, not an admitted
performance experiment or an efficient arbitrary-point reduction.

## Native controls

- `build-01.log`: compiler failure, missing Boost headers.
- `boost-recovery.log`: package download unavailable. The installed GMP
  backend was used instead; the mathematical protocol was unchanged.
- `build-02.log`: `-Werror` rejects misleading loop indentation.
- `build-03.log`: successful GMP/OpenSSL build. `native-01.jsonl` and
  `native-01.stderr` retain stdout/stderr from the successful native call.
- `precision-build-01.log`, `target-precision-01.jsonl` and its stderr retain
  the separate target-coordinate control, added under AMENDMENT-01.md.
- `native-pins.log` pins the source and executable bytes. This is a post-call
  pin record, not a reconstructed pre-call execution certificate.

The model constructor checks three bounded fixtures and six negative controls.
The fourth case checks the public secp256k1 model projection against OpenSSL,
without expanding 2^r. The fixed rational-lift scan finds p=59 after 13 good
primes in its frozen range; it constructs an exact kernel witness with
nonzero component label. The separate target control checks all nonzero
auditor labels in nine fixed order/precision cases. These are producer
observations, pinned by unsigned statement claims. [pcc:unverified]

The source constructor receives coefficients, p and r, not a hidden scalar.
There is no implemented arbitrary-Q evaluator. Scalar enumeration is used
only to select the small adversarial fixture; target parameters 2^k belong
only to a target-side auditor. Neither is presented as a transfer algorithm.

## Lean checks

`assembly-probe.txt` closes a variable-group negation endpoint using only
`map_neg`; `assembly-01.log` is the successful empty output. The initial
shell redirection used the study root rather than its lean directory and
failed before running Lean; it is not counted as a failed proof.

`lean-build-01.log` through `lean-build-03.log` retain development failures.
The corrections supply literal modular branch conditions and use
`div_eq_iff` with explicit nonzero denominators. `lean-build-04.log` passes.
No resource limit, trust manifest, theorem hypothesis or expected coordinate
was weakened. [pcc:unverified]

The unchanged repository gate was run after staging the tracked source:

```sh
PATH=/home/agent/.elan/bin:$PATH USE_FRO_CACHE=1 \
  tools/lean/proof-gate.sh --out <versioned-receipt-directory> \
  research/prime_field_transfer_goal_20261116/lean
```

The first `gate/` receipt audits 15 theorems. After adding the exact nodal
identity, `gate-final/` audits 16, zero failing, zero trusted native exceptions.
Both raw summaries are retained. The expanded explicit `AxiomAudit.lean`
output is in `axiom-audit.log`; only `propext`, `Classical.choice`,
`Quot.sound` occur. One Lake process ran at a time. [pcc:unverified]

The exact endpoint is `ComponentLabelGate.no_compatible_labels`: no additive
homomorphism from the literal source point group can assign 1 to P and 3 to Q.
The source's Q=−P and order-five identities are separately checked. The
geometric interpretation of the proposed labels uses Cremona's classical
component formula and native rational arithmetic; no fully formal Neron
model is claimed. The generic model-construction paragraph remains a written
argument plus the stated finite native instances.

`IDC1hcad63e1a0fbc75b5` is the denominator-cleared nodal polynomial identity.
The C++ issuer builds successfully, accepts 32 derived points and rejects
its one perturbed negative control. Its statement ID and full-file Lean
proof hash are metadata checks. Independent Rust replay is unavailable.
The exact identity is separately checked by Lean. [pcc:unverified]

## Scope, provenance and reporting

PROTOCOL.md preceded the native calls, and AMENDMENT-01.md preceded the
separate precision control in this turn. Neither is represented as an
independently timestamped experiment preregistration. This round has no
sealed external qualifier, L2 isolation, benchmark run, execution proof or
wall-clock result. Future prime-field experiments must create their distinct
build/pre-call/post-call manifests and immutable qualifier before calls.

The first PDF preview had an orphaned paragraph and one overlong diagram
label. It was regenerated on A4 with the corrected label. The final PDF and
all its pages are inspected before publication. The SVG sources and layout
script reproduce the diagrams from recorded data; no cryptographic arithmetic
is performed by the renderer. Previous studies and their figures are unchanged.

Conductor is absent. Root Cargo is absent; no root-suite pass is claimed.
No paid job or cloud build was launched. The new GitHub workflow runs only
native development correctness comparisons against the retained output.
All scientific claims remain unsigned under the repository's independent
PCC process. [pcc:unverified]

PR 4308 merged at 2026-11-16T00:09:29Z. Its old monitor was disabled. The
autonomy outcome is partial: the local formal endpoint and written geometry
were delivered, while the practical general map was not. Post-merge checks
on its final head include gate and artifact failures; a green-at-merge state
is not asserted. Original prompt/intervention timestamps are unavailable in
the carried context; intervention classification is left pending rather
than inventing timestamps or an unsupervised-success score.

## Next action

Specify and test a source-branch-consistent torsion component extractor over
the actual algebra, retaining source/target places and orientation across
every input. Charge faithful target encoding and all setup; do not introduce
a source-log lookup to repair the rejected labels. The active goal's novelty,
large-order applicability and demonstrated total solving advantage are unmet.
