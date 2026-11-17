# Protocol: orbit-aware double-large-prime relation graphs (Lead 3)

Status: written and committed **before** the measurement runs it governs. The
only data that preceded it are the pilot/calibration runs listed in section 6,
which used seeds 9001-9004 that the measurement never uses.

Date: 2026-10-11 (local base `bbd330a9d`, the Oct 11 `main`; not refreshed).
Branch: `claude/orbit-aware-double-large-prime-20261011`.

## 1. Question

Does quotienting the large-prime vertices of the double-large-prime graph by
signed Frobenius orbits, while keeping the phase coefficient `+-lambda^k`,
increase the number of independently *verified* relations obtained from an
identical relation stream, and at what CPU cost per verified relation?

This is a correctness-and-yield measurement on small Koblitz curves. It makes
no ECDLP speedup claim and is not compared with Pollard rho (see section 8).

## 2. Mechanism under test

On a Koblitz curve the 2-power Frobenius `pi(x,y) = (x^2,y^2)` acts on the
prime-order subgroup `<G>` as multiplication by `lambda`, so
`log pi^k(Q) = lambda^k log Q` and `log(-Q) = -log Q`.

The orbit mode (`LargePrimeGraph::FilterOrbit`, default off) canonicalises every
large projected point `Q` to the member of `{+-pi^k(Q)}` with least abscissa
(sign-normalised like the collector's own `canonical`), records the phase `k`
and sign `eps`, and gives the orbit's column the coefficient
`eps * lambda^{-k} (mod r)` times the atom's coefficient. Each factor is checked
against real curve arithmetic when the table is built (`[factor]rep == Q`).

Correctness condition: a recombined cycle must cancel the actual subgroup
elements with their proper coefficients, not merely their canonical
identifiers. It is checked, not assumed, by two replays of every complete
relation the filter emits:

1. **linear replay**: `replay_complete_relation` recomputes the row from its
   source certificate and must equal the emitted row;
2. **curve replay**: the row `sum_c v_c x_c - B d = rhs` is the group identity
   `sum_c [v_c] Q_c = [rhs] G + [B] T` over the small columns' actual points
   `Q_c`; it is checked with scalar multiplications only.

A complete relation is *verified* iff both replays pass. In addition every input
partial row is audited against its own group identity (excluded from the online
cost, reported separately). Any failure is counted and reported; a cell with a
failure is not discarded.

## 3. Arms (identical seeds, factor base, summand count and trial cap per n)

| arm | graph | max large primes | role |
|---|---|---|---|
| `none` (i) | `Filter` | 0 | control: full relations only |
| `dlp` (ii) | `Filter` | 2 | control: double large prime, plain vertices |
| `dlp_orbit` (iii) | `FilterOrbit` | 2 | treatment: vertices are signed Frobenius orbits |
| `dlp_orbit_absorb` (iv) | `FilterOrbitAbsorb` | 2 | secondary: as (iii), and a large prime whose orbit contains a small column is rewritten into it |

All four go through the same pipeline: collect every accepted partial relation
for `cap` trials (no early stop), run the batch `filter_large_primes` once,
replay every emitted row, feed verified rows to an incremental Gaussian
elimination mod `r`. The residual stream `(a,b)` depends only on the seed, so
arms see identical targets. The pre-existing streaming `LargePrimeEliminator`
path stays the default and is not one of the arms; its output is unchanged
(regression test against the base commit).

## 4. Instances and parameters (fixed here)

Curves are the repo's own `koblitz_instance(a, n)` (prime-order subgroup of the
largest prime factor, generator, certified `lambda`), taking the `a` in {0,1}
with the larger admitted `r`; the planted logarithm is a function of `n` only.

| n | curve | #E | r | cofactor | small dim | envelope dim | summands m | primary cap (trials) | dose caps | seeds |
|---|---|---|---|---|---|---|---|---|---|---|
| 17 | K_1 | 131174 | 65587 | 2 | 5 | 10 | 3 | 200 | 100, 400 | 1-5 |
| 19 | K_1 | 525086 | 262543 | 2 | 6 | 10 | 3 | 400 | 200, 800 | 1-5 |
| 23 | K_1 | 8393806 | 4196903 | 2 | 7 | 10 | 3 | 450 | 225, 900 | 1-5 |
| 29 | K_1 | 536911222 | 42457 | 12646 | 7 | 10 | 4 | 300 | 150, 600 | 1-5 |
| 31 | K_0 | 2147574356 | 1439393 | 1492 | 7 | 10 | 4 | 450 | 225, 900 | 1-5 |

