import json,sys,collections
sys.path.insert(0,'.')
from nullsp import *
from planeclass import load
CL=(599,600,602,607)
def fit(p,deg=3):
    R=load(p); pts=[(r['alpha'],r['beta']) for r in R if r.get('dim6') in CL]
    m=monos(2,deg); N=nullspace(evalmat(pts,m,p),p)
    return m,N,pts,R
if __name__=='__main__':
    p=int(sys.argv[1]); m,N,pts,R=fit(p)
    print(p,len(pts),'null',len(N))
    if len(N)==1:
        v=N[0]; v=v*pow(int(v[m.index((2,1))]),p-2,p)%p
        print({m[i]:int(v[i]) for i in range(len(m)) if v[i]})
        allp=[(r['alpha'],r['beta'],r.get('dim6')) for r in R if r.get('dim6') is not None]
        Z=(evalmat([(a,b) for a,b,_ in allp],m,p)@v)%p==0
        print(collections.Counter(c for (a,b,c),z in zip(allp,Z) if z))
