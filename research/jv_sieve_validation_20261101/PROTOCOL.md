# Prospective protocol: corrected full-route Joux–Vitse validation

Frozen on 2026-11-01 before the first datum of this study. The machine-readable
record is [`protocol.json`](protocol.json). This protocol supersedes neither
the 2026-10-19 historical run nor the 2026-10-23 unexecuted weak-curve-only
protocol; it is a new prospective measurement.

## Question and scope

For the deliberately weak Joux–Vitse curve class over `F_(p^6)`, does the
complete implemented route from a non-weak challenge curve remain below a
corrected, strongest-available, equal-advice Pollard rho at
`p ∈ {1009, 1511, 1823}` when both arms are priced in the same `bitop/v1`
primitives and uncertain native work is swept over the required
`0/0.1/1 GAE` band?

This is a class-specific accounting claim, not a general ECDLP break. The
class has full rational 2-torsion and admits a genus-3 cover over `F_(p^2)`;
it does not contain prime-order curves. Joux and Vitse predict an index
calculus advantage on this special extension-field class; Diem supplies the
underlying summation-polynomial framework. The experiment measures the
implemented crossover and does not extrapolate to standard prime curves.

## Prior evidence and why another run is necessary

- `research/jv_sieve_strong_rho_20261019/` omitted the distinguished-point
  hash and two negation-history hashes, had no prospective numeric tolerances,
  did not retain all interrupted work, did not remeasure the isogeny walk, and
  did not emit F4/linear-algebra additions. Its comparison is withheld.
- `research/jv_sieve_strong_rho_v2_20261020/` repaired the rho producer and
  added tolerances, but still excluded the walk and bounded rather than
  measured some additions.
- `research/jv_sieve_prospective_20261023/` froze a useful weak-curve-only
  design but has no committed panel and explicitly excludes the walk.
- `research/ic_acct1_cover_bitop_20261021/` demonstrates the complete
  `jv_bitops` instrument. It is retrospective evidence used to size this
  study, not a source of study data.

## Instances and fixed sample

At each prime, use seeds `33..40`, disjoint from the historical seeds 1..32.
`challenge_instance(p, seed, moves=8, jumps=[3,5,7])` starts from the seeded
weak instance, moves at least eight isogeny steps off the weak locus, and
hands both arms exactly the same public tuple `(C,G,Q,p,l)` and the same
promise that `C` lies in a class containing a weak curve. Instance construction
is excluded from both arms. There is one target per instance.

Each prime is a separate panel with 33 planned attempts:

- 8 complete IC attempts, seeds 33..40;
- 8 strong-rho R3 attempts, run index 0 (the reference);
- 8 R3 attempts, run index 1 (rho A/A);
- 8 R0 attempts (positive control);
- one exact IC A/A repeat, seed 33 in a separate process.

Panels run concurrently only after the frozen-protocol build and unit-test
gate. Within a panel, attempts are submitted in the fixed interleaving
`IC, R3-0, R0, R3-1` by seed, with four independent one-thread processes in
flight. Scheduling affects only diagnostic wall time.

## Arms and equal advice

### IC candidate

`jv_cover_ic_vs_rho_bitops --arm ic`, `moves=8`, dense current linear stage,
one-thread rayon pool, cold per-run F4 trace scope, and
`JV_SIEVE_ROUND_THREADS=4 F4_FP_THREAD_OPS=1`. Every attack phase is charged:

1. isogeny walk from `C` back to a weak curve;
2. transport of `G,Q` and model/cover construction;
3. factor base and relation sieve;
4. descent, including every executed F4 row multiply-subtract as one
   `field_mul + field_add` plus actual inversions and structural probes;
5. linear algebra, including executed Wiedemann/Berlekamp–Massey
   `modr_mul`, `modr_add`, and `modr_inv` operations;
6. scalar verification on the transported instance and original `C`.

### R3 reference

`jv_rho_strong` on `C` itself with negation, window-8 cycle handling,
batched inversion `k=16`, `r=256`, and DP bits 14/15/15 for
`p=1009/1511/1823`. It receives the same public instance and no retained
precomputation, secret, walk path, factor base, or relation advice. This is the
strongest available rho because the challenge curves have no additional
usable automorphism beyond negation; the instance symmetry check remains part
of validation.

The corrected producer must satisfy the exact accounting identity
`hash = steps + 3*dp_check`: one branch hash for every attempted step and,
for every completed negation step, two history hashes plus the
distinguished-point hash. The collision phase separately prices the DP test
and table probes.

### R0 positive control

