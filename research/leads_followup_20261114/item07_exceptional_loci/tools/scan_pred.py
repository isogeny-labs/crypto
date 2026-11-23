import sys,json,collections,itertools
sys.path.insert(0,'.')
from feats import *
p=19; F,sqrt,add=mk(p)
D=json.load(open('../data/alphabeta19.json'))
rows=[]
for a,b,c in D:
    d=features(p,a,b,F,sqrt,add); d['cls']=c; rows.append(d)
json.dump(rows,open('../data/feats19.json','w'),default=str)
names=[k for k in rows[0] if k not in('cls','alpha','beta')]
def hashable(x): return x if not isinstance(x,list) else tuple(x)
def report(target,label):
    T=[r for r in rows if r['cls'] in target]; G=[r for r in rows if r['cls']==605]
    res=[]
    for k in names+['alpha','beta']:
        vals=set(hashable(r.get(k)) for r in rows)
        for v in vals:
            tp=sum(1 for r in T if hashable(r.get(k))==v); fp=sum(1 for r in G if hashable(r.get(k))==v)
            if tp: res.append((tp-fp*0.0-fp,tp,fp,k,v))
    res.sort(key=lambda x:(-(x[1]/(x[1]+x[2])),-x[1]))
    print(label,'n_target',len(T),'n_generic',len(G))
    for r in sorted(res,key=lambda x:(x[2]/(x[1]),-x[1]))[:8]: print('   ',r[3],'==',r[4],'TP',r[1],'FP',r[2])
report({794},'794'); report({602,599,607,600},'Qclass'); report({603,600},'tan0')
