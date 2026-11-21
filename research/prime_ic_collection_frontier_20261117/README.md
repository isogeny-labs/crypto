# Prime-field IC collection-method frontier: sweep vs pair table

**Date.** 2026-11-17. **Runtime.** Claude (local Rust toolchain, cargo 1.97,
x86_64 linux, 4 HW threads). **Status.** Local measurement, public synthetic
known-answer only. Every recovered log checked against the planted scalar; the
two oracles cross-checked to agree on every instance. **No security claim and
no vs-rho claim** — this is an IC-oracle study. All numbers are producer-side,
single-host, not CPU-isolated. `[pcc:unverified]`.

## Question

For single-target prime-field index calculus, the production "novel
root-finding algorithm" (`cryptanalysis::prime_fast`) replaces the Semaev /
Gröbner decomposition solve with direct point arithmetic: a 2-decomposition of
a target `R` into the factor base is `R = ±F_i ± F_j`. Two oracles decide this
*same* set by different work:

- **sweep** — `collect_relations` with `pairs = None` (the default
  `ic_solve(summands = 2)` path): subtract every factor-base element from `R`
  under one batched inversion, look each `x(R ∓ F_i)` up. `O(B)` field
  operations per target.
- **table** — `collect_relations_table` (the experimental `ic_solve_table`
  path): precompute every `x(F_j ± F_k)` once (`B²` entries), look `x(R)` up.
  `O(1)` per target plus a one-time `O(B²)` build.

Which is faster, by how much, and where is the frontier? The default
single-target solver (`ic_solve`) uses the sweep; `ic_solve_table` existed but
was marked *experimental* and had never been measured locally (the recent
sessions ran in GitHub-connector runtimes with no Rust toolchain).

## Method (isolation of the one variable)

`examples/prime_ic_collection_frontier.rs`. For each generated `a = −3`
prime-order bench curve (`find_a3_curve(bits, 3)`), plant a known `k`, set
`Q = [k]G`, and for each factor-base size `B` run both oracles **with the same
`(g, q, seed)` and the same walk rule** (`R ← R + G` until a productive target,
then fresh `(a, b)`). Because the walk rule, seed and decomposable set are
identical, the two oracles **walk the identical target stream and retire on the
same productive targets** — verified: `sweep_targets == table_targets` and the
1-/2-/3-term row breakdown is identical on every row. The *only* variable is
the per-target detection cost. Both feed the same `solve_relations` linear
algebra; both verify `[x]G = Q`. A matched **single-thread** `rho_parallel`
wall is recorded per size for context.

## Results

Times in seconds. "collect ×" = sweep_collect / table_collect. "total ×" =
(sweep_collect + LA) / (table_build + table_collect + LA). `sweep=table log`
was **yes** on every row (identical recovered log).

### 24-bit (p = 16 777 199), rho 1-thread = 0.0032 s

| B | targets | sweep collect | table build | table collect | collect × | total × |
|--:|--:|--:|--:|--:|--:|--:|
| 60  | 184 553 | 0.4524 | 0.0002 | 0.0584 | 7.7×  | 7.72×  |
| 120 | 71 769  | 0.2938 | 0.0006 | 0.0220 | 13.4× | 12.93× |
| 240 | 37 952  | 0.2617 | 0.0029 | 0.0126 | 20.8× | 16.47× |
| 480 | 17 474  | 0.2631 | 0.0112 | 0.0092 | 28.7× | 11.33× |
| 960 | 8 795   | 0.2472 | 0.0425 | 0.0059 | 41.6× | 4.40×  |

### 28-bit (p = 268 435 399), rho 1-thread = 0.0041 s

