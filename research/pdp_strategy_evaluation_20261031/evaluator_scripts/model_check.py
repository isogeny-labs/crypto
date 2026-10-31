# exploratory desk re-derivation of the hybrid rows; reuses descent_degree_cost_20260930.py's model
from math import comb, factorial, log2
N_FIELD=131; RHO=60.81
def log2_add(a,b): return max(a,b)+log2(1+2**(-abs(a-b)))
def cost(m,l,D,w,k):
    n=N_FIELD; N=(m-2)*n+m*l
    per=max(0.0,log2(factorial(m))+n-m*l); targets=l+per
    cols=sum(comb(N-k,i) for i in range(D+1))
    oracle=k+w*log2(cols)
    alg=log2(m)+2*l
    return log2_add(targets+oracle,alg),targets,oracle,alg,N
def best(m,D,w,kf):
    return min((cost(m,l,D,w,kf(l))[0],l) for l in range(4,70))
for name,D,w,kf in [("deg4 k=0",4,1,lambda l:0),("deg5 k=0",5,1,lambda l:0),
    ("S_const k=1",4,1,lambda l:1),("S_const k=2",4,1,lambda l:2),
    ("S_lin k=1(l-6)",4,1,lambda l:max(0,l-6)),("S_lin k=2(l-6)",4,1,lambda l:2*max(0,l-6)),
    ("S_linm k=3(l-6)=m(l-6)",4,1,lambda l:4*max(0,l-6)),
    ("deg4 w=2",4,2,lambda l:0),("deg5 w=2",5,2,lambda l:0),
    ("S_const k=2 w=2",4,2,lambda l:2),("deg4 w=2.2",4,2.2,lambda l:0),("S_const k=2 w=2.2",4,2.2,lambda l:2),("deg5 w=2.2",5,2.2,lambda l:0)]:
    t,l=best(4,D,w,kf); print(f"{name:28s} l*={l:3d} total=2^{t:.2f} vs rho {t-RHO:+.2f}")
# break-even k vs degree 5 (oracle only at N=398, w=1)
N=398
d5=log2(sum(comb(N,i) for i in range(6)))
for w in (1,2):
    d5w=w*d5
    ks=[k for k in range(0,40) if k+w*log2(sum(comb(N-k,i) for i in range(5)))<d5w]
    print("w",w,"hybrid beats deg5 for k <=",max(ks))
# column ratios and projected degree-5 advantage at (3,7)
for N in (40,42):
    c5=sum(comb(N,i) for i in range(6)); c4=sum(comb(N,i) for i in range(5)); print(N,c4,c5,c5/c4)
# ratio hybrid/deg5 under different e using RESULTS draw (19,7) 0: hybrid 9.71e11, ops4 ~3.18e11 (draw 3 scale)
r=7.45
for e in (2.2,3.0,4.1,4.25):
    print("e",e,"deg5/hybrid for k*=2 (9.71e11/3.18e11=3.05):",r**e/3.05,"for k*=1 (1.68):",r**e/1.68)
