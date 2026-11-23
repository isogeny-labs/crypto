import itertools, json, time, sys
from fractions import Fraction as F
t0=time.time()
def reach(pts, w, lead, l):
    # JM componentwise strategy: k(mu)=max k<=l with mu-k*lead>=0; delta=sum k / sum wdeg
    sk=0; sw=0
    for mu in pts:
        k=min([mu[i]//lead[i] for i in range(len(lead)) if lead[i]>0]+[l])
        sk+=k; sw+=sum(a*b for a,b in zip(mu,w))
    return F(sk,sw)
def box(l): return list(itertools.product(range(8*l+1),repeat=4))
def simplex(l,coef):
    L=8*l; out=[]
    for mu in itertools.product(*[range(L//c+1) for c in coef]):
        if sum(a*b for a,b in zip(mu,coef))<=L: out.append(mu)
    return out
res={}
for l in range(1,7):
    r={}
    A=box(l); r['xbox']=(float(reach(A,(1,1,1,1),(8,8,8,8),l)),len(A))
    r['Ybox']=r['xbox']; r['Ymobius_box']=r['xbox']; r['x_plus_k_box']=r['xbox']; r['x_inv_box']=r['xbox']
    Bp=simplex(l,(1,1,1,1)); r['S4_e_x']=max((float(reach(Bp,(1,2,3,4),v,l)),len(Bp),v) for v in [(8,0,0,0),(0,8,0,0),(0,0,8,0),(0,0,0,8)])
    Dp=simplex(l,(2,2,2,1)); r['D4_eY2_pi']=max((float(reach(Dp,(2,4,6,4),v,l)),len(Dp),v) for v in [(4,0,0,0),(0,4,0,0),(0,0,4,0),(0,0,0,8)])
    r['dim_ratio_box_over_D4']=len(A)/len(Dp)
    r['D4_over_S4e']=r['D4_eY2_pi'][0]/r['S4_e_x'][0]
    res[l]=r; print(l,r,flush=True)
json.dump({'results':res,'runtime_s':time.time()-t0,'python':sys.version},open('reach_out.json','w'),indent=1,default=str)
