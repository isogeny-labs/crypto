# Protocol: a table-free linear `2 + 1` decomposition on the frozen m = 3 cells

Registered 2026-11-13, committed before any claim run. A pilot on another
seed (777) came first and is disclosed in §8; it enters no table or fit.

## 1. Question

`research/ic_m3_algebraic_vs_mitm_20261014/` compared the Gröbner and SAT
decomposition oracles for `m = 3` with the pair-table meet-in-the-middle
floor on Koblitz curves with the subspace base `V = span(1, z, …, z^{d−1})`,
`d = ⌈n/3⌉`. Every algebraic arm grew faster than the floor; SAT was
undetermined. The floor, however, needs a table of `|F|(|F| + 1)/2` pair sums.

The repository's linear two-summand solve (`examples/ic_m2_linear.rs`,
2026-10-24; a `2 + 1` split in `examples/ic_m3_cycle.rs`) postdates that
study and was never run on its targets. It uses that
`S₃(x₁, x₂, u) = p² + u·p + u²·s² + b` is `F₂`-affine in
`(s, p) = (x₁ + x₂, x₁x₂)`: no Gröbner basis and no pair table.

**Question.** On the same cells and the same targets, what does a `2 + 1`
decision built on that linear solve cost per relation against the floor, in
wall time, in counted work and in memory; and how far past the largest cell
whose pair table fits this host does it run?

## 2. Arms

Instrument: `examples/ic_algebraic_floor/` (native Rust). New code:
`linear21.rs`, the `lin21`/`lin21-ht` arms of `solve`, and the subcommands
`linpool` and `linsummary`.

| arm | what |
|---|---|
| `floor-mitm` | unchanged from 2026-10-14: `PairSumTable` sorted scan; also the ground truth |
| `f4-chain` | unchanged; rerun in this session on the six core cells as the same-host Gröbner comparator |
| `lin21` | for every base point `P₃` (both signs), `Q = R − P₃`, then the repository's linear solve of `S₃` in `(s, p)`: `n` equations, `3d − 1` unknown bits; split of `z² + sz + p` by a second elimination |
| `lin21-ht` | the same decision with `p` eliminated by the half-trace: `p = u·(s + H(s) + H(b/u²) + ε)`, so the system is `n − 2d + 2` equations in `d + 1` bits; split by `x₁ = s·H(p/s²)`; inversions batched (blocks of 256) |

Every witness is re-added in the group. Both linear arms stop at the first
witness, as the floor does.

## 3. Cells and targets

Seed `20261014` and the cell-seed mix of the 2026-10-14 protocol, so the core
pools and the 12 arm targets per cell are the same targets as that study.
Curve rule unchanged (smaller cofactor); `d = ⌈n/3⌉`.

| group | cells `a:n:d` | pool | what runs |
|---|---|--:|---|
| core | `0:13:5`, `1:17:6`, `1:19:7`, `1:23:8`, `1:29:10`, `0:31:11` | 2,048 | floor, `f4-chain`, `lin21`, `lin21-ht` (12 targets, child processes), `linpool lin21-ht` (whole pool) |
| extension, table fits | `0:37:13`, `0:41:14` | 2,048 | floor, `lin21`, `lin21-ht`, `linpool` (`f4-chain` exceeds the 64-unknown cap: not run) |
| extension, no table | `1:47:16`, `0:53:18`, `1:59:20` | 2,048 / 1,024 / 512 | `linpool lin21-ht` only; the pair table would hold 2.1·10⁹, 3.4·10¹⁰ and 5.5·10¹¹ entries |

The no-table curves have cofactors 1,318, 428 and 22,894. Targets are in the
prime-order subgroup and the base is every curve point with abscissa in `V`,
as in every other cell.

One cell per process invocation, so the recorded peak resident set
(`VmHWM`) is that cell's. One job at a time on the host.

## 4. Estimators

- **12-target arms** (`run`): the 2026-10-14 estimator, unit, budgets (120 s,
  4 GiB), early stop, bootstrap (2,000, seed 20261014) and decision rule,
  unchanged, through `analyze`.
