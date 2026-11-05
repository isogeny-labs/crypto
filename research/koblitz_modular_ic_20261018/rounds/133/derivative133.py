#!/usr/bin/env python3
"""Exact G51 lambda-derivative support over the R132 base fibers."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
def load(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

def main(dest):
    protocol=json.loads((HERE/'protocol.json').read_text())
    paths={'r130_excerpt':ROUNDS/'130/source-data/triangular-f-g.sing',
           'r105_parser':ROUNDS/'105/jets105.py',
           'r132_result':ROUNDS/'132/result.json',
           'r132_arithmetic':ROUNDS/'132/base132.py'}
    for key,path in paths.items():assert sha(path)==protocol['input_sha256'][key],key
    parser=load(paths['r105_parser'],'r105_pinned')
    base=load(paths['r132_arithmetic'],'r132_pinned')
    _,gterms=parser.parse_source(paths['r130_excerpt'].read_text())
    assert len(gterms)==29 and {le for le,_,_,_ in gterms}=={0,1,4,8,16}
    prior=json.loads(paths['r132_result'].read_text())
    result={'protocol_id':protocol['id'],'input_sha256':protocol['input_sha256'],'fibers':[]}
    for fiber in prior['fibers']:
        F=base.Field(fiber['field_modulus_bits'])
        x=2
        C=[0]*7
        for le,ye,den,exps in gterms:
            if le!=1:continue
            numerator=0
            for exponent in exps:numerator^=F.power(x,exponent)
            C[ye]^=F.mul(numerator,F.inv(F.power(x,den)))
        C=base.trim(C)
        f=fiber['specialized_f']
        common=base.gcd(f,C,F)
        parts=[]
        common_product=[1]
        for group in fiber['base_factor_groups']:
            piece=base.gcd(group['factor'],C,F)
            quotient,remainder=base.divmod_poly(group['factor'],piece,F)
            assert not remainder
            parts.append({'base_irreducible_y_degree':group['irreducible_y_degree'],
                          'base_factor_count':group['factor_count'],
                          'zero_derivative_factor':piece,
                          'zero_derivative_y_degree':len(piece)-1,
                          'nonzero_derivative_factor':quotient})
            common_product=base.mul(common_product,piece,F)
        assert common_product==common
        result['fibers'].append({'x_factor':fiber['x_factor'],
                                'x_degree':fiber['field_degree'],
                                'C_coefficients':C,
                                'gcd_f_C':common,
                                'zero_derivative_y_degree':len(common)-1,
                                'nonzero_derivative_y_degree':7-(len(common)-1),
                                'groups':parts})
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
    print('DERIVATIVE133_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
