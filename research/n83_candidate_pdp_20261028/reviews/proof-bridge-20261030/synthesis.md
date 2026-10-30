# Prime n83 proof-bridge technical review

Three independent reviewer reports support the graded-to-filtered bridge for the stated prime selector/Semaev chain with 27 selectors per block and m=3–9. The graded review establishes absence of non-Koszul highest-form syzygies through degree 16 and an explicit degree-17 witness. The filtered review proves degree-tight representation descent and closure, then separates the stored degree-16 remainder from V16 by a nonzero functional while deriving it in V17. The blind review independently derives the degree-17 onset.

These statements use ordinary polynomial total degree, including generator-degree shifts in syzygy coordinates. The auxiliary weights used to order leading monomials are not the degree convention. The characteristic exceeds 27 and all normalization weights are nonzero.

## New centered construction

The blind reviewer additionally constructs P as the product of thirteen paired differences in centered coordinates y_j=z_j-w_j/2. Its alternating projection kills every bounded-product row through degree 16. The projected remainder of P times the terminal equation is nonzero for every target. At t=sum(w_j)/2 its degree drops from 16 to 15, while the coefficient of P U^2 is (w_26/2)^2, which remains nonzero. This is a new symbolic construction, preserved verbatim in TASK-N83-BLIND/report.md. It has not yet been implemented or separately attacked by another reviewer. The next development task is a native exact check of the centered normal form, projection and exceptional-target controls.

## Exact artifact and machine checks

TASK-N83-FILTERED independently implemented a Rust verifier without producer arithmetic imports. On original-prime-m3-natural it checked the prime by an exact Lucas certificate, 17 supplemental file hashes, six protocol source hashes, source reconstruction, polynomial codecs and A factors, the B circuit, C, the full degree-16 remainder and the pairing 4087116827087420414291699. That independent replay covers one case. The separate production panel contains 42 source-functional records; its original metadata remains unchanged.

Fresh Lean 4.34.1 compilation and an exact declaration/axiom audit passed eight arithmetic lemmas. Six controls tested a known counterexample, false bound, missing declaration, sorry, custom axiom and native bypass; all produced the expected acceptance/rejection. Restoring the machine archive from S3 and rerunning Lean reproduced its axiom log. The full polynomial theorem needs formal definitions and proofs for the four obligations in claim-status.json. No independent binary/kernel rechecker was available.

The independence checker passed all three exact attestations against the precommitted checker plan. Its evidence is declaration/source-boundary checking rather than authenticated process isolation. The Coordinator composed this synthesis only after receiving all reports and does not supply an independent fidelity vote.

## Degree quantities and scope

The reviewed bridge yields first strict fall 17 and last fall degree at least 17 for the matching template. An exact LFD or numerical upper bound requires a separate stabilization argument. The blind derivation gives highest-form quotient dimension ceil((m-2)/2); hence there is no first zero homogeneous-component cutoff for m3–9. Castelnuovo–Mumford regularity and solving degree were not computed. The integer 17 is the syzygy/first-fall degree, not a substituted regularity measurement.

This review concerns candidate x-domain equations with unrestricted intermediate variables. The expanded factor-base point lookup ideal is a distinct system. Original fixed bases and the full Koblitz/binary/prime m2–9 study remain in place; no prime-specific theorem is assigned to the other two families.

## Preserved findings and workflow deviations

The original filtered snapshot omitted raw inputs and arithmetic imports. The reviewer retained that finding; a separately committed supplement supplied one exact case. The initial handoff also omitted the native-language policy. The reviewer's Python checker first failed on unavailable int.bit_count, then passed after repair; both are retained as a workflow deviation. The accepted arithmetic evidence is the subsequent Rust implementation. No historical artifact was deleted or rewritten.

Two projections serve different purposes: the older terminal syzygy certificate sets all variables outside the last selector block, including U, to zero to show the H coordinate is nontrivial. The source functional retains terminal U to extract its U^2 coefficient. The older projection is not the definition of the new functional.

## Publication and next work

FORMAT.json describes the packet layout. Individual reports and attestations are verbatim; raw reviewer build outputs and the native binary are in the S3 review archive. Machine and input archives have separate custody records. This technical packet creates no official ledger transition and changes no producer null fields. Earlier production graphs remain historical observations; this packet adds a proof-dependency diagram rather than editing them.
