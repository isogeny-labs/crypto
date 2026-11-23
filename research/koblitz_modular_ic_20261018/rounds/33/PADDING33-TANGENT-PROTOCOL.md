# Full padding tangent diagnostic, prospective

The relative Jacobian has rank3 at both padding points, but those points were
not in R32's degree83/284 total-Jacobian panel. Read R32 and Stacks29.35.14.
To distinguish a total tangent dimension1 from an isolated affine solution,
use the accepted cleared four-source polynomials in F2[n,l,v,u]. Differentiate
ALL FOUR variables n,l,v,u before point evaluation; a derivative with u held
in the coefficient field is insufficient. At both fixed degree5 points check
source values0, clearing D/H units, full4x4 determinant and all sixteen3x3
minors with exact unit-ideal lift when available. Report observed rank3 or4
without requiring a favorable rank. Since J'=D*J and J vanishes at the point,
Jac(J')=D*Jac(J), with D a checked unit. No smoothness/dimension/proper-binding
claim follows solely from this tangent result. Native4GiB/1200s, no statistics.
