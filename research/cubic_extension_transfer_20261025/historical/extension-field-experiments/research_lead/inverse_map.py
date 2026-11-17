"""Riemann-Roch Weil functions for inversion of a newly constructed quotient.

Uses direct ratios f_D(R)/f_D(0), which cancel normal-Weil-set scalars.
This development stage constructs/calibrates the divisor-side functions; no
published numerical isogeny functions or known DLP scalar are used.
"""
import itertools,json,sys
from collections import deque
from pathlib import Path
from time import perf_counter
from sage.all import (GF,EllipticCurve,PolynomialRing,FunctionField,matrix,vector,
                       set_random_seed)
from quotient_theta import (normal_three,action_matrices,invariant_tensor,kernels,
                            gradients,riemann_quartic,rational_quartic,theta_null,
                            contract,bits_code,curve_points)
from obstruction_probe import chi_square_class
ROOT=Path(__file__).resolve().parent;sys.path.insert(0,str(ROOT.parent))
from tian import certified_plane_orders


def half_divisor(D):
    assert all(e%2==0 for pl,e in D.list())
    return sum(((e//2)*pl.divisor() for pl,e in D.list()),D.parent().zero())


class ObstructionSearchExhausted(ValueError):
    """No usable prepared model was found within the stated walk budget."""


def prepared_models(source, max_steps):
    queue=deque([(source,[])]);seen={source.j_invariant()}
    while queue:
        E,chain=queue.popleft()
        if len(E.division_polynomial(3).roots(multiplicities=False))==4:
            yield E,chain
        if len(chain)==max_steps:continue
        for iso in E.isogenies_prime_degree(3):
            E1=iso.codomain();j=E1.j_invariant()
            if j in seen:continue
            seen.add(j);queue.append((E1,chain+[iso]))


def setup(kernel_index=0,ambient_degree=18,tangent_coordinates=False,indices=None,source_curve=None,
          max_isogeny_steps=2):
    start=perf_counter()
    if source_curve is None:
        p=7;K=GF(p**3,name='z');z=K.gen()
        source=EllipticCurve(K,[4*z*z+3*z+3,4*z*z+5*z+2])
    else:source=source_curve;K=source.base_field();p=int(K.characteristic())
    k=GF(p)
    L=GF(p**ambient_degree,name='v');emb=K.embeddings(L)[0]
    Rx=PolynomialRing(L,'t');t=Rx.gen()
    obstruction_walk=[]
    for E,isogeny_chain in prepared_models(source,max_isogeny_steps):
        EL=EllipticCurve(L,[emb(E.a4()),emb(E.a6())]);basis,sec,_=normal_three(EL)
        acts=action_matrices(EL,basis,sec)
        actions=[{c:M.apply_map(lambda a:a**(p**j)) for c,M in acts.items()} for j in range(3)]
        roots=(t**3+EL.a4()*t+EL.a6()).roots(multiplicities=False)
        good=False;tested=0
        for selected_kernel in list(range(kernel_index,12))+list(range(kernel_index)):
            T,h=invariant_tensor(actions,kernels()[selected_kernel]);tested+=1
            chi,good=chi_square_class(T,roots,EL,p,emb)
            if good:break
        obstruction_walk.append({'curve':str(E),'steps':len(isogeny_chain),
                                 'kernels_tested':tested,'usable':good})
        if good:break
    else:raise ObstructionSearchExhausted('No usable quotient within the bounded isogeny walk')
    def phi(P):
        for iso in isogeny_chain:P=iso(P)
        return P
    lines=gradients(T,roots,EL.a4(),p);f,H,A=riemann_quartic(lines);g,B=rational_quartic(f,emb,p)
    if tangent_coordinates:
        from sage.all import Curve
        assert not Curve(g).is_singular()
        original_lines={i:vector(L,[L(c) for c in line*B]) for i,line in lines.items()}
        rational=next(ell/next(c for c in ell if c) for ell in original_lines.values()
                      if all((c/next(a for a in ell if a))**p==c/next(a for a in ell if a) for c in ell))
        Rp=PolynomialRing(k,'u');u=Rp.gen();affine=[]
        for _ in range(100):
            a=k.random_element();ys=Rp(g(a,u,1)).roots(multiplicities=False)
            if ys:affine.append((a,ys[0],k.one()))
        R=g.parent();XYZ=vector(R,R.gens());change=None
        for point in affine:
            if not sum(L(c)*L(v) for c,v in zip(rational,point)):continue
            grad=vector(k,[g.derivative(v)(*point) for v in R.gens()])
            Xrow=vector(k,[0,0,1])
            Yrow=vector(k,[1,0,-point[0]])
            if not matrix(k,[Xrow,Yrow,grad]).is_invertible():Yrow=vector(k,[0,1,-point[1]])
            for s in map(k,range(5)):
                change=matrix(k,[Xrow+s*Yrow,Yrow,grad])
                if not change.is_invertible():continue
                candidate=g(*list(change.inverse()*XYZ))
                if not candidate(0,1,0):continue
                if not candidate.monomial_coefficient(R.gen(0)**2*R.gen(1)**2):continue
                break
            else:continue
            g=candidate/candidate.coefficients()[0];B=B*change.inverse();break
        else:raise ValueError('No ordinary rational tangent chart found')
    assert g(0,1,0)
    C9=GF(p**9,name='c');inc9=C9.embeddings(L)[0];sec9=inc9.section()
    F0=FunctionField(C9,'x');x=F0.gen();Ry=PolynomialRing(F0,'y');y=Ry.gen()
    F=certified_plane_orders(F0.extension(Ry(g(x,y,1)).monic(),'y'));x=F(x);y=F.gen();O=F.maximal_order()
    print('curve and function field ready',perf_counter()-start,flush=True)
    KC=x.divisor().denominator();assert KC.degree()==4
    contact={};linefunc={};linecoeff={}
    for idx,line in lines.items():
        ell=[L(a) for a in line*B];pivot=next(a for a in ell if a)
        ell=[a/pivot for a in ell];assert all(a**(p**9)==a for a in ell)
        ell=[sec9(a) for a in ell];func=ell[0]*x+ell[1]*y+ell[2]
        linecoeff[idx]=ell
        linefunc[idx]=func
        contact[idx]=half_divisor(func.divisor()+KC)
        assert contact[idx].degree()==2
    print('28 contact divisors ready',perf_counter()-start,flush=True)
    triple=[9,19,26];assert triple[0]^triple[1]^triple[2]==0
    delta=sum((contact[i] for i in triple),KC.parent().zero())-KC
    assert delta.degree()==2 and not delta.basis_function_space()
    assert (linefunc[9]*linefunc[19]*linefunc[26]).divisor()==2*delta-KC
    parity=lambda i:sum(((i>>j)&1)*((i>>(j+3))&1) for j in range(3))%2
    indices=[8,16,32,1,2,4] if indices is None else indices
    pairs=[next((a,b) for b in triple for a in [b^i] if parity(a)) for i in indices]
    D=[contact[a]-contact[b] for a,b in pairs]
    print('delta and 2-torsion basis ready',perf_counter()-start,flush=True)
    rational_bitangents=[i for i,ell in linecoeff.items() if all(a**p==a for a in ell)]
    assert len(rational_bitangents)==1
    odd_index=rational_bitangents[0];odd_delta=contact[odd_index];odd_line=linecoeff[odd_index]
    if tangent_coordinates:
        aff=inf=None
        Opt=next(pl.divisor() for pl in F.places_infinite() if pl.degree()==1 and (y/x).evaluate(pl)==0)
    else:
        aff,inf=curve_points(g,k)
        a,b,_=next(P for P in aff if sum(c*t for c,t in zip(odd_line,P)))
        Opt=O.ideal(x-C9(a),y-C9(b)).divisor()
    assert Opt.degree()==1
    # Replace 2*delta by linearly equivalent KC in both determinant spaces.
    rr=[F.one(),x,y]
    s=(delta+Opt).basis_function_space()[0];Dzero=s.divisor()+delta+Opt
    assert Dzero.degree()==3 and Dzero.is_effective()
    residue_cache={};value_cache={}
    def residue_data(place):
        if place not in value_cache:
            KP,_,to=place._residue_field()
            constant=to(F(C9.gen()))
            if KP is C9:inc=lambda v:C9(v);section=inc
            else:inc=C9.hom([constant],KP);section=inc.section()
            coordinates=None if place.is_infinite_place() else (to(x),to(y))
            value_cache[place]=(KP,inc,section,coordinates)
        return value_cache[place]
    def fast_value(ff,place):
        KP,inc,section,coordinates=residue_data(place)
        if coordinates is None:return ff.evaluate(place)
        xx,yy=coordinates
        def polynomial_value(poly):
            out=KP.zero()
            for c in reversed(poly.list()):out=out*xx+inc(c)
            return out
        answer=KP.zero()
        for coefficient in reversed(ff.list()):
            den=polynomial_value(coefficient.denominator())
            if not den:return ff.evaluate(place)
            answer=answer*yy+polynomial_value(coefficient.numerator())/den
        return answer
    def residue_algebra(divisor):
        place=max(divisor.support(),key=lambda pl:pl.degree())
        return residue_data(place)[:3]
    def evaluation_matrix(fs,divisor):
        KP,inc,section=residue_algebra(divisor)
        columns=[]
        for pl,multiplicity in divisor.list():
            for conjugate in range(pl.degree()):
                current=list(fs)
                for order in range(multiplicity):
                    values=[fast_value(ff,pl) for ff in current]
                    values=[inc(v) if v.parent() is C9 else KP(v) for v in values]
                    columns.append(vector(KP,[v**(p**(9*conjugate)) for v in values]))
                    if order+1<multiplicity:
                        assert x.valuation(pl)>=0
                        fy=F(Ry(g(F0.gen(),Ry.gen(),1)).derivative()(y))
                        divisor_derivative=F.one() if fast_value(fy,pl) else y.derivative()
                        current=[ff.derivative()/divisor_derivative for ff in current]
        return matrix(KP,columns).transpose()
    def norm_on_divisor(ff,divisor):
        KP,inc,section=residue_algebra(divisor);answer=KP.one()
        for pl,m in divisor.list():
            a=fast_value(ff,pl);a=inc(a) if a.parent() is C9 else KP(a)
            for j in range(pl.degree()):answer*=a**(m*p**(9*j))
        return answer
    denominator0=evaluation_matrix(rr,Dzero).det()**2;assert denominator0
    functions=[]
    for j,Dj in enumerate(D):
        bs=(Dj+Opt+KC).basis_function_space();assert len(bs)==3
        aa,bb=pairs[j];principal=linefunc[aa]/linefunc[bb]
        assert principal.divisor()==2*Dj
        value=evaluation_matrix(bs,Dzero).det()**2/denominator0*norm_on_divisor(principal,Dzero)
        value=residue_algebra(Dzero)[2](value)
        assert value
        functions.append({'basis':bs,'principal':principal,'at_zero':value})
        print('Weil basis function',j,'ready',perf_counter()-start,flush=True)
    def effective_signature(effective):
        assert effective.degree()==3 and effective.is_effective()
        denominator=evaluation_matrix(rr,effective).det()**2
        if not denominator:return None
        vals=[]
        for fn in functions:
            value=evaluation_matrix(fn['basis'],effective).det()**2/denominator*norm_on_divisor(fn['principal'],effective)
            value=residue_algebra(effective)[2](value)
            vals.append(value/fn['at_zero'])
        return tuple(vals)
    def divisor_signature(Dclass):
        divisor=Dclass+delta+Opt
        s=divisor.basis_function_space()[0];effective=s.divisor()+divisor
        return effective_signature(effective)
    curves=[EllipticCurve(L,[EL.a4()**(p**j),EL.a6()**(p**j)]) for j in range(3)]
    q1=[curves[j](roots[0]**(p**j),0) for j in range(3)]
    q2=[curves[j](roots[1]**(p**j),0) for j in range(3)]
    q3=[curves[j](roots[2]**(p**j),0) for j in range(3)]
    at0=[[L.one(),roots[2]**(p**j),L.zero()] for j in range(3)]
    theta0=contract(T,at0);assert theta0
    null_ratios=[]
    for idx in indices:
        at=at0[:]
        for j,code in enumerate(bits_code(idx)):
            assert code!=3
            if code:at[j]=[L.one(),roots[2-code]**(p**j),L.zero()]
        null_ratios.append((contract(T,at)/theta0)**2)
    def source_signature(P,odd_reference=False):
        if P.is_zero():return None if odd_reference else tuple(C9.one() for _ in indices)
        V=phi(P);points=[curves[j](emb(V[0])**(p**j),emb(V[1])**(p**j)) for j in range(3)]
        if odd_reference:
            codes=bits_code(odd_index)
            for j,code in enumerate(codes):
                if code==1:points[j]+=q1[j]
                elif code==2:points[j]+=q2[j]
                elif code==3:points[j]+=q3[j]
        shifted=[A-B for A,B in zip(points,q3)]
        vv=[[L.one(),U[0],U[1]] for U in shifted];den=contract(T,vv)
        if not den:return None
        values=[]
        for index,idx in enumerate(indices):
            vs=vv[:];normal=L.one()
            for j,code in enumerate(bits_code(idx)):
                assert code!=3
                if not code:continue
                U=shifted[j]+[None,q1,q2][code][j];vs[j]=[L.one(),U[0],U[1]]
                X=points[j][0];normal*=(X-roots[2-code]**(p**j))/(X-roots[2]**(p**j))
            values.append(normal**3*(contract(T,vs)/den)**2/null_ratios[index])
        assert all(a**(p**9)==a for a in values)
        return tuple(sec9(a) for a in values)
    return locals()


if __name__=='__main__':
    set_random_seed(20261028)
    context=setup()
    (ROOT/'results/inverse_calibration.json').write_text(json.dumps({'status':'passed','p':7,
          'quartic':str(context['g']),'ambient_degree':54,'calibrated_functions':6,
          'seconds':perf_counter()-context['start']},indent=2)+'\n')
