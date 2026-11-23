# Proposition B, exact count (subfield base, `m = 2`, characteristic 2)

Date 2026-11-11. Directory: `research/first_fall_closure_20261111/propB_exact_count/`.
All numbers below are [pcc:unverified]. No security or cost consequence is claimed.

## Sources checked

- Conductor is not installed in this session; no task was attached.
- The `research-kb` MCP server is not available in this session; no `kb_status` or `search_knowledge`
  query was run. Novelty and "not tried" claims are therefore NOT made here.
- Read directly: `research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md` §0 and §5;
  `research/fall_degree_bounds_20261008/RESULTS.md` ("What the results establish");
  `research/fall_degree_bounds_20261008/lfd_fast.sage` (how E1 builds the descent);
  `research/fall_degree_bounds_20261008/exp/e1_subfield.jsonl` (the 40 subfield draws).

## Setting

`F = GF(2^n)`, `n = 2n'`, `V = GF(2^{n'}) = {x : x^{2^{n'}} = x}`, `Tr` the absolute trace `F -> F_2`,
`Tr_{F/V}(z) = z + z^{2^{n'}}`. Fix `x_3 in F*`, `b in F`. For `X_1, X_2 in V` put `s_1 = X_1 + X_2`,
`s_2 = X_1 X_2` (both in `V`). Then (char 2)
`S_3(X_1,X_2,x_3) = (s_2 + x_3 s_1)^2 + x_3 s_2 + b = phi(s_2) + psi(s_1) + b`
with the `F_2`-linear maps `phi, psi : V -> F`, `phi(s) = s^2 + x_3 s`, `psi(t) = x_3^2 t^2`.

The descended system is `{ lambda(S_3) : lambda in F^* }` (`F^*` = `F_2`-dual of `F`, dimension `n`),
each `lambda(S_3)` a Boolean polynomial in the `2n'` bits `y_{1j}, y_{2j}` of `X_1 = sum y_{1j} v_j`,
`X_2 = sum y_{2j} v_j` (`v_j` an `F_2`-basis of `V`), reduced mod `y^2 = y`. `span_{F_2}(F)` is the image of
`lambda -> lambda(S_3)`; the count does not depend on the chosen `F_2`-basis of `F` or of `V`.

Write `U = im(phi) + im(psi) subset F`, and `A^perp subset F^*` for the annihilator of `A subset F`.

## Theorem (PROVED)

Let `N(x_3, b) = dim_{F_2}( span_{F_2}(F) ∩ R_{<=1} )`, where `R_{<=1}` = polynomials of degree `<= 1`
in the reduced ring. Let `n = 2n'`.

**(1) General formula.** For every `x_3 in F*`, `b in F`:
`N = dim( U + <b> ) - dim im(phi)`.

**(2) Case `x_3 not in V`.** `U = ker(Tr(x_3^{-2} .))`, a hyperplane, `dim im(phi) = n'`, hence
`N = n' - 1 + Tr(b/x_3^2)`.
Moreover the constant `1` lies in `span_{F_2}(F)` iff `Tr(b/x_3^2) = 1`.
(When `Tr(b/x_3^2)=1` the space `span ∩ R_{<=1}` therefore has dimension `n'` and contains `1`.)

**(3) Case `x_3 in V` (the exceptional set; `x_3 != 0`).** `dim U = n - n'`, `dim im(phi) = n' - 1`, and
`N = 1` if `b in V`, `N = 2` if `b not in V`; the constant `1` lies in the span iff `b not in V`.
(Here the descended equations have rank `n'` if `b in V` and `n'+1` otherwise.)

