#!/usr/bin/env python3
"""Independent polynomial reduction and finite-field check of R127."""
import hashlib
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent.parent
p=json.loads((HERE/'protocol.json').read_text())
r=json.loads((HERE/'result.json').read_text())
source=ROOT/p['source_path']
raw=source.read_bytes()
assert hashlib.sha256(raw).hexdigest()==p['source_sha256']

def red(a):
    while a.bit_length()>7:
        a^=203 << (a.bit_length()-8)
    return a

def times(a,b):
    full=0
    for i in range(7):
        if (b>>i)&1: full^=a<<i
    return red(full)

def pow128(a,k):
    z=1
    for _ in range(k):z=times(z,a)
    return z

def trim(a):
    a=list(a)
    while a and a[-1]==0:a.pop()
    return a

def product(a,b):
    z=[0]*(len(a)+len(b)-1)
    for i,u in enumerate(a):
        for j,v in enumerate(b):z[i+j]^=times(u,v)
    return trim(z)

def rem(a,b):
    a=trim(a);b=trim(b)
    inv=pow128(b[-1],126)
    while len(a)>=len(b):
        off=len(a)-len(b);c=times(a[-1],inv)
        for j,v in enumerate(b):a[off+j]^=times(c,v)
        a=trim(a)
    return a

def gcd(a,b):
    while b:a,b=b,rem(a,b)
    if not a:return []
    inv=pow128(a[-1],126)
    return [times(x,inv) for x in a]

def modpow(a,k,p):
    z=[1]
    for bit in bin(k)[2:]:
        z=rem(product(z,z),p)
        if bit=='1':z=rem(product(z,a),p)
    return z

text=raw.decode('ascii')
f=re.search(r'poly f23=(.*?);',text).group(1)
parts=re.split(r'\+(?=(?:[^()]*\([^()]*\))*[^()]*$)',f)
assert len(parts)==8
coeff=[None]*8
for part in parts:
    if '*y' in part:
        numerator,yr=part.split('*y')
        e=int(yr[1:]) if yr.startswith('^') else 1
    elif part.startswith('y'):
        numerator='(1)';e=int(part[2:]) if part.startswith('y^') else 1
    else:numerator=part;e=0
    numerator=numerator.strip('()')
    value=0
    for term in numerator.split('+'):
        if term=='1':xe=0
        elif term=='x':xe=1
        else:
            assert term.startswith('x^')
            xe=int(term[2:])
        value^=pow128(2,xe)
    coeff[e]=value
assert coeff==r['specialized_f']
linear_a=[123,1];linear_b=[90,1]
quartic=[25,38,83,66,1]
assert product(product(linear_a,product(linear_b,linear_b)),quartic)==coeff
assert rem(coeff,linear_a)==[] and rem(coeff,linear_b)==[]
assert gcd(quartic,[0,1])==[1]
roots=[a for a in range(128) if rem(quartic,[a,1])==[]]
assert roots==[]
frob=modpow([0,1],128*128,quartic)
assert gcd(quartic,trim([frob[i]^(1 if i==1 else 0) for i in range(max(len(frob),2))]))==[1]
assert modpow([0,1],128**4,quartic)==[0,1]
assert r['rational_roots']==[[90,2],[123,1]]
assert r['gcd_f_derivative']==[118,0,1]
assert r['degree_check']==7
print('FIBER127_INDEPENDENT_PASS')
