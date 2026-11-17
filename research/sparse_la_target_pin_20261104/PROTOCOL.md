# Protocol: the target-pin sparse solver, its accounting, and where linear algebra carries weight

Frozen 2026-11-04, before any timed or measured run of the new code. Follow-up to
`research/sparse_la_timing_20261018/` (merged as #2219 and #3384), whose next-improvement
section this study carries out. Code under test: branch `claude/sparse-la-target-pin-20261104`
(unit-tested before this freeze; the specs and this file are pinned by `SHA256SUMS`).

## 1. Questions

Q1. **Exactness.** Does `pin-wiedemann-every` determine the target at the same relation as
`structured-gauss` (the target-pin rule), and does the spaced `pin-wiedemann` stay close to it?
Q2. **End to end.** Does either lower the end-to-end `S` of the baseline pipeline rows? The
previous study found the deferred solvers lose because they need full column rank; a solver that
stops at the target pin removes that cause.
Q3. **Linear-algebra stage.** In counted `row_ops`, where (row weight `w`, size `n`) does the
sparse Krylov test cost less than structured elimination? This is a stage diagnostic on planted
systems with the shape of a relation matrix; it is not an end-to-end claim.
Q4. **Accounting.** How much larger is the deferred solvers' `row_ops` once the `block × block`
arithmetic of the Krylov core is charged (the `+counted` names)? This is an accounting change; it
cannot be an improvement.

## 2. What is built (read before the predictions)

- `pin-wiedemann`, `pin-wiedemann-every` (`src/cryptanalysis/ic_framework/linalg.rs`,
  `PinWiedemann`): buffer the rows; after singleton peeling, test whether the target's unit vector
  `e_t` is in the row space by solving `A z = e_t`, `A = RᵀDR`, with Wiedemann (doubling chunks of
  the Krylov sequence, Berlekamp–Massey), check `A z = e_t` exactly, and take `x_t = zᵀ Rᵀ D a`.
  The pin is certified by the exact check, not by a second seed: the two-seed agreement of
  `sparse-cado` cannot certify a pin, because a Wiedemann solution lies in the row space whatever
  the seed. `pin-wiedemann` waits `⌈ops / 16384⌉` rows after a failed attempt (one relation is
  taken to be worth 16,384 `row_ops`, an assumption; the S2 relation phase costs 10⁴ to 3·10⁴ GAE
  per relation); `pin-wiedemann-every` attempts after every row.
- `+counted` suffix on the four deferred names: same solver, same rows, same answer; the Krylov
  core's `block × block` multiplications (projection, approximant basis, `Z f` products; Lanczos
  Gram products, small inverses and products, block updates) are added to `row_ops`.
- Unit tests (all passing before this freeze): pin row equals the echelon row on 8 planted
  systems; a negative control where the target is undetermined never pins; the `+counted` names
  return the same solution with a larger `row_ops`.
  Result: 29 selected library tests passed (`cargo test --release --lib`), among them the three above.
- Before this freeze the stage example was run once as a correctness smoke test (weights 2, 3, 6; base
  size 128; one seed; seven arms). Every recovered value was right and `pin-wiedemann-every` pinned at
  `structured-gauss`'s row on all three systems. Its counts are not used; the predictions below were
  written knowing only that.

## 3. Baselines and arms

Baselines are those of the previous study, unchanged: S1 the `ic-d4-chains` row (`incremental-gauss`,
end-to-end `S` 1,653,062.36 and 1,779,983.12 on E(F_{29⁴})), S2 `structured-gauss` on the 30-bit prime
curve. Gate G0: `ig` must reproduce the S1 baseline records bit for bit.

| spec | arms (IC arms differ only in `linalg`) |
|:--|:--|
| `specs/SPEC-fp4-p29-pin.json` (S1, 2 workloads) | `rho-neg`; `sg` (baseline role); `ig` (G0); `pin-wiedemann`; `pin-wiedemann-every` |
| `specs/SPEC-prime-scaling-pin.json` (S2, 6 targets, size 128 to 4,096) | `rho-neg`; `sg`, `sg-aa` (A/A), `ig`; `pin-wiedemann`, `pin-wiedemann-every`; `sparse-cado` (control for `+counted`), `sparse-cado-counted`, `filter-lanczos-counted`, `wiedemann-counted`, `lanczos-counted` |
| `examples/sparse_la_stage.rs` (stage diagnostic) | `structured-gauss`, `igfr` (dense, full rank), `pin-wiedemann`, `pin-wiedemann-every`, `sparse-cado`, `sparse-cado+counted`, `filter+lanczos+counted` on planted systems: every row has `w` base columns and the target column; `w` in {2, 3, 4, 6, 8}; base size `n` in {128, 256, 512, 1,024} (and 2,048 for `w` in {2, 3, 4}); 3 seeds; modulus 2⁶¹−1 |

