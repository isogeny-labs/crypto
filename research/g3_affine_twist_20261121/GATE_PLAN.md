# Gate plan: reconstruct the affine `(2,2,2)` plane quartic

Status: plan and literature map. No reconstruction is claimed. Every mathematical
statement below is from Milio (HAL `hal-01589683`, 2019) unless marked otherwise, and is
`[pcc:unverified]` here.

## Inputs

- One p ≈ 47 prime-order genus-3 hyperelliptic curve `C: y² = f(x)` from
  `data/main.jsonl` with an affine kernel whose `L_subsets` codomain is a plane-quartic
  Jacobian over F_p.
- The 18 codomain theta-pair products from `fpeval.eval_formula` (the `formula` entries
  `{a, b1, b2}`), each equal to `theta_B[(a,b1)] * theta_B[(a,b2)]`.
- The `chi_18` square class from the same run (`val`) to fix the twist branch.

## Index map

Milio uses Dupont's notation

```text
theta[b0 + 2 b1 + 4 b2 + 8 a0 + 16 a1 + 32 a2] := theta[(a,b)](0, Omega)
```

with `a = (a0,a1,a2)`, `b = (b0,b1,b2)`. Apply this to `thetalib.thetas` (which returns
`theta[(a,b)]`) to get the 64 indices used below.

## Aronhold coefficients (Milio, Proposition 13)

```text
alpha11 = t12 t5  / (t33 t40)      alpha12 = t21 t28 / (t56 t49)
alpha21 = t27 t5  / (t54 t40)      alpha22 = t2  t28 / (t47 t49)
alpha31 = -t12 t27 / (t33 t54)     alpha32 = t2  t21 / (t47 t56)
alpha13 = t16 t14 / (t42 t35)
alpha23 = t16 t7  / (t61 t35)
alpha33 = t7  t14 / (t61 t42)
```

## Reconstruction (Milio, Theorem 14, Riemann)

The quartic is

```text
(x1 xi1 + x2 xi2 - x3 xi3)^2 - 4 x1 xi1 x2 xi2 = 0
```

where `xi` solves the two 3×3 systems on p. 23–24 of the PDF (the `lambda` system uses
the `alpha` rows, the `k` system uses `lambda_i alpha_ij`). The PDF text layer garbles the
row order; before implementing, re-read the published page images and pin the exact
matrix. Do not guess it from the text layer.

## The real obstruction

The instrument gives **18 products** of pairs of theta constants, not the 36 individual
constants. Milio's `alpha_ij` are ratios of individual constants, so the products alone
do not determine them. Recovering the constants needs either:

1. the Riemann/duplication relations solved with a consistent set of square roots
   (Milio's Gröbner route: fix 6 square roots, `[Gamma_3(2):Gamma_3(2,4)] = 2^6`), or
2. the algebraic theta functions from the tropes and 2-torsion (Milio Section 5.2),
   which our `chi_18` instrument does not currently build.

The `chi_18` square class fixes the twist branch but not the normalization/signs.

## Next implementation steps

1. Re-read PDF pp. 23–24 as page images and write the two 3×3 systems exactly.
2. On the real configuration used by `derive.py` (where `thB = thetas(tau_B)` is already
   computed), build `alpha_ij` and the quartic numerically. This checks the formulas
   without the finite-field square-root problem.
3. Then solve the same system over F_p from the 18 products plus Riemann relations, and
   compare the reconstructed quartic's theta constants/Igusa invariants to the codomain.
4. Only then attempt Milio's rational fractions for the point map and check
   `[r]Psi(G) = 0`, `Psi(G) != 0`.

Step 2 is the first decisive test: if the real-configuration quartic is smooth and has the
expected invariants, the formula path is right; if not, the row order or index map is
wrong and no finite-field work should start.
