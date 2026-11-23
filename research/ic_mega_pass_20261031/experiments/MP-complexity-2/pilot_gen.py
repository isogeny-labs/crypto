import random, sys, subprocess, re, math, time
from sympy import isprime, sqrt_mod, nextprime
class CNF:
    def __init__(s): s.n=1; s.cl=[[1]]  # var1 = TRUE
    def new(s): s.n+=1; return s.n
    def xor3(s,a,b,c):
        o=s.new()
        for sa in(1,-1):
            for sb in(1,-1):
                for sc in(1,-1):
                    # o = a^b^c ; forbid assignments inconsistent
                    par=(sa<0)^(sb<0)^(sc<0)  # value of xor given literals true
                    pass
        # simple: clauses for o<->a^b^c
        for m in range(8):
            va,vb,vc=(m>>2)&1,(m>>1)&1,m&1
            vo=va^vb^vc
            s.cl.append([-a if va else a,-b if vb else b,-c if vc else c, o if vo else -o])
        return o
    def maj(s,a,b,c):
        o=s.new()
        for m in range(8):
            va,vb,vc=(m>>2)&1,(m>>1)&1,m&1
            vo=1 if va+vb+vc>=2 else 0
            s.cl.append([-a if va else a,-b if vb else b,-c if vc else c, o if vo else -o])
        return o
    def and2(s,a,b):
        o=s.new(); s.cl+= [[-o,a],[-o,b],[o,-a,-b]]; return o
    def xor2(s,a,b):
        o=s.new(); s.cl+=[[-o,a,b],[-o,-a,-b],[o,-a,b],[o,a,-b]]; return o
def compress(f,cols):
    # cols: dict w->list of literals (1 = TRUE const). returns bit list
    res=[]; w=0; maxw=max(cols)+1 if cols else 0
    while w<maxw+80:
        c=cols.get(w,[])
        while len(c)>1:
            if len(c)>=3:
                a,b,d=c.pop(),c.pop(),c.pop()
                cols.setdefault(w+1,[]).append(f.maj(a,b,d)); c.append(f.xor3(a,b,d))
            else:
                a,b=c.pop(),c.pop()
                cols.setdefault(w+1,[]).append(f.and2(a,b)); c.append(f.xor2(a,b))
        res.append(c[0] if c else -1)  # -1 = FALSE
        w+=1
        if w>maxw and not any(cols.get(j) for j in range(w,w+80)): break
    return res
def addvar(cols,bits,shift=0):
    for i,b in enumerate(bits): cols.setdefault(i+shift,[]).append(b)
def addconst(cols,c,shift=0):
    i=0
    while c>>i:
        if (c>>i)&1: cols.setdefault(i+shift,[]).append(1)
        i+=1
def mulbits(f,x,y):
    cols={}
    for i,a in enumerate(x):
        for j,b in enumerate(y): cols.setdefault(i+j,[]).append(f.and2(a,b))
    return compress(f,cols)
def build(k,p,a,b,xR):
    f=CNF(); x1=[f.new() for _ in range(k)]; x2=[f.new() for _ in range(k)]
    u=mulbits(f,x1,x2)
    cols={}; addvar(cols,x1); addvar(cols,x2); s=compress(f,cols)
    s2=mulbits(f,s,s); u2=mulbits(f,u,u); su=mulbits(f,s,u)
    c2=xR*xR%p; c1=2*xR%p
    def cm(v,c,sh=0,cols=None):
        i=0
        while c>>i:
            if (c>>i)&1: addvar(cols,v,i+sh)
            i+=1
    Pc={};Nc={}
    cm(s2,c2%p,0,Pc); addvar(Pc,u2); addconst(Pc,a*a%p)
    cm(u,4*c2%p,0,Nc); cm(su,c1,0,Nc); cm(s,c1*a%p,0,Nc); addconst(Nc,2*c1*b%p)
    cm(u,2*a%p,0,Nc); cm(s,4*b%p,0,Nc)
    X=lambda v:(1<<len(v))-1  # bound
    maxP=c2*(2**(k+1))**2+(2**(2*k))**2+a*a
    maxN=4*c2*2**(2*k)+c1*2**(k+1)*2**(2*k)+c1*a*2**(k+1)+2*c1*b+2*a*2**(2*k)+4*b*2**(k+1)
    H=-(-maxN//p)*p
    qbits=(maxP+H).bit_length()-p.bit_length()+2
    q=[f.new() for _ in range(qbits)]
    cm(q,p,0,Nc)
    addconst(Pc,H)
    L=compress(f,Pc); R=compress(f,Nc)
    n=max(len(L),len(R))
    for i in range(n):
        l=L[i] if i<len(L) else -1; r=R[i] if i<len(R) else -1
        f.cl+= [[-l,r],[l,-r]]
    f.cl.append([-(-1)] if False else [1]) 
    return f,x1,x2
def write(f,path):
    cl=[c for c in f.cl]
    # literal -1 means FALSE const: -1 literal is false of var1, consistent
    with open(path,'w') as fh:
        fh.write(f"p cnf {f.n} {len(cl)}\n")
        for c in cl: fh.write(" ".join(map(str,c))+" 0\n")
def ecadd(p,a,P,Q):
    if P is None: return Q
    if Q is None: return P
    if P[0]==Q[0] and (P[1]+Q[1])%p==0: return None
    if P==Q: l=(3*P[0]*P[0]+a)*pow(2*P[1],-1,p)%p
    else: l=(Q[1]-P[1])*pow(Q[0]-P[0],-1,p)%p
    x=(l*l-P[0]-Q[0])%p; return (x,(l*(P[0]-x)-P[1])%p)
def instance(k,L,rng,planted):
    while True:
        p=nextprime(rng.randrange(2**(L-1),2**L)); a=rng.randrange(p); b=rng.randrange(p)
        if (4*a**3+27*b*b)%p==0: continue
        pts=[]
        for x in range(1,2**k):
            r=(x**3+a*x+b)%p
            if pow(r,(p-1)//2,p)==1: pts.append((x,sqrt_mod(r,p)))
        if len(pts)>=2: break
    if planted:
        P1,P2=rng.sample(pts,2); R=ecadd(p,a,P1,P2)
        if R is None: return instance(k,L,rng,planted)
        return p,a,b,R[0]
    return p,a,b,rng.randrange(p)
def run(solver,path,cap):
    t=time.time()
    if solver=='kissat': o=subprocess.run(['kissat','-n','--conflicts=%d'%cap,path],capture_output=True,text=True)
    else: o=subprocess.run(['cryptominisat5','--maxconfl',str(cap),'--verb','1',path],capture_output=True,text=True)
    dt=time.time()-t; out=o.stdout
    st='SAT' if 's SATISFIABLE' in out else 'UNSAT' if 's UNSATISFIABLE' in out else 'UNK'
    m=re.search(r'c conflicts:\s+(\d+)',out) if solver=='kissat' else re.search(r'c conflicts\s*:\s*(\d+)',out)
    return st,(int(m.group(1)) if m else None),dt
if __name__=='__main__':
    k=int(sys.argv[1]); L=int(sys.argv[2]); n=int(sys.argv[3]); cap=int(sys.argv[4]); solver=sys.argv[5]
    rng=random.Random(k*1000+L)
    for planted in (1,0):
        for i in range(n):
            p,a,b,xR=instance(k,L,rng,planted)
            f,x1,x2=build(k,p,a,b,xR); write(f,'t.cnf')
            if i==0 and planted==1 and False: pass
            st,c,dt=run(solver,'t.cnf',cap)
            print(k,L,planted,st,c,round(dt,2),f.n,len(f.cl),flush=True)
