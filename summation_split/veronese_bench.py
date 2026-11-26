"""Exact finite-field affine-slice gates, then bounded SymPy lex diagnostics."""
import csv,random,time,resource,multiprocessing as mp
from functools import lru_cache
from pathlib import Path
import sympy as s
from sympy.polys.rings import ring
from .veronese import system, geometric_system

# Independent homogeneous resultant, fixed Sylvester degrees.
def resultant(f,g,p):
    m,n=len(f)-1,len(g)-1
    M=[([0]*j+f[::-1]+[0]*(n-1-j)) for j in range(n)]
    M += [([0]*j+g[::-1]+[0]*(m-1-j)) for j in range(m)]
    det=1
    for i in range(m+n):
        k=next((k for k in range(i,m+n) if M[k][i]%p),None)
        if k is None:return 0
        if i!=k:M[i],M[k]=M[k],M[i];det=-det
        c=M[i][i]%p;det=det*c%p;iv=pow(c,-1,p)
        for j in range(i+1,m+n):
            a=M[j][i]*iv%p
            for l in range(i+1,m+n):M[j][l]=(M[j][l]-a*M[i][l])%p
    return det%p

def coeff(x,y,a,b,p):
    return [((x*y-a)**2-4*b*(x+y))%p,-2*((x+y)*(x*y+a)+2*b)%p,(x-y)**2%p]

@lru_cache(None)
def interpolation_matrix(p,n):
    rows=[]
    for i in range(n+1):
        den=1;v=[1]
        for j in range(n+1):
            if i==j:continue
            den=den*(i-j)%p;w=[0]*(len(v)+1)
            for k,c in enumerate(v):w[k]=(w[k]-j*c)%p;w[k+1]=(w[k+1]+c)%p
            v=w
        rows.append([c*pow(den,-1,p)%p for c in v])
    return rows

def baseline(p,a,b,xs):
    f=coeff(*xs[:2],a,b,p)
    for x in xs[2:-2]:
        n=2*(len(f)-1)
        pts=[resultant(f,coeff(z,x,a,b,p),p) for z in range(n+1)]
        M=interpolation_matrix(p,n);f=[sum(pts[i]*M[i][j] for i in range(n+1))%p for j in range(n+1)]
    return resultant(f,coeff(*xs[-2:],a,b,p),p)==0

class Oracle:
    def __init__(self,p,a,b,d):
        self.p,self.a,self.b,self.d=p,a,b,d
        self.values=[(u,v) for u in range(p) for v in range(p)]
        self.sqrt={}
        for q in self.values:self.sqrt.setdefault(self.mul(q,q),[]).append(q)
    def add(self,x,y):return tuple((i+j)%self.p for i,j in zip(x,y))
    def neg(self,x):return tuple(-i%self.p for i in x)
    def sub(self,x,y):return self.add(x,self.neg(y))
    def mul(self,x,y):
        u,v=x;r,h=y;p=self.p;return ((u*r+self.d*v*h)%p,(u*h+v*r)%p)
    def scale(self,x,k):return tuple(k*i%self.p for i in x)
    def inv(self,x):
        u,v=x;p=self.p;c=pow((u*u-self.d*v*v)%p,-1,p);return (u*c%p,-v*c%p)
    @lru_cache(None)
    def step(self,t,x):
        if t is None:return [(x,0)]
        a,b=self.a,self.b;add,sub,mul,sc=self.add,self.sub,self.mul,self.scale
        y=(x,0);v=sub(t,y);A=mul(v,v)
        xy=mul(t,y);B=sc(add(mul(add(t,y),add(xy,(a,0))),(2*b,0)),-2)
        z=sub(xy,(a,0));C=sub(mul(z,z),sc(add(t,y),4*b))
        if A==(0,0):
            return [None]+([] if B==(0,0) else [mul(self.neg(C),self.inv(B))])
        disc=sub(mul(B,B),sc(mul(A,C),4));iv=self.inv(sc(A,2))
        return [mul(add(self.neg(B),r),iv) for r in self.sqrt.get(disc,[])]
    def path(self,xs):
        states={(xs[0],0):[]}
        for x in xs[1:-1]:
            states={q:path+[q] for t,path in states.items() for q in self.step(t,x)}
        return states.get((xs[-1],0))

def symbolic_baseline(p,a,b,forms):
    R,t,u,z=ring('t,u,z',s.GF(p))
    def S(x,y,z):return (x-y)**2*z*z-2*((x+y)*(x*y+a)+2*b)*z+(x*y-a)**2-4*b*(x+y)
    def res(f,g,M,N):
        if not f or not g:return R.zero
        mf,ng=f.degree(t),g.degree(t)
        if mf<M and ng<N:return R.zero
        ans=R.from_expr(f.resultant(g).as_expr())
        if mf<M:ans*=(-1)**((M-mf)*N)*R.from_expr(g.as_expr().coeff(s.Symbol('t'),ng))**(M-mf)
        if ng<N:ans*=R.from_expr(f.as_expr().coeff(s.Symbol('t'),mf))**(N-ng)
        return ans
    xs=[c*u+d for c,d in forms];f=S(xs[0],xs[1],t);M=2
    for x in xs[2:-2]:
        f=R.from_expr(res(f,S(t,x,z),M,2).as_expr().subs(s.Symbol('z'),s.Symbol('t')));M*=2
    return res(f,S(t,xs[-2],xs[-1]),M,2).as_expr()

