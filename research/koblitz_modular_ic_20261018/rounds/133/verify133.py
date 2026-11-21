#!/usr/bin/env python3
"""Independent carryless-field and Euclidean audit of R133 derivative support."""
import hashlib
import importlib.util
import json
from pathlib import Path

here=Path(__file__).resolve().parent
rounds=here.parent
protocol=json.loads((here/'protocol.json').read_text())
result=json.loads((here/'result.json').read_text())
paths={'r130_excerpt':rounds/'130/source-data/triangular-f-g.sing',
       'r105_parser':rounds/'105/jets105.py',
       'r132_result':rounds/'132/result.json',
       'r132_arithmetic':rounds/'132/base132.py'}
for key,path in paths.items():
    assert hashlib.sha256(path.read_bytes()).hexdigest()==protocol['input_sha256'][key]
spec=importlib.util.spec_from_file_location('source_parser',paths['r105_parser'])
parser=importlib.util.module_from_spec(spec);spec.loader.exec_module(parser)
_,terms=parser.parse_source(paths['r130_excerpt'].read_text())
prior=json.loads(paths['r132_result'].read_text())

def times(a,b,mod):
    full=0
    while b:
        if b&1:full^=a
        b>>=1;a<<=1
    d=mod.bit_length()-1
    while full.bit_length()>d:full^=mod<<(full.bit_length()-d-1)
    return full
def power(a,n,mod):
    z=1
    for bit in bin(n)[2:]:
        z=times(z,z,mod)
        if bit=='1':z=times(z,a,mod)
    return z
def trim(v):
    v=list(v)
    while v and v[-1]==0:v.pop()
    return v
def remainder(a,b,mod):
    a=trim(a);b=trim(b)
    inv=power(b[-1],(1<<(mod.bit_length()-1))-2,mod)
    while len(a)>=len(b):
        k=len(a)-len(b);c=times(a[-1],inv,mod)
        for i,v in enumerate(b):a[k+i]^=times(c,v,mod)
        a=trim(a)
    return a
def gcd(a,b,mod):
    while b:a,b=b,remainder(a,b,mod)
    inv=power(a[-1],(1<<(mod.bit_length()-1))-2,mod)
    return [times(v,inv,mod) for v in a]
def multiply(a,b,mod):
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):z[i+j]^=times(x,y,mod)
    return trim(z)

for old,new in zip(prior['fibers'],result['fibers']):
    mod=old['field_modulus_bits'];d=old['field_degree'];assert d==new['x_degree']
    coeff=[0]*7
    for le,ye,den,exps in terms:
        if le==1:
            num=0
            for e in exps:num^=power(2,e,mod)
            coeff[ye]^=times(num,power(2,(-den)%((1<<d)-1),mod),mod)
    coeff=trim(coeff)
    assert coeff==new['C_coefficients']
    assert gcd(old['specialized_f'],coeff,mod)==new['gcd_f_C']
    product=[1]
    for group,part in zip(old['base_factor_groups'],new['groups']):
        zero=part['zero_derivative_factor'];nonzero=part['nonzero_derivative_factor']
        assert multiply(zero,nonzero,mod)==group['factor']
        assert gcd(group['factor'],coeff,mod)==zero
        assert part['zero_derivative_y_degree']==len(zero)-1
        product=multiply(product,zero,mod)
    assert product==new['gcd_f_C']
    assert new['zero_derivative_y_degree']==1
    assert new['nonzero_derivative_y_degree']==6
assert [f['x_factor'] for f in result['fibers']]==['P11','P22']
print('DERIVATIVE133_INDEPENDENT_GCD_PASS')
