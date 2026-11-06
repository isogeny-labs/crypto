"""Reproduce Tian's published cubic example, including rational map evaluation.

The numerical isogeny is the published one, not a new general-purpose isogeny
constructor. The kernel's eligibility is checked independently. All target
Jacobian computations use actual divisors on the smooth plane quartic.
"""
import itertools
import json
import math
from pathlib import Path
from time import perf_counter
from sage.all import (GF, PolynomialRing, EllipticCurve, FunctionField, Curve,
                      ProjectiveSpace, Integer)
from common import ec_key, point_data, field_data, bsgs_ec, bsgs_generic


def certified_plane_orders(F):
    """Supply proven integral bases to Sage 10.6, avoiding generic normalization.

    This helper is ONLY for the independently checked smooth quartic below.
    Its y-leading coefficient is nonzero constant, so the affine coordinate
    ring is finite, smooth, and integrally closed over k[x]. Thus 1,y,y^2,y^3
    is a maximal-order basis. The same argument in the X=1 projective patch
    proves the infinite-order basis 1,y/x,(y/x)^2,(y/x)^3.

    Sage's private integral-basis hook is used, with no replacement of genus,
    divisor arithmetic, equality tests, scalar relations, or subgroup checks.
    """
    y=F.gen(); x=F(F.base_field().gen())
    F._maximal_order_basis=lambda: [y**i for i in range(4)]
    Fi,back,to_inverted=F._inversion_isomorphism()
    v=to_inverted(y/x)
    Fi._maximal_order_basis=lambda: [v**i for i in range(4)]
    return F


def curve_data():
    k=GF(523); R=PolynomialRing(k,'u'); u=R.gen()
    K=GF(523**3,name='w',modulus=u**3+2); w=K.gen()
    a=82*w*w+92*w+140; b=359*w*w+339*w+243
    E=EllipticCurve(K,[a,b])
    P=E(7,155*w*w+306*w+310)
    Q=E(311*w*w+393*w+38,418*w*w+167*w+33)
    return k,K,w,E,P,Q


def tian_kernel():
    start=perf_counter()
    k,K,w,E,P,Q=curve_data(); r=143069629
    assert Integer(r).is_prime()
    assert E.cardinality()==r and r*P==E(0) and not P.is_zero()
    assert 1987*P==Q
    L=GF(523**6,name='z'); emb=K.embeddings(L)[0]; W=emb(w)
    theta=(-(522*W+517)).sqrt()
    curves=[EllipticCurve(L,[emb(E.a4())**(523**i),emb(E.a6())**(523**i)]) for i in range(3)]
    # Each coordinate is (constant, coefficient of W, coefficient of W^2).
    raw=[
      [([337,370,222],[400,387,122]),([156,287,312],[493,362,21]),([156,275,415],[451,251,419])],
      [([156,484,319],[108,484,298]),([337,442,245],[6,456,305]),([156,275,415],[72,272,104])],
      [([156,484,319],[415,39,225]),([156,287,312],[493,362,21]),([337,234,56],[222,109,150])],
    ]
    def val(c): return sum(L(t)*W**i for i,t in enumerate(c))
    basis=[tuple(curves[j](val(x),val(y)*theta) for j,(x,y) in enumerate(row)) for row in raw]
    zero=tuple(C(0) for C in curves)
    def key(V): return tuple(ec_key(A) for A in V)
    def add(U,V): return tuple(A+B for A,B in zip(U,V))
    points={}
    for coeff in itertools.product(range(3),repeat=3):
        V=zero
        for t,B in zip(coeff,basis):
            V=add(V,tuple(t*A for A in B))
        points[key(V)]=V
    assert len(points)==27
    assert all((3*A).is_zero() for B in basis for A in B)
    def frob(V):
        return tuple(curves[j]([V[(j-1)%3][0]**523,V[(j-1)%3][1]**523])
                     if not V[(j-1)%3].is_zero() else curves[j](0) for j in range(3))
    assert all(key(frob(V)) in points for V in points.values())
    pairing_checks=0
    for B,C in itertools.product(basis,repeat=2):
        e=L(1)
        for A,D in zip(B,C):
            e*=A.weil_pairing(D,3)
        assert e==1
        pairing_checks+=1
    assert all(sum(not A.is_zero() for A in V)!=1 for V in points.values())
    assert math.gcd(27,r)==1
    # Negative control: a product of three cyclic coordinate-axis subgroups is
    # isotropic, but fails the necessary no-coordinate-factor condition.
    bad=[tuple(basis[0][i] if i==j else curves[i](0) for i in range(3)) for j in range(3)]
    assert all(sum(not A.is_zero() for A in V)==1 for V in bad)
    baseline=bsgs_ec(P,Q,r)
    assert baseline['scalar']==1987
    return {'family':'tian_cubic_kernel','p':523,'n':3,'status':'passed',
            'field':field_data(K),'curve':str(E),'subgroup_order':r,
            'kernel_size':27,'kernel_prime':3,'kernel_rank':3,
            'frobenius_stability':True,'isotropy_checks':pairing_checks,
            'no_coordinate_factor':True,'kernel_degree_coprime_to_r':True,
            'negative_axis_kernel_rejected':True,'point':point_data(P),
            'target':point_data(Q),'source_bsgs':baseline,
            'seconds':perf_counter()-start,
            'claim':'published kernel eligibility and full source ECDLP recovery'}


