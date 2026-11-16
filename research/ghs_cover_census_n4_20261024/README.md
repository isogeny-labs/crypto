# GHS cover census for `n = 4` (`E / F_{q^4}`, odd characteristic), by 2-torsion type

2026-10-24. **Structural stage diagnostic on toy fields.** It counts GHS-type
covers by Riemann-Hurwitz genus. No curve over `F_q` is constructed, no transfer
map is evaluated, no discrete logarithm is solved, and no speedup is claimed.
Nothing here is a statement about any deployed curve. Conductor was not installed
in this session and was skipped. Labels: **derived** (proved in the protocol),
**measured** (written by the Rust tool), **model** (asymptotic formula, not
measured).

Protocol: [PROTOCOL.md](PROTOCOL.md) (frozen before any run; **Amendment A1**
appended after the first `q = 3` run, see "What failed"). Tool:
[cover_census_n4.rs](cover_census_n4.rs) (dependency-free Rust, fork of
`../ghs_cover_census_n5_20261013/cover_census_n.rs`, which is untouched).
Records: [results/run-20261024/](results/run-20261024/). Graph:
[SVG](reports/run-20261024/n4_min_genus_by_2torsion.svg),
[PNG](reports/run-20261024/n4_min_genus_by_2torsion.png). PDF: [REPORT.pdf](REPORT.pdf).

## Question

`K = F_{q^4}`, `B = x(E[2]) U {inf}`; for every `F_q`-structure `tau` the
compositum of the four conjugate double covers has genus `g = 2^(m-2)(r-4)+1`
(`m` = `F_2`-rank of the four conjugate branch sets, `r` = size of their union).
Which genus is attainable, per 2-torsion type, and for odd `#E(K)` (the
prime-order-like case) in particular?

## Result in brief

- **Genus floors at `n = 4` (measured, exhaustive at `q = 3, 5`; derived
  theorems explain them): odd `#E(K)`: 13 if `j not in F_{q^2}`, else 37;
  `E(K)[2] = Z/2` (both `#E = 2 mod 4` and `4 | #E`): 25 on every curve;
  `E(K)[2] = (Z/2)^2`: 5 on most curves (26 of 28 at `q = 3`, 158 of 210 at
  `q = 5`), else 7 or 9.** The mod-4 refinement does not matter at `n = 4`.
- **Admissibility (derived).** A structure is a GHS descent to `F_q` only if
  the four conjugate elliptic quotients are distinct, i.e. the orbit of the
  branch set under Frobenius has length 4 (`m in {3, 4}`). Structures with a
  `sigma^2`-stable branch set (genus as low as 2, 4, 5) are recorded apart and
  excluded; they only descend `E` to `F_{q^2}`.
- **Two preregistered predictions failed** (both from errors in my proofs, found
  by the first run): `E(K)[2] = Z/2` floor 21 (observed and re-derived: 25), and
  odd-class minima in `{13, 25}` (observed and re-derived: `{13, 37}`, never 25).
  The `(Z/2)^2` prediction (class minimum 5) held. After amendment A1 there are
  0 violations of any derived statement in 1 256 (`q = 5`) + 166 (`q = 3`) + 23
  (`q = 7` sample) curve models.
- **Reproduction control passed**: the copied tool reproduces the committed
  `n = 3`, `q = 5, 7` results byte for byte (plus `n = 3, q = 3` and
  `n = 5, q = 3` extras), and agrees with the independent cocycle enumeration at
  `n = 4, q = 3`.
- **Model exponent** `2 - 2/g` against rho's `2` (powers of `q`): 1.600 (`g = 5`),
  1.846 (13), 1.920 (25), 1.946 (37). Gap to rho (`2/g`, model): 0.400, 0.154,
  0.080, 0.054. Constants (~`g!`) and descent costs are not included, so this
  is an exponent comparison, not a cost comparison, and the 2-isogeny / class
  minimum rows below are a different (isogeny-walk) scope.

## Genus floor per class (the main table)

`n` = curves (isomorphism classes over `K`; `q = 3, 5` every class, `q = 7` a
seeded sample of 12 plus 11 neighbour models). "Class min" = smallest per-curve
minimum. Cells show curves at each per-curve minimum genus (**measured**).

| class | prereg. prediction | corrected (derived, A1) | `q = 3` (n) | `q = 5` (n) | `q = 7` sample (n) | model exponent at class min |
| --- | --- | --- | --- | --- | --- | --- |
| `#E` odd | 13 iff `j not in F_{q^2}`; I predicted curve minima in {13, 25} | 13 iff `j not in F_{q^2}` else 37; never 25 | 13: 48, 37: 8 (56) | 13: 400, 37: 20 (420) | 13: 3 (3) | 1.846 (g = 13) |
| `#E = 2 mod 4` | 21 on every curve (**failed**) | 25 on every curve | 25: 42 (42) | 25: 314 (314) | 25: 3 (3), +3 neighbours 25 | 1.920 (g = 25) |
| `E(K)[2] = Z/2`, 4 divides `#E` | 21 on every curve (**failed**) | 25 on every curve | 25: 40 (40) | 25: 312 (312) | 25: 5 (5) | 1.920 (g = 25) |
| `E(K)[2] = (Z/2)^2` | class min 5, not every curve | odd, >= 5; 5 iff a special structure exists | 5: 26, 9: 2 (28) | 5: 158, 7: 48, 9: 4 (210) | 7: 1 (1); neighbour models: 5: 5, 7: 3 | 1.600 (g = 5) |

