# Independent same-team source review

This is an unsigned source-review note, not an independent signed scientific
verdict. Two non-author reviewers inspected the production timing change and
the accounting implementation before the final frozen calls.

## Production visitor timing

Reviewed guards for exclusive wall phases, early returns, plain/traced/reuse
dispatch, default-off serialization, and public API compatibility. The batched
recognized-univariate dense coefficient preparation initially fell into the
residual bucket; a root-recovery guard now covers it. Added a real cold/warm
certified trace-reuse test. The eight unchanged production tests are compiled
through a separate exact-source test target. No arithmetic helper is replaced.

Final production source SHA-256:
`367a323b5bec3df0afe4859cb995c0116a62b336fbc04c1917af33b84f75728e`.

## Signed extraction and useful rank

Reviewed signed-root decoding, zero-ordinate branch handling, finite prefix
replay, scalar-field admission, public schedule rows, G anchor, duplicate
projections, partial traversal, and independent-oracle boundaries. Corrected
the initially inverted true-means-stop visitor callback before any native run.
The zero-y L control now supplies both line assignments and checks each against
the original equations. M31 controls check subset inclusion when incomplete and
exact equality when complete, with independent enumeration outside candidate
work. Group ledgers explicitly emit fields omitted by the default serializer.

Final accounting source SHA-256:
`8c739b9b95dfb7942c854eee8f5bec0d235f87f01fe5c47d4cc8a57e3b3f6c1f`.

The scoped source review found no remaining blocker to the fixed development
control. It does not replace the full-root build gate, establish a calibrated
total primitive cost, or authorize a target cohort or parameter sweep.
