# Descent finder: where the descent work goes, and F4 trace replay as an opt-in path

Lane key `descent-finder`, 2026-11-18. All results are producer observations,
labelled [pcc:unverified]. No pcc claim was registered. No paid compute, Runpod,
EKS or experiment dispatch was used. The local work is finite algebra and
counted work on recorded or reconstructed inputs; there are no timing claims.

## Question

Run50's descent (3,415 residuals, 3,426,280,619,436 observed bitops in every
graph arm) is the dominant phase. Where does that work go? Which
literature-grounded change makes it cheaper with an exact correctness
condition, and does it lower counted work on identical inputs with every output
replayed?

## Results

### 1. Breakdown of the recorded descent

Source: `descent_breakdown.rs` → `breakdown.json`, built from Run50's
published `qualified-original.json`, `run-study.json` and
`original-ic.raw.json`. SHA-256s are in `BUILD_MANIFEST.txt` and match
`run-study.json`'s own pins. All re-priced descent subtotals equal the recorded
`diagnostic_priced_bitops` exactly. [pcc:unverified]

| Arm | Residuals | Accepted | Descent bitops | Per residual | Share of observed subtotal |
|---|---:|---:|---:|---:|---:|
| original IC | 1,728 | 2 | 1,734,424,925,904 | 1.0037e9 | 38.3% |
| quadratic table | 3,415 | 2 | 3,426,280,619,436 | 1.0033e9 | 65.5% |
| grounded pairs | 3,415 | 2 | 3,426,280,619,436 | 1.0033e9 | 54.9% |
| deferred pairs | 3,415 | 2 | 3,426,280,619,436 | 1.0033e9 | 57.9% |

The three graph arms share the descent stream and have identical descent
primitive counts.

- **By primitive** (deferred arm, bitop/v1): `field_mul` 88.32%,
  `field_add` 9.32%, `word_op` 2.01%, `hash_probe` 0.23%,
  `field_inv` 0.12%, `modr_add` under 0.001%.
- **By stage** (original IC legacy counters; the same `nagao_decompose` code,
  1,728 residuals, 1,590,966,615 `F_p` multiplications):

  | Stage | Share |
  |---|---:|
  | F4 Gröbner (6 quadrics, 6 unknowns, staircase stop) | 77.43% |
  | Linear stage (sparse FGLM, `gcd(f, x^p − x)`, read-out) | 21.87% |
  | Weil system construction | 0.30% |
  | Residual stream | 0.23% |
  | Sextic roots, ordinates and signs ("lifting") | 0.17% |
  | Decomposition verification | 0.0006% |

  There is no resultant step: the solver is F4 plus FGLM.
- **Retries and failures.** Only 2 of 1,728 and 2 of 3,415 residuals
  decompose, so 99.88–99.94% of descent work is spent on residuals that give
  nothing. Per-residual cost is nearly constant (about 1.003e9 bitops). There
  are no timeouts and no incomplete systems. Every system stopped at the
  staircase. Original IC had 3 linear-stage fallbacks. Trace replay was never
  used: `replayed = 0`.
- **Accounting gap.** The linear stage is almost entirely outside the
  `jv_bitops` hooks. Run50's own counters show this: hooked descent
  `field_mul` (1,245,470,049) equals the legacy stages without the linear stage
  (1,243,056,697) to within 0.19%, and is 345,496,566 short of the total with
  it. It was measured directly on the identical Run50 stream (section 3): the
  hooks saw 6,622,000,300 bitops inside a linear stage that counted 686,480,526
  `F_p` multiplications, which is at least 8.4e11 bitops at the `fp_mul` price
  alone. The recorded graph-arm descent subtotal therefore omits at least
  about 24.6% of descent work. This flags an incompleteness; it does not
  re-account any existing number. [pcc:unverified]

### 2. Candidates

[`CANDIDATES.md`](CANDIDATES.md) lists five candidates, each with its exact
correctness condition:

