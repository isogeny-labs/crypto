# All-cover framework and Smith eligibility

## Scope and decision

The user selected **All-cover classification framework**, rather than a finite
bounded enumeration. The target is the fixed P-256 curve over its original
prime field. Covers are finite nonconstant morphisms from smooth projective
geometrically connected curves, up to isomorphism over the fixed target.

The framework must include separable, inseparable, tame, wild, Galois and
non-Galois covers; it must not label a short list of explicit families as a
classification of all covers. Separate its exact representation theorem,
trait schema, worked models, and implementation boundaries.

## Sources checked before implementation

- KB unavailable: no KB MCP tools; `scripts/research-kb status` exits 127.
- Existing `.agents/skills/ecdlp-surfaces/` and higher-genus reference,
  report-evidence and research-visuals instructions.
- `src/cryptanalysis/smith_kernel_coverage.rs`, `smith_prop24.rs`,
  `smith_trigonal.rs`: 105 pair kernels versus 135 total Lagrangians,
  rationality and genuine-degree checks, and the currently missing canonical
  plane-quartic model / Jacobian-arithmetic bridge.
- Smith, arXiv:0806.2995, primary PDF downloaded and sections 3–7 read:
  Lemma 1, Definition 2, Theorem 1, Propositions 2 and 4, Remark 4,
  and the canonical-model test in section 7.
- Stacks Project, primary `curves.tex`: curves/function fields,
  Riemann–Hurwitz with the different, purely inseparable smooth-curve
  factorization, and `proposition-unwind-morphism-smooth`.
- Stacks Project fundamental-group chapter (`pione.tex`) for the equivalent
  finite-cover representation; no claim that a finite list of Hurwitz strata
  exhausts unbounded genera/degrees.

## Implementation and interpretation plan

Supply a machine-readable trait schema and worked-model catalogue, a written
classification framework, a diagram/PDF, and native correctness checks for
P-256 genus-3 Smith prerequisites. Reuse the existing small-field Smith code
as a comparison implementation. Do not claim a 256-bit full Smith codomain
construction, a closed-form classification of all isomorphism classes, or a
new performance result. New mathematical and execution results are unsigned
producer assertions until independently verified. [pcc:unverified]

The current C3 cover has extra involutions. A provisional derivation suggests
that its rational pair kernels may be degenerate for Smith's required
degree-three map. Test that derivation against exact wide-field arithmetic and
the existing small-field reference before stating a model-specific conclusion.
Keep non-pair kernels, other isogenous models and extension fields separate.

## Environment and outstanding validation

Conductor, KB catalog, Lean and Lake are unavailable in this runtime. Native
correctness checks use Rust; no Python cryptographic execution path is added.
No benchmark or cost-comparison experiment is planned. Full source-to-codomain
Smith construction and independent signed verification remain separate gates.

## 04:23Z — Native checks and report inspection

### What was done (commands, files, features)
- The standalone check script exited zero: four release tests, formatting,
  strict package Clippy and three deterministic JSON comparisons passed.
  Raw output is retained in results/local-checks.txt. [pcc:unverified]
- Inspected all seven rendered PDF pages. The diagram and tables are readable;
  one long primary-source label clips at the right margin on page seven.
- Recorded prerequisite outputs in PCC-20261115-cc6d93, without signed standing.

### Decisions made (and why)
- Keep exact source labels in SOURCES.md and shorten the PDF citation prose.
- Scope strict Clippy to the new package with --no-deps; the unchanged reference
  module has existing style lints and is exercised through comparison controls.

### Open threads (unfinished, pick up next time)
- Fix and inspect the affected PDF page; finish validation and proof records;
  append the shared session handoff and update existing PR 4209.

### Errors (what went wrong, how it was handled)
- The first Clippy invocation included reference-dependency style lints (four
  needless-range-loop and one too-many-arguments); no reference source was
  changed or lint suppressed. New-package strict Clippy passes. [pcc:unverified]
- A stale process poll returned unknown process id after completion. The
  completed check transcript supplies the observed result; no rerun was needed.
