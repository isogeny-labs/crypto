# Prime-field S₄ 3-decomposition: pair-sum meet-in-the-middle

Measured 2026-11-18 on the `a = p − 3` deployed-shape rungs (P-256 class,
generic `j`), extending the prime `decomposition` / `relation_yield` ladder
from the S₃-anchored pair sweep. Preregistration: `PROTOCOL.md`.

**No subexponential claim.** The decided relation family is unchanged (three
signed factor-base points summing to `R`). What changes is how the third
coordinate is found — a hash probe into a prebuilt pair-sum table instead of
an enumeration of the `B²` pairs — and with it the measured shape of the
relation stage.

## 1. What the oracle does

For a factor base `F = {F_1..F_B}`, build once the table

```text
T[x(S)] ∋ (i, j)   for every  i ≤ j,  S = F_i + F_j .
```

For a random draw `R = aG + bQ` and each anchor `k`, probe `R − F_k` and
`R + F_k`; a hit gives a candidate triple `(i, j, k)`. The stored `y`-sign is
not needed: the probe tests both anchor signs, and `try_finalise_s4` re-adds
every candidate in the group over the eight sign patterns and only returns a
relation when the sum is exactly `R`. Two pair sums with the same abscissa
are `±S`, so an `x` collision costs one failed verify, never a wrong row.

Cost: `B(B+1)/2` group additions once; `2B` probes per draw. Against the
anchored sweep's `2B²` quadratic solves per draw, the relation stage loses a
factor `B/2` in work per draw at identical yield.

## 2. Correctness evidence

1. `examples/prime_mitm_check.rs` runs the S₃-anchored, pair-sum MITM and
   quartic/CZ oracles on the **same** three frozen 16-bit targets
   (seeds 20261005–07): all three recover the same scalar, `[x]G = Q`, and
   every relation passed the shared in-group finaliser.
   Output: `equivalence.jsonl`.
2. Every ladder row below recovered a known-answer scalar, matched the
   validation-only sidecar, and agreed with the paired rho control on the
   identical public point (`ic_ok`, `ic_agrees_rho` true in every row).
3. The relation family is not approximated: MITM and the anchored sweep both
   decide `S₄(x_R, x_i, x_j, x_k) = 0`, and the earlier census test
   (`s3_anchor_sweep_matches_the_quartic_triple_census`) already fixed that
   family exactly.

*Harness note:* the in-tree lib test `mitm_driver_agrees_with_s3_driver` is
added, but `cargo test --lib` cannot build in a fresh worktree because
several `include_str!` research fixtures are not tracked. The equivalence
example covers the same check without those dependencies.

## 3. Fixed-policy oracle comparison (`fb = 120`, seed 20261005, same host)

| bits | S₃ anchored | MITM (seed 05) | speed-up | MITM median (seeds) |
|--:|--:|--:|--:|--:|
| 16 | 0.0429 s | 0.0073 s | 5.9× | 0.0073 s (1) |
| 20 | 0.6206 s | 0.0136 s | 45.6× | 0.0136 s (1) |
| 24 | 14.637 s | 0.1609 s | 91.0× | 0.1935 s (3) |
| 28 | 276.21 s | 3.080 s | 89.7× | 3.080 s (3) |

The control column is a fresh paired run (`control_s3.jsonl`), not the
archived ladder. The relation stage alone (MITM, `fb = 120`, seed
20261005) is 3.2 ms → 9.0 ms → 152 ms → 3072 ms over 16/20/24/28 bits,
i.e. ×16 per +4 bits: at **fixed** `B` the MITM stage still scales like `p`.
The gain is the constant `B/2` (probes vs quadratic solves).

## 4. Tuned factor base (`B ≈ p^{1/3}`), stage split

`T(B) ≈ p·c_p/B + B²·c_a/2 + B³·c_m/3`; with the measured constants
(`c_p ≈ 1.3 µs`/probe, `c_a ≈ 1.1 µs`/addition, elimination cheap at these
sizes) the balance sits near `B = (2p)^{1/3}` — 256 at 24 bits, ≈ 640 at 28.

| bits | B | total | table | relations | LA | rho (same point) |
|--:|--:|--:|--:|--:|--:|--:|
| 24 | 256 | 0.089 / 0.094 s | 22 ms | 64 / 70 ms | 1.9 ms | 162 / 27 ms |
| 28 | 640 | 0.600 / 0.638 s | 159 / 161 ms | 416 / 453 ms | 19 ms | 307 / 97 ms |
| 28 (CryptoPro-B) | 640 | 0.632 s | 158 ms | 453 ms | 16 ms | 605 ms |

Two seeds per cell except the CryptoPro-B row; all verified and rho-agreeing
(`ladder_tuned.jsonl`). Against the archived S₃ medians (255.2 s p256,
362.5 s CryptoPro-B) this is ≈ 412× and ≈ 574×; against the paired p256 control
(276.2 s, same host and seed) it is ≈ 446×, and it moves the
measured relation-stage slope from ×16 to ×6.8 per +4 bits (≈ 2^(2.8/4) per
bit), consistent with the `p^{2/3}` projection of the model. With only two
tuned rungs the slope is **descriptive, not a fitted exponent**.

## 5. What this does and does not buy

- The relation stage is no longer the wall at 28 bits: table 0.16 s,
  relations 0.43 s, LA 0.02 s. `B` can now be raised without paying `B²`
  solves per draw, which is what made the anchored sweep's `B` irrelevant.
- It does **not** give a decomposition whose per-target cost is sublinear in
  `B`. MITM still spends `2B` probes per draw; the total stays super-ρ
  (`p^{2/3}` against rho's `p^{1/2}`). Subexponential prime-field IC still
  needs the algebraic route (`research/prime_fall_degree_20261027`) or a
  factor base with an algebraic membership condition.
- At 28 bits, one-target IC wall (0.60 s) is now within ~1–6× of the paired
  rho wall (0.10–0.61 s), down from the recorded ≈470×. This is **not** a
  `vs_rho` win and is not promoted as one; rho runs here are single
  unoptimised walks.
- Table memory is `B²/2` abscissae: ≈ 0.2 M at `B = 640`, ≈ 1.3 M at `B =
  1600`. Fine at these sizes; a 32-bit rung needs `B ≈ 1600` and an order
  counter that is not the in-tree `O(p)` `count_points_small`.

## 6. Next

1. **32-bit rung**: the blocker is fixture generation, not the oracle — the
   in-tree counter enumerates all `p` abscissae. A Schoof/SEA counter (or a
   family with a known order) unblocks 32 bits, where `B ≈ 1600` and the
   projection is ≈ 2 s.
2. **u64 inner loop** for `p < 2^63`: keys, probes and table stores are
   `BigUint` today; a `u64` path should cut `c_p` several-fold.
3. **Paired 2-decomposition run** at 24/28 bits under its own tuned `B`
   before any cross-family wall claim.

## 7. Reproduce

```bash
cd <repo>
cargo build --release --example a3_s4_ladder --example prime_mitm_check
target/release/examples/prime_mitm_check
target/release/examples/a3_s4_ladder 28 20261005 p256 mitm 640
target/release/examples/a3_s4_ladder 28 20261005 p256 s3 120
```

Data: `ladder.jsonl` (fixed `fb = 120`), `ladder_tuned.jsonl`,
`control_s3.jsonl` (paired S₃), `equivalence.jsonl`, `summarise.py`.

Claim boundary: public synthetic known-answer fixtures only; no deployed
curve, no key recovery, no `vs_rho` claim, no asymptotic or subexponential
claim; wall times are exploratory on a shared host.
