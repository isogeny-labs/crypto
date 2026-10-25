"""Independently verify source answers and measure matching generic baselines."""
import json
from pathlib import Path
from sage.all import GF,EllipticCurve,sage_eval,version
from quotient_sweep import parse_curve
from common import bsgs_ec,point_data,field_data
ROOT=Path(__file__).resolve().parent


def main():
    old=json.loads((ROOT/'results/curve_probe.json').read_text())
    for p in [7,13]:
        path=ROOT/('results/polynomial_inverse.json' if p==7 else f'results/polynomial_inverse_p{p}.json')
        out=json.loads(path.read_text());assert out['status']=='passed'
        K=GF(p**3,name='z');z=K.gen()
        if p==7:
            E=EllipticCurve(K,[4*z*z+3*z+3,4*z*z+5*z+2])
            P=E(2*z*z+5*z+5,6*z*z+6*z+1);Q=E(4*z*z+2*z,z+2)
        else:
            row=next(r for r in old['curves'] if r['p']==p and r['isogeny_steps']==1 and 'source_point' in r)
            E=parse_curve(row['curve'],K)
            P=E([sage_eval(row['source_point'][c],locals={'z':z}) for c in ['x','y']])
            Q=E([sage_eval(row['source_target'][c],locals={'z':z}) for c in ['x','y']])
        assert out['recovered_scalar']*P==Q
        baseline=bsgs_ec(P,Q,out['r']);assert baseline['scalar']==out['recovered_scalar']
        assert not out['target_group_enumeration_used']
        assert out['section_certificate']['extension_field_point_enumeration_used'] is False
        out.update(sage=version(),source_curve=str(E),source_field=field_data(K),
                   source_generator=point_data(P),source_target=point_data(Q),source_bsgs=baseline,
                   limits=['Only small-field complete DLP experiments are claimed.',
                           'The target solver is basic smooth-divisor index calculus, not optimized Diem.',
                           'The driver uses generic affine charts; comprehensive nongeneric input handling is unfinished.',
                           'No cryptographic-sized DLP or end-to-end speedup is demonstrated.'])
        path.write_text(json.dumps(out,indent=2)+'\n')
        print('SOURCE VERIFIED',p,out['r'],out['recovered_scalar'],'BSGS seconds',baseline['seconds'],flush=True)


if __name__=='__main__':main()
