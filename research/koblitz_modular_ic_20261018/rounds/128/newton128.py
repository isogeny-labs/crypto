#!/usr/bin/env python3
"""Exact F128 Hasse expansion of f23(a+s,90+u)."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent

def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def main(dest):
    protocol=json.loads((HERE/'protocol.json').read_text())
    source=ROUNDS/'51/source-data/marked23-field51.sing'
    prior=ROUNDS/'127/result.json'
    code=ROUNDS/'127/fiber127.py'
    assert digest(source)==protocol['input_sha256']['r51_source']
    assert digest(prior)==protocol['input_sha256']['r127_result']
    assert digest(code)==protocol['input_sha256']['r127_source']
    spec=importlib.util.spec_from_file_location('fiber127_pinned',code)
    f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f)
    old=json.loads(prior.read_text())
    assert old['rational_roots']==[[90,2],[123,1]]
    terms=f.parse_source(source.read_text())
    residue=protocol['residue_y']
    assert residue==90 and protocol['field_polynomial_bits']==203
    support={}
    for ye,xexps in terms.items():
        for e in xexps:
            for i in range(e+1):
                if i & ~e: continue
                for j in range(ye+1):
                    if j & ~ye: continue
                    key=(i,j)
                    support[key]=support.get(key,0)^f.gf_mul(f.gf_pow(2,e-i),f.gf_pow(residue,ye-j))
    support={key:a for key,a in support.items() if a}
    by_u=[]
    for j in range(8):
        entries=sorted((i,a) for (i,jj),a in support.items() if jj==j)
        by_u.append({'u_exponent':j,'s_order':entries[0][0] if entries else None,
                     'leading_F128':entries[0][1] if entries else None,'terms':entries})
    shifted=[row['terms'][0][1] if row['s_order']==0 else 0 for row in by_u]
    direct=[0]*8
    for j,a in enumerate(old['specialized_f']):
        for k in range(j+1):
            if k & ~j:continue
            direct[k]^=f.gf_mul(a,f.gf_pow(residue,j-k))
    assert shifted==direct and shifted[0]==shifted[1]==0 and shifted[2]!=0
    points=[(r['u_exponent'],r['s_order']) for r in by_u if r['s_order'] is not None]
    hull=[]
    for point in points:
        while len(hull)>1:
            x0,y0=hull[-2];x1,y1=hull[-1];x2,y2=point
            if (x1-x0)*(y2-y1)-(y1-y0)*(x2-x1)>0:break
            hull.pop()
        hull.append(point)
    edges=[]
    for (lo,vlo),(hi,vhi) in zip(hull,hull[1:]):
        dx=hi-lo;dy=vlo-vhi
        from math import gcd
        divisor=gcd(dx,dy)
        den=dx//divisor;num=dy//divisor
        weight=den*vlo+num*lo
        residual=[[j,r['leading_F128']] for j,r in enumerate(by_u)
                  if r['s_order'] is not None and den*r['s_order']+num*j==weight]
        assert residual[0][0]==lo and residual[-1][0]==hi
        edges.append({'left_u_exponent':lo,'right_u_exponent':hi,
                      'slope_numerator':-num,'slope_denominator':den,
                      'normal_weight':weight,'residual_terms':residual})
    result={'protocol_id':protocol['id'],'input_sha256':protocol['input_sha256'],
            'field_polynomial_bits':203,'x_residue':2,'y_residue':residue,
            'support':[[i,j,a] for (i,j),a in sorted(support.items())],
            'by_u':by_u,'s0_specialization':shifted,'hull':hull,'edges':edges}
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('NEWTON128_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
