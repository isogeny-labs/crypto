import sys,random,json; sys.path.insert(0,'.')
import zmodel
from zmodel import *
from cmpl9 import newrels
K=Field(31); rng=random.Random(5)
z=rng.getrandbits(31); c1=rng.getrandbits(31); c2=rng.getrandbits(31)
for c in (c1,c2,0):
    st,R2,R3,_=closure(K,z,c,0,4,range(4)); new,_,_=newrels(K,R2,R3); print(st['dimW2win'],st['dimW3win'],len(new), new[0] if new else None)
# independent coefficients: monkeypatch Fj
orig=zmodel.Fj
ind={}
def Fi(K,z,c,j,lo):
    P=orig(K,z,c,j,lo)
    key=lambda t: ind.setdefault(t,rng.getrandbits(31))
    A=lambda p: 1<<(p-lo)
    P[(A(j),A(j))]=key(('z',j)); P[(A(j+1),0)]=key(('z',j+1)); P[(0,A(j+1))]=key(('z',j+1)); P[(0,0)]=key(('c',j)); return P
zmodel.Fj=Fi
st,R2,R3,_=zmodel.closure(K,z,c1,0,4,range(4)); print('indep coeffs',st)
