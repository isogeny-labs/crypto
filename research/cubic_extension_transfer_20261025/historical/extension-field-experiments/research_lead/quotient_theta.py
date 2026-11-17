"""Construct level-two quotient theta data from cubic elliptic gluing.

Implements Tian arXiv:2012.07173 Sections 2 and 3.1 with tangent-line
sections for ell=3. All computations are exact finite-field arithmetic.
This first stage does not identify a base-field Jacobian or evaluate its map.
"""
import itertools
import json
from pathlib import Path
import sys
from time import perf_counter
from sage.all import GF, EllipticCurve, PolynomialRing, matrix, vector, set_random_seed, Curve, FunctionField, factor, lcm
from math import isqrt

ROOT=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT.parent))


def dot(a,b): return sum(x*y for x,y in zip(a,b))


def normal_three(E):
    """Normal Weil sections, divisors 3[-P]-3[O], on one elliptic curve."""
    L=E.base_field()
    roots=E.division_polynomial(3).roots(multiplicities=False)
    assert len(roots)==4
    points=[E(x,(x**3+E.a4()*x+E.a6()).sqrt()) for x in roots]
    B1=points[0]
    B2=next(P for P in points if P.weil_pairing(B1,3)!=1)
    coords=list(itertools.product(range(3),repeat=2)); zero=(0,0)
    pts={c:c[0]*B1+c[1]*B2 for c in coords}
    sec={zero:vector(L,[1,0,0])}
    for c,P in pts.items():
        if c==zero: continue
        slope=(3*P[0]**2+E.a4())/(2*P[1])
        sec[c]=vector(L,[P[1]-slope*P[0],slope,1])
    def ev(c,R):
        if c==zero: return L.one()
        return dot(sec[c],[1,R[0],R[1]])
    def add(c,d): return tuple((x+y)%3 for x,y in zip(c,d))
    while True:
        R=E.random_point()
        if R.is_zero(): continue
        if all(not (R+P).is_zero() and all(ev(c,R+P) for c in coords)
               for P in pts.values()): break
    def coc(c,d): return ev(c,R)*ev(d,R+pts[c])/ev(add(c,d),R)
    pairing=coc((1,0),(0,1))/coc((0,1),(1,0))
    assert pairing!=1 and pairing**3==1
    def d(c,e): return pairing**(2*(c[0]*e[1]-c[1]*e[0]))
    alpha={zero:L.one()}
    for c in [(1,0),(0,1)]:
        c2=add(c,c)
        u=1/(coc(c,c)*coc(c,c2))
        v=ev(c2,R)/(ev(c,R)*ev(c2,R+pts[c])*ev(c,-R))
        alpha[c]=u/v
        assert alpha[c]**3==u and alpha[c]**2==v
        alpha[c2]=alpha[c]**2*coc(c,c)
    for i,j in itertools.product([1,2],repeat=2):
        c=(i,0); e=(0,j)
        alpha[(i,j)]=alpha[c]*alpha[e]*coc(e,c)/d(e,c)
    normalized={c:alpha[c]*sec[c] for c in coords}
    for c,e in itertools.product(coords,repeat=2):
        assert alpha[c]*alpha[e]*coc(c,e)==d(c,e)*alpha[add(c,e)]
    # Independent evaluation certifies the rational-function cocycle constants.
    R2=E.random_point()
    for c,e in itertools.product(coords,repeat=2):
        assert dot(normalized[c],[1,R2[0],R2[1]])*dot(normalized[e],
                    [1,(R2+pts[c])[0],(R2+pts[c])[1]]) == \
               d(c,e)*dot(normalized[add(c,e)],[1,R2[0],R2[1]])
    return (B1,B2),normalized,pairing


def kernels():
    data=json.loads((ROOT/'results/kernel_classification.json').read_text())
    return next(r['transverse_kernel_bases'] for r in data['models'] if
                r['q_mod_3']==1 and r['trace_mod_3']==1 and r['source_frobenius_type']=='scalar')


def theta_tensor(sections,rows):
    L=next(iter(sections[0].values())).base_ring()
    T={i:L.zero() for i in itertools.product(range(3),repeat=3)}
    for cs in itertools.product(range(3),repeat=3):
        v=[sum(cs[i]*rows[i][j] for i in range(3))%3 for j in range(6)]
        ss=[sections[j][tuple(v[2*j:2*j+2])] for j in range(3)]
        for i in T: T[i]+=ss[0][i[0]]*ss[1][i[1]]*ss[2][i[2]]
    return T


