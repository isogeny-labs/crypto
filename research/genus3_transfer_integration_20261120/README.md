# Genus-3 transfer evidence integration

This package is the `crypto`-native assessment of two retrospective experiment
packages staged in `cryptanalysis` PR #1876:

- `experiments/nist-333-transfer-20261120`
- `experiments/genus3-hecdlp-cover-reduction-20261120`

It does not import Python into this repository. It compares the supplied evidence
with the existing native genus ladder and NIST cover study, records which transfer
obligations remain open, and defines the next construction goal. All new research
claims have producer-only standing. [pcc:unverified]

Start with [REPORT.md](REPORT.md), then [NEXT_GOAL.md](NEXT_GOAL.md). The machine-
readable obligation state is in [TRANSFER_ASSESSMENT.json](TRANSFER_ASSESSMENT.json).

Rebuild presentation artifacts:

```sh
cd research/genus3_transfer_integration_20261120
dot -Tsvg figures/transfer-program.dot -o figures/transfer-program.svg
dot -Tpng -Gdpi=180 figures/transfer-program.dot -o figures/transfer-program.full.png
magick figures/transfer-program.full.png -resize '1600x>' -strip \
  PNG8:figures/transfer-program.png
rm figures/transfer-program.full.png
pandoc REPORT.md -o REPORT.pdf --pdf-engine=xelatex \
  -V geometry:margin=0.75in -V fontsize=10pt
```
