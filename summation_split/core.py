"""Exact odd-prime Semaev S3 and fixed-degree specialized resultants.

Python 3 + SymPy fallback. No factor-base map occurs in this package.
"""
from __future__ import annotations
from functools import lru_cache
import sympy as sp


def admissible(p: int, a: int, b: int) -> bool:
    return p > 3 and sp.isprime(p) and (4*a**3 + 27*b*b) % p != 0


def s3(x, y, z, a: int, b: int):
    return ((x-y)**2*z*z - 2*((x+y)*(x*y+a)+2*b)*z
            + (x*y-a)**2 - 4*b*(x+y))


def det_mod(rows, p):
    """Small dense Sylvester determinant, entirely in Fp, including degenerate leading terms."""
    M = [[int(x) % p for x in row] for row in rows]
    n = len(M)
    val = 1
    for k in range(n):
        pivot = next((i for i in range(k,n) if M[i][k]), None)
        if pivot is None:
            return 0
        if pivot != k:
            M[k], M[pivot] = M[pivot], M[k]
            val = -val
        v = M[k][k]
        val = val*v % p
        vi = pow(v,-1,p)
        for i in range(k+1,n):
            f = M[i][k]*vi % p
            for j in range(k+1,n):
                M[i][j] = (M[i][j]-f*M[k][j]) % p
            M[i][k] = 0
    return val % p


def fixed_resultant(f_ascending, g_ascending, p):
    """Fixed homogeneous-degree Sylvester resultant, NOT reduced-degree resultant.

    Keeping zero high coefficients is essential for exceptional/infinity fibers.
    """
    m, n = len(f_ascending)-1, len(g_ascending)-1
    f = list(reversed(f_ascending))
    g = list(reversed(g_ascending))
    rows = []
    for j in range(n):
        rows.append([0]*j + f + [0]*(n-j-1))
    for j in range(m):
        rows.append([0]*j + g + [0]*(m-j-1))
    return det_mod(rows, p)


def s3_coeff_z(x, y, a, b, p):
    """S3(x,y,z), degree two in z even if leading coeff is zero."""
    return [((x*y-a)**2 - 4*b*(x+y))%p,
            (-2*((x+y)*(x*y+a)+2*b))%p,
            (x-y)**2%p]


def s3_coeff_first(z, y, a, b, p):
    """S3(t,y,z), degree two in t even when degenerate."""
    return s3_coeff_z(y,z,a,b,p)


def interpolate(points, p):
    """Lagrange interpolation for coefficients ascending, over Fp."""
    out=[0]*len(points)
    for i,(xi,yi) in enumerate(points):
        li=[1]
        den=1
        for j,(xj,_) in enumerate(points):
            if i==j:continue
            den=den*(xi-xj)%p
            nxt=[0]*(len(li)+1)
            for k,c in enumerate(li):
                nxt[k]=(nxt[k]-xj*c)%p
                nxt[k+1]=(nxt[k+1]+c)%p
            li=nxt
        scale=yi*pow(den,-1,p)%p
        for k,c in enumerate(li):out[k]=(out[k]+scale*c)%p
    return out


def eval_poly(coeffs, x, p):
    v=0
    for c in reversed(coeffs):v=(v*x+c)%p
    return v


def specialized_summation(p, a, b, inputs):
    """Return S_{m+1}(x1,...,xm,X) polynomial in X over Fp.

    Uses fixed-degree resultant and Fp interpolation; p MUST exceed final
    X-degree so interpolation defines the whole polynomial.
    """
    m=len(inputs)
    degree=2**(m-1)
    if p <= degree:raise ValueError(f"p={p} must exceed interpolation degree {degree}")
    f=s3_coeff_z(inputs[0],inputs[1],a,b,p)
    for x in inputs[2:]:
        target_degree=2*(len(f)-1)
        pts=[]
        for z in range(target_degree+1):
            # Each polynomial is in the shared eliminated variable t.
            g=s3_coeff_first(z,x,a,b,p)
            pts.append((z,fixed_resultant(f,g,p)))
        f=interpolate(pts,p)
    return f


@lru_cache(None)
def s4_symbolic(a: int,b: int):
    X1,X2,X3,X4,T=sp.symbols('x1 x2 x3 x4 t')
    # Symmetric split avoids possible misleading reduced-degree numerical res.
    out=sp.resultant(s3(X1,X2,T,a,b),s3(X3,X4,T,a,b),T)
    return sp.expand(out), (X1,X2,X3,X4)
