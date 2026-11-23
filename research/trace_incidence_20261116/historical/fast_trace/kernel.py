"""Complete split-divisor translation: blocked trace and paper-style dense routes.

Public API: translate(p,a,b,u,v,Q,method='trace',input_infinity=False,nonsquare=None).
Polynomials are constant-first lists; Q is an affine (x,y) tuple or None.
Return {'components':[{'u':[...],'v':[...]}], 'infinity':bool, 'stats':...}.

Preconditions: p>3 is a prime fitting in 63 bits; E:y^2=x^3+a*x+b is
nonsingular; u is monic, split and squarefree, and v^2=h mod u. Only one
point per x occurs in this simple input divisor. Optional input_infinity
adds O to it. Prime/split are preconditions, not silently certified here.

Both routes use python-flint for native field/polynomial/matrix arithmetic.
Trace uses BSGS power projection, a Hankel-to-convolution trace functional,
and native quadratic Berlekamp-Massey. Thus this is NOT a claim to achieve
the paper's proposed modular-composition exponent: recurrence recovery is
O(d^2), and the actual FLINT matrix multiplication backend also matters.
Dense uses the full multiplication matrix's characteristic polynomial and
the paper's dense inverse-composition solve. No roots/factor call is used.

General-p collision lifting uses deterministic least-nonsquare search when
beta is not supplied. Its search is charged but has no unconditional
polylogarithmic deterministic worst-case guarantee. Supplied beta is checked.
"""

import ctypes
import math
import os
from pathlib import Path
import subprocess
import sys
import time

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'math_vendor'))
import flint
from flint import nmod_poly as Poly, nmod_mat as Mat
flint.ctx.threads=1

_HERE=Path(__file__).resolve().parent
_RECURRENCE=None


def _native():
    global _RECURRENCE
    if _RECURRENCE is None:
        source=_HERE/'recurrence.cpp';binary=_HERE/'recurrence.so'
        if not binary.exists() or binary.stat().st_mtime<source.stat().st_mtime:
            subprocess.run(['g++','-O3','-std=c++17','-shared','-fPIC',str(source),'-o',str(binary)],check=True,capture_output=True)
        lib=ctypes.CDLL(str(binary));f=lib.recurrence
        U=ctypes.c_uint64
        f.argtypes=[ctypes.POINTER(U),ctypes.c_size_t,U,ctypes.POINTER(U),ctypes.c_size_t]
        f.restype=ctypes.c_int
        _RECURRENCE=(lib,f)
    return _RECURRENCE[1]


def initialize():
    """Compile/load the recurrence helper outside algorithm timing, disclosed setup."""
    _native()


def _coeff(poly):return [int(c) for c in poly]


def _monic(f,p):
    return f*pow(int(f.leading_coefficient()),-1,p) if f else f


def _inverse(f,u,p):
    g,s,_=f.xgcd(u)
    if g.degree()!=0:raise ValueError('nonunit denominator')
    return (s*pow(int(g[0]),-1,p))%u


def _const(p,c):return Poly([c%p],p)


def _trace_series(u,p,length):
    d=u.degree();rev=u.reverse(degree=d)
    inverse=rev.inverse_series_trunc(length)
    tail=(rev.derivative()*inverse).truncate(max(0,length-1))
    return (_const(p,d)-tail.left_shift(1)).truncate(length)


def _dual(w,series,d,p):
    # Tr(w X^k)=sum_i w_i Tr(X^(i+k)): one polynomial convolution.
    c=series*w.reverse(degree=d-1)
    return [int(c[d-1+k]) for k in range(d)]


def power_project(u,phi,g,p,stats=None):
    """Return Tr(phi^k),Tr(g phi^k), k=0..2d-1, using BSGS."""
    d=u.degree();L=2*d;m=math.isqrt(L-1)+1;q=(L+m-1)//m
    stats={} if stats is None else stats
    t=time.process_time()
    series=_trace_series(u,p,2*d-1)
    babies=[_const(p,1)]
    for _ in range(1,m):babies.append((babies[-1]*phi)%u)
    giant=(babies[-1]*phi)%u
    rows=[];w=_const(p,1);wg=g%u
    for _ in range(q):
        rows.append(_dual(w,series,d,p));rows.append(_dual(wg,series,d,p))
        w=(w*giant)%u;wg=(wg*giant)%u
    A=Mat(rows,p)
    BT=Mat([[int(baby[k]) for baby in babies] for k in range(d)],p)
    stats['power_projection_setup_cpu']=time.process_time()-t
    t=time.process_time();M=A*BT
    stats['power_projection_matmul_cpu']=time.process_time()-t
    aa=[];bb=[]
    for i in range(q):
        aa.extend(int(M[2*i,j]) for j in range(m))
        bb.extend(int(M[2*i+1,j]) for j in range(m))
    stats['power_projection_block']=m
    stats['power_projection_matrix_shape']=[2*q,d,m]
    return aa[:L],bb[:L]


