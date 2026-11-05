# Root finders against F4, F5 and the F6 analogue on prime-field curves: result

- **Protocol:** [`PROTOCOL.md`](PROTOCOL.md), committed before any
  measured run.
- **Amendment:** [`AMENDMENT_1.md`](AMENDMENT_1.md), which adds the
  AGENTS.md §5a pricing band and records the P8 rerun.
- **Analysis:** [`ANALYSIS.json`](ANALYSIS.json), written by
  [`examples/pkm_rootfind_analyze.rs`](../../examples/pkm_rootfind_analyze.rs)
  from the records alone.
- **Run report:** [`reports/run-20261018T2220Z/`](reports/run-20261018T2220Z/REPORT.pdf).
- **Earlier round on the same curves:**
  [`research/pkm_tower_ecbench_20261011/`](../pkm_tower_ecbench_20261011/RESULT.md).

## Answer in one paragraph

The repository has two root finders, `rootfind-subtract` and
`rootfind-mitm` (from PRs #1765, #1771 and #1889). On one Kummer base and
the same public targets they were run against tower F4 and signature F5 on
four prime-order curves of 9–15 bits:

| comparison | root finder / F4 or F5 | what it shows |
| --- | --- | --- |
| `S` | 1.16–16.9 | F4/F5's `S` is a lower bound, so this decides nothing |
| `F_p` multiplications (F4/F5 multiply-adds included) | about 10⁻⁷–10⁻⁴ | root finder far lower |
| `F_p` inversions | 1.15–17 | F4/F5 lower, on every target |
| relation-phase wall time | 10⁻⁷–10⁻⁴ | root finder far faster |

**Under the §5a band the order of the two flips.**
- At weight 0 the root finders are above F4 and F5 on every target.
- At 0.1 and at 1 GAE per unpriced operation they are below on every
  target.
- §5a therefore reports **"unresolved inside the band"** in every cell.

**The break-even price** (derived) is 10⁻⁸–10⁻⁵ GAE per unpriced
operation, falling with size. The root finders are cheaper unless a
tower-F4 multiply-add costs less than about a millionth of a group
addition.

## What ran

| spec | session | curve | targets | arms | verified | replay audit |
| --- | --- | --- | --: | --: | --: | --- |
| P5 | `ECBS1h2306d857d0af` | `icv1-fp9-t25-33fa1ca2` | 8 | 9 | 72 / 72 | [ok](audits/P5-replay-all.json) |
| P6 | `ECBS1hf6c3a80d1387` | `icv1-fp11-t43-4d73e7f7` | 8 | 9 | 72 / 72 | [ok](audits/P6-replay-all.json) |
| P7 | `ECBS1hede9211000ad` | `icv1-fp13-tm41-756f569a` | 8 | 9 | 72 / 72 | [ok](audits/P7-replay-all.json) |
| P8 | `ECBS1h3c465cff7dc5` | `icv1-fp15-t181-50ff72d2` | 4 | 9 | 36 / 36 | [ok](audits/P8-replay-all.json) |

- **Base.** Every IC arm ran on `prime-kummer:t=T,g=G`, the Kummer base
  on `main`, with `G` = 37, 684, 7047 and 2551. That is the earlier
  round's base point for point (`points_sha256` checked in all four cells).
- **Targets.** The earlier round's target seed, so the targets are the
  same.
- **Code.** Commit `ebed0cc51` and one binary.
- **Host.** A cloud VM, isolation L1 or L2 as recorded per run.
- **P8.** A worker restart killed it after 20 of 36 runs, and it was rerun
  in full. The interrupted session is kept in `interrupted/`.

**Instrument check (Q4): passed.** `f4`, `sig-f5` and `f6` reproduce the
2026-10-11 round's counts on every target: 28 of 28, in solver
multiply-adds, nodes, trials, relations and `S`.

## Results

`M` and the inversions are derived from recorded counts with the curve's
fixed per-operation costs: an affine addition is 2 multiplications,
1 squaring and 1 inversion; a doubling is 5, 2 and 1. For F4 and F5, `M`
includes the solver's multiply-adds. Medians over 8 targets, 4 at P8.

| cell | arm | S | M (F_p mult.) | inversions | relation wall |
| --- | --- | --: | --: | --: | --: |
| P5 (9-bit) | tower F4 | 25.84 | 8.05 × 10⁷ | 394.5 | 0.52 s |
| | signature F5 | 25.84 | 1.82 × 10⁷ | 394.5 | 0.28 s |
| | `rootfind-subtract` | 29.02 | 2,190 | 454.5 | 69 µs |
| | `rootfind-mitm` | 41.53 | 2,762 | 634 | 59 µs |
| P6 (11-bit) | tower F4 | 15.55 | 2.35 × 10⁹ | 575 | 5.80 s |
| | signature F5 | 15.55 | 6.51 × 10⁸ | 575 | 5.52 s |
| | `rootfind-subtract` | 36.03 | 5,200 | 1,358 | 124 µs |
| | `rootfind-mitm` | 35.74 | 5,192 | 1,321.5 | 79 µs |
| P7 (13-bit) | tower F4 | 9.36 | 4.07 × 10¹⁰ | 823 | 55.7 s |
| | signature F5 | 9.36 | 1.25 × 10¹⁰ | 823 | 80.4 s |
| | `rootfind-subtract` | 35.72 | 11,039 | 3,179.5 | 250 µs |
| | `rootfind-mitm` | 58.08 | 16,881.5 | 5,104.5 | 116 µs |
| P8 (15-bit) | tower F4 | 7.48 | 4.86 × 10¹¹ | 1,011 | 410 s |
| | signature F5 | 7.48 | 1.59 × 10¹¹ | 1,011 | 848 s |
| | `rootfind-subtract` | 49.97 | 22,028.5 | 6,819.5 | 487 µs |
| | `rootfind-mitm` | 126.25 | 52,946.5 | 17,057.5 | 158 µs |

The F6 analogue and `enum` are in `ANALYSIS.json`. F6 matches F4 to
within 1 %, as in the earlier round. The `m = 3` context arms,
`rootfind-mitm:m=3` and `rootfind-pairs:m=3`, cost more than the `m = 2`
root finders in every cell.

### The protocol's questions against F4 and F5

**Q1. `S(root)/S(F4)` and `S(root)/S(F5)`** (the two are identical, since
F4 and F5 have the same `S`).

| cell | `rootfind-subtract` | `rootfind-mitm` | rule |
| --- | --- | --- | --- |
| P5 | 1.155 [1.07, 1.31] | 1.616 [1.50, 1.76] | **not decided by S** |
| P6 | 2.230 [2.08, 2.93] | 2.288 [2.13, 2.44] | **not decided by S** |
| P7 | 3.529 [1.98, 7.05] | 6.239 [5.48, 7.78] | **not decided by S** |
| P8 | 6.704 [5.01, 8.24] | 16.92 [16.2, 17.7] | **not decided by S** |

The root finders' `S` is above F4's lower bound on every target. A lower
bound cannot show F4 cheaper, so `S` decides nothing.

**Q2. `M(root)/M(F4)` and `M(root)/M(F5)`.**

| cell | subtract / F4 | mitm / F4 | subtract / F5 | mitm / F5 | targets where root inversions ≤ F4's | rule |
| --- | --: | --: | --: | --: | --: | --- |
| P5 | 2.8 × 10⁻⁵ | 3.5 × 10⁻⁵ | 1.2 × 10⁻⁴ | 1.5 × 10⁻⁴ | 0 of 8 | **mixed** |
| P6 | 2.3 × 10⁻⁶ | 2.1 × 10⁻⁶ | 8.3 × 10⁻⁶ | 7.7 × 10⁻⁶ | 0 of 8 | **mixed** |
| P7 | 3.3 × 10⁻⁷ | 4.2 × 10⁻⁷ | 1.1 × 10⁻⁶ | 1.4 × 10⁻⁶ | 0 of 8 | **mixed** |
| P8 | 4.6 × 10⁻⁸ | 1.1 × 10⁻⁷ | 1.4 × 10⁻⁷ | 3.4 × 10⁻⁷ | 0 of 4 | **mixed** |

- **Multiplications.** The root finders use 10⁻⁷–10⁻⁴ of F4's and F5's on
  every target.
- **Inversions.** They also use 1.15–17 times as many inversions, on
  every target: one per affine addition, against F4's few hundred to a
  thousand.
- **Verdict.** The rule needs both measures to favour the root finder, so
  it reads "mixed". For F4 to be the cheaper, one inversion would have to
  cost more than about 1.3 × 10⁶ multiplications at P5, rising to
  8.4 × 10⁷ at P8. That is F4's extra multiplications divided by
  `rootfind-subtract`'s extra inversions, at the medians. A Fermat
  inversion in these fields costs about 1.5·log₂ p ≈ 14–23
  multiplications.

**§5a band** (AGENTS.md; added by Amendment 1). `S_w` = (GAE + w ×
unpriced operations)/√r, with every arm's unpriced counters listed in
`ANALYSIS.json`:

| cell | root / F4 at w = 0 | at w = 0.1 | at w = 1 | verdict | derived break-even w*, median [range] |
| --- | --: | --: | --: | --- | --- |
| P5 | 1.16 / 1.62 | 6 × 10⁻⁵ / 8 × 10⁻⁵ | 9 × 10⁻⁶ / 1 × 10⁻⁵ | **unresolved inside the band** | 8.5 × 10⁻⁷ [2.7 × 10⁻⁷, 1.9 × 10⁻⁶] |
| P6 | 2.23 / 2.29 | 7 × 10⁻⁶ / 6 × 10⁻⁶ | 1 × 10⁻⁶ / 7 × 10⁻⁷ | **unresolved inside the band** | 3.6 × 10⁻⁷ [2.5 × 10⁻⁷, 4.1 × 10⁻⁷] |
| P7 | 3.53 / 6.24 | 1 × 10⁻⁶ / 1 × 10⁻⁶ | 2 × 10⁻⁷ / 1 × 10⁻⁷ | **unresolved inside the band** | 6.9 × 10⁻⁸ [4.6 × 10⁻⁸, 1.3 × 10⁻⁷] |
| P8 | 6.70 / 16.9 | 2 × 10⁻⁷ / 4 × 10⁻⁷ | 3 × 10⁻⁸ / 4 × 10⁻⁸ | **unresolved inside the band** | 1.2 × 10⁻⁸ [7.3 × 10⁻⁹, 1.6 × 10⁻⁸] |

- **Reading the table.** The first value in each cell is
  `rootfind-subtract`, the second `rootfind-mitm`. The break-even column
  is for `rootfind-subtract` against F4.
- **Against signature F5.** The break-even weights are 3–4.4 times larger,
  4 × 10⁻⁸ to 4 × 10⁻⁶. The verdicts are the same.
- **What the band shows.** At w = 0, where F4's multiply-adds are free,
  F4 is cheaper. At either positive weight the root finders are cheaper by
  4–8 orders of magnitude.
- **What "unresolved" means here.** §5a does not rank an order that flips
  inside the band. The flip happens at a weight 4–7 orders of magnitude
  below the band's next point, 0.1.

**Q3. F6 analogue and `enum`.**
- The root finders sit beside `enum` (the no-algebra closure), and are of
  the same order in `M`. In `S`, the medians of the per-target ratios are:
  - `rootfind-subtract`: 0.85–1.04 times `enum`;
  - `rootfind-mitm`: 0.82–2.38 times `enum`.
- Against F6 the picture is the one against F4, since F6's work is F4's.

**Q5. Context.**
- **Against rho.** Every arm stays above same-target rho-negation:
  - `rootfind-subtract` 5.4–29.2×;
  - `rootfind-mitm` 7.6–65.7×;
  - F4 3.9–5.4× (lower bound).
- **The `m = 3` root finders.** `rootfind-mitm:m=3` costs 1.29–1.90 times
  `rootfind-mitm:m=2` in `S`, and `rootfind-pairs:m=3` costs more still.

## What this shows and does not show

- **The root finders, measured.** They decompose with far less field
  arithmetic and wall time than tower F4 or signature F5 on these curves.
  F4 and F5 spend 10⁷–10¹¹ multiply-adds per run on systems a lookup
  settles directly.
- **The ranking under §5a.** The repository's ranking rule calls the
  order unresolved, because the unpriced-work band is wide enough to
  contain the free-multiply-add case. That is reported as such, and not
  as a win.
- **Not shown.**
  - No arm is below rho.
  - No exponent: four sizes up to 15 bits, and 4 targets at P8.
  - Two-point decompositions only, against the Gröbner engines.
  - The `m = 3` root finders are context.
