# Closure depth: why λ is a closure pseudo-solution, and a route to a growing bound on d_F (2026-11-15)

Status: research note. There is no `pcc` claim, so every number below is `[pcc:unverified]`. Everything ran locally on one core, using the Python closure of lab A (`../../kummer_wave2_20261115/KM3-LQR-CLOSURE/hky_closure.py`), which was validated against committed Rust `d_F` values.

## Round-indexed closure

For a degree cap `c`, define:
- `W_0 = span(F ∩ R_{≤c})`;
- `W_r = W_{r−1} + span{ x·g : g ∈ W_{r−1} ∩ R_{<c}, x a variable }`.

The union of the `W_r` is the HKY closure `V_{F,c}`. Two quantities come out of it:
- `r*(c)`: the first round in which 1 enters;
- `G_t(c)`: the number of rounds in which the space still grows.

## Depth lemma (proof below) `[pcc:unverified]`

**Statement.** Take Kummer towers of height `t`.
- At m = 2, with the hypotheses of D4: `r*(c) ≥ 2t` for every `c`.
- At m = 3, with the hypotheses of D10 (including `λ(1) ≠ 0`): `r*(c) ≥ 2t + 2`.

**Proof.**
1. Every element of `W_r` can be written `Σ a_μ μ·F_i`, where each `μ` is a product of at most `r` variables. This holds by induction, since a closure step multiplies one representative by one variable. With `u`-factors counted, μ reduces in `R` to `z^k u^j`.
2. Popcount is subadditive modulo `K`. So `popcount(k_i)` is at most the number of tower-`i` factors of `μ`.
3. *m = 2.* `λ_f(g) = Σ_x x_1 x_2 g(x)/f(x)` is zero on `μ·f` unless `k_1 = k_2 = K − 1`, by orthogonality of characters on `μ_K` (as in D10). That needs at least `t + t` factors. Since `λ_f(1) ≠ 0`, it follows that `1 ∉ W_r` for `r < 2t`.
4. *m = 3.* By D10's proof, λ is zero on every `ν·F_2`, and on `μ·F_1` unless `k_1 = k_2 = K−1` and either `j ≥ 2`, or `j = 1` with `k_3 = K − 1`. That needs at least `2t + 2` factors. ∎

**Consequences.**
- **Explanation of LQR-CLOSURE.** At every `c`, the elements of `V_{F,c}` reached within fewer than `2t` (resp. `2t+2`) rounds are killed by λ. If the closure saturates within that many rounds, λ kills all of `V_{F,c}`.
- **Lower-bound criterion.** If `G_t(c) < 2t` at m = 2 (resp. `< 2t + 2` at m = 3), then `d_F > c`. A growing lower bound on `d_F` therefore reduces to bounding the saturation depth `G_t(c)` by a function of `c` alone.

## Measurements

The tables below are filled from `raw/*.jsonl`.
