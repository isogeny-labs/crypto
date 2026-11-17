"""Reconstruct and certify all 12 quotients for the 23 newly eligible inputs."""
import argparse
import ast
import hashlib
import json
from pathlib import Path
import sys
from time import perf_counter
import traceback
from sage.all import GF, EllipticCurve, PolynomialRing, sage_eval, set_random_seed, version
from quotient_theta import construct

ROOT=Path(__file__).resolve().parent


def parse_curve(text,K):
    expression=text.split('y^2 = ',1)[1].split(' over ',1)[0]
    R=PolynomialRing(K,'x'); x=R.gen()
    f=R(sage_eval(expression,locals={'z':K.gen(),'x':x}))
    return EllipticCurve(K,[f[1],f[0]])


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--limit',type=int,default=23)
    ap.add_argument('--resume',action='store_true')
    args=ap.parse_args(); seed=20261026; set_random_seed(seed)
    source=json.loads((ROOT/'results/curve_probe.json').read_text())
    inputs=[r for r in source['curves'] if r['isogeny_steps']==1][:args.limit]
    out=ROOT/'results/quotient_sweep.json'; old=[]
    if args.resume and out.exists(): old=json.loads(out.read_text())['curves']
    rows=old[:]; start=perf_counter()
    metadata={'sage':version(),'seed':seed,'source_sha256':hashlib.sha256((ROOT/'results/curve_probe.json').read_bytes()).hexdigest(),
              'constructor_sha256':hashlib.sha256((ROOT/'quotient_theta.py').read_bytes()).hexdigest()}
    for i,row in enumerate(inputs):
        if i<len(old) and old[i]['status']=='passed': continue
        try:
            p=row['p']; K=GF(p**3,name='z'); E=parse_curve(row['curve'],K)
            assert int(E.cardinality())==row['prime_group_order']
            phis=E.isogenies_prime_degree(3); assert len(phis)==1
            E1=phis[0].codomain(); assert str(E1)==row['neighbor_curve']
            result=construct(E1,p,verbose=False)
            result.update(source_input=row['curve'],source_j=row['j_invariant'],
                          isogeny_degree=3,status='passed',input_index=i)
        except Exception as exc:
            result={'status':'failed','input_index':i,'p':row['p'],'error':repr(exc),'traceback':traceback.format_exc()}
        if i<len(rows): rows[i]=result
        else: rows.append(result)
        out.write_text(json.dumps({**metadata,'curves':rows,'seconds_this_run':perf_counter()-start},indent=2)+'\n')
        print(json.dumps({'input_index':i,'p':row['p'],'status':result['status'],
              'smooth_quartics':sum(k.get('smooth',False) for k in result.get('kernels',[])),
              'usable_jacobians':sum(k.get('quotient_is_base_field_jacobian',False) for k in result.get('kernels',[])),
              'seconds':result.get('seconds'),'error':result.get('error')}),flush=True)


if __name__=='__main__': main()