So the claimed count holds exactly iff `x_3 not in V`, and the exceptional set is exactly `{x_3 in V*}`
(`2^{n'} - 1` values), with no condition on `b` for (2). The E1 outlier (n = 16, seed 112005) is
`x_3 in V`, `b not in V`, so `N = 2`, as measured. Contrary to the §5 text ("every coefficient of `S_3`
in the subfield"), `b` is NOT in `V` there (see the cross-check below).

## Proof

**Step 0 (independence lemma).** For `theta in V^*` (an `F_2`-functional on `V`), the function
`(X_1,X_2) -> theta(X_1 X_2) = sum_{j,k} theta(v_j v_k) y_{1j} y_{2k}` is a reduced polynomial (its monomials
`y_{1j} y_{2k}` involve distinct variables) with the coefficient matrix `(theta(v_j v_k))`. If this matrix is 0,
`theta` vanishes on all products `v_j v_k`; since `1 in V`, `v = v * 1 in span{v_j v_k}` for every `v in V`, so
`theta = 0`. Hence `theta -> theta(X_1 X_2)` is injective, and `theta(X_1X_2)` has degree exactly 2 unless
`theta = 0`. (Checked computationally for `n' = 2..6`: the `n'` bilinear forms are independent.)
No dependence on `x_3`.

**Step 1 (quadratic part).** For `lambda in F^*`,
`lambda(S_3) = (lambda∘phi)(s_2) + lambda(psi(s_1)) + lambda(b)`,
where `lambda∘phi in V^*`. The last two terms are affine in the bits of `s_1`, which are the linear forms
`y_{1j} + y_{2j}`: degree `<= 1`. By Step 0, `lambda(S_3)` has degree `<= 1` iff `lambda∘phi = 0`, i.e. iff
`lambda in W := (im phi)^perp`. For `lambda in W`, `lambda(S_3) = lambda(psi(s_1)) + lambda(b)` as polynomials.

**Step 2 (which `lambda` give the zero polynomial).** `span ∩ R_{<=1}` is the image of `W` under
`lambda -> lambda(S_3)`. For `lambda in W` the polynomial is `ell_lambda(s_1) + lambda(b)` with
`ell_lambda = lambda∘psi in V^*`. The bits of `s_1` are independent linear forms (`X_1` free, `X_2 = X_1 + s_1`),
so the polynomial is zero iff `lambda∘psi = 0` and `lambda(b) = 0`, i.e. `lambda in U^perp ∩ b^perp`.
(Note `W ∩ (im psi)^perp = U^perp`.) The kernel is therefore `U^perp ∩ b^perp = (U + <b>)^perp`, and
`N = dim W - dim (U+<b>)^perp = (n - dim im phi) - (n - dim(U+<b>)) = dim(U+<b>) - dim im(phi)`. This is (1). ∎(1)

**Step 3 (dimension of `im phi`).** `ker phi = {s in V : s^2 = x_3 s} = {0, x_3} ∩ V`. So `dim im phi = n'` if
`x_3 not in V`, and `n' - 1` if `x_3 in V`.

**Step 4 (computing `U^perp`).** Write `lambda = Tr(c .)`, `c in F` (unique). Use `Tr(a) = Tr(a^2)` and, for
`t in V`, `Tr(z t) = Tr_V( t Tr_{F/V}(z) )` together with non-degeneracy of `Tr_V` on `V`, so
`Tr(z t) = 0 for all t in V  <=>  Tr_{F/V}(z) = 0  <=>  z in V`  (`z^{2^{n'}} = z`).

- `lambda∘psi = 0`: `Tr(c x_3^2 t^2) = Tr(sqrt(c) x_3 t)` (apply `Tr(a)=Tr(a^2)`), zero for all `t in V` iff
  `x_3 sqrt(c) in V`. Put `w := c x_3^2`; then `sqrt(c) x_3 = sqrt(w)`, and `sqrt(w) in V <=> w in V`
  (V is closed under Frobenius and its inverse). So `c = w / x_3^2`, `w in V`.
- `lambda∘phi = 0`: `Tr(c s^2 + c x_3 s) = Tr(sqrt(c) s) + Tr(c x_3 s)`, zero for all `s in V` iff
  `sqrt(c) + c x_3 in V`. With `c = w/x_3^2`: `sqrt(c) = sqrt(w)/x_3`, `c x_3 = w/x_3`, so the condition is
  `e / x_3 in V`, `e := w + sqrt(w) in V`.

Hence `U^perp = { Tr(w x_3^{-2} .) : w in V, (w + sqrt w)/x_3 in V }`.

*Case `x_3 not in V`.* If `e != 0`, `e/x_3 in V` would give `x_3 = e / (e/x_3) in V`, a contradiction. So
`e = 0`, i.e. `w = sqrt(w)`, `w in F_2`. Thus `U^perp = <lambda_0>`, `lambda_0 = Tr(x_3^{-2} .)`
(this is the Kosters–Yeo functional; it is nonzero), so `dim U = n - 1`, and `U = ker lambda_0`.
Then `U + <b> = F` iff `b not in U` iff `lambda_0(b) = Tr(b/x_3^2) = 1`; otherwise `U + <b> = U`.
By (1) and Step 3, `N = (n - 1 + Tr(b/x_3^2)) - n' = n' - 1 + Tr(b/x_3^2)`. For the constant: `1 in span`
iff some `lambda in W` has polynomial `1`, i.e. `lambda∘psi = 0` and `lambda(b) = 1`, i.e. `lambda in U^perp`,
`lambda(b) = 1`, i.e. `lambda = lambda_0` and `Tr(b/x_3^2) = 1`. ∎(2)

*Case `x_3 in V`.* The condition `e/x_3 in V` is automatic, so `U^perp = {Tr(w x_3^{-2} .) : w in V}` has
dimension `n'` (the map `w -> Tr(w x_3^{-2} .)` is injective), i.e. `dim U = n - n'`. For `b`: `lambda(b) =
Tr(w b/x_3^2) = Tr_V(w Tr_{F/V}(b/x_3^2))`, which vanishes for all `w in V` iff `Tr_{F/V}(b/x_3^2) = 0` iff
`b/x_3^2 in V` iff `b in V`. So `b in U` iff `b in V`, and `dim(U+<b>) = n - n' + [b not in V]`. By (1) and Step 3,
`N = n - n' + [b notin V] - (n' - 1) = 1 + [b notin V]` (using `n = 2n'`). The constant `1` is in the span iff
some `lambda in U^perp` has `lambda(b) = 1`, iff `b not in V`. ∎(3)

**Consistency with Prop. A / the E1 table.** `Tr_{F/V}(x)` kills `V`, so `Tr|_V = 0` (as `n = 2n'`), which is
why `lambda_0` kills `im phi` (`Tr((s/x_3)^2 + s/x_3) = 0`) and `im psi` (`Tr(t^2) = Tr(t) = 0`). The proof
shows these are the ONLY functionals killing both images when `x_3 not in V`: the Kosters–Yeo relation is the only
dependency among the `n` descended equations' quadratic-free combinations. (The rank of the `n` equations is
`n - dim(U^perp ∩ b^perp)`, from Step 2 together with `lambda(S_3)=0` iff `lambda in (U+<b>)^perp` for `lambda in W`.)

