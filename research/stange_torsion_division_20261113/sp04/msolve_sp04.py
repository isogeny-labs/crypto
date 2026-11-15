"""STANGE-SP-04 at n ~ sqrt(p) with msolve; plus arm B (binomial / multiplicative-subgroup factor base).
Arms (all decompose a target x-coordinate xR on E as P1 + P2 + R = O, m = 3):
  E: plain S3 on E, factor base = n smallest x-coords of points on E (dense poly prod(u-c))
  I: plain S3 on E, factor base = x(phi(F1)) (same x-set the pulled arm reaches)
  P: S3 pulled back through the 2-isogeny phi: E1 -> E, factor base F1 on E1 (n' chosen so |x(phi F1)| ~ n)
  B: plain S3 on E, factor base = mu_m (u^m = 1, m | p-1, m ~ n): sparse binomial
Metric: msolve wall time per target (minus a trivial-system baseline), and verified relations
(F_p-rational x-pairs that are x-coords of F_p points). Frozen before data: primes, seeds, n=sqrt(p), 8 targets.
"""
import sys, time, subprocess, random, statistics as st, tempfile, os, math, json
p=int(sys.argv[1]); a=5; x0=3; SEED=int(sys.argv[2]) if len(sys.argv)>2 else 11
b=(-x0**3-a*x0)%p; t=(3*x0*x0+a)%p; w=x0*t%p; A=(a-5*t)%p; B=(b-7*w)%p
assert t and (4*a**3+27*b*b)%p and (4*A**3+27*B*B)%p
issq=lambda z: z%p==0 or pow(z%p,(p-1)//2,p)==1
phix=lambda x:(x*x-x0*x+t)*pow(x-x0,-1,p)%p
def S3(X1,X2,X3): return ((X1-X2)**2*X3**2-2*((X1+X2)*(X1*X2+A)+2*B)*X3+(X1*X2-A)**2-4*B*(X1+X2))%p
n=int(round(math.isqrt(p)))
# m | p-1 closest to n
m=min((d for d in range(2,2*n) if (p-1)%d==0), key=lambda d:abs(d-n))
assert abs(m-n)<=n//4, f'p-1 has no divisor near n={n} (m={m}); pick p = 1 mod n'
xsE=[x for x in range(p) if issq(x**3+A*x+B)][:n]
xs1=[]; img=set()
for x in range(p):
    if x!=x0 and issq(x**3+a*x+b):
        xs1.append(x); img.add(phix(x))
        if len(img)>=n: break
FI=sorted(img)
g=pow(next(z for z in range(2,p) if all(pow(z,(p-1)//q,p)!=1 for q in {f for f in range(2,p) if (p-1)%f==0 and all(f%k for k in range(2,int(f**.5)+1))})),(p-1)//m,p)
MU=sorted({pow(g,k,p) for k in range(m)}); assert len(MU)==m
def poly_str(c):  # univariate prod(z-c) expanded, as msolve string in var z
    co=[1]
    for r in c:
        nc=[0]*(len(co)+1)
        for i,v in enumerate(co): nc[i]=(nc[i]-r*v)%p; nc[i+1]=(nc[i+1]+v)%p
        co=nc
    return co  # co[i] = coeff of z^i
def uni(co,var): return '+'.join(f"{c}*{var}^{i}" for i,c in enumerate(co) if c)
def S3_plain(xR): return f"(u-v)^2*{xR}^2-2*((u+v)*(u*v+{A})+2*{B})*{xR}+(u*v-{A})^2-4*{B}*(u+v)"
def S3_pulled(xR):
    N=lambda z:f"({z}^2-{x0}*{z}+{t})"; D=lambda z:f"({z}-{x0})"
    nu,nv,du,dv=N('u'),N('v'),D('u'),D('v')
    return (f"({nu}*{dv}-{nv}*{du})^2*{xR}^2-2*(({nu}*{dv}+{nv}*{du})*({nu}*{nv}+{A}*{du}*{dv})+2*{B}*{du}^2*{dv}^2)*{xR}"
            f"+({nu}*{nv}-{A}*{du}*{dv})^2-4*{B}*{du}*{dv}*({nu}*{dv}+{nv}*{du})")
coE,coI,co1=poly_str(xsE),poly_str(FI),poly_str(xs1)
def binom(var): return f"{var}^{m}-1"
def run_msolve(polys):
    with tempfile.TemporaryDirectory() as d:
        fi=os.path.join(d,'in.ms'); fo=os.path.join(d,'out.ms')
        open(fi,'w').write("u,v\n%d\n"%p+",\n".join(polys)+"\n")
        t0=time.perf_counter(); r=subprocess.run(['msolve','-g','2','-t','1','-f',fi,'-o',fo],capture_output=True,text=True); dt=time.perf_counter()-t0
        if r.returncode!=0:
            import shutil; shutil.copy(fi,f'fail_p{p}.ms'); open(f'fail_p{p}.log','w').write(f"rc={r.returncode}\nSTDOUT\n{r.stdout[-3000:]}\nSTDERR\n{r.stderr[-3000:]}")
            raise SystemExit(f"msolve rc={r.returncode}; input saved to fail_p{p}.ms")
        return dt, open(fo).read()
base=min(run_msolve(["u-1","v-2"])[0] for _ in range(7))
def tmin(polys):
    best=None
    for _ in range(3):
        dt,out=run_msolve(polys); best=dt if best is None else min(best,dt)
    return best,out
def is_x(X): return issq(X**3+A*X+B)
def rel_plain(F,xR): return sum(1 for X1 in F for X2 in F if is_x(X1) and is_x(X2) and S3(X1,X2,xR)==0)
def rel_pulled(F1,xR): return sum(1 for x in F1 for y in F1 if S3(phix(x),phix(y),xR)==0)
R=random.Random(SEED)
def rand_xE():
    while True:
        z=R.randrange(p)
        if issq(z**3+A*z+B): return z
T={k:[] for k in 'EIPB'}; REL={k:0 for k in 'EIPB'}
for _ in range(8):
    xR=rand_xE()
    arms=[('E',[S3_plain(xR),uni(coE,'u'),uni(coE,'v')],lambda:rel_plain(xsE,xR)),
          ('I',[S3_plain(xR),uni(coI,'u'),uni(coI,'v')],lambda:rel_plain(FI,xR)),
          ('P',[S3_pulled(xR),uni(co1,'u'),uni(co1,'v')],lambda:rel_pulled(xs1,xR)),
          ('B',[S3_plain(xR),binom('u'),binom('v')],lambda:rel_plain(MU,xR))]
    R.shuffle(arms)
    for k,sys_,rel in arms:
        dt,out=tmin(sys_); T[k].append(dt-base); r=rel(); REL[k]+=r
res={'p':p,'n':n,'m_binomial':m,'|F1|':len(xs1),'|x(phiF1)|':len(FI),'baseline_s':base,'timing':'min of 3 msolve wall times minus min-of-7 trivial-system baseline',
     'median_s':{k:st.median(v) for k,v in T.items()},'min_s':{k:min(v) for k,v in T.items()},'max_s':{k:max(v) for k,v in T.items()},
     'relations_total':REL,'targets':8,'seed':SEED,'msolve':'0.6.5','arms_order':'shuffled per target'}
print(json.dumps(res))
