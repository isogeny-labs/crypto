"""Build H(s1,s2,s3,t) = S5(x(Y),xR)*prod(1-Y_i)^8 rewritten in D4 invariants, by interpolation mod p.
Also: curve/planted-root generation for arm Y (small Y) and arm X (small x)."""
import cypari2, random, itertools
pari = cypari2.Pari(); pari.allocatemem(2*10**9)
# Semaev S3 for y^2=x^3+A x^2+B x (a2=A, a4=B, a6=0); verified on 50 random point triples
pari('S3(a,b,c,A,B)=(a-b)^2*c^2-2*((a+b)*(a*b+B)+2*A*a*b)*c+(a*b-B)^2')
# S4(a,b,c,u) in u (resultant over v); then S5 at numeric x1..x4,xR
pari('S5num(x1,x2,x3,x4,xR,A,B)=my(s4=polresultant(S3(x3,x4,v,A,B),S3(xR,u,v,A,B),v)); polresultant(S3(x1,x2,u,A,B),s4,u)')
MONOS = [(a,b,c,d) for d in range(9) for a in range(5) for b in range(5) for c in range(5) if 2*(a+b+c)+d<=8]
assert len(MONOS)==105
WDEG = [2*a+4*b+6*c+4*d for (a,b,c,d) in MONOS]

def inv_vals(Y, p=None):
    q=[y*y for y in Y]
    s1=sum(q); s2=sum(q[i]*q[j] for i,j in itertools.combinations(range(4),2))
    s3=sum(q[i]*q[j]*q[k] for i,j,k in itertools.combinations(range(4),3)); t=Y[0]*Y[1]*Y[2]*Y[3]
    v=(s1,s2,s3,t)
    return tuple(x%p for x in v) if p else v

def mono_eval(v, p):
    return [pow(v[0],a,p)*pow(v[1],b,p)*pow(v[2],c,p)*pow(v[3],d,p)%p for (a,b,c,d) in MONOS]

def G(Y, xR, A, B, c, p):
    xs=[]
    den=1
    for y in Y:
        if (1-y)%p==0: return None
        xs.append(c*(1+y)*pow(1-y,-1,p)%p); den=den*pow(1-y,8,p)%p
    P=pari(f'S5num({xs[0]},{xs[1]},{xs[2]},{xs[3]},{xR},Mod({A},{p}),Mod({B},{p}))')
    return int(pari.lift(P))*den%p

def interpolate(xR, A, B, c, p, npts=220, rng=None):
    rng=rng or random.Random(0)
    rows=[];rhs=[]
    while len(rows)<npts:
        Y=[rng.randrange(p) for _ in range(4)]
        g=G(Y,xR,A,B,c,p)
        if g is None: continue
        rows.append(mono_eval(inv_vals(Y,p),p)); rhs.append(g)
    M=pari.matrix(len(rows),105,[pari.Mod(x,p) for r in rows for x in r])
    V=pari.Col([pari.Mod(x,p) for x in rhs])
    sol=pari.matinverseimage(M,V)
    if len(sol)==0: return None, {'consistent':False}
    coef=[int(pari.lift(s)) for s in sol]
    # consistency check on fresh points
    bad=0
    for _ in range(40):
        Y=[rng.randrange(p) for _ in range(4)]; g=G(Y,xR,A,B,c,p)
        if g is None: continue
        if sum(a*b for a,b in zip(coef,mono_eval(inv_vals(Y,p),p)))%p!=g: bad+=1
    # even-flip scalar and S4 check on raw G
    Y=[rng.randrange(p) for _ in range(4)]; g0=G(Y,xR,A,B,c,p)
    lam=[G([-Y[0],-Y[1],Y[2],Y[3]],xR,A,B,c,p)*pow(g0,-1,p)%p, G([Y[1],Y[0],Y[3],Y[2]],xR,A,B,c,p)*pow(g0,-1,p)%p,
         G([-Y[0],Y[1],Y[2],Y[3]],xR,A,B,c,p)*pow(g0,-1,p)%p]
    return coef, {'consistent':bad==0,'fresh_bad':bad,'support':sum(1 for x in coef if x),
                  'lambda_evenflip':lam[0],'lambda_swap':lam[1],'oddflip_is_1':lam[2]==1}

def curve(p, rng):
    while True:
        A=rng.randrange(p); c=rng.randrange(1,p); B=c*c%p
        if (A*A-4*B)%p and B: return A,B,c

DISTINCT_SQ=False
def on_curve(x,A,B,p):  # Euler criterion in python (PARI issquare(Mod) on 4096-bit p stalls on modulus checks)
    v=x*(x*x+A*x+B)%p
    return v==0 or pow(v,(p-1)//2,p)==1
def sqrt34(v,p):
    assert p%4==3; r=pow(v,(p+1)//4,p); assert r*r%p==v%p; return r
def eadd(P,Q,A,p,B=None):
    """full affine addition on y^2=x^3+Ax^2+Bx; None = infinity."""
    if P is None: return Q
    if Q is None: return P
    (x1,y1),(x2,y2)=P,Q
    if x1==x2:
        if (y1+y2)%p==0 or y1%p==0: return None
        lam=(3*x1*x1+2*A*x1+B)*pow(2*y1,-1,p)%p
    else: lam=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(lam*lam-A-x1-x2)%p; return (x3,(lam*(x1-x3)-y1)%p)

def plant(p,A,B,c,X,rng,model='Y'):
    for _ in range(50):
        try: return _plant(p,A,B,c,X,rng,model)
        except RuntimeError as e:
            if 'sum is O' not in str(e): raise
    raise RuntimeError('plant failed')
def _plant(p,A,B,c,X,rng,model='Y'):
    """4 points with |Y|<=X (model Y) or |x|<=X (model X), return Ys (lifted ints), xR."""
    pts=[];Ys=[]
    tries=0
    while len(pts)<4:
        tries+=1
        if tries>2000: raise RuntimeError('no small points')
        if model=='Y':
            y=rng.randint(-X,X)
            if y in (0,1,-1): continue
            x=c*(1+y)*pow(1-y,-1,p)%p
        else:
            x=rng.randint(2,X); 
        if not on_curve(x,A,B,p): continue
        yy=sqrt34(x*(x*x+A*x+B)%p,p)
        if rng.random()<.5: yy=(-yy)%p
        if any(q[0]==x for q in pts): continue   # distinct x keeps the affine adds generic
        if DISTINCT_SQ and model=='Y' and any(abs(z)==abs(y) for z in Ys): continue
        pts.append((x,yy))
        Y=(x-c)*pow(x+c,-1,p)%p; Ys.append(Y if Y<p//2 else Y-p)
    S=eadd(eadd(eadd(pts[0],pts[1],A,p,B),pts[2],A,p,B),pts[3],A,p,B)
    if S is None: raise RuntimeError('sum is O')
    xR=S[0]
    return Ys,xR
