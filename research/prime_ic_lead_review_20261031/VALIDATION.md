# Validation receipt

This receipt covers the 2026-10-31 literature/repository assessment and its
rendered report. It is not an experiment export and contains no new solver or
ECDLP measurement.

## Starting identities

- `crypto`: `2c03791383f4bb4ce200e84bdef4309065c61e7f`, clean `main` before the
  study branch was created.
- `crypto-autoresearcher`: `5338aead98855d578d444fa6d8b507370847217c`.
- Branch: `codex/prime-ic-lead-review-20261031`.

## Coordination check

Command:

```sh
conductor check --summary "Rank prime-field index-calculus leads and preregister discriminating experiments" \
  --scope path:research/prime_ic_lead_review_20261031 \
  --scope path:docs/session-logs/2026-10-31.md
```

Outcome: exit 127, `conductor: command not found`. `docs/CONDUCTOR.md` permits
warn-and-allow when the binary is absent. No reservation was claimed.

## Primary-source retrieval

The PDFs were retrieved into a temporary directory and not added to the
repository.

```sh
curl -fsSL \
  https://raw.githubusercontent.com/bochum-crypto/subexponential-ecdl/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf \
  -o bochum.pdf
curl -fsSL \
  https://raw.githubusercontent.com/openai/math/7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf \
  -o eth.pdf
curl -fsSL \
  https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf \
  -o pkm.pdf
sha256sum bochum.pdf eth.pdf pkm.pdf
```

Observed hashes:

```text
82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11  bochum.pdf
5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123  eth.pdf
a53030a6cde720c354aebfb65b1eb01e07766ce2d26e543d77f2516c0a49ef5b  pkm.pdf
```

`pdftotext -layout` completed with exit 0 for all three. The source sections
read directly were Bochum's abstract, introduction, Theorems 2 and 4 and the
decomposition encoding; OpenAI Appendix D.1 and D.4; and PKM's contribution,
partial analysis, experiments and conclusion.

## Lead-atlas review

Commands were run at the autoresearcher revision above:

```sh
python3 tools/lead_atlas.py --brief
python3 tools/lead_atlas.py --match "SAT SMT Semaev" --limit 20
python3 tools/lead_atlas.py --match "PKM" --limit 20
python3 tools/lead_atlas.py --match "hashed factor base tensor" --limit 20
python3 tools/lead_atlas.py --match "shared coefficient membership" --limit 20
python3 tools/lead_atlas.py --delta
```

The brief reported 153 total leads: 74 closed with a named obstruction, 53
open with a decided next step, 17 blocked and 9 unexamined. Selected rows named
in `README.md` were then read in full. This review did not stamp or change the
atlas because it generated no new theorem or measurement.

## Protocol validation

Command:

```sh
python3 ../crypto-autoresearcher/tools/experiment_protocol.py validate \
  research/prime_ic_lead_review_20261031/protocol.draft.json
```

Outcome:

```text
OK: experiment.protocol/v1 research/prime_ic_lead_review_20261031/protocol.draft.json (0 finding(s))
```

The record remains `freeze.status: draft`. No `freeze` command and no affected
data collection occurred.

## Figure and PDF rendering

Commands:

```sh
dot -Tsvg figures/lead-funnel.dot -o figures/lead-funnel.svg
dot -Tpng -Gdpi=150 figures/lead-funnel.dot -o figures/lead-funnel.png
XDG_CONFIG_HOME=/tmp/prime-ic-inkscape-config \
XDG_CACHE_HOME=/tmp/prime-ic-font-cache \
inkscape figures/pkm-m4-degree.svg --export-type=png \
  --export-filename=figures/pkm-m4-degree.png --export-dpi=150
pandoc REPORT.md --from=gfm --resource-path=. --pdf-engine=xelatex \
  -V geometry:margin=0.65in -V colorlinks=true -V linkcolor=blue \
  -V urlcolor=blue -o report.pdf
pdfinfo report.pdf
pdftoppm -png -r 105 report.pdf /tmp/prime-ic-final-page
```

Graphviz and Pandoc returned exit 0. Inkscape returned exit 0 with read-only
profile/font-cache warnings after the writable XDG scratch paths were supplied.
`report.pdf` has four letter-size pages. All four rendered pages were opened;
the figures, source pins, experiment gates and dependencies are legible and no
text crosses the page boundary.

## Scientific execution status

- New experiment rows: 0.
- EKS jobs submitted: 0.
- Solver/benchmark claims: 0.
- New algebraic identities: 0; no identity certificate is required.
- Leaderboard/benchmark updates: none, because there is no admitted result row.

## Repository test availability

The required broad Rust check was attempted after staging the report:

```sh
cargo test --release --lib
```

Outcome: exit 127, `/bin/bash: line 1: cargo: command not found`. No Rust source
or build manifest changed in this task. JSON/schema, curve-name, render and PDF
checks above completed; the unavailable Cargo toolchain remains a validation
environment limitation rather than a passing test.
