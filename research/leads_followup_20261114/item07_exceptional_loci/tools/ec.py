"""Elliptic-curve arithmetic over F_{p^2}=F_p[s]/(s^2-nr), points as (x,y) pairs of F_{p^2} elements (u,v)=u+v s. None=infinity."""
def mk(p):
    nr=next(n for n in range(2,p) if pow(n,(p-1)//2,p)==p-1)
    class F:
        __slots__=('u','v')
        def __init__(s,u,v=0): s.u=u%p; s.v=v%p
        def __add__(s,o): o=o if isinstance(o,F) else F(o); return F(s.u+o.u,s.v+o.v)
        __radd__=__add__
        def __sub__(s,o): o=o if isinstance(o,F) else F(o); return F(s.u-o.u,s.v-o.v)
        def __rsub__(s,o): return F(o)-s
        def __mul__(s,o): o=o if isinstance(o,F) else F(o); return F(s.u*o.u+nr*s.v*o.v,s.u*o.v+s.v*o.u)
        __rmul__=__mul__
        def __neg__(s): return F(-s.u,-s.v)
        def __eq__(s,o): o=o if isinstance(o,F) else F(o); return s.u==o.u and s.v==o.v
        def __hash__(s): return hash((s.u,s.v))
        def inv(s):
            n=(s.u*s.u-nr*s.v*s.v)%p; ni=pow(n,p-2,p); return F(s.u*ni,-s.v*ni)
        def __truediv__(s,o): o=o if isinstance(o,F) else F(o); return s*o.inv()
        def iszero(s): return s.u==0 and s.v==0
        def __pow__(s,e):
            r=F(1); b=s
            while e:
                if e&1: r=r*b
                b=b*b; e>>=1
            return r
        def __repr__(s): return f'({s.u}+{s.v}s)'
    def sqrt(c):
        # c in F_p -> sqrt in F_{p^2}
        c%=p
        for y in range(p):
            if y*y%p==c: return F(y)
        for y in range(p):
            if y*y*nr%p==c: return F(0,y)
    def add(P,Q,a):
        if P is None: return Q
        if Q is None: return P
        x1,y1=P; x2,y2=Q
        if x1==x2:
            if (y1+y2).iszero(): return None
            l=(3*(x1*x1)+a)/(2*y1)
        else: l=(y2-y1)/(x2-x1)
        x3=l*l-x1-x2; return (x3,l*(x1-x3)-y1)
    return F,sqrt,add
