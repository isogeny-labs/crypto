# 2026-11-17 — Prime-field IC collection-method frontier (sweep vs pair table)

Runtime: Claude (Fable 5.2 / Opus 4.8 serving), local Rust toolchain available
(cargo 1.97, x86_64 linux) — unlike the recent GitHub-connector sessions that
could not build. Branch `claude/prime-rootfind-fastest-20261117`, PR #4572
(draft). Task: make the prime-field IC root-finding decomposition oracle the
fastest; local baseline + measured optimization.

## What was done (commands, files, features)

- Routed the task (`bin/ws route`) to `crypto`; reused the trunk's sparse
  submodule checkout. Read the prime engine (`prime_fast.rs`), the univariate
  root finder (`prime_root_trace.rs`, Shoup 20.4 CZ — off the IC hot path, used
  only for ml_dataset certificates), and the 2026-10-12 research note
  `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`.
- Found the production "novel root-finding algorithm" for prime-field IC is the
  **direct-subtraction decomposition** in `prime_fast`: the Semaev `S₃` root in
  the third variable is `x(R ∓ F_i)`, so a 2-decomposition is point arithmetic +
  factor-base membership, no polynomial root finding. Default single-target
  entry `ic_solve` uses the **sweep** (`collect_relations`, O(B)/target);
  `ic_solve_table`/`collect_relations_table` (table, O(1)/target + O(B²) build)
  existed but was *experimental* and never locally measured.
- **Build repair (prerequisite).** `main` did not compile locally: incomplete
  merge left `f4_fp::VisitReport` without `first_fall_degree`,
  `last_fall_degree`, `degree_of_regularity_observed` and `F4Options` without
  `recording_degree_falls`, while `aps_single_relation`(+`/leads`) referenced
  them (9 errors). Added the three `Option<u32>` diagnostic fields (default
  `None`) and a no-op `recording_degree_falls` builder — behaviour-preserving;
  the crate now builds. `cargo build --release --lib` → Finished.
- Wrote `examples/prime_ic_collection_frontier.rs`: an isolated A/B of sweep vs
  table collection at fixed B and fixed decomposition set, same `(g,q,seed)` and
  walk rule → identical target stream (verified `sweep_targets==table_targets`
  and identical 1/2/3-term row breakdown), feeding the same linear algebra, with
  a matched 1-thread rho for context. Every recovered log checked == planted k.
- Ran 24/28/32-bit frontiers (`research/prime_ic_collection_frontier_20261117/`,
  JSON + README + SVG). Added regression test
  `prime_fast::tests::ic_solve_table_agrees_with_sweep` (16/20/24-bit: table log
  == sweep log == k, same relation and target counts).

## Decisions made (and why)

- Attack the on-hot-path decomposition oracle, not the univariate CZ root finder
  (`prime_root_trace`), which is off the prime-IC path (ml_dataset certificates
  only). The sweep's per-element arithmetic is already near-minimal (~7 muls +
  2 Montgomery reduces/FB-element); the real lever is the collection *method*.
- Fix the `f4_fp` main breakage with a minimal defaulted/no-op shim rather than
  implement F4 fall-degree recording (unknown intent) or edit the consumers:
  smallest behaviour-preserving change that unblocks the build and CI. Distinct
  from the corrupted `sat.rs` a prior session (rightly) refused to patch — this
  is a clean post-merge field/method mismatch.

## Results (producer-side, single-host, not CPU-isolated; [pcc:unverified])

- Table collection beats the sweep by **6.5–129×**, speed-up **linear in B**
  (O(B)→O(1) per target). Identical relations/targets/log on every row.
- Phase scaling confirmed: sweep collect ~flat (O(p)); table collect ∝1/B²;
  table build ∝B²; dense Gauss–Jordan LA ∝B³ (0.011 s→1.07 s, B=960→7680, 28-b).
