# Quadratic remainder relation component

[Concurrent publication integration](INTEGRATION.md): the incoming stage40
prototype now has a separate rank-comparison record. These reusable-library
checks do not reprice it. Canonical claim PCC-20261116-dfda61 reissues the
development observations, preserving both original claims and the exact
development journal fork; standing remains unverified. [pcc:unverified]

The opt-in Rust API `TwoLargePrimeSieve::verified_quadratic_remainder` keeps
seven signed small points and an Fp-irreducible quadratic as a reduced
divisor over Fp². `filter_quadratic_remainders_checked` assigns bounded
canonical class labels, retains orientation coefficients, checks original
Jacobian identities again and replays filter output certificates.
The existing collectors and earlier evidence remain unchanged.
[pcc:unverified]

[Development contract](../PROTOCOL_QUADRATIC_REMAINDER_COMPONENT.md) ·
[Report](report.pdf) · [Diagram](flow.svg) ·
[Native source](../../../../src/cryptanalysis/jv_sieve_quadratic_remainder.rs)

The extractor checks the exact source norm, smooth curve, squarefree support,
seven distinct small roots, a monic irreducible quadratic, denominator
invertibility, `V²=H mod U`, and the full original Jacobian equation. Its
class descriptor compares the reduced divisor with its negative, keeping
the chosen sign. No conjugacy identification is used. [pcc:unverified]

Development checks are recorded in `../validation/quadratic-remainder-*.log`.
The final03 frozen-source release run reports19 passing scoped tests,
including4 new controls. The actual-cover control accepts
`U=x²+146x+78` at p251/seed2, B-index30, s147; the old split-Fp materializer
refuses that same input. Source03 hashes match after execution.
PCC-20261116-2329b3 has unverified standing. [pcc:unverified]

The constructed control starts from an independently specified principal
norm; the point-level replay splits U over Fp² and reconstructs both
non-Fp points. An actual-cover control searches the fixed p251/seed2 finite
B-space for one admissible remainder. This is an applicability control, not
a yield estimate. Opposite copies must produce one class and zero new rank.
Refusal cases cover malformed norms, wrong fields, split or repeated
support, altered group equations and resource limits. [pcc:unverified]

The first root test build stopped on64 diagnostics, including off-disk
tracked include fixtures. The retained build log records that attempt; the
exact dependencies were materialized before subsequent checks. No global
parent test-suite pass is inferred from a filtered run. [pcc:unverified]

Development tests01 passed2 controls and failed the graph assertion that
cancellation must return no rows. The existing filter emits a zero equation.
The corrected test requires an emitted row with empty coefficients and zero
RHS, checks its certificate replay and preserves the zero added-rank
criterion. Superseded02 compilation was canceled after the test changed;
both logs remain unchanged. Final03 passes this control. [pcc:unverified]

Warnings-denied release library Clippy also returns0. The required broad
parent-suite attempt was bounded at120seconds and exited124; five device
tests failed while their tracked `gpu/certicom` source was off disk. After
restoring that source, all6 focused device tests pass on the unchanged Rust
binary. No whole-parent-suite pass is claimed. The administrative benchrec
gate still reports2489 unavailable checkout/manifest dependencies across215
historical records; its complete output is retained losslessly as
`../validation/quadratic-remainder-benchrec-gate.log.xz`. The prior PR head's
artifact migration had S3 HeadObject403 failures. These gates are not waived.
PCC-20261116-c9e4c3 has unverified standing. [pcc:unverified]

| Requirement | Current status | Evidence |
| --- | --- | --- |
| Native quadratic remainder and signed graph adapter | Implemented; checked locally | Frozen-source03 release19-test log; Clippy log |
| Actual extension-cover applicability | One fixed control checked locally | p251/seed2, B-index30, s147; norm/Mumford/Jacobian replay |
| Additional useful rank and lower complete cost using this library API | Not measured | NEXT.md records the pending matched qualification; separate prototype stage40 is preserved |
| Global validation and merge admission | Unresolved | Parent-suite bound; historical manifest dependencies; PR artifact access |

Useful collision yield, projected rank and a complete target solve using
this library API are pending. A new frozen matched collection must include fully smooth rows,
all remainder partials, distinct class counts, failures, graph work,
descent and original elliptic replay. No timing, total-cost comparison,
cloud experiment, signed PCC verification or performance claim is made.

Sources checked: the exact native modules named in the contract, the prior
JV sections3.2/3.3 review and GTT February2005 sections2/3. KB unavailable
(no MCP; wrapper exit127). Fresh JV PDF retrieval returned403. No novelty
or absence claim follows from this source review.

[Next useful-rank qualification](NEXT.md) records the constrained class
universe, separate-block projection and required matched-cohort observables.
Priced dashboards remain unchanged: this development component supplies no
new priced curve, frozen comparison, reference or leaderboard figure.