def quartic_model(verbose=False,k=None):
    def log(s):
        if verbose: print('Tian: '+s,flush=True)
    log('constructing quartic')
    k=GF(523) if k is None else k
    A=ProjectiveSpace(k,2,names=('X','Y','Z')); X,Y,Z=A.gens()
    quartic=(X**4+144*X**3*Y+112*X**2*Y**2+393*X*Y**3+488*Y**4+
             397*X**3*Z+49*X**2*Y*Z+299*X*Y**2*Z+500*Y**3*Z+
             311*X**2*Z**2+185*X*Y*Z**2+288*Y**2*Z**2+
             462*X*Z**3+468*Y*Z**3+284*Z**4)
    C=Curve(quartic)
    assert not C.is_singular()
    log('quartic is smooth')
    F0=FunctionField(k,'x'); x=F0.gen()
    R=PolynomialRing(F0,'y'); y=R.gen()
    f=(x**4+144*x**3*y+112*x*x*y*y+393*x*y**3+488*y**4+
       397*x**3+49*x*x*y+299*x*y*y+500*y**3+311*x*x+
       185*x*y+288*y*y+462*x+468*y+284)
    F=certified_plane_orders(F0.extension(f.monic(),'y'))
    log('computing genus')
    assert F.genus()==3
    log('computing maximal order')
    x=F(x); y=F.gen(); O=F.maximal_order()
    infinity=next(pl for pl in F.places_infinite() if (x/y).evaluate(pl)==359)
    log('infinity place identified')
    B=infinity.divisor()
    G=F.jacobian(base_div=3*B).group()
    log('Jacobian group constructed')
    Ds=O.ideal(x**3+364*x*x+3*x+380,y**3+340*y*y+302*y+190).divisor()
    Dt=O.ideal(x**3+205*x*x+158*x+25,y**3+108*y*y+424*y+491).divisor()
    assert Ds.degree()==Dt.degree()==3
    S=G.point(Ds-3*B); T=G.point(Dt-3*B)
    log('published image pair constructed')
    return F,G,S,T


def tian_published_pair(recover_interval=True):
    start=perf_counter()
    F,G,S,T=quartic_model()
    assert Integer(143069629).is_prime()
    assert not S.is_zero()
    assert 143069629*S==0*S
    assert 1987*S==T
    interval=None
    if recover_interval:
        interval=bsgs_generic(S,T,4096)
        assert interval['scalar']==1987
    return {'family':'tian_cubic_jacobian','p':523,'n':3,'status':'passed',
            'target_genus':3,'target_hyperelliptic':False,
            'subgroup_order':143069629,'image_order_verified':143069629,
            'scalar_relation_verified':1987,'target_bounded_bsgs':interval,
            'image_generator':str(S),'image_target':str(T),
            'seconds':perf_counter()-start,
            'claim':'published image-pair replay in the actual Jacobian; interval DLP only'}


def tian_full_map():
    """Evaluate the six published functions and trace their divisor classes.

    Arithmetic is performed on C over K to represent each conjugate summand.
    Their sum is F_p-rational by construction, and is checked against the
    independently constructed published F_p image divisor.
    """
    start=perf_counter()
    k,K,w,E,P,Q=curve_data()
    assert Integer(143069629).is_prime() and E.cardinality()==143069629
    data=json.loads((Path(__file__).resolve().parent/'tian_data.json').read_text())
    R=PolynomialRing(K,'u'); u=R.gen()
    def polynomial(rows):
        return R([sum(K(a)*w**j for j,a in enumerate(row)) for row in rows])
    numerator=[(polynomial(c['A']),polynomial(c['B'])) for c in data['functions']]
    denominator=polynomial(data['denominator'])
    F,G,S_published,T_published=quartic_model(k=K)
    construction_seconds=perf_counter()-start
    x=F(F.base_field().gen()); y=F.gen(); O=F.maximal_order()
    B=next(pl for pl in F.places_infinite() if (x/y).evaluate(pl)==359).divisor()
    evaluated=0
    def transfer(Rpoint):
        nonlocal evaluated
        if Rpoint.is_zero(): return G.zero()
        X,Y=Rpoint[0],Rpoint[1]
        den=denominator(X)
        if not den:
            raise ValueError('Published rational map has an exceptional denominator')
        cs=[(a(X)+Y*b(X))/den for a,b in numerator]
        image=G.zero()
        for j in range(3):
            c1,c2,c3,c4,c5,c6=[c**(523**j) for c in cs]
            D=O.ideal(x**3-c1*x*x+c2*x-c3,
                      y**3-c4*y*y+c5*y-c6).divisor()
            assert D.degree()==3
            image+=G.point(D-3*B)
        evaluated+=1
        return image
    before=perf_counter(); S=transfer(P); T=transfer(Q)
    transfer_seconds=perf_counter()-before
    assert S==S_published and T==T_published
    assert not S.is_zero() and 143069629*S==G.zero()
    assert 1987*S==T
    assert transfer(2*P)==2*S
    assert transfer(3*P)==3*S
    assert transfer(5*P)==transfer(2*P)+transfer(3*P)
    return {'family':'tian_cubic_rational_map','p':523,'n':3,'status':'passed',
            'subgroup_order':143069629,'target_genus':3,'target_hyperelliptic':False,
            'published_six_functions_evaluated':True,'map_evaluations':evaluated,
            'published_image_pair_matched':True,'image_order_verified':143069629,
            'homomorphism_checks':3,'scalar_relation_verified':1987,
            'mathematical_target_field_order':523,
            'arithmetic_constant_field_order':int(K.cardinality()),
            'construction_seconds':construction_seconds,
            'two_point_transfer_seconds':transfer_seconds,'seconds':perf_counter()-start,
            'claim':'full evaluation of one published isogeny; no new-isogeny constructor or index-calculus benchmark'}
