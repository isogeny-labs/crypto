# Smith's conditions on even-degree genus-3 models (frozen protocol, 2026-11-15)

Follow-up to `research/smith_near_prime_20261114/RESULTS.md`, which closed Smith's
route for prime-order Jacobians in the odd-degree model (`deg f = 7`, cycle type
`(7, 1)` for irreducible `f`, no Frobenius-stable pairing). This census measures the
even-degree model `y² = f(x)`, `deg f = 8`. Every figure `[pcc:unverified]`.

## 1. Mechanism under test

The eight Weierstrass points are the roots of `f`; Frobenius permutes them with cycle
type equal to the factor degrees of `f`. A rational tractable subgroup needs a
Frobenius-stable pairing: every odd cycle length must occur an even number of times
(`smith_kernel_coverage::has_stable_matching`). An irreducible octic (type `(8)`)
admits one (pairs `{r, r^{p^4}}`), unlike the odd model's `(7, 1)`.

### 1a. Rational 2-rank: the brief's formula and a correction made before data

The task brief states `dim J[2](F_p) = k − 2` when some factor of `f` has odd degree
and `k − 1` when all factors have even degree. Working it out before the run gives a
different value in the second case. With `Ω` the roots, `M_0` the even subsets of
`Ω` and `G = ⟨σ⟩` Frobenius, `0 → F_2 → M_0 → J[2] → 0` (`F_2` = the class of `Ω`)
gives

`0 → F_2 → M_0^G → J[2]^G → H¹(G, F_2) → H¹(G, M_0)`.

`dim M_0^G` is `k − 1` with an odd-degree factor (even-size unions of orbits) and `k`
when all orbits are even. `G` is procyclic, so `H¹(G, M) = M / (σ − 1)M`, and the
connecting class is nonzero in `J[2]^G` exactly when `Ω = v + σv` for some even
`v ∈ M_0`, i.e. `σv = Ω \ v`: possible iff every orbit has even length (alternate
points of each orbit; `|v| = Σ d_i / 2 = 4` is even). Hence

| factor degrees | `dim J[2](F_p)` (this protocol, `r2`) | brief (`r2_brief`) |
|---|---|---|
| some odd degree | `k − 2` | `k − 2` |
| all even degrees | `k` | `k − 1` |

Sanity check in genus 1 (`y² = quartic`): an irreducible quartic has cyclic Galois
action `(1234)`, which fixes the resolvent root `r1 r3 + r2 r4`, so the Jacobian has a
rational 2-torsion point: rank 1 = `k`, not `k − 1 = 0`. Type `(2, 2)` gives full
2-torsion, rank 2 = `k`.

Consequences, stated before data:

- Type `(8)` has `r2 = 1`: `#J` is even for every irreducible octic. A prime-order
  Jacobian (`r2 = 0`) needs an odd-degree factor and `k = 2`: types `(1, 7)` and
  `(3, 5)`. Neither has a stable pairing (each odd length occurs once). So in the
  even-degree model too, **a prime-order genus-3 Jacobian has no rational tractable
  subgroup and Smith's conditions cannot pass** (a theorem given the formula; the
  census tests the implementation against it).
- Odd `#J` occurs exactly for types `(1, 7)` and `(3, 5)`.

Both `r2` and `r2_brief` are recorded per curve. The brief's predictions E1–E5 are
kept verbatim below and scored as written; the corrected formula adds E1b and the
counter-prediction E3' (the brief's E3 is expected to FAIL).

## 2. Instrument

`src/bin/smith_even_census.rs`. Curves: `f` of degree 8, the eight lower coefficients
uniform in `F_p`, leading coefficient uniform in `F_p^*` (not forced monic; `lc_square`
recorded); non-squarefree draws are recorded and skipped. `smith_prop24::analyze_curve`
accepts a non-monic `f` (it factors the monic associate for the Weierstrass points and
evaluates Proposition 4 on `f` itself), so no normalisation is made and
`full_success(false)` is the outcome for the curve `y² = f(x)` as drawn
(`monic_normalised = false` on every record; `full_success(true)`, the twist, is
recorded alongside).

