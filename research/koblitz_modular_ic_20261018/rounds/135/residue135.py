#!/usr/bin/env python3
"""Exact repeated lambda residue factors at P11 and P22 exceptional base places."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent;ROUNDS=HERE.parent
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def load(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod

def ddf(g,F,poly):
    total=len(g)-1;remaining=g[:];h=[0,1];parts=[]
    for d in range(1,total+1):
        if len(remaining)==1:break
        h=poly.frobenius_q(h,remaining,F)
        factor=poly.gcd(remaining,poly.add(h,[0,1]),F)
        if len(factor)>1:
            assert (len(factor)-1)%d==0
            count=(len(factor)-1)//d
            parts.append({'irreducible_degree':d,'count':count,'factor':factor})
            quotient,remainder=poly.divmod_poly(remaining,factor,F)
            assert not remainder
            remaining=quotient
            if len(remaining)>1:h=poly.divmod_poly(h,remaining,F)[1]
    assert remaining==[1]
    product=[1]
    for item in parts:product=poly.mul(product,item['factor'],F)
    assert product==g and sum(item['irreducible_degree']*item['count'] for item in parts)==total
    return parts

def main(dest):
    p=json.loads((HERE/'protocol.json').read_text())
    paths={'r130_excerpt':ROUNDS/'130/source-data/triangular-f-g.sing',
           'r105_parser':ROUNDS/'105/jets105.py',
           'r133_result':ROUNDS/'133/result.json',
           'r132_result':ROUNDS/'132/result.json',
           'r132_arithmetic':ROUNDS/'132/base132.py',
           'r130_polynomials':ROUNDS/'130/lambda130.py'}
    for key,path in paths.items():assert sha(path)==p['input_sha256'][key],key
    parser=load(paths['r105_parser'],'parser105_pinned')
    base=load(paths['r132_arithmetic'],'base132_pinned')
    poly=load(paths['r130_polynomials'],'poly130_pinned')
    _,terms=parser.parse_source(paths['r130_excerpt'].read_text())
    old=json.loads(paths['r132_result'].read_text())
    support=json.loads(paths['r133_result'].read_text())
    result={'protocol_id':p['id'],'input_sha256':p['input_sha256'],'fibers':[]}
    for fiber,deriv in zip(old['fibers'],support['fibers']):
        label=fiber['x_factor'];y=p['exceptional_roots'][label]
        assert deriv['x_factor']==label and deriv['gcd_f_C']==[y,1]
        B=base.Field(fiber['field_modulus_bits']);B.q_bits=B.degree
        coefficients=[0]*17
        for le,ye,den,exps in terms:
            numerator=0
            for exponent in exps:numerator^=B.power(2,exponent)
            value=B.mul(numerator,B.inv(B.power(2,den)))
            coefficients[le]^=B.mul(value,B.power(y,ye))
        assert coefficients[16]==1 and coefficients[1]==0
        power=1
        while all(i% (2*power)==0 for i,c in enumerate(coefficients) if c):power*=2
        assert power>=2 and power<=16
        k=power.bit_length()-1
        radical=[B.power(coefficients[power*j],1<<(B.degree-k)) for j in range(16//power+1)]
        reconstruction=[0]*17
        for j,c in enumerate(radical):reconstruction[power*j]=B.power(c,power)
        assert reconstruction==coefficients
        derivative=poly.trim([radical[i+1] if (i+1)&1 else 0 for i in range(len(radical)-1)])
        assert poly.gcd(radical,derivative,B)==[1]
        factors=ddf(radical,B,poly)
        result['fibers'].append({'x_factor':label,'field_degree':B.degree,
                                 'y_residue':y,'G_coefficients':coefficients,
                                 'power':power,'radical':radical,'radical_derivative':derivative,
                                 'degree_parts':factors,
                                 'monogenic_length_check':power*sum(t['irreducible_degree']*t['count'] for t in factors)})
        assert result['fibers'][-1]['monogenic_length_check']==16
        print('FIBER',label,'power',power,'parts',[(z['irreducible_degree'],z['count']) for z in factors],flush=True)
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
    print('RESIDUE135_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
