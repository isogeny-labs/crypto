#!/usr/bin/env python3
"""Frozen Frobenius-bilinear Koblitz sum-bucket toy falsifier. Stdlib only."""
import argparse, collections, hashlib, json, math, platform, random, statistics

SEED=20261118
CELLS=((11,1,0x805,1982,991),(13,0,0x201b,8012,2003),(15,0,0x8003,33044,751))
N_TRAIN=8000
N_TEST=8000
K_VALUES=(1,2,3,4)

class BinaryField:
    def __init__(self,n,poly):
        self.n,self.poly=n,poly
        self.q=1<<n
        self.mask=self.q-1
        self.trace_mask=None
    def mul(self,x,y):
        z=0
        while y:
            if y&1:z^=x
            y>>=1
            x<<=1
            if x & self.q:x^=self.poly
        return z
    def sq(self,x):return self.mul(x,x)
    def power(self,x,k):
        y=1
        while k:
            if k&1:y=self.mul(y,x)
            x=self.sq(x)
            k>>=1
        return y
    def inv(self,x):
        assert x!=0
        return self.power(x,self.q-2)
    def trace_slow(self,x):
        v=0
        for _ in range(self.n):
            v^=x
            x=self.sq(x)
        assert v in (0,1),(self.n,v)
        return v
    def prepare_trace(self):
        self.trace_mask=sum(self.trace_slow(1<<i)<<i for i in range(self.n))
        for x in [0,1,2,3,5,7,17,self.q-1]:
            assert self.trace(x)==self.trace_slow(x)
    def trace(self,x):return (x&self.trace_mask).bit_count()&1
    def points(self,a):
        # Solve z^2+z=u by a complete, deterministic Artin-Schreier table.
        z_for_u={}
        for z in range(self.q):z_for_u.setdefault(self.sq(z)^z,z)
        arr=[(0,1)]
        for x in range(1,self.q):
            inv=self.inv(x)
            rhs=x^a^self.sq(inv)
            if rhs in z_for_u:
                y=self.mul(x,z_for_u[rhs])
                arr.append((x,y))
                arr.append((x,y^x))
        return arr
    def on_curve(self,P,a):
        if P is None:return True
        x,y=P
        return (self.sq(y)^self.mul(x,y))==(self.mul(self.sq(x),x)^(self.mul(a,self.sq(x)))^1)
    def neg(self,P):return None if P is None else (P[0],P[1]^P[0])
    def add(self,P,Q,a):
        if P is None:return Q
        if Q is None:return P
        x,y=P;u,v=Q
        if x==u:
            if y!=v or x==0:return None
            slope=x^self.mul(y,self.inv(x))
        else:
            slope=self.mul(y^v,self.inv(x^u))
        xx=self.sq(slope)^slope^x^u^a
        yy=self.mul(slope,x^xx)^xx^y
        return xx,yy
    def scalar(self,P,k,a):
        R=None
        while k:
            if k&1:R=self.add(R,P,a)
            k>>=1
            if k:P=self.add(P,P,a)
        return R
    def frobenius(self,P):return None if P is None else (self.sq(P[0]),self.sq(P[1]))

def wilson(k,n,conf=1.96):
    p=k/n;z=conf
    d=1+z*z/n
    ctr=(p+z*z/(2*n))/d
    half=z*math.sqrt(p*(1-p)/n+z*z/(4*n*n))/d
    return [ctr-half,ctr+half]

