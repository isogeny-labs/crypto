# Sieved relation search on hyperelliptic Jacobians (SMOKE, 2026-11-16)

All figures `[pcc:unverified]`, small smoke runs, not a verdict. Code:
`src/cryptanalysis/genus_ladder_sieve.rs` (one line in `mod.rs`). Raw rows: `smoke.jsonl`.

## What was built
Relations from the functions `y - v_t(x)`, `v_t = v + t*u`, where `(u,v)` is `g` factor-base
points (Lagrange interpolant) so `R_t = (f - v_t^2)/u` has degree `g+1`; the roots of `R_t`
give `2g+1` places summing to zero in the Jacobian. A factor-base abscissa `x` is a root of `R_t`
exactly at `t = (+-y_x - v(x))/u(x)` (batched inversion), so one counter array over `t in F_p`
replaces the per-candidate splitting test. A `t` with `g+1-lp` hits is a relation with `lp <= 2`
large primes; the cofactor comes from the top two coefficients of `R_t` plus a sqrt table (no `x^p`).
No group operation in the relation phase.
Individual log: reduced `T + aG` as `(u,v)`, same sieve, full-base decomposition of `R_t`
(`g+1` roots), `a` stepped by one Cantor addition; two such rows (different `a`) fix the scale of
the homogeneous system and the unknown `t` (rows `sum(-eps) l - h t = h a`). Linear algebra:
`build_rows`, `peel`, `wiedemann_coord` (square, fallback `wiedemann`), certificate `[t]G = T`
(counted `curve.mul`, independent `mul_rtl` taken out of the tally). `Rel` holds 5 entries and 2
vertices: the `g` fixed points enter through a family vertex plus defining row; 2 true large primes
are written directly only at g=3 (5 entries); at g=4 `lp_max` is effectively 1.
Counting: every native mul/add (base table, families, sieve; each counter update charged 3 adds),
LA as in `run_ic_ext`, all group ops of the individual-log walk and certificate; table look-ups not counted
(as the baseline's `chi`).

## Tests (release, `cargo test --lib genus_ladder_sieve`)
`family_top_coefficients_match_brute_force` (e1,e2 vs schoolbook division, all t), `sieve_hits_are_roots`,
`sieved_relations_are_valid_and_certify` (every relation and individual row re-checked by Cantor
arithmetic, certified log, p=131/257, g=3/4, lp 0..2). See the final status in the report below.

## Smoke numbers (c = 2.0, nb = fb_size_2lp; mean over curve seeds x solver seeds 1-3; all certified)
Baseline: `run_ic_ext {coord_square, unit_t, ferrari:false}`, DoubleLp, prefilter. Native = mul+add.

| p,g | arm | group ops | Cantor F_p mul | relation-phase native | LA native | total native | relations |
|---|---|---|---|---|---|---|---|
| 1031,3 | run_ic_ext | 4177 | 922k | 1.17M | 0.54M | 1.71M | 496 |
| 1031,3 | sieve lp<=0 | 83 | 20k | 1.13M | 0.77M | 1.91M | 215 |
| 1031,3 | sieve lp<=1 | 83 | 20k | 0.35M | 1.07M | 1.41M | 437 |
| 1031,3 | sieve lp<=2 | 83 | 20k | 0.18M | 1.90M | 2.08M | 517 |
| 2053,3 | run_ic_ext | 8946 | 1.97M | 2.72M | 1.42M | 4.14M | 928 |
| 2053,3 | sieve lp<=1 | 99 | 24k | 0.83M | 4.00M | 4.82M | 763 |
| 2053,3 | sieve lp<=2 | 99 | 24k | 0.38M | 7.86M | 8.23M | 957 |
| 1031,4 | run_ic_ext | 13704 | 5.40M | 5.29M | 2.11M | 7.40M | 533 |
| 1031,4 | sieve lp<=1 | 113 | 49k | 1.25M | 3.97M | 5.22M | 521 |
| 2053,4 | run_ic_ext | 31497 | 12.4M | 12.9M | 6.89M | 19.8M | 1025 |
| 2053,4 | sieve lp<=1 | 112 | 48k | 3.19M | 11.6M | 14.7M | 978 |

Reading (smoke only): relation-phase native work per relation falls 6x (g=3, lp<=2) to 3-4x
(g=4, lp<=1), group operations fall ~50-300x (only the individual-log walk and certificate remain). The
linear algebra grows 2-5x because rows are `2g+1` entries wide (7 or 9) against 3 or 4, so
total native work is 0.8x to 2x of the baseline, and once Cantor operations are priced (at 0.1 or 1 GAE per
op or by their F_p work) the sieve is far below. Larger `lp` lowers relation cost but raises LA cost; no tuning done.

## Open issues / not verified
* Literature: row 28 of `lit_factor_bases.md` lists Joux-Vitse (Eurocrypt 2012), Sarkar-Singh ePrint 2014/815 and
  2015/179, Vitse-Wallet (LATINCRYPT 2015). I did not open them; the family `y - v(x)` with `v` through fixed
  points and the counter-array sieve are my reconstruction from the idea in that row. I could not verify
  what those papers prove, their exact families, or their sieve details.
* Wide rows are the cost: a smarter system (family vertex kept as a column, or two parameters to cut the number of
  fixed points) could lower LA. Not tried. g=4 with two large primes does not fit `Rel` (REL_FB=5).
* Relation statistics (hits per family, dependent rows from one family sharing the fixed points) not analysed.
* The individual-log step needs a full-base decomposition (nlp=0), small for these p; its cost grows ~p
  and is counted (`ind_*`); at larger p a descent with large primes would be needed.
* Smoke only: 1-2 curves per (p,g), 3 solver seeds, wall time not compared, no replay certificate, validator or rho reference.