def worker(q,p,a,b,forms,arm):
    u=s.Symbol('u');xs=[c*u+d for c,d in forms];t=time.perf_counter()
    try:
        if arm=='monolithic':aux=[];eq=[symbolic_baseline(p,a,b,forms)]
        elif arm=='veronese':aux,eq,_=system(p,a,b,xs)
        else:aux,eq=geometric_system(a,b,xs,forms if arm=='geo_shared' else None,u)
        G=s.groebner(eq+[u**p-u],*aux,u,modulus=p,order='lex',method='buchberger')
        terminal=[g.as_expr() for g in G.polys if g.as_expr().free_symbols<={u}]
        roots=[i for i in range(p) if all(int(g.subs(u,i))%p==0 for g in terminal)]
        q.put(('ok',time.perf_counter()-t,resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,roots))
    except Exception as e:q.put(('error:'+repr(e),time.perf_counter()-t,None,None))

def main(out):
    rng=random.Random(20261123);u=s.Symbol('u');gates=[];bench=[];cases=[]
    for p,a,b,m in [(5,0,1,3),(7,1,1,3),(11,1,1,4),(19,2,3,5)]:
        leaves=s.symbols('x:'+str(m+1));aux,eq,d=system(p,a,b,leaves)
        assert max(s.total_degree(f) for f in eq)<=3
        check=s.lambdify([*leaves,*aux],eq,'math');O=Oracle(p,a,b,d);positives=0
        ga,ge=geometric_system(a,b,leaves);assert max(s.total_degree(f) for f in ge)<=3
        # Translate each geometric generator into a polynomial over Fp2.
        w=s.Symbol('omega');gs=[];gx=[]
        for i in range(m-2):
            U,V,Z,R,H=s.symbols(f'U{i} V{i} Z{i} R{i} H{i}');gx.extend((U,V,Z,R,H))
            gs.extend((U+V*w,Z,R+H*w))
        sub=dict(zip(ga, [*aux[:m+1],*gs]));gpoly=[s.Poly(s.expand(f.subs(sub,simultaneous=True)),w).rem(s.Poly(w*w-d,w)) for f in ge]
        gcheck=s.lambdify([*leaves,*aux[:m+1],*gx],[f.nth(j) for f in gpoly for j in [0,1]],'math')
        for i in range(50):
            forms=[(rng.randrange(1,p),rng.randrange(p)) for _ in leaves];roots=[]
            for v in range(p):
                xs=[(c*v+k)%p for c,k in forms];base=baseline(p,a,b,xs);path=O.path(xs)
                assert base==(path is not None),(p,m,xs,base,path)
                if base:
                    roots.append(v);positives+=1;vals=[x*x%p for x in xs]
                    for t in path[:-1]:
                        x,y,z=(1,0,0) if t is None else (*t,1)
                        vals.extend([x,y,z,(x*x+d*y*y)%p,2*x*y%p])
                    assert all(int(r)%p==0 for r in check(*xs,*vals))
                    assert all(int(r)%p==0 for r in gcheck(*xs,*vals))
                    # Shared-square is a definitional substitution in the same generators.
                    assert all((c*c*v*v+2*c*k*v+k*k-x*x)%p==0 for (c,k),x in zip(forms,xs))
            if i<2:cases.append((p,a,b,m,i,forms,roots))
        gates.append([p,a,b,m,50,50*p,positives,len(leaves)+len(aux),len(eq),3]);print('gate',gates[-1],flush=True)
    for p,a,b,m,i,forms,roots in cases:
        for arm in ['monolithic','veronese','geo','geo_shared']:
            q=mp.Queue();proc=mp.Process(target=worker,args=(q,p,a,b,forms,arm));proc.start();proc.join(5)
            if proc.is_alive():proc.terminate();proc.join();r=('timeout',None,None,None)
            elif q.empty():r=('no-result',None,None,None)
            else:r=q.get()
            status,sec,rss,got=r
            if status=='ok':assert got==roots,(p,m,arm,got,roots)
            bench.append([p,m,i,arm,status,sec,rss,len(roots),'NA','NA']);print('bench',bench[-1],flush=True)
    for name,header,rows in [('veronese_gate.csv','p,a,b,m,slices,points,positive_certificates,variables,generators,max_degree',gates),('veronese_bench.csv','p,m,case,arm,status,wall_s,peak_rss_kib,expected_roots,d_F4,first_fall',bench)]:
        with open(Path(out)/name,'w') as f:w=csv.writer(f);w.writerow(header.split(','));w.writerows(rows)

if __name__=='__main__':main(Path(__file__).parent)
