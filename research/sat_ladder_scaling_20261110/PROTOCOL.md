# F4: SAT on the bit-level x-only ladder — conflict-count scaling and clause reuse (protocol, frozen 2026-11-10)

Moonshot M2 (main session) and A1 (`research/prime_moonshot_20261110/ideas-generator.md`).
Prior art: `research/prime_field_smt_20261005` (cvc5 `QF_BV`/`QF_FF`: `unknown` at 20 bits in
120 s on the decomposition system), `research/pdp_ml_sat_ceiling_20261027`. This study measures
the **scaling exponent** of a CDCL solver on the *ladder* formulation (the scalar's bits are the
only free inputs), which no prior study measured, and whether learned clauses amortise across
targets (advice; falls under `S·T² ≳ n`). Class: boundary diagnostic.

## 1. Encoding
Curve `E/F_p` of prime order `r`, `n = ⌈log₂ p⌉ ∈ {10, 12, 14, 16, 18, 20}`, one prime-order curve
per `n` (`p` = largest prime below `2^n`, `(a,b)` from seed `F4/curve/<n>`), `G` fixed.
Unknown: `k` as `⌈log₂ r⌉` Boolean variables. Circuit: projective x-only Montgomery ladder on
`(X : Z)` from `x(G)`, each step the standard differential add + double (`≈ 10` modular
multiplications), Tseitin-encoded with schoolbook multipliers, adders, and modular reduction by
quotient variables (`prod = q·p + rem`, `rem < p`); final constraint `X ≡ x_Q · Z (mod p)`,
`Z ≠ 0`. Every SAT model is decoded and checked: `[k]G = Q` exactly (fatal otherwise).

## 2. Solver and arms
CaDiCaL (vendored through a Rust crate; fallback `varisat`, recorded) with default options.
- **Arm FRESH:** new solver per target.
- **Arm REUSE:** one incremental solver per curve, target `x_Q` bits as assumptions, 20 targets in
  a seeded random order; learned clauses persist.
20 targets per `n` (seed `F4/target/<n>/<i>`, `k` uniform in `[1, r−1]`). Conflict budget
`10^7` per solve (censored rows are reported as censored, never imputed). Metrics: **conflicts**
(primary), decisions, propagations; wall seconds descriptive only (the host is shared).

## 3. Predictions (frozen)
P1. Exponent `α` of median FRESH conflicts in `n` (`log₂ conflicts ≈ α n + c`, fitted on the
uncensored `n`): `α ∈ [0.85, 1.15]` (exhaustive-like); rho is `0.5`.
P2. REUSE / FRESH median conflict ratio at each `n` ≥ 0.5.
P3. All models decode to the true `k`.
Prior that `α` upper CI `< 0.6`: 0.01.

## 4. Decision rules
1. **Instrument failure:** P3 fails, or fewer than three `n` are uncensored.
2. **Lead alive:** `α` upper 95 % CI `< 0.6` with `n = 18` or `20` uncensored, **or** REUSE ratio
   `< 0.2` at two consecutive `n`. Then replicate with a second solver and 40 targets.
3. **Closed:** otherwise. Reading: CDCL on the ladder circuit is exhaustive-like and clause reuse
   does not amortise below the generic bound at these sizes.

## 5. Route and outputs
Local host; counts are the result. `results/run.json`, README, SVG (log₂ conflicts vs `n`, both
arms, censoring marked), REPORT.pdf, session-log entry. Task card after `FREEZE.sha256`.

## 6. Amendment and disclosure before the full run (2026-11-10)
Predictions and rules were committed (`c8bbc80e1a`) before the tool existed. The implementer's
development runs (not committed, not scored): at `n = 4, 5, 6` with 3 targets, FRESH median
conflicts 6,222 / 9,856 / 185,353 and REUSE 315 / 1,024 / 52,295 (ratios 0.051 / 0.104 / 0.282),
`α_FRESH ≈ 2.45`; at `n = 10, 12` every solve was censored at 2·10⁴ and a single `n = 10` target
did not finish at 2·10⁵ conflicts. Conflicts grow about 5× per bit, so the frozen grid
`n ∈ {10, …, 20}` at 10⁷ conflicts would be censored throughout and trip rule 1 by construction:
a grid-design error, not a measurement. **Amendment (grid and budget only):** `n ∈ {4, 5, 6, 7,
8, 9}`, 10 targets per `n`, conflict budget `4·10⁶`, `--threads 2`; one prime-order curve per `n`
as before. P1–P3 and rules 1–3 are unchanged (rule 1's "fewer than three `n` uncensored" now
refers to this grid; rule 2's thresholds are unchanged, and if its clause-reuse clause fires on
these sizes the replication it prescribes is run and its reading stated with `α_REUSE`). Instrument
facts fixed by the tool: CaDiCaL 3.0.1 via `cadical-sys 0.8.0`; propagations are
`propagations.search`; x-only decoding accepts `k' ∈ {k, r − k}` (P3 reads `x([k']G) = x_Q`);
the reduction is `prod + q·c = q·2^n + rem`, `c = 2^n − p`, `rem < p` as no carry out of
`rem + c`; the ladder starts from `(O, G)` over all `nk` bits with `k < r` as a comparator;
7 tests pass and the `n = 6` planted test is `#[ignore]`d (≈ 3·10⁵ conflicts). Expectation stated
before the run: `α_FRESH` well above 1 and P1 failing upward; the amended grid is for measuring
the exponent, not for reaching rho.