Exponent gap to matched rho (model, `2/g`): `g = 5`: 0.400; 13: 0.154; 25: 0.080;
37: 0.054. Rho at `n = 4` costs `q^2` on `E(K)`; every row is a genus-`g`
Jacobian over `F_q` with model `q^{2-2/g}`. For odd `#E(K)` the floor is 13
(exponent gap 0.154), the odd-order class a prime-order curve belongs to.

Odd class, by `j` (measured; derived identities hold on every curve): `j not in
F_{q^2}` on 48 of 56 curves at `q = 3` and 400 of 420 at `q = 5`; for those the
structure histogram is exactly `{13: 1, 25: q^3+q-1, 37: C1, 41: C2, 49: rest}`
(`q = 5`: 13: 1, 25: 129, 37: 19 500, 41: 65 000, 49: 1 949 870 of 2 034 500).
`j in F_{q^2}`: all `q^3+q` special structures are `m = 1` or degenerate, floor 37.
`Z/2` class: structure histogram exactly `{25: 2q^2 (Q-1)/(q^3-q), 29: 2(Q-1), 33,
37, 41, 49}`, e.g. `q = 5`: 25: 260, 29: 1 248, 33: 11 232, 37: 37 440, 41: 124 800,
49: 1 859 520, identical on all 626 `Z/2` curves.

## Predictions versus observed

| statement | status | note |
| --- | --- | --- |
| valid genus odd and >= 5; no valid genus 2, 3, 4 | held (0 violations) | derived L3 |
| no valid `m = 2`; valid `m in {3, 4}` | held | derived L1 |
| genus 5 only for full 2-torsion | held | derived L3(c) |
| `(Z/2)^2` class minimum 5, attained at `q = 3` and `q = 5`, not every curve | held | 26/28 and 158/210 |
| `Z/2` floor 21 on every curve | **failed** | proof error: `E(K)[2] = Z/2` has two `K`-rational branch points; corrected floor 25 (A1) |
| odd curve minima in {13, 25}, 25 for ~`(q^2+1)/q^4` | **failed** (support) | 25 never occurs; 37 occurs on 8/56 (`q = 3`), 20/420 (`q = 5`) = exactly the `j in F_{q^2}` curves; fraction vs model `(q^2+1)/q^4` 12% / 4.2%: observed 14.3% / 4.8% (model-consistent) |
| odd and `j not in F_{q^2}` gives 13 | held | derived |
| mod-4 refinement irrelevant for `Z/2` | held (after A1 the same floor and histogram) | |
| `m1`, `deg` zero on `Z/2` curves; `deg > 0` possible on `(Z/2)^2` | held | `m1 = deg = 0` on all 82 (`q = 3`) and 626 (`q = 5`) `Z/2` curves; `deg > 0` on 12 of 28 and 34 of 210 `(Z/2)^2` curves |

The corrected statements (A1) were derived after seeing `q = 3`; `q = 5` and
the `q = 7` sample are out of sample for them (0 violations).

## Reproduction control and checks (measured, byte comparisons)

See [reproduction-checks.txt](results/run-20261024/reproduction-checks.txt)
(output of [compare.sh](compare.sh)).

| check | result |
| --- | --- |
| R1: copied tool, `n = 3`, `q = 5` and `q = 7`, all classes, versus committed `R1-n3-q{5,7}.jsonl` (all `curve`, `group`, `mr`, `derived_checks` records; `field`/`totals` minus timing) | PASS, identical |
| extra: `n = 3`, `q = 3` versus committed pilot | PASS |
| extra: `n = 5`, `q = 3` versus committed panel D (transversal rewrite is a no-op for odd `n`) | PASS |
| R4: `n = 4`, `q = 3`, `cosets` versus `cocycles` (independent enumeration, 531 360 matrices tested) | PASS, 166 curve records identical except `mismatch` scope |
| genus formula versus sum over quadratic subcovers | 0 mismatches in every arm |

## What changed in the tool, and the theory (all derived, see PROTOCOL.md)

- Pairwise disjointness test up to `k <= n/2` (for `n = 4`: `k = 1, 2`;
  disjoint gives `m = 4, r = 16, g = 49`). Orbit length `d` of the branch set:
  `d = 1` is `m = 1`, `1 < d < n` is "degenerate" and excluded.
- Transversal for the non-free action of `PGL2(F_q)` on `F_{q^2} \ F_q` (stabiliser of
  order `q+1`); total coset count asserted equal `(Q^3-Q)/(q^3-q)`.
- `x^4 - 1 = (x+1)^4` over `F_2`, so `m in {1,2,3,4}`: `m = 2` iff `sigma^2`-stable
  and not `sigma`-stable; `m = 3` iff every point of the union has even
  multiplicity. By Kani-Rosen the conorm map from `Res_{K/F_q} E` has finite
  kernel iff the four conjugate branch sets are distinct.
