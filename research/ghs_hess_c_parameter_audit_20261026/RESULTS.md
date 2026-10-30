# Exact cubic Hess family: subgroup transfer and attack costs

## Claim boundary

The double trace-zero Hess branch is a **known eligible family** (Hess,
2004, Section 6). The transfer is a **known method**. The exact cubic
membership criterion and population formula in `EXACT_CUBIC_FAMILY.md`
refine its published heuristic count; whether that formula was published
elsewhere has not been established. The two equations below are new
**exact witnesses in this project**, not demonstrated new published curve
families. Their isogeny classes are catalogued in LMFDB. Neither exact
source beats rho in the measured implementation.

The distinction is essential: a new exact equation, a new eligible
family, and a new attack method are three separate claims. A curve is
called *measured vulnerable* here only when the nonzero source-subgroup
map is checked, the complete transfer computes the fixed source log,
and its total wall cost is lower than source-subgroup rho under the
same runtime. The two witnesses pass the first two tests and fail the
cost test.

## Exact sources and covers

Use `K=GF(2^9)=F_2[w]/(w^9+w^4+1)` and `k=GF(2^3)=F_2[a]/(a^3+a+1)`,
with Magma's compatible embedding `k -> K`. Integer coordinates denote
Conway polynomial coefficients, with bit `i` the coefficient of `w^i`.
Each source is `E_b: y^2+xy=x^3+b`. The fixed subgroup generator and
target are affine points; `O` is the point at infinity.

| `b` | `#E_b(K)` | `r` | `G` | `Q=[7]G` | Hess factors `(gamma,beta)` | Weil polynomial of cover Jacobian |
| ---: | ---: | ---: | --- | --- | --- | --- |
| 155 | 548 | 137 | `(478,460)` | `(328,58)` | `(392,295)` | `T^6+35T^3+512` |
| 32 | 556 | 139 | `(255,59)` | `(206,467)` | `(218,439)` | `T^6+43T^3+512` |

The factors have relative trace zero and product `sqrt(b)`. In both
cases the Hess descent is a smooth, nonhyperelliptic genus-three curve
over `k`; its canonical plane quartic has geometric automorphism group
`C2^2` and trivial `k`-rational automorphism group in the Magma check.
With coordinates `[X:Y:Z]`, the equations are:

`b=155`:

```text
X^4+a^5 X^2Y^2+a^5 X^3Z+a XY^2Z+a^3 Y^3Z
 +a^6 X^2Z^2+a^3 XYZ^2+a^3 Y^2Z^2+a^4 XZ^3+a^6 YZ^3+Z^4=0.
```

`b=32`:

```text
X^4+a X^2Y^2+a X^3Z+a^4 XY^2Z+a^6 Y^3Z
 +a^6 X^2Z^2+a^6 XYZ^2+a^3 Y^2Z^2+XZ^3+a^2 YZ^3+Z^4=0.
```

The isogeny classes are LMFDB `3.8.a_a_bj` and `3.8.a_a_br`,
respectively. LMFDB class data do not settle whether either exact
quartic equation appeared in a publication.

## Map certificates and complete attack

For the Hess map `h:E_b(K)->Div^0(C)`, form `D=h(G)-h(O)` and
`T=h(Q)-h(O)`, then push both through the canonical map. The retained
Magma certificates check `D` is nonprincipal, `[r]D` is principal, and
`T-[7]D` is principal. Both divisor classes therefore preserve the
fixed prime subgroup. The canonical-map receipts show two positive
degree-three places and a negative degree-six base place for each
image.

Magma's line-sieve `IndexCalculus` could not run on the original
singular sextic over `GF(8)` because its field-size condition failed.
The completed path uses the smooth canonical quartic, extends its
field to `GF(64)`, and reconstructs the positive and negative divisor
ideals there. It checks the lifted degree-zero classes again:
nonprincipal `D`, principal `[r]D`, and principal `T-[7]D`. The
index-calculus solver then obtains `7`, and the source check verifies
`[7]G=Q`. Thus the reported total includes source setup, cover/map,
base change and lifted-class checks, index calculus, and final source
verification. It excludes the subsequent rho calibration loop.

