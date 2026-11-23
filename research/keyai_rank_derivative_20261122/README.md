# KeyAI derivative support and relation-rank controls

**Integration date:** 2026-11-22 (America/Los_Angeles); native evidence dated
2026-11-23 UTC. **Tracking:** [crypto PR #4980](https://github.com/isogeny-labs/crypto/pull/4980).

This study brings two source-derived controls into the existing Rust code:
the multiplicity-aware polynomial residual `rem(F g', g)` and fixed-target
relation accounting that keeps signed witnesses, canonical rows, and independent
rank separate. The derivative API reuses `semaev_prime_poly`; relation checks
reuse `ic_framework::useful_rank::AuditedMatrix`. [pcc:unverified]

## Read the integration

- [Report and mathematical contracts](REPORT.md)
- [Accounting diagram](ACCOUNTING.svg)
- [Rendered report](REPORT.pdf)
- [Visual review and artifact hashes](visual-review.md)
- [Polynomial implementation](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/src/cryptanalysis/semaev_prime_poly.rs)
- [Relation accounting](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/src/cryptanalysis/ic_framework/useful_rank.rs)
- [Focused Rust example](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/examples/keyai_rank_derivative_controls.rs)

**Focused native qualification completed:** four example tests, five separately
authored adversarial tests, and six selected original module tests passed. The
adversarial suite includes 2,830 powered-divisibility cases. Frozen source and
binary hashes were unchanged after the calls. The full-library release gate
exited 101 before any tests because an untouched baseline source file contains
delimiter errors; see the [baseline receipt](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/research/keyai_rank_derivative_20261122/evidence/root-library-baseline.json).
The recorded qualification claim is
`PCC-20261123-08e731`; the baseline-gate record is `PCC-20261123-db257a`.
Both have **unverified** standing.
[pcc:PCC-20261123-08e731] [pcc:PCC-20261123-db257a] [pcc:unverified]

The deterministic Prime31 fixture recorded 32 ordered coordinate tuples,
64 signed witnesses, eight degree-16 candidate polynomials, and one normalized
relation row of rank one. The target stayed unpinned until one explicit generator
anchor raised the rank to two and recovered `2147446829 = r - 2`, with public
group replay. These counts describe the constructed correctness fixture.
[pcc:PCC-20261123-08e731] [pcc:unverified]

- [Qualification protocol and source selection](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/research/keyai_rank_derivative_20261122/qualifier/README.md)
- [Frozen pre-call manifest](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/research/keyai_rank_derivative_20261122/evidence/qualifier/attempt2/pre-call.json)
- [Native outcomes and post-call hashes](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/research/keyai_rank_derivative_20261122/evidence/qualifier/attempt2/post-call.json)
- [Full fixture output](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/research/keyai_rank_derivative_20261122/evidence/qualifier/attempt2/controls.stdout.json)
- [Full-library gate and baseline comparison](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/research/keyai_rank_derivative_20261122/evidence/root-library-baseline.json)
- [Unsigned claim record](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/pcc/claims/PCC-20261123-08e731.json)
- [Unsigned baseline-gate record](https://github.com/isogeny-labs/crypto/blob/988280cc865c46f0d45cbbfa84fa84ebc05c2852/pcc/claims/PCC-20261123-db257a.json)

Implementation and evidence links are pinned to published commit
`988280cc865c46f0d45cbbfa84fa84ebc05c2852`.

Upstream M16 derivative arguments are written mathematics; the inspected
cancellation Lean files are separately pinned draft sources. No upstream Lean
proof or Python runtime is imported. No execution timing or performance gain is
reported. [pcc:unverified]

## Contract boundaries

Polynomial coefficients live in `Fp`, with primality a caller precondition.
The support APIs check representation, an odd modulus below `2^62`, and
`0 < deg(g) < p`. Support is a property of a supplied polynomial. Rational root
extraction, admissible signed lifting, public group replay, subgroup-order
coefficients, and target determination are separate obligations.

Existing `base_rank` remains the rank of the base-column projection. It is not
renamed or interpreted as the rank of a target-cancelled system. The exact
predecessors and upstream source revisions are in the report.

## Rebuild the PDF

The Markdown report and editable SVG are the source artifacts. On a machine
with a complete LaTeX installation, the following Pandoc/Inkscape recipe renders
their content. Python below only prepares layout input; it performs no scientific
calculation. The current environment instead uses ReportLab after both available
LaTeX engines failed; the exact renderer and review are recorded in
`visual-review.md`. From this directory:

```sh
mkdir -p /tmp/keyai-integration-report
inkscape ACCOUNTING.svg --export-type=pdf --export-filename=/tmp/keyai-integration-report/ACCOUNTING.pdf
python3 - <<'PY'
from pathlib import Path
source = Path('REPORT.md').read_text()
source = source.replace('](ACCOUNTING.svg)', '](/tmp/keyai-integration-report/ACCOUNTING.pdf)')
source = source.replace('<!-- PDF page break -->', r'\newpage')
Path('/tmp/keyai-integration-report/REPORT.md').write_text(source)
PY
pandoc /tmp/keyai-integration-report/REPORT.md --from=markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=18mm -V fontsize=10pt -V colorlinks=true -V linkcolor=blue -V urlcolor=blue -o REPORT.pdf
pdftoppm -r 110 -png REPORT.pdf /tmp/keyai-integration-report/page
```

Read every rendered page after a content change and refresh `visual-review.md`.
No canonical performance chart is changed: this component has no timing series,
speedup claim, or leaderboard promotion. [pcc:unverified]