def action_matrices(E,basis,sections):
    """Heisenberg action on H^0(E,3O), certified at extra points."""
    L=E.base_field(); B1,B2=basis
    pts={c:c[0]*B1+c[1]*B2 for c in sections}
    def safe():
        while True:
            R=E.random_point()
            if not R.is_zero() and all(not (R+P).is_zero() for P in pts.values()): return R
    while True:
        samples=[safe() for _ in range(3)]
        V=matrix(L,[[1,R[0],R[1]] for R in samples])
        if V.is_invertible(): break
    inverse=V.inverse(); result={}
    for c,P in pts.items():
        values=matrix(L,[list(dot(sections[c],[1,R[0],R[1]])*vector(L,[1,(R+P)[0],(R+P)[1]])) for R in samples])
        result[c]=inverse*values
    for _ in range(2):
        R=safe()
        for c,P in pts.items():
            assert vector(L,[1,R[0],R[1]])*result[c] == \
                dot(sections[c],[1,R[0],R[1]])*vector(L,[1,(R+P)[0],(R+P)[1]])
    return result


def invariant_tensor(actions,rows):
    """Find a nonzero column of the rank-one group trace projector.

    Averaging the constant section can be zero; that is not a bad quotient.
    The 27-dimensional representation has exactly one invariant dimension.
    """
    for h in itertools.product(range(3),repeat=3):
        ss=[{c:M.column(h[j]) for c,M in actions[j].items()} for j in range(3)]
        T=theta_tensor(ss,rows)
        if any(T.values()): return T,h
    raise AssertionError('Group trace projector is zero')


def contract(T,vectors):
    return sum(c*vectors[0][i[0]]*vectors[1][i[1]]*vectors[2][i[2]] for i,c in T.items())


def theta_null(T,roots,p):
    """Evaluate h_P(0), including exact pole/zero cancellation at Q3.

    Normal level-two g implies g_P^3*(theta o T_P/theta)^2
    is already normal: its cocycle is d_2^3=d_2. No square-root
    normalization on the 64 quotient functions is needed.
    """
    L=next(iter(T.values())).parent()
    e1,e2,e3=roots
    a=((e1-e3)/(e1-e2)).sqrt()
    b=((e2-e3)/(e2-e1)).sqrt()
    # The sign of c is fixed by normality, rather than independently chosen.
    # f10 * T_Q1(f01) = -f11 for Tian's d_2 convention.
    E=EllipticCurve(L,[sum([e1*e2,e1*e3,e2*e3]),-e1*e2*e3])
    Q1=E(e1,0)
    R=E.random_point()
    f10=lambda P:a*(P[0]-e2)/(P[0]-e3)
    f01=lambda P:b*(P[0]-e1)/(P[0]-e3)
    c=-f10(R)*f01(R+Q1)*(R[0]-e3)
    assert c*c==(e3-e1)*(e2-e3)
    gs=[L.one(),a,b,c]
    # Tensor vectors for R=0,Q1,Q2,Q3 in U=R-Q3 coordinates.
    vs=[[L.one(),e3,L.zero()],[L.one(),e2,L.zero()],
        [L.one(),e1,L.zero()],[L.zero(),L.zero(),L.one()]]
    denominator=contract(T,[vs[0],[v**p for v in vs[0]],[v**(p*p) for v in vs[0]]])
    vals=[]
    for idx in range(64):
        bits=[(idx>>(j+3))&1 for j in range(3)]+[(idx>>j)&1 for j in range(3)]
        codes=[bits[j]+2*bits[j+3] for j in range(3)]
        vv=[[v**(p**j) for v in vs[codes[j]]] for j in range(3)]
        gg=gs[codes[0]]*gs[codes[1]]**p*gs[codes[2]]**(p*p)
        vals.append(gg**3*contract(T,vv)**2)
    if denominator:
        vals=[v/denominator**2 for v in vals]
    return vals


def bits_code(idx):
    return [((idx>>(j+3))&1)+2*((idx>>j)&1) for j in range(3)]


def gradients(T,roots,a4,p):
    """Odd-theta gradients in the cotangent basis of the elliptic product."""
    L=next(iter(T.values())).parent(); e1,e2,e3=roots
    at=[e3,e2,e1]
    vs=[[L.one(),e,L.zero()] for e in at]+[[L.zero(),L.zero(),-L.one()]]
    ds=[[L.zero(),L.zero(),3*e*e+a4] for e in at]+[[L.zero(),L.one(),L.zero()]]
    out={}
    for idx in range(64):
        if not sum(((idx>>j)&1)*((idx>>(j+3))&1) for j in range(3))%2: continue
        codes=bits_code(idx)
        vv=[[v**(p**j) for v in vs[codes[j]]] for j in range(3)]
        assert not contract(T,vv)
        g=[]
        for j in range(3):
            derivative=vv[:]; derivative[j]=[v**(p**j) for v in ds[codes[j]]]
            g.append(contract(T,derivative))
        assert any(g)
        out[idx]=vector(L,g)
    return out