def _recurrence(seq,p):
    U=ctypes.c_uint64
    source=(U*len(seq))(*seq);dest=(U*(len(seq)+1))()
    size=_native()(source,len(seq),p,dest,len(seq)+1)
    if size<0:raise RuntimeError('recurrence output capacity')
    return Poly([dest[i] for i in range(size)],p)


def _trace_numerator(q,moments,p):
    n=q.degree()
    # Reverse the first n moments, multiply, then retain the high n terms.
    rev=Poly(list(reversed(moments[:n])),p)
    return (q*rev).right_shift(n)


def reconstruct_trace(u,phi,g,p,stats=None):
    stats={} if stats is None else stats
    moments,weighted=power_project(u,phi,g,p,stats)
    t=time.process_time();q=_recurrence(moments,p)
    stats['recurrence_cpu']=time.process_time()-t
    inv=_inverse(q.derivative(),q,p)
    mult=(_trace_numerator(q,moments,p)*inv)%q
    single=_monic(q.gcd(mult-1),p);double=_monic(q.gcd(mult-2),p)
    if single*double!=q:raise ValueError('image multiplicities exceed promised 1 or 2')
    ordinates=(_trace_numerator(q,weighted,p)*inv)%q
    return single,double,ordinates%single


def reconstruct_dense(u,phi,g,p,stats=None):
    stats={} if stats is None else stats
    d=u.degree();x=Poly([0,1],p)
    t=time.process_time();cols=[];power=phi%u
    for _ in range(d):cols.append(_coeff(power)+[0]*(d-len(power)));power=(x*power)%u
    matrix=Mat([[cols[j][i] for j in range(d)]for i in range(d)],p)
    stats['dense_matrix_setup_cpu']=time.process_time()-t
    t=time.process_time();chi=matrix.charpoly()
    stats['dense_charpoly_cpu']=time.process_time()-t
    double=_monic(chi.gcd(chi.derivative()),p)
    single=_monic(chi//(double*double),p)
    if single.degree()<=0:return single,double,Poly([],p)
    t=time.process_time()
    preimage=_monic(u.gcd(single.compose_mod(phi,u)),p)
    degree=preimage.degree();cols=[];power=_const(p,1)
    for _ in range(degree):
        cols.append(_coeff(power)+[0]*(degree-len(power)))
        power=(power*phi)%preimage
    A=Mat([[cols[j][i] for j in range(degree)]for i in range(degree)],p)
    rhs=g%preimage
    result=A.solve(Mat([[int(rhs[i])]for i in range(degree)],p))
    v=Poly([int(result[i,0])for i in range(degree)],p)
    stats['dense_inverse_composition_cpu']=time.process_time()-t
    return single,double,v


def _sqrt_split(h,u,p,nonsquare,stats):
    """Deterministic square root in a split etale algebra; no factorization."""
    if u.degree()<=0:return Poly([],p)
    if p%4==3:
        v=h.pow_mod((p+1)//4,u)
        if (v*v-h)%u:raise ValueError('collision polynomial does not lift')
        return v
    beta=nonsquare
    trials=0
    if beta is None:
        beta=2
        while pow(beta,(p-1)//2,p)!=p-1:beta+=1;trials+=1
        trials+=1
    elif pow(beta,(p-1)//2,p)!=p-1:raise ValueError('supplied beta is not a nonsquare')
    stats['nonsquare_trials']=trials;stats['nonsquare']=int(beta)
    odd=p-1;s=0
    while odd%2==0:s+=1;odd//=2
    parts=[(u,0)]
    for k in range(s):
        next_parts=[]
        for part,j in parts:
            value=(h*pow(beta,-j,p))%part
            pos=_monic(part.gcd(value.pow_mod((p-1)//(1<<(k+1)),part)-1),p)
            neg=_monic(part//pos,p)
            if pos.degree()>0:next_parts.append((pos,j))
            if neg.degree()>0:next_parts.append((neg,j+(1<<k)))
        parts=next_parts
    factors=[]
    for part,j in parts:
        if j&1:raise ValueError('collision abscissa has no rational ordinate')
        v=((h*pow(beta,-j,p))%part).pow_mod(pow(2,-1,odd),part)*pow(beta,j//2,p)
        factors.append((part,v%part))
    # Product-tree style pairwise CRT, avoiding a degree-d linear solve.
    while len(factors)>1:
        combined=[]
        for i in range(0,len(factors),2):
            if i+1==len(factors):combined.append(factors[i]);continue
            f,v=factors[i];q,w=factors[i+1]
            v=v+f*((_inverse(f%q,q,p)*(w-v))%q)
            combined.append((f*q,v))
        factors=combined
    v=factors[0][1]%u
    if (v*v-h)%u:raise RuntimeError('split square root reconstruction failed')
    return v


def point_add(p,a,P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;z,w=Q
    if x==z:
        if (y+w)%p==0:return None
        slope=(3*x*x+a)*pow(2*y,-1,p)%p
    else:slope=(w-y)*pow(z-x,-1,p)%p
    xx=(slope*slope-x-z)%p
    return xx,(slope*(x-xx)-y)%p


def translate(p,a,b,u,v,Q,method='trace',input_infinity=False,nonsquare=None):
    """Translate a simple split divisor, handling all exceptional support points."""
    started=time.process_time();wall=time.perf_counter()
    if p<=3 or p>=1<<63:raise ValueError('prime p must satisfy 3<p<2^63')
    a%=p;b%=p
    if (4*a*a*a+27*b*b)%p==0:raise ValueError('singular curve')
    u=Poly(u,p);v=Poly(v,p)
    if not u or int(u.leading_coefficient())!=1:raise ValueError('u must be monic')
    if u.gcd(u.derivative()).degree()!=0:raise ValueError('u must be squarefree')
    h=Poly([b,a,0,1],p)
    if (v*v-h)%u:raise ValueError('input divisor is not on curve')
    if method not in ('trace','dense'):raise ValueError('unknown method')
    stats={'method':method,'input_degree':u.degree(),'nonsquare_trials':0,
           'recurrence_algorithm':'native quadratic BM' if method=='trace' else None}
    components=[];infinity=False
    def point(P):
        nonlocal infinity
        if P is None:infinity=True
        else:components.append({'u':[(-P[0])%p,1],'v':[P[1]%p]})
    if Q is None:
        if u.degree()>0:components.append({'u':_coeff(u),'v':_coeff(v%u)})
        infinity=bool(input_infinity)
    else:
        Q=(int(Q[0])%p,int(Q[1])%p)
        if (Q[1]*Q[1]-Q[0]**3-a*Q[0]-b)%p:raise ValueError('Q is not on curve')
        if input_infinity:point(Q)
        if u.degree()>0 and int(u(Q[0]))==0:
            P=(Q[0],int(v(Q[0])))
            point(point_add(p,a,P,Q))
            u=u//Poly([-Q[0]%p,1],p);v=v%u
            stats['exceptional_point_extracted']=True
        if u.degree()>0:
            slope=((v-Q[1])*_inverse(Poly([-Q[0]%p,1],p),u,p))%u
            phi=(slope*slope-Poly([Q[0],1],p))%u
            gg=(-Q[1]+slope*(Q[0]-phi))%u
            t=time.process_time()
            reconstruct=reconstruct_trace if method=='trace' else reconstruct_dense
            single,double,sv=reconstruct(u,phi,gg,p,stats)
            stats['image_reconstruction_cpu']=time.process_time()-t
            if single.degree()>0:components.append({'u':_coeff(single),'v':_coeff(sv)})
            t=time.process_time()
            if double.degree()>0:
                dv=_sqrt_split(h,double,p,nonsquare,stats)
                components.extend([{'u':_coeff(double),'v':_coeff(dv)},
                                   {'u':_coeff(double),'v':_coeff(-dv)}])
            stats['collision_lift_cpu']=time.process_time()-t
            stats['singleton_degree']=single.degree();stats['double_degree']=double.degree()
    stats['complete_cpu_seconds']=time.process_time()-started
    stats['complete_wall_seconds']=time.perf_counter()-wall
    return {'components':components,'infinity':infinity,'stats':stats}
