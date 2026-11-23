import itertools, json, time, random, sys
from fractions import Fraction as F
t0=time.time(); R={}
def kfun(s,L,l): return min([l]+[s[i]//L[i] for i in range(len(L)) if L[i]>0])
def delta(S,L,l):
    num=sum(kfun(s,L,l) for s in S); den=sum(sum(s) for s in S); return num/den
def box(m,M): return list(itertools.product(range(M+1),repeat=m))
def simplex(m,T,w=None):
    w=w or [1]*m; return [s for s in itertools.product(range(T+1),repeat=m) if sum(a*b for a,b in zip(s,w))<=T]
def extrap(ls,vals):  # linear fit in 1/l on last 3
    x=[1/l for l in ls[-3:]]; y=vals[-3:]; n=3; mx=sum(x)/n; my=sum(y)/n
    b=sum((a-mx)*(c-my) for a,c in zip(x,y))/sum((a-mx)**2 for a in x); return my-b*mx
# controls
ctl={}
for d in range(2,6):
    ls=[100,200,400]; v=[delta([(i,) for i in range(d*l)],(d,),l) for l in ls]; ctl[f"univ_d{d}"]=(extrap(ls,v),1/d)
ls=[50,100,200]; v=[delta(simplex(2,l),(1,0),l) for l in ls]; ctl["biv_linear"]=(extrap(ls,v),0.5)
def boxrun(m,ls,t=0):
    d=2**(m-1); L=(d,)*m; v=[delta(box(m,d*l+t),L,l) for l in ls]; return extrap(ls,v),v
for m,ls,tgt in [(2,[100,200,400],1/6),(3,[10,20,30],1/24),(4,[2,3,4],1/80)]:
    e,v=boxrun(m,ls); ctl[f"box_m{m}"]=(e,tgt,v)
R["controls"]=ctl
# m=2 shapes
L=(2,2); sh={}
for l in [50,100,200]:
    sh.setdefault("simplex",[]).append(delta(simplex(2,4*l),L,l))
    for t in [0,1,2,4]: sh.setdefault(f"box_t{t}",[]).append(delta(box(2,2*l+t),L,l))
    for w in [(1,2),(2,1)]: sh.setdefault(f"wsimplex{w}",[]).append(delta(simplex(2,6*l,list(w)),L,l))
R["m2_shapes"]={k:{"vals":v,"extrap":extrap([50,100,200],v)} for k,v in sh.items()}
# downset hill-climb m=2 (staircase heights), l=30, seed 0: delta = ratio; add/remove corner cells
random.seed(0); l=30; M=2*l+10
def dval(h):
    S=[(i,j) for i in range(M+1) for j in range(h[i])]; return delta(S,L,l) if S else 0
h=[2*l+1]*(2*l+1)+[0]*(M-2*l); best=dval(h); start=best
for it in range(3000):
    i=random.randrange(M+1); dlt=random.choice([-1,1]); h2=h[:]; h2[i]=max(0,min(M+1,h2[i]+dlt))
    if all(h2[j]>=h2[j+1] for j in range(M)):
        v=dval(h2)
        if v>best: best,h=v,h2
R["m2_hillclimb_l30"]={"start_box":start,"best":best,"box_l30_ref":delta(box(2,2*l),L,l)}
# m=4 primary: box and symmetric orbit lattice
L4=(8,)*4; m4={}
for l in [2,3,4]:
    orb=list(itertools.combinations_with_replacement(range(8*l+1),4)); m4.setdefault("sym_orbit",[]).append(delta(orb,L4,l))
    m4.setdefault("box",[]).append(delta(box(4,8*l),L4,l))
R["m4"]={k:{"vals":v,"extrap":extrap([2,3,4],v)} for k,v in m4.items()}
# sanity: squarefree S3 mod p
sys.path.insert(0,'/Volumes/SSD990-2/ml-cryptanalysis')
try:
    import sympy as sp
    x1,x2,x3=sp.symbols('x1 x2 x3'); a,b=3,7
    S3=(x1-x2)**2*x3**2-2*((x1+x2)*(x1*x2+a)+2*b)*x3+(x1*x2-a)**2-4*b*(x1+x2)
    sq={}
    for p in [1009,4093,10007]:
        f=sp.Poly(S3.subs(x3,5),x1,x2,modulus=p); fl=sp.factor_list(f.as_expr(),modulus=p)
        sq[p]=all(e==1 for _,e in fl[1])
    R["sanity_squarefree"]=sq
except Exception as ex: R["sanity_squarefree"]=repr(ex)
R["runtime_s"]=time.time()-t0; R["versions"]={"python":sys.version.split()[0]}
json.dump(R,open(sys.argv[1],"w"),indent=1,default=str); print(json.dumps(R,default=str)[:3000])