def riemann_quartic(lines):
    """Recover the canonical quartic from a labelled Aronhold system."""
    indices=[63,38,52,41,13,26,19]
    L=lines[63].base_ring()
    first=matrix(L,[lines[i] for i in indices[:3]])
    co=lines[indices[3]]*first.inverse()
    H=matrix.diagonal(co)*first
    aa=[]
    for i in indices[4:]:
        a=lines[i]*H.inverse(); aa.append(a/a[0])
    A=matrix(L,aa)
    U=matrix(L,3,3,lambda i,j:1/A[j,i]).solve_right(vector(L,[-1]*3))
    kval=(A.transpose()*matrix.diagonal(U)).solve_right(vector(L,[-1]*3))
    Left=matrix(L,[[1]*3]+[[1/v for v in a] for a in aa])
    Right=-matrix(L,[[1]*3]+[[kval[j]*v for v in aa[j]] for j in range(3)])
    Xi=Left.matrix_from_rows([0,1,2]).inverse()*Right.matrix_from_rows([0,1,2])
    assert Left*Xi==Right
    R=PolynomialRing(L,['X','Y','Z']); xyz=vector(R,R.gens())
    new=H*xyz; xi=Xi*new; x,y,z=new
    f=(x*xi[0]+y*xi[1]-z*xi[2])**2-4*x*y*xi[0]*xi[1]
    return f,H,A


def rational_quartic(f,emb,p):
    """Descend via the explicit cotangent basis of the Weil restriction."""
    R=f.parent(); xyz=vector(R,R.gens()); z=emb.domain().gen()
    B=matrix(R,3,3,lambda i,j:emb(z**j)**(p**i))
    out=f(*list(B*xyz)); out=out/out.coefficients()[0]
    assert all(c**p==c for c in out.coefficients()), 'canonical model failed base-field descent'
    k=GF(p); S=PolynomialRing(k,['X','Y','Z'])
    g=S({e:k(c) for e,c in out.dict().items()})
    return g,B


def bitangent_check(f,line):
    """Every supplied odd gradient must cut an even divisor on the quartic."""
    L=f.base_ring(); R=PolynomialRing(L,'t'); t=R.gen()
    pivot=next(i for i,x in enumerate(line) if x)
    free=[i for i in range(3) if i!=pivot]
    v=[L.zero()]*3; v[free[0]]=t; v[free[1]]=R.one()
    v[pivot]=-(line[free[0]]*t+line[free[1]])/line[pivot]
    ff=R(f(*v))
    assert ff and all(e%2==0 for _,e in ff.factor()) and (4-ff.degree())%2==0


def curve_points(f,k):
    R=PolynomialRing(k,'t'); t=R.gen()
    # Projective points: [x:y:1], then [1:y:0], then [0:1:0].
    affine=[]
    for x in k:
        ff=R(f(x,t,1))
        affine.extend((x,y,k.one()) for y in ff.roots(multiplicities=False))
    infinite=[(k.one(),y,k.zero()) for y in R(f(1,t,0)).roots(multiplicities=False)]
    if not f(0,1,0): infinite.append((k.zero(),k.one(),k.zero()))
    return affine,infinite


