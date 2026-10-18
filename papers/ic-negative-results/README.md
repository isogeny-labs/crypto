# Negative Results for Elliptic-Curve Index Calculus: Six Statements, the Proofs or Evidence Behind Them, and What They Do Not Say

## Verdict (assess stage, 2026-10-18)

**DELIVERED** (paper item). Level: none; no exponent, constant or primitive changes, and
by the success criterion a relabelling of existing negatives is not an improvement. No
cross-method row, so no gap to matched rho and no 0 / 0.1 / 1 GAE pricing applies. The
frozen decision rule in [`PROTOCOL.md`](PROTOCOL.md) gives S1 RECORDED, S2 PROVED (Lean
unchecked), S3 PROVED and EXACT, S4 DERIVED UNREVIEWED, S5 EXACT, S6 NOT ESTABLISHED.

Assessment re-ran `examples/blindspots_19_checks.rs` (sha256 `751277a8...8ec5`) and the
output matched `evidence/checks.txt` line for line (ALL CHECKS PASS). Numbers are exact
integer counts from one deterministic run, so there are no intervals: census of 7875
pairs (d_F > d'_F in 6138 outside / 4830 inside; d_F < d'_F in 0 / 224), witnesses A, B
(d_F 3, d'_F 2), witness C (3 outside, 2 inside, 2 fake). Controls passed: positive
(X, X+1), negative (Z/8, Z/9), yield mirror. Caveats: host is shared, L0 isolation, toy
scale; Lean not kernel-checked. Graph: `reports/lemma4_census.svg` / `.png` (source
`reports/plot_census.py`); report: `REPORT.pdf` (source `reports/report.typ`). No
`benchmarks/` copy: not a speed result. Lean gate: pending CI (`lean-proof-gate.yml`).

`ic_negative_results.tex` (11 pages, A4). **Status**: research draft written
2026-10-18 for blind-spot item 19 (`research/blindspots_20261018/EXPERIMENTS.md`,
section 19; protocol in [`PROTOCOL.md`](PROTOCOL.md)). Anonymized, not peer reviewed,
**not submitted**. It is a consolidation of existing negatives: by the programme's
success criterion a relabelling or accounting change is not an improvement, and the
paper claims none. Restore the authors with `make deanonymize` after creating an
`AUTHORS` file (see `../AUTHORS.example`); an author, not an agent, submits.

## What it says

| # | statement | label | evidence |
|:--|:--|:--|:--|
| S1 | compact-orbit Koblitz IC sits on or above `S*T^2 >~ r/|Aut|`; at the rank-stage mean on `S*T ~ r/|Aut|`; single frozen targets at n37, n41, n53, n71, n73 fall below the non-adaptive line | recorded (14 rows) | `research/lit2026_w3_ggm_audit_20261011/` |
| S2 | isogeny oracles add nothing in the generic group model (Thm 15.1, bound `(Q+2)^2/r + 1/r`); counting core in Lean | proved; Lean not yet kernel-checked | `RESEARCH_ISOGENY_FIBERS_ECDLP.md` s15.1 |
| S3 | mean decomposition yield `C(B+m-1,m)/N` for every base of size `B` | proved; exact; recorded | `KN-FIND-007`, cyclotomic falsifier, native check |
| S4 | monodromy of the Semaev cover is `(Z/2)^{m-2}` | derived, unreviewed | `KN-FIND-a8990a`, `EXP-SMON-e5cbe6` |
| S5 | Nagao 2015/984 Lemma 4 (`d_F <= d'_F`) is false over `F_2`, equality reading, both field-equation placements | exact (hand proof for witness A; native re-derivation of both) | `KN-FIND-936151`, `evidence/checks.txt` |
| S6 | Semaev 2015 Assumption 1 off the diagonal | **not established** (the design listed it as a negative; the records do not support that) | `KN-FIND-9cba50`, campaign item 07 |

Campaign results that bear on the statements (items 01, 02, 07, 10, 14) are in
section 8 of the paper, quoted from each item's own verdict.

## What it does not say

No speedup, no key recovery, no sub-rho or deployed-curve claim. S1 is a comparison in
probes and table entries, not a proof of genericity, and the structured generic-group
bound is vacuous at these rungs. S4 has had no independent proof review and its novelty
is not adjudicated. S5 says nothing about Proposition 5 of Nagao's paper. S6 is open.
The new Lean modules are written but **not yet kernel-checked** (no Lean toolchain on
the authoring host); the repository proof gate (`tools/lean/proof-gate.sh`,
`.github/workflows/lean-proof-gate.yml`) is the first check.

## Files

- `ic_negative_results.tex`, `Makefile`, `abstract.txt`, the built `ic_negative_results.pdf`.
- `PROTOCOL.md`: question, arms, controls, decision rule and the list of deviations from
  the frozen design (S6 demoted, S1 split, Lean unchecked, no Conductor).
- `EVIDENCE.md`, `evidence/ledger.list`, `evidence/ledger.full`, `evidence/ledger.tex`:
  every evidence file with its SHA-256; regenerate with `evidence/make_ledger.sh`.
- `evidence/checks.txt`: output of the native checks `examples/blindspots_19_checks.rs`
  (`lemma4`, `affine`, `yield`; std-only Rust, about 0.3 s).
- `lean/`: Lake project `ic_negative` (`IcNegative/AffineCore.lean`,
  `IcNegative/YieldConservation.lean`, `AxiomAudit.lean`), pinned to Lean `v4.34.1` and
  the Mathlib revision of `../rootfinding-decomposition/lean/`. `lean/OPEN_TARGETS.md`
  holds the statements that are not proved (O1 to O4), outside compiled sources because
  the proof gate fails any `sorry`.

## Building

```sh
make          # PDF (pdflatex, no bibtex: the bibliography is inline); rules are ../paper.mk
make check    # unresolved refs, LaTeX errors, overfull boxes, page count
```

Native checks:

```sh
rustc --edition 2021 -C opt-level=3 -o /tmp/b19 examples/blindspots_19_checks.rs   # from the repository root
prlimit --as=4294967296 /tmp/b19 all                                               # ends with ALL CHECKS PASS
```

## Owed before submission

- [ ] Authors, affiliation and email (never invented): `AUTHORS`, `make deanonymize`.
- [ ] `lean-proof-gate.yml` passes on the `lean/` project; fix any proof that fails and
      only then say "machine-checked" (the paper says "written, not yet kernel-checked").
- [ ] An independent reader of function fields on S4 (Artin's theorem for a quotient by a
      finite group; properness of the preimage of `E[2]` in characteristic 2).
- [ ] Re-read the sibling-repository evidence if its heads moved
      (`evidence/make_ledger.sh` rewrites `EVIDENCE.md`).
- [ ] Decide whether S6 stays in the paper as an open item or moves to a note.
- [ ] `python3 .agents/skills/iacr-eprint/scripts/eprint_form.py papers/ic-negative-results ...`
      after the author block is restored; it currently fails only on the anonymous author
      and the missing email, by design.
