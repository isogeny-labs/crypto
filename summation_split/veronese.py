"""Compact chart-complete cubic S3 tensor, no factor-base map."""
import sympy as s
from itertools import permutations


def tensor(A,B,C,a,b):
    # Each node is (x^2, x*z, z^2), in homogeneous Kummer coordinates.
    nodes=(A,B,C)
    out=0
    for i in range(3):
        j,k=[v for v in range(3) if v!=i]
        out += (nodes[j][0]*nodes[k][0]*nodes[i][2]
                -2*nodes[i][0]*nodes[j][1]*nodes[k][1]
                -2*a*nodes[j][1]*nodes[k][1]*nodes[i][2]
                -4*b*nodes[i][1]*nodes[j][2]*nodes[k][2])
    return out+a*a*A[2]*B[2]*C[2]


def system(p,a,b,xs):
    """Return auxiliaries, cubic equations; xs are m+1 affine expressions.
    No auxiliary field equations needed for geometric projection. For Fp
    points, field-restrict inputs when needed and recover rational witnesses.
    """
    m=len(xs)-1
    d=next(k for k in range(2,p) if pow(k,(p-1)//2,p)==p-1)
    w=s.Symbol('omega')
    aux=[]; eq=[]; leaves=[]; inner=[]
    for i,x in enumerate(xs):
        q=s.Symbol(f'q{i}'); aux.append(q); eq.append(q-x*x)
        leaves.append((q,x,1))
    for i in range(m-2):
        u,v,z,r,h=s.symbols(f'u{i} v{i} z{i} r{i} h{i}')
        aux.extend((u,v,z,r,h))
        eq.extend((z*(z-1),(1-z)*(u-1),(1-z)*v,
                   r-u*u-d*v*v,h-2*u*v))
        # z=0 forces (u,v)=(1,0); z=1 is the affine chart.
        # uv_chart = (u+z-1)+v*omega, by the normalization equations.
        inner.append((r+h*w,u+z-1+v*w,z))
    for i in range(m-1):
        left=leaves[0] if i==0 else inner[i-1]
        right=leaves[i+1]
        out=leaves[-1] if i==m-2 else inner[i]
        t=s.Poly(s.expand(tensor(left,right,out,a,b)),w)
        rem=t.rem(s.Poly(w*w-d,w))
        eq.extend((rem.nth(0),rem.nth(1)))
    return aux,eq,d


def geometric_system(a,b,xs,forms=None,parameter=None):
    """Geometric auxiliary witnesses only; original inputs can be field-restricted.
    When forms are supplied, share one square across the affine line.
    """
    m=len(xs)-1;aux=[];eq=[];leaves=[];inner=[]
    if forms is not None:
        q=s.Symbol('q_shared');aux.append(q);eq.append(q-parameter**2)
        leaves=[(c*c*q+2*c*k*parameter+k*k,x,1) for (c,k),x in zip(forms,xs)]
    else:
        for i,x in enumerate(xs):
            q=s.Symbol(f'q{i}');aux.append(q);eq.append(q-x*x);leaves.append((q,x,1))
    for i in range(m-2):
        u,z,r=s.symbols(f'u{i} z{i} r{i}');aux.extend((u,z,r))
        eq.extend((z*(z-1),(1-z)*(u-1),r-u*u));inner.append((r,u+z-1,z))
    for i in range(m-1):
        left=leaves[0] if i==0 else inner[i-1]
        out=leaves[-1] if i==m-2 else inner[i]
        eq.append(s.expand(tensor(left,leaves[i+1],out,a,b)))
    return aux,eq
