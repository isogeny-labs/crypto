"""Direct regression of geometric graph substitution on an actual Jacobian."""
import json
from pathlib import Path
from sage.all import GF,PolynomialRing,sage_eval
from target_index_calculus import jacobian_context
from large_prime_index_calculus import solve
ROOT=Path(__file__).resolve().parent


def main():
    d=json.loads((ROOT/'results/polynomial_inverse.json').read_text());k=GF(7)
    R=PolynomialRing(k,['X','Y','Z']);X,Y,Z=R.gens()
    g=R(sage_eval(d['quartic'],locals={'X':X,'Y':Y,'Z':Z}))
    F,G,B,base,places=jacobian_context(g)
    S=G.point(places[1].divisor()-B.divisor());T=173*S
    result=solve(S,T,373,base,places,g,factor_base_constant=2)
    assert result['scalar']==173
    (ROOT/'results/large_prime_check.json').write_text(json.dumps(result,indent=2)+'\n')
    print('LARGE PRIME CHECK COMPLETE',result['seconds'],flush=True)


if __name__=='__main__':main()
