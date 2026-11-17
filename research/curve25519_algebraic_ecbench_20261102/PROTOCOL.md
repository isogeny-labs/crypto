# Algebraic oracle vs generic baseline in ecbench's counted unit — protocol

Frozen 2026-11-02 before any data of this directory. Amendments, if any, are
appended with their reason and never rewrite this text.

## Question

Where does the algebraic oracle (`pkm-semaev` on PKM's isogeny tower) stand
against the generic Curve25519-family index-calculus baseline
(`research/curve25519_ic_baseline_20261102`) and matched rho when all are
charged in ecbench's counted unit, with the oracle's unpriced native work
priced at the AGENTS.md §5a band `w = 0, 0.1, 1` GAE per operation? Until now
`research/curve25519_guessing_20261102` (lead 4) reported F4 multiply-adds
only. This is a distance measurement, not an improvement claim.

## How ecbench charges `pkm-semaev` today (read from source before this protocol)

- Group operations (factor-base build including the auxiliary-curve search,
  the lift that re-adds the summands of a hit, target draws, verification)
  are charged in GAE.
- F4 field multiply-adds are **not** GAE. The oracle reports them as the
  `solver` block (`ops`, unit `F_p multiply-adds (elimination only)`) and
  lists `solver_F_p_multiply-adds_(elimination_only)_uncharged` in
  `cost.unpriced`. `band.rs::unpriced_counts` resolves a `solver_*` entry to
  `solver.ops`, so `ecbench band` already prices it at `w` per multiply-add.
- Two further native counters are listed unpriced and resolved from the
  relations phase: `pkm_system_fp_mults_uncharged` (specialising the system
  at `x_R`, one multiplication a coefficient) and
  `pkm_grid_fp_mults_uncharged` (reading solutions off the grid). Also
  `sqrt_solves`, `row_ops`, `legendre_symbols`, `target_guard_probes`,
  `velu_evaluations` (all `_uncharged`).

Conclusion: F4 work **is** already reported as a native counter and the band
prices it; no harness change is needed for the measurement. (A regression test
that pins this, built from a committed record of this run, is added with the
results.) One unit caveat, stated here once: one F4 multiply-add and one GAE
are different operations; the band does not convert, it brackets (`w = 0`
ignores the work, `w = 1` charges a multiply-add as a full group addition,
which is generous to the algebraic arm, since an F_p multiply-add is far
cheaper than a group addition; `w = 0.1` is the middle of the band).

## Curve, targets, seeds

The k = 20 Curve25519 miniature of the baseline (`curve25519_family spec 20`,
curve 1 of `specs/c25519-k20.json`: `p = 1048573`, `#E = 8r`, `r = 131203`
(17 bits), slug `icv1-fp20-tm1050-4295f05e`), same curve, fresh seeds:
`target_seed = 2026110231` (baseline used 2026110221), `measurement.seed =
2026110232`. 12 public targets, the same for every arm (targets are indexed
by `(curve, target_seed, index)`). The cofactor-1 control curve of the
baseline is not run: this question needs only the miniature. One curve, one
size: nothing below is a statement about other k or curves.

## Arms

| arm | role | method |
|---|---|---|
| `rho-neg` | reference | `rho.negation` |
| `proj-a` | baseline (the Curve25519-family baseline of the ratchet) | `ic.pipeline`, `prime-projected-abscissa:size=32,scan_cap=512`, `mitm:negation_folded=1` |
| `pkm-iso-t6-g4` | candidate | `ic.pipeline`, `prime-isogeny:t=6`, `pkm-semaev:m=2,mode=full,guess=4` |
| `pkm-iso-t6-g4-aa` | A/A control | copy of `pkm-iso-t6-g4` |
| `pkm-iso-t6-g0` | algebraic control | as the candidate with `guess=0`, second session (below) |

`proj-a` is the arm the baseline protocol selected (lowest mean S at w = 0
among its four recipes); it is re-run here on fresh targets so that every arm
sees the same targets in the same session. `pkm-iso-t6-g4` is lead 4's best
measured cell (isogeny form, t = 6, G = 4, 7.1e5 multiply-adds per F4-decided
target at k = 20).

All arms are **full solves**: relation collection, linear algebra, recovered
logarithm checked by ecbench against the planted scalar (`status = verified`).

Session 1 (`spec.json`): 12 targets, 1 warm-up, 2 measured rounds, arms
alternate, timeout 1800 s a solve. Session 2 (`spec_g0.json`): the first 2
targets of the same curve and `target_seed` (so the same two targets as
session 1), `pkm-iso-t6-g0` only, 1 round, no warm-up, timeout 7200 s a solve.
G = 0 runs F4 on the whole tower in one system; lead 4 measured 5.55e7
multiply-adds per target against 7.1e5 at G = 4, about 78 times more, so a
G = 0 solve is expected to take on the order of an hour.

Isolation: **L0, local**, `--cpus none --allow-busy --wait`, on a 4-core box
shared with three other agents. Counted units only; no wall-clock claim
(AGENTS.md §12). `uptime` load at run start and end is recorded in the README.
This is **not** the Iso Lab EKS route: this container has no Iso Lab identity.
The resume checkpoint was pushed to S3 before any run
(`s3://setec-astronomy-artifacts/checkpoints/crypto/leadA-ecbench-algebraic/20261102T165054Z-56c3e282eedd/`).
Because there is no second-host replay and no validator report, **no
leaderboard row and no `benchmarks/README.md` row results** from this
directory (validation gate, AGENTS.md "Standing workflow rules").

Binaries (frozen copies, built from the worktree HEAD 56c3e282ee, never
`target/release`): `/home/user/frozen-bin/ecbench_leadA` sha256
`14ad4a381ee09f025a543bfb77ca3dc8185b596e2d6b40d56e3ab2b63e285c14`;
`curve25519_family_leadA0` sha256
`fc72092faa52e34f6fa1fb5eeea459b466b4b780416c6724724134374e057311`. Spec
hashes: `spec.json` `94fa765243675b36a141b3ee51c2ce3cef28c01e1381194e5a2d40657cb05389`,
`spec_g0.json` `bea9454c80ff937501b4cdb455b6cadedf9d1d2df83f9407b1a63ff381a68e2d`.

## Disclosure: one probe before this protocol

To learn whether a full algebraic solve is feasible at all, one single-target
ecbench run of `pkm-iso-t6-g4` (same curve, a different `target_seed`:
the baseline's, not the fresh one) was made before this text was written. It
verified in 38 s wall and showed the shape of the record (F4 calls, trials,
native counters), including S(w=0) near 6.4 and a solver count near 6.3e8
multiply-adds. It is not part of any table here. The predictions below are
therefore not blind to the order of magnitude; they are stated at the level
of precision the coordinator froze them, plus the extra items marked
"post-probe".

## Preregistered predictions

Notation: `S_X(w)` is the mean over verified solves of counted cost in GAE
plus `w` times the unpriced native operations, divided by `sqrt(r)`;
`ratio_X(w) = S_X(w) / S_proj-a(w)`.

- **P1 (coordinator's prediction).** At `w = 0.1` **and** `w = 1`,
  `ratio_pkm-iso-t6-g4(w) > 100` and `ratio_pkm-iso-t6-g0(w) > 100`: the
  algebraic arms are behind the baseline by more than 100x in S.
  Rough arithmetic behind it: ~7e7 multiply-adds per modelled relation, times
  0.1 GAE, is ~7e6 GAE per relation against ~4.3e3 GAE for a whole baseline
  solve.
  *Falsified* if either algebraic arm has `ratio <= 100` at `w = 0.1` or at
  `w = 1` (each of the four cells is a separate check; P1 as a whole is
  false if any fails, and the table says which).
- **P2 (post-probe; band verdict).** At `w = 0` the candidate's S is below
  the baseline's (the F4 work is invisible there and the arm charges few
  group operations), so `ecbench band` reports the pair
  `proj-a` vs `pkm-iso-t6-g4` as **unresolved inside the band**: the order does
  not hold across `w = 0, 0.1, 1`. *Falsified* if the band reports a
  strict order for that pair.
- **P3 (post-probe; distance to rho).** `rho-neg` is ahead of `pkm-iso-t6-g4`
  at every `w` of the band. *Falsified* if the candidate is below `rho-neg`
  at any `w` (the band verdict is then not "rho-neg < candidate").
- **P4 (G = 0 control; trials are oracle-independent).** The F4 verdict is
  exact (checked against the grid by the oracle's tests), so G = 0 and G = 4
  decide the same trials on the same target when the algorithm seed matches.
  Expectation: `S_g0(w=0)` equals `S_g4(w=0)` on a target to within the
  factor the algorithm seed gives (the relation order is seeded), and the
  solver multiply-adds per trial are 50 to 100 times higher at G = 0.
  *Falsified* if solver multiply-adds per F4 trial (`solver.ops / trials`)
  at G = 0 are less than 20 times those at G = 4.

Accepted outcome of the A/A arm: `pkm-iso-t6-g4` and its copy differ only
through the harness; counts are deterministic given the seeds, so identical
S at all `w` is expected and reported.

## Extrapolation rule (fixed now, used only if a solve fails)

If a `pkm-iso-t6-g0` solve exceeds its timeout, it is reported as a failure,
not dropped, and no S is claimed for it from the run. A *labelled estimate*
of its band cost is then given as follows, and kept apart from measured rows:
take the same target's `pkm-iso-t6-g4` record(s) in session 1; for each,
replace its `solver.ops` by `trials * C0` with `C0 = 5.55e7` multiply-adds per
trial (lead 4's measured mean cost of one G = 0 F4 call at k = 20,
`research/curve25519_guessing_20261102`, isogeny t = 6, G = 0), leaving every
other counter unchanged; average over the recorded measured solves. If a
`pkm-iso-t6-g4` solve itself fails, the same: the arm is reported with its
failure count and the mean over verified solves, flagged as a lower bound.
Nothing else is extrapolated, and no estimate substitutes for a measured row.

## Reporting

README.md: table of S per arm at w = 0, 0.1, 1 (from `ecbench band`),
`distance to rho-neg` (arm S / rho-neg S, the distance remaining, not a
pass/fail gate), `distance of each algebraic arm to the baseline` (arm S /
`proj-a` S), band verdicts, every arm including failures, F4 multiply-adds per
solve and per trial, load average at start and end. One SVG figure from a Rust
or awk/jq pipeline. REPORT.pdf. Every claim scoped to k = 20, these arms, one
curve, L0 local. Raw sessions over 64 KiB are not committed; their sha256 is
listed. Success criterion (CLAUDE.md): the gap is a distance remaining;
re-labelling and accounting changes are not improvements, and this experiment
claims none.

## Amendment 1 (after the first session was started and stopped; before any measured record was read)

Reason: session 1 of `spec.json` was started and killed (by its own PID)
after its warm-up round and the first 2 of 4 warm-up records had completed
(the warm-up records are discarded by ecbench from every table). A
pkm-iso-t6-g4 solve of the warm-up target took 296 s wall and 161,147 F4
calls (about 10,000 trials at 16 branches), against 38 s for the one-target
probe, so the 12-target, 2-round plan (50 algebraic solves) would take
about four hours on this loaded box. I saw the warm-up records' cost line
before deciding: the pkm arm there had S(w=0) = 34.5 (the proj-a warm-up
record 12.6), i.e. above the baseline at w = 0 on that one target; that
already speaks against P2 as written, and P2 is **kept unchanged** and will be
judged on the measured records.

Changes (nothing else): `spec_a1.json` = `spec.json` with 6 targets
(the same first 6 targets of the same `target_seed`), 1 measured round, no
warm-up (counts do not need a warm-up; the A/A arm stays); sha256
`71c30d842f980f85bc6c14b3b0d52680f44c3a6ac1ca6dd7db6560b6efda236c`. `spec_g0_a1.json` = `spec_g0.json` with 1 target (the first
target of the same seed) and timeout 5400 s; sha256 `a88357240450843af37e3717c1325e53b6c79f5e6954217633ec3ca5b8a83c4a`.
Predictions P1 to P3 are unchanged. P4 is replaced: at the observed trial
counts a G = 0 solve costs about 5.55e7 multiply-adds per trial times
thousands of trials, i.e. several hours; a G = 0 solve is expected to hit
the 5400 s timeout, in which case the extrapolation rule above applies and
the per-trial ratio is taken from lead 4's measurement (5.55e7 / 7.1e5 =
78.2 multiply-adds per F4-decided target, G = 0 against G = 4, 100 targets
at k = 20), not from this session. If the G = 0 solve does finish, P4 is
judged as written (`solver.ops / trials` ratio below 20 falsifies it).
With 6 targets the mean S of every arm is noisier; the ratios asked by P1
are of order 10^3 to 10^4 in the probe, far from the threshold, and the
README reports the per-target spread.
