# Protocol: graph index calculus and non-subfield "smallness" over prime fields

**Version.** v1, 2026-10-23. Frozen when it is committed, before the first
measured run. The run records carry this file's SHA-256 (`results/MANIFEST.json`).
A change after that point is a new, dated version, and both versions are reported.

**Harness.** `examples/prime_smallness_graph_probe.rs` (native Rust, one binary
for every arm and for the rho reference).

**Status.** Pilot on small public synthetic known-answer instances. No
deployed-curve claim, no asymptotic sub-rho claim, no leaderboard row. The
validation gate (`docs/VALIDATION_GATE.md`) is not attempted here. If arm G's
large-prime variation is to be promoted as an IC baseline improvement, that
happens in a follow-on PR with a replay certificate and a validator report.

## 0. Questions

1. **Graph view (arm G).** Can relation finding be recast as a graph problem?
   Here a partial decomposition `R = ±F_i + T` is an edge between the target
   and the vertex `x(T)`, and full relations come from closed cycles. If so,
   does that lower the end-to-end cost of prime-field index calculus, and how
   close does it get to Pollard rho?
2. **Lattice smallness (arm L).** Every `x ∈ F_p` can be written as `a/b` with
   `|a|, |b| ≈ √p`. Does a factor base of points whose `x ≡ a/b` has *both*
   parts small give the Coppersmith small-root method more reach on `S₃` than
   the interval factor base `x ≤ B`, at equal factor-base size? The interval
   reach is about `δ ≈ 1/6` (`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`
   §2.2, §3.2–3.3).
3. **Fake subfields (arm F).** For subsets of `F_p` of size `B` used as
   factor bases (interval, multiplicative subgroup `μ_B`, small fractions),
   is it cheaper to decide `S₃(x₁, x₂, x_R) = 0, L(x₁) = L(x₂) = 0` with the
   owner's F4 (`src/cryptanalysis/f4_fp.rs`) than by direct-subtraction
   enumeration? Program note §2.3 and §4.2 argue that it is not; §4.6 closes
   the subgroup case on paper. This arm measures the claim with the owner's
   engine and also reports §4.2's falsifier, the solution count over `F̄_p`.
4. **Transfer (arm T): not run here.** Isogeny and cover transfers are the
   subject of the Tier 2 thread and the volcano tooling
   (`RESEARCH_ISOGENY_FIBERS_ECDLP.md`, the `isogeny-volcano` skill). This
   protocol makes no statement about them.

## 1. Claim classes (controlled-experiment §1)

| statement | class |
|---|---|
| Arm L determinant identity: at every level the bihomogeneous lattice has the same Howgrave-Graham determinant condition as the interval lattice, in terms of factor-base size | theorem (derivation in §3.2) |
| Arm G cost model `C_rel ≈ p·√(B/(2ℓ))`, floor `≈ ½√(pB)` at `ℓ → p` | heuristic estimate, tested |
| "graph IC-2 is cheaper than IC-2" / "is (not) below rho" | comparative, decided by §3.1 rules |
| "fraction lattice has (no) more reach" | comparative, decided by §3.2 rules |
| "F4 on fake subfields is (not) sub-enumeration" | comparative, decided by §3.3 rules |
| anything about P-256 or other deployed curves | out of scope: not established |

## 2. Instruments, curves, seeds

- **Curves.** Arms G and L use `prime_fast::find_a3_curve(bits, 3)`: the
  largest prime `p ≡ 3 (mod 4)` below `2^bits`, `a = −3`, smallest `b` that
  gives prime order. Arm F needs `μ_B ⊂ F_p^*` for `B | 2^10`, so it uses
  `find_a3_curve_2adic(24, 10)` (same rule with `p ≡ 1 (mod 2^10)`). Every
  curve is written to `results/curves.json` with its ICV1 slug computed in
  Rust, and registered (`scripts/build_curve_registry.py`) in this PR.