- Degrees of `K`-points and `sigma`-orbit sizes: `d_K = 2` forces orbit size 8
  (so a `K`-quadratic pair can never be `sigma`-stable at `n = 4`, unlike `n = 3, 5`),
  `d_K = 3` allows 3, 6, 12. Orbit multiplicity counts give every genus in the
  tables above (`g = 17 + 4 r_rat` for `Z/2`; 13 / 25 / 37 / 41 / 49 for odd).

## What failed or deviated (recorded with reasons)

1. **First `n = 4`, `q = 3` run reported violations**: proof error in T-Z
   (two rational branch points, not one) and a check bug (`m = 1` counted as
   valid). Found immediately, recorded as Amendment A1 in PROTOCOL.md before
   `q = 5` and `q = 7` ran; the first-run output was deleted from the working tree (next item); the
   original survives in this PR's first commit as
   `results/run-20261024/R4-firstrun-n4-q3-cosets-WRONG-CHECKS.jsonl`.
2. **Files deleted mid-run.** At about 05:50 UTC every file in this study
   directory except `bin/` and the open `P2`/`P3` outputs was removed from the
   working tree. Cause, identified afterwards by the parent session: it had
   committed an interim snapshot of this directory to this PR's branch and then
   switched the shared checkout back to `main`, which removed the tracked files.
   The interim snapshot (this PR's first commit) holds the originals. Recovery: the Rust
   source was rebuilt by replaying my four edit scripts on the baseline file and
   **the rebuilt binary is byte-identical** (`cmp`) to the one that ran P2 and P3;
   PROTOCOL.md was rewritten to the same SHA-256 (`e04d1111...c9f11`) before
   A1 was re-appended; `compare.sh`, plotting scripts and all controls (R1, extras, R4)
   were regenerated and rerun after P2/P3 finished, with the new byte-comparisons
   reported above. P2 and P3 outputs survived and are the original runs. The
   `panels.log` timing log was lost; wall times below come from the tool's
   `totals` records.
3. **Stop rule not applied to P2.** P2 was estimated at about 18 min (one timed
   curve, 4 threads, 3.4 s per curve); it took 2 871.5 s wall (48 min) because
   another job on the shared 4-core host (load average 7.5) took about half the
   CPU. The records are only written at the end of a run, so a kill would have
   discarded 36 min of work; I let it finish and report this as a deviation. P3
   (sample of 12 plus 11 neighbours) took 795 s. Panels took 61 min wall in total (P2 48 + P3 13), controls about
   8 min (the `n = 5, q = 3` extra 374 s), inside the 2 h budget. The controls had
   also passed before the deletion (same outputs); the table reports the reruns.
4. **PDF engine.** `pandoc` is installed but no LaTeX/typst/wkhtmltopdf engine,
   so `pandoc -o x.pdf` does not work. REPORT.pdf is laid out from this README by
   `reports/build_pdf.py` (reportlab; text and recorded values only). PNG
   rendering also had no rsvg/cairo backend, so the PNG is `pdftoppm` of the
   reportlab-drawn PDF of the same graph (`reports/plot.py`).
5. The Rust source was patched by small Python text-replacement scripts (editing
   code, not computing results); every number comes from the Rust tool.

## Caveats

- **Exponent comparison is a model.** `2 - 2/g` is the asymptotic index-calculus
  exponent in `q` for a genus-`g` Jacobian over `F_q`; constants (~`g!`), the
  descent/decomposition cost on a non-hyperelliptic cover, and the cost of
  evaluating the transfer are not included. The model is applied as the brief
  framed it; whether it applies to these covers is not verified here.
- **Unpriced native work and the 0 / 0.1 / 1 GAE band (AGENTS.md 5a) is not
  computed**: this is a structural genus census with no operation counts, so
  no ranking of arms by cost is claimed.
- Admissibility needs `E(K)` odd order for injectivity of conorm-norm (the cover
  has degree `2^(m-1)`), as usual for GHS; for the full-2-torsion class the
  2-part is lost in the transfer.
- Sampled panels (`q = 7`) are sample estimates; no interval claimed. 12 curves
  is too few to say anything about fractions.
- "Class min" in the `2isog`/`class` scope (e.g. 4-divisible-order curves with `Z/2` reach 5 through an
  isogenous curve with full 2-torsion; `2 mod 4` classes stay at 25) concerns the
  isogeny class at equal order and ignores the cost of the isogeny.

## Files

`PROTOCOL.md`, `cover_census_n4.rs`, `compare.sh`, `run_controls.sh`,
`results/run-20261024/*.jsonl` (records: `field`, `curve`, `group`, `mr`,
`derived_checks`, `totals`), `timings.txt`, `reproduction-checks.txt`,
`reports/plot.py`, `reports/build_pdf.py`,
`reports/run-20261024/n4_min_genus_by_2torsion.{svg,png,pdf}`, `REPORT.pdf`.

![Minimal genus per curve by 2-torsion class, run 20261024](reports/run-20261024/n4_min_genus_by_2torsion.png)
