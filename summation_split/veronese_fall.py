"""Filtered direct Macaulay row-span diagnostic (not F4 or iterated LFD)."""
from itertools import combinations_with_replacement
import sympy as s
from .veronese import system,geometric_system

def rank(rows,p):
    piv={}
    for row in rows:
        row={k:v%p for k,v in row.items() if v%p}
        while row:
            k=max(row);c=row[k]
            if k not in piv:
                iv=pow(c,-1,p);piv[k]={j:v*iv%p for j,v in row.items()};break
            for j,v in piv[k].items():
                t=(row.get(j,0)-c*v)%p
                if t:row[j]=t
                else:row.pop(j,None)
    return len(piv)

def macaulay(eq,variables,p,D):
    rows=[];n=len(variables)
    for f in eq:
        P=s.Poly(f,*variables,modulus=p)
        if P.is_zero:continue
        d=P.total_degree();terms=[(mon,int(c)) for mon,c in P.terms()]
        for k in range(D-d+1):
            for inds in combinations_with_replacement(range(n),k):
                mul=[0]*n
                for i in inds:mul[i]+=1
                rows.append({tuple(a+b for a,b in zip(mon,mul)):c for mon,c in terms})
    return rows

def diagnostic(eq,variables,p,cap=3):
    old=0
    for D in range(2,cap+1):
        rows=macaulay(eq,variables,p,D)
        full=rank(rows,p);top=rank(({k:v for k,v in row.items() if sum(k)==D} for row in rows),p)
        # dim(V_D intersect R_{<=D-1}) minus dim(V_{D-1}), the direct new fall.
        yield D,len(rows),full,top,full-top-old
        old=full

if __name__=='__main__':
    for p,a,b,m in [(5,0,1,3),(7,1,1,3),(11,1,1,4),(19,2,3,5),(2147483647,1,1,3),(2147483647,1,1,4),(2147483647,1,1,5)]:
        x=s.symbols('x:'+str(m+1));aa,ee,_=system(p,a,b,x);ga,ge=geometric_system(a,b,x)
        for arm,aux,eq in [('veronese',aa,ee),('geo',ga,ge)]:
            for row in diagnostic(eq,[*x,*aux],p):print(p,m,arm,*row,sep=',',flush=True)
