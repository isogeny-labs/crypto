import json,sys
from consist import rank
for line in open(sys.argv[1]):
    d=json.loads(line); r,cc,ct=d['r'],d['cc'],d['ct']
    L=[];Tr=[]
    for ents,rhs in d['rows']:
        v=[0]*cc
        for c,x in ents: v[c]=x%r
        (Tr if v[ct] else L).append(v)
    nt=[c for c in range(cc) if c!=ct]
    rL=rank([[v[c] for c in nt] for v in L],r)
    dL=len(nt)-rL
    rT=rank(L+Tr,r)
    print(d['q'],d['seed'],d['attempt'],'dL',dL,'t_rows',len(Tr),'inc_from_t_rows',rT-rL,'need',dL+1,'ok' if rT==cc else 'FAIL')
