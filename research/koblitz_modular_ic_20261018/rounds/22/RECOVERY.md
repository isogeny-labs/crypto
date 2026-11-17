# Exact integral recovery on the norm-zero factors

The requested calculation succeeds: the least positive recoverable scalar
is **1**. This is the actual Round19/G25 map, restricted to integral
augmentation factors. No bound on the unknown coefficients is imposed.
All numbers below are exact, rather than measurements of IC cost.

Work in O=Z[w],w²+w+2=0. Write

```
R(v0,v1,v2,v3)=(-v0-v1-v2-v3,v0,v1,v2),
J(v)=(v3,v2,v1,v0,-v0-v1-v2-v3),
P(t)=(t3,t2,t1,t0).
```

The native producer checks PJ=I,Pi J=JR,and Phi_j=J M_j for every
actual five-row G25 block. Thus M_j=P Phi_j uses an integral augmentation
basis with a checked orientation. The corresponding literal bindings to
the frozen Round20 rows and Round21 rotation are also kernel checked.

Each commuting map below is displayed as the coefficient tuple of
1,R,R²,R³. These tuples specify the complete four-by-four CM matrices.

| j | M_j coefficients |
|---|---|
|0|14−7w,−7w,21,7−7w|
|1|2+w,7,0,6+2w|
|2|10+4w,0,0,15|
|3|7,0,0,0|

One small integral equivariant recovery uses only M1 and M3=7I:

```
B0=B2=0,
B1=(1+3w)+(2+5w)R+3R²+(1+2w)R³,
B3=(-2+4w)-3R²+(-2+3w)R³.
```

The actual CM products and their independent8-by8 integer expansions give
B1 M1+B3 M3=I. Both matrices commute with R,and all22 both-sign order11
fixtures pass using only these two images. The construction inverts M1
modulo7 and divides I−B1 M1 by7 exactly;it is not a coefficient search.
Its compact universal integer identity is,with t representing R:

```
b(t,w)*m(t,w)+7*c(t,w)-1
 =(3+5t+8t³+10t⁴+4t⁶)*(w²+w+2)
  +((-19+32w)+(20−4w)t+(-2+10w)t²)*(1+t+t²+t³+t⁴).
```

Here m,b,c are the displayed M1,B1,B3 polynomials. The native data are
`runs/g33-small-two-map-recovery.json`;the universal identity and actual
matrix bindings have separate Lean proofs. Their final kernel/audit status
is recorded in VALIDATION.md. This is a smaller representative of the same
complete Smith solution lattice;it changes neither the actual maps nor
the original weighted four-map polarization.

The separate coefficient-pairing construction gives this second recovery:

| j | B_j coefficients |
|---|---|
|0|−4845−6314w,−2408−11991w,−602−6888w,−6300−12649w|
|1|19531+5448w,2+5w,3,1+2w|
|2|4315+11393w,0,0,0|
|3|4932+836w,0,0,0|

Direct multiplication gives sum_j B_j M_j=I,and every B_j and M_j
commutes with R. The full matrices,all transformations and fixtures are
in `runs/g32-normzero-integral-recovery.json`;the independent integer
regular-representation replay is `g32-independent-integer-replay.json`.
The latter retains the 24-column integral kernel basis and the inherited
polarization check. Eight source-bound IDC records expand the actual
matrix products acting on eight integer coordinates. They are evaluated
over the certificate format's odd prime;the separate Lean kernel proofs
and exact native integer products establish the integer statements.

## Completeness of the calculation

The columns e0,Re0,R²e0,R³e0 form the integral triangular matrix

```
 1 -1  0  0
 0  1 -1  0
 0  0  1 -1
 0  0  0  1
```

Its inverse has ones on and above the diagonal. A commuting O-linear map
is therefore uniquely a polynomial of degree less than four in R,with
coefficients determined by its value on e0. Expanding four such unknown
polynomials in 1,w produces the complete 8-by-32 integer system Hx=e0.
The native Smith calculation uses 241 elementary operations and retains
U,V,their integer inverses,and UHV=D. All eight nonzero Smith factors are
1. Thus the image lattice is all of Z^8,with no torsion obstruction.

The stored right inverse W satisfies HW=I8. Every solution of Hx=t has
the form Wt+(z−WHz),z in Z^32;Lean proves both directions. The last24
columns of V give a nonredundant kernel basis K,and the Smith identities
give the equivalent form x=Wt+Kq,q in Z^24. The universal commutant and
solution-parameterization proofs make explicit why this is complete.

The independent control constructs a commuting recovery from the old
unrestricted integral recovery using the perfect coefficient pairing on
O[T]/(1+T+T²+T³+T⁴). For lambda=coefficient of T³,the pairing Gram matrix
in 1,T,T²,T³ is

```
0 0  0  1
0 0  1 -1
0 1 -1  0
1 -1 0  0
```

It is unimodular. Restrict the old recovery to J and let l_j be its
blocks. If a_k=lambda(l_j(R^k e0)),the representing polynomial has
coefficients (a0+a1+a2+a3,a0+a1+a2,a0+a1,a0). Associativity of the
coefficient pairing and sum l_j M_j=I give sum B_j M_j=I. These are the
second displayed coefficient table. The first,larger Smith solution is
retained in `recovery-attempt01` and `Smith_B`;no failed search is hidden.

Controls scale H by2 and obtain least scalar2,make H zero and obtain no
positive scalar,and corrupt one recovery coefficient and reject it.
All22 order11 controls over F32 pass,eleven per ordinary Koblitz sign.
Coefficient w acts by Frobenius for a=0 and negative Frobenius for a=1.
The actual source tuples also satisfy the checked descent rotation.

## Polarization and geometric scope

The induced metric is G=J^t J=I+ones(4),with Smith diagonal1,1,1,5.
The actual weights45,315,63,171 satisfy
sum_j weight_j M_j^dagger G M_j=33075 H_old=3*105² H_old.
The native verifier rejects substituting the identity metric for G.
Lean checks the metric,the determinant5 and the actual weighted identity.
The trace-zero targets therefore retain their inherited polarization.

Under the standard written CM-morphism and Galois-descent bridge,the
matrix identity gives an integral equivariant retraction on the actual
trace-zero source presentation. As a written geometric consequence,once
the CM matrices are interpreted as the specified abelian-variety morphisms,
the map into the four trace-zero targets is a split closed immersion over F2:
the left inverse makes it a monomorphism on every functor of points,and
properness of the abelian source makes that proper monomorphism a closed
immersion. This argument uses the actual fixed map and its recovered
polarization;it is not a formalized schemes theorem in this Lean project.
R21's denominator5 theorem remains true
for regular target cycles. A retraction on their norm-zero subobjects
need not extend to a retraction on the regular cycles. This result does
not construct the final105/19 polarized quotient,its243 sections,or a
map into the level77 modular Jacobian.
