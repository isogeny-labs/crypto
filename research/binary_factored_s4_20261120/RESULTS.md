# Binary Semaev `S₄`: closed form and the `S₃·S₃` factorisation (2026-11-20)

Branch `claude/binary-radical-quartic-20261120`, PR #4839. Code:
`src/cryptanalysis/binary_radical_root.rs`; benches
`examples/binary_radical_quartic_bench.rs` (per pair) and
`examples/binary_factored_decompose_bench.rs` (whole decision). The raw
outputs are in `raw/`. Machine: 4-vCPU Intel Xeon @ 2.10 GHz (cloud
container), rustc 1.97.0, `--release`, every arm compiled with
`pclmulqdq`. These are local development timings: unpriced, unvalidated
(pre-gate), and not a leaderboard row.

## Question

`semaev_decomp::decompose` decides an `m = 3` target over
`V = {x < 2^l}` (Koblitz curve `y² + xy = x³ + x² + 1` over `F_{2ⁿ}`) by
looping over pairs `(X₁, X₂)` and finding the roots in `V` of the quartic
`S₄(X₁, X₂, X, x_R)` in `X`, as `gcd(f, L_V mod f)`: about `l` Frobenius
squarings modulo `f` and a straight-line gcd. Can that step be done in
closed form more cheaply, as the radical root finder did over `F_p`
(#4584, #4593)?

## 1. Generic closed form (Dickson): correct, mostly slower

A quartic over `F_{2ⁿ}` is solvable inside `F_{2ⁿ}`. This contradicts
`petit_quisquater.rs:747` ("no closed-form root via Artin–Schreier"). The
steps:

- An affine shift and an inversion give `z⁴ + αz² + βz + γ`.
- The resolvent cubic `t³ + t = a` is a Dickson equation, `t = D_e(a)`, a
  Lucas ladder of about `n` multiplications.
- Two half-traces finish it.

Agreement with the gcd held on every quartic (about 11.8M, `n ∈ {23,
29, 31, 37, 41, 47, 53, 59, 61}`, `l ∈ {6, 8, 10, 12, 14}`). Speed was
0.36× to 1.72× of the gcd: faster only at small `n` with large `l`,
because the ladder costs `≈ n` multiplications against the gcd's `≈ l`
(`raw/01`).

**Finding:** the resolvent cubic of an `S₄` quartic had no root in
`F_{2ⁿ}` on **0** of about 11.8M quartics, against `1/3` for a random
quartic.

## 2. Why: `S₄ = S₃(u, x_R, X)·S₃(u', x_R, X)`

The roots of `S₄(X₁, X₂, X, x_R)` in `X` are `x(R ± P₁ ± P₂)`. They pair
as `x(R ± S)` and `x(R ± S')` with `S, S' = P₁ ± P₂`. So the quartic is
the product of two `S₃` quadratics with `u, u' = x(P₁ ± P₂)`, which are
the roots of `S₃(X₁, X₂, ·)`. Finding the roots therefore takes only
Artin–Schreier steps (a half-trace each, applied by table). `u` does not
depend on the target.

**Irrational `u` (about half of all pairs).** Then `u' = ū`. A rational
root `X` would be a root of both conjugate factors, which forces
`S₃(x_R, X, ·) ∝ S₃(X₁, X₂, ·)`. Comparing coefficients gives
`{x_R, X} = {X₁, X₂}` or `{1/X₁, 1/X₂}`. So these pairs have no root in
`V` unless `x_R ∈ {X₁, X₂, 1/X₁, 1/X₂}`. That guard is O(1); guarded and
degenerate pairs (`X₁ = X₂`, `X₁X₂ = 0`, `u = x_R`, `u·x_R = 0`) go to
the existing gcd.

Measured: irrational `u` on 44.8–50.1% of pairs; with the gcd applied to
all of them, **0** had a root in `V` (`raw/02`).

## 3. Per pair, on identical quartics (`raw/03`)

Every pair agrees with `gcd(f, L_V mod f)` on the root count.

| `l` | speedup range over `n = 23 … 61` | factored ns/pair |
|---|---|---|
| 6 | 1.75–2.59× | 62–86 |
| 8 | 2.81–3.93× | 47–73 |
| 10 | 3.87–6.00× | 43–67 |
| 12 | 4.48–6.76× | 46–57 |
| 14 | 3.98–5.71× | 50–71 |

The factored cost is flat in `n` and `l`. The gcd grows with `l`.

## 4. Whole decision, `decompose` against `FactoredOracle` (`raw/04`)

Every target got the same decision from all three arms, and every witness
satisfies `eval_f3 = 0`. "stream" computes `u` per target. "table"
precomputes every pair's rational `u`, which costs about `2^{2l}·8` bytes.

The table below covers random targets for `n ∈ {41, 53, 61}`. None of
them decomposes, so every arm exhausts all pairs.

| `l` | targets | stream | table | table build / size |
|---|---|---|---|---|
| 6 | 3000 | 2.12–2.18× | 3.16–4.07× | <1 ms / ~0 MB |
| 8 | 300 | 3.68–4.01× | 5.05–7.67× | 1.3–1.9 ms / 0.3 MB |
| 10 | 24 | 4.69–5.86× | 8.10–12.63× | 23–27 ms / 4.2 MB |
| 12 | 3 | 6.55–8.10× | 11.37–12.78× | 0.3–0.45 s / 67 MB |

Artin–Schreier solves per pair are about 0.99: half the pairs have a
rational `u`, and each such pair has two. The whole-decision gain exceeds
the per-pair gain because the factored route never builds the quartic or
inverts its leading coefficient.

**Caveats.**
- At `n = 23, 31` many targets decompose, so early exits dominate those
  rows; they are in `raw/04` but not in the summary.
- The `l = 12` rows use 3 targets.
- These are single-pass timings in a shared cloud container. Repeated
  baseline runs varied by about ±20%.

## What it does and does not change

- It is a constant factor on the pairs-and-solve oracle. That oracle's
  relation-collection cost stays `Θ(2ⁿ)` independent of `l` (see the
  header of `semaev_decomp.rs`). It does not move IC toward Pollard rho.
- The factorisation shows the quartic step is a pair-sum lookup in
  disguise: `u = x(P₁ ± P₂)`. The subtraction oracle of the Koblitz
  pipeline (`koblitz_fast`: `|F|` subtractions of `R` against a
  pair-sum table) already costs `O(B)` per target against the `O(B²)`
  here. The factored oracle is best read as making the `O(B²)`
  experiments in `semaev_decomp` cheaper, not as a new attack.
- Untested: wide fields (`n > 63`, `Gf2_128`), even `n` (half-trace
  needs odd `n`), and the general-`b` curve (`quartic_in_x3_general`).
  `S₃` holds for any `b` (`x₁²x₂² + x₁²X² + x₂²X² + x₁x₂X + b`), so the
  method should carry over with `b` replacing the constant 1.

## 5. End to end: whole discrete logarithms (`raw/05_*`: console output and a per-repeat JSON extract of the ledger records)

`examples/binary_factored_ic_e2e.rs` runs the boundary ledger's char-2
instance (`ic_boundary::run_char2_instance`). Each instance is a random curve
`y² + xy = x³ + ax² + b` over `F_{2ⁿ}` with cofactor ≤ 8, and the base is the
low-order subspace of dimension `⌈n/3⌉`. The same relation loop, lift,
linear algebra and verification run for every variant, on shared targets
in each repeat, with 8 matched rho walks per repeat. `semaev_s4_factored_m3`
(`FactoredSubspaceOracle`, any `b`) sits beside the existing
`semaev_s4_pairs_and_solve_m3`. Before each instance, the two oracles
were checked on about 200–270 random and planted targets: identical
decisions, every witness verified.

3 repeats per `n`. In all 21 repeat pairs the two S4 arms used the same
trials and relations and recovered the same verified logarithm.

| `n` | log₂ r | pairs-and-solve | factored | speedup (wall) | per-repeat range | factored ÷ rho (wall) | best MITM ÷ rho |
|---|---|---|---|---|---|---|---|
| 15 | 14.0 | 14.0 ms | 7.3 ms | 1.9× | 1.76–2.06× | ~81× | ~4× |
| 17 | 14.4 | 35.1 ms | 21.6 ms | 1.6× | 1.42–1.78× | ~216× | ~5× |
| 19 | 17.0 | 0.39 s | 0.12 s | 3.1× | 3.02–3.27× | ~650× | ~6× |
| 21 | 20.0 | 0.90 s | 0.40 s | 2.2× | 1.92–2.51× | ~1,090× | ~9× |
| 23 | 22.0 | 3.45 s | 1.62 s | 2.1× | 1.88–2.38× | ~2,340× | ~9× |
| 25 | 22.4 | 16.7 s | 6.38 s | 2.6× | 2.47–2.77× | ~8,180× | ~21× |
| 27 | 24.4 | 60.8 s | 24.5 s | 2.5× | 2.45–2.53× | ~16,600× | ~23× |

Linear algebra is under 5 ms everywhere, so the gain is entirely in
relation collection. The measured cost per pair is 110–190 ns factored
against 225–310 ns for the existing loop. That is smaller than the per-pair
gains of §3–4, because here `l = ⌈n/3⌉ ≤ 9` (the gcd's cost grows with `l`)
and many targets decompose and exit early.

**Pricing caveat.** The GAE `S` of the baseline arm can use the pinned
`ns_per_s4_pair` for an instance while the factored arm's rate is
host-measured. At `n = 27` the baseline `S` is 115,459 against an `s_wall`
of 71,541. The table therefore compares wall time on one host and run.

**Answer to "is it faster end to end?"** Yes, by about 1.6–3.1× (2.5× at
`n = 27`) for IC built on the `S₄` pairs-and-solve oracle. It does not
change the ranking. Both S4 arms are 10³–10⁴ times rho and the gap grows
with `n` (the oracle is Θ(2ⁿ)). The meet-in-the-middle pair-table rows
(O(B) per target) stay 4–23× rho and are better than either S4 arm by two
to three orders of magnitude.

## 6. Even `n`: the linear Artin–Schreier solver, end to end (`raw/06_*`)

For even `n` there is no half-trace. PR #5032 adds `ArtinSchreier`, a precomputed `F₂`-linear `M` with `M(c)² + M(c) = c` on the trace-0 hyperplane. It is built by eliminating over `{φ(zⁱ) : 1 ≤ i < n} ∪ {v}`, where `Tr(v) = 1`, and applied through the same byte tables as the half-trace, so a solve costs the same at either parity. `FactoredSubspaceOracle` now accepts any `n ≤ 63`.

Module tests pass 6/6 on a compiling base: the #4839 head with `main`'s `binary_radical_root.rs`, because `main` does not currently parse. The tests:

* `artin_schreier_solves_exactly_the_trace_zero_hyperplane`, for `n = 2..63`.
* `factored_subspace_oracle_decides_as_subspace_oracle`, extended with `n = 12, 16, 18, 22`.

The protocol is the one in §5:
* `binary_factored_ic_e2e`, even `n` from 16 to 26, 3 repeats, 48 matched rho walks per instance.
* A preflight decision check on about 200–250 targets per instance.

| n | log₂ r | preflight | S₄ pairs-and-solve wall | factored wall | speedup (per-repeat) | factored × rho (wall) |
|---|---|---|---|---|---|---|
| 16 | 14.0 | identical (127/253 decompose) | 7.8 ms | 4.2 ms | 1.85× (1.45–2.25) | 121 |
| 18 | 15.0 | identical (79/245) | 36.3 ms | 17.1 ms | 2.12× (2.01–2.21) | 416 |
| 20 | 18.0 | identical (70/227) | 0.32 s | 0.11 s | 2.81× (2.50–3.38) | 1,590 |
| 22 | 20.0 | identical (88/204) | 0.64 s | 0.23 s | 2.77× (2.63–2.95) | 1,629 |
| 24 | 21.0 | identical (19/200) | 2.29 s | 0.98 s | 2.35× (2.35–2.35) | 3,531 |
| 26 | 23.0 | identical (47/202) | 10.7 s | 3.99 s | 2.68× (2.62–2.77) | 9,283 |

All 36 variant runs recovered and verified the logarithm. In each repeat both arms took identical trials and relations, so the speedup is the oracle alone. Per-pair host calibration, `ns_per_s4_pair` against `ns_per_s4f_pair`:

| n | 16 | 18 | 20 | 22 | 24 | 26 |
|---|---|---|---|---|---|---|
| `ns_per_s4_pair` | 243.7 | 195.9 | 224.5 | 264.4 | 249.4 | 258.4 |
| `ns_per_s4f_pair` | 171.7 | 94.5 | 100.8 | 94.5 | 99.0 | 96.9 |

That is about 2.6× per pair from `n = 18` on. The ranking is the same as at odd `n`. The MITM pair-table rows are about 75–100× rho at `n = 26`, and both `S₄` arms remain 3–4 orders of magnitude above rho. Even `n` is a parity extension of a constant-factor gain, not a change in exponent. Speedups are wall-clock ratios on one host (4 vCPU container, single-threaded oracle), with the same caveat as §5 that the GAE `S` columns depend on the calibrated ns per pair.
