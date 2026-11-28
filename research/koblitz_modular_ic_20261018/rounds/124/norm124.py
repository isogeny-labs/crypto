#!/usr/bin/env python3
"""Precision-aware first N51 Laurent form on four marked23 x=0 places."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
PATHS={
 'donor51':ROUNDS/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'audit102':ROUNDS/'102/audit102.py','norm109':ROUNDS/'109/norm109.py',
 'result116':ROUNDS/'116/result.json','result118':ROUNDS/'118/result.json',
 'result123':ROUNDS/'123/result.json'}
P=640
MASK=(1<<P)-1

def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path)
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod

def mul(a,b,width=P):
    z=0
    while b:
        low=b&-b;z^=a<<(low.bit_length()-1);b^=low
    return z&((1<<width)-1)

def square(a,width):
    return sum(1<<(2*i) for i in range((width+1)//2) if a>>i&1)

def inverse(q,width):
    assert q&1
    z=1;known=1
    while known<width:
        known=min(2*known,width)
        z=mul(q,square(z,known),known)
    assert mul(q,z,width)==1
    return z

def val(a): return None if not a else (a&-a).bit_length()-1

def powers(a,n):
    out=[1]
    for _ in range(n):out.append(mul(out[-1],a))
    return out

def eval_poly(bits,xpow):
    out=0
    while bits:
        low=bits&-bits;out^=xpow[low.bit_length()-1];bits^=low
    return out

def f4mul(a,b):
    z=0
    for i in (0,1):
        if b>>i&1:z^=a<<i
    if z&4:z^=7
    return z

def f4pow(a,n):
    out=1
    for _ in range(n):out=f4mul(out,a)
    return out

def evaluate(parsed,base,k):
    x=sum(1<<i for i in base['x_exponents'])
    y=sum(1<<i for i in base['y_exponents'])
    maxdeg=max(max(num.bit_length(),den.bit_length())-1 for _,_,num,den in parsed)
    xp=powers(x,maxdeg);yp=powers(y,6)
    acc={j:{} for j in range(15)};bounds={j:P for j in range(15)}
    max_d=0;hidden=0
    for le,ye,num,den in parsed:
        a=mul(eval_poly(num,xp),yp[ye]);q=eval_poly(den,xp)
        n,d=val(a),val(q)
        assert d is not None and d<P,(le,ye,'denominator invisible')
        max_d=max(max_d,d)
        if n is None:
            hidden+=1;bounds[le]=min(bounds[le],P-d);continue
        width=P-max(n,d)
        assert width>0
        z=mul(a>>n,inverse(q>>d,width),width)
        shift=n-d;until=shift+width
        bounds[le]=min(bounds[le],until)
        for v in range(z.bit_length()):
            if z>>v&1:
                at=shift+v;acc[le][at]=acc[le].get(at,0)^1
    rows=[]
    for j in range(15):
        support=sorted(v for v,c in acc[j].items() if c and v<bounds[j])
        rows.append({'lambda_exponent':j,'s_order':support[0] if support else None,
          's_order_lower_bound':support[0] if support else bounds[j],
          'known_until_exclusive':bounds[j],
          'nonzero_support_below_bound':support})
    first=[]
    for alpha in (2,3):
        known=[(r['s_order']-k*r['lambda_exponent'],r) for r in rows if r['s_order'] is not None]
        minimum=min(w for w,_ in known)
        ties=[r['lambda_exponent'] for w,r in known if w==minimum]
        unknown=[r['lambda_exponent'] for r in rows if r['s_order'] is None and
                 r['s_order_lower_bound']-k*r['lambda_exponent']<=minimum]
        scalar=0
        for j in ties:scalar^=f4pow(alpha,j)
        first.append({'alpha':alpha,'candidate_s_order':minimum,'tied_lambda_exponents':ties,
          'unknown_competitors':unknown,'leading_F4_sum':scalar,
          'certified':not unknown and scalar!=0,
          'n51_s_order':minimum if not unknown and scalar!=0 else None})
    assert first[0]['candidate_s_order']==first[1]['candidate_s_order']
    assert first[1]['leading_F4_sum']==f4mul(first[0]['leading_F4_sum'],first[0]['leading_F4_sum'])
    return {'coefficient_rows':rows,'first_forms':first,
            'denominator_s_order_max':max_d,'hidden_numerator_terms':hidden}

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['precision_exclusive']==P
    for name,path in PATHS.items():
        assert hashlib.sha256(path.read_bytes()).hexdigest()==protocol['input_sha256'][name],name
    audit=load('audit102_r124',PATHS['audit102'])
    parser=load('norm109_r124',PATHS['norm109'])
    donor=PATHS['donor51'].read_text()
    parsed=parser.parse_terms(donor,audit)
    assert len(parsed)==105
    base=json.loads(PATHS['result116'].read_text())
    edges=json.loads(PATHS['result118'].read_text())
    census=json.loads(PATHS['result123'].read_text())
    assert base['precision_exclusive']==P and census['x_zero_divisor_degree']==112
    result={'protocol_id':protocol['id'],'input_sha256':protocol['input_sha256'],
            'precision_exclusive':P,'term_count':105,'branches':{}}
    for key,b in base['branches'].items():
        edge=edges['branches'][key]['newton_lower_hull_if_exact']
        assert len(edge)==2 and edge[0]['lambda_exponent']==0 and edge[1]['lambda_exponent']==16
        delta=edge[1]['s_order']-edge[0]['s_order']
        assert delta%16==0
        k=delta//16
        result['branches'][key]={'lambda_pole_order':k,
          'base_x_order':b['x_order'],**evaluate(parsed,b,k)}
    dest=Path(sys.argv[1]);dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('NORM124_ALL_CHECKS_PASS')

if __name__=='__main__':main()
