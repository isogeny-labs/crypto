# Protocol: the weak-class cover row re-priced in `bitop/v1`, both arms executed

Frozen 2026-10-21, committed before any measured run of this study. Pilot
runs used to size the study were at `p = 101` and `p = 503`, which are not
study instances.

## Question

The scoreboard row "Whole method from a curve that is not weak (section 18):
p = 1,511 (subgroup 2^61.4), 4 seeds", `0.010×`, comes from
`examples/jv_isogeny_walk.rs --e2e` (`run_end_to_end`). Its denominator is not
a measured rho. `run_end_to_end` passes `rho_runs = 0` and divides by the
pooled `rho_s_ref = 1.3609`, which comes from smaller sizes (ledger §15). The
unit is the legacy "`F_p` multiplication equivalent", in which:

- additions are free;
- an `F_{p²}` inversion is a flat 21;
- the F4 counter sees products only;
- a Wiedemann operation modulo `ℓ` is a flat 16;
- table probes and scans are free.

**Does the row stay below Pollard rho when both arms run on the same
challenge curve and seed, and both are counted in `bitop/v1` primitives
(`docs/units/BITOPS.md`)?**

Class: **accounting**. This is a re-measurement of an existing algorithm
against an executed reference, not an improvement. The result is the gap to
rho, reported as the distance remaining.

## Instances

The instances are those of ledger §18.4 (`run_end_to_end`). For each
`(p, seed, moves)`, the weak instance of `generate_spec(p, seed)` is moved
`moves` random isogeny steps off the weak locus, giving the challenge curve
`C` with `G`, `Q = [d]G` and the planted `d`. Jumps are `3, 5, 7` and the walk
cap is `3p²`. Building the instance is not charged.

- **Primary:** `p = 1511` (`ℓ ≈ 2^61.4`), seeds 1–4, `moves = 8`. These are the
  row's four runs.
- **Secondary:**
  - `p = 1511`, seeds 1–4, `moves = 64`;
  - `p = 1009` (`ℓ ≈ 2^57.9`), seeds 1–4, `moves ∈ {8, 64}`.

## Arms (both on `C` itself, the same seeds)

### IC

`run_end_to_end_with(p, seed, moves, [3,5,7], 3, 1.3609, 1.25, None,
paired = false)`. The route is §18.4's:

- sieve from `m = 9`;
- F4 trace replay;
- no staircase stop;
- `JV_LIN` unset (dense).

Every phase is charged:

- walk to a weak curve;
- transport of `G`, `Q`;
- model change, cover construction and transfer;
- factor base;
- relations (sieve);
- descent (residual stream, six-point tests, F4, linear stage);
- linear algebra (core filter, Wiedemann modulo `ℓ`);
- verification (`[d′]G = Q` on the transported instance and on `C`).

The paired walk-free route of §18.4 is a comparison and is not run.

Each run executes inside its own one-thread rayon pool, so every counted path
runs on the thread that owns the tally. Each run has its own F4 trace scope,
so it starts with no recorded trace (cold). The study's code sets the round
plan with `JV_SIEVE_ROUND_THREADS=4`: rounds are sieved as a four-thread run
sieves them.

### Rho

`jv_rho_strong`, with `r = 256` multipliers and distinguished-point bits 15
at `p = 1511` and 14 at `p = 1009`:

- **R3 (primary reference):** negation map with window-8 cycle detection and
  doubling escape, batched inversion `k = 16`. This is the strong rho.
- **R0:** no negation, one inversion per step. This is the ledger's reference
  walk, the one the `0.010×` row's pooled `1.3609` stands for.

Both run on `C` itself: `C` is moved by `x ↦ x − e₀`, an isomorphism, with
`C`'s `G`, `Q`. The walk seed is `1000·seed + moves` and the run index sets
the walk randomness. The recovered scalar is verified, `[d′]G = Q`, by the
arm's own arithmetic, and that check is charged.

Runs per instance:

- R3: run indices 0 and 1, so 16 runs per `p`;
- R0: run index 0, so 8 runs per `p`.

