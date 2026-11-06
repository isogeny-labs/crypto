# Throwaway scoping enumeration (not research code): genus-2/3 curves over F_2,
# their L-polynomials and Jacobian orders over F_2.
import sys, itertools
import numpy as np

MODS = {1:0b11, 2:0b111, 3:0b1011, 4:0b10011, 5:0b100101, 6:0b1000011, 12:0x1053}
class GF:
    def __init__(self,k):
        self.k=k; self.q=1<<k; m=MODS[k]
        q=self.q
        mul=np.zeros((q,q),dtype=np.int64)
        for a in range(q):
            for b in range(q):
                r=0; x=a; y=b
                while y:
                    if y&1: r^=x
                    y>>=1; x<<=1
                    if x&q: x^=m
                mul[a,b]=r
        self.mul=mul
        self.sq=np.array([mul[a,a] for a in range(q)])
        tr=np.zeros(q,dtype=np.int64)
        for a in range(q):
            t=0; x=a
            for i in range(k):
                t^=x; x=self.sq[x]
            tr[a]=t
        self.tr=tr
        inv=np.zeros(q,dtype=np.int64)
        for a in range(1,q):
            for b in range(1,q):
                if mul[a,b]==1: inv[a]=b; break
        self.inv=inv
    def pw(self,a,e):
        r=1
        for _ in range(e): r=self.mul[r,a]
        return r

FIELDS={k:GF(k) for k in (1,2,3,4,5,6)}

def polyval_table(F, deg):
    # returns array P[x, i] = x^i for all x in F
    q=F.q; P=np.zeros((q,deg+1),dtype=np.int64)
    for x in range(q):
        v=1
        for i in range(deg+1):
            P[x,i]=v; v=F.mul[v,x]
    return P

def hyper_count(F,P,h,f,g):
    # y^2 + h y = f over F; h,f lists of coeffs (low->high)
    q=F.q
    hx=np.zeros(q,dtype=np.int64); fx=np.zeros(q,dtype=np.int64)
    for i,c in enumerate(h):
        if c: hx^=P[:,i]
    for i,c in enumerate(f):
        if c: fx^=P[:,i]
    cnt=0
    h0=(hx==0)
    cnt+=int(h0.sum())
    nz=~h0
    hinv=F.inv[hx[nz]]; w=F.mul[fx[nz], F.mul[hinv,hinv]]
    cnt+=2*int((F.tr[w]==0).sum())
    H=h[g+1] if len(h)>g+1 else 0
    F2=f[2*g+2] if len(f)>2*g+2 else 0
    if H==0: cnt+=1
    else:
        cnt+= 2 if F.tr[F.mul[F2, F.mul[F.inv[H],F.inv[H]]]]==0 else 0
    return cnt

