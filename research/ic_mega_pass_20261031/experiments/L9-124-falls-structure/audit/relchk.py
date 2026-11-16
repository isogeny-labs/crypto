import sys, json; sys.dont_write_bytecode=True
from aud_check import *
n=31; s=2; k=16
m,V,z,a6=draw(n,s)
pref='../dump9/n31_A_s2'
monos,R2=load(pref,2); _,R3=load(pref,3); idx={mm:i for i,mm in enumerate(monos)}
B2=rank_basis(R2); B3=rank_basis(R3)
trm=0
for t in range(n): trm|=trace(1<<t,m,n)<<t
def poly(lbl):
    if lbl=='1': return {0:1}
    p={0:1}
    for tok in lbl.split('*'):
        blk=0 if tok[0]=='a' else 1; i=int(tok[1:])
        f={1<<(j+k*blk):frob(V[j],i,m,n) for j in range(k)}
        q={}
        for m1,c1 in p.items():
            for m2,c2 in f.items(): q[m1|m2]=q.get(m1|m2,0)^mulmod(c1,c2,m,n)
        p={a:b for a,b in q.items() if b}
    return p
d=json.load(open('/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/L9-124-falls-structure/analysis/newrel_n31_A_s2.json'))
allrows=[]
for rel in d['new_relations']:
    Q={}
    for lbl,c in rel.items():
        for mm,cc in poly(lbl).items(): Q[mm]=Q.get(mm,0)^mulmod(int(c,16),cc,m,n)
    Q={a:b for a,b in Q.items() if b}
    rows=[]
    for t in range(n):
        r=0
        for mm,c in Q.items():
            cc=mulmod(1<<t,c,m,n)
            if bin(cc&trm).count('1')&1: r^=1<<idx[mm]
        rows.append(r)
    def inspan(B,r):
        while r:
            lb=r&-r; q=B.get(lb)
            if q is None: return False
            r^=q
        return True
    print('terms',len(Q),'traces in W3:',sum(inspan(B3,r) for r in rows),'/',n,' in W2:',sum(inspan(B2,r) for r in rows),'/',n, 'maxdeg', max(bin(mm).count('1') for mm in Q))
    allrows+=rows
print('F2 dim of all trace comps of 4 new relations mod W2:', rank(R2+allrows)-rank(R2), ' mod W3:', rank(R3+allrows)-rank(R3))