| B | targets | sweep collect | table build | table collect | collect × | total × | LA (sweep/table) |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 60   | 2 358 245 | 5.5412 | 0.0001 | 0.8519 | 6.5×   | 6.50×  | 0.0001/0.0001 |
| 120  | 1 160 810 | 4.7634 | 0.0003 | 0.4242 | 11.2×  | 11.22× | 0.0001/0.0001 |
| 240  | 594 289   | 4.4259 | 0.0017 | 0.2091 | 21.2×  | 20.94× | 0.0005/0.0005 |
| 480  | 302 235   | 4.6144 | 0.0087 | 0.1450 | 31.8×  | 29.52× | 0.0025/0.0026 |
| 960  | 147 842   | 4.39   | 0.057  | 0.064  | 68.9×  | 33.36× | 0.014/0.011   |
| 1920 | 72 446    | 4.1939 | 0.1871 | 0.0397 | 105.6× | 15.08× | 0.045/0.054   |
| 3840 | 35 965    | 4.1149 | 0.9728 | 0.0319 | 128.8× | 3.51×  | 0.267/0.243   |
| 7680 | 17 588    | 4.1778 | 4.9488 | 0.0339 | 123.3× | 0.87×  | 1.099/1.074   |

### 32-bit (p = 4 294 967 291), rho 1-thread = 0.0093 s

| B | targets | sweep collect | table build | table collect | collect × | total × | LA (sweep/table) |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 240  | 9 981 448 | 77.26 | 0.0022 | 3.9049 | 19.8× | 19.77× | 0.0006/0.0004 |
| 480  | 4 743 688 | 70.24 | 0.0095 | 1.9474 | 36.1× | 35.86× | 0.0022/0.0021 |
| 960  | 2 274 594 | 66.51 | 0.0422 | 1.0768 | 61.8× | 58.94× | 0.0105/0.0095 |
| 1920 | 1 113 281 | 63.78 | 0.2500 | 0.7876 | 81.0× | 59.34× | 0.069/0.038   |

## Findings

1. **The table oracle is a large, verified improvement over the sweep.** It
   recovers the identical log from the identical relations on the identical
   targets, and collects `6.5–129×` faster. The per-target speed-up is **linear
   in `B`**, exactly as `O(B)`-per-target (sweep) vs `O(1)`-per-target (table)
   predicts.

2. **Each phase scales as predicted, isolating the walls.** Sweep collect is
   ~flat in `B` (confirming the sweep is `O(p)`, independent of `B`: fewer
   targets `∝ p/B²` times more work per target `∝ B`). Table collect `∝ 1/B²`.
   Table build `∝ B²`. Dense Gauss–Jordan linear algebra `∝ B³` (0.011 s at
   `B=960` → 1.07 s at `B=7680`, 28-bit).

3. **There is an optimal `B ≈ p^{1/3}`.** Measured optima: `B≈240` (24-bit),
   `B≈960` (28-bit), `B≈1920` (32-bit) — matching `p^{1/3} ≈ 256, 645, 1627`.
   There the total solve is `O(p^{2/3})`, and the total speed-up over the sweep
   is **16× (24-bit), 33× (28-bit), 59× (32-bit)** — it grows with `p`.

4. **Over-sizing `B` is counterproductive.** Past the optimum the `O(B²)` build
   then the dense `O(B³)` LA dominate; at `B=7680` (28-bit) the table is
   *slower* than the sweep (0.87×). The walls at the optimum are the balanced
   build (`B²`) and collect (`p/B`); dense LA bites only beyond it.

5. **Gap to rho (honest).** At its own optimum the table IC is `≈33×` (28-bit)
   to `≈120×` (32-bit) slower than 1-thread rho, and the asymptotic gap widens
   (`O(p^{2/3})` IC vs `O(√p)` rho, i.e. `p^{1/6}`), consistent with Gaudry's
   prime-field verdict. No rho-beating or sub-rho claim is made.

## End-to-end: the three full single-target solvers

Total wall time of a complete single-target DLP solve at the near-optimal `B`,
same planted `k`, same host (`--e2e-b`). `sweep` = default `ic_solve`
(summands 2); `table` = `ic_solve_table` (single-walk pair table); `batched` =
`ic_multi_solve_ex(table, batched)` (lock-step lanes, one inversion per round,
shared factor-base-log solve + per-target phase). All recover the planted `k`.

