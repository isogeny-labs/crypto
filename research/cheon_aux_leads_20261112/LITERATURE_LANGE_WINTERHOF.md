# Lange–Winterhof interpolation bounds: what can be stated exactly

For `LEAD-EXO-lange-winterhof-interpolation`, whose next step is "read the
paper and record the exact theorem statement and scope". Desk work only;
nothing was run. Every statement below is `[pcc:unverified]` (no pcc claim id
exists for any of it).

## 1. Sources and access (2026-11-12)

| source | status | where |
|:--|:--|:--|
| T. Lange, A. Winterhof, "Polynomial interpolation of the elliptic curve and XTR discrete logarithm", COCOON 2002, LNCS 2387, pp. 137–143, DOI `10.1007/3-540-45655-4_16` | **not read**: Springer redirects to its login page; no author copy is listed on Lange's publication page; the TU/e portal has no file | the bibliographic data was confirmed on Lange's publication page, DBLP and the Springer DOI |
| T. Lange, A. Winterhof, "Interpolation of the elliptic-curve Diffie-Hellman mapping", AAECC-15 (2003), LNCS 2643, pp. 51–60 | **not read**: the TU/e portal gives metadata only | TU/e research portal |
| E. Kiltz, A. Winterhof, "On the interpolation of bivariate polynomials related to the Diffie-Hellman mapping", *Bull. Austral. Math. Soc.* 69 (2004) 305–315, DOI `10.1017/S0004972700036042` | **read in full** (11 pages, open on Cambridge Core; PDF sha256 `e78752637f5505499d44ac9bfe5d4c73421759fae1865fed84d6469d8e6ffad9`, not committed) | its §5 states that its Theorems 5 and 6 "extend and improve [8, Theorem 3]", and reference [8] is the Lange–Winterhof COCOON'02 paper |

The previous attempt (`docs/session-logs/2026-11-01.md`, `research/ecdlp_mechanism_falsifiers_20261101/PLAN.md` item 7)
also found the COCOON chapter paywalled. That remains true. **The Lange–Winterhof theorem text itself is still not recorded here.**
What follows is the strongest exact statement available from an open primary source that the authors present as extending it.

## 2. Exact statements (Kiltz–Winterhof 2004, §5, transcribed)

**Setting.** `E/F_q` given by `Y² + h(X)Y = f(X)` with `h` linear and `f` a monic cubic, nonsingular. `P ≠ O` has prime order `l`.
`x_k` is the first coordinate of `kP` for `k = 1, …, l−1`. The bivariate map studied is the elliptic-curve Diffie–Hellman map on
x-coordinates, `F(x_n, x_m) = x_{nm}`.

**Theorem 5.** Let `S ⊆ {1, 2, …, l−1}²` with `|S| = (l−1)² − s`. If `F ∈ F_q[X, Y]` satisfies `F(x_n, x_m) = x_{nm}` for all
`(n, m) ∈ S`, then

    deg(F) ≥ (1/28) · (l − 2 − 2s/(l−1)).

**Theorem 6.** Let `U, V ⊆ {1, 2, …, l−1}` with `|U| ≥ 3` and `v + 1 ∈ V` for some `v ∈ V`. If `F ∈ F_q[X, Y]` satisfies
`F(x_n, x_m) = x_{nm}` for all `(n, m) ∈ U × V`, then

    deg(F) ≥ |U| / 8.

**Proof device (both).** The doubling map `x(2Q) = θ(x)/ψ(x)`, with `deg θ = 4` and `deg ψ ≤ 3`, turns the interpolation identity
into a nonzero polynomial `U(X, Y)` of degree at most `7·deg(F)` with many zeros. Lemma 1 (zeros against total degree) then bounds
`deg(F)`.

The same paper's finite-field results (Theorems 1–4, for `f(γ^x, γ^y) = γ^{xy}` and the `P`-Diffie–Hellman maps `γ^{P(x,y)}`) give
degree and weight bounds linear in the size of the sampling set. Theorem 3 there is a reduction: a low-degree interpolant of `P`-dh on
known points yields the Diffie–Hellman map.

## 3. Scope: what these bounds do and do not say

- **They bound one representation.** A polynomial in the coordinates that reproduces the DH map (and, in Lange–Winterhof's
  title, the discrete logarithm) on most of the group must have degree linear in `l`. So no low-degree *polynomial formula* exists.
- **They say nothing about algorithms.** An algorithm need not evaluate a single interpolating polynomial. Rho, index calculus and
  Cheon all run in time far below `l` while every such polynomial has degree `Ω(l)`. These are lower bounds on a model, not on cost.
- **The constants are weak.** `1/28` and `1/8` come from the doubling-map degrees. They do not bear on `√l` versus anything.
- **Bearing on `LEAD-EXO-invertible-hint-subset`.** That lead asks whether a subset of hints can be inverted cheaply. These theorems
  exclude a low-degree interpolant on a set of density `1 − O(1/l)` (Theorem 5) or on a product set (Theorem 6). They do not exclude
  low degree on sparse or structured subsets. Theorem 6 shows the authors needed a product shape to get any bound on a sparse set.
- **Bearing on the Cheon value-set question (`LEAD-STUB` in this directory).** None directly. Cheon–Kim's question is about the
  value set of a one-variable polynomial over `F_r`, not about interpolating the group law.

## 4. Next step for the lead

Obtain the COCOON'02 chapter (library access or a request to the authors) and transcribe its Theorem 3 here, with its hypotheses.
Until then the lead's next step is **partly done**: the exact neighbouring theorems are recorded, the original is not.
