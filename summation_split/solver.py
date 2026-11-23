"""Optional finite-field Groebner checks using SymPy Buchberger, not F4.

Infinity charts are explicit disjunctions. Total process time is the sum of
all branches, even inconsistent ones, when run without an external timeout.
"""
import itertools
import sympy as sp
from .core import s3,specialized_summation
from .field import Field

def split(expr,w,d):
    pp=sp.Poly(sp.expand(expr),w)
    return (sp.expand(sum(c*d**(k//2) for (k,),c in pp.terms() if k%2==0)),
            sp.expand(sum(c*d**(k//2) for (k,),c in pp.terms() if k%2==1)))

def projective_charts(p,a,b,xs):
    """Yield disjoint infinity/finite charts of the chain over Fp2.

    Each finite chart S3 has total degree <=4 in the original x and
    intermediate coordinates. Membership polynomials have degree p and are
    separately counted; their inclusion is essential for exact Fp solutions.
    """
    m=len(xs);F=Field(p);w=sp.Symbol('omega');X=sp.Symbol('X')
    for bits in itertools.product((False,True),repeat=m-2):
        params=[];t=[];field_eq=[]
        for i,finite in enumerate(bits):
            if finite:
                u,v=sp.symbols(f'u{i} v{i}');params.extend((u,v));t.append(u+v*w)
                field_eq.extend((u**p-u,v**p-v))
            else:t.append(None)
        equations=[]
        def add_expr(E):equations.extend(split(E,w,F.d))
        if t[0] is None:
            equations.append(sp.Integer(xs[0]-xs[1]))
        else:add_expr(s3(xs[0],xs[1],t[0],a,b))
        for i in range(1,len(t)):
            left=t[i-1];right=t[i];leaf=xs[i+1]
            if left is None:
                if right is None:equations.append(sp.Integer(1))
                else:add_expr(right-leaf)
            elif right is None:add_expr(left-leaf)
            else:add_expr(s3(left,leaf,right,a,b))
        if t[-1] is None: equations.append(X-xs[-1])
        else:add_expr(s3(t[-1],xs[-1],X,a,b))
        equations=equations+field_eq+[X**p-X]
        yield bits,params+[X],equations

def groebner_roots(equations,variables,p):
    """Lex Groebner plus univariate elimination; must verify on full gate."""
    G=sp.groebner(equations,*variables,order='lex',modulus=p)
    X=variables[-1]
    eliminate=[f.as_expr() for f in G.polys if not (f.as_expr().free_symbols-{X})]
    if not eliminate:
        raise ArithmeticError('no X elimination polynomial: cannot certify roots')
    roots={z for z in range(p) if all(int(q.subs(X,z))%p==0 for q in eliminate)}
    deg=max((sp.Poly(g.as_expr(),*variables).total_degree() for g in G.polys),default=0)
    return roots,deg,len(G.polys)

def direct_gb(p,a,b,xs):
    X=sp.Symbol('X');cs=specialized_summation(p,a,b,xs)
    f=sum(int(c)*X**i for i,c in enumerate(cs))
    return groebner_roots((f,X**p-X),(X,),p)

def chained_gb(p,a,b,xs):
    roots=set();basis_degree=0;branches=0
    for bits,vs,eqs in projective_charts(p,a,b,xs):
        rb,db,_=groebner_roots(eqs,vs,p)
        roots|=rb;basis_degree=max(basis_degree,db);branches+=1
    return roots,basis_degree,branches

def symmetric_s4_gb(p,a,b,xs):
    """Full elementary symmetric S4 with quadratic ties (m=3 only)."""
    if len(xs)!=3:raise NotImplementedError('symbolic symmetric terminal only derived for S4')
    from .symmetric import terminal_s4,ties
    X=sp.Symbol('X')
    f,subs,_=terminal_s4(a,b)
    eqs,ee=ties([sp.Integer(x) for x in xs]+[X])
    f=f.xreplace({sym:ee[i] for i,(sym,_) in enumerate(subs)})
    vars_=sorted(set().union(*(g.free_symbols for g in eqs))-{X},key=str)+[X]
    return groebner_roots(eqs+[f,X**p-X],vars_,p)

def linear_gb(p,a,b,xs):
    """A linear slack re-coordinate: no total-degree reduction expected."""
    X,z=sp.symbols('X z');cs=specialized_summation(p,a,b,xs)
    f=sum(int(c)*z**i for i,c in enumerate(cs))
    return groebner_roots((f,z-X,X**p-X),(z,X),p)
