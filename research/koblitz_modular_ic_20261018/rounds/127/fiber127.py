#!/usr/bin/env python3
"""Exact F128 factor support of the marked23 degree-seven base fiber."""
import hashlib
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROUNDS = HERE.parent


def gf_mul(a, b, modulus=203):
    z = 0
    while b:
        if b & 1: z ^= a
        b >>= 1
        a <<= 1
        if a & 128: a ^= modulus
    return z


def gf_pow(a, n):
    out = 1
    while n:
        if n & 1: out = gf_mul(out, a)
        a = gf_mul(a, a)
        n >>= 1
    return out


def trim(p):
    p = list(p)
    while p and p[-1] == 0: p.pop()
    return p


def mul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b): out[i+j] ^= gf_mul(x, y)
    return trim(out)


def divmod_poly(a, b):
    a = trim(a); b = trim(b)
    assert b
    q = [0] * max(0, len(a)-len(b)+1)
    inv = gf_pow(b[-1], 126)
    while len(a) >= len(b):
        shift = len(a)-len(b)
        c = gf_mul(a[-1], inv)
        q[shift] = c
        for j, x in enumerate(b): a[shift+j] ^= gf_mul(c, x)
        a = trim(a)
    return trim(q), a


def monic(a):
    a = trim(a)
    if not a: return []
    inv = gf_pow(a[-1], 126)
    return [gf_mul(x, inv) for x in a]


def gcd(a, b):
    while b: a, b = b, divmod_poly(a, b)[1]
    return monic(a)


def derivative(a):
    return trim([a[i+1] if (i+1)&1 else 0 for i in range(len(a)-1)])


def sqrt_poly(a):
    assert all(a[i] == 0 for i in range(1, len(a), 2))
    return trim([gf_pow(a[i], 64) for i in range(0, len(a), 2)])


def quotient(a, b):
    q, r = divmod_poly(a, b)
    assert not r
    return q


def squarefree(a, multiplier=1):
    if len(a) <= 1: return []
    d = derivative(a)
    if not d: return squarefree(sqrt_poly(a), 2*multiplier)
    c = gcd(a, d)
    w = quotient(a, c)
    i = 1; out = []
    while len(w) > 1:
        y = gcd(w, c)
        z = quotient(w, y)
        if len(z) > 1: out.append((z, i*multiplier))
        w = y; c = quotient(c, y); i += 1
    if len(c) > 1: out += squarefree(sqrt_poly(c), 2*multiplier)
    return out


def modpow(a, n, modulus):
    result = [1]
    while n:
        if n & 1: result = divmod_poly(mul(result, a), modulus)[1]
        a = divmod_poly(mul(a, a), modulus)[1]
        n >>= 1
    return result


def ddf(squarefree_factor):
    remaining = squarefree_factor[:]
    parts = []
    for d in range(1, len(squarefree_factor)):
        if len(remaining) <= 1: break
        z = [0, 1]
        for _ in range(d): z = modpow(z, 128, remaining)
        z += [0] * max(0, 2-len(z))
        z[1] ^= 1
        factor = gcd(remaining, trim(z))
        if len(factor)>1:
            assert (len(factor)-1) % d == 0
            parts.append({'irreducible_degree':d,'factor_count':(len(factor)-1)//d,'factor':factor})
            remaining = quotient(remaining, factor)
    assert len(remaining)==1
    return parts


def gf2_mod(a, b):
    while a and a.bit_length()>=b.bit_length(): a ^= b << (a.bit_length()-b.bit_length())
    return a


def gf2_irreducible(p):
    x = 2; z = x
    for i in range(1,8):
        z = gf2_mod(gf2_mul(z,z),p)
        if i <= 3 and gf2_gcd(z^x,p)!=1: return False
    return z==x


def gf2_mul(a,b):
    z=0
    while b:
        if b&1:z^=a
        a<<=1;b>>=1
    return z


def gf2_gcd(a,b):
    while b:a,b=b,gf2_mod(a,b)
    return a


def parse_source(raw):
    match=re.findall(r'poly f23=([^;]+);',raw)
    assert len(match)==1
    # Coefficient expressions contain only sums of x-monomials; split the
    # eight outer y-terms at + outside parentheses.
    parts=[];depth=0;start=0
    for i,c in enumerate(match[0]):
        if c=='(':depth+=1
        elif c==')':depth-=1
        elif c=='+' and depth==0:
            parts.append(match[0][start:i]);start=i+1
    parts.append(match[0][start:])
    assert depth==0 and len(parts)==8
    out={}
    for part in parts:
        found=re.fullmatch(r'(?:\(([^()]*)\)\*)?y(?:\^([1-9][0-9]*))?',part)
        if found: coeff,ye=found.group(1) or '1',int(found.group(2) or 1)
        else:
            found=re.fullmatch(r'\(([^()]*)\)',part)
            assert found
            coeff,ye=found.group(1),0
        exps=[]
        for atom in coeff.split('+'):
            m=re.fullmatch(r'1|x(?:\^([1-9][0-9]*))?',atom)
            assert m,atom
            exps.append(0 if atom=='1' else int(m.group(1) or 1))
        assert ye not in out and len(exps)==len(set(exps))
        out[ye]=exps
    assert set(out)==set(range(8)) and out[7]==[0]
    return out


def main(dest):
    protocol=json.loads((HERE/'protocol.json').read_text())
    source=ROUNDS.parent / protocol['source_path']
    raw=source.read_bytes()
    assert hashlib.sha256(raw).hexdigest()==protocol['source_sha256']
    p=protocol['field_polynomial_bits']
    assert p==203 and gf2_irreducible(p)
    fterms=parse_source(raw.decode('ascii'))
    f=[0]*8
    for ye,exps in fterms.items():
        for e in exps:f[ye]^=gf_pow(2,e)
    assert f[-1]==1
    derivative_f=derivative(f)
    common=gcd(f,derivative_f)
    factors=squarefree(f)
    parts=[];product=[1]
    for factor,multiplicity in factors:
        distinct=ddf(factor)
        parts.append({'multiplicity':multiplicity,'squarefree_factor':factor,'degree_parts':distinct})
        for _ in range(multiplicity):product=mul(product,factor)
    assert product==f and sum((len(factor)-1)*m for factor,m in factors)==7
    roots=[]
    for a in range(128):
        multiplicity=0;work=f[:]
        while len(work)>1:
            q,r=divmod_poly(work,[a,1])
            if r:break
            multiplicity+=1;work=q
        if multiplicity:roots.append([a,multiplicity])
    result={'protocol_id':protocol['id'],'source_sha256':protocol['source_sha256'],
            'field_polynomial_bits':p,'field_generator':2,'specialized_f':f,
            'derivative':derivative_f,'gcd_f_derivative':common,
            'squarefree_parts':parts,'rational_roots':roots,
            'degree_check':sum((len(factor)-1)*m for factor,m in factors)}
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('FIBER127_ALL_CHECKS_PASS')


if __name__=='__main__':main(Path(sys.argv[1]))
