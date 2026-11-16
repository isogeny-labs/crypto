# Order-4222 cubic descent: an explicit witness in an unresolved isogeny class

## Result and claim boundary

This folder gives a reproducible, end-to-end GHS transfer for one source
elliptic curve of order `4222 = 2*2111` over `F_(2^12)`. The descended curve is
a smooth nonhyperelliptic genus-three curve over `F_16`. Its Jacobian lies in
LMFDB isogeny class `3.16.a_a_ev`, whose `has_jacobian` value was `0`
(**unknown**) in both the 2026-10-19 local snapshot and the live SQL mirror
queried on 2026-10-28. The explicit quartic and its L-polynomial constitute a
positive class witness suitable for catalog review. The earlier
`ghs_twisted_v4_triangle_20261027/REPORT.md` already gives an
explicit construction in this same isogeny class, using source parameters
`(a,b)=(32,853)` in its own field basis. This run adds a second source
equation and a complete measured attack route. Isomorphism of the two source
curves has not been checked. Neither the class witness nor
the geometric family is first identified here. The source equation is new
to this audit; global publication novelty remains unverified.

This curve belongs to the previously identified nonsplit quadratic-pole
family. The construction is neither a newly established geometric family nor
a new attack method. The completed dynamic Magma transfer and attack took
about 271 times the mean cost of a matched source-group rho solve. Thus this
particular curve is **not** a measured GHS speedup.

## Exact source and cover

Let `L = F_2[z]/(z^24+z^16+z^15+z^14+z^13+z^10+z^9+z^7+z^5+z^3+1)`.
The unique subfields `k` and `K` of `L` have 16 and 4096 elements. All
decimal elements below are 24-bit coefficient masks in the basis
`1,z,...,z^23`, rather than integer coefficients. The source is

```
E/K: Y^2 + XY = X^3 + 5522909 X^2 + 1610778
#E(K) = 4222 = 2*2111,  2111 prime
G = (1610779, 12717539),  order(G) = 2111
Q = (10751255, 15683006) = 7G
```

Set `q=16`, `t=16191718`, and `A=5857459` in `L`. Then
`t+t^q=1`, `Tr_(L/F_256)(A)=0`, and
`(A*A^(q^3))^2=1610778`. The three conjugate Artin-Schreier functions are

```
f_i(x) = A^(q^i)/(x+t^(q^i))
       + A^(q^(i+3))/(x+t^(q^(i+3))),  i=0,1,2.
```

They lie in `K(x)`, satisfy `f_0+f_1+f_2=0`, and descend cyclically under
`q`-Frobenius. Their compositum gives a `V4` cover. An exact affine model of
the descended curve over `k` is

```
u^4 + 4534198 u =
  (5763596 + x + 4626534 x^2 + 5763597 x^3)
  / (5522908 + x^2 + x^4).
```

The exported smooth projective plane quartic and its affine-to-canonical map
are in `canonical_4222.json`. Magma 2.30-1 checked geometric genus 3,
nonhyperellipticity, smoothness, and

```
L_C(T) = 1 + 125 T^3 + 4096 T^6.
```

This polynomial is the exact `poly` value `[1,0,0,125,0,0,4096]` for
LMFDB class `3.16.a_a_ev`. The class has `p_rank=3`, `is_simple=true`, and
`is_geometrically_simple=false`. The pinned polynomial and canonical-map
receipt is `canonical_4222_receipt.json`. An independent Sage count finds
`#C(F_16)=17`, `#C(F_256)=257`, and `#C(F_4096)=4472`; the last count uses
the 1117 split affine fibers of the V4 cover plus four points at infinity.
Newton identities and the genus-three functional equation recover the same
L-polynomial. See `independent_counts_4222.json`.

## Subgroup-preserving correspondence

Magma constructed the genus-one quotient `C_0/K` defined by `y_0^2+y_0=f_0(x)`
and an explicit isomorphism from its elliptic model to `E`. The rational map
coefficients are in `quotient_map_4222.json`; the exact Magma input and output
are in `quotient_4222.magma` and `quotient_4222_receipt.json`. Three split
fibers above `R=2G`, `R+G=3G`, and `R+Q=9G` are in `preimages_4222.json`.

For the quotient map `pi:C_K -> E`, define

```
Phi(P) = sum_(i=0)^2 Frob_q^i( pi^*([P+R]-[R]) ).
```

This is a homomorphism from the source elliptic group to `J(C)(k)`. In the
three `V4` quotient components, push-pull gives `pi_i,* pi_i^* = [2]` and
the cross terms vanish. Projecting `Phi` back to the first quotient after
base change to `K` gives `[2]P`. Therefore its kernel has no odd torsion;
in particular, it is injective on the prime-order 2111 subgroup. The
executable Sage Jacobian calculation independently verified

```
Phi(G) != 0,   2111 Phi(G) = 0,
Phi(Q) != 0,   2111 Phi(Q) = 0,
Phi(Q) = 7 Phi(G).
```

The map receipt includes the exact quartic fibers and five Boolean checks.
`Order4222Gate.lean` kernel-checks the finite count, source cofactor, scalar,
and odd-subgroup doubling arithmetic. It does **not** formalize finite-field
descent, smoothness, the quotient correspondence, or the Jacobian identities;
those claims have Sage and Magma evidence and a mathematical argument here.

## Complete attack versus matched rho

