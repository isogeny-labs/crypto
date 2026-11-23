# The rule's comparison at baseline v10: a single-word factor-base selection

**Declared 2026-11-11, before any row below and before the change is
implemented.** This protocol is committed and pushed first. The
implementation, `run.sh` and `binaries.sha256` follow in later commits, all
before the first row. No timed row of either arm exists when this file is
committed. The development facts used to write it are listed under
"What was seen before declaring".

**Everything rule v9 declared holds here** ([`../v9/PROTOCOL.md`](../v9/PROTOCOL.md),
its amendment 1, and through it v8's, v7's, v6's, v3's and v2's):

- §23's six sizes, 64 targets and four A/A repeats a size (408 rows a run);
- the arms' order inside one process, with rho matched;
- the retries and stop rule;
- the claims, the pin against §23's `T01-R1` (with v8's added-counter
  allowance), and each row's logarithms against §23's;
- the reference check against the strong rho, the accounting, class and
  non-claims;
- the pricing statement under AGENTS.md §5a. Both arms are timed in one
  process, so no native work is unpriced. The 0, 0.1 and 1 GAE band gives
  one figure.

The rows under "What changes" are the only differences.

## Question

Table A's Koblitz rows read rule v9. The cold distance from the index
calculus to a matched rho is 4.08x at n47, 4.73x at n57, 6.14x at n41,
7.46x at n53, 6.83x at n59 and 11.13x at n61. Set-up is about 99% of the
cold index calculus. In v9's own rows, the `select` and `select_projection`
clocks hold about 27% of set-up at n47 (3.0 + 0.7 of 13.5 ms in
`runs/k1n47/T01-R1.price.json`), 25% at n57, 13% at n41, 10% at n53, 12% at
n59 and 5% at n61.

How much of that distance goes if the factor base is selected and projected
in single-word (`u64`) arithmetic, with the base unchanged bit for bit?

## The single change

`build_subgroup_orbit_factor_base_with_cost` and the projection behind
`projected_signed_orbit_count` (`src/cryptanalysis/koblitz_index_calculus.rs`)
get a single-word fast path, taken when `n < 64`, `n` is odd, a
`FastCurve` exists and Frobenius is an endomorphism. Every size here is
eligible. The fast path makes no new decision:

- the same draws from the same `StdRng`, in the same order;
- abscissae and their Frobenius orbits kept as `u64` words, squared by the
  field's `sqr_k`, not as `BigUint` keys squared by `F2mElement`;
- the base built from the sorted abscissa words. Its points are lifted and
  its orbits walked on packed points, and lowered to `BinaryPoint` once at
  the end;
- point-keyed tables hashed by the repository's `MixHasher`
  (`ic_boundary.rs`), not SipHash. Lookups do not depend on the hasher.

The `BigUint` path stays as the reference, and is the only path for
`n >= 64`, even `n`, or a non-endomorphic Frobenius. The native counters
(`selection_cost`, the thread's Frobenius-map count) are kept exactly.

**The result must be byte-identical.** The factor base, its orbit maps, the
projected map, every counter and every logarithm do not change. A unit test
compares both paths on several small and medium Koblitz curves.

## What changes from rule v9

- **Two runs, one session, one host:**
  - **`v10`**: `ic` built from this branch with the fast path. Its commit
    and SHA-256 are recorded in `binaries.sha256` and in amendment 0 below
    before the first row.
  - **`v10c`, the control**: `ic` built from `main` at
    `ca1a767b1fc006966e0c56fc15110ceb79f10342`, the branch's base. This is
    rule v9's code plus what landed on `main` since then. It is the same
    source as `v10` without the change. Its SHA-256 is in
    `binaries.sha256`.
  - Each run is paired with its own in-process matched rho, as in v9 and v9c.
- **Interleaved by size**, as v9: manifests and pins first, then the sizes
  in §23's order. At each size, `v10` runs, then `v10c`.
- **Harness:** `icprog` from this branch. Its only change is the
  `V10`/`V10C` comparisons and the `matched` read-out for v10. It changes
  nothing about how a row is run or checked. `isolated_bench` and
  `koblitz_rho_fixture` are built from the branch base.
- **Route: local L0, not Iso Lab.** Iso Lab on EKS Spot is the lab default
  (AGENTS.md §9a), and rule v9 ran there after its amendment 1. This study
  runs in the session's own container: Intel Xeon @ 2.10 GHz, 4 vCPU, no
  hardware PMU, a host that other sessions may share. Every row runs
  through `isolated_bench` on a reserved core, and its contention record is
  kept. Iso Lab's 1 h / L2 admission contract is out of scope here. The
  comparison is the in-process pairing of each arm with its matched rho,
  plus the same-session pairing of the two arms by target. The absolute
  cold ratios are this host's, not v9's EKS host's. They are quoted beside
  v9's and never paired with them.

## Predictions (not drawn from a run)

**P1, identity.** At every size the factor base is byte-identical between
the arms. Every row's `identity_inputs` (the factor base's orbit
representatives among them) and `counts`, hashed per row, are equal to its
paired `v10c` row's. Every recovered logarithm is unchanged: `matched`'s
`walks_checked` and `logarithms_as_earlier` in both analyses. `v10`'s pin
holds against §23. *Decision:* any difference stops the read-out. The
change is then a different algorithm, not a timing change, and is reported as
such.

