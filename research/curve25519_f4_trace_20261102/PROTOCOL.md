# F4 trace ("learn once, apply many") for `pkm-semaev` — protocol

Frozen 2026-11-02, before any data in this directory. Lead C. Branch
`leadC-f4-trace`, based on main 56c3e282ee. Scope of every claim made from
this protocol: the cells listed below (k = 20 miniature, isogeny form, m = 2,
mode full, 100 subgroup targets per cell). Nothing is claimed for other k, m,
t, G, the bits form, or for anything outside the F4 multiply-add unit.

## Question

Within a hybrid-guessing branch the system has the same shape for every
target; only `x_R` (a specialised coefficient) changes. Can the F4 work be
shared across targets?

## Design (chosen after reading `f4_fp_tower.rs` and `pkm_semaev.rs`)

**Learn once per branch system, replay per target, accept only refutations,
fall back to plain F4 otherwise.** Implemented behind `trace=1`.

Why this design and not the alternatives:

- *Verdict structure.* Almost every target is a refutation (`1` in the
  ideal). A row of any F4 matrix lies in the ideal, so if a replay produces
  `1` the target has no solution over the algebraic closure, **whatever the
  trace was learned on**. A refutation by replay is therefore sound without
  checking that anything is a Gröbner basis. A replay that does not refute
  decides nothing and the target goes to plain F4 (then the grid, as before),
  so verdicts equal plain F4's by construction; the tests check it anyway.
  This is why "verify the result is a Gröbner basis" is not needed: the
  trace never has to certify a basis, only a unit.
- *Learning.* Plain F4 with recording on a generic `x_R` (a fixed splitmix
  stream, at most 8 tries, until one refutes). Recorded per F4 step: the
  S-rows (`(multiplier, basis element)`) whose residue gave a new pivot, the
  S-rows promoted to reducers, and the sorted leading monomials of the new
  elements. This is the usual F4 trace (useful rows per step), with the
  generic specialisation playing the role of the "random prime/point".
- *Replay.* Same pair selection (deterministic given the leads), but only the
  recorded S-rows are formed, and `B'` (the reduced reducer block) is formed
  only for the reducers those rows reach, directly or through other
  reducers. A replay stops with a failure at the first step whose new leads
  differ from the trace's (unless it has already produced `1`), so a
  degenerate specialisation (typically a target that is decomposable, whose
  system is consistent) leaves early rather than running to the end.
- *Alternative rejected: `x_R` as a variable.* The F4 would run over
  `F_p[x_R]` (or the tower ring with a free variable): coefficients become
  polynomials in `x_R`, the engine here is `F_p`-only with `u32` coefficients
  (`p < 2^31`) and the number of free variables is capped; the specialisation
  already exists (`specialise_branch`). Rejected as a larger change with no
  way to charge in the existing unit.
- *Honest caveat.* The pruning of unreached reducers in `B'` is only
  applied in a replay (it follows from dropping S-rows); plain F4 is
  unchanged (`trace=0` replays lead 4 byte for byte in multiply-adds).

## Accounting (unit: F4 elimination multiply-adds, the oracle's unit)

- **Learn cost**: all multiply-adds of the learning runs (every try), charged
  once per branch system shape (`2^G` shapes), at `prepare`, reported as
  setup native work `pkm_trace_learn_muladds`. Not in `f4_muladds`.
- **Apply cost**: multiply-adds of replays, every replay attempt, refuted or
  failed (`pkm_trace_replay_muladds`).
- **Fallback cost**: multiply-adds of the plain F4 run after a failed replay
  (`pkm_trace_fallback_plain_muladds`). The failed replay's own work stays in
  apply. `f4_muladds` (the solver ledger) = apply + fallback exactly (asserted
  in a test).
