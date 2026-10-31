"""Base-field polynomial inversion of new cubic quotient images.

Constructs Tian's eight symmetric sections using a rational tangent chart and
an odd theta reference. The last polynomial system has coefficients in F_p.
No target-group enumeration or source discrete logarithm is used.
"""
import argparse,itertools,json,random
from pathlib import Path
from time import perf_counter
from sage.all import GF,PolynomialRing,matrix,vector,set_random_seed
from inverse_map import setup,half_divisor
from quotient_theta import curve_points
from target_index_calculus import jacobian_context,solve as index_calculus
from quotient_sweep import parse_curve
from sage.all import sage_eval
ROOT=Path(__file__).resolve().parent


class ChartFailure(ValueError):
    """An explicitly recognized exceptional evaluation/inversion chart."""


def sampled_points(g,K,count,rng):
    R=PolynomialRing(K,'u');u=R.gen();points=[]
    for _ in range(100*count):
        a=K.random_element();ys=R(g(a,u,1)).roots(multiplicities=False)
        if ys:points.append((a,rng.choice(ys),K.one()))
        if len(points)==count:return points
    raise RuntimeError('Random curve-point sampling budget exhausted')


def symmetric_sections(g,seed=20261101):
    k=g.base_ring();p=int(k.cardinality());R=g.parent();X,Y,Z=R.gens()
    infinity=g(X,Y,0)
    assert all(e[1]>=2 for e,c in infinity.dict().items())
    q=R({(e[0],e[1]-2,0):c for e,c in infinity.dict().items()})
    assert infinity==Y**2*q and q(1,0,0)
    B=[R.one(),X,Y,X*X,X*Y,Y*Y,X*q,Y*q]
    K=GF(p**3,name='b');rng=random.Random(seed);aff=sampled_points(g,K,192,rng)
    triples=list(itertools.combinations_with_replacement(range(8),3));rows=[]
    for _ in range(80):
        P,Q=rng.sample(aff,2);a=[f(*P) for f in B];b=[f(*Q) for f in B]
        vals=[b[i]*a[j]*a[l]+a[i]*b[j]*a[l]+a[i]*a[j]*b[l] for i,j,l in triples]
        for power in range(3):
            rows.append([v.polynomial()[power] for v in vals])
    M=matrix(k,rows);assert M.rank()==112
    basis=M.right_kernel().basis();assert len(basis)==8
    S=PolynomialRing(k,['x1','y1','x2','y2','x3','y3']);v=S.gens()
    BB=[[f(v[2*j],v[2*j+1],1) for f in B] for j in range(3)]
    polys=[]
    for i,j,l in triples:
        polys.append(sum(BB[t[0]][i]*BB[t[1]][j]*BB[t[2]][l] for t in itertools.permutations(range(3))))
    zeta=[sum(c*f for c,f in zip(row,polys)) for row in basis]
    # Exact diagonal identities, beyond the interpolation samples.
    D=PolynomialRing(k,['a','b','u','v'],order='lex');a,b,u,w=D.gens()
    C1=g(u,a,1);C2=g(w,b,1)
    assert all(not f(u,a,u,a,w,b).reduce([C1,C2]) for f in zeta)
    return zeta,B,{'diagonal_constraint_rank':112,'section_dimension':8,'sampled_curve_points':192,
                   'extension_field_point_enumeration_used':False,
                   'exact_diagonal_identities_verified':8,'maximum_total_degree':max(int(f.total_degree()) for f in zeta)}


def interpolate(ctx,zeta,B,seed=20261102):
    rng=random.Random(seed);C=ctx['C9'];p=ctx['p'];g=ctx['g']
    K=GF(p**3,name='b');emb=K.embeddings(C)[0];aff=sampled_points(g,K,96,rng)
    def evaluate(ff,a,b):
        out=C.zero()
        for c in reversed(ff.list()):
            den=c.denominator()(a)
            if not den:raise ZeroDivisionError
            out=out*b+c.numerator()(a)/den
        return out
    rows=[];right=[];holdouts=0
    while len(rows)<8 or holdouts<12:
        points=[(emb(P[0]),emb(P[1])) for P in rng.sample(aff,3)]
        omega=matrix(C,[[1,a,b] for a,b in points]).det()**2
        if not omega:continue
        try:
            values=[omega]
            for fn in ctx['functions']:
                value=matrix(C,[[evaluate(ff,a,b) for a,b in points] for ff in fn['basis']]).det()**2
                for a,b in points:value*=evaluate(fn['principal'],a,b)
                values.append(value)
        except ZeroDivisionError:continue
        zz=vector(C,[f(*[c for P in points for c in P]) for f in zeta])
        if len(rows)<8:
            if matrix(C,rows+[values]).rank()!=len(rows)+1:continue
            rows.append(values);right.append(zz)
            if len(rows)==8:transform=matrix(C,rows).inverse()*matrix(C,right)
        else:
            assert vector(C,values)*transform==zz
            holdouts+=1
    return transform,holdouts


