"""A basic smooth-divisor index-calculus solver on an actual plane-quartic Jacobian.

This uses all rational curve points as a factor base. It is not Diem's optimized
line/large-prime implementation and establishes no O(p) performance bound.
The solver receives only S,T,r and never receives their discrete logarithm.
"""
import json,random,sys
from pathlib import Path
from time import perf_counter
from sage.all import GF,PolynomialRing,FunctionField,matrix,vector,sage_eval
from quotient_theta import curve_points
ROOT=Path(__file__).resolve().parent;sys.path.insert(0,str(ROOT.parent))
from tian import certified_plane_orders


def jacobian_context(g):
    k=g.base_ring();F0=FunctionField(k,'x');x=F0.gen();Ry=PolynomialRing(F0,'y');y=Ry.gen()
    F=certified_plane_orders(F0.extension(Ry(g(x,y,1)).monic(),'y'));assert F.genus()==3
    x=F(x);y=F.gen();O=F.maximal_order();aff,_=curve_points(g,k)
    places=[O.ideal(x-a,y-b).divisor().support()[0] for a,b,_ in aff]
    places += [pl for pl in F.places_infinite() if pl.degree()==1]
    B=places[0];base=3*B.divisor();G=F.jacobian(base_div=base).group()
    return F,G,B,base,places


def solve(S,T,r,base_div,places,seed=20261030,max_trials=10000):
    start=perf_counter();G=S.parent();zero=G.zero();assert not S==zero and r*S==zero and r*T==zero
    B=places[0];fbplaces=places[1:];fb=[G.point(pl.divisor()-B.divisor()) for pl in fbplaces]
    field=GF(r);rng=random.Random(seed);rows=[];rhs=[];trials=0;smooth=0;rank=0;records=[]
    while trials<max_trials:
        trials+=1;a=rng.randrange(r);b=rng.randrange(r);V=a*S+b*T
        D=V.divisor()+base_div
        assert D.degree()==3 and D.is_effective()
        if any(pl.degree()!=1 for pl in D.support()):continue
        coefficients=[int(D.valuation(pl)) for pl in fbplaces]
        # Validate the decomposition in the actual Jacobian before recording it.
        reconstructed=zero
        for c,Fb in zip(coefficients,fb):reconstructed+=c*Fb
        assert reconstructed==V
        smooth+=1;row=coefficients+[-b]
        new=matrix(field,rows+[row]);newrank=int(new.rank())
        if newrank>rank:
            rows.append(row);rhs.append(a);rank=newrank
            records.append({'a':a,'b':b,'factor_base_coefficients':coefficients,'rank':rank})
        if rank==len(fb)+1:
            answer=matrix(field,rows).solve_right(vector(field,rhs));m=int(answer[-1])
            assert m*S==T
            return {'scalar':m,'seconds':perf_counter()-start,'trials':trials,'smooth_relations':smooth,
                    'factor_base_size':len(fb),'linear_system_rank':rank,'independent_relations':records,
                    'algorithm':'basic rational-point smooth-divisor index calculus',
                    'claim':'Actual target Jacobian DLP; not an optimized Diem solver or speedup claim'}
    raise RuntimeError('Index-calculus trial budget exhausted')


if __name__=='__main__':
    d=json.loads((ROOT/'results/theta_point_probe.json').read_text());k=GF(7)
    R=PolynomialRing(k,['X','Y','Z']);X,Y,Z=R.gens()
    g=R(sage_eval(d['associated_quartic'],locals={'X':X,'Y':Y,'Z':Z}))
    F,G,B,base,places=jacobian_context(g)
    S=G.point(places[1].divisor()-B.divisor());T=173*S
    result=solve(S,T,373,base,places);assert result['scalar']==173
    (ROOT/'results/index_calculus_test.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result),flush=True)
