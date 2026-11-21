# Single-word relation check in the log solver: same-session A/B with its own A/A (protocol)

Written on 2026-11-15, **before any row**. The owner asked to keep
optimizing the index calculus.

## Why

After direct sparse rows (merged in #4292), most of the verify phase
(`setup_phases_ns.verify`) is `verify_collected_relation`. That is the
group check of each collected relation, `[a]G == Σ P_i`, or `Σ P_i = O`
for a free relation. Its sum used the generic `point_add`, which inverts
by Fermat's little theorem over `F2mElement`, a multi-word element.
`KoblitzCurve::mul` already took the single-word `FastCurve` route.

## The change

- **Fast sum:** `verify_collected_relation` sums the summands with
  `FastCurve::add` when every point round-trips through the single-word
  field. This covers probe and free relations alike.
- **Order unchanged:** it adds left to right from `O`, as before. The
  generic route remains the fallback.
- **Counted work unchanged:**
  - `FastCurve::add` and `double` count additions and doublings exactly
    where `point_add` and `point_double` do: `P + (−P)` is one add, and
    `P + P` is one double.
  - The scalar multiple is the one `KoblitzCurve::mul` takes.
- **Same verdicts and counts:** the new test
  `fast_relation_checks_match_the_generic_verdicts_and_counts` compares
  the two routes on valid, corrupted, doubled, cancelling and free
  relations.
- **Control:** `KIC_FAST_VERIFY=0` restores the generic route.
- **Exploration** ([`../explore/`](../explore/), one process an arm, three
  repetitions each):
  - verify fell 5.65–6.05 → 3.16–3.34 ms at k0n53 and 6.74–7.46 →
    4.15–4.38 ms at k0n61;
  - 160 and 172 counted and answer fields were identical.

## Arms: one binary, three labels

- **Binary:** `ic`, release, built at this protocol's commit with
  default features. Its SHA-256 goes in `runs/arms.sha256` before the
  first row.
- **Labels:**
  - **a**: `KIC_FAST_VERIFY=0`, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **f**: the default (single-word check).

## Workload, schedule, metrics and statistics

- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`.
- **Schedule:** **24 rounds**, cycled over the six orders of `a q f` (four
  cycles).
- **Metrics:** per-process medians over repetitions of **verify**
  (`setup_phases_ns.verify`, primary) and set-up (`setup_ns`).
- **Statistics:** per-round A/B `a / f` and A/A `a / q`; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261119).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a set-up regression is established if the A/B upper end
  is below 1 and below the A/A lower end.
- **Kept:** the fast check stays the default if the **verify** gain is
  established on at least one row and no set-up regression is
  established.
- **Otherwise:** the default reverts to the generic route, the switch
  stays, and the reason is recorded.

## Prediction (before any row)

Medians `a / f`:

| row | verify | set-up |
|:--|:--|:--|
| k0n53 | [1.4, 2.2] | [1.00, 1.03] |
| k0n61 | [1.3, 2.0] | [1.00, 1.01] |

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512). The Iso Lab
bridge cannot submit while its CodeBuild quota reads 0. The result holds
for this host class only.
