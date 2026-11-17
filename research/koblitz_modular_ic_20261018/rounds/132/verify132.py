#!/usr/bin/env python3
"""Independent carryless-field and Frobenius checks for the R132 base fibers."""
import hashlib
import importlib.util
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
p=json.loads((HERE/'protocol.json').read_text())
r=json.loads((HERE/'result.json').read_text())
paths={'bad_factors':HERE/'source-data/bad-factors.txt',
       'r51_source':ROUNDS/'51/source-data/marked23-field51.sing',
       'r100_verifier':ROUNDS/'100/verify100.py','r127_parser':ROUNDS/'127/fiber127.py'}
for key,path in paths.items():assert hashlib.sha256(path.read_bytes()).hexdigest()==p['input_sha256'][key]
original=ROUNDS/'100/runs/factor100-02/factors100.sing'
if original.exists():
    raw=original.read_bytes();assert hashlib.sha256(raw).hexdigest()==p['r100_full_factor_sha256']
    matches={int(i):expr for i,expr,_ in re.findall(r'factors100\[(\d+)\]=([^;]+);exponents100\[\1\]=(\d+);',raw.decode())}
    assert paths['bad_factors'].read_text()==f'P11={matches[1]}\nP22={matches[2]}\n'
spec=importlib.util.spec_from_file_location('r127parse',paths['r127_parser'])
parser=importlib.util.module_from_spec(spec);spec.loader.exec_module(parser)
terms=parser.parse_source(paths['r51_source'].read_text())

class K:
    def __init__(self,mod):self.mod=mod;self.degree=mod.bit_length()-1
    def mul(self,a,b):
        full=0
        for i in range(self.degree):
            if b&(1<<i):full^=a<<i
        while full.bit_length()>self.degree:
            full^=self.mod<<(full.bit_length()-self.degree-1)
        return full
    def pow(self,a,n):
        z=1
        for bit in bin(n)[2:]:
            z=self.mul(z,z)
            if bit=='1':z=self.mul(z,a)
        return z
    def inv(self,a):return self.pow(a,(1<<self.degree)-2)

def trim(a):
    a=list(a)
    while a and not a[-1]:a.pop()
    return a
def mul(a,b,K):
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):z[i+j]^=K.mul(x,y)
    return trim(z)
def rem(a,b,K):
    a=trim(a);b=trim(b);inv=K.inv(b[-1])
    while len(a)>=len(b):
        shift=len(a)-len(b);c=K.mul(a[-1],inv)
        for i,x in enumerate(b):a[shift+i]^=K.mul(c,x)
        a=trim(a)
    return a
def gcd(a,b,K):
    while b:a,b=b,rem(a,b,K)
    inv=K.inv(a[-1]);return [K.mul(x,inv) for x in a]
def qfrob(a,mod,K):
    for _ in range(K.degree):
        sq=[0]*(2*len(a)-1)
        for i,x in enumerate(a):sq[2*i]=K.mul(x,x)
        a=rem(sq,mod,K)
    return a

for fiber in r['fibers']:
    Kfield=K(fiber['field_modulus_bits'])
    assert Kfield.degree==fiber['field_degree']
    f=[0]*8
    for j,exps in terms.items():
        for e in exps:f[j]^=Kfield.pow(2,e)
    assert f==fiber['specialized_f']
    d=trim([f[i+1] if (i+1)&1 else 0 for i in range(7)])
    assert d==fiber['derivative'] and gcd(f,d,Kfield)==[1]
    product=[1]
    for part in fiber['base_factor_groups']:
        factor=part['factor'];degree=part['irreducible_y_degree']
        assert len(factor)-1==degree*part['factor_count']
        product=mul(product,factor,Kfield)
        h=[0,1]
        for _ in range(degree):h=qfrob(h,factor,Kfield)
        assert rem([(h[i] if i<len(h) else 0)^(1 if i==1 else 0) for i in range(max(len(h),2))],factor,Kfield)==[]
        for prime in (2,3,5,7):
            if degree%prime==0:
                h=[0,1]
                for _ in range(degree//prime):h=qfrob(h,factor,Kfield)
                z=[(h[i] if i<len(h) else 0)^(1 if i==1 else 0) for i in range(max(len(h),2))]
                assert gcd(factor,trim(z),Kfield)==[1]
    assert product==f
    assert sum(x['factor_count']*x['irreducible_y_degree'] for x in fiber['base_factor_groups'])==7
    assert sum(x['factor_count']*x['residue_degree_over_F2'] for x in fiber['base_factor_groups'])==fiber['weighted_fiber_degree']
print('BASE132_INDEPENDENT_FACTOR_DEGREES_PASS')