- **Seeds.** Every row records its seed. Arm G target `t` at `bits`: seed
  `0x6A110000 + 1000·bits + t`, which plants `k` and `Q = [k]G`. The same
  targets feed every G cell and the rho reference. Arm L cell seed:
  `0x1A77 + 1000·bits + 100·level + round(100·δ)`, shared by both kinds at a
  cell. Arm F: `0xF4F40000 + 100·B + s`.
- **Threads.** The host has 2 hardware threads. Rho uses
  `RhoConfig::for_bits(bits, true)` capped at 2 threads. The G, L and F arms
  are single-threaded, apart from F4's internal rayon.

## 3. Arms

### 3.1 Arm G: large-prime graph IC-2

**Method.** This is direct-subtraction IC-2 (the `prime_fast::collect_relations`
sweep). Factor base: the `B` points of smallest `x` with `y ≤ (p−1)/2`.
Targets `R = aG + bQ` come from an r-adding walk: `R ← R + M_{h(R)}`, with
32 table points `M_j = u_jG + v_jQ`, restarted from a fresh random combine
every 1024 steps. For each `F_i`, both candidates `T = R ∓ F_i` are formed
with one shared batched inversion:

- `x(T) ∈ FB` gives a full relation, re-added in the group at once;
- otherwise, if `x(T) ∈ (max_x(FB), X_L]` with `X_L = p >> s`, it gives a
  partial relation `R = ±F_i ± T_c`, an edge to vertex `x(T)`. A partial is
  re-added in the group only if it reaches the 2-core: lazy, fail-closed
  verification, one addition each.

Collection stops when `fulls + Σ_v (deg v − 1) ≥ B + 1 + 8`. Degree-one vertices are peeled, the
2-core goes through `large_prime_filter::filter_large_primes`, and every
complete row is replayed from its sources. The dense Gauss–Jordan solve mod
`n` runs over unknowns `(y_0…y_{B−1}, x)`. If `x` is not pinned, 8 more
relations are collected (at most 4 attempts, all priced). The logarithm is
checked against the planted `k` and by `[x]G = Q`.

**Grid.** `bits ∈ {20, 24, 28, 32}`, `B ∈ {16, 64, 256}`,
`s ∈ {none (baseline), 12, 8, 4, 0}`, 3 targets per cell. A cell is skipped,
and recorded as skipped, when its predicted cost exceeds `2.5·10⁹` GAE. The
prediction is `p` for the baseline and `p·√(B/(2·(p>>s))) + √p` otherwise,
fixed before any run. At 32 bits this skips the three baselines; the
large-prime cells there get no G1 verdict.

**Counted unit.** Group-addition equivalents (GAE):

- sweep: 1 per `(target, F_i)`, covering both candidates;
- step: 1 per walk addition;
- setup: `⌈1.75·bits⌉` per combine (the 32 table points and every restart);
- verify: 1 per verified relation, plus `⌈1.5·bits⌉` for the final `[x]G`.

`S = GAE / √n`.

**Unpriced native counters** (§5a): factor-base square roots, hash
operations (vertex inserts and peel lookups), filter row operations
(merges + pivots + peels), and LA multiplications mod `n` (an inversion
counted as `2·62`). They are reported per row and priced at
`w ∈ {0, 0.1, 1}` GAE per operation as `S_w0, S_w01, S_w1`.

**Reference (matched rho).** `prime_fast::rho_parallel` with negation, same
binary, curve and planted targets. `S = (steps + setup)/√n`, where setup is
`(2^table_bits + walkers)·1.75·bits` GAE. DP inserts are its unpriced
counter, in the same band. The steps-only `S` is also reported.

**Boundaries.** Floor: the generic bound (Shoup), `S = Ω(1)`. For this
method family the predicted floor is `S ≥ ½√B·√(p/n)`. Reference: matched rho
as above.

**Decision rules** (medians over the 3 targets; a cell counts only if all 3
targets solved correctly):

- **G1 (engineering vs this IC-2 baseline).** At fixed `(bits, B)`, a
  large-prime cell "lowers IC-2 cost" if its median `S` is below the
  baseline's at every `w` in the band. Where the baseline was skipped, no G1
  verdict is given; the cell is labelled extrapolated-only.
