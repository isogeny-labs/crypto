"""Source-bound arithmetic certificate for the separate written argument.

Must be paired with fresh native prime_source_audit; no formal proof claim.
"""
from pathlib import Path
from math import comb
import hashlib,json,os,sys
import lefschetz_controls as controls

P=4835703278761981908090881
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def structural(ell,p):
    assert ell==27 and p==P and p>ell and p>17
    strings=[{'primitive_degree':k,'multiplicity':comb(ell,k)-(comb(ell,k-1) if k else 0),
              'maximum_exponent':ell-2*k,'length':ell-2*k+1} for k in range(ell//2+1)]
    hilbert=[]
    for h in range(ell+1):
        got=sum(x['multiplicity'] for x in strings if x['primitive_degree']<=h<=ell-x['primitive_degree'])
        assert got==comb(ell,h);hilbert.append({'degree':h,'dimension':got})
    assert sum(x['multiplicity']*x['length'] for x in strings)==2**ell
    degrees=[{'degree':d,'minimum_slack':ell-2*d+6} for d in range(4,17)]
    assert all(x['minimum_slack']>=0 for x in degrees)
    return {'schema':'n83.lefschetz-structure/v1','ell':ell,'p':str(p),'strings':strings,'hilbert':hilbert,
            'degrees':degrees,'first_uncovered_degree':17,'first_uncovered_slack':ell-2*17+6,
            'external_theorem':'Lundqvist-Nicklasson1604.06820 Theorem2.7 and Section3',
            'argument':'PRIME_LEFSCHETZ_ARGUMENT.md','basis_explicitly_expanded':False,
            'proof_status':'Written argument and arithmetic checks; independent review and formalization pending'}

def upper_witness(p):
    h={0:1}
    for i in range(0,26,2):
        out={}
        for mask,c in h.items():
            controls.put(out,mask|(1<<i),c,p);controls.put(out,mask|(1<<(i+1)),-c,p)
        h=out
    assert len(h)==8192 and all(bin(mask).count('1')==13 and c for mask,c in h.items())
    first=controls.up(h,27,p);second=controls.up(first,27,p);assert not second
    return {'schema':'n83.lefschetz-upper/v1','normalized_terms':[[mask,str(c)] for mask,c in sorted(h.items())],
            'degree':13,'terms':8192,'after_one_L_terms':len(first),'after_two_L_terms':0,
            'first_fall_upper_bound':17,'nontriviality':'Last-block squarefree quotient retains H and kills every other top generator',
            'source_projection_required':True}

def check_case(case,audit,input_path,source_path,upper):
    for field in ['key','input_sha256','source_sha256','factor_base_manifest_sha256']:
        assert case[field]==audit[field]
    assert sha(input_path)==case['input_sha256'] and sha(source_path)==case['source_sha256']
    inp=json.loads(Path(input_path).read_text());m=inp['m'];assert 3<=m<=9
    assert inp['ell']==27 and inp['field_modulus']==str(P) and audit['p']==str(P)
    assert inp['factor_base_manifest_sha256']==case['factor_base_manifest_sha256']
    for field in ['key','m','arm','target_kind','source_sha256','factor_base_manifest_sha256']:
        assert upper[field]==audit[field]
    assert upper['first_fall_upper_bound']==17
    assert audit['source']['full_source_template_verified'] is True
    weights=list(map(int,audit['source']['weights']));inverses=list(map(int,audit['normalization_inverses']))
    assert len(weights)==len(inverses)==27
    assert all(0<w<P and w*inv%P==1 for w,inv in zip(weights,inverses))
    return {'key':case['key'],'m':m,'arm':audit['arm'],'target_kind':audit['target_kind'],
            'input_sha256':case['input_sha256'],'source_sha256':case['source_sha256'],
            'factor_base_manifest_sha256':case['factor_base_manifest_sha256'],
            'first_fall_lower_bound':17,'first_fall_upper_bound':17,'first_fall_degree':17,
            'last_fall_lower_bound':None,'last_fall_degree':None,
            'claim_status':'Conditional on the written Lefschetz/Koszul argument; independent review pending'}

def execute(root,protocol,out,replay):
    p=json.loads(protocol.read_text());assert p['freeze']['status']=='frozen'
    assert os.environ.get('N83_LEFSCHETZ_FROZEN_PRODUCTION')=='1'
    cases=p['extensions']['cases'];expected={f'{a}-prime-m{m}-{t}' for a in ['original','baseline','spike40'] for m in range(3,10) for t in ['planted','natural']}
    assert len(cases)==42 and {x['key'] for x in cases}==expected
    for source in p['extensions']['source_files']:
        assert sha(root/'source'/source['path'])==source['sha256']
    assert sha(root/'instrument')==p['arms'][0]['instrument']['binary_sha256']
    assert sha(root/'upper-summary.json')==p['extensions']['upper_summary_sha256']
    uppers=json.loads((root/'upper-summary.json').read_text())['rows'];assert len(uppers)==42
    upper={x['key']:x for x in uppers};assert set(upper)==expected
    if replay:assert out.is_dir()
    else:assert not out.exists();out.mkdir()
    def save(name,value):
        value=json.loads(json.dumps(value));path=out/name
        if replay:assert json.loads(path.read_text())==value,name
        else:
            with path.open('x') as f:json.dump(value,f,indent=2);f.write('\n')
    save('structure.json',structural(27,P));save('upper-witness.json',upper_witness(P))
    for m in range(3,10):
        gate=controls.leading_gate(m,[m]*m+list(range(m-2,0,-1)));assert gate['valid']
        save(f'leading-m{m}.json',gate)
    rows=[]
    for case in cases:
        key=case['key'];ap=root/'source-audit'/f'{key}.json';audit=json.loads(ap.read_text())
        row=check_case(case,audit,root/'inputs'/key/'input.json',root/'inputs'/key/'system.sing',upper[key])
        record={'schema':'n83.lefschetz-case/v1',**row,'native_source_audit_sha256':sha(ap),
                'structure_sha256':sha(out/'structure.json'),'upper_witness_sha256':sha(out/'upper-witness.json'),
                'leading_sha256':sha(out/f"leading-m{row['m']}.json"),'normalization_inverses':audit['normalization_inverses']}
        save(key+'.json',record);rows.append(row)
    save('summary.json',{'schema':'n83.lefschetz-summary/v1','rows':rows,'shared_structural_certificate':True,
         'all_prior_upper_source_hashes_matched':True,'new_lfd_claim':False,'independent_review':'pending',
         'proof_boundary':'External block SLP theorem plus written chain/Koszul argument and source-bound arithmetic certificates'})
    print(json.dumps({'cases_verified':42,'replay':replay,'first_fall_degree':17,'new_lfd_claim':False}))

if __name__=='__main__':
    assert len(sys.argv)==5 and sys.argv[1] in ['run','replay']
    execute(Path(sys.argv[2]),Path(sys.argv[3]),Path(sys.argv[4]),sys.argv[1]=='replay')
