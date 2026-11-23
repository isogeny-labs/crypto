# `inherited-f4-next`: the same counted work with faster kernels, and `fes-f2` end to end (2026-11-12)

Owner's ask (2026-11-12): "Speed it all up". The M4GB side is
[`../m4gb_update_signatures_20261112/`](../m4gb_update_signatures_20261112/).
This study covers the in-house engines that the algebraic index-calculus
oracle actually runs.

## Correction to the brief

The change was briefed as "`inherited-f4` is the best engine on every cell".
That holds for end-to-end S in the e2e ladder
([`../m4gb_e2e_ic_ladder_20261110/`](../m4gb_e2e_ic_ladder_20261110/)), which
prices `inherited-f4` from its word-XOR count, and in that ladder `fes-f2` was
not an arm. On wall time per call, `fes-f2` (Gray-code exhaustive search) is
below `inherited-f4` up to 24 unknowns, here and already in the descent
ladder (`../m4gb_descent_ladder_20261110/RESULTS.md`, R 21: 3.9 against
9.5 ms) `[pcc:unverified]`. `fes-f2` is also priced in word XORs (two per
Gray-code point), so its S is on the same basis as `inherited-f4`'s; at 14
unknowns its count per call is below `inherited-f4`'s (about 33 thousand
against 49 thousand in the profiling runs), from 20 unknowns up it is 4 to
13× above. The end-to-end run below measures what that does to S.

## Profile (Callgrind, `inherited-f4`, AVX2 path)

`gf2_elim` chooses AVX-512 at run time on this host; Valgrind cannot decode
it, so the profile is of the AVX2 path (`KIC_GF2_FORCE_AVX2=1`), instruction
counts with cold caches `[pcc:unverified]`:

| inclusive share | K 23:12:2 | R 21:11:2 | K 13:7:2 |
|:--|--:|--:|--:|
| tree specialisation (`specialise_shared`) | 64 % | 59 % | 55 % |
| ↳ eager rewrite of kept rows (`apply_step_bmi2`) | 20 % | 16 % | 11 % |
| ↳ re-reduction of displaced rows (`ReducedBasis::insert`) | 16 % | 13 % | 14 % |
| root (`reduce_inherited`) | 22 % | 29 % | 31 % |
| ↳ Macaulay build | 12 % | 19 % | 20 % |
| ↳ root echelon (`eliminate_core`) | 9 % | 9 % | 9 % |
| system substitution | 9 % | 5 % | 5 % |

The structure-reuse-across-targets question ("dynamic programming"): the
column layout is already cached across targets (keyed by the occurring
variables and the degree); what a root still does per target is numeric, and
55 to 64 % of the cost is the specialisation down the tree, which is counted
work. A complete-layout formula index was tried and gave nothing measurable;
the existing Gröbner-trace option (`KIC_F4_ROOT_REPLAY`) drops rows and can
change the tree, so it does not meet the identical-decisions rule.

## What changed (engine `inherited-f4-next`; `inherited-f4` unchanged)

A per-thread switch (`koblitz_groebner::with_next_kernels`) selects, for the
same eliminations and rewrites:

1. SIMD row XOR in `ReducedBasis::insert` (AVX-512 or AVX2 at run time);
2. the `bitop/v1` tally counted inside the rewrite pass instead of a second
   pass over the same words (identical tally);
3. Four Russians table reuse (`gf2_elim::set_reuse_table_thread`, the exact
   `KIC_GF2_REUSE_TABLE` behaviour);
4. existing exact matrix builders that were off by default (direct fused
   pack, support-local streaming and bitmap columns), whose tests assert
   identical matrices;
5. environment variables read once instead of per node.

None of these changes the engine's `ops` (word XORs): the same work, done
cheaper. Tests: `koblitz_groebner::tests::next_kernels_leave_decisions_and_word_xors_unchanged`
(33 fixed systems, cold and warm, identical roots, tree statistics and word
count), `gf2_elim::tests::reuse_table_override_leaves_rows_and_count_unchanged`,
`inherited_f4::tests::simd_row_xor_matches_the_scalar_one`; module suites
`koblitz_groebner` (92), `gf2_elim` (12), `inherited_f4` (14),
`ic_framework::solvers` (13), `pq_f4_f2` (13) pass. Written by a subagent to a
frozen brief; reviewed in the main session.

