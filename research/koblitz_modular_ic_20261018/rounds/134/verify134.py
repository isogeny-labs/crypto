#!/usr/bin/env python3
"""Independent carryless tower and finite-field factor-degree audit for R134."""
import hashlib
import importlib.util
import json
from pathlib import Path

here=Path(__file__).resolve().parent; rounds=here.parent
p=json.loads((here/'protocol.json').read_text());r=json.loads((here/'result.json').read_text())
paths={'r130_excerpt':rounds/'130/source-data/triangular-f-g.sing',
       'r105_parser':rounds/'105/jets105.py',
       'r132_result':rounds/'132/result.json',
       'r133_result':rounds/'133/result.json',
       'r132_arithmetic':rounds/'132/base132.py',
       'r130_polynomials':rounds/'130/lambda130.py'}
for name,path in paths.items():assert hashlib.sha256(path.read_bytes()).hexdigest()==p['input_sha256'][name]
spec=importlib.util.spec_from_file_location('parser105_verify',paths['r105_parser'])
parser=importlib.util.module_from_spec(spec);spec.loader.exec_module(parser)
_,terms=parser.parse_source(paths['r130_excerpt'].read_text())
old=json.loads(paths['r132_result'].read_text())
support=json.loads(paths['r133_result'].read_text())

class K:
    def __init__(self,xmod,ymod):
        self.xmod=xmod;self.ymod=ymod;self.d=xmod.bit_length()-1
        self.m=len(ymod)-1;self.q_bits=self.d*self.m;self.mask=(1<<self.d)-1
    def base_mul(self,a,b):
        z=0
        for i in range(self.d):
            if b&(1<<i):z^=a<<i
        while z.bit_length()>self.d:z^=self.xmod<<(z.bit_length()-self.d-1)
        return z
    def mul(self,a,b):
        if self.m==1:return self.base_mul(a,b)
        aa=[(a>>(self.d*i))&self.mask for i in range(self.m)]
        bb=[(b>>(self.d*i))&self.mask for i in range(self.m)]
        z=[0]*(2*self.m-1)
        for i,x in enumerate(aa):
            for j,y in enumerate(bb):z[i+j]^=self.base_mul(x,y)
        for k in range(2*self.m-2,self.m-1,-1):
            for j,v in enumerate(self.ymod[:-1]):
                z[k-self.m+j]^=self.base_mul(z[k],v)
        return sum(z[i]<<(self.d*i) for i in range(self.m))
    def power(self,a,n):
        z=1
        for bit in bin(n)[2:]:
            z=self.mul(z,z)
            if bit=='1':z=self.mul(z,a)
        return z
    def inv(self,a):return self.power(a,(1<<self.q_bits)-2)

def trim(a):
    a=list(a)
    while a and a[-1]==0:a.pop()
    return a
def multiply(a,b,K):
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
    for _ in range(K.q_bits):
        sq=[0]*(2*len(a)-1)
        for i,x in enumerate(a):sq[2*i]=K.mul(x,x)
        a=rem(sq,mod,K)
    return a
def xor_x(a):
    z=list(a)+[0]*max(0,2-len(a));z[1]^=1
    return trim(z)

expected=[('P11',11),('P11',55),('P22',44),('P22',88)]
assert [(f['x_factor'],f['base_residue_degree_over_F2']) for f in r['fibers']]==expected
for fiber in r['fibers']:
    oldfiber=next(z for z in old['fibers'] if z['x_factor']==fiber['x_factor'])
    deriv=next(z for z in support['fibers'] if z['x_factor']==fiber['x_factor'])
    assert fiber['y_factor'] in [g['nonzero_derivative_factor'] for g in deriv['groups']]
    Kfield=K(oldfiber['field_modulus_bits'],fiber['y_factor'])
    assert Kfield.q_bits==fiber['base_residue_degree_over_F2']
    y=fiber['y_factor'][0] if Kfield.m==1 else 1<<Kfield.d
    assert y==fiber['y_residue']
    coeff=[0]*17
    for le,ye,den,exps in terms:
        num=0
        for e in exps:num^=Kfield.power(2,e)
        value=Kfield.mul(num,Kfield.power(2,(-den)%((1<<Kfield.d)-1)))
        coeff[le]^=Kfield.mul(value,Kfield.power(y,ye))
    assert coeff==fiber['G_coefficients'] and coeff[1]==fiber['lambda_derivative']!=0
    assert gcd(coeff,[coeff[1]],Kfield)==[1]
    product=[1]
    for part in fiber['degree_parts']:
        f=part['factor'];degree=part['lambda_degree'];count=part['count']
        assert len(f)-1==degree*count
        product=multiply(product,f,Kfield)
        h=[0,1]
        for _ in range(degree):h=qfrob(h,f,Kfield)
        assert rem(xor_x(h),f,Kfield)==[]
        for prime in (2,3,5,7):
            if degree%prime==0:
                h=[0,1]
                for _ in range(degree//prime):h=qfrob(h,f,Kfield)
                assert gcd(f,xor_x(h),Kfield)==[1]
        assert part['residue_degree_over_F2']==Kfield.q_bits*degree
    assert product==coeff and sum(z['lambda_degree']*z['count'] for z in fiber['degree_parts'])==16
assert sum(z['count']*z['residue_degree_over_F2'] for f in r['fibers'] for z in f['degree_parts'])==3168
print('LAMBDA134_INDEPENDENT_FACTOR_DEGREES_PASS')
