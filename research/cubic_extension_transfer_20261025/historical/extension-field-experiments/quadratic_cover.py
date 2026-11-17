"""A constructed genus-two transfer from a genuine quadratic-extension curve.

For c nonsquare in k, s^2=c in K, and
 f(X)=X^6+aX^5+bX^4+hX^3+cbX^2+c^2aX+c^3,
the involution (X,Y)->(c/X,s^3Y/X^3) is defined over K.
Its elliptic quotient is computed explicitly. Pull back a point divisor, then
trace the divisor class to k. A rational branch point X=1 is imposed so that
ordinary odd-degree Mumford arithmetic suffices.
"""
import random
from time import perf_counter
from sage.all import GF, PolynomialRing, EllipticCurve, HyperellipticCurve, factor
from common import field_data, point_data, prime_generator, bsgs_ec, bsgs_generic, project_prime_order


def make_cover(p,m=1,attempts=60):
    k=GF(p**m,name='v') if m>1 else GF(p)
    K=GF(p**(2*m),name='z')
    emb=k.embeddings(K)[0]
    project=emb.section()
    c=next(v for v in k if v and not v.is_square())
    s=emb(c).sqrt()
    R=PolynomialRing(k,'t'); t=R.gen()
    RK=PolynomialRing(K,'t'); T=RK.gen()
    for _ in range(attempts):
        a=k.random_element(); b=k.random_element()
        h=-(1+a+b+c*b+c*c*a+c**3)
        f=t**6+a*t**5+b*t**4+h*t**3+c*b*t**2+c*c*a*t+c**3
        if f.gcd(f.derivative()).degree():
            continue
        # Z=1/(X-1), W=Y*Z^3. Infinity is the branch point X=1.
        fodd=sum(f[i]*(t+1)**i*t**(6-i) for i in range(7))
        if fodd.degree()!=5:
            continue
        aa,bb,cc,hh=map(emb,(a,b,c,h))
        L=(-2*s)**3+aa*(-2*s)**2+(bb-3*cc)*(-2*s)+(hh-2*aa*cc)
        if not L:
            continue
        a2=bb+9*cc-4*aa*s
        a4=(aa-6*s)*L
        E=EllipticCurve(K,[0,a2,0,a4,L*L])
        # Exclude curves whose j-invariant is already in the smaller field.
        if E.j_invariant()**(p**m)==E.j_invariant():
            continue
        foddK=RK([emb(v) for v in fodd.list()])
        H=HyperellipticCurve(fodd)
        Jk=H.jacobian()(k)
        # Explicit coefficient embedding is essential for non-prime k: Sage's
        # scheme Hom-set constructor does not infer this embedding from names.
        JK=HyperellipticCurve(foddK).jacobian()(K)

        def pullback(P):
            if P.is_zero():
                return JK(0)
            X,Y=P[0],P[1]
            u=X*((1+s)*T+1)**2-L*(T*T+T)
            if u.degree()!=2:
                raise ValueError('Exceptional quotient fiber; resample the point')
            u=u.monic()
            v=(Y/L*((1+s)*T+1)**3)%u
            assert (v*v-foddK)%u==0
            return JK([u,v])

        def transfer(P):
            D=pullback(P)
            if D.is_zero():
                return Jk(0)
            u,v=D[0],D[1]
            uq=RK([c**(p**m) for c in u.list()])
            vq=RK([c**(p**m) for c in v.list()])
            norm=D+JK([uq,vq])
            if norm.is_zero():
                return Jk(0)
            un,vn=norm[0],norm[1]
            assert all(v**(p**m)==v for v in un.list()+vn.list())
            return Jk([R([project(v) for v in un.list()]),
                       R([project(v) for v in vn.list()])])

        return {'k':k,'K':K,'E':E,'H':H,'Jk':Jk,'transfer':transfer,
                'sextic':f,'odd_model':fodd,'c':c,'a':a,'b':b,'h':h,
                's':s,'L':L,'embedding_generator_image':emb(k.gen())}
    raise ValueError('No nonsingular genuine quadratic quotient found')


def cover_experiment(p,m=1):
    start=perf_counter()
    data=make_cover(p,m)
    construction_seconds=perf_counter()-start
    E=data['E']; phi=data['transfer']
    before=perf_counter()
    N=int(E.cardinality())
    order_count_seconds=perf_counter()-before
    eligible=[int(r) for r,e in factor(N) if r>5 and r!=p]
    if not eligible:
        raise ValueError('No suitable subgroup on this constructed quotient')
    r=max(eligible)
    transfer_seconds=0
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
        raise ValueError('Transfer killed every sampled generator or fibers were exceptional')
    assert r*S==0*S  # r prime and S nonzero proves preservation of the whole subgroup.
    hom_checks=0
    for a,b in [(2,3),(3,5),(5,7)]:
        try:
            assert phi((a+b)*P)==phi(a*P)+phi(b*P)
            assert phi(a*P)==a*S
            hom_checks+=1
        except ValueError:
            continue
    assert hom_checks>=1
    for _ in range(30):
        secret=random.randrange(1,r)
        Q=secret*P
        try:
            before=perf_counter(); T=phi(Q); transfer_seconds+=perf_counter()-before
        except ValueError:
            continue
        break
    assert T==secret*S
    base=bsgs_ec(P,Q,r)
    target=bsgs_generic(S,T,r)
    assert target['scalar']==base['scalar']==secret
    return {'family':'quadratic_genus_two_transfer','p':p,'n':2*m,'d':m,'status':'passed',
            'field':field_data(data['K']),'target_field':field_data(data['k']),
            'curve':str(E),'curve_order':N,'subgroup_order':r,'target_genus':2,
            'j_invariant_in_target_field':False,'cover_sextic':str(data['sextic']),
            'cover_odd_model':str(data['odd_model']),'c':str(data['c']),
            'target_field_generator_embedding':str(data['embedding_generator_image']),
            'a':str(data['a']),'b':str(data['b']),'h':str(data['h']),
            's':str(data['s']),'L':str(data['L']),
            'point':point_data(P),'target':point_data(Q),'secret':secret,
            'image_generator':str(S),'image_target':str(T),
            'image_order_verified':r,'homomorphism_checks':hom_checks,
            'construction_seconds':construction_seconds,'order_count_seconds':order_count_seconds,
            'transfer_seconds_two_points_plus_search':transfer_seconds,
            'source_bsgs':base,'target_bsgs':target,
            'measured_speedup_including_transfer':base['seconds']/(target['seconds']+transfer_seconds),
            'seconds':perf_counter()-start,
            'claim':'explicit genus-two pullback-and-trace transfer; BSGS on both sides, no index-calculus speed claim'}
