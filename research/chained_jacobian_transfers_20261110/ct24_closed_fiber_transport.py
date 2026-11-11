#!/usr/bin/env python3
"""CT24: explicit degree-2 reciprocal fiber after an elliptic 7-isogeny.

All data over F_101. Computes a genuine degree-zero curve-divisor
representative of pi^*(phi(P)-O) for every P in the order-109 source.
Not a Cantor-reduced Jacobian class nor the full proposed J0 -> J1 map.
[pcc:unverified]

Run: python3 -S research/chained_jacobian_transfers_20261110/ct24_closed_fiber_transport.py
"""
P = 101
SOURCE = (1,46)
TARGET = (26,58)
KERNEL = (72,100,35,1)
GENERATOR = (2,37)
CHARTS = (
    # label, genus-2 sextic (y^2=t^6+A*t^4+B*t^2+b), scaling
    ('B+A', (92,44,33), 7),
    ('B+B', (26,42,68), 21),
)


def mod(x): return x % P

def inv(x):
    assert x % P
    return pow(x, P-2, P)

def ec_add(x, y, ab):
    if x is None: return y
    if y is None: return x
    a, _ = ab
    u,v = x; w,z = y
    if u == w and (v+z) % P == 0: return None
    if u == w:
        assert v == z and v != 0
        slope = mod((3*u*u+a)*inv(2*v))
    else:
        slope = mod((z-v)*inv(w-u))
    X = mod(slope*slope-u-w)
    return X,mod(slope*(u-X)-v)

def ec_mul(x,n,ab):
    assert n >= 0
    q=None
    while n:
        if n&1:q=ec_add(q,x,ab)
        x=ec_add(x,x,ab);n//=2
    return q

def ec_points(ab):
    a,b=ab
    roots={}
    for y in range(P): roots.setdefault(mod(y*y),[]).append(y)
    return [None]+[(x,y) for x in range(P) for y in roots.get(mod(x*x*x+a*x+b),())]

def phi(Pt):
    """Kohel cubic-kernel 7-isogeny in differential-normalized coordinates."""
    if Pt is None: return None
    x,y=Pt; a,b=SOURCE
    c0,c1,c2,c3=KERNEL
    assert c3==1
    h=mod(x*x*x+c2*x*x+c1*x+c0)
    assert h, 'rational kernel pole is not permitted on the 109-group'
    d=mod(3*x*x+2*c2*x+c1)
    d2=mod(6*x+2*c2)
    f=mod(x*x*x+a*x+b)
    df=mod(3*x*x+a)
    A=mod(6*x*x+2*a)
    B=mod(d*d-h*d2)
    i=inv(h)
    X=mod(7*x+2*c2-A*d*i+4*f*B*i*i)
    der=mod(7-12*x*d*i-A*(d2*h-d*d)*i*i
            +4*(df*B*i*i+f*(d*d2-6*h)*i*i-2*f*B*d*i*i*i))
    return X,mod(y*der)


def iso_to_long(Pt, coeff, scale):
    """External target short -> internal reciprocal quotient long (U,V)."""
    if Pt is None:return None
    _,B,_=coeff
    x,y=Pt
    return (mod(x*inv(scale*scale)-B*inv(3)),
            mod(y*inv(scale**3)))


def iso_from_long(Pt,coeff,scale):
    if Pt is None:return None
    _,B,_=coeff
    U,V=Pt
    return mod(scale**2*(U+B*inv(3))),mod(scale**3*V)


def elliptic_rec_curve(coeff):
    A,B,b=coeff
    # long cubic V^2=U^3+B*U^2+b*A*U+b^2
    sh=mod(B*inv(3))
    return (mod(b*A-B*sh),mod(b*b-b*A*sh+2*sh**3))


def pullback_divisor(Pt, coeff):
    """Formal divisor fiber(pi,Pt) - D0. D0=(0,+sqrt(b))+(0,-sqrt(b)).

    Tags: zero, finite, infinity. "finite" has monic quadratic U(t)
    and linear V(t). We do NOT identify this with a reduced even-degree
    Mumford coordinate without handling its degree-two base divisor.
    """
    A,B,b=coeff
    if Pt is None:return ('zero',)
    U,V=Pt
    # verify the actual long reciprocal cubic, not only its short isomorphism
    assert mod(V*V-U**3-B*U*U-b*A*U-b*b)==0
    if U==0:
        assert V in (b,mod(-b))
        return ('infinity', '+' if V==b else '-')
    radius=mod(b*inv(U))
    slope=mod(V*inv(U))
    # Support t^2=radius, y=slope*t. Check the sextic equation
    # using formal reduction modulo t^2-radius, not scalar readback.
    assert mod(slope*slope*radius-(radius**3+A*radius**2+B*radius+b))==0
    # reciprocal quotient U=b/t^2,V=b*y/t^3 at either point of the fiber
    assert mod(b*inv(radius)) == U
    assert mod(b*slope*inv(radius)) == V
    return ('finite', mod(-radius),0,1,0,slope)