## 7. Replication prescribed by rule 2 (preregistered 2026-11-11, after F4-RUN-02, before F4-REP-01)

**Trigger.** F4-RUN-02 (CaDiCaL 3.0.1, `n ∈ {4..9}`, 10 targets) gave REUSE/FRESH median ratios
0.005, 0.003, 0.005, 0.003 at `n = 4..7`: rule 2's clause-reuse clause fires, so "replicate with a
second solver and 40 targets". The per-order decomposition of run-02 (first REUSE solve ≈ FRESH
cost; later solves near zero at `n ≤ 7`, decaying at `n = 8`, gone at `n = 9`) was read **after**
the data and is disclosed here as the motivation for the extra observables below; it does not
change rules 1–3.

**Second solver (fixed).** BatSat `=0.6.0`, a pure-Rust port of the MiniSat 2.2 core: a different
CDCL lineage from CaDiCaL, incremental under assumptions (`solve_limited`), per-solve
`conflicts`/`decisions`/`propagations` as deltas. Budget: BatSat has no settable conflict budget,
so the tool stops the solve through the `Callbacks::stop()` hook once a learnt-clause counter,
reset per solve, reaches the budget; the solve then returns UNDEF and is recorded as **censored**
(never imputed). The solver's own conflict counter is reported and may exceed the budget by the
conflicts of the chain that ends before the next poll. `propagations` is not the same counter as
CaDiCaL's `propagations.search` and is **not compared across solvers**; conflicts are compared
in meaning only (one BCP conflict each). `--solver cadical` leaves the run-02 path byte-identical
on the rows (checked on `n ∈ {4,5}`, 4 targets, against the frozen run-02 source).

**Design (fixed).**
- Arm pair as in §2 (FRESH: new solver per target; REUSE: one incremental solver per curve,
  `x_Q` bits as assumptions, targets in the seeded order), **40 targets per `n`**, seeds
  `F4/rep/target/<n>/<i>`; curve and order seeds unchanged (`F4/curve/<n>`, `F4/order/<n>`), so
  the curves are those of run-02.
- Grid `n ∈ {4, 5, 6, 7, 8}` for BatSat; `n = 9` is excluded because run-02's FRESH arm was 80 %
  censored there (no ratio is defined and it would dominate the cost). Budget `4·10⁶`, `--threads 2`.
- Control: CaDiCaL at the **same 40 seeds** on `n ∈ {4, 5, 6, 7}` (same-target solver comparison).
- New observables (both solvers): per `n`, the REUSE arm's **first-solve** conflicts
  (`solve_order = 0`) and **warm median** (`solve_order ≥ 1`); `α_REUSE_warm` fitted like
  `α_FRESH` on the warm medians over the `n` where every warm solve finished (`log₂ max(median, 1)`,
  the floor only matters for zero medians and is stated when used).

**Predictions (fixed before the run).**
- R1. With BatSat the REUSE/FRESH median ratio is `< 0.2` at two consecutive `n ∈ {4..7}` (the
  effect is solver-generic). Prior 0.8.
- R2. The first REUSE solve costs within a factor 3 of the FRESH median at every uncensored `n`
  (the saving is after a set-up at full cost). Prior 0.8.
- R3. `α_REUSE_warm` upper 95 % CI `≥ 0.6` in BatSat and in the CaDiCaL control (warm per-target
  cost is not sub-generic). Prior 0.9.
- P3 (all found models decode to `k' ∈ {k, r−k}`) holds for both solvers.

**Decision rules for the replication (fixed).**
- **Instrument failure:** P3 fails for either solver, or fewer than three `n` have every warm
  REUSE solve uncensored in the BatSat run.
- **Lead alive after replication:** R1 holds **and** `α_REUSE_warm` upper 95 % CI `< 0.6` on at
  least three uncensored `n` in the BatSat run **and** in the CaDiCaL control. Then a third
  replication on larger curves is designed before anything is claimed.
- **Closed:** otherwise. Reading to be stated with `α_REUSE_warm` and R2: clause reuse amortises a
  one-off set-up whose cost is the FRESH solve; the per-target warm cost scales exhaustive-like
  or worse in `n`, so incremental CDCL on the ladder circuit does not amortise below the generic
  bound at these sizes. If R1 fails with BatSat, the run-02 ratios are additionally recorded as a
  CaDiCaL-specific effect (inprocessing/phase behaviour), not a property of the circuit.

**Freeze and card.** The run-02 freeze is kept as `FREEZE-run02.sha256`; `FREEZE.sha256` now binds
this protocol (with §7), `tool/src/main.rs`, `tool/Cargo.toml` and `tool/Cargo.lock` of the
two-solver tool. Card `F4-REP-01` runs the BatSat grid, then the CaDiCaL control, writing
`results/rep-batsat.json` and `results/rep-cadical.json`, after `FREEZE.sha256`. Local host,
shared with the F2/F3 cards; wall time descriptive only. Expected cost: hours (BatSat ≈ 2·10⁴
conflicts/s in development runs, so a censored solve is ≈ 200 s; `n = 8` dominates).