- **Optimal B ≈ p^{1/3}** (≈240/960/1920 at 24/28/32-bit); total solve O(p^{2/3});
  **max total speed-up over the sweep: 16× / 33× / 59×** (grows with p). Past the
  optimum, B² build then B³ LA dominate — at B=7680 (28-b) the table is *slower*
  than the sweep (0.87×).
- Gap to 1-thread rho: ≈33× (28-b) to ≈120× (32-b), widening (IC p^{2/3} vs rho
  √p, i.e. p^{1/6}) — Gaudry's prime-field verdict confirmed. No sub-rho claim.
- **End-to-end (full single-target solve, near-optimal B):** batched lock-step
  pair-table solver (`ic_multi_solve_ex`, table+batched) is the fastest existing
  path — **53× (28-b) / 177× (32-b)** over the default sweep `ic_solve`, and
  1.5×/2.2× over the single-walk `ic_solve_table` (amortized inverse: one per
  lock-step round vs one per walk step). Margin over the sweep grows with p.
  Still ≈22×/43× slower than 1-thread rho.

## Open threads (unfinished, pick up next time)

- Promote the **batched** `ic_multi_solve_ex` (table+batched, B auto-tuned
  ≈ p^{1/3}) to the default single-target prime IC solver; the sweep is strictly
  dominated (53–177×) and the single-walk table by 1.5–2.2×. (Default not yet
  switched — measurement + regression test only; wiring a `strategy` default
  into `ic_solve`/callers is the next code step.)
- Frontier walls to attack next: (a) O(B²) pair-table build + memory (binding at
  the optimum); (b) dense O(B³) LA just past it — relations are sparse (≤3
  nonzeros), so sparse structured elimination/Lanczos → ~O(B²); (c) measure the
  batched lock-step walk (`collect_setup_batched`, one inversion/round over many
  lanes) vs the single-walk table — amortizes the per-step inverse.
- Larger sizes (36/40-bit) table-only, to confirm the p^{2/3} scaling and B_opt
  ≈ p^{1/3} law, and to quantify the p^{1/6} rho gap at scale.
- A genuinely-novel item deferred: a radical/closed-form univariate root finder
  for the low-degree (d≤4) Semaev polynomials in `prime_root_trace`'s regime
  (quadratic/cubic/quartic via modular sqrt/cbrt) — predicted ~10–15× fewer
  field muls than Shoup 20.4 CZ; relevant to the Semaev-poly IC and certificate
  paths, not the subtraction hot path.

## Errors (what went wrong, how it was handled)

- `main` lib did not compile (f4_fp merge mismatch). Repaired the **compile
  break** minimally (above); rebuild clean. My first `cargo build --release
  --lib | tail` reported exit 0 from the pipe (tail), masking cargo's 101 —
  re-ran filtered to get the truth.
- **Pre-existing deeper breakage (NOT this work, out of scope): merge
  `b1514f86e` / PR #4559 ("happy-johnson") landed a half-finished fall-degree
  feature.** It added `AlgebraReport::{first_fall_degree,last_fall_degree,
  degree_of_regularity_observed}` and the `f4_fp` references (the compile errors
  I shimmed) AND touched `aps_single_relation/replay.rs`, but did not update the
  `valid()` replay fixtures, so 12 `aps_single_relation::replay::tests` fail
  (exact-byte binding: the serialized `AlgebraReport` now carries fall-degree
  fields the fixtures lack). Confirmed independent of my shim: `f4_fp::VisitReport`
  is serialized nowhere (no fixture/byte-binding), so my added fields cannot
  perturb any transcript. Finishing #4559 (implement f4 fall-degree recording
  and regenerate its fixtures) is #4559's task, not reverse-engineered here —
  touching those fixtures blind would risk masking the real breakage
  ("sabotage is assumed"). My prime-IC tests pass; these 12 remain red on main
  until #4559 is completed.
- No matplotlib locally → generated the frontier SVG with a dependency-free
  Python emitter.
- No Conductor/PCC/S3/KB access in this runtime (as prior sessions noted); kept
  evidence local under `research/` and labelled everything [pcc:unverified].
