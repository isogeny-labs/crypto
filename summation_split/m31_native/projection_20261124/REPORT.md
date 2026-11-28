# Native M31 projection: completed component and bounded qualification

The native reference solver now extracts the exact geometric parameter
support on the prescribed four-point section from a checked ambient basis.
The first frozen qualification passed 40 library tests and five CLI tests.
The existing unique m=3 geometric sequential fixture returned exactly
`{(0,0)}`, matching the separate signed-point oracle. These are producer-local
observations in `PCC-20261124-3de82e`, with canonical standing **unverified**.
[pcc:unverified]

## What changed

`f4::projection::verify_and_project` checks the actual original ideal and
candidate basis in the same call, using the separate ordinary Buchberger
path. It checks membership of both section equations and then reduces the
four parameter indicator idempotents. A point is included exactly when its
indicator has a nonzero normal form. The mathematical justification and
assumptions are in [PROOF.md](PROOF.md); its unsigned interpretive record is
`PCC-20261124-0c52ec`. [pcc:unverified]

The implementation returns the complete geometric support, the four ambient
indicator normal forms, and generators of the exact elimination ideal:

    (t²-t, u²-u, Q), where Q = sum of the excluded indicators.

The elimination list is not advertised as a reduced Gröbner basis. Auxiliary
coordinates may lie in an extension field, and the ambient ideal may have
nilpotents or positive-dimensional fibers. Every projective chart equation
remains part of the checked original ideal. [pcc:unverified]

The API exposes a solution only after all checks finish. Its separate
statuses distinguish complete output, a rejected basis or missing section,
and a resource cap in the producer, verifier or projection phase. Incomplete
and rejected results carry no point set; a completed empty support does.

`solve-one` exposes the pipeline as an explicit single-instance command.
It preserves all equations in the explicit Fp2 representation and uses the
chart quotient only for decomposed geometric systems. Its signed-point
oracle runs after projection. The new CLI was compiled and its parser,
presentation adapter and result serialization were unit-tested; it was not
used to launch an additional coupled instance during this qualification.

## Recorded coupled fixture

All values below refer to `fixture_plane(3,0)`, arm `preclosed_sequential`,
mode `geometric`, with the unchanged M31 curve and factor-base map. Raw
output is [tests-lib-01.log](evidence/qualification-01/tests-lib-01.log).
These counts have unverified PCC standing. [pcc:unverified]

| Stage | Observation |
|---|---|
| Native F4 | Complete; 30 nonfixed basis polynomials plus four fixed chart polynomials |
| Critical pairs | 561 processed of 561 scheduled |
| Matrices | 10 |
| Maximum raw product degree | 6 |
| Maximum post-normal-form matrix-column degree | 6 |
| Ordinary candidate checking | 561 candidate S-pairs |
| Ordinary original-ideal construction | 528 Buchberger pairs; 33 basis polynomials |
| Both containments | 11 original generators and 30 candidate generators checked |
| Projection support | `{(0,0)}`, equal to the signed-point oracle |
| Projection queries | Two section checks and four indicator queries, all complete |
| Projection work | 29 leading-term iterations; 28 polynomial subtractions |
| Maximum projection division degree | 2 |

The projection work counts begin after the independent basis check. They
exclude basis construction, verifier work, basis scans, coefficient work
within a subtraction, process startup and input preparation. No timing or
memory comparison is inferred from them. The matrix degrees are observations
on this fixture, not a general solving-degree bound.

This extends the existing unique coupled fixture; it does not create another
unique instance. The earlier record contains five executions of that same
fixture, including the retained verifier-limit attempts. This qualification
adds one execution, for six recorded executions of one unique fixture and
one distinct fixture with complete native projected output.

## Qualification coverage

The source was copied into a separate crate and built with pinned Rust
1.100.0, `--release --locked --offline`. Test and CLI executables were copied
to a separate read-only qualifier directory, hashed before calls and again
afterward. The source and executable hashes matched. Both builds and both
test calls exited zero on the first attempt. [pcc:unverified]

| Check group | Scope |
|---|---|
| Library suite | 40 passed, zero failed or ignored |
| CLI suite | Five passed, zero failed or ignored |
| New analytic projection controls | 13 test functions, including all 16 subsets of the four-point section |
| Geometric semantics | Nonreduced fibers, a witness requiring Fp2, the infinity chart, and a free auxiliary coordinate |
| Rejection controls | Missing section equations, invalid parameter indices/widths, a forged unit basis and generators that fail the Gröbner criterion |
| Limit controls | Producer and verifier caps; each projection cap; cap reached after partial indicator progress |
| Elimination output | Independently interpolated Q, its four exact values, and separate ordinary division checks of the returned generators and indicator normal forms |
| New CLI controls | Strict options/JSON escaping, all-arm representation adapter, and distinct complete/empty/incomplete/rejected/disagreement serialization |

The analytic ideals are unit controls over M31. They are not Semaev fixtures
and do not count toward the requested 50 complete solver outputs per
configuration. The earlier multiplicity and generator-witness panels remain
separate evidence categories.

## Evidence and practical next step

The [structured qualification receipt](evidence/QUALIFICATION.json) points to
the retained raw logs, source archive and separate build/pre-call/post-call
manifests. The [prospective protocol](PROTOCOL.md) was frozen before these
calls. All earlier frozen source, reports and failed-attempt receipts are
preserved. The historical operation counts 8,424,950 and 6,420,200 and the
GCD-inclusive ratios 1.006x, 1.022x and 1.056x remain historical records.

The next execution is the already specified comparison of monolithic,
preclosed sequential, preclosed balanced and guarded multiplicity systems,
with at least 50 complete projected outputs per configuration, followed by
matched degree/runtime/RSS accounting. The new postprocessor removes the
native projection implementation gap. Authenticated shared admission and
checkpoint custody are still unavailable; workspace PR #109 owns their
restoration. No scientific campaign was launched here.

The broader root-library gate is separately open: this fresh sparse checkout
does not contain the full root inputs/dependencies, and current main's build
repairs are not automatically part of this older branch. A read-only
[integration record](evidence/INTEGRATION.md) distinguishes this from the
successful standalone qualification. The PCC branch-prefix check and
current-main-prefix check are also recorded separately. Keep PR #5087 draft
while the full comparison and integration gates remain unfinished.
