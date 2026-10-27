# `audit_curve` genus rule: R5.2's reported discrepancy was measured on a stale base (2026-10-24)

**Verdict (measured, this PR).** On `main`, `ec_trapdoor::audit_curve` and
`ghs_genus_with_type` already implement the GHS genus rule R5.2 derived
(`m = m' + [f(1) ≠ 0]`, `g = 2^(m−1) − [f(1) ≠ 0]`, `f` the σ-annihilator of
√b). No `src/` rule change is needed and none is made. The discrepancy
recorded in `research/isogeny_fibers_20261010/round5/r5_2_ghs_class/`
(`RESULTS.md` §0 and §2, `selftest.json` key `library_audit_curve_vs_ours_N12`)
compared R5.2's formula with the rule that was **removed on 2026-10-13**.

## Why R5.2 saw a discrepancy

| Event | Commit | Date |
|--|--|--|
| MMT correction of magic number and genus type | `8ea2296af4` (on `main`) | 2026-10-13 |
| Lemma 6 condition (8) in `audit_curve` | `212f92fb4f` (on `main`) | 2026-10-13 |
| R5.2 commit | `f494ee2a68`, parent `aa57d121c4` (old rule: `audit_curve` used `magic_number_full` with `a` and "type I iff orbit length == m") | 2026-10-18 (parent 2026-10-12) |
| R5.2 merged as PR #2181 | | 2026-10-24 |

R5.2's example linked `crypto_lib` at its own base, so `audit_curve` there was
the removed rule (plain orbit rank as `m`; type I iff the orbit length of √b
equals `m`). The rows R5.2 lists reproduce that rule exactly, not the current
one:

* `lib(m=1,g=0)` for b in F_2: orbit length 1 equals `m` = 1, so "type I", `2^0 − 1 = 0`.
* `lib(m=4,g=7)` at n = 4: plain rank 4 with orbit length 4 gives `2^3 − 1`.
  Impossible, since `x^4 + 1 = (x + 1)^4` forces type II.
* `lib(m=2,g=2) ours(m=3,g=3)`: √b = c + σ(c) at (l, n) = (4, 3) has plain rank
  2 and is type I; the removed rule has no `+1`.

The current `audit_curve` is not the code R5.2 ran. R5.2's own claim that
`magic_number` "agrees 1 500/1 500" is also an artefact: at its base,
`magic_number` was the plain rank, which equals R5.2's `m'`; on `main`
`magic_number` is the MMT rank, equal to R5.2's `m`.

## What this PR adds

Three tests in `src/cryptanalysis/ec_trapdoor.rs` (module `tests`), with a
reference implementation written independently of `mmt_ranks` (first
F_2-dependency among `σ^i(√b)`, giving `f`, then R5.2's formula):

* `audit_curve_genus_matches_annihilator_reference_n12`: all 4 095 b, all five
  factorisations of 12, `a` of trace 0 and trace 1 (40 950 rows). Agreement on
  `(m, genus, type)` in every row; never genus 0; never genus 7 at n = 4. The
  removed rule is evaluated on the same rows and must disagree with the
  reference on more than half of them.
* `audit_curve_r52_hand_checked_cases_n12`: R5.2's hand-checked cases
  (b = 1; √b = c + σ(c) at (4, 3); generic (4, 3), (6, 2), (3, 4)).
* `audit_curve_textbook_n5_over_f2_155`: the Gaudry–Hess–Smart / Menezes–Qu
  `n = 5` case over F_{2^155} (Oakley Group 3 field, l = 31): generic b has
  m = 5, genus 16; √b = c + σ(c) has m = 5, genus 15, type I (literature value
  recalled, and equal to the value derived from `f`).

They pass on `main` unchanged; they could not be made to fail first because
the rule was already fixed. They fail against the removed rule (the first test
asserts it disagrees on most rows). Also fixed: the module header said the
Lemma 6 applicability check was "not checked here", which stopped being true
with `DescentRow::descends`.

## Committed outputs and claims that depend on the removed rule

Nothing is overwritten. None of these changes in this PR.

| Path | Date | What it holds | Status |
|--|--|--|--|
| `docs/ghs-transport-evidence/binary192-screen.json` | 2026-10-11 | 13 towers, `m = 1`, genus `0`, type I | Frozen; correct values are genus 1, type II. Already listed in `docs/audits/GHS_MAGIC_NUMBER_MMT_CORRECTION_20261013.md` §4. All rows stay inside the genus-4 bound. |
| `docs/ghs-transport-evidence/sect113r1-screen.json` | 2026-10-11 | `m = 113`, type I, genus `2^112 − 1` | Frozen; correct is type II, genus `2^112`. Listed in the 2026-10-13 audit §3. No row within the bound either way. |
| `research/ghs_curve_comparison_20261009/structural-standard/sect113r1.screen.json`, `research/sect113r1-weak-curve-20261006/addenda/ghs-three-node-20261007/` | 2026-10-06..09 | Same sect113r1 screen, hash-pinned | Frozen; same correction. |
| `research/isogeny_fibers_20261010/round5/r5_2_ghs_class/{RESULTS.md §0,§2, selftest.json, report.typ}` | 2026-10-18 | The "library discrepancy" and its row table | Frozen; superseded by this note. R5.2's census and walk use their own implementation and are unaffected. |
| `docs/curves/jacobian-transfers.json`, `docs/curves/JACOBIAN_TRANSFERS.md` | 2026-10-18 | `audit_curve_magic_number` per route | Regenerated after the fix; no change. |
| `docs/GHS_SCREEN.md` | 2026-10-13 | Screen documentation | Updated with the fix; no change. |

Not audited line by line: the `website/weakcurves/dist/*.json` snapshots (dated
2026-10-19, after the fix) carry `ghs_magic_number` and `ghs_genus_formula`
fields; they were not regenerated here and not compared with `audit_curve`.

Conductor was not installed in this container (`conductor check` is not on
PATH), so no scope reservation was taken.
