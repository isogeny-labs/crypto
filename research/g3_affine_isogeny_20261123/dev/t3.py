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
P2=[((c1,c2),val[c1]*val[c2]) for c1,c2 in itertools.combinations_with_replacement(E,2)]
logs=sorted((math.log(abs(v)),i) for i,(k,v) in enumerate(P2))
lv=[x for x,_ in logs]
def search(r,tol=1e-9):
    out=[]
    lr=math.log(abs(r))
    for j,(kd,vd) in enumerate(P2):
        t=lr+math.log(abs(vd))
        i=bisect.bisect_left(lv,t-tol)
        while i<len(lv) and lv[i]<=t+tol:
            n=logs[i][1]
            z=r*vd/P2[n][1]  # should be 8th root
            k=cmath.phase(z)/(2*math.pi)*8
            if abs(k-round(k))<1e-7:
                out.append((P2[n][0],kd,round(k)%8))
            i+=1
    return out
ar=C.find_aronhold()
H,A=C.weber_frame([grad[m] for m in ar])
Hi=np.linalg.inv(H)
for m in C.ODD:
    l=grad[m]@Hi
    for s in (1,2):
        if abs(l[0])<1e-12 or abs(l[s])<1e-12:
            print(m,s,'zero coord', abs(l[0]),abs(l[s])); continue
        res=search(l[s]/l[0])
        print(m in ar, m, s, len(res), res[:2])
