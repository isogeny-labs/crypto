# PROTOCOL: KM-POPCODE-STEPS, lab D of wave 2 (2026-11-15)

Frozen before any sigma datum was computed. Every number produced under this protocol is
`[pcc:unverified]`. Code and logs are Python 3 + numpy (task instruction; AGENTS.md "no Python"
means none of this is admitted evidence for the repository; it is a development check).

## 1. Object (most literal reading of the lead's definition)

- `P = F_p[z1,z2]`, infinite tower, degree of a monomial `z1^a z2^b` = `pop(a) + pop(b)` (binary [pcc:unverified]
  digit sums); degree of a polynomial = max over its monomials. `f` = generator (below). `fP` = the
  polynomial multiples of `f` (no relation `z^K = 1`).
- `M_d(s) = { P in fP : every exponent of z1 and of z2 is < 2^s, deg P <= d }`.
- `E_d(s) = span( M_d(s-1)  U  { y*g : g in M_{d-1}(s), y in {z1^(2^j), z2^(2^j) : 0 <= j < s},
  y*g has no exponent >= 2^s ("no carry out of level s-1"), deg(y*g) <= d } )`.
  Reading used: for each of the 2s variables y separately, g ranges over the subspace
  `{ g in M_{d-1}(s) : every monomial of g has no all-ones run from bit j up to bit s-1 in the
  exponent of y's variable }`; `E` is the sum of `M_d(s-1)` and the 2s images `y*(that subspace)`.
  `deg(y*g) <= d` is automatic (deg g <= d-1, y has degree 1).
- `nu_d(s) = dim M_d(s) - dim E_d(s)`; `sigma_W(d) = max{ s-1 : nu_d(s) > 0 }`, taken over the
  computed range `1 <= s <= s_max`. `sigma` is only established when `nu_d(s) = 0` for every
  computed `s` after the last positive one (a finite window; nothing is proved beyond `s_max`).
