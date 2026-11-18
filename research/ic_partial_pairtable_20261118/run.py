#!/usr/bin/env python3
"""EXP-IC-PARTIAL-TABLE-20261118-R2. Public toy ECDLP, standalone, no Sage.

Run: python3 run.py --out raw/primary.json --base 32 --seeds 24 --seed0 0x1c2026111840
"""
import argparse, hashlib, json, math, random, time
P,A,B,R=1009,4,10,1069

class Counter:
    def __init__(self): self.g=self.fp_add=self.fp_mul=self.fp_inv=self.fr_add=self.fr_mul=self.fr_inv=0
    def field(self): return self.fr_add+self.fr_mul+self.fr_inv
    def dict(self): return vars(self).copy()

def add(u,v,c):
    if u is None: return v
    if v is None: return u
    c.g+=1
    x,y=u; z,w=v
    if x==z and (y+w)%P==0: c.fp_add+=1; return None
    if u==v:
        if y==0:return None
        num=(3*x*x+A)%P; den=(2*y)%P;c.fp_mul+=2;c.fp_add+=1
    else:num=(w-y)%P;den=(z-x)%P;c.fp_add+=2
    lam=num*pow(den,-1,P)%P;c.fp_inv+=1;c.fp_mul+=1
    xx=(lam*lam-x-z)%P;yy=(lam*(x-xx)-y)%P;c.fp_mul+=2;c.fp_add+=4
    return (xx,yy)

def mul(k,pt,c):
    if k<0:raise ValueError('negative scalar')
    q=None
    while k:
        if k&1:q=add(q,pt,c)
        k>>=1
        if k:pt=add(pt,pt,c)
    return q

def generator():
    sy={}
    for y in range(P):sy.setdefault(y*y%P,[]).append(y)
    pts=[(x,y) for x in range(P) for y in sy.get((x*x*x+A*x+B)%P,[])]
    assert len(pts)+1==R
    g=pts[0];assert mul(R,g,Counter()) is None
    return g

class Rank:
    def __init__(self,n):
        self.par=list(range(n));self.sz=[1]*n;self.bit=[0]*n;self.odd=[False]*n;self.free=n
    def find(self,x):
        if self.par[x]!=x:
            r,b=self.find(self.par[x]);self.bit[x]^=b;self.par[x]=r
        return self.par[x],self.bit[x]
    def update(self,i,j):
        x,px=self.find(i);y,py=self.find(j)
        if x==y:
            if (px^py)!=1 and not self.odd[x]:self.free-=1;self.odd[x]=True
        else:
            if self.sz[x]<self.sz[y]:x,y=y,x;px,py=py,px
            self.free-=(not self.odd[x])+(not self.odd[y]);self.par[y]=x
            self.bit[y]=px^py^1;self.sz[x]+=self.sz[y];self.odd[x]|=self.odd[y]
            self.free+=(not self.odd[x])
        return len(self.par)-self.free

def graph_solve(rows,n,c):
    t=time.perf_counter_ns();adj=[[] for _ in range(n)]
    for i,j,a in rows:
        adj[i].append((j,a))
        if i!=j:adj[j].append((i,a))
    build=time.perf_counter_ns()-t;t=time.perf_counter_ns()
    from collections import deque
    seen=[False]*n;expressions=[None]*n;out=[None]*n
    inv2=pow(2,-1,R);c.fr_inv+=1
    for root in range(n):
        if seen[root]:continue
        todo=deque([root]);seen[root]=True;expressions[root]=(1,0);comp=[];sol=None
        while todo:
            i=todo.popleft();comp.append(i);si,ti=expressions[i]
            for j,a in adj[i]:
                if not seen[j]:
                    seen[j]=True;expressions[j]=(-si,(a-ti)%R);c.fr_add+=1;todo.append(j)
                else:
                    sj,tj=expressions[j];f=si+sj;res=(a-ti-tj)%R;c.fr_add+=2
                    if f==0:
                        if res:raise ValueError('bad even cycle')
                    else:
                        cand=(res*inv2)%R;c.fr_mul+=1
                        if f==-2:cand=(-cand)%R;c.fr_add+=1
                        if sol is None:sol=cand
                        elif sol!=cand:raise ValueError('bad odd cycle')
        if sol is None:raise ValueError('free component')
        for i in comp:
            si,ti=expressions[i];out[i]=(si*sol+ti)%R;c.fr_mul+=1;c.fr_add+=1
    for i,j,a in rows:
        if (out[i]+out[j])%R!=a:raise ValueError('row inconsistency')
        c.fr_add+=1
    return out,build,time.perf_counter_ns()-t

