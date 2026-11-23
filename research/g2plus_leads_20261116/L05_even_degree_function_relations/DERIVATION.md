# L05 derivation: divisors of `y - a(x)` on an even-degree hyperelliptic model

Status: mathematics written by the L05 coordinator before any experiment code or
data, `[pcc:unverified]`. A throw-away check (coordinator scratch script, not part
of the experiment) mapped about 400 split members of these families on five tiny
curves (g = 2: p = 31, 53, 101; g = 3: p = 37, 61) to an odd model of the same
curve and summed them with Cantor's algorithm: every one was the identity, and
both mutations of section 6 were always rejected. That check fixes conventions;
it is not data of this lead. The experiment's gate G2 repeats it under the
frozen protocol.

## 1. Setting

`p` odd prime, `g >= 2`. `C: y^2 = f(x)`, `f in F_p[x]` monic squarefree of
degree `2g + 2`. Because `f` is monic, the two points over `x = infinity` are
rational. Call them `inf+` and `inf-`. On a neighbourhood of `inf+` we have
`y / x^(g+1) -> +1`, and near `inf-` it tends to `-1`. The hyperelliptic involution
`i(x, y) = (x, -y)` swaps them. `u = 1/x` is a uniformizer at both, and
`ord_{inf+-}(x) = -1`.

`s(x)`: the unique monic polynomial of degree `g + 1` with `deg(f - s^2) <= g`
(the polynomial part of `sqrt f` at infinity). Its coefficients come from the top
down, with `s_{g+1} = 1`, and for `j = 1..g+1`:

    s_{g+1-j} = ( f_{2g+2-j} - sum_{i=1}^{j-1} s_{g+1-i} s_{g+1-j+i} ) / 2 .

Laurent form: `y = sigma * u^-(g+1) * S(u)` at `inf_sigma`, `sigma = +-1`, where
`S(u) = sqrt(u^(2g+2) f(1/u))` is the power series with `S(0) = 1`. Its first
`g + 2` coefficients are those of `s` reversed.

## 2. The divisor of `y - a(x)`

Let `a in F_p[x]` and `N = f - a^2 = (y - a)(y + a)`, `d = deg N`.

Affine part. The affine zeros of `y - a` are the points `(x_k, a(x_k))` for the
roots `x_k` of `N`, with multiplicity. (The two branches cannot vanish together
at a point with `y != 0`. All points with `y = 0` are left out of the factor base
and of every collected relation, see SPEC.)

Infinity part. There are two cases.

(A) `deg a <= g`. Then `deg a^2 <= 2g < 2g + 2`, so `N` is monic of degree `2g + 2`.
At both points at infinity, `y` has a pole of order `g + 1` and `a` a pole of order
at most `g`. So

    div(y - a) = sum_{k=1}^{2g+2} (P_k) - (g+1)(inf+) - (g+1)(inf-) .

(B) `a` monic of degree `g + 1`. At `inf-`, `y - a ~ -2 x^(g+1)`, a pole of order
`g + 1`. At `inf+`, `y + a ~ 2 x^(g+1)` and `ord_{inf+} N = -d`. So

    ord_{inf+}(y - a) = (g + 1) - d ,
    div(y - a) = sum_{k=1}^{d} (P_k) + (g + 1 - d)(inf+) - (g + 1)(inf-) .

Degree check: `d + (g+1-d) - (g+1) = 0`.

Leading-term cancellation. Write `a = s - r`. Then
`N = (f - s^2) + r (2s - r)`. If `r != 0` has degree `m <= g`, then
`deg(r (2s - r)) = g + 1 + m > g >= deg(f - s^2)`, so `d = g + 1 + m`. Let the
"cancellation depth" `e` (with `0 <= e <= g`) be the number of coefficients of
`a` below the leading one that agree with `s`. So `a` agrees with `s` at
`x^(g+1), x^g, ..., x^(g+1-e)`, and `m = deg r <= g - e`. Generically `d = 2g + 1 - e`, so

    div(y - a) = sum_{k=1}^{2g+1-e} (P_k) + (e - g)(inf+) - (g + 1)(inf-) .

`r = 0` (that is, `a = s`) gives `d = deg(f - s^2) <= g` and `ord_{inf+} >= 1`. The
general formula `ord_{inf+} = g + 1 - deg N` still holds, and the pencils below hit
this member only as their one degenerate member.

Case (C), the mirror: `a` with leading coefficient `-1` swaps `inf+` and `inf-`.
Its relations are the images under `i` of case (B) relations, so they are the
same relations up to sign. They are not generated.

## 3. Pencils (linear families that need no Jacobian arithmetic)