## Run 1: paired per call (`run_ab.sh`)

`ic descent --solver inherited-f4 --solver inherited-f4-next --solver fes-f2`,
Koblitz and random cells, m = 2 at 14 to 24 unknowns plus Koblitz 30 and 32
(degrees 25 and 27 are composite and have no Koblitz cell), 6 seeded targets ×
3 repeats, 60 s per call, one thread; engine order rotates per repetition.
Record `20261112T0630Z-inherited-f4-next-ab`.

Expectation: next/old 0.85 to 0.95 per call on every cell, `ops` equal on
every target, 0 disagreements; `fes-f2` below both on wall up to 24 unknowns
and above them at 30 to 32. Falsifier: any `ops` difference or disagreement,
or a median paired ratio at or above 1.

## Run 2: end to end (`run_e2e.sh`)

`ic bench --sweep` on the six instances of the e2e ladder (Koblitz 13, 17,
19, 23; random 15, 21; same subspace bases, 2 repeats, 60 s per call, 16
matched rho runs), the algebraic oracle with `inherited-f4`,
`inherited-f4-next` and `fes-f2`, the pair table as reference row.

Expectation, stated before the run: `inherited-f4` and `inherited-f4-next`
have the same S on every instance (same count, pinned pricing) and the
second has the lower wall; `fes-f2` has a lower S than `inherited-f4` at
degree 13 (14 unknowns) and a higher S from degree 17 or 19 up. If the
first holds, it is a measured gain `[pcc:unverified]` on the best algebraic row at
degree 13 and nothing else. Falsifier: `inherited-f4` and `-next` S
differing beyond run noise, or `fes-f2` S at degree 13 not below
`inherited-f4`'s.

## Results

### Run 1 `[pcc:unverified]`

2026-11-12 06:30 to 06:32 Z, record
`benchmarks/records/20261112T0630Z-inherited-f4-next-ab.json` (measured; raw
`results.tar.gz`, `results_ab.json`). Full table `RESULTS_AB.md`, graph
`results_ab.svg/png`.

| unknowns | `inherited-f4` ms K / R | `-next` ms K / R | paired next/old | `fes-f2` ms K / R | word XORs `inherited-f4` / `fes-f2` (K) |
|--:|--:|--:|--:|--:|--:|
| 14 | 1.03 / 1.19 | 0.94 / 1.13 | 0.92 / 0.93 | 0.09 / 0.10 | 49 k / 33 k |
| 16 | 2.40 / 1.30 | 2.22 / 1.18 | 0.93 / 0.91 | 0.19 / 0.12 | 124 k / 132 k |
| 18 | 2.21 / 2.22 | 2.08 / 2.04 | 0.94 / 0.92 | 0.24 / 0.25 | 251 k / 525 k |
| 20 | 4.25 / 5.58 | 3.65 / 5.11 | 0.86 / 0.89 | 0.82 / 1.24 | 528 k / 2.10 M |
| 22 | — / 7.23 | — / 6.38 | 0.88 | — / 2.77 | 1.06 M / 8.39 M (R) |
| 24 | 15.7 / 14.3 | 13.7 / 12.5 | 0.89 / 0.89 | 10.7 / 11.3 | 2.50 M / 33.6 M |
| 30 (K) | 90.5 | 81.7 | 0.89 | 705 | 20.0 M / 2.15 G |
| 32 (K) | 217 | 192 | 0.87 | 3,036 | 42.9 M / 8.59 G |

- `inherited-f4-next` is 0.86 to 0.94× of `inherited-f4` per call on every
  cell, below it on 74 of 78 targets, with the same per-target `ops` on every
  target and repeat and 0 disagreements with `fes-f2`. The expectation held;
  the falsifier did not fire.
- `fes-f2` is below both inherited engines on wall up to 24 unknowns (0.08 to
  0.11× at 14 to 18, 0.7 to 0.8× at 24) and 7.8 and 14× above at 30 and 32.
  Its word-XOR count is below `inherited-f4`'s only at 14 unknowns.
