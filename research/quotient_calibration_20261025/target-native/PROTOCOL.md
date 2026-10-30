# Native target calibration: execution protocol 1

This continues the exact F343/order-373/kernel-153 experiment in
[the frozen parent protocol](../PROTOCOL.md). It executes the chart/origin,
target frame and 32-point calibration challenge gates. It does not replace
the parent protocol or claim completion of inversion or Profile B.

## Fixed inputs

The original source manifest has SHA-256
`cfca312b80fe7a35da6cd8d31ee44b195885aad5fbd6b1de07b1fc9dfe33dc65`.
The training labels remain 1,2,3,4,5,6,7,8,11. All 32 coordinate challenges
and their P+G partners are copied unchanged into `fixtures/coordinate-inputs.json`.
Their labels and target images have not been computed by this continuation
before protocol freeze. Neither failures nor exceptional training points may
be replaced.

`fixtures/public-target.json` copies only arithmetic data from the pinned
historical reports: the original and symmetric quartics, characteristics
9/19/26 and the eight coefficients on 120 symmetric products. It excludes
the old source/image table and the old generator's scalar label. Historical
F49 triples and their expected values live in a separate positive-control
file, read only by calibration.

All arithmetic uses the unchanged explicit F7^18 modulus. Field elements
retain all 18 coefficients. The residue-algebra trace used to evaluate a
symmetric tensor is L-linear on a length-three algebra; it is not a field
trace used to change or truncate a coordinate's field of definition.

## Source review and method

- [Tian, section 3.2](https://arxiv.org/html/2012.07173v1#S3.SS2): the map from
  curve triples uses origin delta+P_infinity; the eight diagonal-vanishing
  sections differ from normalized Weil coordinates by a fixed linear change.
- [Khuri-Makdisi, Lemmas 2.1--2.3](https://arxiv.org/abs/math/0105182): effective
  divisors are represented by spaces of sections. Multiplication is used
  only with both line-bundle degrees at least 2g+1=7; division tests use
  base-point-free spaces with degree at least 2g=6.
- The four hash-pinned historical reports and their native relocation
  receipts remain the input evidence. Historical Sage source was read to
  preserve characteristic labels, all-six-permutation multiplicities and
  chart conventions; it is not executed.
- Atlas lead `LEAD-TRF-odd-char-descent-smith-quartic` is the neighboring
  program lead. This is a continuation of the existing explicit quotient
  calibration case, not a claim of implementing the atlas's whole pipeline.

Let N=D9+D19+D26, A=(0:1:0), and I=(1:0:0). The fixed infinity divisor
is H=2A+2I, so B=N-2A-I. The bitangent square identities establish
2N~3H and therefore 2B~H+2I. The certified image remains
D*=[Q+2A-3O], Q=(4:6:3), O=(1:5:5).

An effective degree-three divisor D is stored as H0(3H-D), a codimension-three
subspace of the degree-three coordinate ring. To add two classes relative to
B, reduce D1+D2+(2A+I)-N. All Riemann--Roch spaces are computed by exact
multiplication/division of section spaces, without enumerating Jacobian classes.
The output dimension and every basis-product containment are checked.

The target evaluation uses the length-three quotient algebra. Multiplication
by X,Y,Z maps the degree-two quotient to degree three. If Z is invertible,
the resulting 3x3 matrices give x=X/Z and y=Y/Z. For three functions f,g,h,
their all-six-permutation sum is computed by inclusion-exclusion of traces:

    tr(f)tr(g)tr(h) - tr(fg)tr(h) - tr(fh)tr(g) - tr(gh)tr(f) + 2tr(fgh).

This agrees with evaluating at the three geometric points after scalar
extension and avoids assuming that the support splits in L. The original
three F49 fixtures compare this path with direct six-permutation evaluation
and the independent historical expected vectors.

## Execution order and blindness

1. `calibrate PUBLIC CONTROLS SOURCE_MANIFEST FRESH_DIRECTORY` checks the
   chart identities, smoothness, contact squares, original controls, anchor
   plus origin and residual negation. It computes only the declared target
   frame using the certified anchor. The 63x64 fitting system must have rank
   63 and yield an invertible matrix. Every eight-vector minor is checked on
   both frames. It writes `matrix.json` with a canonical coefficient hash
   and a separate frame receipt, using create-new writes.
2. Checkpoint the frozen matrix before target validation. `evaluate MATRIX
   COORDINATE_INPUTS FRESH_PREDICTIONS` receives only the matrix and the frozen
   coordinate envelope. Its strict parser rejects labels, expected targets
   and unknown fields. It computes q(P), Mq(P), Mq(P+G), and a control that
   swaps source coordinates zero and one. It calls no target divisor code,
   source enumeration or discrete-log routine.
3. `validate PUBLIC MATRIX COORDINATE_INPUTS PREDICTIONS FRESH_REPORT` checks
   the hashes, then uses a separate validation-only reference. Only now may
   it enumerate the 372 nonidentity source points to label reference inputs.
   A 16-wide baby/giant decomposition computes the corresponding target
   classes from the certified anchor, with its setup and operations recorded.
   Enumeration and reference labels are never evaluator inputs.
4. Record all 32 exact comparisons and all 32 successor comparisons, including
   failures. The swapped-coordinate control must fail the complete suite.
   A mismatch stops the dependent inversion phase; no refit is allowed.

The published Z=1 evaluation chart and all-zero tensor boundary are explicit
feasibility failures in this version. It does not infer that an exceptional
Jacobian class is nonexistent. It records the original input and stops;
implementing another chart requires a dated amendment preserving this run.
No translation attempts or inverse systems are executed by this version.

## Controls, accounting and stopping

The execution record remains a draft; no target frame or holdout data has
been taken. As required by the current repository instructions, the pending
run uses Iso Lab on EKS Spot (`isolab-k8s`, `cpu-spot`, region `us-east-2`).
Qualify a digest-pinned image containing the tested standalone target binary
and a supported custom-command submission route before freezing the record.
The existing GitHub ecbench build/fixture is not this experiment. Record the
source, workflow, image and binary identities separately. Local unit tests,
static checks and replay of the existing identity certificate remain development
checks, not a target experiment or performance evidence.

Checkpoint uploads for this branch were explicitly approved on 2026-10-30.
The normal S3 gate must still succeed using configured access before launch;
do not reuse the earlier pasted credentials or bypass a failed gate.

Claims are exact finite computations. A/A statistical noise, p-values and
performance isolation are inapplicable: no comparative timing or scaling
claim is made. Three previously recorded F49 triples are positive controls;
direct point evaluation is separate from quotient-algebra evaluation.
Coordinate permutation and strict input/hash rejection are negative controls.
The parent's wrong-origin, incompatible-target and inverse-fiber controls
remain required for full Profile A and are not discharged by this phase.

Use one native thread, at most 2 GiB address space and the parent's total
3600-second wall limit for the experimental pipeline. Preserve failures,
timeouts and raw stdout/stderr. Runtime and memory are resource diagnostics,
not admitted speed evidence. No retry after a mathematical mismatch. An
implementation repair gets a new version/amendment and retains old outputs.

The only passing finite conclusion is 32/32 primary and 32/32 successor
equalities for the one frozen matrix, with the permutation control rejected.
Nine fitted pairs alone establish no validation. Divisor inversion, signs,
eight addition/scalar checks, Frobenius checks on recovered classes and
Profile B's global section certificate remain separate obligations.
