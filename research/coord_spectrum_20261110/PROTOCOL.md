# F3: Fourier spectrum and additive energy of coordinate functions along ⟨G⟩ (protocol, frozen 2026-11-10)

Moonshot M1/M5 (main session) and D1 (`research/prime_moonshot_20261110/ideas-generator.md`).
Class: **boundary diagnostic**, exact counts, no `S`, no rho row.

## 0. Mechanism and the bound it must escape
For `g : ⟨G⟩ → F_p` cheap to evaluate, define `S_g(a, b) = Σ_{k ∈ Z/r} e(a·g([k]G)/p + b·k/r)`
and `η_g = max_{(a,b) ≠ (0,0)} |S_g(a,b)| / r`. A coefficient `η ≫ r^{−1/4}` would give the
logarithm by a hidden-shift step in about `log r / η²` group operations (generator's derivation,
`[derived]`, not checked). For `g` of algebraic degree `d` in the coordinates the Weil bound gives
`η ≲ 2d·r^{−1/2}` (Kohel–Shparlinski, `[recalled]`): rho's barrier restated. Only functions of
degree `≳ r^{1/4}` that stay cheap can escape it. The second statistic is the additive energy
`E(A) = #{(a,b,c,d) ∈ A⁴ : a + b = c + d in F_p}` of `A_K = {x([k]G) : 1 ≤ k ≤ K}`, which single
character sums do not control; an excess over random growing with `p` means Freiman-type
structure in the orbit (4-term x-coordinate relations harvestable without summation polynomials).

## 1. Rows
- **R16, R20:** 8 prime-order curves each, `p` = largest prime below `2^16` / `2^20`, `(a,b)` from
  seed `F3/curve/<bits>/<i>`, `i = 0..7` (first pair with prime order, `j ∉ {0,1728}`, `#E ≠ p`),
  `G` = first point by ascending `x` then `y`.
- **ANOM:** 2 anomalous curves (`#E = p`), `p ≈ 2^16` and `2^20`, built by CM with `D = −11`
  (`4p − 1 = 11v²`, `j = −32768`, the twist with order `p`); seed `F3/anom/<i>` selects `v`.
  Positive control.
- Functions `g` per curve: `x`, `y`, `χ(x + c)` for `c ∈ {0, 1, 2}` (Legendre, values `±1` mapped
  to `{1, p−1}`), `D_n(x)` the Dickson polynomial of degree `n = ⌈r^{1/4}⌉` (the Weil-escape
  class), `λ` the Smart p-adic logarithm (ANOM only: `t([p] s₀(Q)) / p mod p` with H110's section;
  must be linear in `k`), `PLANT`: `g(k) = ⌊k·p/r⌋` (synthetic heavy coefficient, every curve),
  `NULL`: 20 uniformly random functions `Z/r → F_p` per curve (seed `F3/null/<curve>/<j>`).

## 2. Instrument (native Rust, std only, `tool/`)
- `S_g(a, b)` for all `b` by an exact-arithmetic FFT (f64, length `r`, error bound recorded) at
  `a = 1` and 63 further `a` from seed `F3/a/<curve>`; `η_g` = max over the 64 × `r` table,
  reported with the `(a, b)` that attains it and the null's `η` distribution (same 64 `a`).
- `E(A_K)` exactly, by hashing pair sums, for `K ∈ {⌈r^{1/2}⌉, ⌈r^{2/3}⌉}`, against 50 random
  subsets of `F_p` of size `K` (seed `F3/energy/<curve>/<j>`): mean and sd of the null; ratio.
- Counts recorded: FFT operations, pair sums, group operations.

## 3. Predictions (frozen)
P1. `η_x, η_y, η_χ, η_D` within `[0.8, 1.25] × median(η_NULL)` on every R16/R20 curve.
P2. `λ` on ANOM and `PLANT` everywhere: `η ≥ 0.5` (detector sees a linear function).
P3. `E(A_K) / mean E(random)` within `1 ± 3·sd/mean` of the null on every curve and `K`.
P4. ANOM's `x, y, χ` rows behave as P1 (anomaly lives in `λ`, not in coordinates).
Prior that P1 or P3 fails on ≥ 2 curves of one size: 0.03.

## 4. Decision rules (first applicable)
1. **Instrument failure:** P2 fails, or FFT error bound exceeds `10^{−6} r`. Fix, re-freeze, re-run.
2. **Lead alive:** a non-control `g` has `η_g > 1.5 × median(η_NULL)` on ≥ 2 curves of one size, or
   P3 fails in the same direction on ≥ 2 curves of one size. Then replicate on 16 fresh curves and
   at `2^24` before any claim.
3. **Closed:** otherwise: no heavy Fourier coefficient and no additive-energy excess in the
   coordinate orbit for these function classes at `r ≤ 2^20`.
Rule 3 does not bound functions outside the classes listed, higher Gowers norms (`U³` and up),
or larger `p`.

## 5. Route and outputs
Local host, exact counts (no timing result; §9a confirmatory EKS run open). `results/run.json`,
README with the rule-4 reading, one SVG (η ratios and energy ratios per curve with null bands),
REPORT.pdf, session-log entry. Run by a task card after `FREEZE.sha256`.