- Rule under test (the lead's alignment): `d_F(W,t) = 1 + min{ d : sigma_W(d) > t }`.

Ambiguities noted (not resolved by choice beyond the reading above): (a) "no carry" is read per
variable y, as above; a per-monomial truncation reading is impossible because `y*g` must stay in
`fP`. (b) `sigma` as a `max` over all `s` is not computable; the finite window is stated per cell.
(c) `M_1 = 0` (Lemma 4 of lower_bound) so `sigma_W(1)` is undefined (taken as -infinity in the rule).
(d) For `W >= 2` the lead's "level < s" tower variables are `z_i^(2^j)`; the same set is used.

## 2. Exact method

- Membership `P in fP` for P of bidegree <= (N,N), `N = 2^s - 1`, is tested by vanishing at points of [pcc:unverified]
  the curve `f = 0` over `F_p`. For an irreducible bidegree-(W,W) curve, P vanishing at more than
  `W*(N+N) = W*(2^(s+1) - 2)` distinct points forces `f | P` (Bezout on P^1 x P^1). The full point set
  used has `W*2^(s+1)` points. For speed the elimination may use a random subset of `n + 200` of them
  (n = number of unknown monomials); the resulting kernel is then verified against ALL
  `W*2^(s+1)` points, which makes it exact (kernel(subset) contains the true kernel, and the
  verification shows the computed kernel lies in it). Irreducibility certificate: the number of
  `F_p`-points is >= `W*2^(s_max+1)` and the full-evaluation rank at s = 3..5 equals
  `(N+1)^2 - (N+1-W)^2` (the dimension of `{f*Q}`).
- Kernels by dense blocked Gaussian elimination mod p, float64 BLAS with `p < 2^16` (products
  < 2^32, partial sums < 2^53, so exact); no int64 overflow (checked by `A @ K == 0` verification).
- `E_d(s)` rank is computed in the free-column coordinates of `M_d(s)` (each vector of M_d(s) is
  determined by its entries at the free columns of the RREF kernel).
- Primes: 40961 (main; `p > 2^15` so exponents `< 2^14 < p`, no Frobenius effect) and 12289 (second
  prime, one W=1 ladder up to s = 12 where `2^(s+1) <= p`, exponents < p).
- Draws: coefficients `c_ab` (0<=a,b<=W) uniform in `F_p^*` from `numpy.random.default_rng(seed)`,
  seeds `0x504B4D54 + 100*W + draw`; for W=1 redraw while `c00*c11 - c10*c01 == 0`.

## 3. Pre-registered values and classification (stated before computing)

Measured `d_F` steps on disk (growth_law README §5, `m=2`): W=1: `3,3,4,4,4,4,5,5,5,5` for
`t=2..11`, `>= 6` at `t = 12` (falsify cell); W=2: `4,4,4,4,5,5,5,5,6,6`; W=3: `5` for `t=2..8`,
`6` at `t=9`, `>= 6` at `t=10`.

Inverting the rule on those steps gives sigma_1 = 4, 8, 12 (d=2,3,4), sigma_2(3) = 6, sigma_2(4) = 10,
sigma_3(4) = 9. **All six values are fitted**: each is read off a measured step. By the brief's rule
("only values not already fitted from on-disk data count as tests") none of the six listed
numbers is an out-of-sample test of the *value*. What an independent computation of `sigma` tests is
the *definition* (nu via E_d produces these numbers without being shown the d_F steps). So:

| item | status before computing |
|:--|:--|
| sigma_1(2) = 4 | fitted from the t=4 step; native/Python `dim M_2(s) = 4,9,14,15,15,15` on disk (p = 786433, not admitted for the Python part). Used as the **positive control** |
| sigma_1(3) = 8, sigma_1(4) = 12 | fitted (steps t=8, t=12); first computation of sigma at d = 3, 4 (definition-level test) |
| sigma_2(3) = 6, sigma_2(4) = 10 | fitted (steps t=6, t=10); first computation (definition-level test) |
| sigma_3(4) = 9 | fitted (step t=9); first computation (definition-level test) |
| sigma_3(3) undefined (no degree-3 multiple), sigma_2(2) <= 1 | implied by the same data; computed as a by-product |
| period `sigma_W(d+1) - sigma_W(d) = 4` | the claim computed pairs: W=1 (4,8,12), W=2 (6,10) |
| out-of-sample content | only: `nu = 0` persists after the last positive s (finiteness), and the exact value `d_F(1,12) = 6` (disk has only `>= 6`) |

Cells computed (d, s range): W=1: d=2 s<=9, d=3 s<=12 (d=3 also at p=12289 s<=12), d=4 s<=13 and
s=14 if time allows; W=2: d=2 s<=10, d=3 s<=12, d=4 s<=12; W=3: d=2,3 s<=12, d=4 s<=12.
Two draws per W where the budget allows (largest d=4 cells: draw 0 first; any cell run once is
stated).

## 4. Kill / survive rule

- Survives a cell iff every computed sigma equals its registered value AND nu = 0 for all computed s [pcc:unverified]
  after it. Dies on any mismatch (e.g. sigma_1(3) != 8, sigma_1(4) != 12, sigma_2(3) != 6,
  sigma_2(4) != 10, sigma_3(4) != 9), or if nu > 0 persists to s_max (no sigma).
- Period right but offsets wrong, or values right on one draw and not the other, is reported as
  partial and not as support.
- A match is a predictor statement about a fitted rule, not a proof, and not an improvement or a
  speedup.

## 5. Controls

- **Positive control.** sigma_1(2) = 4 reproduces (nu_2(5) = 1, nu_2(s) = 0 for 6 <= s <= s_max), and [pcc:unverified]
  `dim M_2(s) = 4,9,14,15,15,15`, `dim M_3(s) = 8,27,64,121,194` (s = 2..6) reproduce the on-disk
  values, on (i) the on-disk instance `f1 = 3 + 1234 z1 + 5678 z2 + 9012 z1 z2` over `F_786433`
  and (ii) the generic draws at p = 40961.
- **Negative control N1 (planted grid zero).** W=1 `f` with `f(1,1) = 0` (irreducible Moebius curve
  through the grid point (1,1) of every tower height; f is not a unit of any `R_t`, so `d_F = infinity`
  at every t). Prediction registered: none on the sign. Reading: sigma is computed in the infinite
  tower, which does not contain K, so it may be blind to non-unit-ness. If the profile is identical
  to the generic one, the control is a **finding that sigma does not see the grid zero**, so the rule
  `d_F = 1 + min{...}` can only be a statement for units, and the control is vacuous as a detector of
  genericity; if it differs, the difference is reported.
- **Negative control N2 (lead's own).** `f = (1+z1)(1+z2)` (reducible; zeros on the grid at
  z1 = -1 or z2 = -1). sigma must differ from generic.
- **Noise/validity checks.** kernel verified against all Bezout points; two primes at one W=1 ladder;
  elimination routine unit-tested against a naive rank on small random low-rank matrices; point
  count and full-evaluation rank certificate for irreducibility.