- **G2 (model check).** For each `(bits, B)` with at least 3 solved
  large-prime cells, the least-squares slope of `ln S_w0` against `ln ℓ`,
  with `ℓ = (p>>s) − max_x`. The model "holds" if the slope lies in
  `[−0.65, −0.35]`, otherwise "fails".
- **G3 (distance to rho).** At each `bits`, `ρ = min_cells median S_w /
  median S_rho,w` at each `w`. "Below rho" requires `ρ < 1` at every `w` at
  two consecutive sizes including 32 bits; this would contradict the generic
  bound's constant and triggers an audit before anything else. Otherwise
  the result is reported as the distance `ρ`. Prediction: `ρ ≥ 1` everywhere.

### 3.2 Arm L: lattice smallness

**Method.** Plant `R = F_i ± F_j` with both points in the factor base, and
build the level-`m` lattice for:

- *Interval:* `f(u, v) = S₃(u, v, x_R)`, unknowns `(x_i, x_j) ≤ B`.
  Monomials `u^α v^β` with `α, β ∈ [0, 2m]`; row `p^{m−k} u^{α−2k} v^{β−2k} f^k`
  with `k = min(⌊α/2⌋, ⌊β/2⌋, m)`; column scale `B^{α+β}`. This is probe B,
  re-implemented on the shared routine.
- *Fraction:* `H = Σ c_ij a₁^i b₁^{2−i} a₂^j b₂^{2−j}`, unknowns
  `(a₁, b₁, a₂, b₂)` with `|a| ≤ X`, `1 ≤ b ≤ X`, `gcd = 1`. Monomials
  `a₁^α b₁^{2m−α} a₂^β b₂^{2m−β}`; row
  `p^{m−k} a₁^{α−2k} b₁^{2m−α} a₂^{β−2k} b₂^{2m−β} H^k`; column scale `X^{4m}`.

