# GF(16) cubic GHS extension: exact class-C quartic witnesses

26 October 2026. Conductor T-5361. The [frozen protocol](PROTOCOL.md) precedes
the draw. The [screen](screen.py), [raw cohort](results.json),
[independent replay](verify.py), [replay certificate](verification.json), and
[prior-work ledger](prior_work.json) preserve the exact run.

## New finding and claim boundary

The GF(16)/GF(4096) extension found three exact class-C-only source orders
with a large selected prime subgroup and a smooth plane-quartic Jacobian in
the same finite-field isogeny class. The equations are new **entries in this
research screen** and are absent from the LMFDB `curves` arrays queried on
26 October 2026; those arrays currently list only hyperelliptic models for
all eleven matched classes. This is a database/model gap, not proof of
publication novelty or a new vulnerable family. An explicit source-to-quartic
map and its nonzero action on the prime subgroup remain unconstructed.

The screen drew 2,048 quartics. The filter tallies were `N1=17`: 231,
`N2=257`: 14, smooth: 12, full Weil-polynomial/source-order matches: 12.
Those matches covered eleven distinct source orders. The source cohort
contains all 8,190 normalized ordinary elliptic models, with both absolute
trace classes of `a` and every nonzero `b`.

## Exact curves and subgroups

Let `K=GF(2)[w]/(w^12+w^7+w^6+w^5+w^3+w+1)` and
`k=GF(2)[a0]/(a0^4+a0+1)`, using Sage's polynomial-basis integer encoding.
Each source has equation `E(a,b): y²+xy=x³+a*x²+b`. Quartic coefficient
vectors follow monomials `x^i*y^j*z^(4-i-j)` in descending `i=4..0`, then
descending `j=4-i..0`. The zero leading coefficient is forbidden; all
listed vectors begin with the `x⁴` coefficient one.

| `#E(K)` | `a,b` | selected `r`, cofactor | fixed `G`, `Q=[7]G` | `#C(K)` | LMFDB class | fully split distinct rational lines |
| ---: | :--- | :--- | :--- | ---: | :--- | ---: |
| 4036 | `0,112` | `1009,4` | `(4030,3567)`, `(2128,2753)` | 3914 | `3.16.a_a_acj` | 5 |
| 4124 | `0,34` | `1031,4` | `(2705,1890)`, `(23,1463)` | 4178 | `3.16.a_a_bb` | 9 |
| 4126 | `32,63` | `2063,2` | `(62,1499)`, `(2633,1798)` | 4184 | `3.16.a_a_bd` | 7 |

The exact plane quartic vectors are:

- Order 4036: `[1,9,13,10,14,7,15,10,13,7,15,14,2,5,0]`.
- Order 4124: `[1,3,12,3,0,0,2,13,13,6,5,8,0,10,10]`.
- Order 4126: `[1,13,13,0,10,7,11,10,5,10,14,5,15,3,11]`.

For every selected source, `[r]G=[r]Q=O`. The independently reconstructed
quartics are geometrically smooth, have `N1=17` and `N2=257`, and their
Jacobians have the displayed source orders. Rational point-difference
classes reach each selected order-`r` subgroup. These checks give a
factor-base entry point, not an elliptic-to-Jacobian homomorphism. The
five, nine and seven fully split four-point lines cannot alone span a
17-column relation system; other relation types would be necessary.

## Independent full-cohort audit

The verifier uses a point-count identity independent of Sage's elliptic
cardinality routine. For `x≠0`, set `u=1/x²` and `z=y/x`. The equation
becomes `z²+z=x+a+b*u`; absolute trace satisfies
`Tr(x)=Tr(x²)=Tr(u⁻¹)`. With `q=4096`, therefore,

```text
#E(a,b)(K) = q+1 + (-1)^Tr(a) Σ_{u≠0} (-1)^Tr(u⁻¹+b*u).
```