def lpoly_from_counts(N, g, q=2):
    # Newton identities: S_k = q^k+1-N_k = sum alpha_i^k ; L(T)=prod(1-alpha_i T) = sum c_i T^i
    S=[None]+[q**k+1-N[k-1] for k in range(1,g+1)]
    c=[1]
    for k in range(1,g+1):
        # e_k = (-1)^{k-1}/k * (S_k + sum_{i=1}^{k-1} (-1)^i e_{k-i} S_i ) ... use standard: k e_k = sum_{i=1}^k (-1)^{i-1} e_{k-i} S_i
        s=0
        for i in range(1,k+1):
            s+=(-1)**(i-1)*c[k-i]*S[i]
        assert s%k==0
        c.append(s//k)
    # L(T)=sum (-1)^i e_i T^i ; functional eq c_{2g-i} = q^{g-i} c_i
    L=[(-1)**i*c[i] for i in range(g+1)]
    for i in range(g+1,2*g+1):
        L.append(q**(i-g)*L[2*g-i])
    return L

def predict_counts(L,g,q,upto):
    # N_k = q^k + 1 - sum alpha^k ; power sums via Newton from L coefficients
    e=[(-1)**i*L[i] for i in range(2*g+1)]  # e_i with L = sum (-1)^i e_i T^i
    S=[0]*(upto+1)
    for k in range(1,upto+1):
        s=0
        for i in range(1,min(k,2*g)+1):
            s+=(-1)**(i-1)*e[i]*S[k-i] if k-i>0 else 0
        # k e_k term
        if k<=2*g: s+= (-1)**(k-1)*k*e[k]
        S[k]=s
    return [q**k+1-S[k] for k in range(1,upto+1)]

def hyper_smooth(h,f,g):
    # affine: roots of h in F_{2^12}; singular iff h'(x0) y0 + f'(x0) = 0 with y0=sqrt(f(x0))
    F=GF12
    def ev(p,x):
        v=0; xp=1
        for c in p:
            if c: v^=xp
            xp=F.mul[xp,x]
        return v
    def der(p): return [ (p[i] if i%2==1 else 0) for i in range(1,len(p))] or [0]
    hp=der(h); fp=der(f)
    for x in range(F.q):
        if ev(h,x)==0:
            y0=F.pw(ev(f,x), 1<<11)  # sqrt in F_{2^12}: a^(2^11)
            if F.mul[ev(hp,x),y0]^ev(fp,x)==0: return False
    H=h[g+1] if len(h)>g+1 else 0
    if H==0:
        F2=f[2*g+2] if len(f)>2*g+2 else 0
        F1=f[2*g+1] if len(f)>2*g+1 else 0
        hg=h[g] if len(h)>g else 0
        # singular at infinity iff hg*sqrt(F2)+F1==0 (all in F_2: sqrt(F2)=F2)
        if (hg*F2 + F1)%2==0: return False
    return True

GF12=GF(12)

def enumerate_hyper(g, want):
    """want(L) -> bool filter; returns list of (h,f,L,N)"""
    out=[]
    Ptab={k:polyval_table(FIELDS[k],2*g+2) for k in FIELDS}
    for hb in range(1<<(g+2)):
        h=[(hb>>i)&1 for i in range(g+2)]
        for fb in range(1<<(2*g+3)):
            f=[(fb>>i)&1 for i in range(2*g+3)]
            degf=max([i for i,c in enumerate(f) if c], default=-1)
            degh=max([i for i,c in enumerate(h) if c], default=-1)
            if not (degf>=2*g+1 or degh==g+1): continue
            if degh<0: continue  # h=0 -> inseparable, not a curve of this genus in char 2
            N=[hyper_count(FIELDS[k],Ptab[k],h,f,g) for k in range(1,2*g+1)]
            try:
                L=lpoly_from_counts(N[:g],g)
            except AssertionError:
                continue
            pred=predict_counts(L,g,2,2*g)
            if pred!=N: continue
            if not want(L): continue
            if not hyper_smooth(h,f,g): continue
            out.append((h,f,L,N))
    return out

def quartic_scan(want):
    mons=[(a,b,4-a-b) for a in range(5) for b in range(5-a)]
    out=[]
    tabs={}
    for k in (1,2,3,4,5,6):
        F=FIELDS[k]; q=F.q
        pts=[(x,y,1) for x in range(q) for y in range(q)]+[(x,1,0) for x in range(q)]+[(1,0,0)]
        V=np.zeros((len(mons),len(pts)),dtype=np.int64)
        for mi,(a,b,c) in enumerate(mons):
            for pi,(x,y,z) in enumerate(pts):
                V[mi,pi]=F.mul[F.mul[F.pw(x,a),F.pw(y,b)],F.pw(z,c)]
        tabs[k]=V
    for S in range(1,1<<15):
        idx=[i for i in range(15) if (S>>i)&1]
        N=[]
        for k in (1,2,3,4,5,6):
            v=np.bitwise_xor.reduce(tabs[k][idx],axis=0)
            N.append(int((v==0).sum()))
        try: L=lpoly_from_counts(N[:3],3)
        except AssertionError: continue
        if predict_counts(L,3,2,6)!=N: continue
        if not want(L): continue
        out.append((S,[mons[i] for i in idx],L,N))
    return out

if __name__=="__main__":
    mode=sys.argv[1]
    if mode=="g2":
        res=enumerate_hyper(2, lambda L: any(sum(L)%n==0 for n in (5,7,11,13)))
        for h,f,L,N in res: print("g2",h,f,"L=",L,"N=",N,"#Jac=",sum(L))
        print("total",len(res))
    elif mode=="g3h":
        res=enumerate_hyper(3, lambda L: sum(L)==131)
        for h,f,L,N in res: print("g3h",h,f,"L=",L,"N=",N,"#Jac=",sum(L))
        print("total",len(res))
    elif mode=="g3q":
        res=quartic_scan(lambda L: sum(L)==131)
        for S,m,L,N in res: print("g3q",S,m,"L=",L,"N=",N,"#Jac=",sum(L))
        print("total",len(res))
    elif mode=="g3all":
        # distribution of #Jac for all smooth genus-3 hyperelliptic
        res=enumerate_hyper(3, lambda L: True)
        from collections import Counter
        c=Counter(sum(L) for _,_,L,_ in res)
        print(sorted(c.items()))
