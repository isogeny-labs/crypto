# BLINDSPOT-19 protocol: negative-results paper and Lean statements

Frozen before the instrument is run. Source design: section "19" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18. Class: writing
and formalisation item. It adds no cross-method row and no speed claim, so the
section 5a pricing band (0, 0.1, 1 GAE) and the gap to matched rho are not
applicable to any row it produces; the one place the paper touches rho (the
lit2026 W3 audit) cites that study's own numbers and does not re-price them.

## Question

Can the six negative results named in the design each be stated in a paper
(`papers/ic-negative-results/`, LaTeX, IACR format) with a proof or an exact
link to committed evidence, at the evidence tier the repository's records
actually support, and can the two combinatorial cores (Theorem 15.1 and the
yield-conservation identity) be stated as Lean theorems?

## The six statements, each with the tier this item will claim

| # | design wording | evidence found on disk (checked before writing) | tier claimed |
|:--|:--|:--|:--|
| S1 | compact-orbit Koblitz IC is generic and sits on or above `S*T^2 >~ r/\|Aut\|` | `research/lit2026_w3_ggm_audit_20261011/` (report.md, table.json: 14 compact-orbit rows n37-n83; 208 rows, 0 below any bound) | measured record, 14 rows; the `S*T^2` line only. The non-adaptive `S*T` line is stated at the rank-stage mean only, because single frozen targets at n37-n53, n71, n73 fall below it (the audit's own table) |
| S2 | isogeny oracles add nothing in the generic model (fibers note Theorem 15.1) | `RESEARCH_ISOGENY_FIBERS_ECDLP.md` section 15.1 (proof by Shoup's argument) | proof, reproduced; Lean core (agreement lemma, union bound) written; full oracle-model theorem stays an open target |
| S3 | mean decomposition yield is conserved under base geometry | `crypto-autoresearcher:knowledge/findings/KN-FIND-007.md` (derivation, 144 cells, deviation exactly 0); `research/factor_base_designs_20261015/lean/FactorBaseDesigns/Yield.lean` (list form, kernel-checked in CI); `research/cyclotomic_falsifier_20261015/` (yield-neutral, p >= 0.363) | proof (one double count); Lean Finset form written; measured corroboration cited |
| S4 | the Semaev cover monodromy is `C_2^{m-2}` | `crypto-autoresearcher:knowledge/findings/KN-FIND-a8990a.md`, `papers/semaev-conservation-specialization/paper.tex`, `experiments/EXP-SMON-e5cbe6/runs/RUN-SMON-e5cbe6-001/raw-result.json` | derivation with toy-scale verification; the proof has NOT had an independent review by a function-field reader (the source paper's own blocker 1), and the paper says so |
| S5 | Nagao's Lemma 4 is false | `crypto-autoresearcher:knowledge/findings/KN-FIND-936151.md`, `papers/nagao-lemma4-note/note.md`; this item re-derives the witnesses natively in Rust | exact finite arithmetic over `F_2`; stated for the equality reading of condition (1) of Definitions 5 and 6 only; says nothing about Proposition 5 |
| S6 | Semaev 2015 Assumption 1 fails off the diagonal | NO committed record establishes `d_F4 > 4` at any cell. Semaev 2015 section 4.5.1 itself reports regularity degree `> 4` off the diagonal with first fall degree still 4 (cited from the goal record). `crypto-autoresearcher:knowledge/findings/KN-FIND-9cba50.md` says on-diagonal evidence is consistent with the assumption and asserts nothing off it. `research/blindspots_20261018/07-blocked-f4-degree-cells/README.md` has one closure-degree `D > 4` draw at an off-diagonal cell | NOT ESTABLISHED as a negative result. The paper reports the three facts above and calls S6 open (Deviation 1) |

## Arms

There is no method comparison. The paper's "arms" are the six statements and
the two Lean modules. The native instrument (`examples/blindspots_19_checks.rs`,
std-only Rust) re-derives, by exact finite computation, the parts of S2, S3
and S5 that can be checked by machine here:

* `lemma4`: first fall degree `d_F` (true, with the field equations outside
  and inside the generating set) and `d'_F` (fake), over `F_2`, equality
  reading, by exact linear algebra, for the two published witnesses and for
  the exhaustive census of all unordered pairs of nonzero multilinear
  polynomials of degree at most 2 in three variables.
* `affine`: the Lean agreement lemma and union bound, checked exhaustively on
  `Z/p` for `p = 5, 7, 11`, plus the negative control `Z/8`.
* `yield`: the conservation identity on `Z/N`, `N = 11, 12, 13`, over every
  base of size `B = 3, 4`, `m = 3` multisets.

## Controls

* Negative control for `affine`: a composite modulus (`Z/8`, `Z/9`), where two
  distinct affine forms can agree at more than one point, so the Lean
  hypothesis (a field) is needed. The instrument must find such a pair.
* Positive control for `lemma4`: a system whose first fall degree is known by
  hand and where `d_F = d'_F` must hold: `f_1 = X`, `f_2 = X + 1` over
  `F_2[X]` (multipliers `1, 1` give the nonzero constant `1`, a fall at degree
  1, for both definitions, so `d_F = d'_F = 1`). The instrument must return
  exactly that. The two published witnesses are the detected effect.
* Mirror control for `yield`: a base with repeated sums (an arithmetic
  progression) must give the same total as a random base, and strictly lower
  coverage; this shows the instrument sees collisions while the total stays
  fixed.
* Lean: `tools/lean/selftest` already exercises the proof gate; the new
  project is checked by `tools/lean/proof-gate.sh` in CI (Deviation 4).

## Cells and unit

No measured cells. The unit is exact integer arithmetic (counts, ranks over
`F_2`). Wall time is not recorded. The campaign results quoted from
`research/blindspots_20261018/NN-*/README.md` are cited as those READMEs state
them; this item re-measures none of them and interprets none beyond the
sentence each README already contains.

## Decision rule

Each statement receives one label, assigned by the evidence column above and
not by judgement of the result:

* **PROVED** (proof in the paper, plus machine check where listed),
* **EXACT** (exact finite computation reproduced natively, scope stated),
* **RECORDED** (measured committed rows, cited by file and sha256),
* **DERIVED, UNREVIEWED** (proof written, no independent review),
* **NOT ESTABLISHED** (S6).

The paper's claim class is *accounting/relabelling of existing negatives*, not
an advance: AGENTS.md "Success criterion" counts relabelling as no
improvement, and the paper says so. No label is a speedup.

## Budget

Zero task cards, zero CPU-hours of measured runs. The native checks run in
under 10 seconds on one thread. Disk: no per-item target directory; the
instrument is compiled with `rustc` into the scratch directory and deleted.
Address-space cap for any run: `prlimit --as=4294967296`.

## Deviations from the design (never a loosening of a decision rule)

1. **S6 is not stated as a negative result.** The design says "Assumption 1
   fails off the diagonal". The repository records do not contain an
   `F4`-degree certificate above 4 at any cell, the program's own findings say
   the assumption is not refuted, and the only `D > 4` reading is one closure
   draw. The paper states the open boundary with its three facts instead.
2. **S1 is split.** The `S*T^2` line holds on all 14 compact-orbit rows. The
   non-adaptive `S*T` line holds at the rank-stage mean (ratios 1.053 to
   2.734) and does not hold for single frozen targets. The paper states both.
3. **S4 and S5 carry their own tiers** (derivation not independently
   reviewed; equality reading over `F_2` only). Lemma 4 is not claimed false
   for the `inside` placement beyond what the exhaustive native census and the
   two witnesses show; the source ledger's earlier "inside form holds" claim
   was corrected (CORR-20260921-942a62) and the paper cites the correction.
4. **Lean is written but not kernel-checked on this host.** The host has no
   Lean, no Mathlib cache and about 750 MB of free disk; the toolchain alone
   needs more. The new project reuses the pinned toolchain and Mathlib
   revision of `papers/rootfinding-decomposition/lean/` (`v4.34.1`). The
   CI proof gate (`.github/workflows/lean-proof-gate.yml`) is the first
   kernel check. Until it passes, the paper calls the modules "written, not
   yet kernel-checked" and never "machine-checked". Open targets (the full
   oracle-model theorem, the first-fall-degree definitions) are in
   `lean/OPEN_TARGETS.md`, not in compiled sources, because the gate fails any
   file containing `sorry`.
5. **No Conductor.** `conductor` is not installed on this host; the hooks warn
   and let edits through (CLAUDE.md). No task was reserved.
6. **LaTeX was installed for the build** (`texlive-latex-base`,
   `texlive-latex-recommended`, about 130 MB) to run `make check`, and purged
   afterwards to give the disk back.
7. **Item 17 is not cited.** It has raw logs but no README or verdict at the
   time of writing; the paper lists it as pending and cites no number from it.
   The older `research/cyclotomic_falsifier_20261015/` result carries the
   yield-neutral statement.
