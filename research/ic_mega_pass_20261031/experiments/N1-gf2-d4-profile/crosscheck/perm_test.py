import sys,random,json
sys.path.insert(0,'.')
from lib import *
n,k,N=31,16,32
out=[]
for s,a in [(1,'C1'),(0,'D1'),(2,'A')]:
    eqs=system(n,k,s)[a]
    base=call('closure',N,4,eqs)
    for t in range(2):
        pr=random.Random(100+t); perm=list(range(N)); pr.shuffle(perm)
        pe=[{sum(1<<perm[i] for i in range(N) if m>>i&1) for m in e} for e in eqs]
        r=call('closure',N,4,pe)
        out.append(dict(arm=a,s=s,perm=t,base=(base['low'],base['dim'],base['one']),perm_res=(r['low'],r['dim'],r['one'])))
        print(out[-1],flush=True)
json.dump(out,open('../crosscheck/perm_test.json','w'),indent=1)
