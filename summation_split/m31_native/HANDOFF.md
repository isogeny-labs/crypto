# Current continuation: checked native projection

The 2026-11-24 continuation adds exact geometric projection from the checked
native basis on the fixed four-point M31 section. The first frozen attempt
passed 40 library and five CLI tests. The existing unique m3/case0 geometric
sequential fixture returns exactly `{(0,0)}`, matching the signed-point
oracle. Extraction uses six normal-form queries, 29 leading-term iterations
and 28 polynomial subtractions at degree at most two. See
[the projection report](projection_20261124/REPORT.md),
[structured receipt](projection_20261124/evidence/QUALIFICATION.json),
[proof](projection_20261124/PROOF.md) and
[integration state](projection_20261124/evidence/INTEGRATION.md).
The new records are `PCC-20261124-3de82e` (data) and
`PCC-20261124-0c52ec` (interpretive), both with unverified standing.
[pcc:unverified]

`solve-one` now exposes an explicit bounded native pipeline for the four
arms and both witness modes, retaining explicit-Fp2 relations. It is built
and its adapter/serialization are unit-tested; no extra coupled scientific
instance was launched through the CLI. The next admitted execution remains
the full 50-per-configuration projected-output gate and matched cost panel.
Authenticated shared admission and checkpoint custody remain unavailable.
Keep the existing PR draft; the broader comparison is unfinished.

The remainder of this file preserves the earlier handoff as historical
context. Its statements about absent native projection and 29-test counts
describe the earlier source commit and are superseded by the supplement
above. Earlier frozen evidence and source-pinned reports remain unchanged.

## Earlier publication and gates

Source/evidence commit:
`c418251c7a442cd7360986f7711daa3820395e7c` on existing draft PR #5087.
All 71 published blob identities were checked after fetching that head.
`evidence/SOURCE_PUBLICATION.json` records the exact subjects.

The final frozen component suite passed 27 library and two CLI tests. The
recorded coupled F4 result is one unique m=3 geometric sequential fixture,
with 561 pairs, ten matrices and observed raw/normalized degree six. A
separate ordinary Buchberger path checked both ideal containments. These
are local algorithmic observations, not independently signed scientific
standing. [pcc:unverified]

Unsigned PCC records distinguish observations from interpretation:

- `PCC-20261124-705c2d`: data and development qualification.
- `PCC-20261124-2c66bc`: written mixed-sector multiplicity/guard interpretation.

The canonical PCC tool reports `not_covered` for unsigned statement claims
and `unverified` standing. No signer/keyring modification or execution-proof
substitution was performed. The branch's append-only chain extends its own
published source baseline, but the main-base prefix gate is still open:
the older branch has fewer ledger entries than current main. Its history
has been preserved; normal reconciliation is required before merging.

The prose lint also reports two literal matches in negative performance
disclaimers: README.md line 102 (`speedup`) and REPORT.md line 131
(`speedups`). The test did not report a numerical overclaim. Its exit 1 is
retained in `evidence/PROSE_LINT.txt`; a passing report-lint gate is not
claimed. The already attested report bytes remain unchanged. [pcc:unverified]

## Exact next execution

Use `evidence/prepared-inputs-m3-m4.tar.gz`, whose SHA-256 is
`a61cf6887d76c675608f5d4f5b2e71aebf9cbb556c6ce7abd797adbde81cd800`.
It contains 800 input files, including duplicate monolithic inputs across
witness modes, with 50 cases in each arm/arity/mode cell. The archive is input
preparation only. Geometric decomposed headers match the quotient engine's
variable order. Superseded qualification01 inputs must not be substituted.

After shared admission and checkpoint custody are restored, execute the
four-arm projected-output gate, then the matched timing/RSS comparison in
PROTOCOL.md. Preserve every failed or censored attempt. The component does
not yet integrate the production F4 visitor or extract roots from its own
basis. The prepared section has four M31 points and does not establish
natural factor-base scaling.

The broader repository library build stopped at 45s and 180s compilation
caps before tests. The scientific comparison also requires the shared
coordination, supported runner admission and checkpoint transport already
owned by workspace PR #109. Keep this PR draft; no merge is authorized.

The historical 8,424,950 and 6,420,200 counts, and the GCD-inclusive ratios
1.006x/1.022x/1.056x, remain archived operation counts. No new runtime,
memory, solving-degree theorem or ECDLP speedup is claimed. [pcc:unverified]
