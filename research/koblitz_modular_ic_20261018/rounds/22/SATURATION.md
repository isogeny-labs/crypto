# Coupled node187 exclusion and the missing polynomial certificate

Q41 excludes the four Q40 scalar points supported only on node187,
under the explicit proper-specialization/DVR/pole bridges below. This is
a simultaneous valuation argument for the actual functions. It is not
an extracted polynomial separator,an h-power membership certificate or
whole-ideal saturation. Those requested deliverables remain open.

## Exact native source binding

For each of the four unchanged Q8 programs,the first four ordinary Miller
functions are F_j=P_j(x_j)+y_j S_j(x_j). The native expansion checks the
entire rational function,including its denominator: D_j=1,deg P_j=9,
deg S_j=8,and S_j is monic. Each of the five coefficient conjugates is
recorded and checked,80 functions in total. All360 split coefficient
components are retained;coefficient sigma permutes them and fixes the arc
parameter. It is not absolute Frobenius on a Laurent-series field.

The actual gates are F0=node126,F1=node186,F2=node246,and F3=node306.
Node127 equals F0 because node15 is the constant1. Node187=F0 F1 and
node247=F0 F1 F2. Q41 independently matches the complete addition and
inverse-gate circuits for all five point-descent residual pairs;the
program's complete node/residual lists are retained in its JSON. It also
checks that every old model has exactly the nonzero support [187].

## Proper pole geometry

On E_a:y²+xy=x³+a x²+1,the projective chart at O has t=X/Y,z=Z/Y and
z+t z=t³+a t²z+z³. Hence z=t³ times a unit of constant1,x has leading
t^-2 and y has leading t^-3. The polynomial P_j has pole at most18,
whereas the monic y S_j has exact pole19,with leading coefficient1.
Since D_j=1,F_j has no finite poles. On any nonconstant proper source
arc,F_j has negative valuation exactly when P_j specializes to O.
This last valuation/curve assertion is a written geometric bridge;Lean
does not silently assume a formalized elliptic-scheme implementation.

For local parameters t_j=c_j T^m_j+...,the leading coefficient of
F_j is c_j^-19. The leading coefficient of F0 F1 is
(c0 c1)^-19. If h=eta T^N+...,a scalar normalized limit1 on node187
would require N=19(m0+m1) and eta(c0 c1)^-19=1 in every coefficient
component and each of its five requested conjugates. No independent
nineteenth-root argument for either summand is substituted for this
product condition. The contradiction below does not need to solve those
leading-coefficient equations.

## Simultaneous valuation contradiction

Suppose an old scalar point lies in the reduced projective closure of
the full h!=0 source. Curve selection and normalization provide a DVR
arc,allowing ramified reparametrization. Write v(h)=N>0 and v(F_i)=v_i
in each split component. Every projective coordinate is regular. Since
the individual F0 and F1 coordinates have residue0 and node187 has
residue1,

```
v0>-N,   v1>-N,   v0+v1=-N.
```

Both v0 and v1 are negative. Thus P0=P1=O in every proper residue
component. The actual descent gives

```
P1=sigma(P0)-sigma(P3),
P2=sigma(P1)-sigma(P3).
```

The first equality forces sigma(P3)=O;the second forces P2=O. Therefore
v2<0. But the actual retained three-function coordinate satisfies

```
v(h*node247)=N+v0+v1+v2=v2<0,
```

contradicting its regularity. This uses simultaneous proper reductions
and complete coefficient descent. Extra Kummer,inverse and residual
blocks cannot restore an arc once this subsystem is contradictory.

Lean proves this implication for an arbitrary nonempty component index
set,an arbitrary component permutation and an additive residue group,
with all pole equivalences and proper-specialization identities stated
as hypotheses. It also proves the two valuation inequalities separately.
Native controls check129024 finite integer assignments. The uncoupled
assignment N=2,v0=v1=-1,v2=1 satisfies the product normalization;it fails
the proper-descent/pole bridge. This positive control shows why scalar or
single-factor models alone do not settle the coupled question.

## Exact remaining obligation

The argument proves that these four scalar points cannot be complete
source limits under the written bridges. It does not enumerate all
geometric boundary points. It has not produced an explicit homogeneous
polynomial f separating these points with a checked h^k f in I,and it
has not constructed any complete simultaneous arc. Extracting such a
polynomial certificate remains part of the original request. Full
quotient saturation and the proper polarized quotient embedding remain
separate obligations. The earlier finite Q40 models remain frozen and
valid models of their stated necessary system.
