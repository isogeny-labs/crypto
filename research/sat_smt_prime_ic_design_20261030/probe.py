import random, sys, subprocess, re, time
from sympy import nextprime
def inv(a,p): return pow(a,-1,p)
def on_curve_pts(p,a,b,B):
    # points with x<B
    pts=[]
    for x in range(B):
        r=(x*x*x+a*x+b)%p
        if pow(r,(p-1)//2,p)==1:
            # sqrt via Tonelli (p%4==3 assumed)
            y=pow(r,(p+1)//4,p); pts.append((x,y))
    return pts
def add(P,Q,p,a):
    (x1,y1),(x2,y2)=P,Q
    if x1==x2:
        if (y1+y2)%p==0: return None
        l=(3*x1*x1+a)*inv(2*y1,p)%p
    else: l=(y2-y1)*inv(x2-x1,p)%p
    x3=(l*l-x1-x2)%p; return (x3,(l*(x1-x3)-y1)%p)
def S3(x1,x2,x3,a,b,p):
    return ((x1-x2)**2*x3**2-2*((x1+x2)*(x1*x2+a)+2*b)*x3+((x1*x2-a)**2-4*b*(x1+x2)))%p
def smt(p,a,b,k,xr,m):
    n=p.bit_length(); W=2*n+3
    P=f"(_ bv{p} {W})"
    def c(v): return f"(_ bv{v%p} {W})"
    def ze(v,w): return f"((_ zero_extend {W-w}) {v})"
    def mul(u,v): return f"(bvurem (bvmul {u} {v}) {P})"
    def add_(u,v): return f"(bvurem (bvadd {u} {v}) {P})"
    def sub(u,v): return f"(bvurem (bvadd {u} (bvsub {P} {v})) {P})"
    out=["(set-logic QF_BV)"]
    xs=[]
    for i in range(m):
        out.append(f"(declare-const x{i} (_ BitVec {k}))"); xs.append(ze(f"x{i}",k))
    def s3(x1,x2,x3):
        d=sub(x1,x2); d2=mul(d,d); x32=mul(x3,x3)
        t1=mul(d2,x32)
        s=add_(x1,x2); pr=mul(x1,x2)
        t2=mul(mul(add_(mul(s,add_(pr,c(a))),c(2*b)),x3),c(2))
        q=sub(pr,c(a)); t3=sub(mul(q,q),mul(c(4*b),s))
        return add_(sub(t1,t2),t3)
    # chain: ((x0+x1)=u1, (u1+x2)=u2 ... ) last = R
    cur=xs[0]
    for i in range(1,m):
        if i==m-1: nxt=c(xr)
        else:
            out.append(f"(declare-const u{i} (_ BitVec {n}))"); nxt=ze(f"u{i}",n)
            out.append(f"(assert (bvult u{i} {P.replace(str(W),str(n))}))" if False else f"(assert (bvult {nxt} {P}))")
        out.append(f"(assert (= {s3(cur,xs[i],nxt)} {c(0)}))")
        cur=nxt
    out.append("(check-sat)")
    return "\n".join(out)
def run(f,tl):
    t=time.time()
    r=subprocess.run(["z3","-st",f"-T:{tl}",f],capture_output=True,text=True)
    wall=time.time()-t
    res=r.stdout.split()[0] if r.stdout.split() else "?"
    m=re.search(r":sat-conflicts\s+(\d+)",r.stdout); dec=re.search(r":sat-decisions\s+(\d+)",r.stdout)
    return res,int(m.group(1)) if m else 0,int(dec.group(1)) if dec else 0,wall
if __name__=="__main__":
    n=int(sys.argv[1]); ks=[int(x) for x in sys.argv[2].split(",")]; m=int(sys.argv[3]); tl=int(sys.argv[4]); seed=int(sys.argv[5]) if len(sys.argv)>5 else 1
    random.seed(seed)
    p=nextprime(random.getrandbits(n)|(1<<(n-1)))
    while p%4!=3: p=nextprime(p)
    a=random.randrange(p); b=random.randrange(p)
    for k in ks:
        B=1<<k
        pts=on_curve_pts(p,a,b,B)
        if len(pts)<3: continue
        for kind in ("planted","random"):
            if kind=="planted":
                Ps=[random.choice(pts) for _ in range(m)]
                R=Ps[0]
                for Q in Ps[1:]:
                    R=add(R,Q,p,a)
                    if R is None: break
                if R is None: continue
                xr=R[0]
            else: xr=random.randrange(p)
            import os
            f=os.path.join(os.path.dirname(os.path.abspath(__file__)),f"q_{n}_{k}_{m}_{kind}.smt2")
            open(f,"w").write(smt(p,a,b,k,xr,m))
            res,cf,dec,w=run(f,tl)
            print(f"n={n} p={p} m={m} k={k} B={B} {kind:8s} {res:8s} conflicts={cf} decisions={dec} wall={w:.1f}s",flush=True)