The rho reference of a row is the mean over all runs of that variant at that
`p`, pooled over the eight challenge curves. Rho is generic and its cost
depends on `ℓ`, not on the curve. The per-curve runs are recorded.

## Instrument (`bitop/v1`, `src/cryptanalysis/jv_bitops.rs`)

The IC arm is counted at the `F_p` level by hooks in the free functions the
whole route bottoms out in:

- `gaudry_cubic::{mm, am, sm}`;
- `residual_walk::{inv_mod, pow_mod}`.

Each operation is classified by its modulus: `p` gives `field_*`, `ℓ` gives
`modr_*`, and any other modulus is tallied apart. Counting also covers:

- F4's row operations, each a multiply-subtract (`field_mul + field_add`);
- F4's inversions;
- normalisation products and merges, interreduction terms and pivot scans;
- per-step structural charges for symbolic preprocessing and the pair
  criteria: per matrix entry 2 probes and 2 words, per column 1 probe, the
  basis scan and the sort;
- the inline negations;
- the hash probes and scans: sieve counters, base tests, `by_x`, duplicate
  removal, the walk's sets and maps, the core filter;
- RNG draws in `F_{p²}` sampling and Wiedemann.

Widths: field primitives at `bits(p)`, and `modr_*` at `bits(ℓ)`.

The rho arm keeps its native arithmetic. Its counts are mapped onto the same
primitives (`jv_rho_strong::rho_tally`):

- every `F_{q³}` product, squaring and inversion at the primitives the
  harness's own `Fq3` executes, counted by the same hooks: 55 `field_mul` and
  44 `field_add` for a product; 150 `field_mul`, 104 `field_add` and 1
  `field_inv` for an inversion;
- the group law's counted `F_p` additions;
- per canonicalisation, the negation it computes plus one compare;
- one `modr_add` per scalar update;
- six `word_op`s per hash of `x`;
- two probes per table operation;
- the collision solve;
- RNG draws;
- the verification.

The legacy "`F_p` multiplication" outputs of both arms are recorded beside
the counts, unchanged.

**Known uncounted** (reported, not priced):

- memory;
- control flow and allocation;
- RNG internals beyond one word per draw;
- the standard-monomial search and monomial bookkeeping of
  `solve_zero_dim`'s linear stage.

No operand re-validation runs in the counted paths, so `BITOPS.md`'s
self-check split does not arise.

## Measures

- Per run: `bitop/v1` total per phase and in all.
- Per row `(p, moves)` against R3 and against R0: the ratio of means
  `IC / rho`, with a 95% percentile-bootstrap interval. The bootstrap uses
  10⁴ resamples and resamples the IC instances and the rho runs
  independently (seed 20261021).
- **§5a band.** Every arm's `word_op` and `hash_probe` work is the native
  work that `bitop/v1` prices by convention rather than by an arithmetic
  formula. It is re-priced at 0, 0.1 and 1 GAE per operation instead, where
  1 GAE is the `bitop/v1` price of one affine addition of the walk: 2
  products, 1 squaring, 1 inversion and 42 `F_p` additions at the harness
  prices. The ratio and its interval are recomputed at each weight.
- **Descriptive only:** wall times, which differ by thread allocation; the
  legacy row's `S/rho` against the pooled 1.3609; rho's legacy `S`.

## Controls (numeric, fixed now)

1. **Positive control, rho.** The mean `steps/√ℓ` over all runs must fall in
   a window around theory:
   - R0: `[0.9, 1.7]`, theory `√(π/2) = 1.25`;
   - R3: `[0.55, 1.25]`, theory `√(π/4) = 0.89`.
2. **A/A, rho.** At each `p`, R3 run index 0 against run index 1, 8 runs
   each. The 95% interval of their ratio of means must contain 1.
3. **A/A, IC.** `p = 1511`, seed 1, `moves = 8` is run a second time in a
   separate process. Every per-phase count and the legacy total must be
   identical.
4. **IC reproduction.** The walk, transport and model legacy counts of every
   IC run must equal those committed in
   `experiments/40_jv_cover_e2e_fast.json` for the same `(p, seed, moves)`.
   This checks that the refactor extracting the challenge keeps the instance
   and the walk. The route's own count is not comparable: §18.4 ran
   multi-threaded with a warm trace cache.
