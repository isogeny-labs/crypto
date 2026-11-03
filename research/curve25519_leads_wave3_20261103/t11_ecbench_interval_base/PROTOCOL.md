# T11: an interval factor base in ecbench, and selected `bits` frames priced end to end (protocol, 2026-11-03)

**Class:** an engineering change (a new factor-base plug-in), with an
end-to-end ecbench comparison in counted units. It is exploratory on the
algebraic track: a constant-factor comparison of frames on one curve. It is
not a leaderboard row.

Lead T11 of `research/curve25519_torsion_leads_20261103/LEADS.md`. This file
is frozen before any code is written and before any data is taken.
Parents:
- `research/curve25519_frame_search_20261103` (the frame census and P5);
- `research/curve25519_isogeny_seed_20261103` (T5);
- `research/curve25519_leads_wave2_20261103/t6_edwards_outlier` (per-target
  F4 cost by (form, t, G)).

## Question

1. Can ecbench's `ic.pipeline` take a factor base that is **every point of E
   with x in [lo, lo + 2^t)**? It should be charged like the other abscissa
   bases and accepted by `pkm-semaev`'s bits membership.
2. With that base, does the frame chosen without logs (argmax J) lower the
   end-to-end S of the `pkm-semaev` bits arm against `lo = 0`, at the size
   the coverage-per-column model predicts?
3. Where do the selected bits arms stand against the incumbent algebraic
   arms (PKM isogeny tower, seed 0 and seed 1093), the generic baseline
   `proj-a` and `rho-neg`?

## Background: what is missing today (read from the sources, 2026-11-03)

- `PkmSemaevOracle::from_spec` (`src/cryptanalysis/ic_framework/pkm_semaev.rs`)
  accepts `bits=T,lo=L` on **any** base other than prime-kummer and
  prime-isogeny. `prepare` checks only that every abscissa of the base lies
  in [lo, lo + 2^T).
- `pkm_semaev::bits_factor_base(inst, t, lo)` builds the full interval base,
  but only the `curve25519_family` binary calls it. It records no build cost.
- None of the ecbench prime factor bases equals that interval for a selected
  lo:
  - `prime-abscissa:below=2^T` is [1, 2^T), so lo = 0 only.
  - `explicit-x:xs=…` takes a hand-written list. It charges `sqrt_solves`
    only for the listed x, so the Legendre scan that picked them happens
    outside the harness and is not charged. It also fails on any x that
    carries no point.
  - The dispatch is `prime_base_for` and the factor-base dump match in
    `src/cryptanalysis/ecbench/methods.rs`.
- So today a selected frame (`bits@976503`) cannot be priced with the whole
  interval scan charged. T11 adds that base.

## Code stage (done before `scripts/run.sh`; edits limited to the files listed)

### Factor base `prime-interval:t=T[,lo=L]`

- **Meaning:** one column per abscissa x ∈ {(L + k) mod p : 0 ≤ k < 2^T}
  that carries a point, with both signs. Columns are in ascending x. This is
  exactly the point set of `pkm_semaev::bits_factor_base(inst, T, L)`.
- **Parameters:**
  - `t`: required, an integer ≥ 1 with 2^t < p;
  - `lo`: default 0, an integer with 0 ≤ lo < p.
  - Any other key is an error. So is an interval in which no abscissa
    carries a point.
- **Charging:** the same counters as `prime_factor_base`
  (`src/cryptanalysis/ic_boundary.rs`):
  - `abscissae_scanned = 2^t`;
  - one Legendre symbol per scanned x;
  - `sqrt_solves` = the number of x with Legendre ≥ 0;
  - the index build through `count_index_build`, as `prime_factor_base`
    does.
  - So the whole scan is charged inside the harness. Nothing is moved into
    "uncharged".
- **Where it is implemented:** a `PrimeIntervalBase<'i> { instance }` with
  `impl FactorBaseBuilder<PrimeCurve>`. It goes next to `bits_factor_base` in
  `src/cryptanalysis/ic_framework/pkm_semaev.rs`, and the point list is
  shared with `bits_factor_base`, so the two cannot drift apart. It is
  registered in:
  1. `prime_base_for` in `src/cryptanalysis/ecbench/methods.rs`;
  2. the prime factor-base dump match in the same file;
  3. the `ic.pipeline` `factor_base` help string in the same file.
- **Oracle coupling** (`PkmSemaevOracle::from_spec`):
  - Add an arm `"prime-interval" => Self::new_bits(m, fb.t, fb.lo)`, so the
    oracle reads the interval from the base.
  - If the oracle also gives `bits=` or `lo=` and they differ from the
    base's, this is an error. If they are equal, it is accepted.
  - The membership check in `prepare` stays unchanged.