- **C1** (implemented): Joux–Vitse F4 trace replay, CT-RSA 2011.
- **C2** (most promising next, not implemented): anchored `(1^4, 2)` partial
  decompositions against the deferred graph's exact-log class domain
  (Joux–Vitse, EUROCRYPT 2012 §3; Gaudry–Thomé–Thériault–Diem, 2007). It
  meets the Run50 NEXT.md class-domain and incidence requirements explicitly.
  It reuses C1's solves and could cut residual counts, contingent on an
  unmeasured exact-log domain size.
- **C3** (rejected after reading `fast_stage`): early abort.
- **C4** (open): replaying the linear stage's normal forms.
- **C5**: the measured null result of 6→5 coordinate reduction.

### 3. Opt-in traced descent: local exact control

`traced_descent_check.rs` adds `traced_solver`, a `cover::NagaoSolver`
callback. The pipeline default is untouched; the path is opt-in. It runs three
arms on the identical reconstructed residual stream with identical
per-residual RNG seeds:

- production: staircase F4 without trace, as Run50
- traced: C1 with the acceptance and fallback rule
- oracle: F4 to a certified basis, no stop

Checks per residual:

- the three `F_p`-solution sets are identical
- every solution satisfies all six input quadrics under the file's own u128
  evaluator
- the decomposition lists are identical
- every decomposition passes `verify_dec`, which is shared library Jacobian
  arithmetic and not independent
- `R = a·G_J + b·Q_J`

The descent tally runs on one thread with `foreign_hook_calls = 0`.

| Stream | Residuals | Decomposing indices | Disagreements / bad | Replays accepted (after mismatch) / rejected |
|---|---:|---|---:|---|
| Run50 deferred, seed 389 | 3,415 | 263, 3414 | 0 / 0 | 3,414 (33) / 0 |
| nagao-e2e, seed 401 | 1,506 | 997, 1505 | 0 / 0 | 1,505 (11) / 0 |

The stream reconstruction agrees with the recorded runs. Run50 stopped after
exactly 3,415 residuals with 2 accepted descents, and nagao-e2e after 1,506.
The production arm's hooked descent subtotal on the Run50 stream is
3,416,084,890,482, 0.30% below Run50's recorded 3,426,280,619,436. Run50's
descent phase also contains the stream advances and accepted-row
verification/projection, which this window excludes; that explanation is
likely but not separately verified. [pcc:unverified]

Counted work, old (production) vs new (traced), on identical inputs
(`runs/compare-*.json`):

| Stream | F4 multiplications, old / new | Hooked descent bitops, new / old | With linear stage priced | Plus learning-RREF bound |
|---|---|---:|---:|---:|
| seed 389, 3,415 | 2,434,748,236 / 1,065,026,764 (2.29x) | 0.451 | 0.559 | 0.586 (1.71x) |
| seed 401, 1,506 | 1,072,983,726 / 466,191,224 (2.30x) | 0.448 | 0.556 | 0.617 (1.62x) |

- **With linear stage priced:** the linear stage is added at `fp_mul` per
  counted multiplication. It is identical in both arms.
- **Plus learning-RREF bound:** the uncounted tracked RREF of the single
  learning run is charged at its upper bound, 82,922,994 multiply-adds priced
  as `fp_mul` + `fp_add`. This is the most conservative column.

The oracle arm used 2,544,853,703 and 1,121,969,823 F4 multiplications. The
production staircase stop is therefore only about 4% cheaper than a
certified basis. [pcc:unverified]

What this does not show: no wall time, no complete primitive accounting (the
linear stage and other gaps remain), no end-to-end pipeline gain, no fresh or
growing-size target, no signed independent validation and no rho comparison.
The two streams are the exposed p1009 fixture. C5 already showed that a
counted descent saving can vanish end to end. M1 in
[`PROTOCOL_DRAFT.md`](PROTOCOL_DRAFT.md) is the measurement that would decide
it.

## What was done (commands)

1. Read: Run50 `NEXT.md`, `README.md` and records; `RESUME_UNIT51.md`;
   `nagao-e2e/README.md`; `rho-parity/unit_elimination.rs` (by reference);
   the overlay `jv_cover.rs` (`nagao_decompose_using`, `solve_zero_dim_mode`,
   `linear_stage_from_basis`); `f4_fp.rs` (record/replay, staircase stop);
   `f4_fp/reuse.rs`; `zero_dim_la.rs`; `jv_extension_deferred_pair_dlp.rs`
   (descent loop, seeds); the lead and amendment notes on anchored descent; the
   cover-decomposition ledger §12.
