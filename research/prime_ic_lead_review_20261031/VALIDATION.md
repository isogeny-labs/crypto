# Validation receipt

This receipt covers the 2026-10-31 literature/repository assessment, the
subsequent arity/solver-model correction, the 2026-11-01 protocol hardening and
the rendered report. It is not an experiment export and contains no new solver
or ECDLP measurement.

## Starting identities

- `crypto`: `2c03791383f4bb4ce200e84bdef4309065c61e7f`, clean `main` before the
  study branch was created.
- `crypto-autoresearcher`: `5338aead98855d578d444fa6d8b507370847217c`.
- Branch: `codex/prime-ic-lead-review-20261031`.
- Correction base: `c5d13654`; correction branch:
  `codex/prime-ic-arity-correction-20261031`.
- Hardening base: `7b19d771dca0131a986c7e60d397b4f5c7229d51`; hardening branch:
  `codex/prime-ic-protocol-hardening-20261101`.
- Owner development evidence: `cryptanalysis#1347`, merge commit
  `3c769002f0f68d63a5055f6f3e24a9500393180e`.

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

## Correction evidence audit

- Read the accepted theorem, Lean scope, complete-system equivalence and
  measured continuation in `research/prime_factor_base_degree_20261031`.
  Every accepted relation system there is two-summand; no arity-three solver
  exponent is established.
- Re-derived the complete-cost bridge: `Theta(2^s)` relations times
  `2^(e s)` work per PDP gives `p^((e+1)/m)`, hence the strict rho gate
  `e<m/2-1`.
- Inspected `src/cryptanalysis/sat.rs` at correction base `c5d13654`.
  `solve_with_lazy_clauses` is a native Rust theory-callback contract. No
  source-identified CaDiCaL `ExternalPropagator` implementation was found on
  current `main`; binding the reviewer-referenced loop remains a freeze
  blocker rather than an assumed implementation fact.
- The cross-repository ledger audit found the provenance error: immutable
  `DEC-20261031-cd6460` is the checker-defect decision, while
  `DEC-20261031-b51282` is the arity correction. The old local filename is now
  a compatibility notice; current analysis lives in `PDP_ARITY_CORRECTION.md`
  and the 2026-11-01 hardening is additive as `DEC-20261101-26f1fa`.

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

Original outcome:

```text
OK: experiment.protocol/v1 research/prime_ic_lead_review_20261031/protocol.draft.json (0 finding(s))
```

After the correction, the same pinned validator and schema were retrieved to a
temporary directory and rerun. The corrected protocol ID is
`EXP-prime-m3-cdclt-exponent-screen-20261031`; validation again returned
`0 finding(s)`. The record remains `freeze.status: draft`. No `freeze` command
and no affected data collection occurred.

On 2026-11-01, protocol version 2 was validated with the current in-workspace
validator:

```sh
python3 ../crypto-autoresearcher/tools/experiment_protocol.py validate \
  research/prime_ic_lead_review_20261031/protocol.draft.json
```

Observed result:

```text
OK: experiment.protocol/v1 research/prime_ic_lead_review_20261031/protocol.draft.json (0 finding(s))
```

Version 2 adds the setup exponent, complete natural-panel rule, separate
control populations, exact one-hot sizing/admission gate, scoped schoolbook
cost, signed point semantics, QF_FF proof eligibility, compact range encoding
and corrected additive decision identity. It is still an unfrozen draft.

## Figure and PDF rendering

Commands:

```sh
dot -Tsvg figures/lead-funnel.dot -o figures/lead-funnel.svg
dot -Tpng -Gdpi=48 figures/lead-funnel.dot -o figures/lead-funnel.png
dot -Tsvg figures/protocol-gates.dot -o figures/protocol-gates.svg
inkscape figures/protocol-gates.svg --export-type=pdf \
  --export-filename=figures/protocol-gates.pdf
XDG_CONFIG_HOME=/tmp/prime-ic-correction-inkscape \
XDG_CACHE_HOME=/tmp/prime-ic-correction-font-cache \
inkscape figures/arity-cost-bridge.svg --export-type=png \
  --export-filename=figures/arity-cost-bridge.png --export-dpi=96
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

Graphviz and Pandoc returned exit 0. Inkscape returned exit 0 with harmless
font-map/recent-manager warnings after writable XDG scratch paths were
supplied. The hardened `report.pdf` has five letter-size pages. Its text was
extracted and the newly affected rendered pages were opened; the cost bridge,
hardening diagram and decision text are legible. Its 2026-11-01 SHA-256 is
`01633ab9f601cff2f79e9e50e0bc3d81ce5880bac0f99a7f692e3c165bf2f9dc`.

## Scientific execution status

- New experiment rows: 0.
- EKS jobs submitted: 0.
- Solver/benchmark claims: 0.
- New algebraic identities claimed as research: 0. The reviewer-supplied
  quadratic lift was expanded directly, giving discriminant `16t^2 f(x)`;
  the cost bridge was checked row-by-row against `e=m/2-1`. These are
  correction derivations, not novelty or performance claims.
- Leaderboard/benchmark updates: none, because there is no admitted result row.

## Repository test availability

The required broad Rust check was attempted after staging the report:

```sh
cargo test --release --lib
```

Outcome originally, after the correction, and again after the 2026-11-01
hardening: exit 127,
`/bin/bash: line 1: cargo: command not found`. No Rust source or build manifest
changed in this correction. JSON/schema, cost-bridge, artifact-hash, render and
PDF checks above completed; the unavailable Cargo toolchain remains a
validation environment limitation rather than a passing test.

## Remote delivery attempt

The study was committed locally as
`9d6074a2cf46f25f53363b74281909384be0a84a`. The required branch push was then
attempted:

```sh
git push -u origin codex/prime-ic-lead-review-20261031
```

Outcome: exit 128,
`fatal: could not read Username for 'https://github.com': No such device or address`.
The managed environment has no GitHub credential/identity. No remote branch,
pull request or auto-merge was created; those delivery requirements remain
blocked rather than reported as complete.

### Correction delivery

After the arity/solver-model correction was committed, `origin/main` advanced
from `c5d13654` to `9e15e04d3`. The new base was merged forward without a
rebase; the only conflict was the append-only session log, and both histories
were preserved. The combined tree passed the protocol, JSON, cost-bridge,
artifact, 10 MiB large-file and diff checks again.

The correction branch was pushed successfully at merge head `ecdc04d49`, and
the pull request was opened:

- branch: `codex/prime-ic-arity-correction-20261031`
- pull request: <https://github.com/isogeny-labs/crypto/pull/3096>

This receipt update is committed and pushed on the same branch. Required CI
and guarded auto-merge remain the remote authority for the combined current-
head tree.
