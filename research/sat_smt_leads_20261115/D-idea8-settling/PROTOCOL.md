# Lead D: settling run for target-parametric incremental SAT (idea 8)

Written 2026-11-15 before any measured run of this directory. Executed before this file and
nothing else: the exhaustive label validation at (l,n)=(3,11) for the three V seeds
(`validate.py`, verdicts only, no cost metric), and calibration smokes drawn with target-seed
offset 99 (the measured targets use offset 1): 6 targets at l=5 seed 0, 9 targets (fresh arms
only) at l=5 seeds 1 and 2, 30/12/12 targets at l=4. The smokes were used only to size the run
(seconds per solve) and are not analysed; they are in the scratch directory. There is no version
control in this workspace; `PREREG.sha256` records the hashes of this file and of the code before
the first measured run. Everything produced is a pilot-scale assertion, tagged [pcc:unverified].

## Claim under test

m=3 point decomposition on y^2+xy=x^3+1 over F_{2^n}, factor base = rational points with x in a
random l-dimensional F_2-subspace V. One CNF with the target abscissa as n assumption bits. Claim:
one incremental CDCL solver (INC) that solves a long sequence of never-seen targets does less
work per target, counted in propagations, than a fresh solver on the target-specialised formula
(SPEC), once the sequence is long.

## Instances

