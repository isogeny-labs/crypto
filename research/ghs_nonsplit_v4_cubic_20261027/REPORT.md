# Nonsplit quadratic-pole cubic V4 descent

## Scope and claim

This audit isolates a third explicit genus-three cubic-descent screen: the
elliptic quotient has a quadratic conjugate pair of poles over
`K = F_(q^3)`, rather than the `K`-rational poles used by the triangle and
Hess screens. The screen yields an exact count and a reproducible
subgroup-preserving map and complete GHS-versus-rho control for one
`q=16` source. This example is **not** a measured GHS speedup. Earlier
even-characteristic cubic-cover
classifications likely include the same geometric case; a 2015 survey
already attributes a three-quarters density to the earlier
even-characteristic work. Global novelty of the family or density is
unverified. See `prior_work.json`.

Throughout, `q=2^m`, `k=F_q`, `K=F_(q^3)`, `k2=F_(q^2)`, and
`L=F_(q^6)`. Counts concern normalized ordinary models
`E_(a,b): Y^2+XY=X^3+aX^2+b` with `b != 0` and
`Tr_(K/F2)(a) in {0,1}`. There are `2(q^3-1)` such model classes.

## Construction and exact screen

Choose `t in k2` with `t+t^q=1`, and set
`W=ker Tr_(L/k2)`. For `A in W\{0}`, put

```text
f_i(x) = A^(q^i)/(x+t^(q^i))
       + A^(q^(i+3))/(x+t^(q^(i+3))),       i=0,1,2.
s = N_(L/K)(A) = A*A^(q^3),                 b=s^2.
```

Each `f_i` belongs to `K(x)`; the three functions cycle under `q`-Frobenius
and sum to zero because `Tr_(L/k2)(A)=0`. The compositum
`y0^2+y0=f0`, `y1^2+y1=f1` therefore descends to a curve `C/k`.
The two geometric poles are distinct, shared by the two independent
Artin-Schreier characters, and have full V4 inertia. The ramification
formula gives `g(C)=3`; the three degree-two quotients have genus one.
In the normal form above, the first quotient has `j=1/b` and the
**trace-one** normalized twist. Thus the exact parameter screen for this
construction is

```text
Tr_(K/F2)(a)=1,
sqrt(b) in {A*A^(q^3) : 0 != A in ker Tr_(L/k2)}.
```

The twist statement follows from the rational two-torsion point
`T=(0,s)` on `E_(a,b)`. The `x` coordinate of `2P` is
`x(P)^2+b/x(P)^2`, so `2P=T` forces `x(P)=b^(1/4)`.
The point equation at that coordinate reduces to an Artin-Schreier
equation whose absolute trace is `Tr_(K/F2)(a)`.
The quotient involution `P -> T-P` has its two fixed points over `K`
exactly when `T` is divisible by two over `K`; the quadratic pole pair
therefore selects trace one. The explicit `q=16` quotient is checked
independently with Magma and Sage.

## All-q count and overlap proof

Let `V=ker Tr_(K/k)`. The inclusion `V subset W` identifies
`W=V tensor_k k2`, a two-dimensional vector space over `k2`.
Multiplication gives an isomorphism
`Sym^2_(k2)(W) -> L`: for independent `u,v in W`, their ratio has
degree three over `k2`, so `u^2,uv,v^2` are independent.
Consequently, two nonzero elements of `W` have the same norm up to
`k*` precisely when their projective lines are exchanged by the
semilinear involution `A -> A^(q^3)`. Its action on `P^1(k2)` is
coefficient Frobenius in a `V` basis. Of `q^2+1` lines, `q+1` are fixed,
giving `(q^2+q+2)/2` projective norm classes. Each contributes `q-1`
norm values because `N_(k2/k)(k2*)=k*`. Hence

```text
|{A*A^(q^3): A in W*}| = (q^3+q-2)/2.                 (1)
```