def decode_norm(D,coeff):
    """Compute pi_* of the explicit fiber minus base, using its polynomial data.
    As divisors on E, pi_*D=2*(Q)-(2*O): return 2Q as group point.
    """
    A,B,b=coeff
    if D[0]=='zero':return None
    if D[0]=='infinity':
        V=b if D[1]=='+' else mod(-b)
        Q=(0,V)
    else:
        _,constant,zero,one,v0,v1=D
        assert zero==v0==0 and one==1 and constant!=0
        r=mod(-constant)
        Q=mod(b*inv(r)),mod(b*v1*inv(r))
    assert mod(Q[1]*Q[1]-Q[0]**3-B*Q[0]**2-b*A*Q[0]-b*b)==0
    # EC group computation in short coordinates is separately implemented.
    sh=mod(B*inv(3))
    C=elliptic_rec_curve(coeff)
    return ec_mul((mod(Q[0]+sh),Q[1]),2,C)


def reverse_divisor(D):
    if D[0]=='zero':return D
    if D[0]=='infinity':return ('infinity','-' if D[1]=='+' else '+')
    assert D[0]=='finite'
    return D[:-1]+(mod(-D[-1]),)


def is_square(x):return x==0 or pow(x,(P-1)//2,P)==1


def bsgs(G,Q,curve,order=109):
    """Cold BSGS on the target elliptic factor; not a Jacobian group solver."""
    m=11  # ceil(sqrt(109))
    table={}
    R=None
    ops=0
    for i in range(m):
        table[R]=i
        R=ec_add(R,G,curve)
        ops+=1
    minus=(R[0],mod(-R[1])) if R is not None else None
    R=Q
    for j in range(m+1):
        if R in table:
            log=(j*m+table[R])%order
            assert ec_mul(G,log,curve)==Q
            return log,ops
        R=ec_add(R,minus,curve)
        ops+=1
    raise AssertionError('cold BSGS failed')


def run():
    pts=ec_points(SOURCE)
    assert len(pts)==109 and len(ec_points(TARGET))==109
    assert GENERATOR in pts and ec_mul(GENERATOR,109,SOURCE) is None
    assert len({ec_mul(GENERATOR,k,SOURCE) for k in range(109)})==109
    for name, coeff, scale in CHARTS:
        A,B,b=coeff
        assert b!=0
        sqrts=[y for y in range(P) if mod(y*y)==b]
        assert len(sqrts)==2
        C=elliptic_rec_curve(coeff)
        assert all(mod(C[0]*scale**4)==TARGET[0] and mod(C[1]*scale**6)==TARGET[1] for _ in (0,))
        cover_classes={}
        split=nonsplit=infinities=0
        exceptional_logs=[]
        cold_ops=0
        target_gen=phi(GENERATOR)
        for k in range(109):
            src=ec_mul(GENERATOR,k,SOURCE)
            E=phi(src)
            assert E is None or mod(E[1]*E[1]-E[0]**3-TARGET[0]*E[0]-TARGET[1])==0
            internal=iso_to_long(E,coeff,scale)
            assert iso_from_long(internal,coeff,scale)==E
            D=pullback_divisor(internal,coeff)
            cover_classes[k]=D
            if D[0]=='infinity':
                infinities+=1
                exceptional_logs.append((k,D[1]))
            if D[0]=='finite':
                radius=mod(-D[1])
                if is_square(radius):split+=1
                else:nonsplit+=1
            norm_short=decode_norm(D,coeff)
            target_2=ec_mul(E,2,TARGET)
            assert (None if norm_short is None else
                    (mod(scale**2*norm_short[0]),mod(scale**3*norm_short[1]))) == target_2
            neg=ec_mul(src,108,SOURCE)
            Dneg=pullback_divisor(iso_to_long(phi(neg),coeff,scale),coeff)
            assert reverse_divisor(D)==Dneg
            # Dual norm gives the source log without scalar-readback:
            # [55] pi_*D gives E; inverse phi is precomposition of its dual.
            recovered=ec_mul(target_2,55,TARGET)
            assert recovered==E
            log,ops=bsgs(target_gen,recovered,TARGET)
            assert log==k
            cold_ops+=ops
        assert len(set(cover_classes.values()))==109
        assert cover_classes[0]==('zero',)
        assert split+nonsplit+infinities==108
        assert exceptional_logs[0][0]+exceptional_logs[1][0]==109
        # Negative chart control: arbitrary scaling does not preserve target model.
        wrong_scale = scale+1
        wrong=elliptic_rec_curve(coeff)
        assert (mod(wrong[0]*wrong_scale**4),mod(wrong[1]*wrong_scale**6))!=TARGET
        print(name, '109/109 explicit divisor fibers and norm[2] passed',
              'split',split,'nonsplit',nonsplit,'ramified infinity',infinities,
              'exceptional logs',exceptional_logs, 'cold BSGS adds',cold_ops,
              'base divisor t=0 y=±',sqrts[0], '[pcc:unverified]')
    print('PASS genuine E109 -> genus2 divisor correspondences; full Jacobian Phi and solving-cost claims remain open [pcc:unverified]')

if __name__=='__main__':run()
