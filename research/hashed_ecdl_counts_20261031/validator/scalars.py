import json,sys
def parse_icv1(s):
    parts=s.split(':'); p=int(parts[1].split('-')[1]); return p,parts
def add(P,Q,a,p):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P;x2,y2=Q
    if x1==x2:
        if (y1+y2)%p==0: return None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else: l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p; return (x3,(l*(x1-x3)-y1)%p)
def mul(k,P,a,p):
    R=None
    while k:
        if k&1: R=add(R,P,a,p)
        P=add(P,P,a,p); k>>=1
    return R
tot=0;bad=0;curves={}
for name,pth in [('C-10-14','/dev/shm/restore/snapshot-1730/sessions/C-10-14/records.jsonl'),('C-16','/dev/shm/restore/v2/sessions/C-16/records.jsonl'),('B','/dev/shm/restore/snapshot-1730/sessions/B/records.jsonl')]:
    for l in open(pth):
        r=json.loads(l)
        if r['outcome']['status']!='verified': continue
        c=r['workload']['curve']; p,parts=parse_icv1(c['icv1'])
        a=int(parts[2]); b=int(parts[4])  # guess: ICV1:fp-p:a:r? check on-curve below
        G=tuple(int(x,16) for x in c['generator']); Q=tuple(int(x,16) for x in r['workload']['target'])
        key=(c['slug'],)
        if key not in curves:
            if G[0]==Q[0]: continue
            a_=((G[1]**2-G[0]**3)-(Q[1]**2-Q[0]**3))*pow(G[0]-Q[0],-1,p)%p
            b_=(G[1]**2-G[0]**3-a_*G[0])%p
            ok=(a_,b_) if mul(c['r'],G,a_,p) is None and mul(c['r']-1,G,a_,p)==(G[0],(-G[1])%p) else None
            curves[key]=(ok,c['icv1'],c['r'])
        if key not in curves: continue
        ab=curves[key][0]
        if ab is None: print('could not determine a,b for',c['icv1']); sys.exit(1)
        d=int(r['outcome']['recovered']); 
        res=mul(d,G,ab[0],p)
        tot+=1
        if res!=Q: bad+=1
print('curves',curves)
print('verified rows independently checked [d]G==Q:',tot,'failures:',bad)