Let `e=(q+1)^(-1) mod (q^3-1)` and `d=(2(q+1))^(-1)
mod (q^3-1)`. The triangle screen for the trace-one twist selects
`Tr_(K/F2)(b^d)=1`; thus a nonsplit model adds to the union when this
trace is zero. Since `b=s^2`, we have `b^d=s^e`. A projective norm
class has `Tr_(K/k)(s^e)=0` only if the two factors
`[s^e]` and `[(s^e)^q]` of `s=s^e*(s^e)^q` are the same projective
line of `W`. Uniqueness in `Sym^2(W)` then implies
`(s^e)^q=c*s^e` with `c in k*`, `c^3=1`, and `c != 1`.
There are exactly two such eigenlines if `m` is even and none if
`m` is odd. For either zero-relative-trace class, all `q-1` scalars
have absolute trace zero; every other class contributes `q/2-1`.
Writing `Z=2` for even `m` and `Z=0` for odd `m`, the exact number
of nonsplit models beyond the triangle screen is

```text
N_new(q) = ((q^2+q+2)/2)*(q/2-1) + Z*q/2
         = (q^3-q^2+4q-4)/4   if m is even,
           (q^3-q^2-4)/4      if m is odd.            (2)
```

For completeness, the Hess two-rational-pole screen takes products
`s=u*v` with `u,v in V*` and selects the trace-zero twist. Unique
factorization in `Sym^2(V)` gives `(q+1)(q+2)/2` projective product
classes. Exactly `q+1` have `Tr_(K/k)(s^e)=0`: they are the classes
`s=z^(q+1)` with `z in V*`. Each of the other `q(q+1)/2` classes
has `q/2` scalar multiples with absolute trace one, and those are
precisely the trace-zero Hess models outside the triangle screen.
Hence that screen adds `q^2(q+1)/4` models. It is disjoint from
the trace-one nonsplit addition. Thus the **three-screen union** is

```text
U(q) = 3q^3/2 + q - 2   if m is even,
       3q^3/2 - 2       if m is odd.                   (3)
```

This leaves `q^3/2-q` (even `m`) or `q^3/2` (odd `m`) normalized
models outside these three particular screens. Those complement
counts are **not** lower bounds on GHS-resistant curves: other covers,
isogenies, and transfer constructions may still apply. Likewise,
membership in the union is not an attack-cost bound.

`population.py` exhaustively checks equations (1)-(3) at
`q=2,4,8,16`. Its results are:

| q | norm values | additional nonsplit models | three-screen union / all models |
|---:|---:|---:|---:|
| 2 | 4 | 0 | 10 / 14 |
| 4 | 33 | 15 | 98 / 126 |
| 8 | 259 | 111 | 766 / 1022 |
| 16 | 2055 | 975 | 6158 / 8190 |

The Lean file `NonsplitGate.lean` kernel-checks the displayed finite
arithmetic for `q=2,4,8,16,32` and the absence of a two-kernel on
`Fin 2027`. It does **not** formalize the field, orbit, genus, or
Jacobian arguments; those are given above and checked for the exact
source by executable algebra systems.

## Exact `q=16` source and map

Represent `L` as `F2[z]/(z^24+z^16+z^15+z^14+z^13+z^10+
z^9+z^7+z^5+z^3+1)`, and encode field elements as coefficient
bitmasks in `1,z,...,z^23`. `K` and `k` are the unique subfields
of sizes `2^12` and `2^4` in `L`. The source is

```text
E/K: Y^2+XY = X^3 + 5522909*X^2 + 9159130
#E(K)=4054=2*2027
G=(9159131,11791722),  Q=(744530,10466263)=7G
t=16191718,  A=2832837,  A^(q^3)=9705874.
```

The displayed decimal coefficients are the bitmasks just defined,
not integer coefficients. The source fails the triangle trace gate;
its trace-one twist excludes the Hess rational-pole gate. The
constructed quartic `C/k` is smooth and nonhyperelliptic in Magma,
with `L_C(T)=1-43T^3+4096T^6`. Its equation and rational maps
are exported exactly in `canonical_4054.json` and
`quotient_map_4054.json`.

For the quotient `pi:C_K -> E`, take `R=G` and define the source
homomorphism by the traced correspondence

```text
Phi(P) = sum_(i=0)^2 Frob_q^i(pi^*([P+R]-[R])).
```

The split quotient fibers for `R`, `R+G=2G`, and `R+Q=8G`
are retained in `preimages_4054.json`. `subgroup_map.py` independently
checks every Artin-Schreier fiber, the canonical quartic, the
quotient rational map, and the actual Jacobian classes. Its result:

```text
Phi(G) != 0,  2027*Phi(G)=0,
Phi(Q) != 0,  2027*Phi(Q)=0,
Phi(Q)=7*Phi(G).
```

The same construction gives a uniform odd-subgroup statement. Over
`K`, the three quotient maps `pi_i:C_K -> E_i` give the three
nontrivial V4 character components of `J(C)_K`. Their push-pull
relations are `pi_i,* pi_i^*=[2]` and
`pi_i,* pi_j^*=0` for `i != j` (the latter follows by averaging over
the involution that fixes `pi_j` and negates the `i` component).
After extending the traced correspondence to `K`, projection to the
first quotient therefore gives `[2]P`. Thus its kernel on `E(K)` is
2-primary, and it is injective on every odd-order subgroup. The
computational checks above confirm this identity on the named 2027
subgroup, including the exact target relation.

The latest measured Jacobian setup was 15.15 seconds and the two map
evaluations took 5.04 seconds in local Sage. Magma's built-in
plane-quartic index calculus requires at least `d! = 24` elements for
degree `d=4`, so the `F16` model was base-extended to `F256`. The
`fb47` run retained in `index_4054_q256_fb47_receipt.json` found only
3 relations for a 54-element factor base. With requested base size
200 and 260 relations, the complete Magma run checked that both the
base and lifted divisors have exact 2027-order, collected 260
relations over 201 factor-base points, found a matrix kernel vector,
recovered `7 mod 2027`, and verified the source equation `7G=Q`.

The fully checked Magma job reported 67.239 seconds, dominated by its
extra principal-divisor assertions. A separate timed attack run used
the already checked divisor construction and omitted those assertions:
0.190 CPU seconds for Magma setup, 0.170 CPU seconds for the
index-calculus target recovery, and 0.459 seconds for the complete
Magma server job. The Sage source-map setup and evaluation times above
are separate and cannot be added to the server time as if measured on
one machine.

For the same `E,G,Q,r`, `rho_4054.py` completed 100 explicit local
Floyd-rho runs with median 159 group step calls. An independent
Magma implementation completed 100/100 runs with median 168 group
step calls and 0.160 CPU seconds total, or 0.00160 CPU seconds per
solve on average; its server job time was 0.279 seconds. The
measured Magma attack core (`0.360` CPU seconds) is therefore about
225 times this average rho solve, even before the source-map cost.
This is a source-specific measured comparison, not a projection to
cryptographic field sizes.

For members whose genus-three cover is nonhyperelliptic and whose
source odd prime subgroup has order `r=Theta(q^3)`, Diem's plane-quartic
theorem gives an expected `soft-O(q)` target DLP algorithm, whereas
generic rho needs `Theta(sqrt(r))=Theta(q^(3/2))` group steps. This
is a conditional asymptotic comparison with different operation types;
it does not establish a concrete crossover or an advantage for the
displayed `q=16` source. Source transfer and target recovery must be
included in any concrete bound.

## Reproduction

```sh
sage -python research/ghs_nonsplit_v4_cubic_20261027/population.py
sage -python research/ghs_nonsplit_v4_cubic_20261027/nonsplit.py
magma research/ghs_nonsplit_v4_cubic_20261027/canonical_4054.magma
magma research/ghs_nonsplit_v4_cubic_20261027/quotient_isomorphism_4054.magma
magma research/ghs_nonsplit_v4_cubic_20261027/split_preimages_4054.magma
sage -python research/ghs_nonsplit_v4_cubic_20261027/subgroup_map.py
sage -python research/ghs_nonsplit_v4_cubic_20261027/rho_4054.py
sage -python research/ghs_nonsplit_v4_cubic_20261027/index_4054_q256.py --factor-base 200 --relations 260 --benchmark
python3 research/ghs_nonsplit_v4_cubic_20261027/rho_magma_4054.py
python3 research/ghs_nonsplit_v4_cubic_20261027/cost_summary.py
lean research/ghs_nonsplit_v4_cubic_20261027/NonsplitGate.lean
```

The Magma receipts and JSON exports are retained in this directory.
The exploratory inverse/preimage receipts record failed routes before
the evaluated fiber map was found.
