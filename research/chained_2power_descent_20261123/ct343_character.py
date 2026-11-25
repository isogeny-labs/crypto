#!/usr/bin/env python3
"""Public CT343 F103 character-certificate self-check; no timing or independent PCC."""
import hashlib, itertools, json, pathlib, sys
import ct339 as c

def need(ok,msg):
    if not ok: raise ValueError(msg)
need(hashlib.sha256(pathlib.Path(c.__file__).read_bytes()).hexdigest()=='c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695','source hash')
c.P=103;c.DELTA=3;c.F=c.odd_f();c.CS=(c.ecoeff('+'),c.ecoeff('-'));c.TS=(c.ecpoint(c.CS[0],c.K(25)/4,c.K(0)),c.ecpoint(c.CS[1],c.K(4)/25,c.K(0)))
p=c.P;lc=c.F[-1];roots=[pow(i,-1,p) for i in range(1,6)]
need(pow(c.DELTA,(p-1)//2,p)==p-1,'extension nonsquare')

def peval(u,a):
    r=0
    for t in reversed(u):r=(r*a+t)%p
    return r

def chi(D,a):
    u=list(D[0]);d=len(u)-1;v=peval(u,a)
    if v:
        q=pow(-lc,d,p)*v%p
    else:
        w=c.pquot(u,[(-a)%p,1]);v=peval(w,a)
        need(v!=0,'repeated Weierstrass support')
        derivative=peval([i*c.F[i] for i in range(1,len(c.F))],a)
        # Monic chart X=lc*z,Y=lc^2*eta; f_monic prime(X)=lc^3*F prime(z).
        q=pow(lc,3,p)*derivative*pow(-lc,d-1,p)*v%p
        if '--bad-branch' in sys.argv:q=(-q)%p
    need(q!=0,'zero squareclass')
    return int(pow(q,(p-1)//2,p)==p-1)

def signature(D,indices):return tuple(chi(D,roots[i]) for i in indices)

def key(A):return tuple(None if q is None else (q[0].a,q[1].a) for q in A)

W=[c.O]+[c.norm([(-a)%p,1],[]) for a in roots]
K1=W[5];K2=c.jadd(W[1],W[4]);K3=c.jadd(W[2],W[3]);ker=[c.O,K1,K2,K3]
need(len(set(ker))==4 and c.jadd(K1,K2)==K3,'kernel construction')
for K in ker:need(c.jmul(K,2)==c.O and c.image(K)[0]==(None,None),'kernel check')
full=[signature(K,range(5)) for K in ker]
indices=next((pair for pair in itertools.combinations(range(5),2) if len({signature(K,pair) for K in ker})==4),None)
need(indices is not None,'no separating pair')
if '--bad-pair' in sys.argv:indices=(indices[0],indices[0])
ks=[signature(K,indices) for K in ker]
need(len(set(ks))==4,'pair does not separate kernel')
D=c.source_point(10,15);H23=c.jmul(D,100);H25=c.jmul(D,368)

def choose(r,m,g_sig,q_sig):
    # No source group arithmetic. Candidate signatures are parity * signature(G).
    hits=[s for s in (r,r+m) if tuple((s%2)*b for b in g_sig)==q_sig]
    if '--ignore-second' in sys.argv:
        hits=[s for s in (r,r+m) if (s%2)*g_sig[0]==q_sig[0]]
    return hits[0] if len(hits)==1 else None

rows=[]
for m,G in [(23,c.jadd(H23,K1)),(575,c.jadd(c.jadd(H23,H25),K1))]:
    g_sig=signature(G,indices);need(g_sig==ks[1] and any(g_sig),'generator character')
    need(c.jmul(G,m)==K1 and c.jmul(G,2*m)==c.O,'source order')
    V=c.image(G)[0];need(c.mulprod(V,m)==(None,None),'image annihilation')
    table={};T=(None,None)
    for r in range(m):table[key(T)]=r;T=c.sumprod(T,V)
    need(len(table)==m and T==(None,None),'image order')
    Q=c.O;seen=set();wrong=poison=0;categories={}
    for k in range(2*m):
        need(Q not in seen,'source order repeated');seen.add(Q)
        im,cat=c.image(Q);r=table[key(im)]
        got=choose(r,m,g_sig,signature(Q,indices));need(got==k,'character lift mismatch')
        categories[cat]=categories.get(cat,0)+1
        false=(r+m if got==r else r);need(tuple((false%2)*b for b in g_sig)!=signature(Q,indices),'wrong candidate accepted');wrong+=1
        for K in (K2,K3):
            B=c.jadd(Q,K);need(c.image(B)[0]==im,'poison image mismatch')
            need(choose(r,m,g_sig,signature(B,indices)) is None,'off-subgroup accepted');poison+=1
        Q=c.jadd(Q,G)
    need(Q==c.O,'cycle end')
    rows.append({'m':m,'targets':len(seen),'wrong_lifts_rejected':wrong,'off_subgroup_rejections':poison,'categories':categories,'generator_signature':g_sig})
print(json.dumps({'schema':'ct343-character-certificate/v1','p':p,'lc':lc,'roots':roots,'kernel_full_signatures':full,'chosen_indices':indices,'chosen_roots':[roots[i] for i in indices],'kernel_pair_signatures':ks,'cohorts':rows},sort_keys=True,separators=(',',':')))
