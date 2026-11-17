# Index-calculus diagram companion

This imports the existing research-level diagram package into the repository
requested by the owner, `isogeny-labs/crypto`. The earlier PR in the website
repository was a destination error. The original package is preserved byte for
byte under `archives/diagram-companion-v1/`; its file identities are recorded in
[PROVENANCE.json](PROVENANCE.json) and [SNAPSHOT.sha256](SNAPSHOT.sha256).

Open the [ten-figure offline viewer](archives/diagram-companion-v1/diagrams/research/index.html)
in a browser, or read the [ten-page PDF](archives/diagram-companion-v1/diagrams/research/research-diagrams.pdf).
The [four-figure introductory set](archives/diagram-companion-v1/diagrams/index.html)
is also retained. Every figure has SVG and PNG versions; original Python
sources, LaTeX formula sources, public parameters and toy records are included.

## Scope and status

| Requested item | Import status | Boundary |
| --- | --- | --- |
| Research diagrams, editable sources and exports | Preserved | Ten research figures and four introductory figures; not a new experiment |
| Prior arithmetic and viewer checks | Historical evidence | Recorded before this repository import; not a fresh native replay |
| Correct destination | This study directory | Replaces the misplaced website PR |
| Native research implementation | Not added | Archived Python is source evidence, not an adopted research execution path |
| End-to-end index-calculus advantage | Not established | No challenge logarithm, measured rho comparison or speedup claim |

The figures compare elliptic and hyperelliptic relation collection, signed
large-prime cancellation, sparse linear algebra, Semaev/Weil-restriction systems,
explicit covers, Frobenius projections, and total-cost accounting. The public
challenge is ECCp-131 over its original prime field. Its quadratic and cubic
extensions retain the original target subgroup. Ambient-Jacobian benchmarks,
exact identities, heuristic assumptions, and illustrative costs remain labeled
in the archived viewer's figure notes.

The toy relations are constructed examples. The finite fiber counts concern
one stated small-field fixture; they do not enumerate the whole Jacobian.
The archived timing fields are historical Python diagnostics without the lab's
native isolation, matched-reference or performance-admission receipts. They
must not be used as a benchmark or leaderboard result.

## Validation and reproduction

Run the import-integrity check from any working directory:

```sh
sh research/eccp131_index_calculus_diagrams_20261026/verify_snapshot.sh
```

It checks every imported file against its SHA-256. The provenance also records
the source Git blob, byte length and original path for each file. The viewer
works offline without executing the generators.

Before import, the original package passed public-parameter and extension
arithmetic checks, six toy index-calculus controls, worked relation/rank
identities, the fiber census, SVG-reference and label-bound checks, interactive
viewer checks, and PDF/ZIP export checks. These are historical validation
statements; this import does not rerun the Python research path. The original
reproduction instructions are retained inside the archive, not adopted as the
current repository workflow. Follow [the native-execution rule](../../AGENTS.md)
before extending that research path or making new comparisons.

The root Rust library suite was not run in this artifact-only import workspace,
which has no crypto source checkout. Current-head repository CI and review
remain separate gates. Conductor was unavailable (`command not found`, exit
127); its tracked workflow path returned 404, so no task reservation is claimed.

## Sources and relationship to existing work

The import review checked the repository's [cover-transfer assessment](../cover_transfer_assessment_20261013/README.md),
[ICV1 naming rules](../../docs/curves/ICV1.md), root agent instructions, and
report-evidence/research-visuals skills. The assessment already distinguishes
cover geometry from induced-map implementation and full-Jacobian solving;
these diagrams make no new catalog or solver claim.

Primary sources are linked in the archived viewer: Certicom's published
parameters, Semaev's summation polynomials, Gaudry's extension-field index
calculus, Diem's fixed-genus and plane-curve algorithms, and Wiedemann's sparse
linear algebra. The original artifact source is pinned to
[commit 8698de0](https://github.com/aburan28/isogeny-labs/commit/8698de0d53a1d1a93bffb9f4d670428dfdace827).
No novelty, new theorem certificate, native solver, or newly admitted measurement
is asserted by this repository move.

This import changes no canonical scoreboard, leaderboard, curve registry,
cover-transfer ledger, lab-browser data or benchmark row: it introduces no new
measurement, registered model, reclassification or implementation capability.
