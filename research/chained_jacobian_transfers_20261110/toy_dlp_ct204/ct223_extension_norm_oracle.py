#!/usr/bin/env python3
"""CT223: independent F_101^2 closed-degree-2 prime divisor norm/projector audit.

This is exact finite-field arithmetic on public toy curves, not a cold-cost IC solve.
No external dependencies. The different quadratic and Cantor arithmetic are intentionally
implemented here instead of imported from the previous CT222 Node.js oracle.
"""
import json
import sys
if not __debug__:
    raise SystemExit("CT223 requires assertions: Python -O prohibited")

P=101; NONSQUARE=2; R=109; H=1060494; J=115593846; PROJ=80597544; G=4
assert pow(NONSQUARE,(P-1)//2,P)==P-1
Z=(0,0); O=(1,0)
def fp(a): return (a%P,0)
def qa(a,b):return ((a[0]+b[0])%P,(a[1]+b[1])%P)
def qneg(a):return ((-a[0])%P,(-a[1])%P)
def qs(a,b):return qa(a,qneg(b))
def qm(a,b):return ((a[0]*b[0]+NONSQUARE*a[1]*b[1])%P,(a[0]*b[1]+a[1]*b[0])%P)
def qi(a):
    den=(a[0]*a[0]-NONSQUARE*a[1]*a[1])%P
    if not den:raise ZeroDivisionError
    inv=pow(den,P-2,P)
    return ((a[0]*inv)%P,(-a[1]*inv)%P)
def qd(a,b):return qm(a,qi(b))
def qp(a,n):
    out=O
    while n:
        if n&1:out=qm(out,a)
        a=qm(a,a);n>>=1
    return out
def frob(a):return (a[0],-a[1]%P)

def ec_add(a,b):
    if a is None:return b
    if b is None:return a
    x,y=a;X,Y=b
    if x==X and y==qneg(Y):return None
    if x==X:
        if y==Z:return None
        slope=qd(qa(qm(fp(3),qm(x,x)),O),qm(fp(2),y))
    else:slope=qd(qs(Y,y),qs(X,x))
    xx=qs(qs(qm(slope,slope),x),X)
    yy=qs(qm(slope,qs(x,xx)),y)
    return xx,yy

def ec_mul(a,n):
    out=None
    while n:
        if n&1:out=ec_add(out,a)
        a=ec_add(a,a);n>>=1
    return out

def ec_frob(a):return None if a is None else (frob(a[0]),frob(a[1]))
def ec_valid(a):
    if a is None:return True
    x,y=a
    return qm(y,y)==qa(qa(qm(qm(x,x),x),x),fp(46))

def normalize(v):
    a=[x%P for x in v]
    while len(a)>1 and a[-1]==0:a.pop()
    return a

def pa(a,b):return normalize([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def ps(a,b):return normalize([(a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def pn(a):return normalize([-x for x in a])
def pm(a,b):
    r=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):r[i+j]+=x*y
    return normalize(r)
def pk(a,k):return normalize([x*k for x in a])
def pdm(a,b):
    assert b!=[0]
    r=a[:];q=[0]*max(1,len(a)-len(b)+1);c=pow(b[-1],P-2,P)
    while r!=[0] and len(r)>=len(b):
        j=len(r)-len(b);v=r[-1]*c%P;q[j]=v
        for i,w in enumerate(b):r[j+i]=(r[j+i]-v*w)%P
        r=normalize(r)
    return normalize(q),r
def pd(a,b):
    q,r=pdm(a,b)
    if r!=[0]:raise ArithmeticError(('nonexact',a,b,r))
    return q
def pr(a,b):return pdm(a,b)[1]
def monic(a):return pk(a,pow(a[-1],P-2,P))
def egcd(a,b):
    s0,t0,s1,t1=[1],[0],[0],[1]
    while b!=[0]:
        q,r=pdm(a,b)
        a,b=b,r;s0,s1=s1,ps(s0,pm(q,s1));t0,t1=t1,ps(t0,pm(q,t1))
    fac=pow(a[-1],P-2,P)
    return pk(a,fac),pk(s0,fac),pk(t0,fac)

F=[46,0,0,1,0,0,0,0,0,1]
ID=([1],[0])
def valid(d):
    u,v=d
    return u[-1]==1 and len(u)<=G+1 and (v==[0] or len(v)<len(u)) and pr(ps(pm(v,v),F),u)==[0]

def cantor(d1,d2):
    a,b=d1;c,d=d2
    dd1,s1,s2=egcd(a,c);dd,t1,t2=egcd(dd1,pa(b,d))
    u=monic(pd(pm(a,c),pm(dd,dd)))
    num=pa(pa(pm(pm(pm(t1,s1),a),d),pm(pm(pm(t1,s2),c),b)),pm(t2,pa(pm(b,d),F)))
    v=pr(pd(num,dd),u)
    while len(u)>G+1:
        u=monic(pd(ps(F,pm(v,v)),u));v=pr(pn(v),u)
    out=(u,v)
    if not valid(out):raise ArithmeticError(('bad Cantor',out))
    return out

def negd(d):return (d[0],pr(pn(d[1]),d[0]))
def sd(d,n):
    out=ID
    while n:
        if n&1:out=cantor(out,d)
        n>>=1
        if n:d=cantor(d,d)
    return out

def alpha(pt):
    if pt is None:return ID
    x,y=pt
    if x[1] or y[1]:raise ArithmeticError('alpha needs rational E point')
    d=([(-x[0])%P,0,0,1],[y[0]])
    assert valid(d)
    return d

def ec_trace(pt):
    s=ec_add(pt,ec_frob(pt))
    assert ec_valid(s)
    if s is not None:assert s[0][1]==s[1][1]==0
    return s

def prime_divisor(u,y):
    assert u[1]!=0 and qm(y,y)==qa(qa(qp(u,9),qp(u,3)),fp(46))
    ubar=frob(u);ybar=frob(y)
    # X^2 - Tr(u) X + Norm(u), V(X) = s X + t, with V(u)=y
    tr=qa(u,ubar);norm=qm(u,ubar);s=qd(qs(y,ybar),qs(u,ubar));t=qs(y,qm(s,u))
    assert tr[1]==norm[1]==s[1]==t[1]==0
    U=[norm[0],(-tr[0])%P,1];V=[t[0],s[0]]
    D=(U,normalize(V));assert valid(D)
    x=qp(u,3);e=(x,y);assert ec_valid(e)
    return D,ec_trace(e)

def sample():
    squares={}
    for a in range(P):
        for b in range(P):
            y=(a,b)
            squares.setdefault(qm(y,y),y)
    primes=[];seen=set();vanishing=0
    for a in range(P):
        for b in range(1,P):
            u=(a,b)
            if u in seen:continue
            seen.add(u);seen.add(frob(u))
            rhs=qa(qa(qp(u,9),qp(u,3)),fp(46))
            y=squares.get(rhs)
            if y is None:continue
            d,tr=prime_divisor(u,y)
            if tr is None:vanishing+=1;continue
            primes.append((u,y,d,tr))
            if len(primes)==12:return primes,vanishing
    return primes,vanishing

def run():
    assert J==R*H and PROJ%R==1 and PROJ%H==0
    examples,zeros=sample()
    assert len(examples)==12
    assert zeros>=0
    samples=[];n_proj=n_order=n_wrong=n_nonzero=n_mix=0
    for i,(u,y,d,tr) in enumerate(examples):
        lhs=sd(d,PROJ);rhs=alpha(ec_mul(tr,73))
        assert lhs==rhs,('projector mismatch',i,u,y,d,lhs,rhs)
        assert sd(d,J)==ID
        assert sd(lhs,R)==ID and sd(cantor(d,negd(lhs)),H)==ID
        assert sd(lhs,PROJ)==lhs
        # The wrong norm sign should fail if the 109-primary trace is nonzero.
        wrong=alpha(ec_mul(tr,R-73))
        assert rhs!=wrong
        n_proj+=1;n_order+=1;n_wrong+=1;n_nonzero+=lhs!=ID
        samples.append({'u':list(u),'y':list(y),'mumford_u':d[0],'mumford_v':d[1],
                        'trace_x':None if tr is None else tr[0][0],
                        'trace_y':None if tr is None else tr[1][0],
                        'projected_nonzero':lhs!=ID})
    # Combined divisor with a rational C point whose support is explicitly known
    G_source=((2,0),(37,0))
    assert ec_valid(G_source)
    u0=next(u for u in range(P) if u**3%P==2)
    rat=([(-u0)%P,1],[37]);assert valid(rat)
    for i in range(0,8,2):
        d1,t1=examples[i][2:];d2,t2=examples[i+1][2:]
        d=cantor(cantor(d1,d2),rat)
        t=ec_add(ec_add(t1,t2),G_source)
        assert sd(d,PROJ)==alpha(ec_mul(t,73))
        n_mix+=1
    # The nontrivial geometric branch u=0 is defined over Fq^2. Its
    # Frobenius pair (0,+sqrt46),(0,-sqrt46) is hyperelliptically conjugate,
    # and hence represents the principal divisor (no rational nonzero class).
    yy=next(z for a in range(P) for b in range(1,P) if qm((a,b),(a,b))==fp(46) for z in [(a,b)])
    assert frob(yy)==qneg(yy)
    assert ec_trace((Z,yy)) is None
    assert ec_valid((Z,yy))
    out={'schema':'ct223-f101-genuine-quadratic-prime-norm-v1','pcc':'unverified',
         'p':P,'quadratic_non_square':NONSQUARE,'source_order':R,'jacobian_order':J,
         'projector_scalar':PROJ,'nonzero_degree2_projector_checks':n_proj,
         'full_order_checks':n_order,'wrong_norm_sign_rejections':n_wrong,
         'nonzero_projected':n_nonzero,'mixed_rational_degree2_checks':n_mix,
         'branch_pair_trace_zero':True,'degree2_examples':samples,
         'standing':'producer exact arithmetic; genuine quadratic prime input; no general norm pricing or PCC'}
    return out

if __name__=='__main__':
    ans=run()
    if '--self-test' in sys.argv:
        print('CT223_SELFTEST_PASS',ans['nonzero_degree2_projector_checks'],ans['mixed_rational_degree2_checks'],ans['wrong_norm_sign_rejections'])
    else:
        print(json.dumps(ans,sort_keys=True,indent=2))