Group order: `N_k = #C(F_{p^k})`, `k = 1, 2, 3`, by enumeration of `x ∈ F_{p^k}` with a
table of squares of `F_{p^k}` (affine points `1 + χ(f(x))`; two points at infinity
when `lc` is a square in `F_{p^k}`: for odd `k` iff square in `F_p`, always for
`k = 2`). `s_k = N_k − p^k − 1`, `k c_k = Σ_{i=1..k} s_i c_{k−i}`,
`L(T) = 1 + c_1 T + c_2 T² + c_3 T³ + p c_2 T⁴ + p² c_1 T⁵ + p³ T⁶`, `#J = L(1)`,
twist `#J' = L(−1)`. A curve whose Newton division is inexact or whose `N_k` or `#J`
leaves the Weil bounds gets `order = null`.

Order verification before the census (`smith_even_census verify --primes 31,47 --n 30`):
(i) 30 random monic degree-7 curves per prime against `JCurve::jacobian_order`
(Cantor arithmetic, exact), also `N_1, N_2` against `JCurve::count_points` and the
twist order `L(−1)` against the Cantor order of the twist; (ii) 30 degree-8 curves
per prime with a rational root `r`, mapped by `x = r + 1/u` to a monic degree-7 model
and compared against `JCurve::jacobian_order`, plus the twist identity
`L_f(−1) = L_{νf}(1)`. Every comparison must agree; otherwise the census does not run.
(iii) Weil bounds and Newton integrality are checked on every census curve; (iv) E1/E1b
below check the order against the 2-rank.

## 3. Run

`smith_even_census run --primes 31,47,67,97 --admitted 150 --prime-order 40 --max-h 8
--max-seeds 30000 --threads 3 --out even_census.jsonl`. Per prime, seeds `1, 2, ...` in
blocks of 64 until at least 150 curves with `#J = h r` (`ic_control::split_order`,
`h <= 8`, `r > h` prime) and at least 40 curves with `#J` prime are recorded, or 30,000
seeds. Every curve of a started block is recorded.

## 4. Predictions (fixed before data)

Rates are shares of `full_success(false)`; intervals are Wilson 95%; pooled over primes
unless stated.

| id | prediction (brief, verbatim in substance) | falsified if |
|---|---|---|
| E1 | `v_2(#J) >= r2_brief` on every curve with a determined order | a violation |
| E2 | pass rate among `k = 1` (type `(8)`) curves strictly positive, Wilson interval excluding 0 | interval contains 0 |
| E3 | pass rate among prime-order curves positive, and equal (overlapping intervals) to (type-(8) pass rate) × (share of type (8) among prime-order curves) | zero passes, or disjoint |
| E4 | admitted (`h <= 8`) pass rate below the unconditioned rate, intervals disjoint; and the by-cycle-type mixture `Σ_t w_t rate_t` (`w_t` admitted cycle-type distribution, `rate_t` pass rate of all curves of type `t`) lies inside the admitted interval | overlap/reversal, or mixture outside |
| E5 | unconditioned pass rate over all even-degree curves in `[0.15, 0.22]` | outside |

Added by the correction of §1a (also fixed before data):

| id | prediction | falsified if |
|---|---|---|
| E1b | `v_2(#J) >= r2` on every determined order, and `v_2(#J) = 0` iff `r2 = 0` (iff type `(1,7)` or `(3,5)`); in particular every type-`(8)` curve has even `#J` | a violation |
| E3' | prime-order curves: 0 passes, 0 type-`(8)` curves, every one of type `(1,7)` or `(3,5)` | any pass, or any other type |

Instrument diagnostics (must be 0): `smith_prop24` pair-condition, eval-mismatch and
Lemma-4 failures; `x_check` (Recillas' curve point count vs Proposition 4 square
class) disagreements and count errors.

## 5. Outputs

`even_census.jsonl` (`data.sha256`), `verify.json`, `render.py` (layout only: counts
and Wilson intervals) producing `tables.md`, `RESULTS.md` with tables per prime and
pooled, by `k`, by cycle type, and the scorecard.
