import json, hashlib, struct, math
from pathlib import Path
R=Path('/home/ec2-user/n83-candidate-recovery/proof-bridge-review-inputs/filtered/supplement')
M=Path('/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028/reviews/proof-bridge-20261030/filtered-supplement.json')
meta=json.loads(M.read_text())
sha=lambda b:hashlib.sha256(b).hexdigest()
pc=lambda m:bin(m).count('1')
raw_manifest={line.split()[1].removeprefix('./'):line.split()[0] for line in (R/'manifest.sha256').read_text().splitlines()}
for row in meta['files']:
    b=Path(row['path']).read_bytes()
    assert len(b)==row['bytes'] and sha(b)==row['sha256']
    if row['archive_relative_path']!='manifest.sha256':
        assert raw_manifest[row['archive_relative_path']]==sha(b)
key='original-prime-m3-natural'
input_path=R/'inputs'/key/'input.json'
source_path=R/'inputs'/key/'system.sing'
cert_path=R/'terminal'/key/'certificate.json'
top_path=R/'top'/f'{key}.json'
result_path=R/'results'/f'{key}.json'
inp=json.loads(input_path.read_text()); cert=json.loads(cert_path.read_text())
top=json.loads(top_path.read_text()); result=json.loads(result_path.read_text())
protocol=json.loads((R/'protocol.json').read_text())
case=next(c for c in protocol['extensions']['cases'] if c['key']==key)
assert cert['input']==inp
for name,path in [('input',input_path),('source',source_path),('terminal_certificate',cert_path),('top_certificate',top_path)]:
    assert sha(path.read_bytes())==case[name+'_sha256']==result[name+'_sha256']
for name in ['input_sha256','source_sha256','factor_base_manifest_sha256']:
    assert top[name]==case[name]
assert inp['factor_base_manifest_sha256']==case['factor_base_manifest_sha256']
for row in protocol['extensions']['source_files']:
    path=R/'source'/row['path']
    if path.exists():
        assert sha(path.read_bytes())==row['sha256']
p=int(inp['field_modulus']); t=int(inp['target_x']); n=27
assert p==4835703278761981908090881 and 0<=t<p
assert inp['family']=='prime' and inp['m']==3 and inp['ell']==27
assert inp['n_vars']==82 and inp['input_equations']==83 and inp['target_kind']=='natural'
assert inp['curve_a']=='2' and inp['curve_b']=='3' and inp['witness'] is None
factors={}; remaining=p-1
for q in [2,5,677,13999,23203]:
    assert all(q%d for d in range(2,math.isqrt(q)+1))
    e=0
    while remaining%q==0: remaining//=q; e+=1
    assert e>0
    factors[q]=e
