# The Elliptic-Curve Discrete Logarithm Does Not Know Which Curve It Is On

`isogeny_class_rho.tex` (built: [`isogeny_class_rho.pdf`](isogeny_class_rho.pdf),
11 pages, A4) is the manuscript of the isogeny-class rho study in
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

**Change class** (per `AGENTS.md`): none. The paper reports a **null result
with passing positive controls**, rho against rho on four isogeny classes.
It is not an index-calculus result, claims no speedup, and makes no
statement about any deployed curve. The one positive finding (the Frobenius
fold on the Koblitz crater, 1.7–2.2× in wall time) is the known Koblitz
speed-up measured, not a new one.

## What it says

- **The null is a theorem** (§2): an $\mathbb{F}_q$-rational isogeny of
  degree prime to $n$ is an isomorphism on $\langle P\rangle$, so a generic
  algorithm costs the same on every curve of the class; what could still
  differ (coordinates, field basis, automorphisms, Frobenius, implementation
  constants) is what the experiment controls or measures.
- **The tool** (§3) and **the torsion route** (§4): three routes to an
  $\ell$-kernel (Elkies via $\Phi_\ell$, division-polynomial factoring,
  $\ell$-torsion abscissae in $\mathbb{F}_{q^d}$), verified maps on points
  over $\mathbb{F}_p$ (Kohel) and in characteristic two (derived in the
  appendix), and the proposition that for odd $m$ every prime of the
  conductor of a curve over $\mathbb{F}_{2^m}$ is $\pm 1 \bmod 8$, which is
  why the Koblitz class over $\mathbb{F}_{2^{41}}$ needs isogenies of degree
  409 and 1721 (found in about one and five minutes).
- **Design** (§5): four classes (P1 prime, 347,004 curves; P0 $j=0$; B1
  binary, 327,080 curves; K Koblitz, 708,153 curves), 100 seeds per curve,
  Bonferroni per-curve band, Kruskal–Wallis and ANOVA across curves,
  negative controls (isomorphic and other-basis copies, A/A replica),
  planted effects (negation fold 0.707, 4-adding walk 1.137), structural
  folds as positive controls, a generator sweep (100 or 64 generators per
  curve, same secret) and a source×target shift matrix.
- **Results** (§6): 77,900 solves, every answer verified; 200 curves over
  all reachable endomorphism-ring strata; no curve, stratum, level, degree,
  kernel field, generator or source left the band and no heterogeneity
  test fired; every plant detected; the $j=0$ fold cost 0.439 [0.401, 0.481]
  (expected $1/\sqrt 6$), the Koblitz Frobenius fold 0.155 [0.141, 0.170]
  (expected $1/\sqrt{41}$) and 0.113 with negation. The only
  route-dependent cost end to end is finding the isogeny.
- **Reproducibility** (§8): the Rust tool, frozen protocol, command cards,
  per-solve rows and exact replays, all in this repository.

## What it does not say

- Nothing about index calculus, nothing sub-rho, nothing about any
  standardised curve; the instances are 33–40 bits, chosen so that whole
  classes could be measured, not for cryptographic size.
- Not a new attack and not a new speed-up: the crater folds it detects are
  the known automorphism and Frobenius equivalence classes.
- Not a proof that no isogeny-dependent structure exists at any size; it
  is a bounded, preregistered search that found none where the theorem
  says there should be none, with the sensitivity the plants measure
  (effects of 4–13 % on the mean detected at 100 seeds per curve).

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
   (2026-10-18) its only failures were the three author rules (anonymous
   author block, anonymous page 1, no email), and the abstract, title,
   keywords and page size passed. `eprint-abstract.txt` is the paste-ready
   abstract from that run.
3. **Novelty audit.** Not yet done. The claims most in need of a literature
   check: that this is the first *measured* whole-class rho comparison with
   explicit isogenies (the theorem itself is folklore; Jao–Miller–Venkatesan
   is the random-reducibility statement); the $\pm 1 \bmod 8$ proposition
   (elementary, probably known in some form); the characteristic-two Vélu
   map on points (Vélu's formula is characteristic-free, but an explicit
   normalised point map for ordinary binary curves should be compared with
   the literature before being described as new).
4. **Measurements still owed.** None for the claims made. Optional
   strengthening: a second independent host for the wall-time rows of §6.4,
   and a larger-$n$ replica (45–50 bits) of one class to show the null is
   not a small-$n$ artefact.
5. **Consent.** Every named author agrees to the irrevocable ePrint
   license; an author presses Submit, not an agent.

After acceptance, record the ePrint number and the password holder here.
