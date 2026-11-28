# Results — binary Koblitz fall-degree pilot

All numbers `[pcc:unverified]`, local 4-vCPU container, master seed
`1262698074`, instrument `examples/binary_falldeg_arms.rs`
(`cargo build --release`), raw rows in `raw/pilot.jsonl` (72 rows),
stderr in `raw/pilot.stderr`. This is calibration/pilot data, **not** a
study row; no leaderboard, no cost claim (PREREGISTRATION §9).

Gates: **G1** `cargo test --release --example binary_falldeg_arms` → `4 passed`.
**G4/ordering** `binary_falldeg_arms selftest` → `[selftest] OK`.
**G3** (equality with `hky_par`): the HKY closure (`closure_par`) is a verbatim
copy of `examples/hky_par.rs`, so `d_F` is the same number by construction; a
seed-matched cross-run was not done because the two binaries derive draws
differently.

## Last-fall degree `d_F` (HKY mutant closure), median over 3 draws

`cmax = 4`; every cell resolved by `c = 4` (no censoring). `dF[min,max]` is the
A/A spread across the 3 draws (the noise floor). `—` = orbit arm not available
(no `τ`-invariant subspace of that dimension below `n`; see H-avail).

| m | n  | ℓ_eff | orbit d_F | random d_F | subspace d_F | A/A spread (random) |
|--:|---:|------:|:---------:|:----------:|:------------:|:-------------------:|
| 2 | 9  | 3     | 2         | 2          | 2            | 0 |
| 2 | 15 | 5     | 2         | 2          | 2            | 0 |
| 2 | 15 | 6     | **2**     | **3**      | 2            | 0 |
| 2 | 21 | 8     | 3 ([2,3]) | 3          | 2 ([2,3])    | 0 |
| 2 | 23 | 11    | —         | 4          | 3            | 0 |
| 2 | 13 | 5     | —         | 3          | 2            | 0 |
| 2 | 19 | 7     | —         | 3          | 2            | 0 |
| 3 | 9  | 3     | 4 ([3,4]) | 4          | 4            | 0 |
| 3 | 15 | 5     | 4         | 4          | 4            | 0 |

## Readings (against the ±1 A/A floor)

- **`d_F` is small and flat: 2–4** across `n = 9…23`, `m ∈ {2,3}`, matching the
  prior binary program's 3–4 (`research/fall_degree_bounds_20261008/`). "Last
  fall degree" is **well-defined and computable** at these sizes — every cell
  reached `1 ∈ V_{F,4}` exactly, no censoring.
- **H-orbit (the central claim): no Frobenius-specific effect in range.** The
  orbit median equals the random median in every cell **except** `m=2, n=15,
  ℓ=6`, where orbit `= 2 < 3 =` random. But at `m=2, n=21, ℓ=8` the orbit
  **equals** random (3) while the *structured window* is lower (2); and at
  `n=15, ℓ=6` the window also reads 2. So the orbit base does **not**
  consistently differ from random, and where a structured base is lower the
  **non-Frobenius** window is lower too. Every gap is `≤ 1`, i.e. at the A/A
  floor. **Reading: no detectable Frobenius-orbit-specific change in `d_F` vs
  random in the measured range.**
- **H-struct / H-null-matches: a weak "structured < random" effect of 1
  degree.** In every `m=2` cell the polynomial-window (`subspace`) median
  `d_F` is `≤` random, and strictly `1` lower in 5 of 7 (`n=15 ℓ6, n=21, n=13,
  n=19, n=23`). This is the clearest signal — but it is a *linear/low-degree
  subspace* effect, not a Frobenius one, and it is exactly `1` degree, at the
  edge of the floor. At `m=3` all three arms read `d_F = 4` (no separation at
  this `m`/size).
- **H-avail (dimension constraint — a structural finding, not a degree).** A
  `τ`-invariant (squaring-closed) `F_2`-subspace has dimension equal to a sum
  of 2-cyclotomic-coset sizes mod `n`. Availability tracks `ord_n(2)`, not
  primality:
  - `n = 13` (`ord = 12`): invariant dims `{1, 12, 13}` → **no orbit base at
    ℓ = 5**; skipped.
  - `n = 19` (`ord = 18`): dims `{1, 18, 19}` → **none at ℓ = 7**; skipped.
  - `n = 23` (`ord = 11`): dims `{1, 11, 12, 22, 23}` → a **non-subfield** orbit
    base exists at `ℓ = 11` (ran as the random/subspace dimension).
  - composite `n` (9, 15, 21): rich coset structure → orbit bases at
    index-calculus dimensions: `n=15 ℓ=6` and `n=21 ℓ=8` are genuine
    **non-subfield** `τ`-invariant subspaces (6∤15, 8∤21); `n=9 ℓ=3`, `n=15
    ℓ=5` are the subfields `GF(2^3)`, `GF(2^5)`.
  So a Frobenius-orbit factor base of the dimension `ℓ ≈ n/m` that index
  calculus wants **does not exist for many prime `n`** — the base is
  dimension-constrained. This is independent of the fall degree and is the
  study's firmest finding.
- **Instrument caveat (reported, not a bug).** The library's
  `first_fall_degree` is a **rank-deficiency proxy**; it **exceeded** the HKY
  `d_F` in several cells (e.g. `m=2, n=15, ℓ=6`, orbit: proxy `d_ff = 4` but
  `d_F = 2`; selftest logs the same for the window). This confirms the
  library's own warning — the proxy is not the mathematical first-fall degree
  and is not a lower bound. **`d_F` from the HKY closure is the trustworthy
  number here.** A true syzygy first-fall degree (Caminata–Gorla) was not
  computed.

## `a = 0` vs `a = 1`

Not a system difference. The binary `S_3`
`x_1²x_2² + x_1²x_3² + x_2²x_3² + x_1x_2x_3 + b` is `a_2`-independent, so `K_0`
and `K_1` give the identical summation system and identical `d_ff`/`d_F`
(PREREGISTRATION §2). The study ran the system once per `(n, m, ℓ, arm,
draw)`; the two Koblitz values share every row.

## Honest limits

- `n ≤ 23`, `m ≤ 3`, `ℓ ≤ 11`, `cmax = 4`. `d_F ∈ {2,3,4}` only; no growth is
  visible because the ceiling is below where the prior program first saw
  `d_F ≥ 5` (`m = 3`, `ℓ ≥ 7`). The question "does the orbit effect appear
  only beyond toy size" is **open** — this pilot cannot answer it.
- 3 draws per cell; the A/A floor is read as `≤ 1` degree but is not a
  calibrated variance. A validated study needs more draws and a proper A/A
  arm (two random sub-arms), which the pod plan sizes.
- The orbit arm's `ℓ_eff` is dimension-constrained and sometimes overshoots
  the target (e.g. `n=21`: target 7 → `ℓ_eff = 8`); all arms are matched to
  `ℓ_eff`, so comparisons are clean, but the ladder in `n` is not at a fixed
  `ℓ`.
- The polynomial-window "structured < random by 1" signal is the one result
  worth a larger run; it is **not** the Frobenius effect the study set out to
  find.

## Per-degree profiles / operation counts

Each row in `raw/pilot.jsonl` carries `per_degree_macaulay`
(rows/cols/rank per Macaulay degree) and `per_degree_closure`
(plain/closed/of/rounds/`fell` per HKY degree `c`) — the exact row/rank
counts and closure dimensions behind every `d_ff`/`d_F`.
