# How sharp is the Kousidis-Wiemers bound `m^2 - m + 1`? Exact reproduction at m = 3 and what can be proved

Date 2026-11-11. Directory: `research/first_fall_closure_20261111/kw_sharpness/`.
Every number below is `[pcc:unverified]` (no pcc claim id exists for any of it).
No security, cost or "beats rho" claim is made. Labels: **PROVED**, **MEASURED**, **SKETCH**, **CONJECTURE**.
"PROVED (exact computation)" means a finite exact computation over F_2 or GF(2^n) that this directory reproduces.

## 0. Bottom line

1. **Convention matters, and here it decides the answer.** For the Weil descent of `S_4` (m = 3), the Kousidis-Wiemers
   (KW) first fall degree in the top-form-syzygy reading (HPS Def. 2.2 = KW Def. 2.1/2.2; Caminata-Gorla Def. 1.3 agrees
   whenever all descended equations have degree exactly 6) can only take the two values **6 or 7**, and it is 7 **if and
   only if** a field-theoretic rank condition holds: `dim_F2 span{e_p e_q e_r} = dim V` (Theorem A below).
   So "sharpness of `m^2-m+1`" at m = 3 is, in this reading, not a Groebner-basis statement at all. It is the surjectivity
   of an explicit F_2-linear evaluation map. **PROVED** (given KW's Lemma 3.1, which I re-verified symbolically for m = 3).
2. **Measured value reproduced exactly.** 2400 random instances (n = 7..21, n' = 3..7, N = 3n' = 9..21 Boolean variables):
   whenever the top forms are independent (1276 instances, including every cell of KW's Table 1 for m = 3,
   n = 13..21, n' = ceil(n/3)), both the top-form and the operational first fall degree are **7**. Zero exceptions.
   Pure Python 3 + numpy, about one minute on 4 cores, peak 237 MB.
3. **Lower bound for m = 3: partial, with exact scope.**
   - PROVED: `D_ff >= 7` (top-form and operational) for every instance whose top forms are independent; and every
     such instance is certified by an exact rank computation (all 1276 are in the raw files, each with field polynomial,
     curve, point and subspace basis).
   - PROVED: `D_ff = 6` (top-form) for **every** instance with `n > rho(n')` (rho = 10, 49, 170, 475, 1141 for n' = 3..7)
     and for every `V` contained in a proper subfield. So the value 7 is not generic in all regimes: the bound `m^2-m+1`
     is **not** attained there. This is a counterexample to an unqualified reading of "sharp for n' >= m".
   - OBSTRUCTION (no proof): that the rank condition holds for generic parameters in the regime `n <= rho(n')`, in
     particular for all `n' = ceil(n/3)`. It needs uniform control of a degree-9 linear system whose members all have a
     common base locus of multiplicity 3 (section 5). Heuristically it holds with probability `1 - O(2^(n-rho))`.
4. This note does **not** touch the degree of regularity or the last fall degree. KW's question (4.1)
   `D_reg = m^2-m+1+o(1)` is untouched; the first fall degree being 7 is a statement about the top forms of the generators only.

## 1. Sources checked

- Conductor: not installed in this environment (as instructed). No conductor task was attached; I only created files in this directory.
- `research-kb` MCP server: not available (as instructed). Fallback: read repository files and the literature directly.
  No `source_id` exists for any item below; the `repo:path` / arXiv ids are:
  - `crypto:research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md` sections 0, 2.1, 2.2, 2.4
  - `crypto:papers/ecdlp-index-calculus-treatise/sections/falldegree.tex` lines 1-140 and the m = 2 corollary (about 180-200)
  - `crypto:src/cryptanalysis/ffd_harness.rs` header (m = 2, rank-deficit convention; not used for m = 3)
  - `crypto:docs/session-logs/` contains only `README.md` and `TEMPLATE.md` (no dated log to read). I did not write a
    session log because I was told not to edit outside this directory; the caller should log this work.
  - arXiv:1906.05594 (Kousidis-Wiemers), **full text read** (12 pages, `pdftotext`), including Defs 2.1/2.2, Lemma 3.1,
    Theorem 3.2, Remark 3.3, Table 1 and the experimental section.
  - arXiv:2112.05579 (Caminata-Gorla), Def. 1.3 (first fall degree) and Def. 1.5 (last fall degree), read.
  - Not re-fetched: Hodges-Petit-Schlatter (used through KW's statements of their Defs 2.1, 2.2, Cor. 2.4), Petit-Quisquater,
    Kosters-Yeo, Huang-Kosters-Yeo. I rely on KW and on the repo ledger for these.

## 2. Conventions and the conjecture, stated precisely

Field GF(2^n), curve `y^2 + xy = x^3 + a2 x^2 + a6`, `x_{m+1} = c` a nonzero constant, `V = span_F2(nu_1..nu_n') in GF(2^n)`,
`x_i = sum_l nu_l y_il`, Boolean ring `B = F_2[y]/(y^2 - y)` in `N = m n'` variables, graded ring `Gr = F_2[y]/(y^2)`.
`S_{m+1}(x_1..x_m, c) = sum_k z^k f_k`, `s_k = f_k mod field equations`, `V_F = span_F2 (s_0..s_{n-1})`.

- **(TF) top-form first fall degree** (HPS Def. 2.2, used by KW Thm 3.2). Let `d` be the filtration level (`d = m^2-m`).
  `Vbar` = image of `V_F` in the degree-`d` part of `Gr`. If `dim Vbar < dim V_F` then `D_ff = d`; otherwise `D_ff` is the least `j`
  such that `phi_{j-d}: (Gr_{j-d})^r -> Gr_j`, `(g_i) -> sum g_i hbar_i`, has a kernel not in `U` = module generated by Koszul
  syzygies `h_i e_j + h_j e_i` and `h_k e_k` (these generators have coefficient degree `d`, so `U` is zero in coefficient
  degrees below `d`). Caminata-Gorla Def. 1.3 (`min{d : Syz(F^top)_d / Triv(F^top)_d != 0}`, trivial syzygies generated by
  `f_i e_j - f_j e_i` and `f_i^{q-1} e_i`) gives the same number when all `s_k` have degree exactly `d`, because trivial syzygies live in degree `>= 2d`.
  (If some `s_k` has degree `< d`, the CG number can differ; this happened in 14 of 2400 samples, all with dependent top forms.)
- **(OP) operational first fall degree** (HKY/HKYY, the repo's `RESEARCH_FALL_DEGREE_BOUNDS.md` section 0): smallest `c` with
  `V_{F,c} cap R_{<=c-1} != V_{F,c-1}`, computed in `B` as the closure `W_c` of `F cap B_{<=c}` under `g -> y_i g` whenever
  `deg g + 1 <= c`. Here `F` is the **F_2-span** of the `s_k` (so low-degree combinations are already generators at their own degree).
  The reduction-mod-field-equations bridge `pi(V_{F+FE,c}) = W_c` is quoted from the repo ledger, not re-proved here.
- Petit-Quisquater's literal reading (a combination whose degree drops) is not computed here.

**Conjecture as stated by KW** (their section 4, "we believe our first fall degree bound `m^2-m+1` to be sharp for `m >= 3`"):

> (C-TF) For `m >= 3`, `n' >= m`, `c != 0` and generic curve and `V`: `D_ff^{TF}(Weil descent of S_{m+1}) = m^2 - m + 1`.
> (C-OP) The same with (OP).

KW's own measurement (Table 1) is neither of these exactly: it is the first Magma F4 step degree at which a polynomial of lower
degree than the step degree is added, with `n' = ceil(n/m)`, random subspace, random curve, `x_{m+1}` the abscissa of a random point,
averaged over 10 runs; for m = 3: n = 13..21, n' = 5, 5, 5, 6, 6, 6, 7, 7, 7, observed `D_ff = D_reg = 7`.
That quantity is an (OP)-type quantity (the step at degree 7 pairs each `s_j` with the field equation `y_i^2 - y_i`, which is the
multiplication `y_i s_j` modelled below). **Both** (C-TF) and (C-OP) are scoped below by an explicit hypothesis, because (C) as written is false
for `n > rho(n')` and for subfield `V`.

## 3. Mechanism of the upper bound (m = 3 in full detail, general m in brackets)

1. **Shape of S.** `S_{m+1}(x_1..x_m, x_{m+1}) = Res_X(S_m(x_1..x_{m-1}, X), S_3(x_m, x_{m+1}, X))`, degree `2^{m-1}` in each variable.
   Computed here for `S_4` (`semaev.py`): 24 monomials in `F_2[a6, x1..x4]`, degree 4 in every variable, symmetric under all 24
   permutations of the four variables, closed-form resultant equal to the 4x4 Sylvester determinant (independent Leibniz expansion).
   Semantics checked by brute force over GF(2^3), GF(2^4), GF(2^5): every F_q-rational relation `P1+...+Pk = O` gives a zero of `S_k`
   (`verify_semantics.txt`).
2. **Descent turns exponents into Hamming weights.** On Boolean points `x^{2^j} = sum_l nu_l^{2^j} y_l` (Frobenius is additive and `y^2 = y`).
   So `x^e = prod_{j in bits(e)} L_j`, `L_j := sum_l nu_l^{2^j} y_l`, a product of `wt(e)` linear forms, and the monomial `prod x_i^{e_i}`
   descends to degree `<= sum_i wt(e_i)`. This is the Hodges-Petit-Schlatter weight bound.
3. **Which monomial is heaviest.** `e_i <= 2^{m-1}`. For `m >= 3` the weight `wt(e)` is at most `m-1`, attained only at `e = 2^{m-1}-1`
   (`e = 2^{m-1}` has weight 1). So the maximum total weight is `m(m-1)` and it is attained only by `prod x_i^{2^{m-1}-1}`.
   [For `m = 2`, `e in {1,2}` both have weight 1, which is why m = 2 is a different case.]
   KW's Lemma 3.1 shows this monomial occurs in `S_{m+1}` (Sylvester permutations `sigma`, `tau`). **For m = 3 I re-derived it
   symbolically:** the coefficient of `x1^3 x2^3 x3^3` in `S_4` is exactly `kappa = x4` (no `a6` term, no other power of `x4`);
   it is the unique monomial of weight 6. Hence `kappa(c) = c != 0` suffices.
4. **Top form.** `T = c * prod_{i=1}^{3} L_{i,0} L_{i,1}` in `Gr`, where `L_{i,j} = sum_l nu_l^{2^j} y_il`.
   Expanded: `L_{i,0}L_{i,1} = sum_{l<l'} e_{ll'} y_il y_il'` in `Gr`, with `e_{ll'} = nu_l nu_l'^2 + nu_l^2 nu_l' = nu_l nu_l' (nu_l + nu_l')`
   (a 2x2 Moore determinant). I checked that the degree-6 part of the descended system equals `c * prod_i sum_{l<l'} e_{ll'} y_il y_il'`
   in all 2400 samples (`formula_ok`).
5. **Why the next degree falls.** The `F_2`-valued top forms `tau_k` of `s_k` are the coordinates of `T`. Over GF(2^n) the span of
   the `tau_k` equals the span of the `n` Frobenius conjugates `T^{(t)} = c^{2^t} prod_{k=1}^{3} L_{k,t} L_{k,t+1}` (coefficientwise Frobenius,
   indices `t` mod n; the change of basis is a Moore matrix of a basis of GF(2^n), invertible). In `Gr`, `L^2 = 0` for every linear form `L`
   (squares of variables vanish and char 2 kills the cross terms). So every `T^{(t)}` is annihilated by `L_{1,t}, L_{1,t+1}, L_{2,t}, L_{2,t+1},
   L_{3,t}, L_{3,t+1}`: a syzygy with coefficients of **degree 1**, i.e. of degree `d+1 = 7`. KW use only `x_k = L_{k,0}` (three per `T`); the Frobenius shifts
   give six. Since the trivial syzygies `U` start in degree `2d = 12`, none of these is trivial. If `T != 0` and the top forms are independent,
   the first non-injective `phi` is at `j = d+1`. If the top forms are dependent, `D_ff = d` by definition. That is the whole
   upper bound; no property of the curve is used beyond `kappa(c) != 0`.
   [General m: `T = kappa prod_i prod_{j=0}^{m-2} L_{i,j}`, each `T^{(t)}` is killed by the `m(m-1)` forms `L_{k,t..t+m-2}`, giving `D_ff <= m^2-m+1`. Not computed here for m >= 4, since I did not compute `kappa` for `S_5`.]
6. **Operational reading.** In `B`, `y_i^2 = y_i`, so `x_k * x_k^3 = x_k^4` is *linear*: the product of `x_k` with a polynomial whose top form is
   divisible by `x_k`-factors drops far below its nominal degree. This is the degree fall that an F4 run sees at step 7.

## 4. Exact computation (task 2)

Code: `kwgf.py` (GF(2^n), n <= 21, primitive polynomials verified), `semaev.py` (S_3, S_4, resultant, verification),
`verify_semantics.py`, `ffd.py` (descent, top-form analysis, operational closure; Boolean polynomials are Python big integers, one bit per monomial),
`test_descent.py` (descended system equals `S_4` at 200 random Boolean points per instance; degree-6 part equals the closed formula),
`run_table.py` (the experiment), `rho_generic.py`, `structured_V.py`, `full_closure_check.py`, `aggregate.py`, `reproduce.sh` (everything, about 57 s).
Raw output: `raw_*.jsonl` (one JSON object per instance, containing field polynomial, `a2, a6, c`, basis `nu`, all ranks and the operational log),
`table_measured.md` (all cells), `aggregate_out.txt`.

How each convention is computed:
- (TF): `dim V`, `dim Vbar` by F_2 elimination of the degree-6 parts; if equal, the rank of the `N * dim V` rows `y_i * tau_k` in `Gr_7`
  (`Gr` multiplication by `y_i` kills monomials containing `y_i`). Nullity = rows - rank.
- `dim E^3`: computed independently in the field as the F_2-rank of all products `e_p e_q e_r`. It equals `dim Vbar` in every one of the 2400 samples (assertion in `aggregate.py`).
- (OP): for `c = 1,2,...`, a cheap stage-one test (span of `F cap B_{<=c}` and its variable multiples; a detected low-degree element is a real fall because that span is inside `W_c`),
  and only if it fails the full iterated closure `W_c`. For the independent-top-form instances no fall can occur before `c = 7`: `W_c = 0` for `c <= 5`, `W_6 = F`.
  Full closure was also run for three instances (`full_closure_check.txt`): `W_7 = B_{<=7}` entirely (dimension 502 for N = 9; 3302 for N = 12), so these overdetermined
  instances with a random target refute at degree 7; stage-one low dimension (e.g. 91 at n = 13) is a strict lower bound for the full value (2510 at n = 12).

Sizes and timings (4 cores, 15 GB machine; limit 8 GB not approached): `dim B_{<=7}` = 502, 3302, 16384, 63004, 198440 for N = 9, 12, 15, 18, 21;
`C(N,7)` = 36, 792, 6435, 31824, 116280. Largest cell N = 21: rows `21*21 = 441` against 116280 columns, 5.7 s wall, 237 MB peak RSS.
Whole sweep: 1.9 s (N <= 12 small cells) + 3.7 s (N = 15, 18) + 14.2 s (N = 21, two processes) + 11 s for the full-closure check.

### Measured table (MEASURED, `[pcc:unverified]`; random curve, random subspace, c = x of a random point; target `m^2-m+1 = 7`)

KW's Table 1 cells for m = 3 (`n' = ceil(n/3)`):

| n | n' | N | instances | dim V | dim Vbar | D_ff top-form | D_ff operational | nullity in degree 7 | 6n | KW Table 1 |
|--:|--:|--:|--:|--|--|--|--|--:|--:|--|
| 13 | 5 | 15 | 20 | 13 | 13 | **7** (20/20) | **7** (20/20) | 78 | 78 | 7 |
| 14 | 5 | 15 | 20 | 14 | 14 | **7** (20/20) | **7** (20/20) | 84 | 84 | 7 |
| 15 | 5 | 15 | 20 | 15 | 15 | **7** (20/20) | **7** (20/20) | 90 | 90 | 7 |
| 16 | 6 | 18 | 6 | 16 | 16 | **7** (6/6) | **7** (6/6) | 96 | 96 | 7 |
| 17 | 6 | 18 | 6 | 17 | 17 | **7** (6/6) | **7** (6/6) | 102 | 102 | 7 |
| 18 | 6 | 18 | 6 | 18 | 18 | **7** (6/6) | **7** (6/6) | 108 | 108 | 7 |
| 19 | 7 | 21 | 2 | 19 | 19 | **7** (2/2) | **7** (2/2) | 114 | 114 | 7 |
| 20 | 7 | 21 | 2 | 20 | 20 | **7** (2/2) | **7** (2/2) | 120 | 120 | 7 |
| 21 | 7 | 21 | 2 | 21 | 21 | **7** (2/2) | **7** (2/2) | 126 | 126 | 7 |

Smallest feasible cells (n' = 3 is the minimum allowed by `n' >= m`; 330 random instances per n, so frequencies are meaningful):

| n | n' | N | rho(3) | instances | top forms independent | D_ff top-form | D_ff operational (indep. / dep.) | nullity (independent instances) | 6n |
|--:|--:|--:|--:|--:|--|--|--|--|--:|
| 7 | 3 | 9 | 10 | 330 | 330 (100%) | 7:330 | 7 / - | 45 or 48 | 42 |
| 8 | 3 | 9 | 10 | 330 | 277 (84%) | 7:277, 6:53 | 7 / 6:1, 7:52 | 54 | 48 |
| 9 | 3 | 9 | 10 | 330 | 243 (74%) | 7:243, 6:87 | 7 / 7 | 63 | 54 |
| 10 | 3 | 9 | 10 | 330 | 116 (35%) | 7:116, 6:214 | 7 / 6:21, 7:193 | 72 | 60 |
| 11 | 3 | 9 | 10 | 330 | 0 | 6:330 | - / 6:77, 7:253 | - | 66 |
| 12 | 3 | 9 | 10 | 330 | 0 | 6:330 | - / 6:189, 7:141 | - | 72 |
| 13, 14, 16, 20 | 3 | 9 | 10 | 20, 30, 30, 30 | 0 | 6 (all) | 6 (all) | - | - |

Other cells (n' = 4: n = 9..16, 20; n' = 5: n = 10, 12..17; n' = 6: n = 16..20): all top-form independent, all `D_ff = 7` in both readings, nullity exactly `6n`.
Totals over 2400 instances: 1276 independent, all with top-form 7 and operational 7; 1124 dependent, all top-form 6, operational 6 (398) or 7 (726).
For `n' >= 4` the nullity is exactly `6n`, matching the six syzygies per conjugate of section 3; for `n' = 3` it is larger because `9n` rows are
squeezed into the 36-dimensional `Gr_7` (rank at most 18 observed), so the count forces extra syzygies. In all 1276 independent instances the stage-one low dimension
equals `n + nullity` exactly, i.e. there is no Boolean-ring syzygy among `{s_j, y_i s_j}`: every top-form syzygy lifts to a genuine element of degree `<= 6` outside `F`.

Structured subspaces (`structured_V.txt`, MEASURED): `V` inside a proper subfield: `dim Vbar = k < n` (k = 4,6,6,7,5,6 for n = 12,12,12,14,15,18), top-form `D_ff = 6`;
the operational value there is 5 or 6. `V = span{1, x, .., x^{n'-1}}` (Semaev's low-degree polynomial basis, n = 13..21, n' = 5..7): `dim Vbar = n`, top-form 7, operational 7 in all nine cells.

## 5. Lower bound for m = 3 (task 3)

### Theorem A (PROVED, given KW Lemma 3.1 = the computed `kappa = x4`)
Let `m = 3`, `n' >= 3`, `nu_1..nu_n'` F_2-independent, `c != 0`, `n >= 2`, and write `E = span_F2{e_{ll'}}`, `E^3 = span_F2{e_p e_q e_r}`.
(a) `dim Vbar = dim E^3` (the F_2-dimension of the span of the coefficients of `T`; multiplication by `c` is an F_2-isomorphism).
(b) `D_ff^{TF} in {6, 7}`; it equals 6 iff `dim E^3 < dim V_F`, and 7 iff `dim E^3 = dim V_F`.
(c) If `dim E^3 = dim V_F = n`, the kernel of `phi_1: (Gr_1)^n -> Gr_7` has dimension `>= 6n` and no kernel element lies in `U`.
(d) If moreover every `s_k` has degree 6, `D_ff^{CG} = 7` too.

Proof. (a) The matrix of top coefficients has columns `c e_p e_q e_r` (one per choice of one pair in each block; the three blocks are disjoint, so the 27 monomials
are distinct); its row rank over F_2 equals the F_2-dimension of the span of its columns. (b) By section 3, `V_F` lives in filtration level 6, so if `dim Vbar < dim V_F`
then `D_ff = 6`; otherwise injectivity of `phi_0` means no fall at 6, trivial syzygies vanish below degree 12, and (c) gives a fall at 7.
(c) Basis-independence of the kernel dimension under extension of scalars (HPS Cor. 2.4) and the 6n independent syzygies of section 3 (for fixed `i` the forms
`L_{k,t}, L_{k,t+1}` for `k = 1,2,3` are independent: different blocks, and within a block the vectors `(nu_l^{2^t})_l`, `(nu_l^{2^{t+1}})_l` are proportional only if all `nu_l` are equal);
syzygies for different conjugates `t` sit in different coordinates. (d) By definition of the CG degree (`deg b + deg f`, all `deg f = 6`) and `Triv_{<12} = 0`. QED

Remark (observations that make `e` explicit). `e_{ab} = (v+w)^3 + v^3 + w^3` for `v = nu_a`, `w = nu_b`: `E` is the span of the polarisation of the Gold cube map `x -> x^3` on `V`,
equivalently the image of the alternating F_2-bilinear map `Lambda^2 V -> GF(2^n)`, `v ^ w -> v^2 w + v w^2`. The statement is basis-free.

### Corollary B (PROVED; the numbers `rho` are exact computations, `rho_generic.txt`)
(i) `dim E^3 <= rho(n')`, where `rho(n')` = F_2-dimension of the span of the polynomials `e_p e_q e_r` in `F_2[nu_1..nu_n']` (evaluation at `nu` is linear):
`rho(3..7) = 10, 49, 170, 475, 1141` (against `C(C(n',2)+2, 3) = 10, 56, 220, 680, 1771` products). Hence **`D_ff^{TF} = 6` for every `nu`, `c`, curve whenever `n > rho(n')`** and `dim V_F = n`.
For `n' = 3` this is every `n >= 11`; measured: all 330 + 330 + 20 + 30 + 30 + 30 instances at `n = 11, 12, 13, 14, 16, 20` have top-form 6.
(ii) If `V` lies in a subfield `K = GF(2^k)`, `k | n`, `k < n`, then `E^3 subset K`, `dim Vbar <= k < n`, so `D_ff^{TF} = 6`.
(iii) Union-bound converse (PROVED): if `n >= rho(n') + 4`, a random `nu` has `dim E^3 = rho(n')` exactly (each of the `2^rho - 1` nonzero F_2-combinations is a nonzero polynomial of degree 9,
vanishing on at most a fraction `9/2^n` of `GF(2^n)^{n'}` by Schwartz-Zippel, and `2^rho * 9 / 2^n < 1`), so for such `n` the value 6 holds for a positive fraction, indeed almost all `nu`.
Consequently (C-TF) is false in the form "for all `n' >= m`, generic `V`": it needs the extra hypothesis `n <= rho(n')` (for m = 3).

### Proposition C (PROVED, operational reading)
If `dim E^3 = dim V_F = n` and every `s_k` has degree 6, then `D_ff^{OP} >= 7`: for `c <= 5`, `F cap B_{<=c} = 0`, so `W_c = 0`; for `c = 6`, `W_6 = F` (nothing may be multiplied) and `W_6 cap B_{<=5} = 0 = W_5`.
`D_ff^{OP} = 7` iff `dim(W_7 cap B_{<=6}) > n`. By the lifted syzygies of section 3 and 4 this is **MEASURED** in all 1276 independent instances (low dimension `n + 6n` for `n' >= 4`), and
SKETCH in general: it holds iff not every lifted syzygy `L * S^{(t)}` (an element of `B` of degree `<= 6`) lies in `F`; I have no proof that none does for all parameters.
The operational and top-form values differ in the dependent regime (e.g. `n = 11, n' = 3`: top-form 6, operational 7, because the low-degree combinations of `F` are present as generators from degree 5 or 6 on and are not counted as a fall at degree 6).

### What remains open, and why the standard tools stop here (obstruction)
The missing statement is **(G): for the parameters in KW's regime `n' = ceil(n/3)`, a generic `nu in GF(2^n)^{n'}` has `dim_F2 span E^3 = n`**, i.e. the F_2-linear map
`F_2^{rho} -> GF(2^n)`, `(c_t) -> sum c_t t(nu)` (t running over the cubic monomials in the `e`) is onto. What is known and what is not:
- Known (Corollary B): impossible when `n > rho`; possible: it holds on all independent instances sampled (1276), which includes every instance with `n' >= 4`, and on 84%, 74%, 35% of the `n' = 3` instances at `n = 8, 9, 10`.
  For n' = 3 the observed frequencies of full rank (0.84, 0.74, 0.35 at `n = 8, 9, 10`) are close to the random-matrix heuristic `prod(1 - 2^{i-rho})` (0.77, 0.58, 0.29 for `n = 8, 9, 10`), not a proof.
- Standard probabilistic tools do not give it. The dual condition is: for every `gamma != 0` some `Tr(gamma t(nu)) != 0`. Counting `nu` with all traces zero via Fourier
  gives `Pr[not onto] <= 2^{n-rho} + 2^n * max_{c != 0, gamma != 0} |E_nu chi(gamma P_c(nu))|` with `P_c = sum c_t t` homogeneous of degree 9 in `nu`.
  Schwartz-Zippel is too weak (error 8 instead of < 1). Deligne's bound `(d-1)^{n'} q^{-n'/2}` would suffice numerically for `n' >= 5` (it gives `2^n (8/sqrt q)^{n'} = 0.04` at `n = 13`, `n' = 5`),
  but it needs `P_c = 0` to be a smooth projective hypersurface, and it never is: every `P_c` is a sum of products of three `e_{ab}`, and the `e_{ab} = nu_a nu_b (nu_a + nu_b)` vanish simultaneously on the
  points of `P^{n'-1}` whose nonzero coordinates are all equal, so every member of the linear system is singular (multiplicity >= 3) at this common base locus. The Hooley-Katz singular-locus bounds
  have much worse constants (of order `(d+1)^{n'}`) and require the singular locus of `P_c` to stay 0-dimensional for all but a negligible share of the `2^{rho}` members; some members are non-reduced
  (for instance `e_{12}^3`), so one would have to bound the number of bad `c` among `2^{170}` (for n' = 5). I do not know how to do this uniformly in `n'`. SKETCH only.
- Hilbert-series route: the top-form ideal `(tau_1..tau_n)` of `Gr` is generated by `n` GF(2^n)-combinations of the conjugates of one product of six linear forms (two per block). It is as far from semi-regular as possible
  (nullity in degree 7 is at least `6n` instead of the generic 0 when `N n < C(N,7)`), so Froeberg/Moreno-Socias-type Hilbert-series statements, which describe generic forms, do not apply, and I know no replacement.
  What the structure does give exactly is the *first* fall (Theorem A); the exact Hilbert function in degrees `>= 8` is not controlled (in the measured data the degree-7 nullity is exactly `6n` for `n' >= 4`,
  i.e. no "mixed" relation between different conjugates; CONJECTURE that this holds for all `n' >= 4`).
- Field equations: in `Gr` the field equations become `y^2 = 0`, and the fall is precisely the ring-structure fact `L^2 = 0` (CG's "trivial" syzygies `f_i^{q-1} e_i` do not include it because `L` is a linear form dividing a top
  form, not a generator). In `B` the same fact shows up as `x^2 = x^{(2)}`, a linear form, which is why OP falls are large (nullity 6n lifts to elements of degree `<= 6` in `B`, rather than a single cancellation).

So: for m = 3, **the lower bound `D_ff >= 7` is proved in the top-form convention exactly on the set where `dim E^3 = dim V`, and for each concrete instance it is a finite certificate; the uniform "generic" version is reduced to (G) and not proved.**

## 6. Status table

| statement | convention | label |
|---|---|---|
| `D_ff <= 7` (m = 3) when `kappa(c) != 0`; `kappa = x4` | TF | PROVED (KW Thm 3.2; `kappa` re-derived by `semaev.py`) |
| `D_ff in {6,7}` and `= 7` iff `dim E^3 = dim V` | TF (= CG if all `deg s_k = 6`) | PROVED (Theorem A) |
| nullity of top syzygies in degree 7 is `>= 6n` (KW's argument gives 3 per conjugate; Frobenius gives 6) | TF | PROVED; equality MEASURED for `n' >= 4` |
| `D_ff = 6` when `n > rho(n')` or `V` in a proper subfield | TF | PROVED (Cor. B), MEASURED 1124/1124 dependent instances |
| `D_ff = 7` on all 1276 independent instances; all KW m = 3 cells reproduced | TF and OP | MEASURED, `[pcc:unverified]` |
| `D_ff^{OP} >= 7` if `dim E^3 = n` | OP | PROVED (Prop. C) |
| `D_ff^{OP} = 7` for all independent parameters | OP | CONJECTURE (MEASURED 1276/1276; SKETCH of mechanism) |
| (G) generic `nu` has `dim E^3 = n` for `n <= rho(n')`, in particular `n' = ceil(n/3)` | TF/OP | CONJECTURE (heuristic `1 - O(2^{n-rho})`; obstruction in section 5) |
| analogue for m = 4 (value 13), general m | TF | SKETCH: top form is a product of `(m-1)`-order Moore determinants, `m(m-1)` syzygies per conjugate; not computed, `kappa` for `S_5` not derived |
| KW question (4.1) `D_reg = m^2-m+1+o(1)`; last fall degree | - | untouched; nothing here bears on it |

## 7. Errors and caveats

- First implementation attempts at the full iterated closure at N = 15 did not finish in minutes; replaced by the stage-one detection (a sound lower bound for `dim W_c cap B_{<=c-1}`) plus full closure only where needed. The full closure at N <= 12 was run and agrees (section 4).
- The bridge `pi(V_{F+FE,c}) = W_c` for the operational closure is taken from the repository ledger; not re-proved here.
- The CG comparison used its own top-form degrees; 14 dependent instances have an `s_k` of degree 5, where CG's number could differ from HPS's; not computed.
- Samples per cell are small for `n' >= 6` (2 to 6); the `n' = 3` cells have 330 each. Frequencies for `n' = 3` are measured on those; the other cells are existence certificates rather than frequency estimates.
- Ten KW repetitions per cell and their Magma timings are not reproduced; only the first-fall value.
- `a2` was drawn at random but, as in KW's formula, does not enter `S_m`; the semantics check only exercised `a2 = 0` in the five sampled fields.
- No session log was written (outside permitted directory).

## 8. Reproduce

```
cd research/first_fall_closure_20261111/kw_sharpness && ./reproduce.sh      # about 57 s, 4 cores, < 300 MB
python3 aggregate.py                                                       # table_measured.md and aggregate_out.txt
```