5. **Completeness.** Every IC run must have zero hook calls on unclaimed
   threads and zero operations on an unclassified modulus.
6. **Instrument, by unit tests.**
   - the hooks classify by modulus and count `pow_mod` as executed;
   - the `Fq3` price vectors are 55/44 and 150/104/1;
   - a counted rho run equals the uncounted run and verifies;
   - rho on the challenge curve recovers `d`;
   - a counted end-to-end run keeps its legacy counts, charges every phase
     and is identical across two cold runs.

If a control fails, the verdict is **withheld**, and the numbers are reported
as descriptive.

## Decision rule (primary row: `p = 1511`, `moves = 8`, against R3)

Let `U` be the largest and `L` the smallest 95% bound over four intervals:
the `bitop/v1` ratio and the three band weights.

- **Stays below rho:** `U < 1`.
- **Does not stay below rho:** `L > 1`.
- **Unresolved:** otherwise, or if any run of the row failed (IC not solved
  or not verified on `C`; rho capped, wrong or unverified).

The same rule is applied and reported for every other row, against both R3
and R0. Only the primary row answers the question.

## Lost attempts

- Every attempt that ran is recorded and charged, solved or not. A failed run
  is kept and makes its row unresolved. It is not replaced.
- Algorithmic restarts inside a run are charged inside the run: rho's
  restarts and self-collisions, the sieve's surplus rounds and failed
  linear-algebra attempts, and descent residuals that did not decompose.
- A process killed by the host has no counts and is re-run from the start
  with identical seeds. Every count is a deterministic function of the seeds
  and the code, so the re-run gives the counts the killed run would have
  given. Any such event goes in a deviations log appended below.

## Runtime sizing (from the pilots at `p = 503`, not a study size)

At `p = 503`, single-threaded:

- an IC run took 150 s;
- an R3 run took 21 s;
- an R0 run took 31–40 s.

Rho scales as `√ℓ`, which is 28× larger at `p = 1511` and 8× larger at
`p = 1009`. The expected per-run times are:

| `p` | R3 | R0 |
|--:|--:|--:|
| 1511 | ≈ 10 min | ≈ 17 min |
| 1009 | ≈ 3 min | ≈ 5 min |

With four runs side by side (`--jobs 4`), the rho arms take ≈ 1.5 h of wall
time and the IC arm ≈ 0.5 h, on a host shared with other sessions. The run
counts above are fixed and do not depend on the time a run takes.

## Execution

Runs go through task cards in `cards/` (`tools/task_card.py run`), and the
cards and receipts are committed. The order is:

1. IC at `p = 1511` with the IC A/A;
2. rho R3 and R0 at `p = 1511`;
3. IC at `p = 1009`;
4. rho at `p = 1009`;
5. the analysis.

The analysis is `examples/jv_cover_bitop_analyse.rs`. Raw JSON lines go to
`raw/`, and the analysis to `analysis.json` and `README.md`.

---
## Deviations log (appended after the runs; the text above is unchanged from commit 9a6e3de35)

1. **Host kill, card 2.** The container restarted, probably from an
   out-of-memory kill caused by other sessions' compiles, while card 2 was in
   its second command (R0 at `p = 1511`).
   - Card 2 has no receipt.
   - Its first command, R3 at `p = 1511`, had written all 16 of its runs.
     Each line is written when its run completes, so these are complete
     records, and they are kept in `raw/rho_1511_R3.jsonl`.
   - The killed R0 command had written one of its 8 runs. That partial file
     is set aside in `raw/killed_card2/` and is not used.
   - All 8 R0 runs were re-run from the start by card 2b, with identical
     seeds.
   - Counts are deterministic. The one R0 run the killed attempt completed is
     compared with its re-run: see the README.
2. **Four cards concurrently.** Cards 1–4 were launched together, 16 worker
   threads on a 4-core host shared with other sessions, so wall times are
   inflated and are descriptive only. No count depends on the schedule.
   After the restart, measured runs were also serialised under the shared
   build lock.