The verifier computes all `b` sums as a 4096-entry Walsh transform after
mapping each `u` to the trace-dual coordinate vector
`(Tr(u*w^0),...,Tr(u*w^11))`. Its 8,190 resulting orders exactly match the
screen population. It recounts each retained quartic by rooting in the
alternate projective charts `(u:1:v)`, `(1:0:v)`, `(0:0:1)`, rather than
the screen's `(1:u:v)` charts. This replay checked all eleven retained
point-count triples.

## Prior work for every matched class

The exact full-polynomial query to LMFDB found all eleven classes. Listed
model counts equal hyperelliptic counts in every row; the three highlighted
labels link to their class records: [4036](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_acj),
[4124](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bb), and
[4126](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bd).

| Source order | LMFDB label | stored models / hyperelliptic | class-B source present? |
| ---: | :--- | ---: | :--- |
| 3978 | `3.16.a_a_aep` | 20 / 20 | no |
| 4008 | `3.16.a_a_adl` | 76 / 76 | yes |
| 4030 | `3.16.a_a_acp` | 56 / 56 | no |
| 4036 | `3.16.a_a_acj` | 156 / 156 | no |
| 4094 | `3.16.a_a_ad` | 128 / 128 | no |
| 4098 | `3.16.a_a_b` | 104 / 104 | yes |
| 4106 | `3.16.a_a_j` | 100 / 100 | yes |
| 4108 | `3.16.a_a_l` | 140 / 140 | no |
| 4124 | `3.16.a_a_bb` | 64 / 64 | no |
| 4126 | `3.16.a_a_bd` | 140 / 140 | no |
| 4140 | `3.16.a_a_br` | 100 / 100 | no |

Order 4106 is a direct class-B positive control with a `2053` prime
subgroup and cofactor two, so it belongs to the prior-known direct binary
GHS family. Exact-label web searches for the three highlighted classes
returned no source identifying these particular quartic equations; that
bounded search cannot establish they were unpublished.

## Transfer-method audit

[Tian's cubic isogeny construction](https://arxiv.org/pdf/2012.07173)
provides a known quotient idea. Its displayed Weil functions and quartic
reconstruction use a full symplectic geometric two-torsion basis, three
nonzero two-torsion points on a short Weierstrass equation, and a classical
bitangent Aronhold system. Ordinary characteristic-two elliptic curves have
one nonzero geometric two-torsion point. Consequently, those **published
coordinate steps cannot be directly run on the three binary sources**.
An odd-degree quotient adapted to characteristic two remains a research
possibility. This methodological gap does not make the overall attack
method new; [Joux--Vitse](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf)
and [Hess's binary Weil descent](https://www.math.uzh.ch/sepp/magma-2.19.8-cr/html/text1383.htm)
already supply related cover and descent routes.

## Attack-cost gate and novelty labels

| Order | new exact curve | new eligible family | new attack method | evaluated subgroup map | full GHS cost vs rho |
| ---: | :--- | :--- | :--- | :--- | :--- |
| 4036 | unresolved outside the catalog | no | no | missing | unknown |
| 4124 | unresolved outside the catalog | no | no | missing | unknown |
| 4126 | unresolved outside the catalog | no | no | missing | unknown |

An eligible vulnerability claim needs an evaluated map `Phi` with
`Phi(G)≠0`, `Phi(Q)=[7]Phi(G)`, a complete relation/decomposition and
target-descent solve, independently replayed scalar, and total same-target
cost below rho. The current results establish none of those attack-cost
conditions. The existing checked Lean subgroup theorem remains conditional
on constructing `Phi`; it is not instantiated by these structural matches.

## Reproduction

```sh
sage -python research/ghs_binary_cubic_isogeny_q16_20261026/screen.py
sage -python research/ghs_binary_cubic_isogeny_q16_20261026/verify.py
```

The raw result binds the protocol and screen SHA-256; the replay certificate
binds the raw result and verifier SHA-256. Elapsed screen time is diagnostic
and is not an attack cost.
