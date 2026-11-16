"""Complete a DLP starting from the certified all-obstructed p=109 curve."""
import json
from pathlib import Path
from sage.all import GF,set_random_seed,sage_eval
from quotient_sweep import parse_curve
from transfer_pipeline import solve
ROOT=Path(__file__).resolve().parent


def main():
    seed=20261111;set_random_seed(seed)
    old=json.loads((ROOT/'results/counterexample_audit.json').read_text());p=old['p'];r=old['r']
    K=GF(p**3,name='z');z=K.gen();E=parse_curve(old['curve'],K)
    pd=old['prime_order_certificate']['point']
    P=E([sage_eval(pd[c],locals={'z':z}) for c in ['x','y']]);Q=1729*P
    result=solve(E,P,Q,r,seed=seed,max_isogeny_steps=1)
    assert result['scalar']==1729 and result['isogeny_steps']==1
    assert result['obstruction_walk'][0]['kernels_tested']==12 and not result['obstruction_walk'][0]['usable']
    result['generation_scalar_verified_only_after_solve']=True
    (ROOT/'results/repaired_pipeline.json').write_text(json.dumps(result,indent=2)+'\n')
    print('REPAIRED PIPELINE COMPLETE',result['seconds'],flush=True)


if __name__=='__main__':main()
