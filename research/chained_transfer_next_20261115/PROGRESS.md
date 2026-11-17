# O4 protocol: progress (2026-11-15)

Status: protocol amended; code written and unit-tested (2026-11-15, see
"Code status" below); no measured run and no benchmark record yet. Every number here is
`[pcc:unverified]`. This file is not covered by `frozen.sha256`; the frozen
text is `PROTOCOL.md` (original sections plus "Amendment 1") and `NOTE.md`.

## Freeze history

| when | `frozen.sha256` covers | hash |
|---|---|---|
| first freeze (commit `f619a7a76b`) | `PROTOCOL.md` | `fa7edef8adbcd383d06285b37651556456f20424754c887a1b0a4e4fb6fb898b` |
| Amendment 1 (2026-11-15, before any code or data) | `PROTOCOL.md` | `e48aa56c70d807b5766ff044ed9f473256afbd4e61d00d15b06ea1afe8022650` |
| Amendment 1 | `NOTE.md` (newly covered, R16; unchanged) | `e97a070eff8f1f23523384dacfe4bd9d57b81ff6a79b52d49274329bc3252af6` |

Check: `cd research/chained_transfer_next_20261115 && sha256sum -c frozen.sha256`.

Amendment 1 implements `REDTEAM.md` R1 to R17; none is rejected. Pre-freeze
work: a selection-only Python script in the session scratchpad
(`select_only.py`, sha256
`371d33568723d5c4bfd142656e3f08a8a3d63edebda3edb14a92a721a9198d8b`; output
sha256 `f8ed37874a617d0c6caeef7bf5877a53556b6fbfb26179986914e09d95c7c1cb`).
The scratchpad may be cleared; the rule it applies is Amendment 1 items A2,
A3 and A4, and its table is A4.

## Code checklist (from Amendment 1)

Order of work: module and tests, then the binary, then the pre-run checks.
Item ids refer to Amendment 1.

### `src/cryptanalysis/chained_transfer/quartic_ic.rs` (new, registered in `chained_transfer/mod.rs`)

- [x] `QuarticCover::new(p, a, b)` for `p ≤ 2^20`, every product reduced mod
      `p` before the next multiplication or done in `u128` (A8); keep the
      `rational_points`, `secant`, `line_relation` semantics of
      `bielliptic_quartic.rs` (copied, that file not edited).
- [x] Counters per run: `MulP`, `AddP`, `InvP` (one per `pow(·, p − 2)`),
      `MulR`, `AddR`, `InvR`, `Canon`, `Lookup`, `Insert`, `Draw` (A1, A13);
      square-root and fourth-power tables charged at one MulP per entry, or
      counted Tonelli–Shanks (A6).
- [x] Cold runs: each Q or D run built from `(p, a, b, r, G, T)` only, `r` an
      input, no `group_order` inside an arm, no state shared between runs (A6).
- [x] Selection: `candidate_ab(p, i, u)` with
      `a_u = mix64(0xA4B1 + 101 i + u) mod p`, `b_u = mix64(0xB7C3 + 101 i + u) mod p`,
      `i` the position in stage 1 then stage 2 (0..9), `u = 0..3999`; checks:
      nonsingular, `a != 0`, `b != 0`, `r` prime, `r != p`, `r ∤ p^k − 1` for
      `k = 1..6`, `r ≥ ceil(0.85 p)`, at least 8 points on `C`; assert
      `p % 4 == 1`; hop to the next prime `≡ 1 (mod 4)`, at most 20; report
      `n` and `n/p` (A2, A3).
- [x] The A4 table as a constant; `--select-only` compared row for row
      (`p, u, a, b, r, n`), mismatch stops before any arm (A4).
- [x] Q collection: uniform draws of two distinct affine rational points with
      the collection RNG, duplicate lines skipped and counted; residual
      quadratic must split; folded `±1` rows (A10).
- [x] D collection: three uniform columns with uniform signs; keep the row if
      `−(P1 + P2 + P3)` is a column up to sign; additions tallied by
      `full_jacobian_ic::ec_add_tally` (A1, A10).
- [x] Stopping rule: every column weight ≥ 1 and rows ≥ `n + 20`, then solve;
      on `None` or a failed residual, `ceil(0.1 n)` more rows and retry; every
      attempt counted; rows used and attempts recorded; budget `64 · n` pair
      trials (Q) or triples (D) over all attempts (A7).
- [x] Linear algebra: a counted copy of `ic_control::peel` with no target
      column, then the counted Wiedemann, then counted back-substitution of
      every peeled column; residual `A x = c` over every collected row;
      `InvR` recorded separately instead of `pivot_inversion_steps(r)`
      multiplications (A5, A9, A13). `ic_control.rs` not edited.