- **No other behaviour changes.** Existing bases, oracles, spec ids and
  counters are untouched. C2 checks the spec id.

### Unit tests

All test names contain `prime_interval`. The k = 20 miniature is p = 1048573,
a = 63700, b = 872826, #E = 1049624 = 8 · 131203, G = (349837, 122).

- **U1:** for t = 5 and lo ∈ {0, 976503, 541434}:
  - the columns are 11, 24 and 16;
  - `abscissa_list` equals the lists in `selection/frames.json`;
  - points, `col_of` and `coef_of` equal `bits_factor_base(inst, 5, lo)`.
- **U2:** `prime-interval:t=5,lo=0` has the same point set as
  `prime-abscissa:below=32`. x = 0 carries no point on this curve.
- **U3:** `prime-interval:t=5,lo=976503` has the same points and columns as
  `explicit-x` with the 24 abscissae of `selection/frames.json`.
- **U4:** on that base, `abscissae_scanned = 32` and `sqrt_solves = 24`.
- **U5:** these are rejected:
  - a missing `t`, `t = 0`, `2^t ≥ p`, `lo ≥ p`, and an unknown key;
  - an interval in which no abscissa carries a point (on a small fixture);
  - `pkm-semaev:…,bits=4` on `prime-interval:t=5`, and `lo=1` on `lo=0`.
  - The same parameters given twice and equal are accepted.
- **U6:** an end-to-end pipeline solve on the small prime fixture the
  existing `pkm-semaev` ecbench tests use. It runs `prime-interval:t=T,lo=L`
  with `pkm-semaev:m=2,mode=full,guess=1`, and the returned log must equal
  the known log.

### Build and static checks (exact commands, run from `/home/user/crypto`)

The logs go to `t11_ecbench_interval_base/build/`. Each ends with a line
`RC=<exit code>`.

```bash
export CARGO_TARGET_DIR=/home/user/ctarget-t11          # never /home/user/ctarget
B=research/curve25519_leads_wave3_20261103/t11_ecbench_interval_base/build; mkdir -p $B
cargo build --release --bin ecbench > $B/build.log 2>&1; echo "RC=$?" >> $B/build.log
rustfmt --edition 2021 --check <changed .rs files> > $B/rustfmt.log 2>&1; echo "RC=$?" >> $B/rustfmt.log
cargo clippy --release -- -D warnings > $B/clippy.log 2>&1; echo "RC=$?" >> $B/clippy.log
cargo test --release --lib prime_interval > $B/test.log 2>&1; echo "RC=$?" >> $B/test.log
sha256sum $CARGO_TARGET_DIR/release/ecbench > $B/ecbench.sha256
sha256sum <changed .rs files> > $B/source.sha256
```

- Only one heavy process (a cargo build, clippy or test) runs at a time.
- If crate-wide clippy fails only on lints in files T11 did not change,
  this is recorded in `build/clippy_changed.log`: the lints in the changed
  files, then a line `CHANGED_FILE_LINTS=<n>`. The gate passes only if n = 0.
  This is reported as a deviation.
- No git. The source identity is `build/source.sha256`, and the
  binary identity is `build/ecbench.sha256`. `run.sh` copies both into
  `data/`.

## Frame and seed choices (computed now from existing data)

The choices come from `scripts/select_frames.py`, which reads
`curve25519_frame_search_20261103/confirm/k20.json` (2,000 translation
frames at t = 5). It writes `selection/frames.json` (sha256
`b2852740…118b7f`).

| arm | lo | rule | cols | J = covered | covered / col | model M ÷ lo0 |
|:--|--:|:--|--:|--:|--:|--:|
| `pkm-int-t5-g3-lo0` | 0 | the incumbent bits frame | 11 | 42 | 3.818 | 1 |
| `pkm-int-t5-g3-L976503` | 976503 | argmax J; tie-break: more cols, then lower L (parent E6) | 24 | 198 | 8.250 | **0.4628** |
| `pkm-int-t5-g3-L541434` | 541434 | argmax J ÷ cols; tie-break: higher J, then lower L (the T9 rule) | 16 | 138 | 8.625 | 0.4427 |

- The argmax-J frame is also the coverage-best of the 2,000 frames. The
  J ÷ cols frame is the best by covered ÷ col.
- The model is M = cols × r ÷ covered, the expected number of trials per
  solve up to a common factor.
  - The per-target F4 cost is frame-independent at t = 5, G = 3:
    3,036,911.76 (lo = 0) and 3,036,916.11 (L = 976503) mul-adds per target
    (parent engine files).
  - So M is also the model for F4 work and for S at w ≥ 0.1.
  - M(J ÷ cols) ÷ M(J) = 0.9565.
