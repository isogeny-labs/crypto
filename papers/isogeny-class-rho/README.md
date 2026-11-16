# Pollard Rho Across Isogeny Classes: Historical Measurements and Review Limits

`isogeny_class_rho.tex` (built: [`isogeny_class_rho.pdf`](isogeny_class_rho.pdf),
13 pages, A4) is the manuscript of the isogeny-class rho study in
[`research/isogeny-class-rho/`](../../research/isogeny-class-rho/): one
ECDLP instance carried through explicit, verified isogenies to many curves of
its class, the same Pollard rho run on each, with a protocol frozen before
the first measurement.

**Status**: research draft, anonymized, not peer-reviewed, **not submitted,
not cleared**. Target venue: IACR Cryptology ePrint Archive, category
*Attacks and cryptanalysis*. Submission needs the author block restored
(`make deanonymize` from an untracked `AUTHORS` file copied from
[`../AUTHORS.example`](../AUTHORS.example)), then the `iacr-eprint` skill's
form generator; see "Before submission" below. The authors are supplied by
the owner, never invented.

**Review, 2026-10-19:** this manuscript preserves historical results; its
inferential null is provisional. See [the dated review](REVIEW-20261019.md)
and [the proof map](PROOF-MAP.md). Companion source review reproduced a
KS defect (`[1,1]` against itself gives D=1 rather than 0), missing rank-test
tie corrections, and unassessed covariance from shared seeds. Corrected
native analysis tied to the frozen rows and the actual producer revision is
required; the historical p-values and graphs have not been recomputed.

**Change class**: a correction to research claims and their proof boundary.
No admitted speedup or new experiment is claimed. The recorded 1.7–2.2×
wall-time ratio comes from an unisolated shared VM and is not an admitted
performance result.

## What the historical record says

- Scalar transport preserves the logarithm under an injective group
  homomorphism. Equality of an algorithm's operation distribution additionally
  needs coupled oracles, encodings and random choices. Arbitrary injective
  encodings do not make concrete hash-partitioned walks or machine costs equal.
- The study records 85,425 solves on 200 curves across four classes, with
  primary class comparisons inside their reported bands. Ancillary analyses
  include one B1 generator flag, two P1 source–target flags, one P0
  source–target flag and all-arm B1 generator Kruskal–Wallis p=0.039.
  Consequently, “no test fired” is false even before corrected analysis.
- Historical structural-fold ratios include 0.439 [0.401, 0.481] for j=0,
  0.155 [0.141, 0.170] for Koblitz Frobenius and 0.113 with negation.
  These are historical estimates, with inference and admission still under
  review. The reported resolution does not certify detector sensitivity
  after a statistical defect is found.
- The odd-exponent conductor argument restricts prime factors to ±1 modulo 8.
  The Koblitz degrees 409 and 1721 additionally depend on the recorded
  factorization 703889=409·1721 and the stated volcano hypotheses; the mod-8
  restriction alone permits smaller primes such as 7 and 17.
- The Rust source is in companion PR #2174, not contained in this paper's
  branch ancestry. The evidence is in parent PR #2186. Neither relationship
  substitutes for source/producer binding or native-harness admission.

## What remains open

This is a bounded historical study on small instances, with known crater
folds as controls. It establishes no claim about deployed curves, no new
attack, and no admitted performance improvement. Required follow-up includes
correct native statistical analysis, shared-seed covariance assessment,
source/row lineage, sealed native experiment/control/answer/audit records,
and the concrete proof bridges enumerated in the proof map. Frozen raw rows,
protocols and historical receipts remain unchanged.

## How to build

```sh
make          # pdflatex + bibtex, writes isogeny_class_rho.pdf
make check    # references and citations resolved, no overfull boxes
make eprint   # eprint-submission.zip (PDF plus sources)
make clean
```

`microtype` runs with `expansion=false` so the build works on a TeX Live
without scalable Computer Modern (the `cm-super`/`lmodern` packages); with
them installed the option can be dropped. Figures in `figures/` are copies
of the PNG renders in the run directory below.

## Reproduction pointers

Every number cites a committed artefact (paths in the PDF are repository
paths, prefix `crypto:`):

- protocol, frozen before timing:
  `research/isogeny-class-rho/PROTOCOL.md` (sha256
  `81c3232bdab481244324101d10e2d50199da1990ceba10c8dc66cc206286409e`);
- the run: `research/isogeny-class-rho/runs/run-20261018T0857Z/` with
  `COMMANDS.md`, per-class `walk.json`, `class/`, `generators/`, `shift/`
  and `replay/`;
- the tool: `src/bin/isoclass_rho/` (`cargo run --release --bin
  isoclass-rho -- selftest`);
- the long-form report and graphs: `research/isogeny-class-rho/REPORT.md`,
  `report.pdf`, and `benchmarks/20261018-isogeny-class-rho/`.

## Before submission

Owed, in order:

1. **Authors.** `AUTHORS` file from the owner (names in order, affiliation,
   contact email), `make deanonymize`, rebuild; never invented.
2. **Form material.** `python3 .claude/skills/iacr-eprint/scripts/eprint_form.py
   papers/isogeny-class-rho --category "Attacks and cryptanalysis"
   --keywords "…" --license "CC BY"` must exit 0; on the anonymized build
   (2026-10-19) its only failures were the three author rules (anonymous
   author block, anonymous page 1, no email), and the abstract, title,
   keywords and page size passed. `eprint-abstract.txt` is the paste-ready
   abstract from that run.
3. **Novelty audit.** A first pass was done on 2026-10-18 by web search only
   (four queries: random self-reducibility and isogeny experiments; measured
   rho across an isogeny class; the mod-8 condition on conductors of binary
   curves; Vélu point maps in characteristic two). Findings: Jao–Miller–
   Venkatesan (random reducibility) and Galbraith (1999 and the 2024 ePrint
   "Climbing and descending tall isogeny volcanos") are reductions and
   algorithms for moving an instance, with no measurement found; no earlier
   measured whole-class rho comparison was found; no source stating the mod-8
   proposition was found; Vélu's formulas are characteristic-free, so the
   characteristic-two map is a specialisation, not new theory (one
   unreviewed arXiv listing, 2608.01675, writes a binary-curve Vélu map and
   was not checked). The paper states these limits in its related-work
   paragraph and claims nothing as "first". Still owed: a proper literature
   review by an author (Sutherland, Galbraith, Kohel and Lercier–Sirvent
   style isogeny-class computations, and Koblitz-curve isogeny papers),
   before any priority claim.
4. **Evidence still owed.** Corrected native analysis and covariance
   assessment, exact producer lineage, sealed native-harness admission and
   concrete proof bridges. Any performance comparison also requires a
   qualified host and the repository’s isolation/admission controls. A second
   unqualified host does not satisfy those gates.
5. **Consent.** Every named author agrees to the irrevocable ePrint
   license; an author presses Submit, not an agent.

After acceptance, record the ePrint number and the password holder here.