The same rho without negation and with one inversion per step. Its exact hash
identity is `hash = steps + dp_check`.

## Instrument and prices

The primary metric is counted `bitop/v1`; wall time is descriptive. Field
primitives use `bits(p)`, subgroup primitives use `bits(l)`. The producer
emits every primitive by phase, raw rho operation counts, answers, scalar
checks, walk length, and diagnostic wall time.

The native-work sensitivity band removes the conventional `bitop/v1` price
of every `word_op` and `hash_probe`, then re-prices each at
`w ∈ {0,0.1,1}` GAE. One GAE is the measured rho price of an affine group
addition: two `F_(p^6)` products, one square, one inversion, and 42 `F_p`
additions. Both arms are re-priced by the identical formula.

## Prospective controls and numeric tolerances

Every item is fixed before data. A failed control withholds the affected
prime; thresholds do not move.

1. **Correctness:** every IC and rho scalar equals the planted logarithm and
   verifies on `C`; no rho is capped.
2. **Attempt census:** exactly 33 status records per panel. Every process and
   malformed/missing output is retained. A failed scientific attempt is not
   replaced. Infrastructure retries are disabled for this study.
3. **Counter completeness:** every IC row has zero foreign hooks, zero
   attack-phase operations at an unclassified modulus, and zero unattributed
   bitops.
4. **Walk/F4/LA presence:** every IC row has a nonzero charged walk and path,
   nonzero descent `field_mul` and `field_add`, and nonzero linear-algebra
   `modr_mul` and `modr_add`.
5. **Corrected hashes:** the exact R3 and R0 identities above hold for every
   rho row.
6. **IC A/A:** all non-wall producer fields for the seed-33 repeat are
   bit-identical to the first attempt.
7. **R3 A/A:** at each prime, the paired ratio of means
   `bitops(run1)/bitops(run0)` has point estimate in `[0.50,2.00]` and its
   95% paired-bootstrap interval contains 1. This deliberately broad window
   was chosen from the pre-existing 8-per-arm intervals in the 2026-10-21
   study, before seeing these seeds.
8. **Positive rho:** at each prime, mean `steps/sqrt(l)` is in `[0.90,1.70]`
   for R0 and `[0.55,1.30]` for R3. The theory anchors are approximately
   1.25 and 0.89 respectively.

No synthetic negative population is meaningful for this deterministic
algorithm comparison; the prospective R3 A/A is the null comparison.

## Analysis and decision

At each prime, pair IC and R3-run0 by seed. Compute the ratio of means with a
95% paired percentile-bootstrap interval (10,000 resamples, fixed derived
seeds in `jv_validation_analyse.rs`) for native `bitop/v1` and each of the
three GAE weights.

- **Below strong rho:** every control passes and all four upper bounds are
  below 1.
- **Above strong rho:** every control passes and all four lower bounds are
  above 1.
- **Unresolved:** controls pass but neither condition holds.
- **Withheld:** an attempt, correctness check, accounting check, or control
  fails or is missing.

Each prime is judged separately. A cross-prime statement is licensed only if
all three prime panels receive the same non-withheld verdict. No exponent is
fit from three points.

## Failures, replay, and validation gate

`run_panel.sh` creates the complete plan before execution and writes one raw,
stdout, stderr, and status artifact per attempt. Internal restarts, surplus
sieve rounds, failed residual decompositions, failed linear systems, rho
cycles, and restarts are already charged in the row. Process failures retain
their partial artifacts and invalidate rather than silently shrink the panel.

After producer panels finish, clean workflow jobs replay IC seed 33 and R3
seed 33/run0 for every prime. Excluding diagnostic `wall_ms`, every producer
field must be bit-identical and the replayed scalar must verify. A distinct
verifier implementation reparses raw data and recomputes all four headline
intervals to relative tolerance `1e-12` without calling the producer analysis.

`VALIDATION.json` remains `pending` until it names nonempty, passing replay
certificates and validator reports with SHA-256 digests, and records R3 as the
same-binary, same-unit, equal-advice strongest rho with the `0/0.1/1` band.
Only a complete gate may move a leaderboard or benchmark row.

## Inadmissible interpretations

- Generalising to random or prime-order curves, standardised curves, or other
  field families.
- Dropping failed/capped/incorrect attempts, changing seeds or tolerances, or
  substituting wall time for counted work after launch.
- Treating the uncharged construction of the planted challenge as an attack,
  or omitting the charged attack walk back to the weak class.
- Claiming validation from producer analysis alone or populating
  `VALIDATION.json` before independent replay and recomputation pass.
