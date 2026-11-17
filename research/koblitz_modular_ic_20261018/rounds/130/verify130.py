#!/usr/bin/env python3
"""Independent factor-degree certificates for the three degree-7 lambda fibers."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
p=json.loads((HERE/'protocol.json').read_text())
r=json.loads((HERE/'result.json').read_text())
paths={'source_excerpt':HERE/'source-data/triangular-f-g.sing',
       'r105_parser':ROUNDS/'105/jets105.py','r127_result':ROUNDS/'127/result.json',
       'r129_result':ROUNDS/'129/result.json'}
for name,path in paths.items():assert hashlib.sha256(path.read_bytes()).hexdigest()==p['input_sha256'][name]
original=ROUNDS/'51/runs/base23-struct51-03/marked23-triangular51.sing'
if original.exists():
    raw=original.read_bytes()
    assert hashlib.sha256(raw).hexdigest()==p['full_r51_donor_sha256']
    assignments=[]
    for name in (b'f51',b'G51'):
        start=raw.index(b'poly '+name+b'=');end=raw.index(b';',start)+1
        assignments.append(raw[start:end])
    assert paths['source_excerpt'].read_bytes()==b''.join(assignments)+b'\n'
spec=importlib.util.spec_from_file_location('jets105_verify',paths['r105_parser'])
parser=importlib.util.module_from_spec(spec);spec.loader.exec_module(parser)
_,gterms=parser.parse_source(paths['source_excerpt'].read_text())

def gf128mul(a,b):
    z=0
    for i in range(7):
        if b&(1<<i):z^=a<<i
    while z.bit_length()>7:z^=203<<(z.bit_length()-8)
    return z

class K:
    def __init__(self,degree):self.degree=degree;self.bits=7*degree
    def mul(self,a,b):
        if self.degree==1:return gf128mul(a,b)
        left=[(a>>(7*i))&127 for i in range(4)]
        right=[(b>>(7*i))&127 for i in range(4)]
        c=[0]*7
        for i,x in enumerate(left):
            for j,y in enumerate(right):c[i+j]^=gf128mul(x,y)
        for k in range(6,3,-1):
            v=c[k]
            for j,q in enumerate((25,38,83,66)):c[k-4+j]^=gf128mul(v,q)
        return sum(c[i]<<(7*i) for i in range(4))
    def power(self,a,n):
        v=1
        for bit in bin(n)[2:]:
            v=self.mul(v,v)
            if bit=='1':v=self.mul(v,a)
        return v
    def inverse(self,a):return self.power(a,(1<<self.bits)-2)

def trim(a):
    a=list(a)
    while a and a[-1]==0:a.pop()
    return a
def add(a,b):
    out=[0]*max(len(a),len(b))
    for i,x in enumerate(a):out[i]^=x
    for i,x in enumerate(b):out[i]^=x
    return trim(out)
def multiply(a,b,K):
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):out[i+j]^=K.mul(x,y)
    return trim(out)
def remainder(a,b,K):
    a=trim(a);b=trim(b);inverse=K.inverse(b[-1])
    while len(a)>=len(b):
        k=len(a)-len(b);v=K.mul(a[-1],inverse)
        for i,x in enumerate(b):a[k+i]^=K.mul(v,x)
        a=trim(a)
    return a
def gcd(a,b,K):
    while b:a,b=b,remainder(a,b,K)
    if not a:return []
    inv=K.inverse(a[-1]);return [K.mul(x,inv) for x in a]
def qfrob(a,mod,K):
    for _ in range(K.bits):
        sq=[0]*(2*len(a)-1)
        for i,x in enumerate(a):sq[2*i]=K.mul(x,x)
        a=remainder(sq,mod,K)
    return a

for fiber in r['fibers']:
    Kfield=K(4 if fiber['field_degree_over_F2']==28 else 1)
    y=fiber['y_residue']
    coeff=[0]*17
    x=2
    for le,ye,den,exps in gterms:
        numerator=0
        for e in exps:numerator^=Kfield.power(x,e)
        value=Kfield.mul(numerator,Kfield.inverse(Kfield.power(x,den)))
        coeff[le]^=Kfield.mul(value,Kfield.power(y,ye))
    assert coeff==fiber['G_coefficients'] and coeff[1]==fiber['lambda_derivative']!=0
    product=[1]
    for part in fiber['degree_parts']:
        factor=part['factor'];d=part['lambda_degree']
        assert factor[-1]==1 and len(factor)-1==d*part['count']
        product=multiply(product,factor,Kfield)
        h=[0,1]
        for _ in range(d):h=qfrob(h,factor,Kfield)
        assert add(h,[0,1])==[]
        for prime in (2,3,5,7):
            if d%prime==0:
                h=[0,1]
                for _ in range(d//prime):h=qfrob(h,factor,Kfield)
                assert gcd(factor,add(h,[0,1]),Kfield)==[1]
    assert product==coeff
    assert sum(x['count']*x['lambda_degree'] for x in fiber['degree_parts'])==16
assert r['x_fiber_degree_check']==784
print('LAMBDA130_INDEPENDENT_FACTOR_DEGREES_PASS')
