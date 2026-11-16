import sys,random,subprocess,time,re,json
class C:
    def __init__(s): s.n=0; s.cl=[]; s.T=s.new(); s.cl.append([s.T])
    def new(s): s.n+=1; return s.n
    def AND(s,a,b): o=s.new(); s.cl+= [[-o,a],[-o,b],[o,-a,-b]]; return o
    def XOR(s,a,b): o=s.new(); s.cl+=[[-o,a,b],[-o,-a,-b],[o,-a,b],[o,a,-b]]; return o
    def OR(s,a,b): o=s.new(); s.cl+=[[o,-a],[o,-b],[-o,a,b]]; return o
    def FA(s,a,b,c):
        t=s.XOR(a,b); return s.XOR(t,c), s.OR(s.AND(a,b),s.AND(t,c))
    def add(s,A,B):
        F=-s.T; L=max(len(A),len(B)); A=A+[F]*(L-len(A)); B=B+[F]*(L-len(B)); out=[]; c=F
        for a,b in zip(A,B): r,c=s.FA(a,b,c); out.append(r)
        return out+[c]
    def mul(s,A,B):
        acc=[-s.T]
        for i,b in enumerate(B):
            row=[-s.T]*i+[s.AND(a,b) for a in A]; acc=s.add(acc,row)
        return acc
    def cmul(s,A,c):
        acc=[-s.T]
        for i in range(c.bit_length()):
            if c>>i&1: acc=s.add(acc,[-s.T]*i+A)
        return acc
def S3(x1,x2,x3,a,b,p): return ((x1-x2)**2*x3**2-2*((x1+x2)*(x1*x2+a)+2*b)*x3+(x1*x2-a)**2-4*b*(x1+x2))%p
def inst(p,k,a,b,x3,free1=False):
    # coefficients of f(x1,x2) mod p
    import sympy
    X,Y=sympy.symbols('X Y'); f=sympy.Poly(sympy.expand((X-Y)**2*x3**2-2*((X+Y)*(X*Y+a)+2*b)*x3+(X*Y-a)**2-4*b*(X+Y)),X,Y)
    c=C(); k1=(p.bit_length() if free1 else k)
    x1=[c.new() for _ in range(k1)]; x2=[c.new() for _ in range(k)]
    pw1={0:None,1:x1}; pw2={0:None,1:x2}; pw1[2]=c.mul(x1,x1); pw2[2]=c.mul(x2,x2)
    tot=[-c.T]
    for (i,j),co in f.terms():
        co=int(co)%p
        if co==0: continue
        if i and j: m=c.mul(pw1[i],pw2[j])
        elif i: m=pw1[i]
        elif j: m=pw2[j]
        else: m=[c.T]
        tot=c.add(tot,c.cmul(m,co))
    L=len(tot); qb=max(1,L-p.bit_length()+1); q=[c.new() for _ in range(qb)]
    qp=c.cmul(q,p); L2=max(L,len(qp))
    tot=tot+[-c.T]*(L2-len(tot)); qp=qp+[-c.T]*(L2-len(qp))
    for u,v in zip(tot,qp): c.cl+=[[u,-v],[-u,v]]
    if free1: # x1<p
        pass
    return c
def run(c,solver):
    fn=sys.argv[1]
    with open(fn,'w') as fh:
        fh.write(f"p cnf {c.n} {len(c.cl)}\n"); fh.writelines(" ".join(map(str,cl))+" 0\n" for cl in c.cl)
    t=time.time()
    try: o=subprocess.run(solver+[fn],capture_output=True,text=True,timeout=60).stdout
    except subprocess.TimeoutExpired: return None,None,60
    m=re.search(r'c conflicts\s*:\s*(\d+)',o) or re.search(r'c\s+conflicts:\s+(\d+)',o)
    st='SAT' if re.search(r'^s SATISFIABLE',o,re.M) else 'UNSAT'
    return st,int(m.group(1)) if m else -1,time.time()-t
if __name__=='__main__':
    from sympy import nextprime
    random.seed(1); res=[]
    for pb in [20,24]:
        p=nextprime(2**pb-random.randrange(2**10)); a,b=random.randrange(p),random.randrange(p)
        for k in range(4,int(sys.argv[2])+1):
            for planted in (1,0):
                cs=[]
                for t in range(int(sys.argv[3])):
                    if planted:
                        # plant: pick x1,x2 small with x3 root of S3 in x3
                        import cypari2
                        while True:
                            x1,x2=random.randrange(2**k),random.randrange(2**k)
                            import sympy
                            Z=sympy.symbols('Z'); r=sympy.polys.galoistools
                            pol=sympy.Poly((x1-x2)**2*Z**2-2*((x1+x2)*(x1*x2+a)+2*b)*Z+(x1*x2-a)**2-4*b*(x1+x2),Z,modulus=p)
                            rts=[int(z) for z in sympy.polys.polytools.ground_roots(pol).keys()] if False else [z for z in range(0)]
                            from sympy.ntheory import sqrt_mod
                            cf_=[int(v)%p for v in pol.all_coeffs()]
                            if len(cf_)<3: continue
                            A_,B_,C_=cf_
                            if A_==0: continue
                            d=(B_*B_-4*A_*C_)%p; s=sqrt_mod(d,p)
                            if s is None: continue
                            x3=(-B_+s)*pow(2*A_,-1,p)%p; break
                    else: x3=random.randrange(p)
                    st,cf,tm=run(inst(p,k,a,b,x3),['cryptominisat5','--verb','1'])
                    cs.append((st,cf,round(tm,2)))
                res.append(dict(pb=pb,k=k,planted=planted,runs=cs)); print(json.dumps(res[-1]),flush=True)