assert remaining==1
a=next(a for a in range(2,1000) if pow(a,p-1,p)==1 and all(pow(a,(p-1)//q,p)!=1 for q in factors))
weights=[1<<j for j in range(n)]
assert [int(w) for w in cert['source']['weights']]==weights
assert all((w*int(v))%p==1 for w,v in zip(weights,top['normalization_inverses']))
def s3(x,y,z):
    return f'(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+2)+2*3)*({z})+(({x})*({y})-2)^2-4*3*(({x})+({y}))'
source=f'ring r=(integer,{p}),(v(1..82)),dp;\nideal I;\n'
for block in range(3):
    source+=f'poly x{block}='+ '+'.join(f'{w}*v({27*block+j+1})' for j,w in enumerate(weights))+';\n'
source+='I[1]='+s3('x0','x1','v(82)')+';\nI[2]='+s3('v(82)','x2',str(t))+';\n'
source+=''.join(f'I[{j+3}]=v({j+1})^2-v({j+1});\n' for j in range(81))
assert source.encode()==source_path.read_bytes()
assert sha(source.encode())==inp['source_sha256']
def parse(path):
    raw=path.read_bytes()
    assert raw[:8]==b'N83PFF1\n'
    width,deg,count=struct.unpack('<IIQ',raw[8:24])
    prime=int.from_bytes(raw[24:40],'little')
    assert (width,prime)==(n,p) and len(raw)==40+count*24
    poly={}; prior=-1
    for i in range(count):
        mask=int.from_bytes(raw[40+24*i:48+24*i],'little')
        coeff=int.from_bytes(raw[48+24*i:64+24*i],'little')
        assert prior<mask<(1<<n) and 0<coeff<p
        poly[mask]=coeff; prior=mask
    assert max(map(pc,poly),default=0)==deg
    return poly
def encode(poly):
    header=b'N83PFF1\n'+struct.pack('<IIQ',n,max(map(pc,poly),default=0),len(poly))+p.to_bytes(16,'little')
    return header+b''.join(m.to_bytes(8,'little')+c.to_bytes(16,'little') for m,c in sorted(poly.items()))
def norm(poly):
    return {m:c%p for m,c in poly.items() if c%p}
def multiply_x(poly):
    out={}
    for m,c in poly.items():
        for j,w in enumerate(weights):
            bit=1<<j; k=m|bit
            out[k]=out.get(k,0)+c*(w if m&bit else 1)
    return norm(out)
H={sum(1<<(2*j+((bits>>j)&1)) for j in range(13)):(p-1 if pc(bits)%2 else 1) for bits in range(1<<13)}
A=[H,multiply_x(H)]; A.append(multiply_x(A[1]))
for i,poly in enumerate(A):
    path=R/'terminal'/key/f'A{i}.poly'
    assert parse(path)==poly and encode(poly)==path.read_bytes()
    rec=cert['factor_files'][i]
    assert rec['sha256']==sha(encode(poly))==result['factor_sha256'][i]
    assert rec['terms']==len(poly) and rec['degree']==max(map(pc,poly))
B=[[4-12*t,-4*t-12,t*t],[-4*t-12,-2*t*t-4,-2*t],[t*t,-2*t,1]]
B=[[v%p for v in row] for row in B]
wc=cert['witness_circuit']
assert [[int(v) for v in row] for row in wc['B_coefficients_low_to_high']]==B
assert wc['U_variable_zero_based']==81 and wc['U_is_unbounded_indeterminate'] is True
C={}
for poly,scalar in zip(A,[t*t,-2*t,1]):
    for m,c in poly.items(): C[m]=C.get(m,0)+scalar*c
C=norm(C)
coefficient_path=R/'results'/f'{key}.poly'
assert parse(coefficient_path)==C and encode(C)==coefficient_path.read_bytes()
assert sha(encode(C))==result['coefficient_sha256']
pairing=sum(c*C.get(((1<<n)-1)^m,0) for m,c in H.items())%p
delta=(sum(weights)-2*t)%p
assert pairing==pow(-2,13,p)*delta%p==int(result['pairing'])!=0
assert delta==int(result['delta'])
# Assemble the circuit, including all U powers, to detect cross-term cancellations.
G={}
for poly,row in zip(A,B):
    for mask,c in poly.items():
        for u,scalar in enumerate(row):
            G[mask,u]=(G.get((mask,u),0)+c*scalar)%p
G={mon:c for mon,c in G.items() if c}
degree_g=max(pc(mask)+u for mask,u in G)
assert degree_g==16 and max(map(pc,C))==14
curve_checks=[(int(x),int(y)) for x,y in inp['points']]+[(t,int(inp['target_y']))]
assert all((y*y-x*x*x-2*x-3)%p==0 for x,y in curve_checks)
print(json.dumps({'status':'PASS','case':key,'supplement_files_hash_checked':len(meta['files']),'field_modulus':str(p),'lucas_n_minus_1_factors':factors,'lucas_witness':a,'source_sha256':sha(source.encode()),'A_terms':[len(poly) for poly in A],'A_degrees':[max(map(pc,poly)) for poly in A],'coefficient_terms':len(C),'coefficient_degree':max(map(pc,C)),'coefficient_sha256':sha(encode(C)),'g_terms':len(G),'g_degree':degree_g,'delta':str(delta),'pairing':str(pairing),'provided_point_and_target_curve_lifts_checked':len(curve_checks),'full_other_41_case_replay':False},indent=2))