- **Whole pool** (`linpool`, `linsummary`): cost per relation
  `Σ wall / #decomposable` for `lin21-ht` and for the floor's decision mode
  on the same targets; their ratio with a paired bootstrap over targets
  (2,000 replicates, seed 20261113), online and amortized (floor: table build
  over `|F|cols`; linear arm: its setup over `|F|cols`).
- **Counted work per relation:** linear-arm trials (third summands tried)
  and floor group additions; OLS slope of `log₂` on `n` over the floor cells,
  and the difference Δ with the same bootstrap.
- **Memory:** `VmHWM` of the floor's orchestrator and of the `linpool`
  process, per cell.

## 5. Predictions (declared before claim data)

1. **Exactness.** `lin21-ht` gives the floor's verdict on every pool target
   of every floor cell (0 disagreements in 8 × 2,048), and `lin21` and
   `lin21-ht` agree with the floor on all 12 arm targets of every cell.
2. **Counted exponent.** Δ(trials slope − floor additions slope) has a 95%
   interval inside [−0.02, 0.02].
3. **Wall.** `lin21-ht ÷ floor` (online) lies between 2 and 30 on every floor
   cell. Under the frozen 2026-10-14 rule on wall time the arm may still read
   "grows faster" by a few hundredths per bit, because a trial costs
   `O(d)` multiplications where a probe costs `O(1)`; that would be a
   polynomial factor, checked by item 4.
4. **Polynomial trial cost.** `ns/trial` at n = 59 is at most 6× that at
   n = 13.
5. **Memory.** The linear arm's peak resident set is under 100 MB on every
   cell; the floor's exceeds 2 GB at n = 41.
6. `lin21-ht` is cheaper than `lin21` on every cell.

**Falsifiers.** Any disagreement or bad witness (item 1) makes the arm
inexact and voids the rest. Item 2 fails if the interval leaves
[−0.02, 0.02]. Items 3 to 6 fail as stated.

## 6. What this can and cannot show

A stage diagnostic (AGENTS.md §8): decomposition cost per relation, one core,
isolation L0 (shared 4-vCPU cloud container). No end-to-end, `vs_rho` or
exponent claim. The `2 + 1` scan tries `|F|` third summands per target by
construction, so it cannot grow more slowly than the floor in counted work;
the question is what removing the table costs. Results are
`[pcc:unverified]` until a validator replays them.

## 7. Sources checked

`research-kb` and the lead atlas are not available in this container (no
executable, no sibling checkout), so prior art was searched in the two
checkouts directly:

- `crypto:research/ic_m3_algebraic_vs_mitm_20261014/` (the matched study this extends)
- `crypto:research/ic_m2_linear_ladder_20261024/`, `crypto:research/ic_m2_cycle_leads_20261030/REPORT.md` (the linear solve; "the landed ladder is not new algebra")
- `crypto:examples/ic_m3_cycle.rs` (the `2 + 1` split as an end-to-end producer; not run against the floor)
- `crypto:research/sat_pdp_pricing_band_20261111/`, `crypto:research/pdp_smt_oracle_20261008/`, `crypto:research/phase_paired_native_20261026/` (SAT and SMT)
- `crypto:research/fes_then_batched_20261112/`, `crypto:research/fes_k13_validated_20261112/` (FES)
- `crypto:research/prime_leads_chase_20261113/L2-lattice/`, `crypto:RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (lattices)
- `crypto:research/semaev_qubo_annealing_20261110/` (local search)

The "not run on the 2026-10-14 targets" statement rests on these searches and
should be repeated when the knowledge base is reachable.

## 8. Pilot (not claim data)

Seed 777, pools of 256, 20 + 20 targets: `lin21` and `lin21-ht` agreed with
the floor on 240/240 child solves each (n = 13, 17, 19, 23, 29, 37), and
`linpool lin21-ht` on 768/768 pool targets (n = 13, 23, 37). The pilot also
fixed the implementation (batched inversion and the half-trace split were
added after a first pilot at about 1.1 µs per trial) and showed that
`Cell::new` succeeds at n = 47, 53 and 59. Predictions 3 to 5 were written
after seeing pilot timings.
