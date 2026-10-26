# Index Calculus for the Elliptic-Curve Discrete Logarithm, Measured

`ic_program.tex` (built: [`ic_program.pdf`](ic_program.pdf)) is the
programme record of the repository's ECDLP index-calculus work from
2026-07-01 to 2026-10-24 (last folded in: 21–24 October): the routes pursued, the measurement infrastructure
(ecbench, the boundary ledger, claim hygiene), what improved and on which
boundary, the negative results, and where the frontier stands.

**Status**: research draft, anonymized, not peer-reviewed, not submitted,
not cleared. A synthesis of committed evidence: **no new measurement** was
run for it. Every number cites a committed artefact (Appendix B of the PDF;
plotted values with their sources in [`figures/data.json`](figures/data.json)).
Restore authors with `make deanonymize` after creating an `AUTHORS` file
(see `../AUTHORS.example`).

**Change class** (per `AGENTS.md`): none of its own; it reports results
already classed by their studies. With one modelled exception (the `k = 4`
crossover of §10.9, an extrapolation fed by a measured constant) every
improvement it reports is *engineering* or *accounting*.

## What it says

- **Measurement** (§4): the unit `S = GAE / √r`, cold one-target cost
  beside the online window, sealed records and exact replays, isolation
  levels L0–L3, paired cluster-bootstrap ratios of totals, the frontier
  comparability key, and seven corrections the accounting forced on earlier
  claims (exploratory online ratios, a Buchberger baseline labelled F4,
  unpriced F4 word XORs, cross-host counter drift, an `f4_fp` soundness
  bug, F6-IC's charged cost, the ECC2K-130 constant).
- **Chronology** (§5): commit activity by ISO week and theme, seven phases
  from 6 September to 24 October, with the July–August prime-field prelude.
- **Root-finding decomposition** (§6): how one suggestion in an audited
  manuscript (the roots of `S₃(x_R, x_F, X)` are `x(R ± F)`) became a
  proved identity (Lean), prime and binary oracles, `m ≤ 4` pair tables,
  the PDP family, a CUDA kernel, Kummer-tower and dense-index variants, and
  four ecbench improvement loops; stage factors 383–1,535× (binary vs
  matrix F4), 3,021–5,802× (NIST vs Buchberger), end to end 50–105× (vs
  msolve); best complete Koblitz `n = 13` arm `S` 38.82 → 22.97, still
  1.55× strong rho.
- **Gröbner engines** (§7).
- **Koblitz curves** (§8): bases and oracles, the September results below
  rho that reversed against a stronger rho, the online ladder and its
  retirement, the full `n = 83` campaign (145 entries, 4–21 Oct), Table A
  rules v3–v8 (4.67–18.07× at v8), and ECC2K-130 (every constructed or priced route at
  least about 5 bits above rho).
- **Random and prime-degree binary curves** (§9, new): the degree question
  measured (fall degrees 3–4, solving degree tracks ℓ within one of
  semi-regular, closure and hybrid studies), the pair-table floor and the
  oracles measured against it, SAT/SMT/WDSat as enumerators, the PDP oracle
  programme, Table B binary rows (10.6–38.2×, validated), ECC2-131.
- **Prime fields** (§10): ladders and Table B prime rows (5.47–13.5×,
  validated), the Kummer tower's solving degree (plateau at 7, `D ≥ 8`
  implied at `N = 21`), 24 structured-base rounds on the P-256 field (no
  unplanted relation), small-x bases, isogeny walks over a million
  P-256-isogenous curves, lattices, ECCp-131, audits of standard curves,
  Gaudry `k = 4` over `F_{p^4}`.
- **Covers and transfers** (§11): the conservation lemma, genus 3 at parity
  with the strongest rho over `q = 2^14..2^20`, the two generations of
  Joux–Vitse comparisons, cover censuses, volcano levels.
- **Frontier** (§12): a table of every measured improvement by boundary and
  class; cold one-target frontier still rho; one admitted online point
  (compact-orbit IC at Koblitz `n = 61`, 13.9× [9.8, 19.1] over strong rho,
  64 targets, set-up excluded; 83× rho with set-up charged).
- **21–24 October** (frontier section, "The ladder continued, 21–24 October"): four factor bases closed under known-log maps
  (Z[τ], Z[φ], integer multiples, LEAD-19 cofactor projection), 1.3–20×
  against the programme's own arms, exponent unchanged; Koblitz Table A
  re-priced at the canonical rho step (9.9–43×); hybrid guess closes the
  (3,7) descent cell, m = 4 row stays +8.4 to +58 bits over rho (+45.7 at
  w = 2 even at degree 5); Joux–Vitse cover of E(F_{p^6}) at 1.6–2.0× a
  pooled rho constant at p = 1009, slope −2.05 (lower bound); the inherited-F4
  replay regression traced to a BMI2 path.
