import sys, random
sys.dont_write_bytecode=True
sys.path.insert(0, "/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/N3-capacity-lp-converse")
from n3lib import fpoly
for p in (13,19):
    for s in range(4):
        rng=random.Random(p) if s==0 else random.Random(5000*s+p)
        f,(a,b,xR)=fpoly(3,p,rng)
        print(p,s,(a,b,xR),'rhs(xR)=',(xR**3+a*xR+b)%p,'disc',(4*a**3+27*b**2)%p)
# which seed family gives npts 253 at p=13 twice?
import itertools
from n3lib import evalf
def npts(p,rng):
    f,_=fpoly(3,p,rng); return sum(1 for pt in itertools.product(range(p),repeat=3) if evalf(f,list(pt),p)==0)
for name,fn in (("1000s+p",lambda s,p:1000*s+p),("p+s",lambda s,p:p+s),("5000s+p",lambda s,p:5000*s+p),("s",lambda s,p:s)):
    print(name,[npts(13,random.Random(fn(s,13))) for s in (0,1,2)])
