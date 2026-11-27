# Checked projection on the prescribed four-point M31 section

Prospective development protocol, 2026-11-24. Existing lane: crypto PR #5087,
branch `automation/semaev-veronese-projective-20261123`; tracking commit
`f80a6afef699299b9c3cee5bd4ddf54517280dfa`. This supplements the earlier
frozen protocol and evidence; it does not replace them.

## Component and fixed mathematical scope

Implement exact parameter projection from a completed native ambient
Gröbner basis on the existing section
`t*(t-1) = u*(u-1) = 0`, at p = 2147483647. Preserve the original curve
`y^2 = x^3 - 3*x + 21`, factor-base map, fixtures, four formulations and
witness semantics. No field equations are added to geometric auxiliary
coordinates. For decomposed geometric inputs, the ambient ideal includes
every fixed chart generator as well as the supplied input equations.

The implementation must check the candidate basis against the original
ideal using the separate ordinary Buchberger verifier. It must then check
membership of both section polynomials and reduce the four indicator
idempotents `(1-t)*(1-u)`, `(1-t)*u`, `t*(1-u)`, `t*u`. A nonzero normal form
selects that parameter fiber. Return exact elimination generators
`t^2-t, u^2-u, Q`, with Q the sum of the excluded indicators. These are
generators, not a promised reduced Gröbner basis.

## Acceptance and failure semantics

- Bind verification to the actual original inputs and candidate basis in
  the same call; do not accept an externally asserted verification flag.
- Return a complete point set only after verification, both section tests
  and all four indicator reductions finish.
- Keep invalid inputs/rejected bases, resource-limited computations and
  complete empty output distinct. Do not return partial points as complete.
- Record verifier counts separately from projection queries, ordinary
  division reductions, leading-term iterations and maximum division degree.
- Enforce finite declared projection work/term/degree limits. The current
  degree-compatible order bounds each degree-two division by the number of
  monomials of degree at most two; this is a postprocessing bound only.

## Bounded development qualification

Use Rust only. Small exact algebraic unit controls all use M31 and are
development correctness checks; they do not count as a Semaev comparison
cohort. Controls include full, empty, three-point and disconnected support;
nonreduced auxiliary fibers; an extension-only auxiliary witness satisfying
`w^2 = -1`; the infinity chart; missing section equations; invalid parameter
indices; forged/incomplete/non-Gröbner producer results; and explicit
verification/projection limit exits. Check the elimination generators
against each analytically known support.

Extend the existing unique `fixture_plane(3,0)` geometric sequential test
to compare the extracted support with `systems::expected_projection`.
Reuse its frozen producer caps (2000 pairs, 512 rows/columns, 25000 sparse
entries, 64 basis entries, degree 8, 32 matrix steps, 20000 reductions) and
ordinary verifier caps (10000 pairs/steps, 1000000 reductions, 128 basis
entries, degree 12, otherwise the producer limits). Projection caps:
1000 leading-term iterations, 1000 polynomial subtractions, 1000 terms,
degree 2. This is one unique coupled Semaev instance even if qualification
requires a corrected executable or replay; retain every failed call.

An optional explicit single-instance CLI may expose the same pipeline for
all four arms and both witness modes. Parser/serialization controls are
unit tests. Do not automatically launch additional Semaev instances. The
scientific 50-instance-per-configuration comparison remains a separate,
previously specified execution gate.

Before each qualification, freeze the source digest and build manifest,
copy each executable into a distinct immutable qualifier directory, and
record separate pre-call and post-call hashes and exit logs. No replacement
of pre-call manifests. Preserve compiler errors and failed assertions.
Run the relevant crate checks. Broader repository build status is separate;
do not infer it from this standalone crate or stale main-build failures.

## Literature and shared-state review

The research-KB wrapper was tried and returned exit 127 (no executable);
there is no loaded research-KB MCP or live Conductor endpoint in this
runtime. Fallback review read the published lane source and evidence,
current main's session log, and adjacent PRs #5120, #5144, #5136, #5137,
#5099, crypto-autoresearcher #3459 and workspace #109. Their published
scopes do not establish live queue ownership. No shared experiment is
launched without authenticated admission and checkpoint custody.

Primary mathematical references read before implementation:

- [Macaulay2 normal forms](https://macaulay2.com/doc/Macaulay2/share/doc/Macaulay2/Macaulay2Doc/html/_normal_spforms.html): membership requires a genuine Gröbner basis.
- [Stacks, idempotent product decomposition, 00EM](https://stacks.math.columbia.edu/tag/00EM).
- [Stacks, idempotents and open/closed spectra, 04PN](https://stacks.math.columbia.edu/tag/04PN).
- [Stacks, Hilbert Nullstellensatz, 00FS](https://stacks.math.columbia.edu/tag/00FS): a nonzero finite-type fiber algebra has a geometric point.

This applies standard algebra to this fixed diagnostic section. It makes
no novelty assertion, no first-fall-to-solving-degree inference, and no
runtime or memory comparison. Preserve the historical operation counts and
the earlier source-pinned evidence unchanged.
