# Line-sieve feasibility probe, 2026-11-14 (not under a frozen protocol)

This probe tests lead 1 of `LEADS-20261114.md`, the line sieve of Sarkar–Singh (ePrint 2014/815).
It is orientation, run on branch `research/hec-sieve-probe-20261114`. It carries no pre-registered
expectations and no `S` claim. Its job is to decide what the next frozen slice should be. Any
protocol that builds on it must say that these numbers were used to choose it.

- **Code:** `crates/hec-native-ic/src/sieve.rs` and `examples/sieve_probe.rs`.
- **Raw output:** `sieve-probe-20261114/`, with load logged in `host.txt`. The host was shared, at
  load average 46–101.
- **Status of the numbers:** wall clock is descriptive only. Counts are deterministic.

## Mechanism

Take a reduced `D = (u, v)` with `deg u = g`, and put `a_c = v + c·u`:

- `u` divides `f − a_c²`, so `w_c = (f − a_c²)/u` is monic of degree `g + 1`;
- `div(y − a_c) = D + E_c − (2g+1)∞`, so `[D] = −[E_c]`.

A base place `(x_j, ±y_j)` lies on `E_c` exactly when `c = (±y_j − v(x_j))/u(x_j)`. One pass over
the base works out these `c` values and buckets them. The cost is Horner evaluation, one batched
inversion, and two multiplications per point. A `c` hit at `g + 1` distinct base points is a
relation, and no smoothness test is needed.

Lines are walk positions `αD₁ + βD₂`, one Cantor addition apart. Rows follow `ic.rs`'s convention,
with `g + 1` base entries plus `k`.

## Results

**Correctness.**

- In the verified run (`round8-stopmid.txt`), every relation is checked in the Jacobian, and all of
  them hold.
- On every run (55 round-eight runs and 15 larger runs, verified and timing builds alike), `k` is
  recovered and verified, and the first solve round succeeds.
- Test `sieve::tests::sieve_solves_and_verifies` passes. [pcc:unverified]

**Yield per line**, with whole lines kept (`round8.txt`):

| row | relations per line | `p/(g+1)!` | ratio |
|---|--:|--:|--:|
| genus 2, p = 101 | 12.6 | 16.8 | 0.75 |
| genus 2, p = 251–4001 | 38.8–664 | 41.8–667 | 0.93–1.03 |
| genus 3, p = 61, 211, 401 | 2.26, 7.12, 14.3 | 2.54, 8.79, 16.7 | 0.89, 0.81, 0.85 |
| genus 4, p = 31, 61 | 0.11, 0.69 | 0.26, 0.51 | 0.42, 1.35 |

The table below sets `stop_mid_line`, which truncates the last line, so its relations per line read
low; use this table for yield. At genus 3, p ≤ 401, yield runs short of `p/(g+1)!`, and the cause
has not been counted. At genus 4 the rows are too small for the figure to mean much.

**Counts and phases.** Medians over seeds, verification off (`round8-timing-v2.txt`,
`larger-timing-v2.txt`, `summary.md`).

- The relation phase is total − solve − factor base, in both arms.
- The walk uses `Solver::Dense`; the sieve uses `Solver::Fast`.

| g | p | m | walk trials/rel | sieve mults/rel | sieve line adds/rel | relation phase ms, walk / sieve | solve mulmods, walk / sieve | total ms, walk / sieve |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 503 | 259 | 1.86 | 30.8 | 0.015 | 1.07 / 0.18 (5.9×) | 116k / 7.9k | 1.49 / 1.07 |
| 2 | 4001 | 2005 | 2.00 | 31.9 | 0.002 | 3.78 / 0.60 (6.3×) | 916k / 1.39M | 8.14 / 12.35 |
| 2 | 16411 | 8205 | 2.00 | 32.0 | 0.001 | 18.1 / 3.75 (4.8×) | 3.57M / **70.3M** | 33.7 / 363 |
| 3 | 401 | 196 | 6.35 | 143 | 0.073 | 2.65 / 0.27 (9.7×) | 136k / 19.9k | 2.99 / 0.70 |
| 3 | 1021 | 506 | 6.17 | 128 | 0.025 | 5.89 / 0.50 (11.8×) | 269k / 194k | 6.86 / 2.51 |
| 3 | 4001 | 1997 | 6.11 | 119 | 0.006 | 28.2 / 3.08 (9.2×) | 2.20M / 8.54M | 38.0 / 50.6 |
| 4 | 127 | 66 | 21.1 | 735 | 0.946 | 3.34 / 0.48 (7.0×) | 67k / 4.6k | 3.56 / 0.61 |
| 4 | 1009 | 488 | 26.2 | 849 | 0.145 | 33.1 / 2.58 (12.8×) | 482k / 488k | 36.9 / 6.38 |