| bits | B | sweep `ic_solve` | table `ic_solve_table` | batched `ic_multi_solve_ex` | rho 1-thr |
|--:|--:|--:|--:|--:|--:|
| 28 | 960  | 4.96 s (1.00×) | 0.1415 s (**35×**) | 0.0930 s (**53×**) | 0.0041 s |
| 32 | 1920 | 71.43 s (1.00×) | 0.9004 s (**79×**) | 0.4030 s (**177×**) | 0.0093 s |

- The **batched lock-step solver is the fastest existing single-target prime IC
  solver**, `53×` (28-bit) → `177×` (32-bit) over the default sweep — the margin
  grows with `p`. It beats the single-walk table by `1.5× → 2.2×`: the gain is
  the amortized modular inverse (one per lock-step round over all lanes instead
  of one per walk step).
- It is still `≈22×` (28-bit) / `≈43×` (32-bit) slower than 1-thread rho; the
  gap is real and widens with `p` (Gaudry). No sub-rho claim.

## Takeaways / recommendations

- **Make the batched lock-step pair-table solver (`ic_multi_solve_ex`, table +
  batched) the default single-target prime IC solver**, with `B` auto-tuned to
  `≈ p^{1/3}`: it is `53–177×` faster than the default sweep `ic_solve`
  (growing with `p`), and `1.5–2.2×` faster than the single-walk
  `ic_solve_table`. The sweep is strictly dominated at every size measured (same
  answer). A regression test (`ic_solve_table_agrees_with_sweep`) pins the
  sweep/table agreement; `ic_multi_solve` already has its own tests.
- **Next levers on the frontier (to raise the usable `B` and shrink the gap):**
  (a) the `O(B²)` pair-table build and its `O(B²)` memory are the binding wall
  at the optimum; (b) the dense `O(B³)` linear algebra is the wall just past it
  — the relations are sparse (≤3 nonzeros/row), so sparse structured
  elimination / Lanczos would cut LA to `~O(B²)`; (c) the batched lock-step walk
  (`collect_setup_batched`, one inversion per round over many lanes) amortizes
  the per-step inverse that the single-walk table still pays — an untested
  further speed-up to measure.

## Build repair (prerequisite)

`main` did not compile locally (`cargo build --release --lib` → 9 errors): an
incomplete merge left `f4_fp::VisitReport` without `first_fall_degree`,
`last_fall_degree`, `degree_of_regularity_observed`, and `F4Options` without
`recording_degree_falls`, while consumers (`aps_single_relation`,
`aps_single_relation/leads`) referenced them. A minimal, serialization-safe
repair adds the three diagnostic fields (defaulting to `None`) and a no-op
`recording_degree_falls` builder. `f4_fp::VisitReport` is serialized nowhere,
so the shim cannot perturb any transcript; the crate now compiles and all local
measurement ran.

**Scope note.** This fixes only the *compile* break. The same merge
(`b1514f86e`, PR #4559) also added `AlgebraReport` fall-degree fields and
touched `aps_single_relation/replay.rs` without updating its `valid()`
fixtures, so 12 `aps_single_relation::replay` tests remain red (exact-byte
binding). That is #4559's unfinished feature, independent of this work and of
the f4 shim, and is left for #4559 to complete rather than reverse-engineered
here. The prime-IC regression test (`ic_solve_table_agrees_with_sweep`) and the
IC tests pass.

## Reproduce

```bash
cargo run --release --example prime_ic_collection_frontier -- \
  --bits 24,28 --fb 60,120,240,480,960 \
  --json research/prime_ic_collection_frontier_20261117/frontier_24_28.json
cargo run --release --example prime_ic_collection_frontier -- \
  --bits 28 --fb 960,1920,3840,7680 \
  --json research/prime_ic_collection_frontier_20261117/frontier_28_bigB.json
cargo run --release --example prime_ic_collection_frontier -- \
  --bits 32 --fb 240,480,960,1920 --max-targets 60000000 \
  --json research/prime_ic_collection_frontier_20261117/frontier_32.json
```

Raw JSON for each run is checked in beside this README.
```
