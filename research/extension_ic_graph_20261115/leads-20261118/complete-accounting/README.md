# Complete accounting: closing the descent's linear-stage gap and recounting Run50

Lane key `complete-accounting`, 2026-11-18. Every number here is a producer
observation labelled [pcc:unverified]. No pcc claim was registered. No paid
compute, Runpod, EKS or protocol dispatch was used. The local runs are counted
work on recorded or reconstructed inputs. They make no timing claims.

**This is accounting, not an improvement.** No algorithm changed. Each total
below is larger than or equal to the recorded subtotal, and every arm is still
a lower bound.

## Question

Where do the graph-IC pipeline and its matched rho references do native work
that the `jv_bitops` hooks miss? How much of that work can be recovered from
recorded data? I wanted to close the largest gap with counters written in
Rust, checked by exact conservation tests, and then recount Run50 per phase at
0, 0.1 and 1 GAE per uncounted native operation (AGENTS.md §5a).

## Inventory of native work and its counting status

Sources read:
- Run50 `README.md`, `NEXT.md`, `run-study.json`, `qualified-original.json`,
  `original-ic.raw.json` and `rho-*.raw.json`
- `AMENDMENT_25`, `AMENDMENT_27`, `NATIVE_LA_COUNTING_REVIEW.md`,
  `STRONG_RHO_ACCOUNTING_FOLLOWUP.md`, `NATIVE_RHO_PRICE_REVIEW.md`,
  `COUNTING_COVERAGE.md`, and the Run46 and Run43 READMEs
- `descent-finder/README.md` and `CANDIDATES.md`
- the code: `zero_dim_la.rs`, `jv_bitops.rs`, `jv_bitops_uniform.rs`,
  `bitops.rs`, `jv_cover.rs` (`linear_stage_from_basis`, the dense stage),
  `f4_fp::eval`, `gaudry_cubic::PolyRing`, `residual_walk::inv_mod`

"R" is the recorded observed subtotal.

