"""Verify saved large source-order certificates without CM or point counting."""
import json
from math import isqrt
from pathlib import Path
from sage.all import GF,PolynomialRing,Integer,sage_eval
from quotient_sweep import parse_curve
ROOT=Path(__file__).resolve().parent


def main():
    rows=[]
    for path in sorted((ROOT/'results').glob('large_instance_*.json')):
        d=json.loads(path.read_text());p=d['p'];r=d['r'];cert=d['order_certificate']
        assert Integer(p).is_prime() and Integer(r).is_prime()
        R=PolynomialRing(GF(p),'x');x=R.gen()
        K=GF(p**3,name='z',modulus=R(sage_eval(d['field_modulus'],locals={'x':x})))
        E=parse_curve(cert['certifying_curve'],K)
        P=E([sage_eval(v,locals={'z':K.gen()}) for v in cert['nonzero_point']])
        assert not P.is_zero() and (r*P).is_zero()
        lo=p**3+1-isqrt(4*p**3);hi=p**3+1+isqrt(4*p**3)
        assert cert['hasse_interval']==[lo,hi] and (lo+r-1)//r==hi//r==1
        E.set_order(r,num_checks=0)
        source=parse_curve(d['source_curve'],K)
        assert any(str(phi.codomain())==str(source) for phi in E.isogenies_prime_degree(3))
        assert len(source.division_polynomial(3).roots(multiplicities=False))==1
        phis=source.isogenies_prime_degree(3);assert len(phis)==1
        prepared=phis[0].codomain();assert str(prepared)==d['prepared_curve']
        assert len(prepared.division_polynomial(3).roots(multiplicities=False))==4
        assert all(bool(GF(p)(k['chi18_up_to_square_factor']).is_square())==k['predicts_base_field_jacobian'] for k in d['kernels'])
        rows.append({'file':path.name,'r_bits':r.bit_length(),'prime_order_certificate_passed':True,
                     'descending_and_preprocessing_isogenies_verified':True,
                     'source_one_3_kernel_line':True,'prepared_four_3_kernel_lines':True})
    (ROOT/'results/large_source_verification.json').write_text(json.dumps({'status':'passed','instances':rows},indent=2)+'\n')
    print(json.dumps(rows),flush=True)


if __name__=='__main__':main()
