import json, sys, bisect, itertools, cmath, math, numpy as np
sys.path.insert(0, '.')
import cnum as C
def setup(seed,Sb):
    d=json.load(open(f'dev/tau_{seed}.json')); T=np.array(d['T_re'])+1j*np.array(d['T_im'])
    S=np.zeros((3,3)); k=0
    for i in range(3):
        for j in range(i,3):
            S[i,j]=S[j,i]=Sb[k]; k+=1
    return C.theta_all((T+S)/2)
val,grad=setup(1,[0,0,1,0,0,0])
E=C.EVEN
pairs=[((c1,c2),val[c1]*val[c2]) for c1,c2 in itertools.combinations(E,2)]
logs=sorted((math.log(abs(v)),i) for i,(k,v) in enumerate(pairs)); lv=[x for x,_ in logs]
trip=[((a,b,c),val[a]*val[b]*val[c]) for a,b,c in itertools.combinations(E,3)]
def find5(r,tol=1e-8):
    out=[]
    for k3,v3 in trip:
        t=math.log(abs(r))-math.log(abs(v3))
        i=bisect.bisect_left(lv,t-tol)
        while i<len(lv) and lv[i]<=t+tol:
            k2,v2=pairs[logs[i][1]]
            if not set(k2)&set(k3):
                z=r/(v3*v2); ph=cmath.phase(z)/(2*math.pi)*8
                if abs(ph-round(ph))<1e-6: out.append((tuple(sorted(k2+k3)),round(ph)%8))
            i+=1
    return sorted(set(out))
rng=np.random.default_rng(1)
cnt=0
for n1,n2,n3 in itertools.combinations(C.ODD,3):
    if C.q(C.cadd(n1,n2,n3))!=0: continue
    cnt+=1
    if cnt%97: continue
    D=np.linalg.det(np.array([grad[n1],grad[n2],grad[n3]]))/np.pi**3
    res=find5(D)
    s=C.cadd(n1,n2,n3)
    print((n1,n2,n3),'sum',s, res)
    for es,k in res:
        # test hypothesis: all triples among the 8 azygetic? sum of 8 zero?
        allc=[n1,n2,n3]+list(es)
        tot=C.cadd(*allc)
        print('  sum8',tot,'contains n1+n2+n3:',s in es)
print('n azygetic triples',cnt)