## Status labels

- PROVED: the Theorem (1), (2), (3) and Step 0–4 above, by the argument written out. No step is a sketch.
  The proof uses only: `Tr(a) = Tr(a^2)`, `F_2`-non-degeneracy of `Tr_V` on `V`, `Tr_V(Tr_{F/V}) = Tr`,
  and `V*V = V`.
- MEASURED: the agreement table below (computer arithmetic with explicit irreducible polynomials).
  This tests the statement; it is not part of the proof.
- CONJECTURE: none left for this proposition. (P3 of RESULTS.md, that `span ∩ R_{<=1}` is exactly `<L>` for a
  random base, is a different statement and is not addressed. For `x_3 not in V` the theorem says nothing about
  non-subfield bases.)
- The generalisation to `[F : V] = d > 2` is not claimed. Step 4 would need `Tr_{F/V}` conditions of higher
  degree and the "trace vanishes on V" argument no longer holds. Not examined.

## Computational check (MEASURED)

`propB_check.py` (pure Python 3, no Sage). `gf2n.py` implements GF(2^n) with the explicit moduli below, each
verified irreducible by Rabin's test at construction. `V` is computed as the fixed set of the `n'`-th Frobenius
power; its size `2^{n'}` is asserted, additivity and multiplicative closure are asserted on a 40x40 sample of
elements, and an `F_2`-basis of dimension `n'` is extracted. The `n` Boolean polynomials are built literally from
`S_3 = (x1x2 + x1x3 + x2x3)^2 + x1x2x3 + b` (NOT from the `s_1, s_2` rewriting) by symbolic multilinear
arithmetic (`F`-valued coefficients, monomials as bitmasks, `x^2 = x`), then `c_i` = bit `i` of each coefficient.
`N = rank(all rows) - rank(rows restricted to monomials of degree >= 2)`; `1 in span` is a rank test.
"Predicted" is the Theorem. Raw output: `propB_check_results.json`.

| n' | n | modulus | x3 checked | b checked | (x3,b) pairs | count agrees | const-in-span agrees | disagreements |
|--|--|--|--|--|--:|--:|--:|--:|
| 2 | 4 | z^4 + z + 1 | all F* | all F | 240 | 240 | 240 | 0 |
| 3 | 6 | z^6 + z + 1 | all F* | all F | 4032 | 4032 | 4032 | 0 |
| 4 | 8 | z^8 + z^4 + z^3 + z + 1 | all F* | all F | 65280 | 65280 | 65280 | 0 |
| 5 | 10 | z^10 + z^3 + 1 | all F* | sample | 12261 | 12261 | 12261 | 0 |
| 6 | 12 | z^12 + z^6 + z^4 + z + 1 | all F* | sample | 40939 | 40939 | 40939 | 0 |

Per-class measured counts (class = x3 in V?, b in V?, Tr(b/x3^2)); value: {count: number of pairs}:

- n=4: x3inV_binV_Tr0: {'1': 12}; x3inV_bnotV_Tr0: {'2': 12}; x3inV_bnotV_Tr1: {'2': 24}; x3notV_binV_Tr0: {'1': 24}; x3notV_binV_Tr1: {'2': 24}; x3notV_bnotV_Tr0: {'1': 72}; x3notV_bnotV_Tr1: {'2': 72}
- n=6: x3inV_binV_Tr0: {'1': 56}; x3inV_bnotV_Tr0: {'2': 168}; x3inV_bnotV_Tr1: {'2': 224}; x3notV_binV_Tr0: {'2': 224}; x3notV_binV_Tr1: {'3': 224}; x3notV_bnotV_Tr0: {'2': 1568}; x3notV_bnotV_Tr1: {'3': 1568}
- n=8: x3inV_binV_Tr0: {'1': 240}; x3inV_bnotV_Tr0: {'2': 1680}; x3inV_bnotV_Tr1: {'2': 1920}; x3notV_binV_Tr0: {'3': 1920}; x3notV_binV_Tr1: {'4': 1920}; x3notV_bnotV_Tr0: {'3': 28800}; x3notV_bnotV_Tr1: {'4': 28800}
- n=10: x3inV_binV_Tr0: {'1': 127}; x3inV_bnotV_Tr0: {'2': 118}; x3inV_bnotV_Tr1: {'2': 127}; x3notV_binV_Tr0: {'4': 2465}; x3notV_binV_Tr1: {'5': 1660}; x3notV_bnotV_Tr0: {'4': 3983}; x3notV_bnotV_Tr1: {'5': 3781}
- n=12: x3inV_binV_Tr0: {'1': 253}; x3inV_bnotV_Tr0: {'2': 190}; x3inV_bnotV_Tr1: {'2': 187}; x3notV_binV_Tr0: {'5': 10083}; x3notV_binV_Tr1: {'6': 6207}; x3notV_bnotV_Tr0: {'5': 12046}; x3notV_bnotV_Tr1: {'6': 11973}

Sampling for n' = 5, 6: x3 ranged over ALL of F*; for each x3, b was sampled (b = 0, b = 1, two random elements of V*, 6 or 4 further random elements, plus one forced element of each class Tr(b/x3^2) = 0, 1), seed 20261111. n' = 2, 3, 4: all (x3, b) with x3 != 0.

Both the dimension and the constant-in-span prediction agree on every pair. Disagreement lists in the JSON are empty.

## Cross-check against the E1 draws (MEASURED)

`crosscheck_e1.py` recomputes `N` from the 40 records of `exp/e1_subfield.jsonl` (they store `x3`, `b` as
polynomials in `z`; they do NOT store the modulus). ASSUMPTION: Sage's `modulus='minimal_weight'` is the
irreducible of least weight with ties broken by the smallest integer; this is not verified here. The
moduli used were `z^8+z^4+z^3+z+1`, `z^10+z^3+1`, `z^12+z^3+1`, `z^14+z^5+1`, `z^16+z^5+z^3+z+1`.
`N` depends on `V`, which is canonical, so only the identification of `x_3, b` as field elements depends on the
assumption. Result (`crosscheck_e1_results.json`): for all 40 draws, recorded `deg_le1_dim` = recomputed `N`
= the Theorem's value. For seed 112005 (n = 16): `x_3 in V`, `b not in V`, recomputed `N = 2`, constant 1 in the
span, descended rank 9 = n'+1, matching the recorded `deg_le1 = [x1+x6+x7+x9+x14+x15, 1]`. In the other 39
draws `x_3 not in V`. If the assumed modulus is wrong, the match of all 40 recorded values with an independent
formula would be a coincidence; of all weight-5 irreducible degree-16 polynomials (scanned in a scratch run, not saved) only the assumed one
puts `x_3` of that draw in `V`.
The `§5` sentence "the outlier has every coefficient of `S_3` in the subfield" is, under that assumption,
inaccurate: `b` is not in `V`. The descent rank `n'+1` rather than `n'` reflects this.

## Files

`NOTE.md`, `gf2n.py`, `propB_check.py`, `propB_check_results.json`, `crosscheck_e1.py`,
`crosscheck_e1_results.json`.