`max_states` = 3,000,000 everywhere. Repeats: arms `dlp` and `dlp_orbit` run 3
times per cell (2 at n=31) to estimate timing noise; `none` and
`dlp_orbit_absorb` once. The dose series (caps 0.5x and 2x the primary cap; arms
`dlp` and `dlp_orbit`; seeds 1-3; one repeat) is secondary.

`m = n-1` (the repo's exact default) is infeasible at these `n` under any
`max_states` that fits memory; `run_all.sh` records the collector's own refusal
message per `n` rather than silently substituting a smaller `m`. The chosen `m`
and the factor-base dimensions were selected in the pilot (section 6) by two
criteria, not by the quantity being compared: the no-large-prime arm must find a
full relation in at most roughly 10% of trials (so large primes can matter), and
the dlp arm's rank must be neither empty nor saturated at the cap (so the rank
can move). The caps were fixed from the pilot seeds before any measurement seed
was run.

## 5. Metrics and analysis (fixed here)

Per cell: partial relations by class (full/single/double); complete relations
emitted by the filter; replays verified / failed (linear, curve); partial-row
audit failures; newly independent verified relations (`indep`); rank out of
`S+1` columns; CPU seconds by phase; peak RSS of the cell's own process; MITM
states (left+right); whether the recovered scalar equals the planted one.

* **Online CPU** = process CPU (`CLOCK_PROCESS_CPUTIME_ID`, the collector is
  single-threaded) of *collect + filter + replay*. Setup (factor base, exact
  combination table: identical across arms) and the partial-row audit are
  reported but excluded.
* **Primary metric**: online CPU seconds per newly independent verified
  relation = online CPU / `indep` (undefined when `indep = 0`).
* **Secondary metric**: complete relations per attempt = `complete / cap`
  (an attempt is one residual trial), and `indep` / rank at the cap.
* **Paired differences**: for each (n, cap, seed), orbit minus plain (`dlp`), on
  complete, `indep`, rank (deterministic given the seed, so no timing noise),
  and on online CPU % and CPU-per-indep % (timing noise applies).
* **Aggregate**: mean and 95% Student-t half-width over the S seeds, plus the
  count of seeds where orbit exceeded / equalled / fell below plain.
* **Noise rule for timing**: the control is the mean absolute relative change of
  online CPU between repeats of the *same* cell. A CPU difference is called
  **within noise** if |mean paired difference| <= the t half-width, or <= the
  control; otherwise **outside noise**. Counter differences are reported as
  measured with their sign counts; with S = 5, a sign pattern of 5/5 has exact
  two-sided sign-test p = 0.0625, so none is called significant on that test
  alone.
* **Mechanism check**: observed `merged_points` is compared with the random
  model `C(L,2)(2n-2)/(r-1)` (L large points). The expectation written down here
  is agreement within a factor of 2 where the model predicts at least 5.

## 6. What preceded this protocol

Pilot/calibration only (not measurements): seeds 9001-9004, n=17 with several
`(small, env, m)` and cap choices, then the five-n sweep that fixed section 4.
Their receipts are kept under `pilot/`. Observations that shaped the design are
disclosed in RESULTS.md. The very first pilot (n=17, small=5, env=9, m=4) showed
both double-large-prime arms saturating the rank (15 of 15) at 1000 trials, and
the no-large-prime arm costing 54 s CPU for 1000 trials; that is why the caps
are low and the table sizes moderate.

## 7. Validity gates

A run is reported with its failures, never silently filtered. The experiment
counts as *internally valid* iff, across all receipts: replay failures = 0,
partial-audit failures = 0, every recovered scalar equals the planted log, and
the repeats of each cell agree exactly on every deterministic counter.

## 8. Scope limits (stated in advance)

* Toy subgroups (`r` between 2^15 and 2^21), prefix-subspace factor bases, tiny
  summand counts. Nothing here says anything about ECC2K-130 or any deployed
  curve.
* No matched-rho run, so ledger hypothesis H-BINSTD-555991 is not tested.
* Expected orbit collisions among a fixed prefix-subspace factor base are a
  birthday effect, `~ C(L,2)(2n-2)/r`; a Frobenius-*closed* factor base (where
  every large prime has `n` orbit mates in the base) is a different regime and is
  not measured here.
* CPU numbers are single-threaded process CPU on a shared 4-core VM; they are
  diagnostic, not a benchmark.
