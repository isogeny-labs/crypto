#!/usr/bin/env python3
"""Independently replay the frozen leaf-sum panel roots and group relations."""
import hashlib
import importlib.util
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def source(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def evaluate(poly,assignment):
    value=0
    for term in poly.split('+'):
        product=1
        for factor in term.strip().split('*'):
            if factor=='1':continue
            match=re.fullmatch(r'v(\d+)(?:\^(\d+))?',factor.strip())
            assert match,factor
            product&=(assignment>>int(match.group(1)))&1
        value^=product
    return value

def equations(path):
    lines=Path(path).read_text().strip().splitlines()
    assert lines[:2]==[','.join(f'v{i}' for i in range(30)),'2']
    result='\n'.join(lines[2:]).split(',\n')
    assert len(result)==60
    return result

def basis_roots(path):
    content=Path(path).read_text()
    assert '#field characteristic: 2' in content
    match=re.search(r'\[(.*?)\]:',content,re.S)
    assert match
    polynomials=[part.strip() for part in match.group(1).split(',')]
    normalized=[]
    for poly in polynomials:
        terms=set()
        for term in poly.split('+'):
            monomial=0
            for factor in term.split('*'):
                if factor=='1':continue
                var=re.fullmatch(r'v(\d+)\^(\d+)',factor)
                assert var,factor
                index=int(var.group(1));assert 0<=index<30
                monomial|=1<<index
            terms.symmetric_difference_update((monomial,))
        normalized.append(terms)
    linear=[]
    for poly in normalized:
        if all(term.bit_count()<=1 for term in poly):
            row=0
            for term in poly:
                if term:row^=term
            linear.append([row,1 if 0 in poly else 0])
    pivots={}
    for row,rhs in linear:
        for pivot in sorted(pivots,reverse=True):
            if row>>pivot&1:
                old,old_rhs=pivots[pivot];row^=old;rhs^=old_rhs
        if row==0:
            if rhs:return [],'unit'
            continue
        pivot=row.bit_length()-1
        pivots[pivot]=(row,rhs)
    free=[index for index in range(30) if index not in pivots]
    assert len(free)<=8,free
    roots=[]
    for seed in range(1<<len(free)):
        assignment=sum(1<<j for k,j in enumerate(free) if seed>>k&1)
        for pivot,(row,rhs) in sorted(pivots.items()):
            if rhs^((row&assignment).bit_count()&1):assignment|=1<<pivot
        if all(evaluate(poly,assignment)==0 for poly in polynomials):roots.append(assignment)
    assert len(set(roots))==len(roots)
    return roots,f'{len(polynomials)}_polynomials_{len(free)}_free_boolean_variables'

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

def enumerate_s3_roots(prior,modulus,values,coordinates,q):
    """Enumerate all u in the selected subspace and solve S3 for v in the field."""
    mul,square,inv,sqrt,solve_artin=prior.field_ops(modulus)
    roots=set();field_candidates=0
    for u_mask,u in enumerate(values):
        a=square(u^q);b=mul(q,u);c=square(b)^1
        candidates=[]
        if a==0:
            if b:candidates=[mul(c,inv(b))]
            else:assert c
        elif b==0:candidates=[sqrt(mul(c,inv(a)))]
        else:
            d=mul(mul(a,c),inv(square(b)))
            t=solve_artin(d)
            if t is not None:
                scale=mul(b,inv(a))
                candidates=[mul(scale,t),mul(scale,t^1)]
        field_candidates+=len(candidates)
        for v in candidates:
            assert square(mul(u,v)^mul(q,u^v))^mul(mul(u,v),q)^1==0
            v_mask=coordinates(v)
            if v_mask is not None:roots.add(u_mask|(v_mask<<15))
    return roots,field_candidates

def main():
    protocol=json.loads((HERE/'validation_protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'validate.py')==protocol['validator_sha256']
    for item in protocol['sources'].values():assert sha(item['path'])==item['sha256']
    assert json.loads((HERE/'sage_runtime_info.json').read_text())['status']=='verified'
    preparation=json.loads((HERE/'preparation.json').read_text())
    panel=json.loads((HERE/'panel_run.json').read_text())
    assert panel['status']=='COMPLETED' and len(panel['arms'])==4
    assert panel['preparation_sha256']==sha(HERE/'preparation.json')
    assert panel['protocol_sha256']==sha(HERE/'protocol.json')
    prior=source(protocol['sources']['prior_validator']['path'],'prior_validator')
    algebra=json.loads(Path(protocol['sources']['algebra']['path']).read_text())
    relations=json.loads(Path(protocol['sources']['relations']['path']).read_text())
    basis=next(item['basis_bits'] for item in algebra['invariant_subspaces']
               if item['selection']=='S235')[1:]
    values,coordinates=prior.values_and_coordinates(basis)
    K,modulus=prior.field(31)
    assert modulus==algebra['field_modulus_bits']
    E=prior.curve(K)
    arm=next(item for item in relations['arms'] if item['selection']=='S235')
    base={tuple(point) for point in arm['base_point_keys']}
    rows=[]
    for record in preparation['records']:
        index=record['query_index']
        pair={a['kind']:a for a in panel['arms'] if a['query_index']==index}
        assert set(pair)=={'control','leaf_sum'}
        assert all(a['residual_point']==record['residual_point'] for a in pair.values())
        metrics={};roots={};shapes={}
        for kind,a in pair.items():
            expected=record['control_sha256' if kind=='control' else 'sum_sha256']
            assert a['input_sha256']==expected
            for meta in a['artifacts'].values():
                path=HERE/'runs'/meta['file']
                assert path.stat().st_size==meta['bytes'] and sha(path)==meta['sha256']
            if a['status']!='COMPLETED':continue
            assert a['exit_code']==0
            roots[kind],shapes[kind]=basis_roots(HERE/'runs'/a['artifacts']['basis']['file'])
            metrics[kind]=log_metrics(HERE/'runs'/a['artifacts']['stderr']['file'])
            input_path=HERE/record['control_input' if kind=='control' else 'sum_input']
            eqs=equations(input_path)
            assert all(evaluate(eq,assignment)==0 for assignment in roots[kind] for eq in eqs)
        status='CENSORED'
        witnesses=[]
        if set(roots)=={'control','leaf_sum'}:
            mask=(1<<15)-1
            mapped=[(assignment&mask)|((((assignment&mask)^(assignment>>15)))<<15)
                    for assignment in roots['leaf_sum']]
            assert set(mapped)==set(roots['control'])
            control_eqs=equations(HERE/record['control_input'])
            assert all(evaluate(eq,x)==0 for x in mapped for eq in control_eqs)
            independent,field_candidates=enumerate_s3_roots(
                prior,modulus,values,coordinates,record['residual_point'][0])
            assert independent==set(mapped)
            Q=E(prior.from_bits(K,record['query_point'][0]),prior.from_bits(K,record['query_point'][1]))
            P=E(prior.from_bits(K,record['singleton_point'][0]),prior.from_bits(K,record['singleton_point'][1]))
            R=E(prior.from_bits(K,record['residual_point'][0]),prior.from_bits(K,record['residual_point'][1]))
            assert Q-P==R
            for assignment in mapped:
                u_mask=assignment&mask;v_mask=assignment>>15
                u=values[u_mask];v=values[v_mask]
                assert coordinates(u)==u_mask and coordinates(v)==v_mask
                found=[];group_pairs=[]
                for Pa in E.lift_x(prior.from_bits(K,u),all=True):
                    for Pb in E.lift_x(prior.from_bits(K,v),all=True):
                        if Pa+Pb==R:
                            assert P+Pa+Pb==Q
                            group_pair=[prior.point_key(Pa),prior.point_key(Pb)]
                            group_pairs.append(group_pair)
                            if tuple(group_pair[0]) in base and tuple(group_pair[1]) in base:
                                found.append(group_pair)
                witnesses.append({'assignment':assignment,'u_x_bits':u,'v_x_bits':v,
                                  'group_pairs':group_pairs,'base_point_pairs':found})
            status='PASS'
        rows.append({'query_index':index,'archived_query_status':record['archived_query_status'],
                     'residual_point':record['residual_point'],'status':status,
                     'arm_status':{kind:a['status'] for kind,a in pair.items()},
                     'basis_shape':shapes,'root_count':{kind:len(v) for kind,v in roots.items()},
                     'mapped_root_assignments':sorted(roots.get('control',[])),
                     'independent_field_candidates':field_candidates if status=='PASS' else None,
                     'witnesses':witnesses,'metrics':metrics,
                     'wall_seconds_exploratory':{kind:a['wall_ns_exploratory']/1e9 for kind,a in pair.items()}})
    result={'status':'PASS' if all(row['status']=='PASS' for row in rows) else 'PARTIAL',
            'validation_protocol_sha256':sha(HERE/'validation_protocol.json'),'rows':rows}
    (HERE/'validation.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':result['status'],'rows':[(r['query_index'],r['status'],r['root_count']) for r in rows]}))

if __name__=='__main__':main()
