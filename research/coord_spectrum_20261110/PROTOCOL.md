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

## 6. Amendment and disclosure before the full run (2026-11-10, after the implementer's `--quick`)
Predictions and rules were committed (`c8bbc80e1a`) before the tool existed. The implementer's
`--quick` (R16, 2 curves, 5 nulls; not committed, not scored) exposed two defects in §2's
*statistic*, not in the mechanism, and they are corrected here with their reasons:
1. **`b = 0` is excluded from the gated `η`.** Because `⟨G⟩ = E(F_p)` on a prime-order curve, the
   coefficient `S_g(a, 0) = Σ_{Q ∈ E(F_p)} e(a·g(Q)/p)` is a sum over the whole group and does not
   depend on `G` or on the labelling `k`; it is a character sum of `g` over `E(F_p)` (Kloosterman-type
   for Dickson maps when `n | p − 1`, which the quick run showed as an inflated `b = 0` value) and
   cannot carry logarithm information. The gated statistic is `η_g = max_{a, b ≠ 0} |S_g(a,b)|/r`;
   the `b = 0` column is reported separately as a function-level statistic.
2. **Legendre rows enter as complex `±1` values**, i.e. `S_χ(b) = Σ_k χ(x([k]G) + c)·e(b·k/r)`,
   `η_χ = max_{b ≠ 0} |S_χ(b)|/r`, against a null of 20 uniformly random `±1` sequences of length `r`
   (seed `F3/nullpm/<curve>/<j>`). The original mapping `±1 ↦ {1, p − 1}` made `e(a·g/p)` nearly
   constant, which is an encoding artefact (quick run: `η = 1` at `(1, 0)` for every χ row).
P1–P4 and the rules of §4 are unchanged in form and now read the `b ≠ 0` statistics; the null for
P1 is the matching null (`F_p`-valued random functions for `x, y, D_n`; `±1` sequences for `χ`).
Also recorded: the "2^16" anomalous curve has `p = 74869` (the `D = −11` CM search lands above
`2^16`); orders are by exhaustive Legendre count; `g(O) = 0` at `k = 0`.
3. **Nulls carry the negation symmetry of the coordinate map.** Along the orbit `k ↦ [k]G`,
   `x, χ(x + c), D_n(x)` are even (`g_{r−k} = g_k`) because `x(−P) = x(P)`, and `y, λ` are odd
   (`g_{r−k} = −g_k`) because `y(−P) = −y(P)` and `λ` is a homomorphism. A uniformly random
   function has neither symmetry, so its `η` distribution is not the right null for either
   class (the quick run after items 1–2 showed every χ row at `0.59–0.61` of the uniform `±1`
   null, on both curves and all three shifts: a symmetry-class effect, not an orbit effect).
   Nulls are therefore drawn per symmetry class: `NULL_even`: a random value per `{P, −P}` class
   (random `F_p` values for `x, D_n`; random `±1` for `χ`), assigned to both members; `NULL_odd`:
   a random `v` on `P` and `−v` (resp. `p − v`) on `−P`, for `y` and `λ`; `g(O) = 0`. Seeds
   `F3/null/<class>/<curve>/<j>`, 20 each. Every ratio in P1/P4 is against the null of its
   function's class. PLANT keeps the uniform null (it is a synthetic control).
Items 1–3 change the statistic and the null, not the predictions or the rules.
**Freeze note.** After items 1–3 the quick run (R16, 2 curves, 5 nulls per class) read x 0.74–0.79,
y 0.73, χ 0.455–0.464 and D_n 0.99–1.04 of their class-null medians, energy 0.9965–0.9999, PLANT
1.0. P1's band `[0.8, 1.25]` is therefore expected to **fail downward** on x, y, χ; it is kept as
written and will be scored as failed. (Reading to be tested in the full run, not a rule: a Weil-type
bound `|S_g(a,b)| ≤ C√p` for every `b` sits below the random extreme-value maximum `≈ √(r ln r)`, so
orbit spectra are flatter than random; rule 2 fires only upward and is unaffected.) No further
amendment after this line.
