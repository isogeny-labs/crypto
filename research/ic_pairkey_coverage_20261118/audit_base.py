#!/usr/bin/env python3
"""Independent exact-census and field elimination audit. Does NOT import run.py."""
import hashlib, json, math, random, sys
p=1009;a=4;b=10;n=1069

def affine(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;v,w=Q
    if x==v and (y+w)%p==0:return None
    if x==v:
        if y==0:return None
        m=(3*x*x+a)*pow(2*y,-1,p)%p
    else:m=(w-y)*pow(v-x,-1,p)%p
    X=(m*m-x-v)%p;Y=(m*(x-X)-y)%p
    return (X,Y)

def scalar(k,P):
    O=None
    while k:
        if k%2:O=affine(O,P)
        P=affine(P,P);k//=2
    return O

def row_rank(rows,m):
    vals=[]
    for i,j,alpha in rows:
        v=[0]*(m+1);v[i]=(v[i]+1)%n;v[j]=(v[j]+1)%n;v[-1]=alpha;vals.append(v)
    rank=0
    for col in range(m):
        pivot=next((r for r in range(rank,len(vals)) if vals[r][col]%n),None)
        if pivot is None:continue
        vals[rank],vals[pivot]=vals[pivot],vals[rank]
        inv=pow(vals[rank][col],-1,n)
        vals[rank]=[(v*inv)%n for v in vals[rank]]
        for r in range(len(vals)):
            if r==rank:continue
            fac=vals[r][col]
            if fac:vals[r]=[(x-fac*y)%n for x,y in zip(vals[r],vals[rank])]
        rank+=1
        if rank==m:break
    if rank==m:
        assert all(vals[i][-1]==scalars[i] for i in range(m)) # assigned in caller
    return rank

def audit(path):
    global scalars
    data=json.load(open(path));g=tuple(data['curve']['G']);assert scalar(n,g) is None
    assert n>2 and all(n%d for d in range(2,math.isqrt(n)+1))
    allpoints=[(x,y) for x in range(p) for y in range(p) if (y*y-x*x*x-a*x-b)%p==0]
    assert len(allpoints)+1==n
    counts={'cases':0,'arms':0,'passed':0,'failures':[],'censored':0}
    for record in data['records']:
        counts['cases']+=1;seed=record['seed'];m=record['B'];scalars=record['factor_base_scalars'];fb=list(map(tuple,record['factor_base_points']))
        assert len(set(fb))==m;assert all(scalar(x,g)==q for x,q in zip(scalars,fb))
        k=record['target'];Q=tuple(record['Q']);assert scalar(k,g)==Q
        assert record['rho']['k']==k and record['rho']['group_add']>0
        indices=[(i,j) for i in range(m) for j in range(i,m)]
        random.Random(seed^0x65f3).shuffle(indices)
        for name,arm in record['arms'].items():
            counts['arms']+=1
            try:
                M=math.ceil(arm['fraction']*len(indices));assert M==arm['pair_entries']
                tbl={}
                for i,j in indices[:M]:tbl.setdefault(affine(fb[i],fb[j]),(i,j))
                assert len(tbl)==arm['distinct_pair_sums']
                expected=[]
                for off in range(arm['relation_attempts']):
                    t=(record['start']+off)%n;key=scalar(t,g)
                    if key in tbl:
                        i,j=tbl[key];expected.append((i,j,t))
                raw=[tuple(row) for row in arm['rows']]
                assert raw==expected, (seed,name,'relation trace mismatch')
                assert arm['relations']==len(raw)
                assert hashlib.sha256(json.dumps(arm['rows']).encode()).hexdigest()==arm['rows_sha256']
                assert all(affine(fb[i],fb[j])==scalar(t,g) and (scalars[i]+scalars[j])%n==t for i,j,t in raw)
                rank=row_rank(raw,m);assert rank==arm['rank']
                assert arm['phases']['matrix']['rows']==len(raw) and arm['phases']['matrix']['columns']==m
                p0=arm['phases'];summed=sum(p0[phase]['group_add'] for phase in ['factor_base','pair_table','collection','descent'])
                assert summed==arm['total_group_add']
                assert abs(arm['total_proxy_w01']-(summed+.1*p0['linear_algebra']['mod_ops']))<1e-5
                assert arm['total_proxy_w1']==summed+p0['linear_algebra']['mod_ops']
                if arm['status']=='ok':
                    assert rank==m and arm['recovered_log']==k
                    assert arm['descent_trials']>=1 and arm['descent_trials']<=n
                    i,j=arm['descent_pair'];shift=arm['descent_trials']-1
                    assert (scalars[i]+scalars[j]-shift)%n==k
                    assert affine(fb[i],fb[j])==affine(Q,scalar(shift,g))
                    counts['passed']+=1
                else:counts['censored']+=1
            except Exception as e:counts['failures'].append({'seed':seed,'arm':name,'error':repr(e)})
    print(json.dumps({'file':path,**counts},sort_keys=True));assert not counts['failures']
if __name__=='__main__':
    for f in sys.argv[1:]:audit(f)
