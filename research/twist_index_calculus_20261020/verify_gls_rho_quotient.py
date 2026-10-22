"""Independent scalar and pairing audit for both frozen GLS rho controls."""
from __future__ import annotations

import collections
import json
import pathlib
import statistics
import sys

ROOT = pathlib.Path(__file__).resolve().parent


def addf(x, y, p):
    return ((x[0]+y[0])%p,(x[1]+y[1])%p)


def negf(x, p):
    return (-x[0]%p,-x[1]%p)


def subf(x, y, p):
    return addf(x,negf(y,p),p)


def mulf(x, y, p, d):
    return ((x[0]*y[0]+d*x[1]*y[1])%p,
            (x[0]*y[1]+x[1]*y[0])%p)


def powf(x, n, p, d):
    a=(1,0)
    while n:
        if n&1:a=mulf(a,x,p,d)
        n//=2
        if n:x=mulf(x,x,p,d)
    return a


def invf(x, p, d):
    assert x!=(0,0)
    return powf(x,p*p-2,p,d)


def unpack(x):
    return tuple(x[:2]),tuple(x[2:])


def group_add(a,b,p,d,a_curve):
    if a is None:return b
    if b is None:return a
    x,y=a;u,v=b
    if x==u and addf(y,v,p)==(0,0):return None
    if a==b:
        num=addf(mulf((3,0),mulf(x,x,p,d),p,d),a_curve,p)
        den=mulf((2,0),y,p,d)
    else:
        num,den=subf(v,y,p),subf(u,x,p)
    lam=mulf(num,invf(den,p,d),p,d)
    xx=subf(subf(mulf(lam,lam,p,d),x,p),u,p)
    yy=subf(mulf(lam,subf(x,xx,p),p,d),y,p)
    return xx,yy


def scalar(pt,k,p,d,a_curve):
    out=None
    while k:
        if k&1:out=group_add(out,pt,p,d,a_curve)
        k//=2
        if k:pt=group_add(pt,pt,p,d,a_curve)
    return out


def verify(path):
    main=[json.loads(line) for line in (ROOT/'gls_symmetry_results_v2.ndjson').read_text().splitlines()]
    curves={x['p']:x for x in main if x['kind']=='curve'}
    rows=[json.loads(line) for line in path.read_text().splitlines()]
    assert len(rows)==579 and [x['p'] for x in rows if x['kind']=='curve']==[101,211,307]
    curves_new={x['p']:x for x in rows if x['kind']=='curve'}
    for p in (101,211,307):
        old,new=curves[p],curves_new[p]
        assert old['generator']==new['generator'] and old['lambda']==new['lambda']
        assert old['r']==new['r'] and old['a']==new['a'] and old['b_curve']==new['b']
    rho=[x for x in rows if x['kind']=='rho']
    assert len(rho)==576
    seen=set()
    failures=collections.Counter()
    summaries=[]
    for x in rho:
        p=x['p']; old=curves[p]; r=old['r']; d=old['d']; c=tuple(old['c'])
        c2=mulf(c,c,p,d);c3=mulf(c2,c,p,d)
        a_curve=mulf((old['a'],0),c2,p,d)
        b_curve=mulf((old['b_curve'],0),c3,p,d)
        g=unpack(old['generator'])
        k=x['target_scalar_fixture']; seed=x['seed'];arm=x['arm']
        assert arm in ('rho-negation','rho-psi4')
        assert k in (37,113,271) and 0<=seed<32
        assert x['r']==r
        key=(p,k,seed,arm)
        assert key not in seen
        seen.add(key)
        assert x['states']==x['probes']>0
        assert x['canonical_comparisons']==x['states']*(3 if arm=='rho-psi4' else 1)
        assert x['group_add']>=0 and x['f2_mul']>=0 and x['f2_inv']>=0
        if arm=='rho-psi4':
            assert x['lambda']==old['lambda'] and pow(x['lambda'],2,r)==r-1
            assert x['psi_eval']==x['states']+1
        else:
            assert x['lambda'] is None and x['psi_eval']==0
        q=scalar(g,k,p,d,a_curve)
        assert q is not None and mulf(q[1],q[1],p,d)==addf(
            addf(mulf(mulf(q[0],q[0],p,d),q[0],p,d),mulf(a_curve,q[0],p,d),p),b_curve,p)
        if x['recovered'] is None:
            assert not x['verified']
            failures[(p,arm)]+=1
        else:
            recovered=x['recovered']
            assert 0<=recovered<r
            assert scalar(g,recovered,p,d,a_curve)==q
            assert recovered==k and x['verified']
    assert len(seen)==576
    for p in (101,211,307):
        for arm in ('rho-negation','rho-psi4'):
            group=[x for x in rho if x['p']==p and x['arm']==arm]
            assert len(group)==96
            summaries.append({'p':p,'arm':arm,'failures':failures[(p,arm)],
                'median_states':statistics.median(x['states'] for x in group),
                'median_f2_mul':statistics.median(x['f2_mul'] for x in group)})
    return {'status':'pass','records':len(rows),'rho_runs':len(rho),
        'failures':sum(failures.values()),'summary':summaries,
        'independent_group_law':True,'all_success_scalars_replayed':True}


if __name__=='__main__':
    print(json.dumps(verify(pathlib.Path(sys.argv[1])),sort_keys=True))
