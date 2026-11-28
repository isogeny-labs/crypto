# Validation boundary — iteration 10

Date 2026-11-23 UTC. Mathematical source was published at commit `b1d3da5dbcd0b21f5616060df314e22c5fec59bc` before the symbolic algebra cross-check below. **All evidence here is same-session, unsigned and PCC-unverified.** The original curve group arithmetic was not executed or replaced by Python; this is only a formal-polynomial residual check, not a native cryptographic test.

## Symbolic algebra, second attempt passed

Using installed SymPy in the local analysis environment, executed:

```python
import sympy as sp
u,v,r,a,b,y,z=sp.symbols('u v r a b y z')
fu=u**3+a*u+b; fv=v**3+a*v+b
s3=(u-v)**2*r**2-2*((u+v)*(u*v+a)+2*b)*r+(u*v-a)**2-4*b*(u+v)
A=(r+u+v)*(u-v)**2
num=sp.expand((A-(y-z)**2)*(A-(y+z)**2)-(u-v)**2*s3)
remy=sp.Poly(num,y).rem(sp.Poly(y*y-fu,y)).as_expr()
remz=sp.Poly(remy,z).rem(sp.Poly(z*z-fv,z)).as_expr()
diag_s3=sp.expand(s3.subs(v,u))
diag_should=-4*fu*r + ((3*u**2+a)**2-8*u*fu)
C=sp.Matrix([[a*a-4*b*r,-2*(a*r+2*b),r*r],
             [-2*(a*r+2*b),-2*(r*r+a),-2*r],
             [r*r,-2*r,1]])
fC=sp.expand((sp.Matrix([[1,u,u*u]])*C*sp.Matrix([1,v,v*v]))[0])
print("offdiag residual:",sp.factor(remz))
print("diag residual:",sp.factor(diag_s3-diag_should))
print("rank-three pencil residual:",sp.factor(s3-fC))
```

Exact stdout:

```text
offdiag residual: 0
diag residual: 0
rank-three pencil residual: 0
```

The first attempt tried to coerce an expression still containing (u-v)^(-2) into a polynomial over `ZZ[u,v,r,a,b]` and failed with `CoercionFailed`. It produced **no validation result or file**. The successful retry first cleared the denominator and then reduced modulo the two curve equations. It does not address subgroup membership, root isolation, sign replay, element collisions, output completeness or algorithmic complexity.

## Independent Rust test queue (not yet executed)

On a source-frozen odd-order small-field original Rust curve, compare the exact (diagonal, off-diagonal) factors with genuine point addition for every pair-spectrum root. Build `H`, `diag`, `off`, `Delta` and `Q` over the original source arithmetic. For **every** target R, including O as a special path, check that the support of Q equals the independently enumerated signed four-multiset sums and the ordered A^4 solver, and that `Delta` and the resultants agree exactly (not merely in vanishing support). Validate negative gcd/Bezout certificates and full root-to-leaf signed replay, including repeated labels and collisions. For D>=4 freeze an actually missed R with `det(M)=0` and all entries nonzero. Explicitly test affine zero-prefix restrictions, P,P,P,-P, diagonal/2-torsion guards, and Fp² projected-ghost negative controls. Record each test count, executable SHA, stdout/stderr, memory, failures and stage cost once the native environment is admitted.

**Do not call a symbolic residual an independent validator, do not assume the tests passed and do not promote claims without signed verification.**
