#!/usr/bin/env python3
"""Greedy invertible XOR sparsification of the frozen N31 S3 equations."""
import hashlib
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def parse(poly):
    terms=set()
    for token in poly.strip().split('+'):
        mask=0
        for factor in token.strip().split('*'):
            if factor=='1':continue
            match=re.fullmatch(r'v(\d+)',factor)
            assert match,factor
            bit=int(match.group(1));assert 0<=bit<30
            mask|=1<<bit
        if mask in terms:terms.remove(mask)
        else:terms.add(mask)
    return terms

def render(poly):
    return '+'.join('1' if term==0 else '*'.join(f'v{j}' for j in range(30) if term>>j&1)
                    for term in sorted(poly,key=lambda value:(value.bit_count(),value)))

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'build.py')==protocol['builder_sha256']
    source=Path(protocol['control_input']['path'])
    assert sha(source)==protocol['control_input']['sha256']
    lines=source.read_text().strip().splitlines()
    assert lines[:2]==[','.join(f'v{i}' for i in range(30)),'2']
    polynomials='\n'.join(lines[2:]).split(',\n')
    assert len(polynomials)==60
    original=[parse(poly) for poly in polynomials[:30]]
    assert sum(map(len,original))==3768
    field=polynomials[30:]
    assert field==[f'v{i}^2+v{i}' for i in range(30)]
    current=[set(poly) for poly in original]
    operations=[]
    while True:
        candidates=[(len(current[i])-len(current[i]^current[j]),i,j)
                    for i in range(30) for j in range(30) if i!=j]
        gain,i,j=max(candidates,key=lambda item:(item[0],-item[1],-item[2]))
        if gain<=0:break
        current[i]^=current[j]
        operations.append({'row':i,'add_row':j,'term_reduction':gain})
    assert sum(map(len,current))==2936 and len(operations)==61
    replay=[set(poly) for poly in current]
    for op in reversed(operations):replay[op['row']]^=replay[op['add_row']]
    assert replay==original
    output=HERE/'sparsified.ms'
    output.write_text(lines[0]+'\n2\n'+',\n'.join([render(poly) for poly in current]+field)+'\n')
    assert len(output.read_text().strip().splitlines())==62
    result={'status':'PASS','source_sha256':sha(source),'output_sha256':sha(output),
            'original_terms':3768,'sparsified_terms':2936,'operations':operations,
            'field_equations_unchanged':True,'inverse_replay':'PASS'}
    (HERE/'build_result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':'PASS','terms':2936,'operations':61,'output_sha256':sha(output)}))

if __name__=='__main__':main()