def image_coordinates(ctx,transform,P):
    try:mu=ctx['source_signature'](P,odd_reference=True)
    except ZeroDivisionError:return None
    if mu is None:return None
    vals=vector(ctx['C9'],[1]+[fn['at_zero']*a for fn,a in zip(ctx['functions'],mu)])*transform
    assert any(vals)
    # A stronger check than projective descent: these ratios are individually rational.
    assert all(a**ctx['p']==a for a in vals),'image coordinates failed base-field descent'
    k=GF(ctx['p']);return vector(k,[k(a) for a in vals])


def invert(g,zeta,coordinates):
    start=perf_counter();k=g.base_ring();p=int(k.cardinality())
    S=PolynomialRing(k,['x1','y1','x2','y2','x3','y3','t'],order='degrevlex')
    v=S.gens();zz=[S(f) for f in zeta];pivot=next(i for i,a in enumerate(coordinates) if a)
    equations=[g(v[2*i],v[2*i+1],1) for i in range(3)]
    equations += [coordinates[pivot]*zz[i]-coordinates[i]*zz[pivot] for i in range(8) if i!=pivot]
    equations += [v[6]*zz[pivot]-1]
    I=S.ideal(equations)
    print('GROEBNER START',flush=True)
    gb=I.groebner_basis(algorithm='libsingular:slimgb')
    print('GROEBNER DONE',perf_counter()-start,'basis',len(gb),flush=True)
    dimension=int(I.dimension())
    if dimension!=0:
        raise ChartFailure(f'Inversion chart has fiber dimension {dimension}; a common-scalar retry is required')
    lex=S.change_ring(order='lex')
    transformed=I.transformed_basis('fglm',other_ring=lex)
    J=lex.ideal(transformed)
    solutions=J.variety(ring=GF(p**6,name='d'))
    if not solutions:raise ChartFailure('No affine divisor in the selected inversion chart')
    print('SOLUTIONS',len(solutions),'seconds',perf_counter()-start,flush=True)
    return solutions,lex,{'seconds':perf_counter()-start,'ordered_solutions':len(solutions),
                         'groebner_basis_length':len(gb),'equations':len(equations),
                         'algorithm':'base-field symmetric-section polynomial inversion'}


def effective_divisor(F,pairs):
    """Recover closed places, including coincident x coordinates and multiplicity.

    Fast interpolation handles the generic case. Otherwise factor the x-fiber
    and identify its closed place by the two residue coordinates. This also
    handles y not belonging to the field generated by x.
    """
    x=F(F.base_field().gen());y=F.gen();O=F.maximal_order();k=F.constant_base_field()
    p=int(k.cardinality());L=pairs[0][0].parent();R=PolynomialRing(L,'u');u=R.gen()
    if len({a for a,b in pairs})==len(pairs):
        U=R.one()
        for a,b in pairs:U*=u-a
        V=R.lagrange_polynomial(pairs)
        if not all(c**p==c for c in list(U)+list(V)):
            raise ChartFailure('Recovered divisor is not Frobenius invariant')
        U=PolynomialRing(k,'u')([k(c) for c in U]);V=PolynomialRing(k,'u')([k(c) for c in V])
        D=O.ideal(U(x),y-V(x)).divisor()
    else:
        from collections import Counter
        remaining=Counter(pairs);D=x.divisor().parent().zero()
        while remaining:
            a,b=next(iter(remaining));orbit=[];point=(a,b)
            while point not in orbit:
                orbit.append(point);point=(point[0]**p,point[1]**p)
            multiplicity=remaining[(a,b)]
            if any(remaining.get(P,0)!=multiplicity for P in orbit):
                raise ChartFailure('Recovered repeated-coordinate divisor is not rational')
            for P in orbit:del remaining[P]
            U=a.minimal_polynomial();candidates=O.ideal(U(x)).factor();found=None
            for prime,exponent in candidates:
                pl=prime.divisor().support()[0]
                if pl.degree()!=len(orbit):continue
                KP,_,to=pl._residue_field();xx,yy=to(x),to(y)
                embeddings=[lambda c:L(c)] if KP is k else KP.embeddings(L)
                if any(emb(xx)==a and emb(yy)==b for emb in embeddings):found=pl;break
            if found is None:raise ChartFailure('No closed place matches the recovered coordinates')
            D+=multiplicity*found.divisor()
    assert D.degree()==len(pairs) and D.is_effective()
    return D