- The `ops` are unchanged, so the S of an `inherited-f4` row priced from its
  pinned word-XOR count does not move; the change lowers wall time only.

### Run 2 `[pcc:unverified]`

2026-11-12 06:33 to 06:36 Z, record
`benchmarks/records/20261112T0633Z-inherited-f4-next-e2e.json` (measured;
raw `results_e2e.tar.gz`, `results_e2e.json`). Full table `RESULTS_E2E.md`,
graph `results_e2e.svg/png`. Every one of the 24 configurations recovered its
planted logarithm in every repeat, and so did every matched rho run.

| instance (unknowns) | rho S | pair table S | `inherited-f4` S | `-next` S | `fes-f2` S | ms per call `inherited-f4` / `-next` / `fes-f2` |
|:--|--:|--:|--:|--:|--:|:--|
| Koblitz 13 (14) | 2.76 | 197.7 | 123.1 (117.8–128.4) | 123.1 | **94.7 (83.7–105.7)** | 0.76 / 0.74 / 0.06 |
| random 15 (16) | 2.64 | 474.0 | 357.5 | 357.5 | 426.5 | 1.91 / 1.41 / 0.18 |
| Koblitz 17 (18) | 2.02 | 493.0 | 249.1 | 249.1 | 494.0 | 2.80 / 2.78 / 0.30 |
| Koblitz 19 (20) | 1.48 | 1,063.2 | 411.0 | 411.0 | 2,548.0 | 4.57 / 4.12 / 0.73 |
| random 21 (22) | 1.36 | 1,975.8 | 2,758.3 | 2,758.3 | 21,807.9 | 8.83 / 7.70 / 3.13 |
| Koblitz 23 (24) | 0.92 | 3,986.2 | 1,852.5 | 1,852.5 | 18,444.0 | 16.14 / 14.82 / 12.00 |

S = total group-addition equivalents / sqrt(r), mean over 2 repeats (range
for degree 13). Every algebraic row is priced from its solver's word-XOR
count (`pinned` on the Koblitz instances, `measured` ratio on the random
ones), the same basis for all three engines.

- **Degree 13: `fes-f2` lowers the best index-calculus row from 123.1 to
  94.7 (0.77×), 34.3× the matched rho instead of 44.6×.** The two repeats of
  each engine do not overlap (83.7–105.7 against 117.8–128.4). At 14
  unknowns, two word XORs per Gray-code point over 2^14 points cost less than
  the inherited tree's eliminations. This is the expectation stated before
  the run; the falsifier did not fire. The distance remaining to rho at
  degree 13 is 34.3×.
- **From 16 unknowns up `fes-f2` has the higher S** (1.2× at random 15, 2×
  at Koblitz 17, 6 to 10× from 19 up): its count doubles per unknown while
  the inherited tree's grows far more slowly. `inherited-f4` stays the best
  algebraic row there, and the pair table stays best on random 21.
- **`inherited-f4-next` has exactly the S of `inherited-f4` on every
  instance** (same counted work) and a lower wall per call inside the whole
  pipeline: 0.74 at random 15, 0.87 at random 21, 0.90 at Koblitz 19, 0.92
  at 23, 0.97 to 0.99 at 13 and 17.
- `fes-f2` tried 47 targets for 31 relations at degree 13 against 42 for 28
  with the inherited engines (hit rate 0.657 against 0.654); S counts all of
  them.

Not a leaderboard row: the row has no replay certificate or validator
report (validation gate), the run is local and unisolated, and 2 repeats is
a small sample. The next step for degree 13 is a validated isolab run of the
`fes-f2` configuration; a size rule in the oracle (`fes-f2` up to 14
unknowns, `inherited-f4` above) would carry the gain into any sweep without
choosing the engine per instance.

## Sources checked

KB (`research-kb`) unavailable in this session. `docs/ic/FRAMEWORK.md`
(engine table: `fes-f2` and `inherited-f4` both priced in word XORs),
`research/m4gb_e2e_ic_ladder_20261110/README.md` and its sweeps,
`research/m4gb_descent_ladder_20261110/RESULTS.md`,
`docs/ic/GROEBNER_F4F5F6_AUDIT_20261010.md`; the subagent's Callgrind files.
