#!/usr/bin/env python3
"""Deterministic toy test of a smooth-companion torsion sieve on split genus-2 covers.
No discrete logarithms are inferred; only exact 2-sum coverage is compared.
Usage: python3 bielliptic_torsion_sieve_20261118.py > results.json
"""
import json, math, random
from collections import Counter

SEED = 20261118
PS = (101, 211, 307)
MAX_COEFF = 11
NULLS = 128

class EC:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b
        self.ops = 0
    def rhs(self,x):
        return (x*x*x + self.a*x*x + self.b*x + 1) % self.p
    def all_points(self):
        root = {}
        for y in range(self.p):
            root.setdefault(y*y%self.p, []).append(y)
        return [None] + [(x,y) for x in range(self.p) for y in root.get(self.rhs(x),())]
    def add(self,P,Q):
        self.ops += 1
        p=self.p
        if P is None: return Q
        if Q is None: return P
        x,y=P; u,v=Q
        if x==u:
            if (y+v)%p==0: return None
            if y==0: return None
            slope=((3*x*x+2*self.a*x+self.b)*pow(2*y,-1,p))%p
        else:
            slope=((v-y)*pow(u-x,-1,p))%p
        xx=(slope*slope-self.a-x-u)%p
        return (xx,(slope*(x-xx)-y)%p)
    def mul(self,k,P):
        R=None
        while k:
            if k&1: R=self.add(R,P)
            k//=2
            if k: P=self.add(P,P)
        return R

def factors(n):
    t=n; out=[]; d=2
    while d*d<=t:
        if t%d==0:
            v=0
            while t%d==0: t//=d; v+=1
            out.append((d,v))
        d+=1
    if t>1: out.append((t,1))
    return out

def divisors(n):
    ds=[1]
    for q,e in factors(n):
        ds=[d*q**i for d in ds for i in range(e+1)]
    return sorted(ds)

def smooth_sextic(p,a,b):
    # f(X)=X^3 + aX^2 + bX + 1 separable, and x^6+... has no repeated roots.
    # discriminant of monic cubic (coefficient x^2=a, x=b, const=1)
    disc=(a*a*b*b-4*b*b*b-4*a*a*a-27+18*a*b)%p
    return disc != 0

def mappings(p,a,b):
    e1=EC(p,a,b); e2=EC(p,b,a)
    points1=e1.all_points(); points2=e2.all_points()
    n1,n2=len(points1),len(points2)
    fac=factors(n1)
    r=max(q for q,_ in fac)
    if r < p//2 or n1%r != 0 or n1//r>4:
        return None
    if max(q for q,_ in factors(n2))>13:
        return None
    h=n1//r
    roots=Counter((y*y)%p for y in range(p))
    ry={v:[] for v in range(p)}
    for y in range(p): ry[y*y%p].append(y)
    # Map all affine C(Fp) points with x!=0 to both elliptic quotients.
    cand={}
    for x in range(1,p):
        XX=x*x%p
        vv=e1.rhs(XX)
        for y in ry[vv]:
            P=(XX,y)
            Q=(pow(XX,-1,p),y*pow(x,-3,p)%p)
            assert e2.rhs(Q[0])==Q[1]*Q[1]%p
            T=e1.mul(h,P)
            if T is None: continue
            assert e1.mul(r,T) is None
            cand.setdefault(T, []).append(Q)
    if len(cand) < 6: return None
    # m candidate chosen by cardinality only, never by any pair-sum coverage.
    target=.67*math.sqrt(r)
    opts=[]
    for m in divisors(n2):
        hits={T for T,qs in cand.items() if any(e2.mul(m,Q) is None for Q in qs)}
        if 3<=len(hits)<=int(1.5*math.sqrt(r)):
            opts.append((abs(len(hits)-target),m,hits))
    if not opts: return None
    _,m,hits=min(opts,key=lambda it:(it[0],it[1]))
    return e1,e2,points1,r,h,n2,cand,m,hits

def sum_coverage(e, base):
    cover=set()
    for i,P in enumerate(base):
        for Q in base[i:]:
            cover.add(e.add(P,Q))
    return len(cover)

