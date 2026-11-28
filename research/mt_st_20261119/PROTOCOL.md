# Single-target baseline: the multi-target IC instrument at T = 1 against batched single-target rho, at 2^32, 2^36 and 2^40 (2026-11-19)

Frozen before any measured run.

**Setting.** The owner asked for single-target measurements only (no multi-target or batched-target arms). All
earlier rounds in this series measured T ≥ 1024, where the IC builds one table and one set of factor-base logs and
amortises both over every target.

At T = 1 nothing amortises. The IC still needs about m relations (m = 20–63 factor-base points here), each found by
a search of about n / (2N) trials against a table of N entries, plus the target's own search. Its cost is therefore
about N + (m + 1)·n / (2N), minimised near N ≈ √((m + 1)·n / 2): of order √((m + 1)·n), against √n for rho.

Exploratory single runs on this host [pcc:unverified]: at 2^36 the IC took about 183 ms at its best table size
(μ = 1.5–2.5) against 27 ms for batched single-target rho; at 2^40, 0.65–1.1 s against 0.06–0.24 s.

This round records that baseline properly, with variance over many targets, and tests how the gap scales.

## Instrument: `examples/mt_fieldops.rs` at commit `f47b6e9d42`

- **`rhodpb … st`** (args[8]): single-target lanes. All L lanes walk the current target, each from its own random
  chained start, sharing one pending table and one simultaneous inversion per round. A collision of two walks with
  opposite Q signs, or a database hit, gives the log. Walk, step, distinguished-point and abandon rules are `rhodpb`'s.
  This is the standard parallel distinguished-point rho for one target, with batched inversion.
- **`ic5b … blk`**: a target checks 64 trial pairs per round under one simultaneous inversion, in order, stopping at
  the first hit. Trials, hits and logs are unchanged.
- **Every execution runs with `MTF_HP=1` and `MTF_Z=1`** (huge pages, zero-encoded slots), as in the last two rounds.
- **Binary.** Built standalone with `rustc` at cargo's release settings:

  ```
  rustc --edition 2021 -C opt-level=3 -C codegen-units=16 --crate-type bin examples/mt_fieldops.rs \
        -L dependency=target/release/deps --extern rand=… --extern serde_json=…
  ```

  The SHA-256 is in `check.out` and on every row.

### Pre-run checks, recorded in `check.out` before the main run

1. Unchanged multi-target behaviour: on W28-0 and W36-0, T = 1024, `ic5b … flatf 128 bi rel,ib` and `rhodpb` κ = 0,
   64 lanes (without `st`) give byte-identical output, all fields but times, to the table-build round's binary
   (`198a6e49…`).
2. `blk` is exact: on W32-4 and W36-0 at T = 1, and W32-4 at T = 1024, `ic5b … rel,ib,blk` has `rel,ib`'s table,
   trials, relations and p1_trials and recovers every target.
3. `st` recovers the target at T = 1 for seeds 1–8 on W24-0, W32-4, W36-0 and W40-0, with 16, 32 and 64 lanes.

## Keys (14), every one at T = 1

- IC: `ic5b <inst> 1 <seed> μ 4 1.2 flatf 128 bi rel,ib,blk`, μ ∈ {1.5, 2.0, 2.5, 3.5}. These bracket the exploratory
  optimum. The multi-target tuning (μ = 0.85) was about 1.3× worse at T = 1 in exploration.
- Rho (batched): `rhodpb <inst> 1 <seed> 0 offset L st`, L ∈ {16, 32, 64} × offset ∈ {−1, 0, 1}.
- Rho (serial reference): `rhodpup <inst> 1 <seed> 0 0`.

## Instances, seeds, order

- 2^32: W32-{4..7} (`../mt_fieldops_v5_20261112/instances/`).
- 2^36: W36-{0..3} (`../mt_ic_scale_20261115/instances/`).
- 2^40: W40-{0..3}, made here with `../ic_e222_m221_20261106/gen_instances.py --families W --bits 40 --per 4 --seed
  mtst-20261119` (`instances/MANIFEST.json`).
- Seeds 1–32 per instance; each seed plants one target. 3 sizes × 4 instances × 32 seeds × 14 keys = 5,376
  executions.
- One process at a time, pinned to one CPU, all executions in one fixed-seed shuffled order (20261124).

**Host deviation.** The owner asked for RunPod. The account balance is negative (−0.32, `myself.clientBalance`, 2026-11-19), so no pod can
be rented. The round runs on this session's container (Intel Xeon @ 2.10 GHz, 4 vCPU), as the scale round did under
its amendment A1. Rerunning on a pod is the same frozen `run.py` with no other change.

## Measures and gates

Cost of a key on an instance = mean `ms_arm` over its 32 seeds (the expected cost per target). Best IC and best rho
are the lowest-mean IC and batched-rho keys per instance. Cell = size; cell value = median over its 4 instances; a
cell decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers its target, and every key has 32 executions per
  instance.
- **Primary (scaling):** best IC / best rho, cell medians.
  - **gap grows** if 2^32 < 2^36 < 2^40;
  - **gap shrinks** if 2^32 > 2^36 > 2^40;
  - else **mixed**.
- **Distance:** best IC / best rho per cell, with "IC ahead" (≤ 1, 4 of 4) and "IC behind" (> 1, 4 of 4).
- **Secondary:**
  - serial rho / batched rho (what lanes buy single-target rho);
  - each μ's cell median of means;
  - the best keys.

Priors (stated before data): gap grows, 0.7. IC behind in all three cells, 0.95. IC ahead in any cell, 0.02.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report, and the targets are
  planted on generated instances.
- This IC is a generic combination-table index calculus with no point decomposition (no summation polynomials). Its
  single-target result says nothing about decomposition-based index calculus.
- Wall time on one container, one binary.

## Amendment A1 (2026-11-19, before any data was read)

The first main run, started 17:36Z, was stopped at 17:41Z after 1,954 of 5,376 executions. During 17:37–17:41Z this
session ran `cargo check` and `cargo clippy` builds on the other cores, and one was killed for lack of memory. That
load could bias the timings, the memory-sensitive IC arm most. Its partial rows were set aside unread (not copied
into this directory). The main run restarts from scratch, unchanged in every other respect: same binary, keys,
seeds and shuffled order. No build or other heavy work runs on the container while it measures.
