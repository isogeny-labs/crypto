# Sparse relation rows built straight from their summands: same-session A/B with its own A/A (protocol)

Written on 2026-11-15, **before any row**. The owner asked to keep
optimizing the index calculus.

## Why

The verify phase (`setup_phases_ns.verify`) costs 10–13 ms at k0n53 and
15–16 ms at k0n61. Temporary timers in `FactorBaseLogSolver::push` (not
committed; `RAYON_NUM_THREADS=1`, k0n53, three repetitions) split it:
- about 5.1 ms a repetition goes to `verify_collected_relation`, the group
  check of each collected relation;
- about 7.4 ms a repetition goes to `LogSystem::push`, about 28 µs a row.

`LogSystem::push` builds each row through
`relation_from_decomposition_with_mode`. That function:
- allocates a dense `Vec<BigUint>` over every column (264–320 of them);
- computes `λ^k mod r` with a `modpow` for each summand;
- has `SparseRows::push_dense` scan the dense row back into sparse form.

## The change

- **Direct rows:** on the sparse path, `LogSystem::push` reads each
  summand's projected location `(column, k, negated)` and takes `λ^k mod r`
  from a table of `λ^0 .. λ^n` built once per system. It pushes the
  `(column, ±λ^k)` pairs with `SparseRows::push_entries`, whose
  `SparseRow::new` sorts, adds per column mod `r` and drops zeros.
- **Same row:** this is the row the dense route gives. Debug builds assert
  equality on every row against the dense route.
- **Control:** `KIC_DIRECT_ROWS=0` restores the dense route. The dense
  linear-algebra path is unchanged.
- **Unchanged:** answers and counted work. Row building is not counted
  work.
- **Exploration** ([`../explore/`](../explore/), one process an arm, three
  repetitions each):
  - verify fell 9.99–11.1 → 4.22–4.68 ms at k0n53 and 14.7–16.5 →
    6.75–6.96 ms at k0n61;
  - 160 and 172 counted and answer fields were identical.
- **Replay sentinels:** `ecbench` reproduced 51 sessions and 90 records,
  0 failing.

## Arms: one binary, three labels

- **Binary:** `ic`, release, built at this protocol's commit with
  default features. Its SHA-256 goes in `runs/arms.sha256` before the
  first row.
- **Labels:**
  - **a**: `KIC_DIRECT_ROWS=0`, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **d**: the default (direct rows).

## Workload, schedule, metrics and statistics

- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`.
- **Schedule:** **24 rounds**, cycled over the six orders of `a q d` (four
  cycles).
- **Metrics:** per-process medians over repetitions of **verify**
  (`setup_phases_ns.verify`, primary) and set-up (`setup_ns`).
- **Statistics:** per-round A/B `a / d` and A/A `a / q`; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261118).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a set-up regression is established if the A/B upper end
  is below 1 and below the A/A lower end.
- **Kept:** direct rows stay the default if the **verify** gain is
  established on at least one row and no set-up regression is
  established.
- **Otherwise:** the default reverts to the dense route, the switch
  stays, and the reason is recorded.

## Prediction (before any row)

Medians `a / d`:

| row | verify | set-up |
|:--|:--|:--|
| k0n53 | [1.9, 3.0] | [1.00, 1.04] |
| k0n61 | [1.8, 2.8] | [1.00, 1.02] |

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512). The Iso Lab
bridge cannot submit while its CodeBuild quota reads 0. The result holds
for this host class only.
