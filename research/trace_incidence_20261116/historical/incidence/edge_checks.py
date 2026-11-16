"""Small untimed rejection/exception controls for the incidence candidate."""
import copy
import json
from pathlib import Path
from candidate import Ring,product_poly,normalized_s3,incidence_gcd,evaluate_horner,nmod_poly

ROOT=Path(__file__).parent


def main():
    fixture=json.loads((ROOT.parent/'prime_fixtures/b008_c0_natural_s0.json').read_text())
    duplicate=copy.deepcopy(fixture)
    duplicate['pools'][0][0]['x']=duplicate['pools'][2][0]['x']
    try:incidence_gcd(duplicate)
    except ValueError:duplicate_rejected=True
    else:raise AssertionError('Duplicate coordinates accepted')
    curve=fixture['curve'];p=curve['p']
    h=product_poly([r['x'] for r in fixture['pools'][2]],p)
    ring=Ring(p,h)
    try:normalized_s3(fixture['pools'][2][0]['x'],curve,ring)
    except ValueError:nonunit_rejected=True
    else:raise AssertionError('Nonunit denominator accepted')
    count=0
    p=101;h=product_poly([1,2,3],p);ring=Ring(p,h)
    for repeated_root in (0,1,7,100):
        beta=ring.c(-2*repeated_root);gamma=ring.c(repeated_root**2)
        for coeffs in ([7,6,1],[-repeated_root,1],[1,0,0,0,1]):
            u,v=evaluate_horner(coeffs,beta,gamma,ring)
            norm=ring.mul(u,u)-ring.mul(beta,ring.mul(u,v))+ring.mul(gamma,ring.mul(v,v))
            scalar=sum(c*pow(repeated_root,i,p) for i,c in enumerate(coeffs))%p
            if norm!=ring.c(scalar*scalar):raise AssertionError('Repeated-root norm failed')
            count+=1
    result={'schema':'incidence-edge-checks/v1','duplicate_coordinates_rejected':duplicate_rejected,
        'nonunit_denominator_rejected':nonunit_rejected,'repeated_quadratic_root_norm_cases':count,
        'all_passed':True,'scope':'Caller supplies finite valid points in globally disjoint pools; rejection is explicit, not a miss.'}
    (ROOT/'edge_check_results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))


if __name__=='__main__':main()
