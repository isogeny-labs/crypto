"""Evaluate the symbolic relations (analysis/sym_rref.json, coefficients in F2(z,c)) at an instance (K,z,a6) -> dict label->int."""
import json, cypari2; pari=cypari2.Pari()
def ffK(n,m):
    pol='+'.join('x^%d'%i for i in range(n+1) if m>>i&1); g=pari('ffgen(Mod(1,2)*(%s),\'x)'%pol); return g
def toff(g,v,n): return sum((g**i for i in range(n) if v>>i&1), 0*g)
def toint(e,n):
    p=pari.lift(pari.Pol(pari.ffembed if False else e.polrep() if hasattr(e,'polrep') else e)) if False else None
    pol=pari('(e)->lift(e.pol)')(e)
    return sum(int(pari.lift(pari.polcoef(pol,i)))<<i for i in range(n))
def evalrels(rels,n,m,z,c):
    g=ffK(n,m); Zv=toff(g,z,n); Cv=toff(g,c,n)
    f=pari('(s,Z,C)->subst(subst(s,z,Z),c,C)')
    return [{lbl:toint(f(pari('Mod(1,2)*('+s+')'),Zv,Cv),n) for lbl,s in r.items()} for r in rels]
