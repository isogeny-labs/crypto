#!/usr/bin/env python3
"""Exact degree-11 and degree-22 marked23 base-fiber factor census."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module

class Field:
    def __init__(self,modulus):self.modulus=modulus;self.degree=modulus.bit_length()-1;self.mask=(1<<self.degree)-1
    def mul(self,a,b):
        z=0
        while b:
            if b&1:z^=a
            b>>=1;a<<=1
            if a&(1<<self.degree):a^=self.modulus
        return z
    def power(self,a,n):
        z=1
        while n:
            if n&1:z=self.mul(z,a)
            a=self.mul(a,a);n>>=1
        return z
    def inv(self,a):assert a;return self.power(a,(1<<self.degree)-2)

def trim(a):
    a=list(a)
    while a and a[-1]==0:a.pop()
    return a
def add(a,b):
    z=[0]*max(len(a),len(b))
    for i,x in enumerate(a):z[i]^=x
    for i,x in enumerate(b):z[i]^=x
    return trim(z)
def mul(a,b,F):
    if not a or not b:return []
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        if x:
            for j,y in enumerate(b):
                if y:z[i+j]^=F.mul(x,y)
    return trim(z)
def divmod_poly(a,b,F):
    a=trim(a);b=trim(b);assert b
    q=[0]*max(0,len(a)-len(b)+1);inv=F.inv(b[-1])
    while len(a)>=len(b):
        k=len(a)-len(b);c=F.mul(a[-1],inv);q[k]=c
        for i,x in enumerate(b):a[k+i]^=F.mul(c,x)
        a=trim(a)
    return trim(q),a
def gcd(a,b,F):
    while b:a,b=b,divmod_poly(a,b,F)[1]
    if not a:return []
    inverse=F.inv(a[-1]);return [F.mul(x,inverse) for x in a]
def q_frobenius(a,modulus,F):
    for _ in range(F.degree):
        square=[0]*(2*len(a)-1)
        for i,x in enumerate(a):square[2*i]=F.mul(x,x)
        a=divmod_poly(square,modulus,F)[1]
    return a
def ddf(f,F):
    remaining=f[:];h=[0,1];parts=[]
    for degree in range(1,8):
        if len(remaining)==1:break
        h=q_frobenius(h,remaining,F)
        factor=gcd(remaining,add(h,[0,1]),F)
        if len(factor)>1:
            assert (len(factor)-1)%degree==0
            count=(len(factor)-1)//degree
            parts.append({'irreducible_y_degree':degree,'factor_count':count,'factor':factor,
                          'residue_degree_over_F2':F.degree*degree,'ramification_index':1})
            remaining,remainder=divmod_poly(remaining,factor,F)
            assert not remainder
            if len(remaining)>1:h=divmod_poly(h,remaining,F)[1]
    assert remaining==[1]
    product=[1]
    for item in parts:product=mul(product,item['factor'],F)
    assert product==f and sum(x['irreducible_y_degree']*x['factor_count'] for x in parts)==7
    return parts

def main(dest):
    p=json.loads((HERE/'protocol.json').read_text())
    paths={'bad_factors':HERE/'source-data/bad-factors.txt',
           'r51_source':ROUNDS/'51/source-data/marked23-field51.sing',
           'r100_verifier':ROUNDS/'100/verify100.py','r127_parser':ROUNDS/'127/fiber127.py'}
    for name,path in paths.items():assert sha(path)==p['input_sha256'][name],name
    verify100=load(paths['r100_verifier'],'r100_parse_pinned')
    parse127=load(paths['r127_parser'],'r127_parse_pinned')
    fterms=parse127.parse_source(paths['r51_source'].read_text())
    factor_lines=paths['bad_factors'].read_text().splitlines()
    assert len(factor_lines)==2
    result={'protocol_id':p['id'],'input_sha256':p['input_sha256'],'fibers':[]}
    for degree,line in zip(p['field_degrees'],factor_lines):
        label,expression=line.split('=',1)
        assert label=='P'+str(degree)
        modulus=verify100.parse(expression)
        assert modulus.bit_length()-1==degree and verify100.irreducible(modulus)
        F=Field(modulus)
        f=[0]*8
        for ye,exps in fterms.items():
            for e in exps:f[ye]^=F.power(2,e)
        assert f[-1]==1
        deriv=trim([f[i+1] if (i+1)&1 else 0 for i in range(7)])
        assert gcd(f,deriv,F)==[1]
        parts=ddf(f,F)
        weighted=sum(item['residue_degree_over_F2']*item['factor_count'] for item in parts)
        assert weighted==degree*7
        result['fibers'].append({'x_factor':label,'x_polynomial':expression,
            'field_modulus_bits':modulus,'field_degree':degree,'specialized_f':f,
            'derivative':deriv,'base_factor_groups':parts,
            'base_place_count':sum(item['factor_count'] for item in parts),
            'weighted_fiber_degree':weighted})
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('BASE132_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
