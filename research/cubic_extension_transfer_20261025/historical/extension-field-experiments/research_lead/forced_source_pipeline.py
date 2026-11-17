"""End-to-end transfer from a source forced through an obstructed prepared model."""
import json
from pathlib import Path
from sage.all import GF,set_random_seed
from quotient_sweep import parse_curve
from transfer_pipeline import solve
ROOT=Path(__file__).resolve().parent


def main():
    seed=20261114;set_random_seed(seed)
    rows=json.loads((ROOT/'results/neighborhood_random.json').read_text())['curves']
    bad=next(row for row in rows if row['usable_witness'] is None
             and any(n['three_lines']==1 for n in row['neighbors']))
    row=next(n for n in bad['neighbors'] if n['three_lines']==1);p=bad['p'];r=bad['r']
    K=GF(p**3,name='z');E=parse_curve(row['curve'],K);P=E.random_point()
    while P.is_zero():P=E.random_point()
    assert (r*P).is_zero();Q=271828*P
    out=solve(E,P,Q,r,seed=seed,target_solver='large-prime',max_isogeny_steps=2)
    assert out['scalar']==271828 and out['isogeny_steps']==2
    assert out['obstruction_walk'][0]['kernels_tested']==12 and not out['obstruction_walk'][0]['usable']
    out['generation_scalar_verified_only_after_solve']=True
    (ROOT/'results/forced_source_pipeline.json').write_text(json.dumps(out,indent=2)+'\n')
    print('FORCED SOURCE PIPELINE COMPLETE',out['p'],out['r'],out['seconds'],flush=True)


if __name__=='__main__':main()
