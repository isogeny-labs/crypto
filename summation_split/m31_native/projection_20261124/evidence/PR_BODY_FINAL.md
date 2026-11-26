## Native M31 F4 with checked geometric projection

This PR implements the projective quotient comparison and its separate guarded multiplicity branch at p=2147483647, with the unchanged curve y²=x³−3x+21 and factor-base map. The latest continuation closes the missing native projection step on the prescribed four-point parameter section.

### Latest component

- Added `verify_and_project`: ordinary Buchberger checking against the original ideal, membership checks for both section equations, then four indicator normal forms. The returned support retains geometric extension witnesses, infinity components and nonreduced auxiliary fibers. It also returns exact elimination generators `t²−t, u²−u, Q`. [pcc:unverified]
- A complete point set is exposed only after every check finishes. Producer/verifier/projection limits and rejected bases remain distinct from completed empty output.
- Added explicit bounded `solve-one` for all four arms and both witness modes. Geometric decompositions use the quotient adapter; explicit Fp² models retain all original equations. It records inputs, basis, limits, exact output and separate work counters. The oracle is consulted after projection.
- First frozen qualification: **40 library + 5 CLI tests passed**, with no failed or ignored tests. Thirteen new analytic projection tests cover all 16 four-point supports, nonreduced/extension/infinity fibers, forged or non-Gröbner bases, parameter errors and resource limits. Three new CLI tests cover options, representation adapters and status serialization. [pcc:unverified]
- The existing unique **m=3, case 0, geometric sequential** fixture returned exactly **{(0,0)}**, matching the separate signed-point oracle. Extraction completed **6 queries, 29 leading-term iterations and 28 polynomial subtractions at degree ≤2**. F4 retained 561/561 pairs, ten matrices and observed raw/post-normal-form degree six. The ordinary checker completed 561 candidate S-pairs, 528 original-ideal pairs and both containments. [pcc:unverified]

The six projection queries happen after basis construction and ordinary verification. Their degree/work counts are postprocessing observations; they do not establish a general solving-degree bound or a runtime/RSS comparison.

### Evidence

Source/evidence publication: **[9ce6aa69](https://github.com/isogeny-labs/crypto/commit/9ce6aa69200ad155e5512d11e04b27c628f8fc56)**.

[Current component](https://github.com/isogeny-labs/crypto/blob/automation/semaev-veronese-projective-20261123/summation_split/m31_native/README.md) · [Projection report](https://github.com/isogeny-labs/crypto/blob/automation/semaev-veronese-projective-20261123/summation_split/m31_native/projection_20261124/REPORT.md) · [Prospective protocol](https://github.com/isogeny-labs/crypto/blob/automation/semaev-veronese-projective-20261123/summation_split/m31_native/projection_20261124/PROTOCOL.md) · [Written argument](https://github.com/isogeny-labs/crypto/blob/automation/semaev-veronese-projective-20261123/summation_split/m31_native/projection_20261124/PROOF.md) · [Qualification](https://github.com/isogeny-labs/crypto/blob/automation/semaev-veronese-projective-20261123/summation_split/m31_native/projection_20261124/evidence/QUALIFICATION.json) · [Handoff](https://github.com/isogeny-labs/crypto/blob/automation/semaev-veronese-projective-20261123/summation_split/m31_native/HANDOFF.md)

Data claim **PCC-20261124-3de82e** and interpretive claim **PCC-20261124-0c52ec** are unsigned statements. Canonical PCC standing is **unverified**; the local algorithm's `Verified` enum is not signed standing. Source archives, separate build/pre-call/post-call manifests, unchanged executable hashes and raw logs are retained. [pcc:unverified]

Earlier source/evidence commit c418251c7a442cd7360986f7711daa3820395e7c and its frozen reports remain unchanged. Its 27+2 test receipt, 500 specialized fibers, 792 whole-equation certificates, 400 monolithic exact-value checks and 800 prepared input files remain separate historical evidence categories. Monolithic files repeat across witness modes. The earlier five producer executions and this continuation's one execution all concern the same unique coupled fixture. [pcc:unverified]

Historical 8,424,950→6,420,200 operation counts and GCD-inclusive 1.006x/1.022x/1.056x ratios remain archived records.

### Open gates — keep draft

- The requested ≥50 complete projected outputs per configuration and matched degree/runtime/RSS campaign require authenticated shared admission and checkpoint custody. Workspace #109 owns connection restoration. No scientific campaign or extra coupled CLI execution was launched.
- The fresh runtime has restored pinned Rust. Sage/Singular/msolve are currently absent. The full root-library gate was not rerun on the incomplete sparse root checkout; earlier 45s/180s compilation caps are historical, and main's repairs do not validate this older branch.
- GitHub's [artifact migration job](https://github.com/isogeny-labs/crypto/actions/runs/45441435994/job/138613342662) stopped at an S3 `HeadObject` 403 while checking the preserved prepared-input archive. Artifact-store access remains an integration gate.
- The PCC chain extends the published branch baseline, but main-prefix reconciliation remains open. All earlier entries and verifier-limit receipts are preserved.
- The current supplement/README report prose gate passes. Its initial negative-word lint finding is retained with the corrected rerun; earlier frozen report lint receipts remain historical. No merge or signed-verdict change.

## Original PR description — historical evidence

## Result: cubic projective tensor, exactness and a local first-fall trap

Follow-up to merged #5058, #5044 and #5023. Only the Semaev equation is changed; no factor-base map L. Producer-only [pcc:unverified].

- Normalized projective Veronese chain uses degree at most three. At m=5 the geometric arm has 21 variables and 19 generator slots; explicit Fp² coordinate arm 27/29. These are before original-input field equations. Infinity and extension witnesses retained.
- Exact fixed-degree resultant versus independent Fp² root propagation: 200/200 random affine-slice projected sets, 2,100 point decisions and 225 positive generator certificates across four small-field cells.
- SymPy lex diagnostics: 26 completed solves matched the gated root sets; six split m=5 workers timed out. All completed paired split arms slower than monolithic. One-free-parameter comparison only; no F4, no speed claim.
- Exact direct Macaulay ranks, including Prime31 m=3–5: first direct fall at degree three. Uniform chart identity explains it. Pre-inserting redundant local quadratics removes every observed degree-three fall; bounded local first fall is not bounded global solving degree.
- Three tested source files have exact matching GitHub blob identities. No independent CI/PCC, Sage/FGb/msolve/Singular or paid compute. [pcc:unverified]

[Derivation, negative panel and next experiment](summation_split/VERONESE_FINDINGS.md) · [Exact gate CSV](summation_split/veronese_gate.csv) · [Partial timings with run provenance](summation_split/veronese_bench_partial.csv) · [Filtered rank CSV](summation_split/veronese_fall.csv) · [Preclosure control](summation_split/veronese_local_closure.csv) · [Rank plot](summation_split/veronese_degree3_falls.svg).

Next: true instrumented F4, at least two free parameters with matched original-variable slicing, preclosed and balanced-tree arms, 50 full projected-set gates per configuration. Record actual maximum Macaulay degree, runtime and memory. Keep draft; no merge. [pcc:unverified]


