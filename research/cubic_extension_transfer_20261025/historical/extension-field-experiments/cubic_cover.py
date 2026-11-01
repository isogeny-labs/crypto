"""Explicit genus-three GHS-style covers in odd characteristic.

For K/k cubic, a in K, E: Y^2=X(X-a)(X-a^q), construct a hyperelliptic
genus-three curve over k and a degree-four map to E. A conic parametrization
gives A(t) in K[t] and u,v in k[t] with A(t)^2=u(t)+v(t)*a.
Then C: w^2=-u*v and X=-u/v, Y=-w*A*A^q/v^2.

This constructed special family has full rational 2-torsion, unlike Tian's
prime-order example. Pullback and trace use odd-degree Mumford arithmetic.
"""
import random
from time import perf_counter
from sage.all import GF, PolynomialRing, EllipticCurve, HyperellipticCurve, factor
from common import field_data, point_data, bsgs_ec, bsgs_generic, project_prime_order


def make_cubic_cover(p,m=1):
    k=GF(p**m,name='v') if m>1 else GF(p)
    K=GF(p**(3*m),name='z'); q=p**m
    emb=k.embeddings(K)[0]; project=emb.section()
    alpha=K.gen(); conjugates=[alpha**(q**i) for i in range(3)]
    assert len(set(conjugates))==3
    R=PolynomialRing(k,'t'); t=R.gen()
    RK=PolynomialRing(K,'t'); T=RK.gen()
    RX=PolynomialRing(K,'a'); X=RX.gen()

    def relative_coefficients(z):
        f=RX.lagrange_polynomial([(conjugates[i],z**(q**i)) for i in range(3)])
        assert all(c**q==c for c in f.list())
        return [project(f[i]) for i in range(3)]

    def relative_polynomials(poly):
        cols=[relative_coefficients(c) for c in poly.list()]
        return [R([row[j] for row in cols]) for j in range(3)]

    delta=relative_polynomials((T*alpha+alpha**2)**2)[2]
    A=RK([emb(c) for c in (-delta/2).list()])+T*alpha+alpha**2
    u,v,z=relative_polynomials(A*A)
    assert z==0 and u.degree()==4 and v.degree()==3
    f=-u*v
    assert f.degree()==7 and f.gcd(f.derivative()).degree()==0
    uK=RK([emb(c) for c in u.list()]); vK=RK([emb(c) for c in v.list()])
    fK=RK([emb(c) for c in f.list()])
    Aq=RK([c**q for c in A.list()])
    a,b=conjugates[:2]
    E=EllipticCurve(K,[0,-(a+b),0,a*b,0])
    H=HyperellipticCurve(f); Jk=H.jacobian()(k)
    JK=HyperellipticCurve(fK).jacobian()(K)
    origin_fiber=JK([vK.monic(),RK(0)])
    # Certify the covering map as a rational-function identity.
    frac=RK.fraction_field(); XX=-frac(uK)/vK
    YY_squared=frac(fK*A*A*Aq*Aq)/vK**4
    assert YY_squared==XX*(XX-a)*(XX-b)

    def pullback(P):
        if P.is_zero():
            return JK(0)
        X0,Y0=P[0],P[1]
        U=uK+X0*vK
        if U.degree()!=4:
            raise ValueError('Exceptional quotient fiber')
        U=U.monic()
        try:
            V=(-Y0*vK*vK*(A*Aq).inverse_mod(U))%U
        except (ZeroDivisionError,ValueError):
            raise ValueError('Ramified quotient fiber')
        assert (V*V-fK)%U==0
        while U.degree()>3:
            Unext=((fK-V*V)//U).monic()
            V=(-V)%Unext; U=Unext
        return JK([U,V])-origin_fiber

    def transfer(P):
        D=pullback(P)
        if D.is_zero():
            return Jk(0)
        U,V=D[0],D[1]
        Dq=JK([RK([c**q for c in U.list()]),RK([c**q for c in V.list()])])
        Dqq=JK([RK([c**(q*q) for c in U.list()]),RK([c**(q*q) for c in V.list()])])
        S=D+Dq+Dqq
        if S.is_zero():
            return Jk(0)
        U,V=S[0],S[1]
        assert all(c**q==c for c in U.list()+V.list())
        return Jk([R([project(c) for c in U.list()]),R([project(c) for c in V.list()])])

    return {'k':k,'K':K,'E':E,'H':H,'transfer':transfer,
            'alpha':alpha,'u':u,'v':v,'A':A,'f':f,
            'embedding_generator_image':emb(k.gen())}


def cubic_experiment(p,m=1):
    start=perf_counter(); data=make_cubic_cover(p,m)
    construction_seconds=perf_counter()-start
    before=perf_counter()
    E=data['E']; phi=data['transfer']; N=int(E.cardinality())
    order_count_seconds=perf_counter()-before
    rs=[int(r) for r,e in factor(N) if r>5 and r!=p]
    if not rs:
        raise ValueError('No suitable odd subgroup for this cubic cover')
    r=max(rs); transfer_seconds=0
    for _ in range(100):
        P=project_prime_order(E.random_point(),N,r)
        if P.is_zero():
            continue
        try:
            before=perf_counter(); S=phi(P); transfer_seconds+=perf_counter()-before
        except ValueError:
            continue
        if not S.is_zero():
            break
    else:
        raise ValueError('No preserved subgroup found')
    assert r*S==0*S
    checks=0
    for a,b in [(2,3),(3,5)]:
        try:
            assert phi((a+b)*P)==phi(a*P)+phi(b*P)
            checks+=1
        except ValueError:
            continue
    assert checks
    for _ in range(50):
        secret=random.randrange(1,r); Q=secret*P
        try:
            before=perf_counter(); T=phi(Q); transfer_seconds+=perf_counter()-before
        except ValueError:
            continue
        break
    assert T==secret*S
    src=bsgs_ec(P,Q,r); dst=bsgs_generic(S,T,r)
    assert src['scalar']==dst['scalar']==secret
    return {'family':'cubic_genus_three_transfer','p':p,'n':3*m,'d':m,'status':'passed',
            'field':field_data(data['K']),'target_field':field_data(data['k']),
            'curve':str(E),'curve_order':N,'subgroup_order':r,'target_genus':3,
            'target_hyperelliptic':True,'cover_polynomial':str(data['f']),
            'target_field_generator_embedding':str(data['embedding_generator_image']),
            'cover_u':str(data['u']),'cover_v':str(data['v']),'cover_A':str(data['A']),
            'cover_identity_verified':True,
            'point':point_data(P),'target':point_data(Q),'secret':secret,
            'image_generator':str(S),'image_target':str(T),
            'homomorphism_checks':checks,'image_order_verified':r,
            'construction_seconds':construction_seconds,'order_count_seconds':order_count_seconds,
            'transfer_seconds_two_points_plus_search':transfer_seconds,
            'source_bsgs':src,'target_bsgs':dst,
            'measured_speedup_including_transfer':src['seconds']/(dst['seconds']+transfer_seconds),
            'seconds':perf_counter()-start,
            'claim':'explicit special-family genus-three cover; not a full Joux--Vitse reproduction'}