- The script checked each column count against the ecbench Weierstrass model
  of the curve: 11, 24 and 16 abscissae of [lo, lo + 32) carry a point.
- **Why t = 5 and G = 3:** this is the lowest-cost measured bits cell.
  - The per-target cost at k = 20 falls with G: 1.27·10⁸ (G = 0),
    3.23·10⁷ (G = 1), 1.28·10⁷ (G = 2) and 3.04·10⁶ (G = 3) (wave-2 T6
    Block C). G = 4 was never measured.
  - At t = 6 the per-target cost is 1.7·10⁸ even at G = 3. With
    columns × r ÷ coverage that is about 2·10¹² mul-adds a solve, which is
    not feasible here.
- **Incumbent arms:** the same as T5, `prime-isogeny:t=6` (seed 0) and
  `prime-isogeny:t=6,seed=1093` (argmax J), both with
  `pkm-semaev:m=2,mode=full,guess=4`. Seed 1308 (J ÷ cols) belongs to T9 and
  is not repeated here.
- **Model for the interval arms against the isogeny arms.**
  - The F4 work a solve is E = cols × muladds_per_relation_at_exact_coverage.
  - Interval arms: E = 1.044·10¹¹ (lo0), 4.83·10¹⁰ (L976503) and
    4.62·10¹⁰ (L541434).
  - Isogeny arms (parent R4): E = 1.354·10¹⁰ (seed 0) and 8.05·10⁹
    (seed 1093).
  - So E(L976503) ÷ E(s1093) = 6.0 and E(L976503) ÷ E(s0) = 3.57.
  - In T5 the measured per-solve F4 work was 0.45 × E (seed 0) and 0.48 × E
    (seed 1093).
- **Model at w = 0.** S counts mainly the trial group operations, which are
  proportional to M. The tower build adds a fixed cost to the isogeny arms;
  the 32-symbol interval scan is negligible.
  - Calibrated on T5, where seed 1093 had S(w = 0) = 26.4 with about
    5.4·10³ trials a solve, the predicted S(w = 0) is about 44 (lo0), 21
    (L976503) and 20 (L541434).
  - This w = 0 model is rough. It is used only for P5 and P9.
- **Feasibility (wall time is diagnostic only).** In T5, a pkm trial took
  about 20 ms of wall time. The estimate is about 15 minutes per (target,
  round) over the seven arms, so 12 (target, round) pairs take about 3 hours.
  Every execution has a 3,600 s timeout.

## Measurement design (`scripts/run.sh`; starts after the code stage)

- **Instrument:** `/home/user/ctarget-t11/release/ecbench`, built in the code
  stage.
  - `run.sh` aborts in any of these cases:
    - the binary is missing;
    - the binary is the shared `/home/user/ctarget` binary (sha256
      `54737834…29641b`);
    - its sha256 differs from `build/ecbench.sha256`;
    - any gate log in `build/` lacks `RC=0`;
    - the spec sha256 differs from the frozen value.
- **Spec:** `ecbench/spec.json`, sha256
  `630f7722270399ce05ffcd585ef12ecd48f98e7f7401ace4c88d235b6becdbc2`.
  - Today's shared binary plans it as spec id **`ECS1hd4f9ace982f5`**, with
    84 executions and no ordering warning. `ecbench plan` does not check the
    factor-base name.
  - The curve is the k = 20 miniature, the same `workloads.curves` entry as
    T5.
  - There are 6 public targets, with fresh target_seed **3110311101** (used
    in no other curve25519 spec).
  - Measurement seed 3110311102, order `alternate_measured`, 2 measured
    rounds, 0 warm-up, timeout 3600 s, isolation L0.
- **Arms** (7 arms × 6 targets × 2 rounds = 84 executions):

  | arm | role | factor_base | oracle |
  |:--|:--|:--|:--|
  | `rho-neg` | reference | — (`rho.negation`) | — |
  | `proj-a` | baseline | `prime-projected-abscissa:size=32,scan_cap=512` | `mitm:negation_folded=1` |
  | `pkm-iso-t6-g4` | candidate (incumbent) | `prime-isogeny:t=6` | `pkm-semaev:m=2,mode=full,guess=4` |
  | `pkm-iso-t6-g4-s1093` | candidate (incumbent) | `prime-isogeny:t=6,seed=1093` | same |
  | `pkm-int-t5-g3-lo0` | candidate | `prime-interval:t=5,lo=0` | `pkm-semaev:m=2,mode=full,guess=3` |
  | `pkm-int-t5-g3-L976503` | candidate | `prime-interval:t=5,lo=976503` | same |
  | `pkm-int-t5-g3-L541434` | candidate (secondary) | `prime-interval:t=5,lo=541434` | same |