Magma's plane-quartic line sieve ran on the base change to `F_256`. The checked
job verified that the source and lifted divisors had exact 2111-order and
that the target was seven times the base. It found 260 relations over 206
factor-base points, found a matrix kernel, recovered `7 mod 2111`, and
checked `7G=Q` on the source. It took 71.599 server seconds, mostly in the
extra principal-divisor assertions. The separately timed job used the same
mapped divisors and complete target recovery, taking 0.200 CPU seconds for
setup and 0.210 CPU seconds for index calculus (0.490 server seconds total).

For a stricter same-backend comparison, `full_attack_4222.magma` starts with
the exact source points, rebuilds the quotient and split fibers, evaluates
their canonical-quartic images, forms the traced divisors, lifts them to
`F_256`, and runs the line sieve. Its checked job verified the 2111-order and
`7` relation both before and after lifting. The separate timed job omitted
the expensive principal-divisor assertions, while recovering and verifying
`7` through the same dynamic path. It measured 0.190 CPU seconds through
source mapping, 0.250 CPU seconds through all pre-index work, 0.210 CPU
seconds in index calculus, and 0.460 CPU seconds from start to verified
source answer (0.550 server seconds). The canonical quartic equation is
reusable curve setup supplied as an exact input to both jobs.

An explicit Floyd-rho implementation on the same `E,G,Q,2111` in Magma 2.30-1
completed 100/100 seeded runs, with median 207 group-step calls and 0.170 CPU
seconds for all 100, or 0.00170 CPU seconds per solve. The 0.460-second
dynamic Magma attack was about 271 times that matched rho mean. The older
precomputed-divisor attack core took 0.410 CPU seconds, 241 times rho. Local Sage
Jacobian setup took 13.65 wall seconds and two map evaluations took 5.33 wall
seconds; these are separate-machine measurements and are excluded from the
Magma CPU ratio. The full dynamic Magma run supersedes them for the main
same-backend comparison. The two failed preliminary dynamic jobs are retained
with `failed_syntax` and `failed_verbose` names; neither is counted as a
completed solve. The exact inputs and receipts are summarized in
`cost_4222.json`.

## Bounds and prior work

For the normalized ordinary models `Y^2+XY=X^3+aX^2+b` over `F_(q^3)` with
`q=2^m`, the nonsplit quadratic-pole construction has the exact coefficient
criterion

```
Tr_(F_(q^3)/F_2)(a)=1,
sqrt(b) in { A*A^(q^3) : 0 != A in ker Tr_(F_(q^6)/F_(q^2)) }.
```

The norm image has exactly `(q^3+q-2)/2` values. The count of models added
outside the triangle screen is `(q^3-q^2+4q-4)/4` when `m` is even, and
`(q^3-q^2-4)/4` when `m` is odd. Together with triangle and Hess screens,
the union contains `3q^3/2+q-2` or `3q^3/2-2` of the `2(q^3-1)` normalized
models, respectively. At `q=16`, these numbers are 975 additional nonsplit
models and 6158/8190 in the three-screen union. The remaining 2032 models
are simply outside these screens; the count gives no resistance bound against
other covers or isogenies. A screen count also does not imply a concrete
attack advantage. See the companion nonsplit-family report and executable
population count for the all-`q` proof and the `q=2,4,8,16` enumeration.

Momose and Chao (2006) and Murai et al. (2022) classify related binary cover
routes, so the geometry cannot be called a new family from these data. Joux
and Vitse (2012) studied cover-and-decomposition attacks and isogeny walks.
Searches for the exact class label, Weil polynomial, source order, and subgroup
prime did not find an indexed publication of this exact instance; those
negative searches do not prove global novelty. The per-source prior-work
ledger is `prior_work_4222.json`. The class's `has_jacobian=0` is an
uncertainty marker, not a negative theorem, per the LMFDB inventory's
three-valued Jacobian status.

Primary links: [Momose-Chao 2006](https://eprint.iacr.org/2006/347),
[Murai et al. 2022 abstract](https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf),
[Joux-Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf),
[LMFDB inventory](https://github.com/LMFDB/lmfdb-inventory/blob/master/db-abvar.md),
[LMFDB class](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_ev).

## Reproduce

From the repository root:

```sh
sage -python research/ghs_nonsplit_order4222_20261028/source_4222.py
python3 research/ghs_nonsplit_order4222_20261028/magma_canonical_4222.py
sage -python research/ghs_nonsplit_order4222_20261028/independent_counts_4222.py
sage -python research/ghs_nonsplit_order4222_20261028/quotient_4222.py
python3 research/ghs_nonsplit_order4222_20261028/parse_quotient_4222.py
sage -python research/ghs_nonsplit_order4222_20261028/subgroup_map_4222.py
sage -python research/ghs_nonsplit_order4222_20261028/index_4222_q256.py --factor-base 200 --relations 260
sage -python research/ghs_nonsplit_order4222_20261028/index_4222_q256.py --factor-base 200 --relations 260 --benchmark
python3 research/ghs_nonsplit_order4222_20261028/full_attack_magma_4222.py
python3 research/ghs_nonsplit_order4222_20261028/full_attack_magma_4222.py --benchmark
python3 research/ghs_nonsplit_order4222_20261028/rho_magma_4222.py
python3 research/ghs_nonsplit_order4222_20261028/cost_4222.py
lean research/ghs_nonsplit_order4222_20261028/Order4222Gate.lean
```

## Q16 direct-screen isogeny frontier

The companion [Q16_FRONTIER.md](Q16_FRONTIER.md) exhausts all 8,190
normalized ordinary source models. The three established direct genus-three
V4 screens cover 6,158 models and their full source-field isogeny closure
covers 8,124, leaving 66 models in three exact source-order classes. Those
classes have catalogued genus-three Jacobians, but their subgroup-preserving
isogenies are not yet evaluated and their complete attack costs are unknown.