- Not charged (as in lead 4, listed in `native`): specialisation of the
  system, symbolic preprocessing and row formation, pair bookkeeping, the
  grid reading after a consistent system. Under AGENTS.md §5a any
  cross-method comparison prices them at 0 / 0.1 / 1 GAE. **This experiment
  compares two arms of one oracle and makes no cross-method claim and no
  distance to rho (lead A handles that).** Because the replay forms rows
  anyway, wall time is not a measured claim and is not compared (shared
  machine).

## Cells

`curve25519_family f4cost 20 isogeny <t> <G> 100 <seed> 600 2 full 0 <trace>`
(trace is the new argument at position 12), `t in {6, 7, 8}`,
`G in {0, 2, 4}`, `trace in {0, 1}`, seed `2026111400 + t`, 100 targets,
cap 600 s per cell, sequential, each under `ulimit -v 3500000` and
`timeout 900`. 18 cells. `trace=0` cells replay lead 4's G = 0, 2, 4 cells
(same seeds, same targets, same binary lineage); the trace arm sees the same
100 targets. `uptime` is recorded before each cell; a cell that hits the cap
is flagged and its sample is the targets finished (the two arms of a capped
pair are then not paired target for target; they are compared on the
targets-done they report).

## Metrics per (t, G)

1. learn cost `L` (multiply-adds, once); learn runs and branches learned;
2. apply cost per target (replay multiply-adds / targets) and per replay;
3. fallback rate = fallbacks / replays (branch level), and fallbacks per
   target;
4. total per target without learning (`f4_muladds / targets`), and amortised
   over the 100 targets, `(L + f4_muladds) / targets`;
5. plain per target `P` (the `trace=0` cell), the ratio apply / plain, and
   the break-even target count `n* = L / (P - T)` where `T` is the trace
   arm's per-target total without learning (reported "never" if `T >= P`);
6. per relation on `coverage_exact`: amortised per target / `coverage_exact`
   for the trace arm, and plain per target / `coverage_exact` for the plain
   arm. Hits and verified hits per arm; they must be equal between arms.

## Frozen predictions and falsifiers

- **P1 (apply cost).** At G = 0, apply cost per target is at most 0.25 x the
  plain F4 cost per target, at each of t = 6, 7, 8. *Falsified if* any of the
  three ratios exceeds 0.25.
- **P2 (saving shrinks with G).** The ratio (apply cost per target / plain
  per target) is larger at G = 4 than at G = 0 at each t (branch systems
  become small, so there is less to drop and fixed work dominates).
  *Falsified if* at any t the G = 4 ratio is not larger than the G = 0
  ratio. (G = 2 is reported; the claim is the two ends.)
- **P3 (fallbacks).** The fallback rate (fallbacks / replays) is at most
  10 % in every trace cell (fallbacks should come from consistent systems,
  i.e. hits, and degenerate specialisations, which are rare for `p ~ 2^20`).
  *Falsified if* any cell exceeds 10 %.
- **P4 (verdicts).** Hits and verified hits of the trace arm equal those of
  the plain arm in every cell pair. *Falsified by one difference.*
- **P5 (break-even).** Learning pays back within 5 targets in every cell
  where the apply cost is below the plain cost (learn is about one plain run
  per branch). *Falsified if* any such cell has `n* > 5`.

Classification: a claim is judged on the measured ratios; a pair with a
capped cell is reported as inconclusive for that cell. A prediction cannot be
repaired after data; the outcomes are reported as measured, including the
false ones.

## Tests (before data)

`cargo test --release --lib pkm_semaev` (all, including the existing ones),
with the new `trace_agrees_with_plain_f4_and_grid`: trace verdicts equal
plain-F4 verdicts and the grid verdict and the exact decomposable set on
random and planted targets (isogeny G = 0, 1, 2 and bits G = 1), relations sum
to the target, zero crosscheck disagreements, the fallback is exercised on
the planted targets and charged, and the ledger identity holds.
`cargo clippy --release --lib --bin curve25519_family -- -D warnings`;
rustfmt on the changed files.
