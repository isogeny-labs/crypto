# Explicit plane quartic for an affine-type (2,2,2) codomain of a genus-3 hyperelliptic Jacobian: results (2026-11-23)

Standing: every number here is `[pcc:unverified]`: one session, a shared 4-vCPU host, no
independent replay, no pcc claim filed. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen
before the step-3 data (`frozen.sha256`). Data hashes: `data.sha256`. Literature:
[`LITERATURE.md`](LITERATURE.md).

Labels: **[lit]** literature, **[der]** derived (argument given), **[comp]** computed
(data given), **[conj]** conjecture.

## Answer

**Steps 1, 2 and 3 all succeeded.** For `y^2 = f(x)` over `F_p` (`f` monic irreducible
septic, `#J(F_p)` prime) and either Frobenius-stable affine-type kernel `L`, the
construction below gives an explicit plane quartic `X` over `F_p`. On all **311/311**
non-degenerate kernels of the frozen population (p = 17 to 47, 20 curves per prime, both
kernels), `X` was produced, and its point counts over `F_p, F_{p^2}, F_{p^3}` give
[comp]:

- **159/159** kernels labelled "Jacobian over F_p" by the twist study: `P_X = P_J`
  exactly, so `Jac(X)` is isogenous to `J` and `#Jac(X)(F_p) = #J(F_p)`;
- **152/152** kernels labelled "twist" (the negative control): `P_X = P_J(-T)` exactly,
  i.e. `Jac(X)` is the quadratic twist of `B`; 0 of them match `P_J`;
- **9/9** degenerate kernels (`chi_18 = 0`): an even theta-null of `B` vanishes, so the
  codomain is hyperelliptic, as the twist study inferred.

The preregistered success rule holds (production share 1.00 >= 0.95; no mismatch; 152 >= 20
informative negative controls). Gates: G1 replay (fresh roots, `zeta8`, descent seed,
relabelling `sig` and model parameter `t`) gave the same `N1, N2, N3` on 46/46 kernels;
G2 integral Frobenius polynomial on 311/311. This independently confirms the twist study's
`chi_18` labels on these 311 kernels: the quartic is built without the label, the model's
leading coefficient or `chi_18`.

What this does **not** give: the isogeny on points (the map `J -> Jac(X)`), and anything
at cryptographic size. Runtime is about 1 s per kernel to construct (in `F_{p^56}`, flint)
plus the point counts.

## The construction

Inputs: branch points of the even model `Y^2 = f(t) u prod(u - u_i)`, `u_i = 1/(z^{p^i} - t)`,
`u_8 = 0`, labelled by the twist study's permutation `sig`; work in `K = F_{p^56}`.

1. **Theta-null squares and same-row products of B [lit + comp].** Duplication with
   `tau_B = (tau + S)/2`, `S = 001000`:
   `theta_B[a;b1] theta_B[a;b2] = sum_beta theta[a+beta; b1+b2](tau+S) theta[beta; b1-b2](tau+S)`,
   each `theta(tau+S)` by Thomae (`kappa^(1/4) det(A)^(1/2) eps_c prod rho_ij`). This gives
   all 36 squares and the 70 cross products with equal top row `a` as
   `C * sum_k zeta8^k prod rho_ij^v` with one common factor `C` (`derive2.py`,
   `formulas/prod_001000.json`; rel. dev. 4.2e-12 against direct theta series).
   Answer to "individual squares from pair products": nothing extra is needed, since the
   same identity with `b1 = b2` gives each square directly.
2. **Individual theta constants [comp].** One square root per top row; the rest of the row
   by dividing the same-row products. The 7 relative row signs: 64 of the 128 patterns
   satisfy all 315 Riemann quartic relations `M1 +- M2 +- M3 = 0` (products over cosets
   of 2-dimensional subspaces, `formulas/riemann_rel.json`). Over C, all 64 give the same
   Aronhold matrix as the true thetas (`data/cchain.json`), so any of them works; the other
   64 give a wrong curve. Every row of the run had exactly 64 consistent patterns.
3. **Bitangents from theta-nulls [lit + comp]: Jacobi's derivative formula, not Weber's.**
   The bitangent for odd `n` is `grad theta[n](0)` (L2). For an azygetic odd triple,
   `D(n1,n2,n3) = det(grad theta[n_i](0)) = s pi^3 prod_{e in E5} theta[e]`. The 5-set and
   sign were identified numerically, uniquely, for all 2,016 triples
   (`formulas/jacobi.json`; in every case the 8 characteristics sum to 0). For an Aronhold
   set `(m1..m7)` (Cramer, frame `x1, x2, x3, x1+x2+x3` from `m1..m4`):
   `a_{k1} = D(m_k,m2,m3)/D(m4,m2,m3)`, `a_{k2} = D(m1,m_k,m3)/D(m1,m4,m3)`,
   `a_{k3} = D(m1,m2,m_k)/D(m1,m2,m4)`, for k = 5, 6, 7.
4. **Riemann's quartic [lit].** `xi1+xi2+xi3 + u4 = 0`, `sum_i xi_i/a_ji + k_j u_{5+j} = 0`,
   `X: (x1 xi1 + x2 xi2 - x3 xi3)^2 - 4 x1 x2 xi1 xi2 = 0`.
