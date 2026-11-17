# Exact cubic criterion for Hess genus-three descent

## Claim and prior work

Let `k=F_q`, `K=F_{q^3}`, where `q` is a power of two, and let
`U=ker(Tr_{K/k})`. For each nonzero `b in K`, consider the ordinary
elliptic curve

`E_b : y^2+xy=x^3+b`.

Hess's 2004 construction allows an Artin--Schreier equation
`v^2+v=gamma/x+beta*x` when `gamma*beta=sqrt(b)`. His Theorems 11, 12,
and 14 establish the descent, genus, and subgroup kernel conditions.
His Section 6 explicitly discusses the branch with both factors of
relative trace zero and gives a heuristic near-injectivity count.
Consequently this branch is a **published eligible family**, and the
Hess transfer is a **published attack method**. The result below is an
exact cubic classification and count refining that heuristic. We have
not established that this exact count is absent from all later papers.

Primary references:

- Hess, *Generalising the GHS attack*, 2004, Theorems 11, 12, 14 and
  Section 6: <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf>
- Menezes--Teske, *Cryptographic implications of Hess' generalized GHS
  attack*, 2004, Section 7: <https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf>.
  Their quantified cubic case chooses one factor in `k` and obtains a
  hyperelliptic cover; the trace-zero × trace-zero branch here gives a
  nonhyperelliptic cover when `b` is outside the direct branch.
- Karabina, Menezes, Pomerance and Shparlinski, *On the asymptotic
  effectiveness of Weil descent attacks* (2010):
  <https://math.dartmouth.edu/~carlp/WeilDescent-Product.pdf>.
  Their asymptotic genus and coefficient-count bounds address growing
  extension degree. They do not give the exact cubic product count here.

## The exact theorem

Define

`P_q={gamma*beta : gamma,beta in U\{0}}`.

For `delta=sqrt(b)`, the double-trace-zero Hess genus-three criterion is
exactly `delta in P_q`. Write `delta` uniquely as

`delta=A*u^2+B*u*v+C*v^2`,

where `u,v` are any `k`-basis of `U` and `A,B,C in k`. Then:

1. If `B=0`, `delta in P_q` whenever `delta != 0`.
2. If `B!=0`, `delta in P_q` if and only if
   `Tr_{k/F_2}(A*C/B^2)=0`.
3. `|P_q|=(q-1)(q+1)(q+2)/2`.
4. Exactly `q^2-1` elements of `P_q` have a repeated projective factor.
   The other `(q-1)q(q+1)/2` have two distinct projective factors.
5. `P_q` meets `U` in exactly the `q^2-1` repeated-factor products.
   Thus the distinct-factor branch outside the direct trace-zero
   hyperelliptic branch has exactly `(q-1)q(q+1)/2` coefficients.
6. If `q=2^ell` with odd `ell`, then `P_q` contains no nonzero element
   of `k`; all `|P_q|` coefficients `b=delta^2` have cubic field of
   definition. If `ell` is even, `P_q` contains every element of `k^*`,
   so exactly `|P_q|-(q-1)` corresponding `b` have cubic field of
   definition.

The resulting Hess curve has genus
`2^2-2^(2-2)-2^(2-2)+1=3` by Hess's Theorem 12. This is an
**eligibility statement**. The number of curves for which a complete
attack beats source-subgroup rho is bounded above by the corresponding
full-degree count above; that stronger vulnerability property also
depends on subgroup maps and measured or proved attack cost.

