#!/usr/bin/env python3
"""Independent Fp group, row trace, matrix, cost and rho audit; never imports run.py."""
import hashlib,json,math,random,sys
p,a,b,n=1009,4,10,1069

def add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;v,w=Q
    if x==v and (y+w)%p==0:return None
    if x==v:
        if y==0:return None
        m=((3*x*x+a)*pow(2*y,-1,p))%p
    else:m=((w-y)*pow(v-x,-1,p))%p
    X=(m*m-x-v)%p;Y=(m*(x-X)-y)%p
    return X,Y

def scalar(k,P):
    acc=None
    while k:
        if k&1:acc=add(acc,P)
        k//=2
        if k:P=add(P,P)
    return acc

def scalar_count(k,P):
    c=0;acc=None
    while k:
        if k&1:
            if acc is not None and P is not None:c+=1
            acc=add(acc,P)
        k//=2
        if k:
            if P is not None:c+=1
            P=add(P,P)
    return acc,c

def candidate_order(seed,m):
    # Independently constructed round-robin cycles; no producer import.
    order=list(range(m));random.Random(seed^0x9bb2).shuffle(order)
    pairs=[];seen=set()
    for k in range(1,1+m//2):
        shell=[]
        for t in range(m):
            edge=tuple(sorted([order[t],order[(t+k)%m]]))
            if edge not in seen:
                seen.add(edge);shell.append(edge)
        random.Random(seed^0x9bb2^(k*65537)).shuffle(shell)
        pairs+=shell
    pairs.extend((i,i) for i in order)
    assert len(pairs)==m*(m+1)//2==len(set(pairs))
    return pairs

def random_order(seed,m):
    pairs=[(i,j) for i in range(m) for j in range(i,m)]
    random.Random(seed^0x65f3).shuffle(pairs)
    return pairs

def dense_rank(rows,m,logs):
    v=[]
    for i,j,val in rows:
        row=[0]*(m+1)
        row[i]=(row[i]+1)%n;row[j]=(row[j]+1)%n;row[-1]=val
        v.append(row)
    rk=0
    for col in range(m):
        pivot=next((i for i in range(rk,len(v)) if v[i][col]),None)
        if pivot is None:continue
        v[rk],v[pivot]=v[pivot],v[rk]
        inv=pow(v[rk][col],-1,n)
        v[rk]=[val*inv%n for val in v[rk]]
        for i in range(len(v)):
            if i==rk:continue
            factor=v[i][col]
            if factor:
                v[i]=[(val-factor*other)%n for val,other in zip(v[i],v[rk])]
        rk+=1
        if rk==m:break
    if rk==m:assert [v[i][-1] for i in range(m)]==logs
    return rk

def audit_rho(g,q,k,seed,expected):
    rng=random.Random(seed^0x5bf03619)
    group_add=0;steps=0;gq=add(g,q);group_add+=1
    for restart in range(60):
        aa=rng.randrange(n);bb=rng.randrange(n)
        p1,x=scalar_count(aa,g);p2,y=scalar_count(bb,q)
        group_add+=x+y
        if p1 is not None and p2 is not None:group_add+=1
        P=add(p1,p2); seen={};salt=rng.randrange(1<<24)
        for _ in range(4000):
            if P in seen:
                a0,b0=seen[P];den=(bb-b0)%n
                if den:
                    v=((a0-aa)*pow(den,-1,n))%n
                    assert v==k and scalar(v,g)==q
                    assert {'k':v,'steps':steps,'restarts':restart,'group_add':group_add}==expected
                    return
                break
            seen[P]=(aa,bb)
            part=((P[0] if P else 0)^salt)%4
            if P is not None:group_add+=1
            if part==0:P=add(P,g);aa=(aa+1)%n
            elif part==1:P=add(P,q);bb=(bb+1)%n
            elif part==2:P=add(P,gq);aa=(aa+1)%n;bb=(bb+1)%n
            else:P=add(P,P);aa=2*aa%n;bb=2*bb%n
            # Exactly one group add iff both input points are nonidentity.
            # Correctly inspect pre-step point; recompute from preceding seen key.
            steps+=1
    raise AssertionError('rho replay failed')

def audit_file(path):
    d=json.load(open(path));g=tuple(d['curve']['G'])
    assert d['curve']=={'p':p,'a':a,'b':b,'r':n,'G':list(g)}
    assert n>2 and all(n%t for t in range(2,math.isqrt(n)+1))
    assert scalar(n,g) is None
    allpoints=[(x,y) for x in range(p) for y in range(p) if (y*y-x*x*x-a*x-b)%p==0]
    assert len(allpoints)+1==n
    audit={'path':path,'seeds':0,'arms':0,'passed':0,'censored':0,'rho_replayed':0,'failures':[]}
    for r in d['records']:
        audit['seeds']+=1
        if 'arms' not in r:
            audit['failures'].append({'seed':r['seed'],'error':'producer failed: '+str(r.get('error'))});continue
        seed=r['seed'];m=r['B'];scalars=r['factor_base_scalars'];fb=[tuple(z) for z in r['factor_base_points']]
        assert len(scalars)==m and len(set(fb))==m
        assert all(scalar(s,g)==point for s,point in zip(scalars,fb))
        k=r['target'];q=tuple(r['Q']);assert scalar(k,g)==q
        try:audit_rho(g,q,k,seed,r['rho']);audit['rho_replayed']+=1
        except Exception as e:audit['failures'].append({'seed':seed,'arm':'rho','error':repr(e)})
        initial=r['start'];_,starting_adds=scalar_count(initial,g)
        fb_adds=sum(scalar_count(s,g)[1] for s in scalars)
        for name,arm in r['arms'].items():
            audit['arms']+=1
            try:
                method='distance' if name=='distance' else 'random'
                order=candidate_order(seed,m) if method=='distance' else random_order(seed,m)
                selected=order[:math.ceil(arm['fraction']*len(order))]
                assert len(selected)==arm['pair_entries']
                table={}
                for i,j in selected:table.setdefault(add(fb[i],fb[j]),(i,j))
                assert len(table)==arm['distinct_pair_sums']
                attempts=arm['relation_attempts']
                expected_rows=[]
                for t in range(attempts):
                    x=(initial+t)%n
                    if scalar(x,g) in table:
                        i,j=table[scalar(x,g)]
                        expected_rows.append((i,j,x))
                rows=[tuple(z) for z in arm['rows']]
                assert rows==expected_rows,('row trace mismatch',seed,name)
                assert arm['relations']==len(rows)
                assert hashlib.sha256(json.dumps(arm['rows']).encode()).hexdigest()==arm['rows_sha256']
                assert all(add(fb[i],fb[j])==scalar(x,g) and (scalars[i]+scalars[j])%n==x for i,j,x in rows)
                rank=dense_rank(rows,m,scalars)
                assert rank==arm['rank']
                phases=arm['phases']
                assert phases['factor_base']['group_add']==fb_adds
                assert phases['pair_table']['group_add']==len(selected)
                running=scalar(initial,g);expected_collection=starting_adds
                for _ in range(attempts if arm['rank']<m else attempts-1):
                    if running is not None:expected_collection+=1
                    running=add(running,g)
                assert phases['collection']['group_add']==expected_collection, (seed,name,'walk adds')
                assert phases['matrix']['rows']==len(rows) and phases['matrix']['columns']==m
                assert phases['linear_algebra']['mod_ops']==sum(phases['linear_algebra'][z] for z in ['fr_add','fr_mul','fr_inv'])
                assert phases['scheduling']['pair_entries_ordered']==len(order)
                total=sum(phases[z]['group_add'] for z in ['factor_base','pair_table','collection','descent'])
                assert total==arm['total_group_add']
                assert abs(arm['total_proxy_w01']-(total+0.1*phases['linear_algebra']['mod_ops']))<1e-5
                assert arm['total_proxy_w1']==total+phases['linear_algebra']['mod_ops']
                if arm['status']=='ok':
                    assert rank==m and arm['recovered_log']==k
                    shift=arm['descent_trials']-1
                    ij=tuple(arm['descent_pair'])
                    assert table[add(q,scalar(shift,g))]==ij
                    assert (scalars[ij[0]]+scalars[ij[1]]-shift)%n==k
                    P0=q;descent_adds=0
                    for _ in range(shift):
                        if P0 is not None:descent_adds+=1
                        P0=add(P0,g)
                    assert phases['descent']['group_add']==descent_adds
                    audit['passed']+=1
                else:
                    assert arm['error']
                    audit['censored']+=1
            except Exception as e:
                audit['failures'].append({'seed':seed,'arm':name,'error':repr(e)})
    print(json.dumps(audit,sort_keys=True))
    assert not audit['failures']
    return audit
if __name__=='__main__':
    for path in sys.argv[1:]:audit_file(path)