5. **Frobenius action on labels [der + comp].** A Galois conjugate of a theta-null vector is
   a theta-null vector of the same ppav with other labels. So `(Theta^p)(c) = lambda mu4 Theta(g c)`
   with `g` an affine symplectic map of `F_2^6`. `g` was recovered by matching fourth powers
   of ratios, and was unique on every row. Then pick an Aronhold set with `g(A) = A`. On all
   311 rows exactly one of the 288 sets is stable, and `g` acts on it as a 7-cycle.
6. **Descent [lit + der].** `sigma(F)(H x) = lambda F(x)`, where `H` is the frame change to
   the cyclically re-ordered Aronhold system. This was checked exactly on every row. The
   cocycle norm `H^{-1} sigma(H^{-1}) ... = c I` has `c` in `F_p`. After rescaling `H` by
   `mu` with `N(mu) = c`, Hilbert 90 gives `P = sum_i H_i sigma^i(R)`, and `G(y) = F(P y)`
   has `F_p` coefficients (asserted). `Aut(X)` is generically trivial, so the descended
   curve is unique and its Jacobian is `B` or `B^tw`.

Example [comp] (p = 47, `f = x^7 + 16x^6 + 38x^5 + 29x^4 + 5x^3 + 21x^2 + 20x + 33`,
kernel 0, label Jacobian, `#J(F_47) = 90523` (prime), `P_J = [103823, -15463, 2491, -375, 53, -7, 1]`):

```
X: z^4 + 32 y z^3 + 21 y^2 z^2 + 10 y^3 z + 8 y^4 + 42 x z^3 + 24 x y z^2 + 22 x y^2 z
   + 25 x y^3 + 6 x^2 z^2 + 14 x^2 y z + 5 x^2 y^2 + 31 x^3 z + 41 x^3 y + 42 x^4 = 0   over F_47
N1, N2, N3 = 41, 2267, 103469  ->  P_X = P_J
```

## Step 2 over C [comp]

`tables.py` and `cchain.py`, with period matrices from the frozen `theta_arm.period_matrix`:

- On 5 period matrices (seeds 1 to 4, kernels 001000, 110111, 011101, 101110), for 3
  Aronhold sets each: Riemann's quartic built from 7 gradient lines has all 28 gradient
  lines as bitangents. Max defect is 1.1e-4 (double roots are found to sqrt(eps)); the
  minimum for random lines is 0.034. The Jacobi-formula Aronhold matrix equals the
  gradient one to 1e-12. Jacobi table max rel. dev. 1.3e-12, Riemann relations max
  residual 2.1e-15 (`data/tables_check.json`).
- The full algebraic chain from the `rho` (true branch) reproduces the gradient Aronhold
  matrix to 6.0e-14 for all 64 sign patterns. With random fourth-root choices (another
  level structure), the 10 non-Aronhold bitangents the chain predicts by Jacobi's formula
  are bitangents of the chain's quartic (5 trials, defect <= 2.3e-5).
- No independent invariant check (Dixmier-Ohno, or a period matrix of `X`) was done over C.
  Equality of the Jacobian is shown by the canonical-model argument (L2, L4) plus the
  `F_p` point counts.

## Limits and caveats

- The Jacobi table and the Riemann relations are **numerically identified** (unique
  matches, re-checked on 4 independent period matrices), not transcribed from a proof.
  Weber's 2/2 formula was not used, because the papers were unreachable.
- The descent relies on a Frobenius-stable Aronhold set. That always existed here: `g` is a
  7-cycle, because `f` is irreducible. For other Frobenius types (`g` not conjugate into
  `S_7` in `Sp_6(F_2)`), the general route needs bitangents for syzygetic triples, which
  are not implemented.
- Curves with nontrivial `Aut(X)` would make the descent non-unique. None was flagged:
  every row passed the Frobenius frame check and matched.
- Small primes only. `K = F_{p^56}` is a fixed working field, which is fine at these sizes.

## Deviations

- **D1.** The run process was killed externally (OOM on the shared host, during another
  agent's build) after 281/320 rows, at p = 47 curve 1 kernel 1. A resume option was added
  to `run_validation.py`: skip `(p, curve, L)` already written, append. That changes its
  hash from the frozen `55d798f7...` to `2b036997...` (`data.sha256`). Nothing else changed.
  The run was relaunched and completed; no duplicate rows.
- **D2.** `analyze.py` and `cchain.py` were written after the freeze (they are not part of
  the frozen instrument). The figure scale in `analyze.py` was edited before its first output.

## Files

`PROTOCOL.md`, `frozen.sha256`, `cnum.py` (complex thetas, gradients, Riemann quartic),
`derive2.py`, `tables.py`, `cchain.py`, `fpquartic.py` (finite-field construction),
`build.py`, `population.py`, `run_validation.py`, `analyze.py`, `formulas/`
(`prod_001000.json`, `jacobi.json`, `riemann_rel.json`), `data/` (`tables_check.json`,
`cchain.json`, `validation.jsonl` with every quartic `G`, `summary.json`, `run.log`),
`fig_validation.svg`, `dev/` (development scripts, p = 23 dev run), `data.sha256`,
`LITERATURE.md`.