2. `rustc -O --edition=2021 descent_breakdown.rs -o descent_breakdown`, then
   `descent_breakdown research/extension_ic_graph_20261115/runpod-deferred-pair-20261117-50 > breakdown.json`.
   It is std-only and has no crates.
3. `mini-build/build.sh OUT`. It runs `fetch_frozen.sh` (`git show` of the
   frozen commit `fcda1e22`'s descent closure) and `assemble.py` (tooling:
   copies files unchanged and extracts six helper shims of byte-copied items),
   then `cargo build --offline --locked --release`. Receipts:
   `mini-build/check.log`, `release-build.log`, `BUILD_MANIFEST.txt` (rustc
   1.97.0, every source and binary SHA-256).
4. `traced_descent_check <Run50 public.json> 389 3415 rows.jsonl` and
   `traced_descent_check <nagao-e2e public.json> 401 1506 rows.jsonl`. Each
   takes about 32 s and 14 s on one thread. Summaries and gzipped per-residual
   rows are in `runs/`, hashes in `runs/RAW.sha256`.
5. `descent_breakdown compare runs/traced-*.summary.json 1009 > runs/compare-*.json`.

## Errors and deviations

- **No full crate build.** A full `cargo build --lib` filled the shared root
  volume, which went to 0 MB free; it was killed and its scratch target deleted.
  Root stayed at 0–550 MB free during the session because of other worktrees'
  checkouts. The local control therefore uses a declared hybrid build
  boundary: the frozen `fcda1e22` sources, the nagao-e2e `jv_cover` overlay,
  and byte-copied shims. Shim deviations:
  - `ic_measurement::callgrind_dump` is a no-op. It is inert outside Valgrind.
  - `prime_hyperelliptic::ops` drops the unused `divisor_hash_words`.
- **Current-main `src/cryptanalysis/f4_fp.rs` does not compile in isolation.**
  It has duplicate `recording_degree_falls` (lines 727 and 736) and duplicate
  degree-fall fields (lines 6450 and 6467) in one struct. Current main's
  `jv_cover.rs` also has no `NagaoSolver` callback. This lane edits neither;
  it is reported for a separate task.
- **First smoke run (n = 20) lost hooked work.** F4's rayon reduction ran on
  unclaimed threads, so `jv_bitops` tallies were dropped. It was fixed with a
  one-thread pool, as `nagao-e2e/native/worker.rs` does. The defective summary
  is kept as `runs/smoke-seed389-n20-multithread-DEFECT.summary.json`. Its F4
  counters and agreement checks were unaffected.
- **`traced_descent_check.rs` was first type-checked against an API stub,**
  `mini-build/api-stub-lib.rs`. This was superseded by the real mini build.
- **No research-kb.** `scripts/research-kb` has no executable, and no MCP
  tools are loaded. The receipt is in `CANDIDATES.md`.
- **Run50's LOCAL raw witnesses are absent from this host,** at
  `/workspace/ic-runpod-runs/...`. The graph-arm stage split therefore uses
  the original-IC legacy counters, which come from the same code and have the
  same per-residual cost.

## Open next steps

1. Owner decision: freeze and run M1 (`PROTOCOL_DRAFT.md`), traced descent in
   the complete deferred pipeline on a fresh target. Paid or isolated compute
   is the owner's call.
2. M2: measure the deferred graph's exact-log class domain and the class
   universe at p1009. This needs a recorded graph state. If the C2 model then
   predicts at least 1.5x fewer residuals, implement the anchored materializer
   behind the same callback and compose it with C1.
3. Hook the linear stage (`zero_dim_la::fast_stage`) into `jv_bitops`, or
   charge `lin_muls` explicitly in the pipelines. Recorded descent subtotals
   miss at least about 25%.
4. Count the learning run's tracked RREF in `f4_fp` instead of bounding it.
5. Separate task: repair the duplicate definitions in current-main
   `f4_fp.rs`, and port `NagaoSolver` to main if C1 or C2 is adopted.
