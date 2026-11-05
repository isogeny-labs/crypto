#!/usr/bin/env python3
"""Independent polynomial long-division audit of R124 N51 rows."""
import hashlib
import importlib.util
import json
from pathlib import Path

here=Path(__file__).resolve().parent
rounds=here.parent
result=json.loads((here/'result.json').read_text())
p=json.loads((here/'protocol.json').read_text())
paths={'donor51':rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
       'audit102':rounds/'102/audit102.py','norm109':rounds/'109/norm109.py',
       'result116':rounds/'116/result.json','result118':rounds/'118/result.json',
       'result123':rounds/'123/result.json'}
for name,path in paths.items():
    assert hashlib.sha256(path.read_bytes()).hexdigest()==p['input_sha256'][name]

def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path)
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

audit=load('audit102_check124',paths['audit102'])
parser=load('norm109_check124',paths['norm109'])
terms=parser.parse_terms(paths['donor51'].read_text(),audit)
base=json.loads(paths['result116'].read_text())
P=640;mask=(1<<P)-1

def mul(a,b):
    z=0
    for i in range(b.bit_length()):
        if b>>i&1:z^=a<<i
    return z&mask

def powers(a,n):
    out=[1]
    for _ in range(n):out.append(mul(out[-1],a))
    return out

def poly(bits,xp):
    z=0
    for i in range(bits.bit_length()):
        if bits>>i&1:z^=xp[i]
    return z

def order(a):return None if not a else (a&-a).bit_length()-1

def quotient(a,q,width):
    assert q&1
    rem=a&((1<<width)-1);out=0
    for i in range(width):
        if rem>>i&1:
            out|=1<<i;rem^=q<<i
    assert rem&((1<<width)-1)==0
    return out

for key,b in base['branches'].items():
    x=sum(1<<i for i in b['x_exponents']);y=sum(1<<i for i in b['y_exponents'])
    degree=max(max(a.bit_length(),q.bit_length())-1 for _,_,a,q in terms)
    xp=powers(x,degree);yp=powers(y,6)
    sums={j:{} for j in range(15)};bounds={j:P for j in range(15)}
    for le,ye,num,den in terms:
        a=mul(poly(num,xp),yp[ye]);q=poly(den,xp)
        n,d=order(a),order(q)
        assert d is not None and d<P
        if n is None:bounds[le]=min(bounds[le],P-d);continue
        width=P-max(n,d);offset=n-d;until=offset+width
        ratio=quotient(a>>n,q>>d,width)
        bounds[le]=min(bounds[le],until)
        for i in range(width):
            if ratio>>i&1:
                v=offset+i;sums[le][v]=sums[le].get(v,0)^1
    rows=result['branches'][key]['coefficient_rows']
    for row in rows:
        j=row['lambda_exponent'];support=sorted(v for v,c in sums[j].items() if c and v<bounds[j])
        assert row['known_until_exclusive']==bounds[j]
        assert row['nonzero_support_below_bound']==support
        assert row['s_order']==(support[0] if support else None)
    k=result['branches'][key]['lambda_pole_order']
    for first in result['branches'][key]['first_forms']:
        weights=[(row['s_order']-k*row['lambda_exponent'],row['lambda_exponent']) for row in rows if row['s_order'] is not None]
        minimum=min(v for v,_ in weights)
        tied=[j for v,j in weights if v==minimum]
        scalar=0
        for j in tied:scalar^=(1,first['alpha'],first['alpha']^1)[j%3]
        assert first['candidate_s_order']==minimum and first['tied_lambda_exponents']==tied
        assert first['leading_F4_sum']==scalar==0
        assert not first['unknown_competitors'] and not first['certified']
    print(key,'independent 105-term rows match')
print('NORM124_INDEPENDENT_LONG_DIVISION_PASS')