Diem proved expected `O~(q)` discrete-log complexity for smooth
nonhyperelliptic plane quartics over `F_q` (see
<https://www.numdam.org/articles/10.5802/jtnb.815/>).
For a sequence of this family with subgroup prime `r=Theta(q^3)` and
efficient subgroup transfer, this combines with Hess to give an
**asymptotic** improvement over rho's `Theta(sqrt(r))` group-operation
cost. This comparator is generic rho; direct elliptic-curve
index-calculus algorithms may also apply. The theorem does not provide
a crossover field size or a speedup
for the two `q=8` witnesses; the full costs are in `RESULTS.md`.

### Proof

`U` is two-dimensional over `k` because trace is a nonzero `k`-linear
map `K -> k`. For independent `u,v in U`, the ratio `t=u/v` is outside
`k`; otherwise `u,v` would be dependent. Since `[K:k]=3` is prime,
`t` has minimal polynomial of degree three. A relation
`A*u^2+B*u*v+C*v^2=0` would yield `A*t^2+B*t+C=0`, hence
`A=B=C=0`. Multiplication therefore gives a `k`-linear isomorphism
`Sym^2(U) -> K` between two spaces of dimension three.

Under that isomorphism, `gamma*beta` is the binary quadratic form
obtained by multiplying the two linear forms representing `gamma` and
`beta`. Unique factorisation in `k[X,Y]` identifies its unordered pair
of projective factor lines. There are `q+1` lines in `U`, hence
`binom(q+2,2)` unordered pairs with repetition, and each projective
quadratic has `q-1` nonzero scalar multiples. This proves (3). Repeated
lines give `(q+1)(q-1)=q^2-1`, proving (4).

For (5), the trace pairing `Tr_{K/k}(xy)` is nondegenerate on `K`.
Because `Tr_{K/k}(1)=1`, its restriction to the orthogonal complement
`U=1^perp` is nondegenerate. In characteristic two,
`Tr_{K/k}(x^2)=Tr_{K/k}(x)^2=0` for `x in U`, so this restriction is an
alternating form on the two-dimensional space `U`. Consequently
`Tr_{K/k}(gamma*beta)=0` for nonzero `gamma,beta in U` if and only if
they lie on the same projective line. Those products are precisely
the repeated-factor products. Squaring permutes `U^*`, yielding the
count and direct-branch distinction.

The finite field `k` is perfect. A binary quadratic with `B=0` is a
square of a linear form. For `B!=0`, factorisation is equivalent to the
solvability of `z^2+z=A*C/B^2`; over `k`, this is equivalent to zero
absolute trace. This proves (1) and (2).

For (6), a product `gamma*beta` in `k^*` implies both
`Tr(gamma)=0` and `Tr(1/gamma)=0`. The first two elementary symmetric
coefficients of the conjugates of `gamma` vanish, so its irreducible
polynomial is `T^3+c` for some `c in k^*`. If `3` is coprime to `q-1`,
the cube map is bijective on `k^*`, and `T^3+c` has a `k`-root, a
contradiction. If `3` divides `q-1`, choose a noncube `c`; then
`T^3+c` is irreducible, and a root gives trace-zero `gamma` and
`1/gamma`. Scaling the second factor by `k^*` gives every nonzero
subfield product. Since `q=2^ell`, the two cases are respectively odd
and even `ell`.

## Exact `q=8` population and witnessed sources

For `q=8`, the formula gives `315` normalized `a2=0` source models
among the `511` nonzero `b`. Of these, `63` also have the direct
hyperelliptic branch, while `252` have only the distinct-factor
branch within this criterion. The exhaustive `GF(512)` census reports
`189` models outside the prior direct-B **isogeny-order closure**;
`144` of these have group order `4 mod 8`. This is a finite-field
census, not a general formula for isogeny classes.

Two exact `4 mod 8` sources with verified nonzero  prime-subgroup maps:

| source `b` in Conway `GF(512)` | `#E_b(K)` | subgroup prime | `gamma`, `beta` in Conway integer form | canonical cover `L(T)` |
| --- | ---: | ---: | --- | --- |
| 155 | 548 | 137 | 392, 295 | `T^6+35T^3+512` |
| 32 | 556 | 139 | 218, 439 | `T^6+43T^3+512` |

`cert_548.magma`, `map_class_556.magma`, and the canonical-map receipts
check the field identities, source orders, the degree-six divisor
images, nontriviality of the image of `G`, and `h(Q)-h(O)=7(h(G)-h(O))`
for `Q=7G`. The source primes are preserved. The canonical images are
smooth plane quartics over `GF(8)`, with full equations in the receipts.
The 511-parameter Magma sweeps find fourteen genus-three parameters
for each of these two sources.

`verify_split_criterion.sage` independently checks the exact count,
quadratic-splitting test, and subfield exception for `q=2,4,8,16`.
`Bounds.lean` checks the integer consequences and abstract subgroup
scalar preservation. Its field isomorphism, the enumerated source
orders, the geometric cover, and the concrete map identities remain
external mathematical and computational inputs; the Lean file is not
a formal proof of the entire Hess construction.

For context, Karabina--Menezes--Pomerance--Shparlinski prove that as
the extension degree `n` grows, almost all coefficients lead to genus
at least `2^(n/2+o(n))`, and cite an upper bound of
`q^(16 log_2(n)+O(1))` coefficients for genus at most `n^4`.
Those are asymptotic broad-family results; this theorem gives an exact
count for `n=3` and this specified Hess branch. Neither result by
itself is a total-cost vulnerability bound after isogenies.

The exact canonical quartic equations, lifted subgroup checks, three
native index-calculus runs per source, and matched rho costs are in
`RESULTS.md` and `audit.json`. Both complete attacks recover the fixed
source logarithm, but their measured total costs exceed rho.

The LMFDB abelian-variety catalog already contains the isogeny
classes `3.8.a_a_bj` and `3.8.a_a_br`, with respective Weil
polynomials above (local `av_fq` snapshot extracted 2026-10-19).
That catalog has class-level data and cannot establish whether either
specific quartic equation is unpublished. The equations are **new
exact witnesses in this project**. Publication novelty of the
equations and of the exact product count remains open.