def run():
    records=[]
    for p in PS:
        chosen=None
        for a in range(MAX_COEFF):
            for b in range(MAX_COEFF):
                if not smooth_sextic(p,a,b): continue
                value=mappings(p,a,b)
                if value is not None:
                    chosen=(a,b,value);break
            if chosen:break
        if chosen is None: raise RuntimeError(f'no curve at {p}')
        a,b,(e1,e2,points1,r,h,n2,cand,m,hits)=chosen
        B=len(hits)
        # uniform exact subgroup census, not restricted to cover-image.
        group={e1.mul(h,P) for P in points1}
        assert len(group)==r
        assert all(e1.mul(r,P) is None for P in group)
        sampled=sorted(hits,key=lambda pt: (-1,-1) if pt is None else pt)
        cover=sum_coverage(e1,sampled)
        pool=sorted(group,key=lambda pt: (-1,-1) if pt is None else pt)
        rng=random.Random(SEED+p)
        uniform=[sum_coverage(e1,rng.sample(pool,B)) for _ in range(NULLS)]
        cover_pool=sorted(cand,key=lambda pt:(-1,-1) if pt is None else pt)
        cover_null=[sum_coverage(e1,rng.sample(cover_pool,B)) for _ in range(NULLS)]
        # Control for the simple E1 x-coordinate geometry of any cover.
        xcontrol=sorted(group,key=lambda pt: (-1,-1) if pt is None else (pt[0],pt[1]))[:B]
        xcover=sum_coverage(e1,xcontrol)
        mean=sum(uniform)/NULLS
        sd=math.sqrt(sum((u-mean)**2 for u in uniform)/(NULLS-1))
        tlo=sum(v<=cover for v in uniform)/NULLS
        thi=sum(v>=cover for v in uniform)/NULLS
        rho=math.sqrt(math.pi*r/4) # idealized negation folded rho, no setup
        # exhaustive pair-table construction alone; excludes sieve & group order census.
        pairadds=B*(B+1)//2
        records.append({
            'p':p,'a':a,'b':b,'genus2':'y^2 = x^6+a*x^4+b*x^2+1',
            'E1_order':len(points1),'prime_subgroup_order':r,'cofactor':h,
            'E2_order':n2,'E2_factors':factors(n2),'torsion_m':m,
            'cover_image_unique_subgroup_points':len(cand),'B':B,
            'coverage':cover,'coverage_ratio_uniform':round(cover/mean,6),
            'coverage_ratio_xcontrol':round(cover/xcover,6),
            'uniform_mean':round(mean,6),'uniform_sd':round(sd,6),
            'cover_image_null_mean':round(sum(cover_null)/NULLS,6),
            'coverage_ratio_cover_image':round(cover/(sum(cover_null)/NULLS),6),
            'cover_image_null_ge_fraction':sum(v>=cover for v in cover_null)/NULLS,
            'uniform_min':min(uniform),'uniform_max':max(uniform),
            'uniform_ge_fraction':thi,'uniform_le_fraction':tlo,
            'xcontrol_coverage':xcover,
            'rho_negation_reference_group_adds':round(rho,3),
            'pair_table_adds_lower_bound':pairadds,
            'pair_table_to_rho_lower_bound':round(pairadds/rho,3),
            'admission_and_selection_group_ops_including_rejected_curves':e1.ops+e2.ops-pairadds*(2*NULLS+2),
        })
    return {'claimClass':'EMPIRICAL','seed':SEED,'nulls':NULLS,
        'parameter_selection':'first lexicographic (a,b)<11 with squarefree sextic, prime E1 subgroup >p/2, cofactor<=4, E2 13-smooth and m chosen by B~0.67sqrt(r) without looking at coverage',
        'falsifier':'Reject if coverage gain <=1.20 versus both uniform and cover-image matched B on every p, or if charged pair-table ops already exceed ideal folded-rho ops; never infer ECDLP break from coverage alone.',
        'cases':records}

if __name__=='__main__':
    print(json.dumps(run(),indent=2))
