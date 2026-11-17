import sys,json
from kker import *
n,name,s,pref=31,'A',2,'/tmp/claude-501/l9d/n31_A_s2'
k=16; mul,frob,trm=field(n); V,V2=subspaces(n,k,s,name)
monos,_=load(pref,0); idx={m:i for i,m in enumerate(monos)}
WB=basis(load(pref,3)[1])
d=json.load(open('../analysis/kker_n31_A_s2_w0123_r3.json'))
def parse(lbl):
    if lbl=='1': return (),()
    Ia=tuple(int(t[1:]) for t in lbl.split('*') if t[0]=='a'); Ib=tuple(int(t[1:]) for t in lbl.split('*') if t[0]=='b'); return Ia,Ib
for rel in d['relations'][:4]:
    Q={}
    for lbl,c in rel.items():
        p=kmono(*parse(lbl),V,V2,k,mul,frob)
        for mm,cc in p.items(): Q[mm]=Q.get(mm,0)^mul(int(c,16),cc)
    Q={m:c for m,c in Q.items() if c}
    bad=sum(1 for r in trace_rows(Q,n,idx,mul,trm) if WB.red(r)); print(len(Q),bad)
