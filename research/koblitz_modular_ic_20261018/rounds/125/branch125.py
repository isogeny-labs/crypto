#!/usr/bin/env python3
"""Precision-aware N51 evaluation after the marked23 lambda corrections."""
import hashlib
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
DEG=15

def mul(a,b):
    z=0
    for i in (0,1):
        if b>>i&1:z^=a<<i
    if z&4:z^=7
    return z

def power(a,n):
    z=1
    for _ in range(n):z=mul(z,a)
    return z

def source_poly(rows):
    out=[]
    for r in rows:
        assert r['lambda_exponent']==len(out)
        out.append((r['known_until_exclusive'],{v:1 for v in r['nonzero_support_below_bound']}))
    assert len(out)==DEG
    return out

def translate(poly,shift,beta):
    out=[({},None) for _ in range(DEG)]
    for le,(bound,series) in enumerate(poly):
        for j in range(le+1):
            if j&~le:continue
            coeff,old_bound=out[j]
            until=bound+shift*le
            new_bound=until if old_bound is None else min(old_bound,until)
            factor=power(beta,le-j)
            for v,a in series.items():
                at=v+shift*le
                coeff[at]=coeff.get(at,0)^mul(a,factor)
            out[j]=(coeff,new_bound)
    return [(bound,{v:a for v,a in series.items() if a and v<bound}) for series,bound in out]

def summary(poly):
    rows=[]
    for j,(bound,series) in enumerate(poly):
        support=[[v,a] for v,a in sorted(series.items()) if a and v<bound]
        digest=hashlib.sha256(json.dumps(support,separators=(',',':')).encode()).hexdigest()
        rows.append({'correction_exponent':j,'s_order':support[0][0] if support else None,
                     's_order_lower_bound':support[0][0] if support else bound,
                     'known_until_exclusive':bound,
                     'leading_F4':support[0][1] if support else None,
                     'support_count':len(support),'support_sha256':digest,
                     'first_support':support[:8]})
    return rows

def weighted(rows,ram,q,lead):
    known=[(ram*r['s_order']+q*r['correction_exponent'],r) for r in rows if r['s_order'] is not None]
    if not known:return {'certified':False,'reason':'no visible coefficient'}
    minimum=min(w for w,_ in known)
    tied=[r for w,r in known if w==minimum]
    unknown=[r['correction_exponent'] for r in rows if r['s_order'] is None and
             ram*r['s_order_lower_bound']+q*r['correction_exponent']<=minimum]
    scalar=0
    for r in tied:scalar^=mul(r['leading_F4'],power(lead,r['correction_exponent']))
    certified=not unknown and scalar!=0
    return {'ramification_over_base':ram,'correction_order_in_normalized_parameter':q,
            'correction_leading_F4':lead,'candidate_normalized_order':minimum,
            'tied_correction_exponents':[r['correction_exponent'] for r in tied],
            'unknown_competitors':unknown,'leading_F4_sum':scalar,
            'certified':certified,'n51_normalized_order':minimum if certified else None}

def path_for(key,alpha,data):
    at=lambda n:next(e for e in data[n]['branches'][key] if e['alpha']==alpha)
    if key=='y1_e1':return [],at(119)
    if key=='y1_e2':return [(2,at(120)['beta'])],at(120)
    if key=='y0_e1':return [(1,at(120)['beta']),(2,at(121)['gamma'])],at(121)
    if key=='y0_e3':return [(1,at(120)['beta']),(1,at(121)['gamma']),(1,at(122)['delta'])],at(122)
    raise AssertionError(key)

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    data={}
    for n,expected in protocol['input_sha256'].items():
        path=ROUNDS/n/'result.json'
        assert hashlib.sha256(path.read_bytes()).hexdigest()==expected,n
        data[int(n)]=json.loads(path.read_text())
    assert data[123]['normalized_places_over_f2']==8
    result={'protocol_id':protocol['id'],'input_sha256':protocol['input_sha256'],'branches':{}}
    for key,b in data[124]['branches'].items():
        k=b['lambda_pole_order']
        entries=[]
        for alpha in (2,3):
            poly=translate(source_poly(b['coefficient_rows']),-k,alpha)
            shifts,stage=path_for(key,alpha,data)
            for shift,beta in shifts:poly=translate(poly,shift,beta)
            rows=summary(poly)
            local=stage['translated_rows']
            c0=local[0]['nonzero_support_below_bound'][0][1]
            c1=local[1]['nonzero_support_below_bound'][0][1]
            c8=local[3]['nonzero_support_below_bound'][0][1]
            assert c1==c8==1 and c0 in (2,3)
            simple_q=local[0]['s_order']-local[1]['s_order']
            seven_q=local[1]['s_order']-local[3]['s_order']
            assert simple_q>0 and seven_q>0 and seven_q%7!=0
            entries.append({'alpha':alpha,'lambda_pole_order':k,
              'substitution_path':[[shift,beta] for shift,beta in shifts],
              'final_coefficient_rows':rows,
              'simple':weighted(rows,1,simple_q,c0),
              'tame_seven':weighted(rows,7,seven_q,1)})
        a,beta=entries
        for kind in ('simple','tame_seven'):
            x,y=a[kind],beta[kind]
            assert x['candidate_normalized_order']==y['candidate_normalized_order']
            assert mul(x['leading_F4_sum'],x['leading_F4_sum'])==y['leading_F4_sum']
            assert x['certified']==y['certified']
        result['branches'][key]=entries
    dest=Path(sys.argv[1]);dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('BRANCH125_ALL_CHECKS_PASS')

if __name__=='__main__':main()
