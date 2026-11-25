import json, sys, itertools, numpy as np, bisect, math, cmath
sys.path.insert(0, '.')
import cnum as C
exec(open('dev/t4.py').read().split("rng=np.random")[0])
# table of Jacobi formula for all azygetic triples (sorted tuple order)
tab={}
for n1,n2,n3 in itertools.combinations(C.ODD,3):
    if C.q(C.cadd(n1,n2,n3))!=0: continue
    D=np.linalg.det(np.array([grad[n1],grad[n2],grad[n3]]))/np.pi**3
    res=find5(D)
    assert len(res)==1,(n1,n2,n3,res)
    tab[(n1,n2,n3)]=res[0]
print('table',len(tab))
import pickle; pickle.dump(tab,open('dev/jacobi_tab.pkl','wb'))
def Dval(th,n1,n2,n3):
    trip=(n1,n2,n3); srt=tuple(sorted(trip, key=C.ODD.index))
    # permutation sign
    perm=[srt.index(x) for x in trip]
    sg=1
    for i in range(3):
        for j in range(i+1,3):
            if perm[i]>perm[j]: sg=-sg
    es,k=tab[srt]
    return sg*(-1)**(k//4)*np.prod([th[e] for e in es])
ar=C.find_aronhold()
def amat(th):
    m=ar
    A=np.zeros((3,3),dtype=complex)
    for r,k in enumerate((4,5,6)):
        A[r,0]=Dval(th,m[k],m[1],m[2])/Dval(th,m[3],m[1],m[2])
        A[r,1]=Dval(th,m[0],m[k],m[2])/Dval(th,m[0],m[3],m[2])
        A[r,2]=Dval(th,m[0],m[1],m[k])/Dval(th,m[0],m[1],m[3])
    return A/A[:,:1]
H,Atrue=C.weber_frame([grad[m] for m in ar])
print('jacobi A vs gradient A', np.abs(amat(val)-Atrue).max())
cnt=0;same=0
for bits in itertools.product((1,-1),repeat=7):
    if np.prod(bits)!=1: continue
    s=dict(zip(C.BITS3,(1,)+bits))
    th={c:val[c]*s[c[0]] for c in C.EVEN}
    cnt+=1
    if np.abs(amat(th)-Atrue).max()<1e-8: same+=1
print('patterns',cnt,'giving the true Aronhold matrix',same)
same=0
for bits in itertools.product((1,-1),repeat=7):
    s=dict(zip(C.BITS3,(1,)+bits))
    th={c:val[c]*s[c[0]] for c in C.EVEN}
    if np.abs(amat(th)-Atrue).max()<1e-8: same+=1
print('all 128 group patterns: same',same)
rg=np.random.default_rng(5); same=0
for t in range(50):
    th={c:val[c]*rg.choice([1,-1]) for c in C.EVEN}
    if np.abs(amat(th)-Atrue).max()<1e-8: same+=1
print('random per-char flips: same',same,'/50')
# multiplicities
m=ar
from collections import Counter
for r,k in enumerate((4,5,6)):
    for trips in [((m[k],m[1],m[2]),(m[3],m[1],m[2])),((m[0],m[k],m[2]),(m[0],m[3],m[2])),((m[0],m[1],m[k]),(m[0],m[1],m[3]))]:
        cn=Counter()
        for t in trips:
            srt=tuple(sorted(t,key=C.ODD.index)); cn.update(tab[srt][0])
        print(sorted(cn.values()), end=' ')
print()