The relations come from one-parameter families of functions that vanish at
prescribed points. We call these points "anchors". A family is a pencil
`a_lambda = c(x) + lambda * Pi(x)`, with `lambda in F_p`.

| arm | model | form of `a` | affine anchors `j` | free coefficients | residual degree `R` | divisor at infinity |
|---|---|---|---|---|---|---|
| O | odd, `deg f = 2g+1` | `deg a <= g` | `g` | `g+1` | `g+1` | `-(2g+1)(inf)` |
| E_low | even | `deg a <= g` | `g` | `g+1` | `g+2` | `-(g+1)inf+ - (g+1)inf-` |
| E_0 (`e = 0`) | even | `x^(g+1) + b`, `deg b <= g` | `g` | `g+1` | `g+1` | `-g inf+ - (g+1) inf-` |
| E_1a (`e = g-1`) | even | `t_{g-1}(x) + b`, `deg b <= 1` | `1` | `2` | `g+1` | `-1 inf+ - (g+1) inf-` |
| E_g (`e = g`) | even | `t_g(x) + lambda` | `0` | `1` | `g+1` | `0 inf+ - (g+1) inf-` |

Here `t_e(x)` is the "top part": `x^(g+1) + s_g x^g + ... + s_{g+1-e} x^(g+1-e)`,
that is, the top `e + 1` coefficients of `s`. `b` has `g - e + 1` free coefficients,
and `j = g - e` anchors leave a pencil, so `R = d - j = (2g+1-e) - (g-e) = g + 1` for every `e`.
In general, `R = d - j` and `j = (free coefficients) - 1`.

Point-to-lambda map. Take a pencil with anchors `A_i = (x_i, y_i)`. Let
`Pi = prod (x - x_i)` (with `Pi = 1` when `j = 0`), and let
`b0` be the interpolant of degree `<= j - 1` with `b0(x_i) = y_i - t(x_i)`. Then
`c = t + b0`. An affine point `(x0, y0)` whose abscissa is not an anchor abscissa
lies on exactly one member:

    lambda(x0, y0) = (y0 - c(x0)) / Pi(x0) .

Every member's residual divisor is a fibre of the rational function
`phi = (y - c(x)) / Pi(x)`, which has degree `R`. The fibres are disjoint, so
no lambda bucket holds more than `R` points. A member is a full relation
exactly when its bucket holds `R` points.

For E_g, `phi = y - t_g(x)` has a single pole, of order `g + 1`, at `inf-`. It is a
degree-(g+1) function that needs no anchors and no division. Its only cost is
`y0 - c(x0)`, which is additions, and it gives about `p/(g+1)!` relations in total.
This is Riemann-Roch: `l((g+1) Q) = 2` at any non-Weierstrass point `Q`, and
`inf-` is one.

For E_1a (`Pi = x - x_1`, `b0 = y_1 - t_{g-1}(x_1)`),
`lambda = (psi(P) - psi(A)) / (x(P) - x(A))` with `psi = y - t_{g-1}(x)`.
This is the slope of the line through the images of `A` and `P` in the
`(x, psi)`-plane. With a table of
inverses in `F_p` this costs one multiplication per point.

## 4. Relations as rows

Base: the abscissae `x` with `f(x)` a non-zero square. The canonical lift is
`Q_x = (x, y_x)` with `1 <= y_x <= (p-1)/2`, and `eps(y) = +1` on the canonical
lift and `-1` on the other.

Let `delta = [inf+ - inf-]` in `J(F_p)`. It is never zero, since otherwise a
function of degree 1 would exist. For an affine `Q`, `[Q - inf-] + [iQ - inf-] = [inf+ - inf-]`.
Work modulo an odd prime `r` (the subgroup order in the DLP). The
sign-symmetric normalization is then

    l(Q) = log [Q - inf-] - delta/2 ,  so  l(iQ) = -l(Q) .

A case (B) relation with cancellation depth `e` reads `sum_k [P_k - inf-] = (g - e) delta`
over its `d = 2g+1-e` affine points. In `l`-coordinates this becomes

    sum_k eps_k l(Q_{x_k}) + ((e + 1)/2) * delta = 0 .

A case (A) relation is `sum_k [P_k - inf-] = (g+1) delta` over `2g + 2` points.
In `l`-coordinates it is `sum_k eps_k l(Q_{x_k}) = 0`, with no `delta` column.

Corrections to the brief:

1. `inf+` and `inf-` do not need two factor-base elements. Only one class enters,
   `delta`, as one extra column. Its coefficient is `(e+1)/2`. It is zero for the
   brief's case `deg a <= g`.