- [x] Dense Gauss–Jordan at `p ≤ 421`, uncounted, as a cross-check; a
      disagreement stops the run (A5).
- [x] Anchor and target shifts `T + [h]G`, `h ≤ 512`, tallied by
      `ec_add_tally` (A1).
- [x] Verification outside the total, each count reported: `verify_relation`
      on every row (failure count reported), factor-log replay, fibre witness,
      final `[t]G == T` (A6, A9).
- [x] `lower_bound: true` and `lower_bound_note` on Q and D rows (A6).
- [x] RNGs: collection `StdRng::seed_from_u64(IC seed)`, linear algebra
      `StdRng::seed_from_u64(IC seed ^ 0x4C41)` (A10).
- [x] Unit tests:
  - [x] `reproduces_p53_demo` in `--exhaustive` order: 45 points, 79
        sections, 11 columns, scalar 17, and the 59-scalar replay with two
        generators (A9 i).
  - [x] `tamper_fails` (row index `mix64(0x7A3B ^ p) mod rows`) and a
        tampered certificate rejected by `verify_relation` (A9 iii).
  - [x] `selection_binds_r_to_p`, `selection_rejects_j0_j1728_anomalous_low_embedding`,
        `selection_reproduces_a4_table` (A2, A3, A4).
  - [x] `folding_is_sign_consistent`.
  - [x] `section_matches_u128_reference` on `10^4` random lines at
        `p = 102409` (A8).
  - [x] `peel_without_target_column` and back-substitution (A5).
  - [x] `la_copy_matches_ic_control_counts`: `MulR + pivot_inversion_steps(r) · InvR`
        equals `ic_control`'s `Nat.mul` on the same input (A13).
  - [x] `seeds_match_formulas` (A10).
  - [x] `slope_rule_two_phase_toy` (dominant low-exponent phase plus a
        1%-share steep phase, and a fixed-cost case) (A16).

### `src/bin/quartic_bielliptic_ic.rs` (new, following `src/bin/full_jacobian_ic.rs`)

- [x] Flags: `--threads`, `--min-r-frac`, `--max-prime-hops`, `--stage1`,
      `--stage2`, `--budget-s`, `--inv-weight` (default 10), `--seed0`
      (default `0x5EED0415`), `--out-dir`, `--protocol`, `--select-only`,
      `--exhaustive` (A19).
- [x] Seeds: `seed_k = seed0 ^ (k · 0x1000)`, `G_k = E.point_from_seed(seed_k)`,
      `x_k = (seed_k as u128 · 2246822519 + 7) mod r` (0 becomes 1, printed);
      IC seed `base + j + 10 k` (`Q` 700, `D` 500); rho seed
      `1000 + j + 100 k`, 16 per instance (stage 2: 8), same for negation and
      plain; every seed printed (A10).
- [x] Rho: `full_jacobian_ic::rho_full_ec_tallied` (negation primary, plain
      context), base `muls + sqrs + w_inv · invs`, set-up reported beside the
      total (A1, A6).
- [x] Pricing: field-unit base per arm; native band 0, 0.1, 1 and calibrated
      0, 1, 10 `w_cal` with `w_cal = time(class) / time(one F_p multiplication)`;
      classes by `negation_remeasure::classify` on both full vectors with the
      arm's `lower_bound` flag (A1).
- [x] `t_mul`, `t_add` for `ec.rs` and for a plain `u64` affine addition
      (`ic_boundary::invmod`); each cell re-classed with the rho base
      `total_ops · t_add / t_mul` for both; timing with no arm or other thread
      running; host and isolation recorded (A15, A18).
- [x] Aggregation: `R(p)` and `R_cal(p)` as ratios of means (Q over
      `3 × 4` runs, rho over `3 × 16`); OLS of `ln R`, `ln R_cal` on `ln p`
      over the 7 stage-1 points with `t95(5)`; stage 2 separate (A10).
- [x] `verdict` ∈ `Q_above_rho`, `unresolved`, `unresolved (lower bound)`,
      `Q_below_rho`, with `counted_below_rho` per cell; `verdict_gae` by
      `classify` on the GAE vectors, labelled secondary (A1, A11).
- [x] Sensitivities: `w_inv` 0 and 30 (base only, native at 0); verification
      counts; Q's own exponent, rho walk-only and total exponents, phase-share
      table (A1, A6, A16).
- [x] 32-column BSGS check at `k = 0` of every prime, columns from
      `StdRng::seed_from_u64(0xB565 ^ p)`; a mismatch stops the run (A9 ii).
