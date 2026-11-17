import random, math, json, itertools
p=68719476731; a=68719476728; b=33; r=68719022539; G=(2,51741851039)
def add(P,Q):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P;x2,y2=Q
    if x1==x2:
        if (y1+y2)%p==0: return None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else: l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p; return (x3,(l*(x1-x3)-y1)%p)
def mul(k,P):
    R=None
    while k:
        if k&1: R=add(R,P)
        P=add(P,P); k>>=1
    return R
def leg(n,m): return 1 if pow(n%m,(m-1)//2,m)==1 else 0
rnd=random.Random(20261102); N=60000
feats=[];targs=[]
for _ in range(N):
    k=rnd.randrange(1,r); x,y=mul(k,G)
    f={}
    for j in range(8): f[f"x_bit{j}"]=(x>>j)&1
    f["x_top1"]=int(x>p//2); f["x_top2"]=int((x*4//p)==1)|int((x*4//p)==3)  # second bit
    f["y_par"]=y&1; f["y_top"]=int(y>p//2)
    f["leg_x"]=leg(x,p); f["leg_x+1"]=leg(x+1,p); f["leg_x-1"]=leg(x-1,p); f["x_mod3"]=int(x%3==0); f["x_mod5"]=int(x%5<2)
    g={}
    for j in range(8): g[f"k_bit{j}"]=(k>>j)&1
    g["k_top1"]=int(k>r//2); g["k_top2"]=int((k*4//r) in (1,3)); g["k_top3"]=int((k*8//r)%2)
    g["k_mod3"]=int(k%3==0); g["k_mod5"]=int(k%5<2); g["k_mod7"]=int(k%7<3)
    g["leg_k"]=leg(k,r); g["k_lt_sqrt"]=int(k<math.isqrt(r)*8)
    feats.append(f);targs.append(g)
fn=list(feats[0]);tn=list(targs[0])
def z(xs,ys):
    n=len(xs);c=[[0,0],[0,0]]
    for u,v in zip(xs,ys): c[u][v]+=1
    r0=[sum(c[0]),sum(c[1])];c0=[c[0][0]+c[1][0],c[0][1]+c[1][1]]
    if min(r0+c0)==0: return 0.0
    chi=0
    for i in range(2):
        for j in range(2):
            e=r0[i]*c0[j]/n; chi+=(c[i][j]-e)**2/e
    s=1 if c[0][0]*c[1][1]-c[0][1]*c[1][0]>0 else -1
    return s*math.sqrt(chi)
res=[]
for f in fn:
    xs=[d[f] for d in feats]
    for t in tn:
        res.append((abs(z(xs,[d[t] for d in targs])),f,t))
res.sort(reverse=True)
m=len(res); bon=-__import__("statistics").NormalDist().inv_cdf(0.05/(2*m))
print(f"N={N} tests={m} Bonferroni |z| threshold={bon:.2f}; top 5:")
for zz,f,t in res[:5]: print(f"  |z|={zz:.2f}  {f} vs {t}")
# expected max under null for m tests ~ sqrt(2 ln m)
print("expected max |z| under null ~", round(math.sqrt(2*math.log(m)),2))
# planted positive control: copy a feature bit into a target with extra prob eps
for eps in (0.01,0.02,0.03):
    xs=[d["x_bit0"] for d in feats]
    ys=[x if rnd.random()<0.5+eps*2 else 1-x for x in xs]   # P(y=x)=0.5+2eps... => bias 2eps/ (centred)
    print(f"planted bias P(match)={0.5+2*eps:.2f}: |z|={abs(z(xs,ys)):.2f}")