- **Not yet measured**: any comparison with a published index calculus at
  the same (q, n, m) (open problem 10).
- **Appendices**: the artefact index, the open pull requests not used
  (111, titles as filed), and the **catalogue**: all 1,444 study
  directories, benchmark rows, problem records and notes on `main` by
  family, with headline number, class and status
  ([`data/catalogue.tsv`](data/catalogue.tsv), built by
  [`tools/catalogue.py`](tools/catalogue.py), which also checks that none
  is missing).

## Rebuilding the catalogue

```sh
python3 tools/catalogue.py                 # sections/catalogue.tex
python3 tools/catalogue.py --check paths.txt   # paths with no row
python3 tools/open_prs.py                  # sections/openprs.tex
```

## What it does not say

- No ECDLP speed-up on any deployed curve, no key recovery, no asymptotic
  sub-rho claim, no change to the Gaudry–Diem picture for prime fields, no
  ECC2K-130 or P-256 security statement.
- Stage ratios are not end-to-end speed-ups; L0 wall ratios are
  descriptive; lower-bound costs are labelled; exploratory online ratios
  (n = 61/71/73/83) are not admitted results.
- The commit-activity figure counts keyword matches on commit subjects; it
  indicates where effort went, not how much of it was index calculus.

## How to organise the publication

The PDF is a programme record and deliberately long. Appendix A proposes
splitting it into five self-contained papers, each citing this record for
provenance:

| paper | content | owed before submission |
|:--|:--|:--|
| P1. Root-finding decomposition | §6; extends `../rootfinding-decomposition/` with the PDP family, GPU, Kummer tower, ecbench loops and the audit corrections | novelty audit; Magma/msolve cross-check on every row; a GPU run on hardware; `m ≥ 3` prime measurements |
| P2. ecbench: one unit for every ECDLP method | §4, Definition 1, corrections, isolation rubric, frontier synthesis | an L3 cross-host replay certificate; inherited-F4 counter drift resolved; a four-size bound per incumbent |
| P3. Index calculus on Koblitz curves, measured | §8, compact orbit, online ladder, admitted `n = 61` panel, ECC2K-130 extrapolation | isolated panels at `n = 71, 73, 83`; the constant `C` reconciled; a cold `n = 83` session |
| P4. Symmetry at `k = 4` | §10.9 | a second prime and more seeds; the degenerate target explained; a measured end-to-end point |
| P5. Negative results | §§9–11, lit2026 negatives, SAT scaling, symmetrised bases, fibre-aware IC, Lemma 1 | literature check of Lemma 1; the conductor-gap yield experiment |

Each paper should follow: the question stated as a boundary (Definition 1)
→ the mathematics with proofs or certificates → implementation and
fail-closed verification → results on the stated boundary first, stage
results second → what is not established → reproduction (commit, binary
hash, spec, session id, replay certificate). P1, P3 and P4 suit a
cryptanalysis venue via IACR ePrint (`iacr-eprint` skill); P2 an artefact or
methodology track.

## Build

```sh
make          # pdflatex + bibtex
make check    # rebuild; fail on LaTeX errors, unresolved refs/cites, overfull boxes
```

Figures are rendered from `figures/data.json` (matplotlib, recorded values
only; PDF for LaTeX, SVG and 150 dpi PNG for browsing):

```sh
python3 figures/make_figures.py
```

## Reproduction of the commit-activity tally

From a full (unshallow) clone at the paper's base commit:

```sh
git log --since=2026-07-01 --format='%ad|%s' --date=format:'%G-W%V'
```

Themes are case-insensitive regular expressions on the subject:

| theme | regex |
|:--|:--|
| Koblitz / binary IC | `koblitz\|n83\|n53\|n61\|n71\|n73\|orbit\|frobenius\|ggmp\|binary ic\|boundary` |
| Gröbner F4/F5/F6 | `\bf4\b\|\bf5\b\|\bf6\|groebner\|gröbner\|macaulay\|m4ri\|inherited` |
| root finding / PDP | `root.?find\|rootfind\|semaev\|pdp\|pair.?table\|subtract\|summation` |
| ecbench / measurement | `ecbench\|isolab\|replay\|audit\|icms\|bound` |
| isogeny / transfer | `isogen\|conductor\|fiber\|ghs\|weil\|transfer\|glv\|gls\|volcano\|jacobian\|genus` |
| prime-field IC | `prime\|gaudry\|pkm\|kummer\|p-256\|p256\|nist` |

## Before submission

- [ ] Split per the table above; each split paper gets its own directory here.
- [ ] Verify every bibliography entry against the publisher's record.
- [ ] Re-read every cited artefact at the commit of submission (numbers can
      be superseded by later studies; frozen evidence is not rewritten).
- [ ] `make deanonymize`, `make check`, then the `iacr-eprint` workflow.