def rho(g,q,k,seed):
    # 4-partition rho, identical target and seed across all table arms.
    rng=random.Random(seed^0x5bf03619);c=Counter();steps=0
    gq=add(g,q,c)
    for restart in range(60):
        a=rng.randrange(R);b=rng.randrange(R)
        p=add(mul(a,g,c),mul(b,q,c),c);seen={};salt=rng.randrange(1<<24)
        for _ in range(4000):
            if p in seen:
                x,y=seen[p];den=(b-y)%R
                if den:
                    guess=((x-a)*pow(den,-1,R))%R
                    if guess!=k or mul(guess,g,Counter())!=q:raise ValueError('rho wrong result')
                    return {'k':guess,'steps':steps,'restarts':restart,'group_add':c.g}
                break
            seen[p]=(a,b);part=((p[0] if p else 0)^salt)%4
            if part==0:p=add(p,g,c);a=(a+1)%R
            elif part==1:p=add(p,q,c);b=(b+1)%R
            elif part==2:p=add(p,gq,c);a=(a+1)%R;b=(b+1)%R
            else:p=add(p,p,c);a=2*a%R;b=2*b%R
            steps+=1
    raise TimeoutError('rho restarts exhausted')

def trial(g,seed,n):
    rng=random.Random(seed);fbm=Counter();t=time.perf_counter_ns()
    scalars=rng.sample(range(1,R),n);fb=[mul(s,g,fbm) for s in scalars];fb_wall=time.perf_counter_ns()-t
    k=rng.randrange(1,R);q=mul(k,g,Counter())
    start=random.Random(seed^0x72d1).randrange(R)
    indices=[(i,j) for i in range(n) for j in range(i,n)]
    random.Random(seed^0x65f3).shuffle(indices)
    rho_res=rho(g,q,k,seed)
    result={'seed':seed,'B':n,'target':k,'Q':q,'factor_base_scalars':scalars,
            'factor_base_points':fb,'start':start,'rho':rho_res,'arms':{}}
    M=len(indices)
    for fraction in [1.0,0.75,0.50,0.25,1.0]:
        name='aa_full' if fraction==1 and 'full' in result['arms'] else ('full' if fraction==1 else str(fraction))
        count=math.ceil(fraction*M);selected=indices[:count]
        tabc=Counter();t=time.perf_counter_ns();table={}
        for i,j in selected:table.setdefault(add(fb[i],fb[j],tabc),(i,j))
        tab_wall=time.perf_counter_ns()-t
        c=Counter();t=time.perf_counter_ns();walk=mul(start,g,c);a=start
        rnk=Rank(n);rows=[];attempts=0;dependent=0;seen_rows=set()
        while rnk.free and attempts<R:
            ij=table.get(walk)
            if ij is not None:
                i,j=ij;key=(i,j,a)
                if key not in seen_rows:
                    seen_rows.add(key);before=n-rnk.free;after=rnk.update(i,j)
                    dependent+=(before==after);rows.append((i,j,a))
            attempts+=1
            if rnk.free:walk=add(walk,g,c);a=(a+1)%R
        coll_wall=time.perf_counter_ns()-t
        lc=Counter();solution=None;build_ns=la_ns=0;dc=Counter();desc_ns=0;desc_trials=0;recovered=None;pair=None
        err=None
        if rnk.free==0:
            try:
                solution,build_ns,la_ns=graph_solve(rows,n,lc)
                if solution!=scalars:raise ValueError('bad known-answer factor-base solution')
                startd=time.perf_counter_ns();p=q
                for t0 in range(R):
                    desc_trials+=1
                    ij=table.get(p)
                    if ij is not None:
                        i,j=ij;recovered=(solution[i]+solution[j]-t0)%R;pair=[i,j]
                        if recovered!=k or mul(recovered,g,Counter())!=q:raise ValueError('bad target recovery')
                        break
                    if t0+1<R:p=add(p,g,dc)
                desc_ns=time.perf_counter_ns()-startd
                if recovered is None:err='descent exhausted'
            except Exception as exc:err=repr(exc)
        else:err=f'no full rank: {n-rnk.free}/{n}'
        group=fbm.g+tabc.g+c.g+dc.g
        out={'status':'ok' if err is None else 'censored', 'error':err,'fraction':fraction,
            'pair_entries':count,'distinct_pair_sums':len(table),'rank':n-rnk.free,
            'rows':rows,'rows_sha256':hashlib.sha256(json.dumps(rows).encode()).hexdigest(),
            'relation_attempts':attempts,'relations':len(rows),'dependent_rows':dependent,
            'descent_trials':desc_trials,'descent_pair':pair,'recovered_log':recovered,
            'phases':{'factor_base':{'group_add':fbm.g,'wall_ns':fb_wall},
                      'pair_table':{'group_add':tabc.g,'wall_ns':tab_wall},
                      'collection':{'group_add':c.g,'wall_ns':coll_wall},
                      'matrix':{'wall_ns':build_ns,'rows':len(rows),'columns':n},
                      'linear_algebra':{'mod_ops':lc.field(),'wall_ns':la_ns,**lc.dict()},
                      'descent':{'group_add':dc.g,'wall_ns':desc_ns}},
            'total_group_add':group,'total_proxy_w01':group+0.1*lc.field(),
            'total_proxy_w1':group+lc.field()}
        result['arms'][name]=out
    comparable=('status','error','fraction','pair_entries','distinct_pair_sums','rank','rows_sha256','relation_attempts','relations','dependent_rows','descent_trials','descent_pair','recovered_log','total_group_add','total_proxy_w01','total_proxy_w1')
    assert all(result['arms']['full'][key]==result['arms']['aa_full'][key] for key in comparable), 'A/A deterministic metrics differ'
    return result