- Generator: `pdp.py`, copied from `../../sat_smt_leads_20261114/followup-08-powered/pdp.py`
  (x=0 defect already fixed there). One addition, on in every arm: per summand, one clause for
  each nonzero x in V that does not lift to a rational point (the rationality constraint the
  powered run's skeptic found missing). Consequence known from the smokes: instances are several
  times cheaper than in the powered run, so numbers here are NOT comparable with its numbers.
- Cells: l=5, n=17 (primary) and l=4, n=13 (l-trend). l=3 is used for validation only.
- Seeds: master 2026111508. V seed = master + 1000 l + 10 s for s = 0, 1, 2; target seed = V seed
  + 1; shuffle seed = V seed + 2. Three independent V per l, one target order per V. Seeds are not
  changed after seeing data. The number of rational base points differs between V (smokes: 34, 30,
  28 signed points at l=5; 8, 18, 12 at l=4); it is reported per seed and is a covariate, not a
  selection criterion.
- Planned sequence per V: 1,000 distinct targets. l=5: 940 non-decomposable + 60 planted
  decomposable; l=4: 985 + 15 (planted count capped at half the exact number of decomposable
  abscissae of that V). Non-decomposable = random rational point, x(R) not in V, verified by the
  real-point 2+1 meet-in-the-middle. One seeded shuffle. No target repeats within a sequence.
- Table-exhaustion control: reported per seed: targets solved by INC, decomposable targets among
  them, exact number of distinct decomposable abscissae outside V, number of non-decomposable
  rational abscissae outside V. Every evaluation is on non-decomposable targets INC has not seen.

## Arms (CaDiCaL 1.5.3 through python-sat, default options, one core)

- INC: one solver for the whole sequence, parametric CNF, target by assumptions. Treatment.
- SPEC: new solver per target, CNF regenerated with the target as a constant. The single
  pre-specified fresh arm.
- FP (FRESH-PARAM): new solver per target, the same parametric CNF and the same assumptions as
  INC. This is the emulation of INC-DROP: python-sat exposes no call that deletes CaDiCaL's learnt
  clauses. It removes learnt clauses AND the inprocessed formula AND the scheduler state, so
  INC/FP bounds the total effect of everything carried between targets; it does not separate
  learnt-clause reuse from amortised inprocessing. Same formula as INC, so the formula difference
  is removed.
- AA: SPEC with its clause list shuffled (per-target seed). A/A arm for SPEC.

## Execution and the length actually run

One process at a time, one core, `nice`, each process under 10 minutes. An INC sequence cannot
be checkpointed across processes (python-sat cannot serialise solver state), so each INC sequence
is one process (`run.py l s A`) that stops at a 565 s wall guard. From the smokes (about 0.75 s
per l=5 target on seed 0) the guard is expected to stop l=5 before 1,000 targets. Rule fixed now:
the analysed l=5 sequence length N* is the minimum over the three seeds of the number of targets
INC completed; all three seeds are analysed on positions 0..N*-1 (equal counts per seed, no seed
dropped); rows beyond N* stay in the raw file and are not analysed. N* depends on machine speed
only. The fresh arms then run in separate processes on T[0..N*-1] (`BL` last third, `BE` earlier).
At l=4 everything runs in one process per seed with all 1,000 targets.

Fresh-arm schedule (`common.schedule`), fixed by position, never by outcome. Last third =
positions idx >= floor(2N*/3). l=5: SPEC on every non-decomposable target of the last third and
every 2nd earlier one; FP on every 2nd of the last third and every 4th earlier one; AA on every
2nd of the last third. l=4: SPEC and FP on every non-decomposable target, AA on the last third.

Counters (propagations, conflicts, decisions) are deterministic and are taken as deltas of
`accum_stats` around each INC solve. Wall time per solve (solver only; CNF build and load
recorded separately) is recorded but INC and the fresh arms run in different processes on a
shared, loaded machine at l=5, so wall time is indicative only. Every verdict is compared with
ground truth and every SAT model is lifted to curve points; mismatches are counted and reported.

## Decision rule (fixed)

Per-target ratio rho_i = INC_i / SPEC_i in propagations.

PRIMARY: geometric mean of rho_i over the non-decomposable targets in the last third of each
l=5 sequence, pooled over the three seeds (all targets weighted equally). 95% interval:
two-stage percentile bootstrap, 10,000 resamples, numpy `default_rng(1508)`: resample the three
seeds with replacement, then the targets within each drawn seed.
- ALIVE if the upper bound is below 0.9.
- KILL if the lower bound is above 1.0.
- INCONCLUSIVE otherwise.
The verdict is this rule and nothing else. Known weakness, stated now: three seeds give a coarse
seed-level bootstrap (10 distinct seed multisets); the per-seed geometric means are tabulated
next to the pooled value.

SECONDARY (reported, no verdict attached):
1. Power law: OLS of ln rho_i on ln p, p = idx + 1, over non-decomposable targets with idx >= 1
   that have a SPEC solve, pooled over seeds, ln rho = a - beta ln p. Interval for a, beta by the
   same two-stage bootstrap (seed 1509). Crossing points: p(1.0) = exp(a/beta), p(0.5) =
   exp((a + ln 2)/beta) with bootstrap interval; "never" if beta <= 0 (or already below at p=1).
   Also per seed. The fit is an extrapolation outside the measured positions.
2. Mechanism: the same statistic and fit for INC/FP (propagations). Reading: if INC/FP in the
   last third is not below 1 beyond the A/A spread, nothing carried between targets helps; if it
   is, the carried state helps, cause not separated (see FP above). Also FP/SPEC, the cost of the
   parametric formula with nothing carried.
3. l=4: primary statistic and fit, same definitions; plus the l=4 statistic on the position
   window equal to the l=5 last third (matched positions). l-trend = ln GM(l=5) - ln GM(l=4) on
   matched positions, independent two-stage resampling per cell (seed 1510), and beta(l=5) -
   beta(l=4) with the l=4 fit restricted to p <= N*(l=5).
4. A/A: geometric mean and sd of ln(AA/SPEC) propagations, last third.
5. Conflicts and wall time: the primary statistic with conflicts and with solver seconds.
6. Next to every table: the linear 2+1 scan reference, 2^l leaf solves per target (32 at l=5, 16
   at l=4), with the median propagations and conflicts per target of each arm. Units differ; no
   ratio is formed.

## Prediction (before data)

The propagation ratio in the last third at l=5 is above 1 (INC does more propagations than SPEC),
beta is positive and small (below 0.15), and the fitted crossing of 1.0 lies beyond the sequence.

## Not covered

l >= 6; m = 4; other curves; more than three V per l; other target orders; other solvers; native
XOR; a true INC-DROP (learnt clauses deleted inside one solver); 1,000 targets at l=5 if the wall
guard stops earlier; wall time on an isolated core; common units with the 2+1 scan.