def divisor_images(g,ctx,solutions,ring,group_context=None):
    F,G,B,base,places=jacobian_context(g) if group_context is None else group_context
    x=F(F.base_field().gen());y=F.gen();O=F.maximal_order();k=g.base_ring()
    line=[k(c) for c in ctx['odd_line']]
    canonical=x.divisor().denominator();odd=half_divisor((line[0]*x+line[1]*y+line[2]).divisor()+canonical)
    Opt=next(pl.divisor() for pl in F.places_infinite() if pl.degree()==1 and (y/x).evaluate(pl)==0)
    reference=odd+Opt;images=[]
    names=ring.gens()
    for solution in solutions:
        pairs=[(solution[names[2*i]],solution[names[2*i+1]]) for i in range(3)]
        divisor=effective_divisor(F,pairs)
        image=G.point(divisor-reference)
        if not any(image==other for other in images):images.append(image)
    assert len(images) in [1,2]
    if len(images)==2:assert images[0]==-images[1]
    else:images.append(-images[0])
    return images,(F,G,B,base,places)


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--solve',action='store_true')
    ap.add_argument('--p',type=int,default=7);args=ap.parse_args()
    set_random_seed(20261101);start=perf_counter()
    source_curve=None
    if args.p!=7:
        data=json.loads((ROOT/'results/curve_probe.json').read_text())
        row=next(r for r in data['curves'] if r['p']==args.p and r['isogeny_steps']==1 and 'source_point' in r)
        source_curve=parse_curve(row['curve'],GF(args.p**3,name='z'))
    ctx=setup(tangent_coordinates=True,indices=[8,16,24,32,40,48,56],source_curve=source_curve);print('SETUP READY',flush=True)
    zeta,B,certificate=symmetric_sections(ctx['g']);print('SECTIONS READY',certificate,flush=True)
    transform,holdouts=interpolate(ctx,zeta,B);print('INTERPOLATION READY',flush=True)
    E=ctx['source'];z=E.base_field().gen();r=int(E.cardinality())
    if args.p==7:P=E(2*z*z+5*z+5,6*z*z+6*z+1);Q=E(4*z*z+2*z,z+2)
    else:
        P=E([sage_eval(row['source_point'][c],locals={'z':z}) for c in ['x','y']])
        Q=E([sage_eval(row['source_target'][c],locals={'z':z}) for c in ['x','y']])
    for multiplier in range(1,min(r,100)):
        coordinates=[image_coordinates(ctx,transform,multiplier*R) for R in [P,Q,P+Q]]
        if all(c is not None for c in coordinates):break
    else:raise AssertionError('No admissible source chart')
    out={'status':'coordinates_passed','p':args.p,'r':r,'quartic':str(ctx['g']),
         'section_certificate':certificate,'interpolation_holdouts':holdouts,
         'source_randomization_multiplier':multiplier,'image_coordinates':[[int(a) for a in c] for c in coordinates],
         'all_image_coordinates_in_base_field':True,'target_group_enumeration_used':False,
         'zeta':[str(f) for f in zeta],'seconds':perf_counter()-start}
    path=ROOT/('results/polynomial_inverse.json' if args.p==7 else f'results/polynomial_inverse_p{args.p}.json');path.write_text(json.dumps(out,indent=2)+'\n')
    if not args.solve:return
    lifted=[];timings=[];group_context=None
    for c in coordinates:
        solutions,ring,timing=invert(ctx['g'],zeta,c)
        images,group_context=divisor_images(ctx['g'],ctx,solutions,ring)
        lifted.append(images);timings.append(timing)
    S=lifted[0][0];T=next(T for T in lifted[1] if any(S+T==U for U in lifted[2]))
    F,G,B,base,places=group_context;result=index_calculus(S,T,r,base,places)
    assert result['scalar']*P==Q
    out.update(status='passed',recovered_scalar=result['scalar'],inversion_runs=timings,
               target_index_calculus=result,three_image_sign_recovery=True,seconds=perf_counter()-start)
    path.write_text(json.dumps(out,indent=2)+'\n');print('COMPLETE',result['scalar'],out['seconds'],flush=True)


if __name__=='__main__':main()
