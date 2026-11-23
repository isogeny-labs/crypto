# Post-freeze structured-feature hypothesis

This is a fresh static follow-up, not a change to frozen author v4 or its source pins. No scientific execution or performance claim.

Before eliminating k, each coefficient of the residual cubic is an ordinary quadratic in k,h: P and Q are affine in k,h, the norm is quadratic, and division by the fixed target/anchor factors is linear on coefficients. Thus
T=a*k²+b*k*h+c*k+d*h²+e*h+f
and likewise U (with separate coefficients) over Fp.

Let kappa=2A!=0, c0=z+x+y, and Z=kappa*k=h²−2B1*h−2B0−c0−S. With rho=kappa inverse,
T=(a*rho²)Z²+(b*rho)Z*h+(c*rho+d)Z+d*S+(e+2B1*d)h+f+d*(2B0+c0).
The same identity holds for U. Consequently both maps share only five nonconstant structured features Z²,Z*h,Z,S,h instead of the eight expanded monomials. The primal relaxation has seven coordinates (five features,T,U) and two modular equations (index/determinant p²); its certificate lattice has five congruences and determinant p^5. This is a basis/representation hypothesis, not a surviving-dimension reduction or a proven pruning improvement.

After h=h0+u, write Z=u²+alpha*u+beta−S. Translation preserves the same span Z²,Z*u,Z,S,u,1; translating Z by an integer constant also preserves it. An exact rational enclosure of the quadratic g(u)=u²+alpha*u+beta over0<=u<=H gives Z in[min g−3M,max g]. Safe ranges for Z² and Z*u follow from interval squaring/product (or tighter exact polynomial/Bernstein bounds); these intervals can be signed and cannot reuse the nonnegative-monomial code without modification. A checker must derive them independently and verify the five modular congruences plus the constant.

There is a concrete risk: reduced dimension can enlarge feature heights. Coefficient alpha can have size comparable to p; even short u intervals can then give Z range comparable to p*H and Z² range comparable to p²*H². Independent feature boxing may lose more pruning than the smaller lattice saves. Centering constants does not remove this slope issue. Canonically reducing Z modulo p changes its integer relation to S,u and introduces wrap branches, which must not be silently discarded. A dependency-aware polynomial bound may be required.

Next experiment in a new version: prove the six-term representation coefficient-for-coefficient against the frozen nine-monomial symbolic oracle, independently test small norm/support cases including kappa0, characteristic3 and planted repeated roots, then compare7D and10D certificate discovery on exactly the same predeclared h blocks, with setup/search/checking/serialization and RSS charged. Record feature-height bounds and refuse to infer speed from dimension alone. Known-positive blocks must not close; cap means UNKNOWN. Preserve every source/input and prior failed block.