**P2, selection time.** Per size, take the paired ratio of
`select + select_projection` wall time (`setup_phases_ns`, median
repetition), `v10 / v10c`, over the 68 rows. The median of that ratio is
at most 0.25 (selection time at most a quarter of the control's). *Decision:* met at a size
where the median is at most 0.25; reported per size with the ratio's
quartiles.

**P3, the cold distance.** At n47 the cold ratio IC/rho falls from 4.08
to 3.3–3.5, and at n57 from 4.73 to 3.9–4.1. v9's figures are from another
host, so the primary reading is the paired one, `cold_matched_over_control`
from `icprog rule matched --comparison v10`, with its bootstrap interval:

| size | predicted `v10 / v10c` cold | from |
|---|---|---|
| n47 | 0.81–0.86 | 3.3–3.5 of 4.08 |
| n57 | 0.82–0.87 | 3.9–4.1 of 4.73 |

`v10`'s absolute cold ratios are reported beside v9's as a secondary
reading. The other four sizes are reported without a declared range. The
expectation, from the selection shares above, is a smaller gain at n41,
n53 and n59 and the smallest at n61.

**Falsified** if the paired cold `v10 / v10c` at n47 is above 0.90.

**Reporting.** The cold ratio of each run is reported as the distance
still remaining to the matched rho, not as a pass or fail. Under the
owner's success criterion (AGENTS.md, 2026-10-15), a fall in
`v10 / v10c` with every answer checked counts as an improvement [pcc:unverified] over the index-calculus baseline at the
scope measured. It is also reported if it is smaller than predicted.

## Read-out

- Each run's cold ratio per size, with its 95% bootstrap over targets
  (`icprog rule analyse`, `analysis.json`), as every rule reports it.
- `icprog rule matched --comparison v10` (`matched.json`): per size, the
  paired ratios `v10 / v10c` of the cold ratio and of the online time, with
  bootstrap intervals.
- P1 and P2 come from the committed rows with `jq` (`p1p2.sh`, committed
  with `run.sh`). No measurement is computed in Python.
- **The board:** this study does not change the board. If P1 holds and the
  run later passes the 2026-10-19 validation gate (replay certificate,
  validator report, the §5a band), rule v10's `analysis.json` can replace
  v9's Table A rows in a later change. That gate is not run here.

## Success and stop

The rule makes no acceptance decision; it is a measurement.

**Stopped**, as in v9, if a row fails its check, a logarithm differs from
§23's, a pin fails, or the reference check finds a run's rho slower per step
than the strong walk at some size.

## What was seen before declaring

- v9's committed rows, above.
- One untimed development run of the control binary at n47 T01 (not
  isolated, not a row). Its `identity_inputs` and `counts` hash equal to v9's
  `T01-R1`.
- One Callgrind profile of that run's `select` and `select_projection`
  (development, instruction counts only): 32.5 M instructions, about 75%
  selection and 25% projection. The largest shares were SipHash on `u64`
  keys, `BigUint` conversions and the `BTreeMap<BigUint, _>` of abscissae.
  This shaped the change; it is not a measurement of it.

## Commands

[`run.sh`](run.sh), as v9's but local, run without `benchrec` if the record
store is unavailable (stated in the report). For `v10` and `v10c`: `manifest`
and `pin`. Then, for each size, `size` for `v10` and then for `v10c`. Then,
for each run, `reference`, `claims` and `analyse`. Then
`icprog rule matched --comparison v10` and `p1p2.sh`.

## Amendment 0 (2026-11-11, before any row)

Recorded after the change was implemented and before the first row. The
question, the arms, the sizes, the order, the predictions and the decision
rules above are unchanged.

- **The binaries** ([`binaries.sha256`](binaries.sha256)) are as follows.
  `ic-v10` is built from `f67d6c481b`, which is this branch with the change.
  `ic-v10c` is built from `ca1a767b1f`. `icprog` is built from
  `f67d6c481b`. `isolated_bench` and `koblitz_rho_fixture` are built from
  `ca1a767b1f`. The toolchain is rustc 1.97.0 on this container.
- **The implementation** has one detail beyond the list under "The single
  change". A point is lifted once per Frobenius orbit, not once per
  abscissa. For odd `n`, the lift of `x^q` is the `q`-th power of the lift
  of `x`, because `rhs` and the half-trace commute with `q`-th powering.
  The orbit's other points therefore cost two `sqr_k`. A representative
  that does not lift to two points falls back to one lift per abscissa.
  The base is unchanged; the unit test
  `subgroup_orbit_selection_agrees_in_both_representations` compares both
  paths on `a = 0, 1` curves for `n = 13 … 47` and on subfield curves with
  `k = 3, 5, 7`. It checks the base field by field, the selection cost,
  the thread counters, and the projected map against the general-field
  construction. It passes, as do the module's 193 tests and the lib's
  `subgroup` (51) and `factor_base` (60) tests.
- **What was seen** beyond the list above was development only, with no
  row and no isolation. Two unisolated runs of n47 T01 on each binary gave
  equal `identity_inputs` and `counts` hashes for the two arms.
  Their median `select + select_projection` was 1.57 ms (`v10`) against
  4.95 ms (`v10c`), and an earlier build of `v10` (one lift per abscissa)
  gave about 2.0 + 0.6 ms. Callgrind put the two phases at 9.0 M
  instructions (`v10`) against 32.5 M (`v10c`). These figures led to the
  per-orbit lift. They are not the measurement, and no prediction was
  changed after seeing them. On those figures P2's 4x may not be met at
  n47.