- **Commands** (all in `scripts/run.sh`, in this order, one heavy process at
  a time):
  1. `ecbench plan --spec ecbench/spec.json`. The plan must report spec id
     `ECS1hd4f9ace982f5` with 84 executions and no warning, or the script
     aborts before any data.
  2. `ecbench run --wait --spec ecbench/spec.json --out data/ecbench/s1`.
  3. `ecbench band --dir data/ecbench/s1 --ranks --json data/ecbench/band.json`.
  4. `ecbench compare --dir … --a A --b B --weight w` for w ∈ {0, 0.1, 1},
     saved as text and as `--json`. The pairs (B ÷ A) are:
     - L976503 ÷ lo0;
     - L541434 ÷ lo0;
     - L541434 ÷ L976503;
     - L976503 ÷ iso-s1093;
     - L976503 ÷ iso-s0;
     - iso-s1093 ÷ iso-s0;
     - L976503 ÷ proj-a;
     - L976503 ÷ rho-neg.

     The bootstrap uses compare's default seed and resamples.
  5. `ecbench verify --dir … --replay 12 --out data/ecbench/verify.json`.
     The auditor seed comes from OS entropy and is recorded in the receipt.
  6. `data/SHA256SUMS`.
- **Ratios:** each prediction uses the point ratio and the 95% interval
  that `ecbench compare` prints at that weight, with no post-processing.
  Band orders are as `ecbench band --ranks` prints them.
- **The frame search is reported separately.** The K = 2,000 search at t = 5
  cost 64,000 Legendre symbols and 814,514 group operations
  (`curve25519_frame_search_20261103/analysis/summary.json`).
  - Charged to one target, it adds 814,514 ÷ √r = 2,249 to S at every weight,
    plus 0.1 × 64,000 ÷ √r = 17.7 at w = 0.1 and 177 at w = 1 for the
    Legendre symbols.
  - The selected arms are reported both amortised (S as measured) and
    charged. The lo0 arm needs no search.

## Predictions (numbered; each decided by the rule stated; no retuning)

- **C1 (code gate).** These pass:
  - the build;
  - `rustfmt --check` on the changed files;
  - clippy `-D warnings` (no lint in the changed files);
  - `cargo test --release --lib prime_interval`, with U1–U6 all passing.
  - **Holds** iff all pass. If C1 fails, no data is taken.
- **C2 (no spec drift).** The t11 binary plans the frozen spec to spec id
  `ECS1hd4f9ace982f5`, with 84 executions and no warning. This is the id the
  shared binary gives now. **Holds** iff exact.
- **P1 (exact base).** U1–U4 hold:
  - 11, 24 and 16 columns, with the frozen abscissa lists;
  - the same points as `bits_factor_base`, as `prime-abscissa:below=32`
    (lo = 0) and as the `explicit-x` list (lo = 976503);
  - `abscissae_scanned = 32`.
- **P2 (correctness).** Every execution of every arm verifies: 12 of 12 per
  arm, 84 of 84 in all. `ecbench verify` reports 0 problems. **Holds** iff
  both.
- **P3 (the frame gain, end to end).** At w = 0.1 **and** w = 1, the point
  ratio S(L976503) ÷ S(lo0) ≤ 0.65.
  - Model 0.463. With T5's measured-to-model inflation of 1.07 it is about
    0.50.
- **P4 (the frame gain, interval).** At w = 0.1 **and** w = 1, the upper end
  of the 95% interval of S(L976503) ÷ S(lo0) is < 1.
- **P5 (the frame gain at w = 0).** The point ratio S(L976503) ÷ S(lo0) at
  w = 0 is ≤ 0.75. The interval base's build cost is negligible, so w = 0
  should follow the model (0.463).
- **P6 (J ÷ cols against J, on frames).** At w = 0.1, the point ratio
  S(L541434) ÷ S(L976503) ∈ [0.70, 1.30].
  - Model 0.957. No material difference is predicted at this sample size.
  - Whether the interval excludes 1 is reported but not predicted.
- **P7 (the J ÷ cols frame against lo = 0).** At w = 0.1 **and** w = 1, the
  point ratio S(L541434) ÷ S(lo0) ≤ 0.65. Model 0.443.
- **P8 (the incumbent stays ahead on the native-priced track).** At w = 0.1
  **and** w = 1, the point ratio S(L976503) ÷ S(iso-s1093) ≥ 2.0 (model
  about 6.0). `ecbench band` must also order `pkm-iso-t6-g4-s1093` below
  `pkm-int-t5-g3-L976503` at both weights. **Holds** iff both.
