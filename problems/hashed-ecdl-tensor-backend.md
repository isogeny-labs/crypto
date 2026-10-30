# Independent audit of the conditional hashed ECDL tensor algorithm

- **Status:** implementation gap addressed; independent general audit open
- **Kind:** correctness/complexity validation, conditional external theorem
- **Affected work:** [native implementation](../docs/hashed-ecdl.md)
- **Next action owner:** unassigned independent reviewer
- **Last updated:** 2026-10-29

## Problem

The initial bounded reference omitted the compact encoder and contraction
backend. The tensor follow-up implements both, including general integer
inputs, fixed-cofactor projection and the relation pipeline. That closes the
code-availability gap. It does not independently prove the external theorem,
the general implementation's correctness, or its asymptotic complexity.

The literal source constants force exact finite-size fallback on practical
graphs. Small forced-recursion controls exercise the general transformation
but cannot establish a runtime bound, a crossover or a deployed-curve result.
Explicit work policies can terminate a run with an unknown result.

## Evidence and uncertainty

- **Observed:** Bochum v0.2 revision
  `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f` contains only a PDF.
  Its Theorem 2 relies on external Theorem D.1. The exact external manuscript
  was obtained at OpenAI math revision
  `7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`; Appendix D.1/D.4 was read.
- **Implemented:** compact Fourier arithmetic and point/hash encodings;
  ordinary BTN graphs, exact counts and prefix witnesses; coefficient ring,
  packet packing, Cauchy deformation, modes, implicit child tensors,
  creation-precision lifting, tapered Hermite reconstruction and recursion;
  inverse-hash bases, tensor relation collection, elimination and replay.
- **Checked locally:** bounded independent arithmetic/graph controls and a
  tiny full pipeline. Actual final commands, source hashes and test results
  are in the [validation receipt](../docs/hashed-ecdl-tensor-validation.md).
- **Not established:** independent proof of the external result or of all
  implementation invariants for arbitrary instances. No new theorem is
  claimed or formalized. The absence of a general audit is a validation gap,
  not mathematical impossibility or evidence that the theorem is false.
- No timing, sub-rho result, security conclusion or practical attack follows
  from these development tests.

## Obligations

- [x] Obtain and pin the exact external algorithm and its parameter guards.
- [x] Implement the compact arithmetic/decomposition networks and bounded
  assignment controls, including exceptional point operations and size bounds.
- [x] Implement exact contraction and compare forced small recursive counts
  with independent direct contraction, including creation-precision retention.
- [x] Integrate the tensor reduction, native runner and independent algebraic
  replay in both repositories.
- [ ] Independently audit packet packing, mode orientation, precision loss,
  implicit-tensor evaluation, degree/shrink invariants, exact reconstruction
  and resource-error propagation against the complete primary proof.
- [ ] Audit the construction cost and polynomial-space/time accounting of
  the implementation against the source hypotheses. Any corrected theorem
  or new mathematical result requires the repository's proof/Lean rules.

## Next discriminating step

An independent reviewer should reproduce the direct-versus-recursive controls
and audit the general invariants above from the pinned manuscript and source.
Preserve the first disagreement with exact arithmetic as a regression fixture.
Only after that review should the status of the general conditional bound be
reconsidered. A practical timing study is a separate requested experiment with
the existing compute, checkpoint, external-reference and measurement gates.

## Resolution and history

2026-10-30 engineering follow-up: [native exact-primality and witness changes](../research/hashed_ecdl_speed_20261029/README.md)
pass bounded correctness controls and preserve complete seeded certificates.
Checked Pocklington input now validates ECCp-131 parameters within the original
two-million-step policy. The functional leaf can avoid repeated prefix counts;
count queries, full graph replay and source recursion guards are preserved.
Profiling and the frozen baseline/candidate panel remain blocked by checkpoint/
execution access, so no general performance conclusion follows. This does not
close the independent audit or either published challenge solve requirement.

2026-10-29, first reference: bounded generic reduction landed; missing compact
encoder and backend were correctly recorded as an implementation gap.

2026-10-29, tensor follow-up: code gap addressed with a native mirrored crate
and bounded correctness controls. This record remains open specifically for
independent general correctness and complexity audit, without reclassifying
unit tests or code delivery as a theorem proof or performance result.

2026-10-29 prime-field companion extension: the user's prime counterpart is
officially Certicom ECCp-131. The paired
[implementation and validation receipt](https://github.com/isogeny-labs/cryptanalysis/blob/a3e8d29f05625a38a6c5f6c1361527b782dcb43c/experiments/hashed-ecdl-tensor-reference/prime-development/README.md)
now composes canonical prime-field arithmetic, modular Fourier multiplication,
point validity/addition, fixed-cofactor image and affine hash constraints with
the scalar BTN engine. Its 39 combined development tests pass, including 16
OpenSSL multiples of published P/Q. The actual-target graph's 1,564,506 edges
compile consistently; default u_0=0 exact summation exceeds the configured
2,000,000-operation budget and produces no witness. The native crate was independently added on main; this companion does not
replace it. Companion general ell, native parity, full soundness/complexity
audit and both challenge solves remain open. Preserve the native implementation
checkboxes above; these checks assert no speedup.