### Same-process costs

Three runs of the fixed Magma script were retained for each source.
Each run then performed 512 deterministic source-subgroup rho solves
on the same `E_b,G,Q,r`. The rho value is the mean per solve in a run;
the table uses medians across the three runs. Timings have about
0.01-second resolution, and the repeated rho starts are deterministic
rather than 512 independent challenge instances.

| `b` | index relations / factor base | full attack median | rho mean per solve, median | ratio of median costs |
| ---: | ---: | ---: | ---: | ---: |
| 155 | 59 / 51 | 0.720 s | 0.0003125 s | 2,304 |
| 32 | 51 / 49 | 0.870 s | 0.0003125 s | 2,784 |

The pure index line-sieve stage took 0.140 s for `b=155` in all three
runs and 0.240–0.260 s for `b=32`. Omitting setup and transfer would
understate the cost of using GHS on the original subgroup. A separate
Sage relation-walk implementation also solved both fixed source logs
end to end, in 792.828 s and 261.920 s respectively; it is a slower
transfer control, not the native index-calculus cost.

`audit.py` checks all three native receipts for matching script SHA-256,
successful complete output, lifted subgroup tests, index answer,
source verification, relation counts, and timing fields. The
`index_rho_*_run{1,2,3}_receipt.json` files retain raw Magma output;
`audit.json` retains the computed summary. `Bounds.lean` checks count
arithmetic and an abstract subgroup scalar-preservation lemma. The
finite-field product-isomorphism proof, geometric Hess map, and exact
Magma identities are not represented as fully formal Lean proofs.

## What the bound does and does not say

For `q=2^ell`, the exact number of nonzero coefficients `b` in this
double trace-zero genus-three branch is
`(q-1)(q+1)(q+2)/2`, less `q-1` for full cubic degree when `ell` is
even. The intersection with the direct trace-zero hyperelliptic branch
is exactly `q^2-1`, so the distinct-factor branch outside it has
exactly `(q-1)q(q+1)/2` coefficients. This is a rigorous upper bound
on the number of *measured
vulnerable* source models **within this normalized Hess branch**.
It is not a bound on every GHS construction, every isogenous curve,
or every possible cover attack. Karabina--Menezes--Pomerance--
Shparlinski (2010) give broad asymptotic genus and coefficient-count
bounds as the extension degree grows; those bounds and the exact cubic
count answer different questions. Diem's later plane-quartic
`O~(q)` algorithm suggests an asymptotic advantage over generic rho
if prime subgroup order grows as `Theta(q^3)` and efficient transfer
is available, though a best direct elliptic-curve index-calculus
comparison is a separate question. The two `q=8` witnesses do not
furnish a measured speedup. A global novelty or vulnerability claim
needs a literature audit of exact equations and larger-field complete
comparisons.

## Primary references

- Hess, *Generalising the GHS attack on the elliptic curve discrete
  logarithm problem* (2004), Theorems 11, 12, 14 and Section 6:
  <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf>
- Menezes and Teske, *Cryptographic implications of Hess' generalized
  GHS attack* (2004), Section 7:
  <https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf>
- Diem, *On the discrete logarithm problem for plane curves* (2012):
  <https://www.numdam.org/articles/10.5802/jtnb.815/>
- Karabina, Menezes, Pomerance and Shparlinski, *On the asymptotic
  effectiveness of Weil descent attacks* (2010), Theorems 8 and 9:
  <https://math.dartmouth.edu/~carlp/WeilDescent-Product.pdf>
- Nart and Ritzenthaler, *Non-hyperelliptic curves of genus three over
  finite fields* (2009), geometric automorphism families:
  <https://arxiv.org/pdf/0905.0546>
