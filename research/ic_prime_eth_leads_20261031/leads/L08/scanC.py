import sys, json, time
from cnt import *
def cands(d):
    res=[]
    for a in range(0,9):
        L=3**a; nmax=(L+1)//2
        k=-(-d//nmax)
        J=3
        while J<k or 3*J<L: J*=3
        res.append((J,k,L))
    return sorted(set(res))
if __name__=="__main__":
    out=[]; t=time.time()
    ds=list(range(1,401))+list(range(401,5001,13))+[5000]
    for d in ds:
        rows=[]
        for J,k,L in cands(d):
            n_=-(-d//k)
            if J>6*d+30 or (L>3*(2*n_-1) and L>3) or J>=9*k: continue
            e,deg,L2=count_poly(d,J,k); rows.append((e,deg,J,k,L2))
        out.append((d,rows))
        if d%400==0: print(d,min(rows),round(time.time()-t),flush=True)
    json.dump(out,open("scanC_rows.json","w"))
