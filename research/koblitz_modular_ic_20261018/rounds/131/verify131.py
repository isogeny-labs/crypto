#!/usr/bin/env python3
"""Independent N51 rational evaluation and factor coprimality check."""
import hashlib
import importlib.util
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
p=json.loads((HERE/'protocol.json').read_text())
r=json.loads((HERE/'result.json').read_text())
paths={'n51_excerpt':HERE/'source-data/N51.sing','r102_parser':ROUNDS/'102/audit102.py',
       'r130_result':ROUNDS/'130/result.json','r130_source':ROUNDS/'130/lambda130.py'}
for label,path in paths.items():assert hashlib.sha256(path.read_bytes()).hexdigest()==p['input_sha256'][label]
original=ROUNDS/'51/runs/base23-struct51-03/marked23-triangular51.sing'
if original.exists():
    raw=original.read_bytes();assert hashlib.sha256(raw).hexdigest()==p['full_r51_donor_sha256']
    start=raw.index(b'poly N51=');end=raw.index(b';',start)+1
    assert paths['n51_excerpt'].read_bytes()==raw[start:end]+b'\n'
spec=importlib.util.spec_from_file_location('verify130_for_131',ROUNDS/'130/verify130.py')
prior=importlib.util.module_from_spec(spec);spec.loader.exec_module(prior)

def split_outer(s):
    parts=[];depth=0;start=0
    for i,c in enumerate(s):
        if c=='(':depth+=1
        elif c==')':depth-=1
        elif c=='+' and depth==0:
            parts.append(s[start:i]);start=i+1
    assert depth==0
    parts.append(s[start:]);return parts

def xpoly_value(s,K):
    result=0
    seen=set()
    for term in s.split('+'):
        if term=='1':e=0
        elif term=='x':e=1
        else:
            assert term.startswith('x^')
            e=int(term[2:])
        assert e not in seen
        seen.add(e);result^=K.power(2,e)
    return result

source=paths['n51_excerpt'].read_text()
assert source.startswith('poly N51=') and source.endswith(';\n')
parts=split_outer(source[len('poly N51='):-2])
assert len(parts)==106 and parts[0]=='n'
terms=[];keys=set()
for part in parts[1:]:
    m=re.fullmatch(r'\(([^()]*)\)/(?:\(([^()]*)\)|(x(?:\^[1-9][0-9]*)?))(.*)',part)
    assert m,part[:100]
    num,den,bare,suffix=m.groups();den=den or bare
    le=ye=0
    for atom in (suffix[1:].split('*') if suffix else []):
        z=re.fullmatch(r'([ly])(?:\^([1-9][0-9]*))?',atom)
        assert z
        if z.group(1)=='l':le=int(z.group(2) or 1)
        else:ye=int(z.group(2) or 1)
    assert (le,ye) not in keys
    keys.add((le,ye));terms.append((le,ye,num,den))
assert len(keys)==105

for fiber,found in zip(prior.r['fibers'],r['fibers']):
    K=prior.K(4 if fiber['field_degree_over_F2']==28 else 1)
    y=fiber['y_residue'];h=[0]*16
    for le,ye,num,den in terms:
        numerator=xpoly_value(num,K);denominator=xpoly_value(den,K)
        assert denominator!=0
        c=K.mul(numerator,K.inverse(denominator))
        h[le]^=K.mul(c,K.power(y,ye))
    h=prior.trim(h)
    assert h==found['N51_lambda_coefficients']
    for part,group in zip(fiber['degree_parts'],found['groups']):
        residue=prior.remainder(h,part['factor'],K)
        assert residue==group['residue_remainder']
        assert prior.gcd(part['factor'],residue,K)==group['gcd_with_N51']==[1]
        assert group['unit_residue_count']==group['count']
assert r['place_count_check']==r['unit_place_count']==12
assert r['zero_residue_place_count']==0
print('RESIDUE131_INDEPENDENT_ALL_UNITS_PASS')
