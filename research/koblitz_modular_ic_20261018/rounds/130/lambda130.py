#!/usr/bin/env python3
"""Degree-7 marked23 lambda-fiber distinct-degree census over F128 and F2^28."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

class Field:
    def __init__(self,degree,base,q4):
        self.degree=degree;self.base=base;self.q4=q4;self.q_bits=7*degree
    def mul(self,a,b):
        if self.degree==1:return self.base.gf_mul(a,b)
        aa=[(a>>(7*i))&127 for i in range(4)]
        bb=[(b>>(7*i))&127 for i in range(4)]
        tmp=[0]*7
        for i,x in enumerate(aa):
            if x:
                for j,y in enumerate(bb):
                    if y:tmp[i+j]^=self.base.gf_mul(x,y)
        for k in range(6,3,-1):
            c=tmp[k]
            if c:
                for i in range(4):tmp[k-4+i]^=self.base.gf_mul(c,self.q4[i])
        return sum(tmp[i]<<(7*i) for i in range(4))
    def power(self,a,n):
        out=1
        while n:
            if n&1:out=self.mul(out,a)
            a=self.mul(a,a);n>>=1
        return out
    def inv(self,a):
        assert a
        return self.power(a,(1<<self.q_bits)-2)

def trim(a):
    a=list(a)
    while a and a[-1]==0:a.pop()
    return a

def add(a,b):
    out=[0]*max(len(a),len(b))
    for i,x in enumerate(a):out[i]^=x
    for i,x in enumerate(b):out[i]^=x
    return trim(out)

def mul(a,b,F):
    if not a or not b:return []
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        if x:
            for j,y in enumerate(b):
                if y:out[i+j]^=F.mul(x,y)
    return trim(out)

def divmod_poly(a,b,F):
    a=trim(a);b=trim(b)
    assert b
    q=[0]*max(0,len(a)-len(b)+1)
    inv=F.inv(b[-1])
    while len(a)>=len(b):
        offset=len(a)-len(b);c=F.mul(a[-1],inv);q[offset]=c
        for j,x in enumerate(b):a[offset+j]^=F.mul(c,x)
        a=trim(a)
    return trim(q),a

def monic(a,F):
    if not a:return []
    inverse=F.inv(a[-1])
    return [F.mul(x,inverse) for x in a]

def gcd(a,b,F):
    while b:a,b=b,divmod_poly(a,b,F)[1]
    return monic(a,F)

def square_mod(a,modulus,F):
    if not a:return []
    out=[0]*(2*len(a)-1)
    for i,x in enumerate(a):out[2*i]=F.mul(x,x)
    return divmod_poly(out,modulus,F)[1]

def frobenius_q(a,modulus,F):
    for _ in range(F.q_bits):a=square_mod(a,modulus,F)
    return a

def ddf(g,F):
    remaining=g[:];h=[0,1];parts=[]
    for d in range(1,17):
        if len(remaining)==1:break
        h=frobenius_q(h,remaining,F)
        factor=gcd(remaining,add(h,[0,1]),F)
        if len(factor)>1:
            assert (len(factor)-1)%d==0
            parts.append({'irreducible_degree':d,'factor_count':(len(factor)-1)//d,'factor':factor})
            quotient,remainder=divmod_poly(remaining,factor,F)
            assert not remainder
            remaining=quotient
            if len(remaining)>1:h=divmod_poly(h,remaining,F)[1]
    assert remaining==[1]
    product=[1]
    for item in parts:product=mul(product,item['factor'],F)
    assert product==g and sum(p['irreducible_degree']*p['factor_count'] for p in parts)==16
    return parts

def main(dest):
    protocol=json.loads((HERE/'protocol.json').read_text())
    paths={'source_excerpt':HERE/'source-data/triangular-f-g.sing',
           'r105_parser':ROUNDS/'105/jets105.py','r127_result':ROUNDS/'127/result.json',
           'r129_result':ROUNDS/'129/result.json'}
    for name,path in paths.items():assert sha(path)==protocol['input_sha256'][name],name
    spec=importlib.util.spec_from_file_location('jets105_pinned',paths['r105_parser'])
    parser=importlib.util.module_from_spec(spec);spec.loader.exec_module(parser)
    fterms,gterms=parser.parse_source(paths['source_excerpt'].read_text())
    assert len(fterms)==8 and len(gterms)==29
    old=json.loads(paths['r127_result'].read_text())
    q4=old['squarefree_parts'][0]['degree_parts'][1]['factor']
    assert q4==[25,38,83,66,1]
    base_spec=importlib.util.spec_from_file_location('f127_pinned',ROUNDS/'127/fiber127.py')
    base=importlib.util.module_from_spec(base_spec);base_spec.loader.exec_module(base)
    r129=json.loads(paths['r129_result'].read_text())
    result={'protocol_id':protocol['id'],'input_sha256':protocol['input_sha256'],
            'field_polynomial_bits':203,'quartic_over_F128':q4,'fibers':[]}
    for place,y in zip(r129['base_places'],protocol['base_residue_roots']):
        degree=4 if y==128 else 1
        F=Field(degree,base,q4)
        coeff=[0]*17
        for le,ye,den,exps in gterms:
            xvalue=0
            for e in exps:xvalue^=base.gf_pow(2,(e-den)%127)
            coeff[le]^=F.mul(xvalue,F.power(y,ye))
        assert coeff[16]==1 and coeff[1]!=0
        parts=ddf(coeff,F)
        factors=[{'lambda_degree':item['irreducible_degree'],
                  'count':item['factor_count'],
                  'residue_degree_over_F2':place['residue_degree_over_F2']*item['irreducible_degree'],
                  'ramification_index':place['ramification_index'],
                  'factor':item['factor']} for item in parts]
        result['fibers'].append({'base_factor':place['source_factor'],
                                 'base_residue_degree_over_F2':place['residue_degree_over_F2'],
                                 'base_ramification_index':place['ramification_index'],
                                 'field_degree_over_F2':F.q_bits,'y_residue':y,
                                 'G_coefficients':coeff,'lambda_derivative':coeff[1],
                                 'degree_parts':factors,'lambda_degree_check':16})
    result['x_fiber_degree_check']=sum(
        part['count']*part['residue_degree_over_F2']*part['ramification_index']
        for fiber in result['fibers'] for part in fiber['degree_parts'])
    assert result['x_fiber_degree_check']==784
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('LAMBDA130_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