Both are LLL-reduced with δ = 0.99 by `lattice::lll_reduce`. Outcome per
trial: the number of reduced rows that vanish at the planted root over `Z`
(success at ≥ 2, probe B's criterion) and whether Howgrave-Graham holds on
the first two rows.

**Equal size.** At grid value `δ`, the interval uses `B = ⌊p^δ⌋`. The
fraction set uses the largest `X` with `#{a/b} ≤ ⌊p^δ⌋`, counted exactly by
`fraction_count`. The achieved `δ_eff = log_p |set|` is recorded.

**Derivation (the prediction).** Both lattices are triangular, with diagonal
`p^{m−k}` (both `f` and `H` are monic in `u²v²`, respectively `a₁²a₂²`), so
`det = p^{Σ(m−k)} · Π scale`. Interval:
`Π B^{α+β} = B^{(2m+1)²·2m}`. Fraction: `Π X^{4m} = X^{(2m+1)²·4m} = (X²)^{(2m+1)²·2m}`.
With `|set| ≈ (12/π²)X²` against `B`, the Howgrave-Graham conditions match up
to a constant factor in `B`. **Prediction:** the same threshold in `δ_eff`,
within one grid step.

**Grid.** Curves at 40 and 56 bits. Levels 1 and 2 at both sizes, level 3 at
40 bits only. `δ ∈ {0.06, 0.08, …, 0.20}`. 16 trials per cell.

**Positive control.** The interval arm at `δ ≤ 0.10` must succeed in at least
14/16 trials per cell (probe B saw 100%). If it does not, arm L is
reported as an invalid instrument.

**Decision rule.** For each (bits, level, kind), let `δ*` be the largest grid
`δ` with successes ≥ 8/16.

- **"Fraction extends reach"** if `δ*_frac ≥ δ*_int + 0.04` at both sizes for
  some level.
- **"Same reach"** if `|δ*_frac − δ*_int| ≤ 0.02` at every (bits, level).
- **"Fraction loses reach"** if `δ*_frac ≤ δ*_int − 0.04` at both sizes for
  some level.
- Otherwise **"inconclusive"**.

Power: under a common per-trial success rate, a 0.04 (two-step) gap at both
sizes is unlikely by chance. At a cell with true rate 0.5, P(≥ 8/16) ≈ 0.6, so
single-step differences are expected noise, and the rule ignores them.

**Milestone (would justify an `S₄` follow-up).** Fraction successes ≥ 8/16 at
`δ ≥ 0.20`, against the 0 the interval arm and probe C recorded there.

### 3.3 Arm F: fake subfields under F4

**Method.** On the arm F curve, for `B ∈ {8, 16, …, 1024}` (powers of two) and each family
build the x-set of size exactly `B`:

- interval `{1..B}`;
- `μ_B` (dense in `F_p`, sparse polynomial `x^B − 1`);
- the first `B` fractions `a/b` by height.

Take `L = Π (X − x)`. Plant `R = F_u ± F_v` with `u, v` in the set and on the
curve. Run `f4_fp::f4` on `⟨S₃(x₁, x₂, x_R), L(x₁), L(x₂)⟩` with grevlex,
degree bound `2B + 8`, and a 600 s budget, 3 seeds per cell.

**Recorded.** Counted `F_p` multiplications (`field_ops`), solving degree,
widest matrix, steps, wall time, timeout, pairs above the bound, whether the
planted root lies in the variety of the basis, and the number of standard
monomials (the solution count over `F̄_p` with multiplicity).

**Positive control.** The planted root lies in the variety in every
non-timed-out run. If not, arm F is invalid.

**Reference.** Direct subtraction decides the same question for one target
in `7·|on-curve part of set| + 2·bits` multiplications: 3 in the shared
inversion, 2 slopes and 2 squarings per point, plus the one inversion. This
count is derived from the `collect_relations` code path, not measured.

**Decision rule.** For each family, fit the least-squares slope of
`ln field_ops` against `ln B` over the B values that completed (at least 4):

- **"Sub-enumeration (advance candidate)"** if the slope is < 1 and the ratio
  to direct subtraction is < 1 at the largest completed `B`.
- **"No lever"** if the slope is ≥ 1 and the ratio is > 1 at every completed
  `B`.
- Otherwise **"inconclusive"**.

The §4.2 falsifier is reported as `max(solutions / B)`.

## 4. What does not count

The following are not permitted:

- changing grids, seeds, budgets, success criteria or decision thresholds
  after seeing data;
- dropping a failed or timed-out row;
- reporting a stage cost as a method cost;
- comparing arm G against rho without the §5a band;
- quoting wall time as the metric. Wall time is a practicality note only.

## 5. Pre-flight

The dry run (`--quick`, scratch output, not committed) and the per-cell
estimates are in §5.1, added before the freeze.

### 5.1 Dry-run findings and timings (2026-10-23, before freezing)

The host has 2 threads and 7 GB of memory. The dry run used `--quick`
(arm G at 20 and 24 bits with B ∈ {16, 64} and 2 targets; arm L at level 1
with 4 trials; arm F at B ≤ 128 with 1 seed). Its output was written to
scratch and is not committed. It changed the harness in two ways before the
freeze; both are recorded here.

1. **Arithmetic walk made the graph degenerate.** With `R ← R + G` and a
   fixed `b`, every large-prime cell failed "x not pinned after 4 attempts".
   The cycle relations were the same row repeated: two targets `R_s, R_t`
   meeting at one vertex give `R_s + R_t = F_i + F_j`, which depends only on
   `s + t`. That is a property of the walk, not of the graph method. The fix
   is the r-adding walk in §3.1. After it, every dry-run cell solved and was
   checked.
2. **Eager partial verification dominated at `s = 0`.** Re-adding every
   partial costs one GAE per partial. At `s = 0` almost every candidate is a
   partial, so this was more than half of `S` (24 bits, B = 16: `S_w0` 9.2
   eager against 3.4 lazy). Only the 2-core rows enter the solve, so only
   they are verified now. The check stays fail-closed.

Measured dry-run rates: the arm G sweep costs about 55 ns per GAE (24-bit
baseline, B = 16: 8.7·10⁶ GAE in 0.48 s). An arm L level-1 trial takes under
1 ms. Arm F at B = 128 takes about 80 ms per F4 run and `2.4·10⁶`
multiplications, growing about 4× per doubling of B. The arm F grid was
therefore extended from B ≤ 128 to B ≤ 1024 before the freeze.

The dry run did not show the target arms' outcome in any way that changed a
rule. Its level-1 arm L numbers (4 trials) had the fraction lattice
succeeding at one grid step beyond the interval lattice at 40 bits. That is
inside the one-step band the decision rule treats as noise, and the rule is
unchanged.

**Estimates for the full run** (single-threaded except rho):

- Arm G: 28-bit baselines `≈ p·55 ns ≈ 15 s` each. All other cells are below
  that, except 32-bit `s = 12` at B = 256 (`≈ 5·10⁷` GAE). Total about 10–20
  min.
- Arm L: level 3 at 40 bits costs 1–8 s per LLL (probe B's timing), so
  16 cells × 16 trials ≈ 20–35 min. Levels 1–2 take a few minutes.
- Arm F: B = 1024 is extrapolated at `≈ 1.5·10⁸` multiplications and
  `≈ 10` s per run, within the 600 s budget. Total a few minutes.

## 6. Outputs

`results/arm_g.jsonl`, `results/arm_l.jsonl`, `results/arm_f.jsonl` (one
meta line, then one row per run, failures included), `results/curves.json`,
`results/MANIFEST.json` (protocol hash, commit, command, host, hashes of the
raw files), and the run report with graph and PDF under
`reports/run-<UTC start>/`. The study README carries the results table,
verdicts and scope.

## 7. Amendment v1.1 (2026-10-23, after the v1 run of arms G and F, before any v1.1 run)

This amendment was written after seeing v1 data. v1 stays the frozen
record: its rows, summary and verdicts are committed under `results/v1/` and
reported beside v1.1, never replaced.

**A1. Arm G: deduplicate rows before the solve.** In v1, 4 of 171 arm G runs
ended "x not pinned after 4 attempts": all at `s = 0` with `B = 256`, one at
24 bits and all three at 28 bits. The cause is in the implementation, not the
method. A cycle `R − F_i = R' − F_j` is the same equation as
`R + F_j = R' + F_i`, so when `ℓ` covers almost every x-value it closes at
two vertices. The stopping rule then counts it twice, and the LA receives
about half as many distinct rows as the count suggests. The four retries
(8 rows each) cannot cover a shortfall of about 130 rows.

v1.1 normalises every row to leading coefficient 1 (one inversion, priced
as LA multiplications plus one hash operation) and drops duplicates. If
fewer than `B + 1 + 8` distinct rows remain, it collects twice the shortfall
and filters again; this does not count as a failed solve. Everything else in
§3.1 is unchanged: grid, seeds, budget, unit, band and decision rules. The
full arm G grid is rerun under v1.1 as `results/v1_1/g/`. v1.1 decides
G1–G3; v1's verdicts are reported beside it.

**A2. Arm L: recoverability audit (secondary, post hoc).** In v1's arm L,
the fraction lattice at level 2 on the 40-bit curve met the primary
criterion (≥ 2 reduced rows vanishing at the root, in ≥ 8/16 trials) up to
`δ = 0.18`; the interval lattice did so up to `0.14`. That is a two-step gap,
which is what the rule calls "extends reach" if it repeats at 56 bits. The
primary criterion is necessary but not sufficient for recovering the root.
Two vanishing rows that share a factor through the root cut out a curve,
not a point. The bihomogeneous lattice has more room for such rows than the
interval lattice.

The audit reruns arm L's grid with the identical seeds and trial code
(`lattice_trial_keep` is `lattice_trial` with the vanishing rows returned).
It must reproduce v1's primary counts exactly; any mismatch voids the audit.
For each trial it also records **recoverable**: whether some pair of
vanishing rows has a zero-dimensional intersection. To decide this, the
rows are dehomogenised (fraction: `b₁ = b₂ = 1`), and the resultants with
respect to each variable are checked to be not identically zero at 24
random specialisations modulo `q = 2^61 − 1`.

**Use.** The primary arm L verdict is reported exactly as §3.2 defines it.
It is qualified as follows. The same decision rule (thresholds, sizes and
levels unchanged) is applied to the recoverable counts. If the primary
verdict is "Fraction extends reach" and the recoverable verdict is not, the
report states that the extension is an artifact of the vanishing criterion
and is not evidence of more reach. Only a "Fraction extends reach" verdict
under both criteria may be described as more reach. Because this criterion
was added after the data, any such description is labelled post hoc and
needs a fresh preregistered run (a new curve family and seeds) before it is
cited.

## 8. Protocol v2: arm L2, size scaling and a centred-interval control (2026-10-23, before any L2 run)

**Why.** v1's primary arm L verdict is "Fraction extends reach": `δ*`
differs by 0.04 at level 2 at both 40 and 56 bits, and by 0.04 at every
level at 40 bits. The v1.1 audit's early rows show the recoverable counts
equal to the vanishing counts. Two explanations fit and predict different
things:

- **Constant factor.** The v1 interval arm uses `x ∈ [1, B]`, a one-sided
  set whose bound equals its size. The fraction set uses signed `a` and has
  `|set| ≈ 1.216 X²`. Both differ from the best interval set by constant
  factors in the bound. A constant factor `c` moves the reach by
  `log_p c`, which shrinks like `1/bits`. Prediction: the gap measured in
  bits of `B` (`Δδ · bits`) stays roughly constant as `p` grows, and a
  centred interval `|x| ≤ h` (`|set| = 2h + 1`, bound `≈ |set|/2`) matches
  or beats the fraction lattice.
- **Exponent.** The bihomogeneous lattice reaches further in `δ` itself.
  Prediction: `Δδ` does not shrink with `p`.

The determinant derivation in §3.2 predicts the constant-factor case.

**Method.** Same trial routine as arm L (`lattice_trial_keep`) with a third
kind, `Centered`: plant `x` uniform in `[−h, h]` and use bound `h`, with
`h = ⌊(⌊p^δ⌋ − 1)/2⌋`. Interval and fraction are sized as in §3.2.

- Curves: `find_a3_curve(bits, 3)` for bits ∈ {40, 50, 60}.
- Levels: 1 and 2.
- `δ ∈ {0.08, 0.09, …, 0.24}`.
- 24 trials per cell. Seed `0x2A770000 + 1000·bits + 100·level + round(100·δ)`,
  shared by the three kinds at a cell.
- Per trial: success (≥ 2 vanishing rows), recoverable (§7 A2), and
  Howgrave-Graham on the first two rows.

**Metric.** `δ50` for (bits, level, kind): the `δ_eff` at which the
**recoverable** fraction first falls below 0.5, linearly interpolated
between the last cell at or above 0.5 and the next cell. The vanishing-
criterion `δ50` is also reported. All of this is computed in Rust
(`--arm s`, `summarize_l2`).

**Decision rules** (per level; `Δ = δ50_frac − δ50_interval`):

- **"Gap persists in δ (exponent-like)"** if `Δ(60) ≥ 0.75 · Δ(40)`.
- **"Gap constant in bits of B (constant factor)"** if `Δ(60) < 0.75 · Δ(40)`
  and `Δ(60)·60 / (Δ(40)·40) ∈ [0.65, 1.35]`.
- Otherwise **"inconclusive"**.
- **"The centred interval explains the gap"** if
  `δ50_frac − δ50_centred ≤ 0.005` at every size.

Only "gap persists in δ" together with a centred interval that does *not*
explain it would justify the §3.2 milestone follow-up (an `S₄`
fraction-lattice run). Any other outcome closes the lever, with the reason
stated.

**Pre-flight.** The `--quick` dry run (40 bits, level 1, 4 trials,
scratch) ran end to end. Level 2 costs about 50–150 ms per trial at
40–56 bits (v1). The estimate is about 7,300 trials, 15–30 min,
single-threaded.