def jacobian_order_certificate(f,p,r):
    """Certify the rational Jacobian order without enumerating F_(p^3).

    N1=p+1 and N2=p^2+1 imply L(T)=1+a3*T^3+p^3*T^6.
    Weil bounds put #J in p^3+1 +/- floor(2 sqrt(p^3)). Exact
    divisor orders determine a unique multiple inside this interval.
    """
    k=GF(p); aff,inf=curve_points(f,k)
    K2=GF(p*p,name='b'); aff2,inf2=curve_points(f,K2)
    n1=len(aff)+len(inf); n2=len(aff2)+len(inf2)
    assert n1==p+1 and n2==p*p+1
    # Pick a coordinate direction outside C so the affine equation is monic y.
    if not f(0,1,0):
        R=f.parent(); X,Y,Z=R.gens()
        direction=next(a for a in k if f(a,1,0))
        f=f(X+direction*Y,Y,Z)
        aff,inf=curve_points(f,k)
    F0=FunctionField(k,'x'); x=F0.gen(); Ry=PolynomialRing(F0,'y'); y=Ry.gen()
    equation=Ry(f(x,y,1)).monic(); F=F0.extension(equation,'y')
    from tian import certified_plane_orders
    F=certified_plane_orders(F); assert F.genus()==3
    x=F(x); y=F.gen(); O=F.maximal_order()
    divisors=[O.ideal(x-a,y-b).divisor() for a,b,_ in aff]
    assert all(D.degree()==1 for D in divisors)
    B=divisors[0]; G=F.jacobian(base_div=3*B).group()
    zero=G.zero()
    # Sage 10.6 Hess points can represent the same class by different ideals.
    # is_zero()/!= use representation inequality in some cases. Class equality
    # invokes the required principal-divisor test. See zero_test_regression.py.
    lo=p**3+1-isqrt(4*p**3); hi=p**3+1+isqrt(4*p**3)
    twist=2*(p**3+1)-r; exponent=1; orders=[]
    for D in divisors[1:]:
        V=G.point(D-B)
        if V==zero: continue
        if r*V==zero: N=r
        else:
            assert twist*V==zero, 'Neither expected isogeny class annihilates a divisor'
            N=twist
        order=N
        for ell,_ in factor(N):
            while order%ell==0 and (order//ell)*V==zero: order//=int(ell)
        exponent=int(lcm(exponent,order)); orders.append(order)
        multiples=[j*exponent for j in range((lo+exponent-1)//exponent,hi//exponent+1)]
        if len(multiples)==1:
            assert multiples[0]==N
            return {'N1':n1,'N2':n2,'jacobian_order':N,'source_order':r,
                    'quotient_is_base_field_jacobian':N==r,
                    'serre_twist_obstruction':N!=r,'witness_divisor_orders':orders,
                    'order_divisibility_certificate':exponent,'weil_interval':[lo,hi]}
    raise AssertionError('Rational-point divisors did not certify a unique Jacobian order')


def construct(E,p,verbose=False,certify=True):
    started=perf_counter()
    L=GF(p**18,name='v'); emb=E.base_field().embeddings(L)[0]
    EL=EllipticCurve(L,[emb(E.a4()),emb(E.a6())])
    basis,sec,pairing=normal_three(EL)
    if verbose: print('normal_three complete',flush=True)
    assert all(EL(P[0]**(p**3),P[1]**(p**3))==-P for P in basis)
    sections=[{c:vector(L,[x**(p**j) for x in v]) for c,v in sec.items()} for j in range(3)]
    action=action_matrices(EL,basis,sec)
    if verbose: print('action matrices complete',flush=True)
    actions=[{c:M.apply_map(lambda x:x**(p**j)) for c,M in action.items()} for j in range(3)]
    Rx=PolynomialRing(L,'x'); x=Rx.gen()
    roots=(x**3+EL.a4()*x+EL.a6()).roots(multiplicities=False)
    assert len(roots)==3
    rows=[]
    for i,kern in enumerate(kernels()):
        before=perf_counter(); T,h=invariant_tensor(actions,kern)
        vals=theta_null(T,roots,p)
        count=sum(bool(v) for v in vals)
        parity=[sum(((i>>j)&1)*((i>>(j+3))&1) for j in range(3))%2 for i in range(64)]
        assert all(not vals[j] for j in range(64) if parity[j])
        row={'kernel_index':i,'nonzero_theta_constants':count,
             'tensor_nonzero':any(T.values()),'origin_nonzero':bool(vals[0]),
             'averaged_section_basis':list(h),
             'seconds':perf_counter()-before}
        if count==36:
            if verbose: print('reconstructing kernel',i,flush=True)
            lines=gradients(T,roots,EL.a4(),p)
            f,H,A=riemann_quartic(lines)
            for line in lines.values(): bitangent_check(f,line)
            g,B=rational_quartic(f,emb,p)
            smooth=not Curve(g).is_singular(); assert smooth
            row.update(quartic=str(g),quartic_coefficients={str(tuple(e)):int(c) for e,c in g.dict().items()},
                       base_field_descent=True,smooth=smooth,bitangents_verified=28,
                       seconds=perf_counter()-before)
            if certify:
                row.update(jacobian_order_certificate(g,p,int(E.cardinality())))
                row['seconds']=perf_counter()-before
        rows.append(row)
        if verbose: print(json.dumps(row),flush=True)
    return {'p':p,'source_curve':str(E),'kernels':rows,'seconds':perf_counter()-started}


if __name__=='__main__':
    set_random_seed(20261026)
    p=7; K=GF(p**3,name='z'); z=K.gen()
    E=EllipticCurve(K,[3*z*z+6*z+4,4*z*z+5*z+2])
    assert E.cardinality()==373
    result=construct(E,p,verbose=True)
    (ROOT/'results/quotient_theta.json').write_text(json.dumps(result,indent=2)+'\n')
