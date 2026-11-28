# Continued polynomial arithmetic study

User requested continuation on 16 November. Continue the substantive Rust work
from the completed local scaling matrix. Existing local/Git-only authorization
persists; no cloud spending. Original protocols/data/reports remain immutable.

## Sources and implementation decision

Sources checked (KB): no KB MCP loaded; wrapper status and both exact searches
(`vieta Karatsuba polynomial division Newton`, `polynomial multiplication
Karatsuba inverse_series Newton division`) returned127. Read repository
`isogeny_eval/poly.rs` Karatsuba/inv_series, `zero_dim_la.rs` Karatsuba/Barrett,
`pkm_smooth_tower.rs`, `prime_fast.rs` and the current Vieta implementation.
FLINT official nmod_poly division/inverse-series documentation and version3.3.1
inverse-series source were checked; a guessed divrem_newton source URL was404.
No novelty claim. The repository already implements these standard methods.

Adapt the existing Karatsuba/series-reversal approach to the generic counted
WordField/BigUint Vieta engine. Preserve `Arithmetic::new` as the original
schoolbook backend and offer explicit Karatsuba and Karatsuba+Newton modes.
Retain the quadratic Vieta formulas, membership representation, extraction,
exception handling and independent scalar replay. Do not infer that classical
Euclidean GCD becomes quasi-linear: short-quotient Euclid remains classical.

## Prospective observation plan

- Same binary/field/input/seed/budgets on all arms: schoolbook, Karatsuba,
  Karatsuba+Newton, its exact A/A repeat, batched lookup and batched lookup with
  the fast mode (unaffected-path negative control). Full ordered outputs agree.
- Proposed primary panel: word62 m8192,16384 and P224 m512,2048, two warmups and
  six retained targets each, alternating random-rational/conditioned populations.
  Supplemental P224/m8192: two retained targets, no warmups, one of each population, kept
  separate. Fixed mixed arm orders; all failures/limits retained.
- Counted field operations are primary; all clocks below L2 diagnostics, never
  an isolated timing/fastest-backend claim. Native Rust analysis frozen before
  calls; unblinded descriptive comparison explicitly disclosed. Three retained
  targets per population is descriptive, not a population significance claim.
- Freeze final exact protocol, source, native analysis, executable hash and
  pre-call publication before calls. Proposed stage cap30s, multiplication
  cap4e9, total call envelope3600s, one CPU, no concurrent builds/tests/analysis.
  Calibration separate and excluded. No missing rows imputed.
- Reuse the prior exact field/curve fixtures to attribute backend changes;
  word62 is a component fixture, not asserted prime subgroup order. This is an
  explained exception to the standard curve panel, not a substituted security
  experiment. P224 retains the standard curve.

## What was done (commands, files, features)

Reviewed prior results and arithmetic sources. Requested Copilot resolution of
open study-lane PR4410 in comment6921041260; no shared conflicted metadata edited
while pending. Conductor missing127, KB unavailable. No new performance data.

## Decisions made (and why)

Use the existing study PR4410 for continuation, update title/body to final scope.
First implement and check arithmetic; then freeze the new experiment. The
completed original study and its proposed degree bound are not rewritten.

## Open threads (unfinished, pick up next time)

Implement backend selection, meaningful polynomial/division tests and same-input
native comparison; freeze/run/replay/analyze/report. Review Copilot response or
use documented timeout fallback before editing shared conflicted metadata.

## Errors (what went wrong, how it was handled)

KB wrapper absent; direct primary/internal fallback. A nonexistent prime_tower
path was corrected to ic_framework/prime_tower. No failed source lookup is used
as an absence/novelty claim. No measurement has started.

## Implementation checkpoint — before calibration

Added explicit schoolbook/Karatsuba/Karatsuba+Newton choices; old constructor
remains schoolbook. Karatsuba handles odd/unbalanced/zero inputs, Newton division
falls back on short quotients, and all primitive products remain counted. New
tests use independent naive products and known quotient/remainder identities
over word and BigUint fields, plus complete relation checks and budget failure.
Seven unit groups and four CLI/replay tests pass; clippy corrected after one
constant-chunk lint [pcc:unverified]. No performance call yet.

The six-arm native qualifier and native descriptive summary are implemented.
Exact A/A and unaffected lookup controls, separate excluded scalar verification,
fixed seeds and full failed rows are retained. Newton may not engage in typical
short-quotient Euclid steps; its explicit counter will show this, without a
presumed whole-GCD complexity improvement.

Copilot did not respond after eight minutes. Comment6921269657 explains the
documented fallback. Main a409d0037a merged as7d807f2ce3, preserving exact main
ledger bytes plus one unchanged missing closeout claim body and full journal
block multisets. The new merge receipt's link typo was corrected before any
publication/freeze. No frozen original receipt was changed.