- **P9 (w = 0, low confidence).** The point ratio S(L976503) ÷ S(iso-s1093)
  at w = 0 is < 1.0.
  - Model about 0.8: at w = 0 the trial count dominates S, and the isogeny
    arm carries a tower-build cost.
  - No interval claim.
- **P10 (replicating T5).** At w = 0.1, the point ratio S(iso-s1093) ÷
  S(iso-s0) ≤ 0.75. T5 measured 0.641 on 6 targets.
- **P11 (scope).** In `ecbench band --ranks`:
  - all five PKM arms rank behind `proj-a` and behind `rho-neg` at w = 0.1
    and w = 1;
  - the three interval arms also rank behind `proj-a` at w = 0.
  - **Holds** iff all hold.
- **P12 (charging is visible).**
  - The session's per-arm counters (in the `ecbench compare` arm lines or
    `band.json`) report for each interval arm:
    - `abscissae_scanned` totalling 12 × 32 = 384;
    - for the factor base, `sqrt_solves` of 12 × cols.
  - If the session output shows no such counter, P12 is recorded as **not
    evaluable**. That is reported as a reporting gap of the implementation
    and is not called held.

## Order of analysis and failure handling

- **Order:**
  1. the code stage (C1);
  2. `run.sh`: identity checks, then plan (C2), then run, band, compares and
     verify;
  3. the analysis, which reads `data/` only.
- **Precondition.** The main session fills and pushes `.resume/RESUME.md`
  (`scripts/checkpoint.sh init`) before it launches `run.sh`. Neither stage
  runs git.
- **If the implementation proves infeasible:** the reason is recorded in
  `build/INFEASIBLE.md`, the lead stops, and no data is taken. Infeasible
  means, for example, that the pipeline cannot accept a base whose columns
  are not fixed by the oracle, or that the change would alter existing spec
  ids or counters.
- **If C1 fails:** the failure is recorded and nothing is measured.
  - The code may be fixed and the gate re-run before any data. Each attempt
    is kept in `build/` and reported.
  - The spec, arms, seeds and predictions are not changed.
- **If C2 fails:** the spec id moved, so the change altered the spec
  hashing or the planning. Nothing is measured. This is treated as a C1
  failure.
- **If an execution fails, times out or is censored:** it is reported as
  censored, and P2 fails. P3–P11 are decided on whatever `ecbench compare`
  pairs, with the number of pairs stated.
- **If the session is interrupted** (host or process failure): one fresh
  session with the identical spec may be run into `data/ecbench/s2`, and
  this is reported. The interrupted session is kept and not mixed in.
- **If `verify` reports a problem:** the results are reported as unaudited,
  and no prediction is called held.
- **Failed predictions are reported as failed.** Thresholds, frames, seeds,
  the target set, arms, weights and the selection rules are not changed
  after data.
- **Units:** counted units only (GAE, S, F4 multiply-adds). Wall time is
  diagnostic only (isolation L0).

## Scope and distance remaining

- **What is new.** T11 lets ecbench price an interval base. The new base
  changes no existing arm. The measured comparison is a frame choice for
  `pkm-semaev` on bits: a constant factor, not an exponent change.
- **What is expected.** Per the model, the selected bits frame does not beat
  the incumbent algebraic arm on the native-priced track: E(L976503) ÷
  E(s1093) ≈ 6.0 (P8).
  - In T5, `pkm-iso-t6-g4-s1093` was 6.9·10⁵× behind `rho-neg` and 8.3·10⁴×
    behind `proj-a` at w = 0.1, and 4.4·10⁶× behind `rho-neg` at w = 1.
  - If the model holds, the selected bits arm will be about 4·10⁶× behind
    `rho-neg` at w = 0.1.
  - This session's own band replaces these projections.
- **What is reported.** S(L976503) ÷ S(rho-neg) and S(L976503) ÷ S(proj-a)
  at w = 0, 0.1 and 1, from this session's compares and band, as the
  distance remaining. **Rho remains ahead.** No row here is a leaderboard or
  `benchmarks/` claim, and there is no validation-gate submission.
- **Limits.** One curve (k = 20), one interval width (t = 5), one guess
  size (G = 3). The frame census shows that the gain over the mean frame
  shrinks with t (1.3–1.5× at t = 8). Nothing is extrapolated to 255 bits.
- **Environment.** Shared 4-core host; other wave-3 leads may run at the
  same time. The counts are deterministic given the seeds, so load affects
  only wall time.