`summary.md` has all 16 rows.

**What the sieve costs, by count.**

- Field multiplications per relation:
  - about 32 at genus 2;
  - 119–146 at genus 3;
  - 600–1,600 at genus 4 (735–849 for p ≥ 127).
- On top of that, about `(g+1)!/2 · m/p` Cantor additions per relation, from the one addition per
  line. That is significant only at genus 4 with small p.
- The walk spends 2, 6 and 21–26 trials per relation. The code hunt counted 176–362, 513–711 and
  752–930 REDC per trial at genus 2, 3 and 4 after S3.

So by count the sieve is roughly 6–11× cheaper per relation at genus 2, 11–15× at genus 3 and
about 12× at genus 4. These combine the code hunt's per-trial counts with this probe's
trials-per-relation, so they are estimates, not a measurement. On the clock the relation phase is
5–13× cheaper on the larger rows.

The genus-2 gap is smaller on the clock than by count. Something the counter does not see is part of
the sieve's cost. Per-relation allocations (`row`, the `grp` clone, `Vec<Vec>` groups) are the
likely suspect; they have not been measured.

## The solve is now the bottleneck, and the reason is structural

At m ≳ 1000 (genus 2) and m ≈ 2000 (genus 3), the sieve loses end to end. Its solve multiplications
grow much faster than the walk's: 70.3M against 3.57M at genus 2, m = 8205. [pcc:unverified]

This is not because sieve rows are harder for a sparse solver. With the same plain sparse solver
(`Solver::Fast`), sieve rows solve no worse than walk rows. At genus 2, m ≈ 2000, sieve rows take
8.6 ms here, against S2b's 13.7 ms for walk rows (record `20261114T2233Z-hec-s2b-dense-solve`).

The walk's advantage comes entirely from `Solver::Dense`.

- A walk row has a dense part over the ≈ 64 step columns, plus at most `g` sparse entries.
- Dense sets the step block aside. Phase 1 then eliminates only the weight-≤ g sparse parts.
- At genus 2 those parts have weight ≤ 2, so they form a graph. Eliminating `{j, a}` from
  `{j, b}` leaves `{a, b}`, still weight 2. Elimination on a graph never fills in.
- This explains S2b's near-linear genus-2 solve (0.21 → 1.76 ms for m = 259 → 2005).
- Sieve rows carry `g + 1` random base entries. That is weight 3 at genus 2, a hypergraph, and
  Markowitz fills in.

Below m ≈ 500 the sieve's solve is the cheaper one, because its rows have no dense block.

## What this decides

1. **The relation phase: go.** The sieve is exact: every relation holds by construction, and the
   probe checked all of them. It replaces both the walk's per-trial loop and the oracle, at about
   32 / 120–146 / 735–849 field multiplications per relation.
2. **The full-base solve: not yet.** Sieve rows need a real sparse solver at large m. That is plan
   track T3, structured Gaussian elimination then block Wiedemann (the `lpsgebw2` port). Or the
   solve needs a row design that keeps sparse parts at weight ≤ g, as the walk's do.
3. **The exponent regime is untested.** This probe used the full base and no large primes. The
   genus-3/4 targets use a base of κ·q^{2/3} with large primes, where most usable `c` have `g` or
   `g − 1` base hits:
   - With `g` hits the cofactor is linear. Its root comes from `w_c`'s `x^g` coefficient minus the
     known roots.
   - With `g − 1` hits it is quadratic: one Legendre symbol and one square root.

   Those economics have not been measured. The full-base result must not be carried over.
4. **Engineering, before any wall-clock protocol:** allocation-free relation output, and a dense
   bucket array sized to p, which needs replacing beyond p ≈ 2^24.

## Next steps (proposed)

- **(A) A reduced-base, large-prime sieve probe.** It needs a base-subset mask and a hit threshold of
  `g` or `g − 1`, with the cofactor finish. Count partial relations per unit of work against the
  walk's partials per trial, at T1 sizes (genus 3 to q ≈ 2^15). This decides the T1 relation source.
- **(B) A solver for sieve rows (T3).** Port SGE plus block Wiedemann, and measure solve
  multiplications against m on sieve rows.
- **(C) A frozen slice for the full-base sieve at genus 3/4**, where it already wins end to end at
  m ≤ 500.
  - Primary statistics: counted multiplications per relation, including the per-line addition, and
    solve multiplications.
  - Gate: every relation checked in the Jacobian, and every `k` verified.
  - Wall clock: descriptive only, unless a reserved host is named, because S3's A/A floor failed
    twice on this laptop. [pcc:unverified]
