#!/usr/bin/env python3
"""Exhaustively compare the frozen H4 n13 input and returned Boolean bases."""
import hashlib
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def polynomial_masks(poly):
    masks=set()
    for term in poly.split('+'):
        mask=0
        for factor in term.strip().split('*'):
            if factor=='1':continue
            match=re.fullmatch(r'v(\d+)(?:\^(\d+))?',factor.strip())
            assert match,factor
            index=int(match.group(1));assert 0<=index<15
            mask|=1<<index
        masks.symmetric_difference_update((mask,))
    return masks

def input_polynomials(path):
    lines=path.read_text().strip().splitlines()
    assert lines[:2]==[','.join(f'v{i}' for i in range(15)),'2']
    polys='\n'.join(lines[2:]).split(',\n')
    assert len(polys)==28
    return [polynomial_masks(poly) for poly in polys]

def basis_polynomials(path):
    content=path.read_text()
    assert '#field characteristic: 2' in content
    match=re.search(r'\[(.*?)\]:',content,re.S)
    assert match
    polys=[part.strip() for part in match.group(1).split(',')]
    return [polynomial_masks(poly) for poly in polys]

def vanishes(polys,assignment):
    return all(sum((mask&assignment)==mask for mask in poly)%2==0 for poly in polys)

def metrics(path):
    data=path.read_text()
    def number(pattern):
        m=re.search(pattern,data,re.M)
        assert m,pattern
        return int(m.group(1))
    matrix=re.search(r'max\. matrix data\s+(\d+) x (\d+)',data)
    assert matrix
    rounds=[int(m.group(1)) for m in re.finditer(r'^\s*(\d+)\s+\d+\s+\d+\s+\d+ x \d+',data,re.M)]
    return {'rows_reduced':number(r'^#rows reduced\s+(\d+)'),
            'pairs_reduced':number(r'^#pairs reduced\s+(\d+)'),
            'max_matrix_rows':int(matrix.group(1)),'max_matrix_columns':int(matrix.group(2)),
            'selected_pair_degrees':rounds,'max_selected_pair_degree':max(rounds) if rounds else None}

def main():
    protocol=json.loads((HERE/'validation-protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'validate.py')==protocol['validator_sha256']
    for item in protocol['sources'].values():assert sha(item['path'])==item['sha256']
    prep=json.loads((HERE/'preparation.json').read_text())
    run=json.loads((HERE/'run-receipt.json').read_text())
    assert prep['status']=='PASS' and run['status']=='COMPLETED'
    assert run['preparation_sha256']==sha(HERE/'preparation.json')
    assert [a['arm'] for a in run['arms']]==['planted','random']
    rows=[]
    for rec,arm in zip(prep['records'],run['arms']):
        assert rec['arm']==arm['arm']
        input_path=HERE/rec['input_file']
        assert sha(input_path)==rec['input_sha256']==arm['input_sha256']
        for item in arm['artifacts'].values():
            path=HERE/'runs'/item['file']
            assert path.stat().st_size==item['bytes'] and sha(path)==item['sha256']
        original=input_polynomials(input_path)
        original_roots=[x for x in range(1<<15) if vanishes(original,x)]
        assert original_roots==sorted(rec['exact_roots'])
        row={'arm':rec['arm'],'status':arm['status'],'original_root_count':len(original_roots),
             'wall_seconds_exploratory':arm['wall_ns_exploratory']/1e9,
             'peak_process_tree_rss_bytes':arm['peak_process_tree_rss_bytes']}
        if arm['status']=='COMPLETED':
            assert arm['exit_code']==0
            basis=basis_polynomials(HERE/'runs'/arm['artifacts']['basis']['file'])
            basis_roots=[x for x in range(1<<15) if vanishes(basis,x)]
            assert basis_roots==original_roots
            row.update({'basis_root_count':len(basis_roots),'basis_length':len(basis),
                        'basis_max_boolean_degree':max(mask.bit_count() for poly in basis for mask in poly),
                        'solver_metrics':metrics(HERE/'runs'/arm['artifacts']['stderr']['file']),
                        'validation':'PASS'})
        else:row['validation']='CENSORED'
        rows.append(row)
    result={'status':'PASS' if all(row['validation']=='PASS' for row in rows) else 'PARTIAL',
            'validation_protocol_sha256':sha(HERE/'validation-protocol.json'),'rows':rows}
    (HERE/'validation.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
    print(json.dumps({'status':result['status'],'rows':[(x['arm'],x['status'],x['original_root_count']) for x in rows]}))

if __name__=='__main__':main()
