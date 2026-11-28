#!/usr/bin/env python3
"""Exact degree-16 lambda DDF over four P11/P22 ordinary base residue fields."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def load(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod

class Tower:
    def __init__(self,base,modulus):
        self.base=base;self.modulus=modulus;self.m=len(modulus)-1
        self.d=base.degree;self.mask=(1<<self.d)-1;self.q_bits=self.d*self.m
    def pack(self,c):return sum(v<<(self.d*i) for i,v in enumerate(c))
    def unpack(self,a):return [(a>>(self.d*i))&self.mask for i in range(self.m)]
    def mul(self,a,b):
        if self.m==1:return self.base.mul(a,b)
        av=self.unpack(a);bv=self.unpack(b);z=[0]*(2*self.m-1)
        for i,x in enumerate(av):
            if x:
                for j,y in enumerate(bv):
                    if y:z[i+j]^=self.base.mul(x,y)
        for k in range(2*self.m-2,self.m-1,-1):
            c=z[k]
            if c:
                for j,v in enumerate(self.modulus[:-1]):
                    if v:z[k-self.m+j]^=self.base.mul(c,v)
        return self.pack(z[:self.m])
    def square(self,a):return self.mul(a,a)
    def power(self,a,n):
        z=1
        while n:
            if n&1:z=self.mul(z,a)
            a=self.square(a);n>>=1
        return z
    def inv(self,a):
        assert a
        return self.power(a,(1<<self.q_bits)-2)

def main(dest):
    p=json.loads((HERE/'protocol.json').read_text())
    paths={'r130_excerpt':ROUNDS/'130/source-data/triangular-f-g.sing',
           'r105_parser':ROUNDS/'105/jets105.py',
           'r132_result':ROUNDS/'132/result.json',
           'r133_result':ROUNDS/'133/result.json',
           'r132_arithmetic':ROUNDS/'132/base132.py',
           'r130_polynomials':ROUNDS/'130/lambda130.py'}
    for key,path in paths.items():assert sha(path)==p['input_sha256'][key],key
    parser=load(paths['r105_parser'],'parser105_pinned')
    base=load(paths['r132_arithmetic'],'base132_pinned')
    poly=load(paths['r130_polynomials'],'poly130_pinned')
    _,gterms=parser.parse_source(paths['r130_excerpt'].read_text())
    old=json.loads(paths['r132_result'].read_text())
    support=json.loads(paths['r133_result'].read_text())
    out={'protocol_id':p['id'],'input_sha256':p['input_sha256'],'fibers':[]}
    for fiber,deriv in zip(old['fibers'],support['fibers']):
        assert fiber['x_factor']==deriv['x_factor']
        B=base.Field(fiber['field_modulus_bits'])
        for part in deriv['groups']:
            ymod=part['nonzero_derivative_factor']
            m=len(ymod)-1
            if not m:continue
            assert m==part['base_irreducible_y_degree']*part['base_factor_count']-part['zero_derivative_y_degree']
            # R133 separates the two P11 linear roots; all included factors are irreducible.
            assert part['base_factor_count']==1 or m==1
            K=Tower(B,ymod)
            y=ymod[0] if m==1 else 1<<B.degree
            coefficients=[0]*17
            for le,ye,den,exps in gterms:
                value=0
                for exponent in exps:value^=B.power(2,exponent)
                value=B.mul(value,B.inv(B.power(2,den)))
                coefficients[le]^=K.mul(value,K.power(y,ye))
            assert coefficients[16]==1 and coefficients[1]!=0
            factors=poly.ddf(coefficients,K)
            converted=[{'lambda_degree':item['irreducible_degree'],
                        'count':item['factor_count'],
                        'residue_degree_over_F2':K.q_bits*item['irreducible_degree'],
                        'factor':item['factor']} for item in factors]
            out['fibers'].append({'x_factor':fiber['x_factor'],
                                  'base_residue_degree_over_F2':K.q_bits,
                                  'y_factor':ymod,
                                  'y_residue':y,
                                  'G_coefficients':coefficients,
                                  'lambda_derivative':coefficients[1],
                                  'degree_parts':converted,
                                  'lambda_degree_check':sum(item['lambda_degree']*item['count'] for item in converted)})
            assert out['fibers'][-1]['lambda_degree_check']==16
            print('FIBER',fiber['x_factor'],'base_degree',K.q_bits,'parts',[(z['lambda_degree'],z['count']) for z in converted],flush=True)
    out['ordinary_weighted_degree']=sum(z['residue_degree_over_F2']*z['count'] for f in out['fibers'] for z in f['degree_parts'])
    assert out['ordinary_weighted_degree']==3168 and len(out['fibers'])==4
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
    print('LAMBDA134_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
