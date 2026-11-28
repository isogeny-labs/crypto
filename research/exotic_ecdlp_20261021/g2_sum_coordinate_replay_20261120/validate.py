#!/usr/bin/env python3
"""Replay leaf-sum F4 roots through the inverse map and curve group."""
import hashlib
import importlib.util
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def import_source(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def evaluate_polynomial(poly,assignment):
    answer=0
    for term in poly.split('+'):
        product=1
        for factor in term.strip().split('*'):
            if factor=='1':continue
            match=re.fullmatch(r'v(\d+)(?:\^(\d+))?',factor.strip())
            assert match,factor
            product&=(assignment>>int(match.group(1)))&1
        answer^=product
    return answer

def equations(path):
    lines=Path(path).read_text().strip().splitlines()
    assert lines[0]==','.join(f'v{i}' for i in range(30)) and lines[1]=='2'
    result='\n'.join(lines[2:]).split(',\n')
    assert len(result)==60
    return result

def triangular_roots(path):
    content=Path(path).read_text()
    assert '#field characteristic: 2' in content
    match=re.search(r'\[(.*?)\]:',content,re.S)
    assert match
    polynomials=[part.strip() for part in match.group(1).split(',')]
    assert len(polynomials)==30
    nonlinear=[poly for poly in polynomials if '^2' in poly]
    assert len(nonlinear)==1
    free_match=re.fullmatch(r'1\*v(\d+)\^2\+1\*v\1\^1',nonlinear[0])
    assert free_match,nonlinear[0]
    free=int(free_match.group(1))
    rules={}
    for poly in polynomials:
        if poly==nonlinear[0]:continue
        terms=poly.split('+')
        bits=[];constant=0
        for term in terms:
            if term=='1':constant^=1;continue
            var=re.fullmatch(r'1\*v(\d+)\^1',term)
            assert var,term
            bits.append(int(var.group(1)))
        leading=[bit for bit in bits if bit!=free]
        assert len(leading)==1
        index=leading[0]
        assert index not in rules
        rules[index]=(constant,free in bits)
    assert set(rules)==set(range(30))-{free}
    roots=[]
    for t in (0,1):
        assignment=t<<free
        for j,(constant,uses_free) in rules.items():
            if constant^(t if uses_free else 0):assignment|=1<<j
        assert all(evaluate_polynomial(poly,assignment)==0 for poly in polynomials)
        roots.append(assignment)
    assert len(set(roots))==2
    return free,roots

def log_metrics(path):
    data=Path(path).read_text()
    def num(pattern):
        match=re.search(pattern,data,re.M)
        assert match,pattern
        return int(match.group(1))
    matrix=re.search(r'max\. matrix data\s+(\d+) x (\d+)',data)
    assert matrix
    return {'rows_reduced':num(r'^#rows reduced\s+(\d+)'),
            'zero_reductions':num(r'^#zero reductions\s+(\d+)'),
            'pairs_reduced':num(r'^#pairs reduced\s+(\d+)'),
            'max_matrix_rows':int(matrix.group(1)),
            'max_matrix_columns':int(matrix.group(2))}

def main():
    protocol=json.loads((HERE/'validation_protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'validate.py')==protocol['validator_sha256']
    for name,item in protocol['sources'].items():assert sha(item['path'])==item['sha256'],name
    assert sha(HERE/'sage_runtime_info.json')==protocol['runtime_sha256']
    assert json.loads((HERE/'sage_runtime_info.json').read_text())['status']=='verified'
    prior=import_source(Path(protocol['sources']['prior_validator']['path']),'prior_f4_validator')
    previous=json.loads(Path(protocol['sources']['control_before']['path']).read_text())
    current=json.loads((HERE/'run_receipt.json').read_text())
    assert previous['status']==current['status']=='COMPLETED'
    arms=[previous['arms'][0],*current['arms']]
    assert [arm['status'] for arm in arms]==['COMPLETED']*3
    before_path=Path(protocol['sources']['control_before']['path']).parent
    for arm in arms:
        base=before_path if arm['name']=='control' else HERE
        for meta in arm['artifacts'].values():
            path=base/meta['file']
            assert path.stat().st_size==meta['bytes'] and sha(path)==meta['sha256']
    _,_,control_roots=prior.parse_basis(before_path/'control_basis.txt')
    _,_,after_roots=prior.parse_basis(HERE/'control_after_basis.txt')
    assert control_roots==after_roots
    free,sum_roots=triangular_roots(HERE/'sum_coordinate_basis.txt')
    mask=(1<<15)-1
    mapped=[]
    original_equations=equations(protocol['sources']['control_input']['path'])
    sum_equations=equations(protocol['sources']['sum_input']['path'])
    for assignment in sum_roots:
        u=assignment&mask;s=assignment>>15
        old=u|((u^s)<<15)
        assert all(evaluate_polynomial(eq,assignment)==0 for eq in sum_equations)
        assert all(evaluate_polynomial(eq,old)==0 for eq in original_equations)
        mapped.append(old)
    assert set(mapped)==set(control_roots)
    archived_sum=json.loads(Path(protocol['sources']['sum_validation']['path']).read_text())
    assert archived_sum['status']=='verified'
    assert sorted(sum_roots)==sorted(archived_sum['new_root_assignments'])
    assert sorted(mapped)==sorted(archived_sum['old_root_assignments'])
    algebra=json.loads(Path(protocol['sources']['algebra']['path']).read_text())
    relations=json.loads(Path(protocol['sources']['relations']['path']).read_text())
    K,modulus=prior.field(31)
    assert modulus==algebra['field_modulus_bits']
    E=prior.curve(K)
    basis=next(item['basis_bits'] for item in algebra['invariant_subspaces']
               if item['selection']=='S235')[1:]
    values,coordinates=prior.values_and_coordinates(basis)
    arm=next(item for item in relations['arms'] if item['selection']=='S235')
    base={tuple(point) for point in arm['base_point_keys']}
    query=arm['query_rows'][1]
    Q=E(prior.from_bits(K,query['point_key'][0]),prior.from_bits(K,query['point_key'][1]))
    singleton=arm['base_point_keys'][2]
    P=E(prior.from_bits(K,singleton[0]),prior.from_bits(K,singleton[1]))
    R=Q-P
    assert prior.point_key(R)==[551855874,2085658254]
    recovered=[]
    for old in mapped:
        u_mask=old&mask;v_mask=old>>15
        u=values[u_mask];v=values[v_mask]
        assert coordinates(u)==u_mask and coordinates(v)==v_mask
        witnesses=[]
        for Pa in E.lift_x(prior.from_bits(K,u),all=True):
            for Pb in E.lift_x(prior.from_bits(K,v),all=True):
                if Pa+Pb==R and tuple(prior.point_key(Pa)) in base and tuple(prior.point_key(Pb)) in base:
                    assert P+Pa+Pb==Q
                    witnesses.append([prior.point_key(Pa),prior.point_key(Pb)])
        assert witnesses
        recovered.append({'original_assignment_bits':old,'u_x_bits':u,'v_x_bits':v,
                          'group_pair_witnesses':witnesses})
    metrics={arm['name']:log_metrics((before_path if arm['name']=='control' else HERE)/arm['artifacts']['stderr']['file'])
             for arm in arms}
    result={'status':'PASS','protocol_sha256':sha(HERE/'validation_protocol.json'),
            'free_boolean_variable':free,'sum_coordinate_roots':sum_roots,
            'mapped_original_roots':sorted(mapped),'recovered':recovered,
            'metrics':metrics}
    (HERE/'validation.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':'PASS','roots':len(recovered),'metrics':metrics}))

if __name__=='__main__':main()