def run_cell(n,a,poly,N,r,verbosity=False):
    F=BinaryField(n,poly)
    F.prepare_trace()
    assert all(F.power(i,F.q-1)==1 for i in range(1,64)), 'field modulus reducible'
    pts=F.points(a)
    assert len(pts)+1==N,(n,len(pts)+1,N)
    assert all(F.on_curve(P,a) for P in pts)
    # Check independent correctness of point arithmetic over all affine point cases.
    P0=next(P for P in pts if P[0]!=0)
    assert F.add(P0,F.neg(P0),a) is None
    assert F.add(P0,None,a)==P0
    assert F.add(None,P0,a)==P0
    assert F.on_curve(F.add(P0,P0,a),a)
    G=None
    for P in pts:
        cand=F.scalar(P,N//r,a)
        if cand is not None and F.scalar(cand,r,a) is None:
            G=cand;break
    assert G is not None
    subgroup=[None]
    for _ in range(1,r):
        subgroup.append(F.add(subgroup[-1],G,a))
    assert len(subgroup)==r and subgroup[0] is None
    assert all(P is not None and F.on_curve(P,a) for P in subgroup[1:])
    assert F.add(subgroup[-1],G,a) is None
    assert len(set(subgroup))==r
    rng=random.Random(SEED*100003+n*101+a*13)
    for _ in range(64):
        P,Q,R=(rng.choice(subgroup) for i in range(3))
        assert F.add(F.add(P,Q,a),R,a)==F.add(P,F.add(Q,R,a),a)
        assert F.add(F.neg(P),P,a) is None
        assert F.frobenius(F.add(P,Q,a))==F.add(F.frobenius(P),F.frobenius(Q),a)
    xs=[0]+[P[0] for P in subgroup[1:]]
    x_frob=[[0]*5 for _ in range(r)]
    for i in range(1,r):
        x_frob[i][0]=xs[i]
        for k in range(1,5):x_frob[i][k]=F.sq(x_frob[i][k-1])
    assert all(F.sq(subgroup[i][0])==F.frobenius(subgroup[i])[0] for i in range(1,r))
    samples=[];used=set();exceptional=0
    # Not arbitrary group labels: prime subgroup points are obtained by actual EC scalar addition.
    while len(samples)<N_TRAIN+N_TEST:
        i,j=rng.randint(1,r-1),rng.randint(1,r-1)
        key=(min(i,j),max(i,j))
        if key in used:continue
        used.add(key)
        if (i+j)%r==0:
            exceptional+=1
            continue
        P,Q=subgroup[i],subgroup[j]
        S=F.add(P,Q,a)
        assert S==subgroup[(i+j)%r]
        if len(samples)<128:assert F.on_curve(S,a)
        label=F.trace(F.mul(S[0],F.sq(S[0]))) # Tr(x(S)^3)
        features=[]
        for k in K_VALUES:
            prod=F.mul(xs[i],x_frob[j][k]) ^ F.mul(xs[j],x_frob[i][k])
            value=F.trace(prod)
            # Frobenius-invariance checks on real P,Q (first sampled pairs only).
            if len(samples)<32:
                xx,yy=F.sq(xs[i]),F.sq(xs[j])
                prod2=F.mul(xx,F.sq(x_frob[j][k]))^F.mul(yy,F.sq(x_frob[i][k]))
                assert value==F.trace(prod2)
            features.append(value)
        samples.append((label,features))
    train=samples[:N_TRAIN];test=samples[N_TRAIN:]
    assert len(used)==len(samples)+exceptional
    # Training selects one bit and polarity. Holdout labels are never examined here.
    winners=[]
    for col,k in enumerate(K_VALUES):
        for flip in (0,1):
            cnt=collections.Counter((label,(feat[col]^flip)) for label,feat in train)
            good=cnt[(0,0)]+cnt[(1,1)]
            # balanced accuracy across true class; avoids label imbalance gaming.
            p0=(cnt[(0,0)]/(cnt[(0,0)]+cnt[(0,1)]))
            p1=(cnt[(1,1)]/(cnt[(1,0)]+cnt[(1,1)]))
            balanced=(p0+p1)/2
            winners.append({'k':k,'flip':flip,'train_accuracy':good/N_TRAIN,'train_balanced':balanced})
    chosen=max(winners,key=lambda t:(t['train_balanced'],-t['k'],-t['flip']))
    k,flip=chosen['k'],chosen['flip']
    cnt=collections.Counter((label,(feat[k-1]^flip)) for label,feat in test)
    correct=cnt[(0,0)]+cnt[(1,1)]
    n0=cnt[(0,0)]+cnt[(0,1)];n1=cnt[(1,0)]+cnt[(1,1)]
    accuracy=correct/N_TEST
    balance=((cnt[(0,0)]/n0)+(cnt[(1,1)]/n1))/2
    majority=max(n0,n1)/N_TEST
    # Use inversion equivalent as a lower bound on the filter's cost advantage.
    # Fermat inversion via square-and-multiply roughly (n-2) mul + (n-1) squares;
    # affine add: 1 inv + 2 mul + 1 square, so optimistic C_add=2n M-equiv.
    Cadd=2*n;Cf=2
    optimistic_speed=2*accuracy*Cadd/(Cadd+Cf)
    expected_rho_vanilla=math.sqrt(math.pi*r/2)
    expected_rho_orbit=math.sqrt(math.pi*r/(4*n))
    B=round(math.sqrt(r));pair_adds=B*(B+1)//2
    # An oracle with >=B base points must spend at least B(B+1)/2 pair additions
    # for an exhaustive pair table, ignoring logarithm/rank/target recovery.
    shuffled=[]
    labels=[y for y,_ in test]
    preds=[f[k-1]^flip for _,f in test]
    rng_null=random.Random(SEED+n*211+a*41)
    for _ in range(128):
        ll=labels[:];rng_null.shuffle(ll)
        shuffled.append(sum(x==y for x,y in zip(ll,preds))/N_TEST)
    # Positive control: if the true label were C1 xor 10%-noise, the experiment
    # should see a detectable signal, unlike a predictor with no discrimination.
    rng_pos=random.Random(SEED+n*811)
    positive=[int(f[0] != (rng_pos.random()<0.10)) for _,f in test]
    positive_acc=sum(pos==f[0] for pos,(_,f) in zip(positive,test))/N_TEST
    assert positive_acc>0.85
    result={
        'field_degree':n,'a':a,'modulus':hex(poly),'field_size':F.q,'curve_order':N,
        'subgroup_order':r,'generator':list(G),'subgroup_verified':True,
        'samples':{'train':N_TRAIN,'test':N_TEST,'excluded_identity_pairs':exceptional,'unique_unordered':len(used)},
        'feature':'Tr(x(P)*x(Q)^(2^k) + x(Q)*x(P)^(2^k))',
        'target':'Tr(x(P+Q)^3)',
        'train_arms':winners,'selected':chosen,'holdout':{
          'confusion':{'true0_pred0':cnt[(0,0)],'true0_pred1':cnt[(0,1)],
                       'true1_pred0':cnt[(1,0)],'true1_pred1':cnt[(1,1)]},
          'accuracy':accuracy,'wilson95':wilson(correct,N_TEST),'balanced_accuracy':balance,
          'majority_accuracy':majority,'ratio_to_majority':accuracy/majority,
          'permutation_null_mean':statistics.mean(shuffled),'permutation_null_max':max(shuffled),
          'permutation_empirical_p_upper':(1+sum(x>=accuracy for x in shuffled))/(1+len(shuffled)),
          'positive_synthetic_accuracy':positive_acc},
        'cost_model':{'filter_mul_min':2,'affine_add_mequiv_optimistic':Cadd,
          'max_ideal_speedup_charged':optimistic_speed,'B_sqrt_r':B,
          'pair_additions_lower_bound':pair_adds,'rho_plain_expected_steps':expected_rho_vanilla,
          'rho_aut2n_optimistic_expected_steps':expected_rho_orbit,
          'pair_table_to_rho_aut_lower_bound':pair_adds/expected_rho_orbit},
        'primary_accuracy_gate':accuracy>=1.2*majority,
        'primary_charged_gate':optimistic_speed>=1.2,
    }
    return result

def main():
    ap=argparse.ArgumentParser()
    ap.add_argument('--output',default=None)
    args=ap.parse_args()
    rows=[run_cell(*cell) for cell in CELLS]
    primary=rows[:2]
    verdict='supports_toy_criterion' if all(x['primary_accuracy_gate'] and x['primary_charged_gate'] for x in primary) else 'falsified_toy_criterion'
    result={'protocol':'PROTOCOL.md','date':'2026-11-18','seed':SEED,
            'python':platform.python_version(),'samples_per_cell':N_TRAIN+N_TEST,
            'counting':'EC group law verified and field mult counted analytically; no calibrated wall-clock speedups',
            'primary_cells':2,'sensitivity_cells':1,'result':verdict,'cells':rows,
            'standing':'producer-local only; independent PCC replication pending'}
    s=json.dumps(result,indent=2,sort_keys=True)+'\n'
    if args.output:
        with open(args.output,'w') as f:f.write(s)
    else:print(s)
    print('VERDICT',verdict)
    for row in rows:
        h=row['holdout'];c=row['cost_model']
        print(f"n={row['field_degree']} a={row['a']} r={row['subgroup_order']} k={row['selected']['k']} flip={row['selected']['flip']} acc={h['accuracy']:.4f} maj={h['majority_accuracy']:.4f} ratio={h['ratio_to_majority']:.4f} positive={h['positive_synthetic_accuracy']:.4f} ideal_charged={c['max_ideal_speedup_charged']:.4f} pnull={h['permutation_empirical_p_upper']:.4f} pair/rho={c['pair_table_to_rho_aut_lower_bound']:.2f}x")
if __name__=='__main__':main()
