#!/usr/bin/env python3
"""Source-bound one-input exceptional-dispatch witness; no performance claim."""
import csv
import hashlib
import re
import argparse
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
parser = argparse.ArgumentParser()
parser.add_argument('--author-source', required=True, type=Path)
parser.add_argument('--check', action='store_true')
args = parser.parse_args()
A = args.author_source
R = ROOT / 'src/bin/projective_law_gate.rs'
OUT = HERE / 'ROWS.csv'
HASHES = {
    A: '7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a',
    R: '5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0',
}
for path, want in HASHES.items():
    got = hashlib.sha256(path.read_bytes()).hexdigest()
    assert got == want, (path, got, want)
a, r = A.read_text(), R.read_text()

def demand(pattern, src, label):
    assert re.search(pattern, src, re.S), label

demand(r'if Q1ZZ2 eq Q2ZZ1 and R1ZZZZ2 eq R2ZZZZ1 then\s*if S1ZZZ2W2 eq -S2ZZZ1W1 and T1ZZZZZ2W2 eq -T2ZZZZZ1W1 then\s*return 1,__,0,__,0,1,0;\s*elif S1ZZZ2W2 eq S2ZZZ1W1 and T1ZZZZZ2W2 eq T2ZZZZZ1W1 then\s*return DBL2', a, 'author public inverse before double')
direct=a.split('DBL2B0:=function(',1)[1].split('end function;',1)[0]
for exact in ['X2:=-T1; XP1:=-(Q1*S1+X2)*S1; YP1:=(S1*T1+XP1)*S1^4; ZP1:=S1*Z1; WP1:=W1;', 'Q3:=-2*XP1; R3:=XXP1;', 'Z3:=ZP1; W3:=2*YP1;', 'return Q3,R3,S3,T3,Z3,W3,2;']:
    assert exact in direct, exact
demand(r'if q1c == q2c && r1c == r2c \{\s*if s1c == f\.neg\(s2c\) && t1c == f\.neg\(t2c\) \{\s*return \(Some\(ResultPoint::Identity\), "same_u_inverse"\);\s*\}\s*if s1c == s2c && t1c == t2c', r, 'Rust public inverse before double')
demand(r'if b == 0 \{\s*if s == 0 \{\s*return \(ResultPoint::Identity, "b_zero_identity"\);', r, 'Rust direct B0/s0 identity')

p=17
f=[2,1,0,0,0,1]
u=[5,6,1]

def trim(v):
    while v and v[-1] == 0: v.pop()
    return v

def modpoly(v,d):
    v=trim([x%p for x in v]); d=trim([x%p for x in d])
    assert d
    while len(v)>=len(d):
        c=v[-1]*pow(d[-1],-1,p)%p
        off=len(v)-len(d)
        for i,x in enumerate(d): v[off+i]=(v[off+i]-c*x)%p
        trim(v)
    return v

def gcd(v,d):
    while d: v,d=d,modpoly(v,d)
    return [x*pow(v[-1],-1,p)%p for x in v]

def ev(v,x):
    return sum(c*pow(x,i,p) for i,c in enumerate(v))%p

assert gcd(f,[1,0,0,0,5]) == [1], 'curve not smooth'
assert modpoly(f,u)==[], 'u does not divide f'
assert [x for x in range(p) if ev(f,x)==0 and ev(u,x)==0]==[12,16]
# Unit weights; q=6,r=5,s=t=0. Author public inverse test is true,
# preceding its doubling call. Direct DBL2B0 is evaluated separately.
q,r0,s,t,z,w=6,5,0,0,1,1
same_support = True
inverse = (s == -s%p and t == -t%p)
assert same_support and inverse
b=2*((q*s-t)*t-r0*s*s)%p
assert b==0
x2=-t%p
xp1=-(q*s+x2)*s%p
yp1=(s*t+xp1)*pow(s,4,p)%p
zp1=s*z%p
wp1=w
q3=-2*xp1%p; r3=xp1*xp1%p
s3=0; t3=0 # every ADD11DBL term contains XP1, ZP1, or both
z3=zp1; w3=2*yp1%p
author_direct=[q3,r3,s3,t3,z3,w3]
assert author_direct == [0]*6
row={
 'schema':'projective-law-f17-torsion-dispatch/v1',
 'p':p,'curve':'x^5+x+2','u':'x^2+6x+5','v':'0',
 'u_roots':'12;16','smooth':True,'valid_reduced':True,
 'author_public_branch':'inverse_before_double',
 'author_public_result':'identity',
 'author_direct_branch':'DBL2B0',
 'author_direct_raw':','.join(map(str,author_direct)),
 'author_direct_valid_weighted':False,
 'rust_public_branch':'same_u_inverse',
 'rust_public_result':'Identity',
 'rust_direct_branch':'b_zero_identity',
 'rust_direct_result':'Identity',
 'cantor_result':'Identity',
 'public_undefined_inverse_count':0,
 'direct_raw_zero_weight_count':2,
 'direct_normalization_required_zero_denominators':2,
 'author_sha256':HASHES[A],'rust_sha256':HASHES[R],
}
with (OUT if not args.check else HERE / '.check-rows.tmp').open('w',newline='') as h:
    writer=csv.DictWriter(h,fieldnames=row.keys(),lineterminator='\n')
    writer.writeheader(); writer.writerow(row)
if args.check:
    temporary = HERE / '.check-rows.tmp'
    assert temporary.read_bytes() == OUT.read_bytes(), 'CSV differs from source-bound recompute'
    temporary.unlink()
print(OUT)
print(row)
