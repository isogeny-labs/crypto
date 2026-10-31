"""Targeted arithmetic checks for exceptional divisor recovery and API inputs."""
import json
from pathlib import Path
from time import perf_counter
from sage.all import GF,PolynomialRing,sage_eval,set_random_seed
from polynomial_inverse import effective_divisor
from target_index_calculus import jacobian_context
from transfer_pipeline import solve,instance
ROOT=Path(__file__).resolve().parent


def main():
    set_random_seed(20261110);start=perf_counter();k=GF(7)
    R=PolynomialRing(k,['X','Y','Z']);X,Y,Z=R.gens()
    old=json.loads((ROOT/'results/polynomial_inverse.json').read_text())
    g=R(sage_eval(old['quartic'],locals={'X':X,'Y':Y,'Z':Z}))
    F,G,B,base,places=jacobian_context(g);x=F(F.base_field().gen());y=F.gen()
    rational=[pl for pl in places if not pl.is_infinite_place()]
    pairs=[(x.evaluate(pl),y.evaluate(pl)) for pl in rational]
    a=next(a for a,b in pairs if sum(aa==a for aa,bb in pairs)>1)
    equal_x=[P for P in pairs if P[0]==a][:2];third=next(P for P in pairs if P[0]!=a)
    checks=[]
    for label,points in [('coincident_x_distinct_y',equal_x+[third]),('repeated_rational_point',[equal_x[0]]*2+[third])]:
        D=effective_divisor(F,points)
        expected=sum((next(pl for pl in rational if x.evaluate(pl)==a and y.evaluate(pl)==b).divisor()
                      for a,b in points),base.parent().zero())
        assert D==expected;checks.append({'case':label,'degree':int(D.degree()),'passed':True})
    # A degree-two closed point with rational x and non-rational y checks the
    # case where interpolation y=V(x) is mathematically impossible.
    L=GF(7**2,name='w');Ru=PolynomialRing(L,'u');u=Ru.gen();closed=None
    for aa in k:
        roots=Ru(g(L(aa),u,1)).roots(multiplicities=False)
        for bb in roots:
            if bb**7!=bb:closed=[(L(aa),bb),(L(aa),bb**7)];break
        if closed:break
    assert closed
    thirdL=(L(third[0]),L(third[1]));D=effective_divisor(F,closed+[thirdL])
    assert sorted(pl.degree() for pl in D.support())==[1,2]
    checks.append({'case':'quadratic_closed_point_with_rational_x','degree':3,'passed':True})
    E,P,Q,r,secret=instance(7,20261110)
    for label,target,expected in [('zero',E(0),0),('equal',P,1),('opposite',-P,r-1)]:
        out=solve(E,P,target,r);assert out['scalar']==expected and not out['target_solver_used']
        checks.append({'case':label,'passed':True})
    try:solve(E,E(0),P,r)
    except ValueError:checks.append({'case':'zero_generator_rejected','passed':True})
    else:raise AssertionError('Zero generator accepted')
    out={'status':'passed','checks':checks,'seconds':perf_counter()-start,
         'scope':'Closed-place recovery, multiplicities, trivial targets and invalid generator; not complete generic-chart coverage'}
    (ROOT/'results/transfer_edge_cases.json').write_text(json.dumps(out,indent=2)+'\n');print(out,flush=True)


if __name__=='__main__':main()