- [x] Determinism check: second Q pass on `k = 0` of 229, 421, 821 with the
      same seeds; counts identical (A9).
- [x] Output `quartic_ic.json`, table and SVG; RESULTS.md quotes `verdict`
      exactly and `verdict_gae` labelled.

### Before the run (not authorised by the protocol)

- [x] Unit tests pass; `--exhaustive` at `p = 53` reproduces; `--select-only`
      matches A4 (2026-11-15, outputs in the session scratchpad, not here).
- [ ] `scripts/checkpoint.sh init` (`.resume/RESUME.md`), a `bench.record/v1`
      record, and the experiment director's task card for the route.

## Code status (2026-11-15)

Written: `src/cryptanalysis/chained_transfer/quartic_ic.rs` (registered in
`chained_transfer/mod.rs`) and `src/bin/quartic_bielliptic_ic.rs` (found by
Cargo's bin discovery; no `[[bin]]` entry needed). `bielliptic_quartic.rs`,
`ic_control.rs`, `negation_remeasure.rs` and `full_jacobian_ic.rs` are not
edited. Everything below is a check of the instrument, not a measurement, and
is `[pcc:unverified]`.

- `cargo test --release --lib chained_transfer::quartic_ic`: 16 passed, 0
  failed. Names: `reproduces_p53_demo`, `sections_match_old_module_at_p53`,
  `tamper_fails`, `tamper_detected_at_every_stage1_prime_up_to_3209`,
  `selection_binds_r_to_p`,
  `selection_rejects_j0_j1728_anomalous_low_embedding`,
  `selection_reproduces_a4_table`, `folding_is_sign_consistent`,
  `section_matches_u128_reference`, `peel_without_target_column`,
  `la_copy_matches_ic_control_counts`, `seeds_match_formulas`,
  `slope_rule_two_phase_toy`, `inv_r_count_check`,
  `arms_q_and_d_certify_and_are_deterministic`,
  `budget_exhaustion_is_a_failed_run`.
- `--select-only`: `p, u, a, b, r, n` and `#C(F_p)` equal to the A4 table and
  to `select_only.tsv` at all 10 primes; no hop.
- `--exhaustive` (`p = 53, a = 2, b = 1`): 45 points, 79 certified sections,
  11 columns, dense rank 11, Wiedemann logs equal to the dense logs, target
  scalar 17 certified, 116 of 116 scalars recovered over two generators.
- A smoke at 229, 421 and 821 (1 instance, 2 IC seeds, 2 rho seeds, unit class
  times, scratchpad output) ran every control: dense cross-check, 32-column
  BSGS, tamper, determinism. Its numbers are not data and are not quoted.

### Choices where the protocol is open (the frozen text is not edited)

1. Tamper control (A9 iii): a change in a row that peeling removes is always
   consistent (the peeled column absorbs it), so neither the residual nor
   `[t]G == T` can see it unless the target uses that column. The control
   therefore also replays every factor-base log of the tampered solve; it
   counts as detected on a failed solve, a failed `[t]G == T` or any replay
   mismatch, and reports which. The tamper is applied to the system of the
   clean run's final solve and run once (fresh LA stream, no retry). The
   tampered entry is the first entry of the row; the anchor is row 0.
2. "Rows reach `n + 20`" counts relation rows; the anchor row is extra.
3. Shifts are `h = 0..511` (512 shifts, the old `MAX_TARGET_SHIFTS`).
4. A rank-deficient core can pass the Wiedemann residual with a kernel-shifted
   solution. The retry rule only reacts to a failed solve or residual, as
   frozen; peeling reports free columns as a failed attempt. A wrong log then
   shows as an uncertified run or a BSGS/replay mismatch.
5. Arm D discards a triple whose sum is the identity (`-(P1+P2+P3) = O` is not
   a column), as the text reads.
6. The calibrated weights of Q and D are timed on their own code paths
   (`measure_quartic_class_times`), rho's by `measure_class_times`; both are
   divided by Q's `F_p` multiplication time.
7. `verdict_gae` uses the band and calibrated GAE vectors only; the slope
   condition is applied to the primary verdict alone.
8. Dense cross-check: every run with a kept system at `p <= 421`; BSGS check:
   Q and D seed `j = 0` at `k = 0`; altered certificate: at 12841 and at every
   stage-2 prime run (a superset of "the largest").

### Not done

- The run itself, its `bench.record/v1` record and `.resume/RESUME.md`: blocked
  by the checkpoint gate and not authorised by the protocol.
- RESULTS.md: needs the run.