| # | Arm(s) | Phase | Native operation | Status in Run50 | Gap size recoverable from recorded data |
|---|---|---|---|---|---|
| 1 | IC (all four) | descent | Linear stage `zero_dim_la::fast_stage` `F_p` arithmetic: `Fp::mul/add/sub`, lazy dot/axpy kernels (NF rows, transposed Krylov, BM, `x^p mod f`, gcd, read-out) | **uncounted**. Only `inv_mod` and `PolyRing` are hooked. | Graph arms: **+942,912,085,926 bitops**, by replaying the reconstructed Run50 stream with the new counters. That is +27.6% on the stream's pre-patch hooked descent, and 21.6% of the corrected graph descent. Original IC: ESTIMATE 477,870,025,544, from the recorded `descent_lin_muls` 347,909,918. |
| 2 | IC | descent | Linear-stage words, probes and RNG: staircase and NF divisibility tests, memo/index probes, non-zero scans, `StdRng` draws | **uncounted** | Stream: 28,428,573 words, 4,810,782 probes and 223,119 draws. These are now counted (included in #1). |
| 3 | IC | descent | Candidate check `f4_fp::eval` (unhooked `mulmod`/`powmod`) | **uncounted** | Counted by the patch; part of #1's `mul_outside_legacy` (8,358,644 products, which also include normalizations and divisions). |
| 4 | IC | descent | Dense fallback stage (`jv_cover`) | hooked (`mm`/`am`), except its `f4_fp::eval` | 5 fallbacks on the stream and 3 in original IC. The eval gap is not recovered (null; small). |
| 5 | IC | descent | `PolyRing::roots` RNG draws; staircase setup copies and allocations | uncounted | null |
| 6 | IC | descent | Stream advances and accepted-row verification | counted in R | Explains the 0.30% gap between replay and Run50 (for example `modr_add` 6,828 = 2 × 3,414 advances). |
| 7 | IC | all phases | Uniform-sampler native integer mul64/add64 | priced but **excluded from R** | Recorded `known_native_priced_bitops` per arm (column U below). |
| 8 | IC | all phases | Uniform-sampler integer sub64, clz64, shl64 | **unpriced**, counts recorded | 10,664 / 13,976 / 3,146,544 / 2,647,504 ops (original / table / grounded / deferred). These go into the §5a band. |
| 9 | IC + rho | all phases | PRNG generator internals (ChaCha12 behind each draw) | uncounted | null. Only the draw counts are known (`rng_words`, `raw_draws`). |
| 10 | IC graph | relations | Graph containers: maps/sets, witness storage, activation queue, sort/clone/normalization/scheduling | uncounted | null. Event counts are recorded but have no per-event operation model, for example deferred: 125,491 vertices, 77,994 edges, 8,145 activated, 297,157 candidates, 206,280 refused, 251,537 discriminant calls. |
| 11 | IC | relations | Cohort hashing and serialization | uncounted | null |
| 12 | IC | linear_algebra | Wiedemann, BM and Lanczos modular arithmetic and data scans | counted (amendments 25 and 26) | none |
| 13 | IC | linear_algebra | Incremental RREF scans and fixed-factor internals; "solver native bookkeeping" | uncounted | null |
| 14 | IC graph | linear_algebra | `kernel_and_projection_events` (12 additions, 12 products, 5,611 word events) | recorded | Whether they are inside R was not verified here. |
| 15 | traced descent (not Run50) | descent | `f4_fp` learning-run tracked RREF | uncounted | Only an upper bound exists (descent-finder). |
| 16 | IC + rho | harness | Instance generation, domain checks, calibration | excluded by design | Recorded as `excluded_harness_primitives`. |
| 17 | rho | walk/collision | Native Barrett tower arithmetic, step hashes, DP tests, table probes | counted (Amendment 27 calibration; about 6 `word_op` per step in walk and 7 in collision) | none |
| 18 | rho | all phases | `tuned_reference_bitop_pricing`, `rho_setup_and_native_bookkeeping_audit`, `rng_internal_work` | named, **count null** in every rho raw | null. Under §5a no arm can be ranked. |
| 19 | rho | collision | DP table container internals beyond the counted probes and words | uncounted | null |

The only gap with a recorded count of unpriced native operations is #8. The
large gap (#1–#3) has no count in any recorded Run50 file. It was recovered
here by replay.

## New counters (Rust patch; shared `src/` is not edited)

[`counted-linear-stage/zero_dim_la.counted.patch`](counted-linear-stage/zero_dim_la.counted.patch)
is 572 lines. It is a patch against `src/cryptanalysis/zero_dim_la.rs`, whose
SHA-256 `5ae85b34…` is byte-identical at Run50's `11c9a031`, the frozen
`fcda1e22` and current main. `git apply --check` passes on current main. The
patched source is `counted-linear-stage/zero_dim_la.rs`.

How the patch counts:

- **Operation-level counting.** A thread-local `LinOps` counts work at the
  operation itself, not by formula at the call site:
  - each `Fp::mul` is a `mul`
  - each `Fp::add` or `Fp::sub` is an `add`; so are inline negations and the
    lane doubling in `square_into`
  - each lazily accumulated product is a `mac`; kernels count their length
  - divisibility tests and non-zero scans are `word`s
  - each `FxMap` insert or lookup is a `probe`
  - each `StdRng` draw is an `rng`
- **Charging.** `fast_stage` charges the per-call difference through the
  existing hooks: `field_mul = mul + mac` and `field_add = add + mac` (F4's
  multiply-subtract convention), plus `words`, `probes(n, 1)` and
  `rng_words`.
- **What is not charged.** Reductions of lazy sums are not charged again.
  `inv_mod` stays hooked once. A pool-thread `dot_nc` is charged by its
  caller, so no tally is foreign.
- **Reconciliation counters.** `mul_outside_legacy`, `legacy_overcount` and
  `ring_muls` are not charged. They exist so the legacy `FastStats::muls()`
  can be reconciled exactly.
- **Legacy formula discrepancies found.** The legacy formula
  - misses products in basis normalization, gcd normalization, coordinate
    divisions, the multiple-point ψ and the candidate check;
  - overcounts one product per root: the `Q` chain runs δ−1 products but δ is
    counted.
- **Unchanged outputs.** `FastStats`, its serialization and every output are
  unchanged.

Exact conservation tests, all passing (`build/build.log`):

- **Hand controls in the patch** (`zero_dim_la::tests::counted_*`):
  - BM on `(1, 2)` over `F_101` gives `x − 2` with 4 `mul`, 3 `add`, 1 `mac`
    and legacy 5 = `mac + mul`.
  - The kernels count exactly their lengths; reductions count 0.
  - Every legacy-counted product of `pmul`, `ModF` and the powering equals
    `mac + mul`.
- **Differential test** (`conservation_test.rs`): 108 random planted
  zero-dimensional systems (p ∈ {101, 1009, 65521}, k = 2..4; 107 with roots,
  1 give-up). The frozen and counted `fast_stage` run on the same basis with
  cloned RNGs. The checks:
  - (d) outputs and legacy counters are identical
  - (a) tally(counted) − tally(frozen) equals the charged `LinOps` exactly for
    every primitive, and is zero for every other kind
  - (b) legacy `muls()` + `mul_outside_legacy` = `mac + mul +
    legacy_overcount + ring_muls`
- **On the Run50 stream** (`linear_stage_recount.rs`): (a), (b), (d) and
  (c) pass on all 3,415 residuals with 0 failures and 0 foreign hooks.
  - (c): the pipeline's own linear-stage tally equals the counted harness
    tally whenever the stage did not fall back.

## Results

### Linear stage on the reconstructed Run50 stream (seed 389, 3,415 residuals)

`runs/recount-seed389-n3415.json`. This is a local replay, not a recorded
Run50 measurement. The reconstruction matches the recorded run:

- decompositions occur at residuals 263 and 3414, as recorded;
- the pre-patch hooked descent is 3,416,084,890,482, which equals
  descent-finder's production arm exactly;
- `hash_probe` 199,017,146 equals Run50's recorded descent value exactly.

| | Pre-patch hooks | With counted linear stage | Increment |
|---|---:|---:|---:|
| field_mul | 2,452,814,393 | 3,143,773,798 | 690,959,405 |
| field_add | 2,449,914,314 | 3,147,591,146 | 697,676,832 |
| word_op | 1,076,590,863 | 1,110,053,337 | 33,462,474 |
| hash_probe | 199,017,146 | 203,827,928 | 4,810,782 |
| descent bitops | 3,416,084,890,482 | 4,358,996,976,408 | **942,912,085,926** |

The descent-finder lower bound for this gap was 837.7e9. The exact count is
12.5% higher, because it includes additions, words, probes and the products
the legacy formula missed.

### Run50 per-phase recount (`runs/run50-recount.json`; `accounting_recount.rs`)

This is a recount of recorded data plus the replay-derived term L.

- Every recorded phase was repriced in bitop/v1 at b = 10 and r = 58, and
  equals its recorded `diagnostic_priced_bitops`.
- U and the unpriced operations come from the recorded uniform-sampling
  records.
- 1 GAE is one affine E(F_{p^6}) addition priced from the rho reference's
  recorded native tower calibration: 245 `field_mul` + 210 `field_add` +
  1 `field_inv` = **331,250 bitops**.

| Arm | R recorded | U uniform priced (excluded from R) | L linear stage | Unpriced ops | Lower bound w=0 | w=0.1 | w=1 |
|---|---:|---:|---:|---:|---:|---:|---:|
| original IC | 4,525,556,132,414 | 67,411,584 | 477,870,025,544 (ESTIMATE) | 10,664 | 5,003,493,569,542 | 5,003,846,814,542 | 5,007,026,019,542 |
| quadratic table | 5,230,830,441,710 | 90,210,368 | 942,912,085,926 | 13,976 | 6,173,832,738,004 | 6,174,295,693,004 | 6,178,462,288,004 |
| grounded pairs | 6,235,988,188,572 | 19,624,869,440 | 942,912,085,926 | 3,146,544 | 7,198,525,143,938 | 7,302,754,413,938 | 8,240,817,843,938 |
| deferred pairs | 5,913,875,255,822 | 16,513,505,984 | 942,912,085,926 | 2,647,504 | 6,873,300,847,732 | 6,960,999,417,732 | 7,750,286,547,732 |
| rho plain b1 | 196,545,134,504,212 | 0 | 0 | 0 (3 named gaps, count null) | same | same | same |
| rho neg b1 | 181,105,019,144,798 | 0 | 0 | 0 (null) | same | same | same |
| rho plain b16 | 109,992,238,398,614 | 0 | 0 | 0 (null) | same | same | same |
| rho neg b16 | 274,525,819,370,690 | 0 | 0 | 0 (null) | same | same | same |

Per phase, the graph arms change only in descent: 3,426,280,619,436 becomes
4,369,192,705,362. U is arm-level, because the records do not attribute the
sampler events to phases. Descent is now 63.6% of the deferred arm's w=0
lower bound.

**Distance to rho, a ratio between two lower bounds.** Deferred pairs over
the cheapest rho (plain b16) is 0.0625, 0.0633 and 0.0705 at w = 0, 0.1 and 1.
Over all four rho roles it ranges from 0.025 to 0.070. Both sides keep
null-count gaps (#5, #9–#11, #13 and #18–#19), so §5a forbids ranking and no
parity statement follows. This is one exposed p1009 target. The ratios are not
a cost comparison.

Deferred pairs stay below grounded pairs at every weight. The difference grows
from 325.2e9 at w=0 to 490.5e9 at w=1, because the deferred arm has fewer
sampler events. This only restates the Run50 relation-phase saving under
fuller accounting; it is not a new improvement.

**Effect on descent-finder's C1 (derived, not re-run).** Its traced and
production arms share the linear stage (identical `lin_muls`). Adding the
exact counted increment instead of the `fp_mul` lower bound changes the
production/traced descent ratio from 1.79x to **1.755x**. With the
learning-RREF bound it changes from 1.71x to **1.679x**. This is
[pcc:unverified] and assumes the traced arm's linear-stage work equals
production's, which the equal `lin_muls` support but do not prove.

## Commands run

1. `bash build/build.sh` with `SCRATCH=<scratchpad>/ca` and
   `CARGO_TARGET_DIR=/home/user/cargo-target-round2`. It checks for at least
   5 GB free on `/`. Then:
   - `fetch_frozen.sh` takes read-only `git show` copies of `fcda1e22`'s
     descent closure;
   - `assemble.py` applies the descent-finder mini-build boundary with the
     nagao-e2e `jv_cover` overlay, the counted `zero_dim_la.rs`, the frozen
     original as `zero_dim_la_orig.rs`, the bitops unit vectors, and
     `ops.rs` without its curve-dependent test module;
   - `cargo test --release --lib zero_dim_la::tests`
   - `cargo test --release --test conservation`
   - `cargo build --release --bin linear_stage_recount --bin accounting_recount`

   SHA-256s of the sources, `Cargo.lock` and the binaries are at the end of
   `build/build.log`. `Cargo.lock` `3d1cb733…` is the same as
   descent-finder's. The build target is 132 MB.
2. `linear_stage_recount ../../runpod-deferred-pair-20261117-50/public.json 389 3415 runs/recount-seed389-n3415.json`
   (13.1 s on one thread; indicative only).
3. `accounting_recount <Run50>/qualified-original.json <Run50>/original-ic.raw.json runs/recount-seed389-n3415.json > runs/run50-recount.json`
4. Hashes are in `runs/RAW.sha256`.

## Failures, deviations and limits

- **First conservation run failed (b) by 4 products** (p=101, k=2). The cause
  was the uncounted coordinate divisions `fp.mul(ψ, 1/den)` in the read-out.
  I added the reconciliation term and the test passed. This is recorded here
  rather than hidden.
- **First lib-test build failed** because `bitops.rs` tests include
  `docs/units/*.vectors.json` and the `ops.rs` tests need curve types. The
  assembly now copies the vectors and drops the `ops.rs` test module. The
  descent-finder's `Cargo.lock`, and its worktree paths under
  `.claude/worktrees/`, no longer exist. This lane's scripts use repository
  paths.
- **Hybrid build boundary.** The build is descent-finder's frozen `fcda1e22`
  closure with byte-copied shims. It is not a full crate build and not
  Run50's exact binary. Run50's descent closure has the same
  `zero_dim_la.rs`, `jv_bitops.rs` and `jv_cover.rs`; Run50 (`11c9a031`) has a
  different `f4_fp.rs` hash. The replay matching the recorded `hash_probe`
  exactly, and Run50's descent within 0.30%, supports this boundary.
- **L for the graph arms comes from a replay, not from recorded files.**
  Run50's graph-arm raw witnesses are on the deleted pod or LOCAL and absent
  here. L for original IC is a proportional **estimate**, because its
  residual stream (1,728 residuals) was not replayed.
- **Counting conventions.** A `mac` is charged as a `field_mul` plus a
  `field_add`; this overcounts by at most one add per reduced output. A
  divisibility test counts as one word even when it compares up to n `u32`
  exponents. `gen_range` rejection draws are not visible. All three are
  declared in the patch header.
- **research-kb unavailable.** `scripts/research-kb` has no executable, and
  no research-kb MCP tools are loaded (`runs/kb-receipt.log`). Sources read
  are listed above. This lane makes no literature or novelty claim.
- No complete total exists for any arm, and no ranking or parity claim is
  made.

## Next steps

1. Land the patch in shared `src/` (owner or the lane that owns
   `zero_dim_la.rs`), with its three `counted_*` tests. Port the differential
   test into `tests/` against a frozen copy. Then every future descent
   subtotal includes the linear stage with no estimate.
2. Replay original IC's 1,728-residual stream with the counted stage. That
   needs the `jv_sieve_two_large_prime` residual derivation from the
   nagao-e2e overlay. It replaces the 477.9e9 estimate with a count.
3. Emit counts for the null gaps, in order of likely size:
   - graph containers (#10), with per-event operation counts at the map, set
     and queue call sites;
   - incremental RREF scans and fixed-factor internals (#13);
   - PRNG internals (#9), as a declared per-draw cost of the ChaCha12 block;
   - rho's three named gaps (#18).

   Until both sides emit them, §5a ranking stays impossible.
4. Count the dense fallback's `f4_fp::eval` (#4) and `PolyRing` RNG draws
   (#5) in `jv_cover` and `gaudry_cubic`.
5. Measure a per-host `w_cal` (mulmod/GAE) with `perf-ipc` on an isolated
   host when one is admitted, as an extra point beside the band.
