#!/usr/bin/env python3
"""Independent binomial/F4-table replay of all R125 translated rows."""
import hashlib
import json
import math
from pathlib import Path

here=Path(__file__).resolve().parent
rounds=here.parent
p=json.loads((here/'protocol.json').read_text())
data={}
for n,h in p['input_sha256'].items():
    path=rounds/n/'result.json'
    assert hashlib.sha256(path.read_bytes()).hexdigest()==h
    data[int(n)]=json.loads(path.read_text())
out=json.loads((here/'result.json').read_text())
TABLE=((0,0,0,0),(0,1,2,3),(0,2,3,1),(0,3,1,2))

def times(a,b):return TABLE[a][b]
def pow4(a,n):
    z=1
    for _ in range(n):z=times(z,a)
    return z

def stage(key,alpha):
    get=lambda n:next(e for e in data[n]['branches'][key] if e['alpha']==alpha)
    if key=='y1_e1':return [],get(119)
    if key=='y1_e2':return [(2,get(120)['beta'])],get(120)
    if key=='y0_e1':return [(1,get(120)['beta']),(2,get(121)['gamma'])],get(121)
    return [(1,get(120)['beta']),(1,get(121)['gamma']),(1,get(122)['delta'])],get(122)

for key,entries in out['branches'].items():
    source=data[124]['branches'][key]['coefficient_rows']
    k=data[124]['branches'][key]['lambda_pole_order']
    for entry in entries:
        alpha=entry['alpha'];path,terminal=stage(key,alpha)
        assert entry['substitution_path']==[list(x) for x in path]
        coeff=[{v:1 for v in r['nonzero_support_below_bound']} for r in source]
        bound=[r['known_until_exclusive'] for r in source]
        for shift,beta in [(-k,alpha)]+path:
            new=[{} for _ in range(15)];cut=[None]*15
            for le in range(15):
                for j in range(le+1):
                    if not math.comb(le,j)&1:continue
                    atbound=bound[le]+shift*le
                    cut[j]=atbound if cut[j] is None else min(cut[j],atbound)
                    scale=pow4(beta,le-j)
                    for v,a in coeff[le].items():
                        at=v+shift*le
                        new[j][at]=new[j].get(at,0)^times(a,scale)
            coeff=[{v:a for v,a in row.items() if a and v<cut[j]} for j,row in enumerate(new)]
            bound=cut
        rows=entry['final_coefficient_rows']
        for j,row in enumerate(rows):
            support=[[v,a] for v,a in sorted(coeff[j].items())]
            assert row['known_until_exclusive']==bound[j]
            assert row['support_count']==len(support)
            assert row['support_sha256']==hashlib.sha256(json.dumps(support,separators=(',',':')).encode()).hexdigest()
            assert row['s_order']==(support[0][0] if support else None)
            assert row['leading_F4']==(support[0][1] if support else None)
        local=terminal['translated_rows'];c0=local[0]['nonzero_support_below_bound'][0][1]
        for label,ram,q,lead in [('simple',1,local[0]['s_order']-local[1]['s_order'],c0),
                                 ('tame_seven',7,local[1]['s_order']-local[3]['s_order'],1)]:
            vals=[(ram*r['s_order']+q*j,j,r['leading_F4']) for j,r in enumerate(rows) if r['s_order'] is not None]
            minimum=min(v for v,_,_ in vals)
            ties=[j for v,j,_ in vals if v==minimum]
            scalar=0
            for v,j,a in vals:
                if v==minimum:scalar^=times(a,pow4(lead,j))
            found=entry[label]
            assert found['candidate_normalized_order']==minimum
            assert found['tied_correction_exponents']==ties
            assert found['leading_F4_sum']==scalar
    print(key,'independent translated rows match')
expected={'y0_e1':-30,'y0_e3':-21,'y1_e1':0,'y1_e2':0}
for key,entries in out['branches'].items():
    for entry in entries:
        assert entry['simple']['certified'] and entry['simple']['n51_normalized_order']==expected[key]
        assert not entry['tame_seven']['certified']
        assert entry['tame_seven']['tied_correction_exponents']==[0,7]
        assert entry['tame_seven']['leading_F4_sum']==0
print('BRANCH125_INDEPENDENT_BINOMIAL_PASS')