def main():
    p=argparse.ArgumentParser()
    p.add_argument('--out',required=True);p.add_argument('--base',type=int,default=32)
    p.add_argument('--seeds',type=int,default=24);p.add_argument('--seed0',type=lambda v:int(v,0),default=0x1c2026111840)
    a=p.parse_args();assert 2<=a.base<=R-2
    g=generator();started=time.perf_counter_ns();records=[]
    for i in range(a.seeds):
        try:records.append(trial(g,a.seed0+i,a.base))
        except Exception as e:records.append({'seed':a.seed0+i,'status':'failed','error':repr(e)})
    out={'protocol':'EXP-IC-PARTIAL-TABLE-20261118-R2','curve':{'p':P,'a':A,'b':B,'r':R,'G':g},
         'base':a.base,'seeds':a.seeds,'seed0':a.seed0,'wall_ns':time.perf_counter_ns()-started,
         'records':records}
    with open(a.out,'w') as f:json.dump(out,f,indent=2,sort_keys=True)
    print(json.dumps({'out':a.out,'seeds':a.seeds,'record_count':len(records),
        'errors':[x.get('error') for x in records if 'error' in x],
        'censored':{arm:sum(r['arms'][arm]['status']!='ok' for r in records if 'arms' in r)
                    for arm in ['full','0.75','0.5','0.25']}}))
if __name__=='__main__':main()