2. The brief's case `deg a <= g` with `g` anchors leaves a residual of degree
   `g + 2`, not `g + 1`. Its split probability is `1/(g+2)!`, so it is `g + 2` times
   costlier per relation than the odd-degree sieve. This is not a model effect.
   It is the wrong linear system.
3. Monic `a` of degree `g + 1` (case B) with leading cancellation only (`e = 0`)
   is the exact analogue of the odd-degree sieve: `g` anchors, residual `g + 1`,
   row weight `2g + 1` plus the `delta` column. Each further cancellation trades
   one affine anchor for one condition at `inf+`. `e = g - 1` needs a single
   anchor, and its rows have `g + 2` affine entries instead of `2g + 1`.
4. "Needs no Jacobian arithmetic" is true of the relation phase only. A DLP on
   an even model also has to express `G` and `T` over the base (descent). That
   needs divisor arithmetic on the even model (balanced divisors,
   Galbraith-Harrison-Mireles) or a move to an odd model. The second exists only
   when `f` has a rational root. L05 does not measure descent.
5. Nothing here is special to even models as such. The linear system
   `|(g+1) inf- + (g - e) inf+|` exists on any model of the curve. On an odd model
   of the same curve it is a system with poles at the images `W+-` of `inf+-`. So a
   cost difference between E_1a and O is a difference between linear systems,
   not evidence that even models are intrinsically cheaper.

## 5. Odd model of the same curve (reference and paired baseline)

Suppose `f(alpha) = 0` with `alpha in F_p`. Let `c_i` be the Taylor coefficients
of `f` at `alpha`, so `f = sum_i c_i (x - alpha)^i`, `c_0 = 0`,
`c_1 = f'(alpha) != 0` and `c_{2g+2} = 1`. With `X = 1/(x - alpha)`, `X' = c_1 X`
and `Y' = c_1^g y X^(g+1)`:

    Y'^2 = F(X') = sum_{i=1}^{2g+2} c_i c_1^(i-2) X'^(2g+2-i) ,

which is monic of degree `2g + 1`. The point map is:

- affine `(x, y)` with `x != alpha` goes to `(c_1/(x - alpha), c_1^g y / (x - alpha)^(g+1))`;
- `(alpha, 0)` goes to the point at infinity of the odd model;
- `inf+` goes to `W+ = (0, c_1^g)` and `inf-` goes to `W- = (0, -c_1^g)`.

The map is an isomorphism of curves, so `J` is the same group. Under it, a
degree-0 divisor `D` on the even model is principal if and only if
`sum_Q n_Q [Q' - inf_odd]` is the identity in Cantor arithmetic on `F`. This
check is independent of the infinity bookkeeping of section 2.

Base correspondence: an even-base abscissa `x != alpha` corresponds to an
odd-base abscissa `X' != 0`. The abscissa `X' = 0` is in the odd base, because
`F(0) = c_1^(2g)` is a non-zero square. So `n_odd = n_even + 1`, which equals the
column count of the even arms (`n_even` plus `delta`).

## 6. Independent per-relation checks

- V1 (affine part): `f - a^2 = lc(N) * prod_k (x - x_k)` over all `d` affine
  points, `y_k = a(x_k)`, and `#points + ip + im = 0`.
- V_inf (infinity part, model-free): `ord_{inf_sigma}(y - a) = ord_u(sigma S(u) - A(u)) - (g+1)`
  with `A(u) = u^(g+1) a(1/u)`, computed to precision `2g + 3`. That precision is
  enough because `S^2 - A^2 = u^(2g+2) N(1/u)` is a non-zero polynomial of degree
  at most `2g + 2`.
- V2 (group law, root curves only): the odd-model Cantor sum of the mapped
  divisor is the identity.
- Mutations that V2 must reject: m1 shifts the infinity part by `+1 inf+ - 1 inf-`,
  which changes the class by `delta != 0`. m2 replaces one residual point `P` by
  `iP`, which changes the class by `[iP - P] != 0` because `P != iP`.

Hand values (scratch computation, used as unit-test fixtures in SPEC section 12):
take `f = random_f_even(101, 2, 1) = [75, 29, 79, 35, 2, 13, 1]` (coefficients
lowest degree first). Then:

- `s = [41, 43, 57, 1]` and `f - s^2 = [10, 38, 20]`;
- `S(u) = 1 + 57u + 43u^2 + 41u^3 + 10u^4 + 55u^5 + 76u^6 + ...`;
- `ord_{inf+-}(y - a)` is `(0, -3)` for `a = [3, 43, 57, 1]`, `(1, -3)` for `a = s`,
  `(-3, -3)` for `a = [0, 0, 1]`, `(-2, -3)` for `a = [5, 0, 0, 1]` and
  `(-1, -3)` for `a = [7, 9, 57, 1]`.
