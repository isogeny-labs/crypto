"""Bit-level check of the symbolic R_1..R_4 against macdump round-2/3 dumps of the real system (A, n, s)."""
import sys,json,os; sys.path.insert(0,'.'); sys.path.insert(0,'../../L9-124-falls-structure/src')
from krel import field, subspaces, kmono, trace_rows
from an9 import load, basis
from evalsym import evalrels
from sys9 import base
n,s=int(sys.argv[1]),int(sys.argv[2]); k=(n+1)//2; pref=os.path.join(os.environ['TMPDIR'],'l12d','n%d_A_s%d'%(n,s))
mul,frob,trm=field(n); V,V2=subspaces(n,k,s,'A'); m,_,z,a6=base(n,k,s)
S=json.load(open('../analysis/sym_rref.json')); R=evalrels(S['new_raw'],n,m,z,a6)
monos,W2=load(pref,2); _,W3=load(pref,3); idx={mm:i for i,mm in enumerate(monos)}
B2=basis(W2); B3=basis(W3)
def parse(l):
    if l=='1': return (),()
    return tuple(int(t[1:]) for t in l.split('*') if t[0]=='a'),tuple(int(t[1:]) for t in l.split('*') if t[0]=='b')
per=[]; allrows=[]
for r in R:
    Q={}
    for l,c in r.items():
        for mm,cc in kmono(*parse(l),V,V2,k,mul,frob).items(): Q[mm]=Q.get(mm,0)^mul(c,cc)
    rows=trace_rows({a:b for a,b in Q.items() if b},n,idx,mul,trm)
    per.append(dict(in_W3=sum(1 for x in rows if B3.red(x)==0),in_W2=sum(1 for x in rows if B2.red(x)==0)))
    allrows+=rows
B=basis(W2); d0=len(B)
for x in allrows: B.add(x)
out=dict(n=n,s=s,dimW2=d0,dimW3=len(B3),new_total=len(B3)-d0,per_relation=per,span_mod_W2=len(B)-d0,pred=4*n)
print(json.dumps(out))
