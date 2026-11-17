# Protocol addendum: identical-stream re-measurement of the orbit-aware double large prime

Status: written and committed **before** any re-measurement it governs. It adds
to [`PROTOCOL.md`](PROTOCOL.md) and corrects none of the merged files:
`PROTOCOL.md`, `RESULTS.md`, `receipts/` and `results/` stay exactly as merged
(write-once evidence). Corrections are additive: this file now, and
`RESULTS_ADDENDUM_20261026.md` after the runs.

Date: 2026-10-26. Branch: `claude/orbit-followups-20261026`. Base: `origin/main`
at `3f31162cd4` (contains the merge `f24efb19d` of PR #2591). Toolchain on the
measuring VM: rustc/cargo 1.97.0, 4 vCPU Xeon 2.1 GHz, 16 GB, shared with other
workers (so timing is diagnostic, as in `PROTOCOL.md` section 8).

## 1. Why this addendum exists

An independent review of the merged PR found that arm (ii) `dlp` and arm (iii)
`dlp_orbit` did **not** see an identical relation stream, although `PROTOCOL.md`
section 1 ("from an identical relation stream") and `RESULTS.md` section 4
("counter differences are exact properties of the arm") say they did.

Mechanism, from the code at the merge (`src/cryptanalysis/ecbench_large_prime.rs`,
`Decomposer::find`): the collector walks the right-hand table and accepts the
**first** decomposition whose number of distinct nonzero large *columns* is at
most `max_large_primes` (2). In orbit mode the column map is coarser (several
large points share one orbit column, and coefficients can cancel), so a
decomposition with three distinct large points of which two are Frobenius
conjugates has two large columns and is accepted by arm (iii) but rejected by
arm (ii). Arm (iii) therefore accepts a superset of the decompositions arm (ii)
accepts, and its first-hit decomposition of a trial can differ.

Checked against the merged receipts before writing this file (`jq` on
`partials_by_class`, repeat 1, 25 primary `(n, seed)` pairs): the class
histograms of arms (ii) and (iii) differ in 6 pairs (n = 23 seed 4, n = 29 seeds
1 and 3, n = 31 seeds 1, 2 and 4); for example n = 31 seed 1 is `[1,39,331]`
(371 accepted) for (ii) and `[1,40,334]` (375) for (iii). Equal histograms do not
prove equal streams, so 6 is a lower bound on the number of differing pairs.
The size of the difference is small (at most 4 of about 375 accepted relations
in these 6 pairs), but the claim "identical stream" was false as written.

Other facts known **before** this protocol (so the reader can discount them as
predictions): everything in `RESULTS.md`; the reviewer's window-structure counts
in section 7 below; the reviewer's cap observations in section 8. Nothing from
the code of this PR has been run.

## 2. Questions

* **Q1 (identical stream).** Does the quotient of the large-prime vertices by
  signed Frobenius orbits still yield more verified independent relations than
  the plain double large prime when both arms are fed the *same* accepted
  decompositions? (arm (iii-post) against arm (ii).)
* **Q2 (acceptance-set effect).** How much of the original (iii) minus (ii)
  difference is due to arm (iii) accepting more decompositions? ((iii) minus
  (iii-post).) This is a genuine property of an orbit-aware *collector* and is
  reported as such, not as an error.
* **Q3 (cap-free yield).** `RESULTS.md` reports yield at a fixed trial cap,
  which is a windowed metric (finding E ii). How many trials does each arm need
  to reach a given verified rank, without a cap?
* **Q4 (replication).** Do the five seeds, which share one factor base per n,
  say anything about other realisations of the base? (Section 6.)
* **Q5 (structure).** What is the structure behind the n = 31 merge excess and
  the "birthday effect" reading? (Section 7; no relation to the arms.)

## 3. Arms

All arms keep the pipeline of `PROTOCOL.md` section 3: collect every accepted
partial relation for `cap` trials (no early stop), run `filter_large_primes`
once, replay every emitted row twice (linear replay, curve replay), feed verified
rows to an incremental Gaussian elimination mod `r`.

| arm | `LargePrimeGraph` | acceptance (columns counted for `max_large_primes`) | graph the filter sees |
|---|---|---|---|
| (ii) `dlp` | `Filter` | plain | plain large columns |
| (iii) `dlp_orbit` | `FilterOrbit` | **orbit** columns | orbit columns |
| (iii-post) `dlp_orbit_post` (new) | `FilterOrbitPostCollect` | **plain** columns | orbit columns, applied after collection |

(iii-post) builds both the plain and the orbit factor base from the same atom
table (same raw points, same order; checked at run time), runs the unchanged
meet-in-the-middle search with the plain column map, and only for storage
re-expresses each accepted decomposition in the orbit columns (`+-lambda^k`
coefficients, cancellations dropped). By construction its accepted decompositions
are those of (ii).

**Identity check (an acceptance criterion of the experiment, not an assumption).**
With `SolveOptions::stream_digest` the collector hashes (BLAKE3) every accepted
decomposition `(trial, a, b, raw term list)` in order. For every `(n, seed)`
the digest of (iii-post) must equal the digest of (ii), and their `trials`,
`decompositions`, `accepted_relations`, `relation_histogram` and
`partials_by_class` must be equal. A cell pair that fails is reported as an
**invalid identity pair** and excluded from the Q1 contrast (not silently
filtered: listed). The digest of (iii) is reported next to them and is expected
to differ in the pairs where the acceptance sets differ.

Arms (i) `none` and (iv) `dlp_orbit_absorb` are **not** re-measured here; their
published numbers stand as measured, with arm (iv) carrying the same
acceptance-set caveat as arm (iii) (its column map is coarser still).

Default mode (`LargePrimeGraph::Online`) and the arms (ii), (iii), (iv) are not
changed: the golden fixture test must pass unchanged, and the deterministic
counters of (ii) and (iii) re-measured here must equal those in the merged
receipts cell by cell (**reproduction check**; a mismatch is a finding about
code drift and is reported, not smoothed over).

## 4. Primary re-measurement

Same instances, factor bases, summand counts, caps and seeds as `PROTOCOL.md`
section 4:

| n | curve | small dim | envelope dim | summands m | cap | seeds |
|---|---|---|---|---|---|---|
| 17 | K_1 | 5 | 10 | 3 | 200 | 1-5 |
| 19 | K_1 | 6 | 10 | 3 | 400 | 1-5 |
| 23 | K_1 | 7 | 10 | 3 | 450 | 1-5 |
| 29 | K_1 | 7 | 10 | 4 | 300 | 1-5 |
| 31 | K_0 | 7 | 10 | 4 | 450 | 1-5 |

`max_states` 3,000,000. Arms (ii), (iii), (iii-post). Repeats for the timing
control: 2 for n <= 29, **1 at n = 31** (the merged run used 3 and 2; reduced to
fit the compute budget; timing is diagnostic and the counters are deterministic,
so a repeat adds nothing to them). This is a deliberate deviation from
`PROTOCOL.md` section 4 and is stated here, not discovered later.

**Primary contrast (fixed now).** For each n, `D_post = indep(iii-post) -
indep(ii)` paired by seed (S = 5, 95% Student-t half-width, 4 degrees of
freedom). Verdict per n:

* `survives`: every seed has `D_post >= 0`, and the interval lies strictly above 0;
* `no gain`: the interval contains 0;
* `reversed`: the interval lies strictly below 0.

Reported next to it: `D_orig = indep(iii) - indep(ii)` (the merged headline),
`D_acc = D_orig - D_post` (acceptance-set effect, Q2), the same three for
`complete`, and the per-seed signs. The merged headline gains (+3.0, +0.6, 0.0,
+32.4, +16.0 independent relations at n = 17, 19, 23, 29, 31) are the numbers to
compare. The headline sentence "more verified relations from the same relation
stream at n = 17, 29 and 31 (every seed)" is called **confirmed on the identical
stream** only for those n whose `D_post` verdict is `survives`; otherwise it is
reported as **not confirmed**, with the number. No multiplicity correction: five
curves, descriptive.

**Prediction recorded before the run** (so a failure of it is visible): the
merged gains at n = 17, 29, 31 survive with `|D_orig - D_post| <= 4`
independent relations (because at most 4 of about 375 relations differ), n = 19
stays indistinguishable from 0, n = 23 stays exactly 0.

## 5. Cap-free metric (trajectory series)

Same instances and seeds, arms (ii), (iii), (iii-post), one repeat, **extended
cap**: `4 x cap` (n = 17: 800; 19: 1600; 23: 1800; 29: 1200) and `3 x cap` at
n = 31 (1350; `4 x` would cost about 13 CPU minutes by itself). The residual
stream depends only on `(seed, trial)`, so the stream of a cell is the prefix
of the stream of a longer cell and the primary-cap counters of the primary cell
must be reproduced from the trajectory cell at trial cutoff = primary cap
(**prefix-consistency check**).

For each arm and cell the pipeline re-runs filter, both replays and the rank
update on prefixes of the stored partial relations (every `max(1, N/64)`-th
prefix of N stored partials, then exact refinement inside the first interval
that reaches each target). Recorded, in trials:

* `T_q`: the trial index of the first stored partial relation whose prefix
  reaches verified rank >= `q`, for `q = ceil(f (S+1))`, `f` in {0.5, 0.75, 0.9,
  1.0}; and `T_rec`: the first prefix at which the target column is pinned (the
  scalar is recovered);
* rank, complete, independent at cutoffs `cap/4, cap/2, cap, 3cap/2, 2cap, 3cap,
  4cap` (those within the run);
* `monotone` (rank never decreased along the scanned prefixes) and
  `consistent_with_pipeline` (the full prefix equals the cell's own result).

A target that is not reached within the run is **censored** (reported as
`> max trials`, never imputed). Paired comparison per seed: `T_q(ii)` against
`T_q(iii-post)` and `T_q(iii)`; sign counts and the per-seed table; a mean
difference with a t interval only where no seed is censored. This metric needs
no cap choice, but it is capped by the run length; that is stated, not hidden.

## 6. Realisation series (replication unit)

In the merged run the five seeds of one n share **one** factor base and one
target (`merged_points` is identical across seeds). The seeds therefore replicate
only the residual stream; the replication unit across factor bases is the five
curves. To see how much the result depends on the particular base, a second
realisation per n is run: the **same abstract curve** (same group, same `r`, same
cofactor) represented over a different irreducible polynomial, so that the
prefix window `x < 2^10` is a different `F_2`-subspace of the field. The modulus
is the first irreducible found from a seeded random search
(`SplitMix64(0x6d6f_6475_6c75_73 ^ n)`; masks `1 | 2^n | random bits below n`,
rejecting those with fewer than `ceil(n/3)+2` terms). Arms (ii), (iii),
(iii-post), seeds 1-3, primary caps, one repeat. This series is exploratory: it
never replaces a primary verdict and is reported separately. It also measures,
at the level of the merge table, whether the structure of section 7 survives a
change of representation (the dense-modulus question).

## 7. Structure of the merges (finding D)

`examples/orbit_window_structure.rs` (Rust; no Python) reproduces, with the
same factor-base window the bench uses (`x < 2^10`, large part `2^s <= x`):

* **Pair counts.** The number of unordered pairs `{x0, x1}` of abscissae in the
  window with `x1 = x0^(2^m)` for some `1 <= m < n`, for the prefix window and
  for `K = 8` seeded random 10-dimensional subspaces (nested: the first `s`
  basis vectors span the "small" part), n in {17, 19, 23, 29, 31}. The reviewer's
  prefix counts (stated before this protocol): 76, 14, 0, 20, 112; random
  subspaces: 53-61, 9-20, 0-3, 0, 0 (four subspaces each).
* **Merge classification.** The large-point table is recomputed from the
  specification (not from the library's `canonicalise`): canonical projected
  points, brute-force pairwise Frobenius test, union-find. Each merge is split
  into *raw-Frobenius image* (some raw abscissae of the two points satisfy
  `x1 = x0^(2^m)`) and *projected-only coincidence* (conjugate after the
  cofactor projection although no raw abscissae are Frobenius images). The
  recomputed `L`, orbits and merged points must equal the merged receipts'.
* **Dense modulus.** The same classification on the random modulus of section
  6, in particular at n = 31 (the reviewer did not test it). Recorded
  prediction: the structural (raw-Frobenius) merges vanish and the merged count
  falls to the random-model level; the measurement may contradict this.

## 8. Statistical design caveats declared in advance (finding E)

* (i) Replication unit: see section 6; the t intervals in `RESULTS.md` cover
  residual-stream variability for one base realisation per n.
* (ii) Caps: the pilot sweep (`pilot/p3_cap_sweep`) used both arms and the
  chosen caps sit at the knee before rank saturates (n = 29: cap 300 gives
  22 -> 57 independent relations for (ii) -> (iii) in the pilot at seed 9004, cap 600
  gives 62 -> 62; n = 31: cap 300 gives 5 -> 11, cap 600 gives 66 -> 66; n = 17: cap 200
  gives 8 -> 13, cap 300 gives 14 -> 14; checked against `pilot/p3_cap_sweep`). "Yield at a cap" is a windowed metric, not an effect size;
  section 5 adds the cap-free one.
* (iii) CPU per independent relation: at n = 31 the collect phase is about
  10 s and filter + replay about 2 ms, so the metric is `collect / indep` and
  its verdicts duplicate the independent-relation counts. The orbit arms also
  carry set-up not counted in "online CPU" (certifying `LambdaPowers`,
  canonicalisation, one self-check scalar multiplication per large point;
  `phase_group_ops.factor_base` is identical across arms because those
  multiplications are not routed through the counter). This re-measurement
  reports collect, filter, replay and set-up CPU separately and ranks nothing
  by CPU per relation.

## 9. Not changed by this addendum

Unit-test and tooling fixes that accompany it (golden-test normalisation, tests
with real merges and absorptions, a negative test with a corrupted absorb factor,
set-up self-check of the small-column factors, fail-closed integrity fields in
the summarizer, `pipefail` in `run_all.sh`, CLI help, `r < 2^63` guard) are code
changes with their own tests; they do not touch any recorded measurement.

## 10. Stop conditions and budget

Bounded runs (`timeout` per cell). Total measurement compute about 15 CPU
minutes (primary about 4, trajectory about 9, realisation about 2) on top of the
builds. A cell that fails or times out is kept in the receipts directory as a
failure line and reported. No result is dropped; null and negative results are
results.

## 11. Cost accounting

Counted unit: process CPU of the single-threaded collector, by phase (set-up,
collect, filter, replay, audit), as in `PROTOCOL.md`. The unpriced native work
(combination table, hash lookups, canonicalisation, modular row operations) is
unpriced and listed; no cross-method (rho) comparison is made, so the
section 5a pricing band is not applicable to this correction.