Every arm in a workload shares the algorithm seed, so the relation stream is common. Counted
operations are the primary unit; wall time on this shared VM is isolation L0 and descriptive.
Unpriced native work (`row_ops`, rho's canonicalisations) is priced at 0, 0.1 and 1 GAE per
operation on every arm with `ecbench band` (AGENTS.md §5a); two arms are ranked only where the
order holds at all three weights.

## 4. Predictions, written now

- **P1 (exactness).** `pin-wiedemann-every` uses exactly `structured-gauss`'s number of relations
  on every S1, S2 and stage workload. Falsified by any workload where it differs.
- **P2 (spacing).** `pin-wiedemann` uses at most 10 % more relations than `structured-gauss` at the
  median over S2 workloads at every size, and never fewer.
- **P3 (end to end, S1 and S2).** `pin-wiedemann`'s end-to-end `S` over `sg`'s lies in [1.000,
  1.05] at every S2 size (the extra relations of P2, plus a linear-algebra share of at most
  10⁻³ of `S`). It is not below 1. If it is, the cause is investigated before any claim, because
  the same rows reach the same pin.
- **P4 (stage, S2).** Counted linear-algebra work of `pin-wiedemann-every` exceeds `sg`'s at every
  S2 size: the two-summand rows of the prime-abscissa matrices leave structured elimination almost
  no fill-in.
- **P5 (stage, planted).** There is a crossover: for `w ≥ 4` and `n ≥ 1,024` the counted work of
  `pin-wiedemann-every` is below `structured-gauss`'s, and the ratio improves with `n`.
  Falsified if `structured-gauss` is cheaper at every `(w, n)` of the grid.
- **P6 (accounting).** `+counted` raises `row_ops` by a factor above 1 on every size, with
  identical relations and answers (the unit test asserts it on one system); at 4,096 columns by
  more than 1.2× for `sparse-cado`.
- **A/A.** `sg-aa` equals `sg` in every count; `filter-wiedemann` is not run (covered by the
  previous study).

## 5. Analysis, fixed now

1. Per workload and arm: `S` by phase, `relations_found`, `targets_tried`, LA `row_ops`, LA wall.
2. End to end: totals ratio over paired workloads with the bootstrap interval of `ecbench compare`
   and the band of `ecbench band`. An **improvement** is a candidate below the base at every band
   weight on S1 with `ig` as base, every answer verified; on S2 `sg` is the incumbent. A candidate
   that ties the relation count and costs more linear algebra is **not** an improvement.
3. Stage: ratio of counted `row_ops`, candidate over `structured-gauss`, per `(w, n)`, median over
   seeds with the range; the crossover is the smallest `n` per `w` at which the median ratio is
   below 1. A stage win is reported in its own section and becomes a pipeline improvement only
   once it moves end-to-end `S`.
4. Scaling: least-squares slope of log `row_ops` on log `n` per `w` and arm (descriptive).
5. Genus 3 and the stacked D₄ + double-large-prime arm: read from committed data
   (`genus3_la_share/`, `stacked_d4_2lp/`), no new experiment; they give the share of `S` that
   linear algebra holds, which bounds what any solver can gain there (Amdahl).
6. Gap to matched rho on every row, as distance remaining.

Inadmissible after freezing: changing arms, workloads, seeds, sizes, weights, the spacing constant
16,384, or the improvement rule; dropping a run that failed or timed out (it is reported censored).

## 6. Scope, limits and the route

- No rented compute. The isolab EKS route (`isolab-eks`) was not reachable in this session (the MCP
  server failed to connect), so counts are taken in this container at isolation L0.
- Larger sizes (`n` above 4,096 on S2, above 2,048 on the stage grid) and an isolated wall-clock
  comparison need other hardware; they are written as backlog cards, not run.
- Stage rows are planted and random: they carry no cycles of two-summand rows and no repeated
  columns, and say nothing about real relation structure.
- Conductor was not available in this session (no control plane), so `conductor check` was not run.
