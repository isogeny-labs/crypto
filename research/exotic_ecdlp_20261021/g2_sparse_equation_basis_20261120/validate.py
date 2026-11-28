#!/usr/bin/env python3
"""Replay both reduced bases against frozen S3 equations and curve points."""
import hashlib
import importlib.util
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def load_module(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def parse_metrics(path):
    data=Path(path).read_text()
    def number(pattern):
        match=re.search(pattern,data,re.M)
        assert match,pattern
        return int(match.group(1))
    matrix=re.search(r'max\. matrix data\s+(\d+) x (\d+)',data)
    assert matrix
    return {'rows_reduced':number(r'^#rows reduced\s+(\d+)'),
            'zero_reductions':number(r'^#zero reductions\s+(\d+)'),
            'pairs_reduced':number(r'^#pairs reduced\s+(\d+)'),
            'max_matrix_rows':int(matrix.group(1)),'max_matrix_columns':int(matrix.group(2))}

def main():
    protocol=json.loads((HERE/'validation_protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'validate.py')==protocol['validator_sha256']
    for name,item in protocol['sources'].items():assert sha(item['path'])==item['sha256'],name
    prior=load_module(Path(protocol['sources']['prior_validator']['path']),'frozen_prior_validator')
    builder=load_module(HERE/'build.py','sparse_builder')
    receipt=json.loads((HERE/'run_receipt.json').read_text())
    assert receipt['status']=='COMPLETED' and len(receipt['arms'])==2
    assert [arm['status'] for arm in receipt['arms']]==['COMPLETED','COMPLETED']
    for arm in receipt['arms']:
        for item in arm['artifacts'].values():
            path=HERE/item['file'];assert path.stat().st_size==item['bytes'] and sha(path)==item['sha256']
    control=HERE/'control_basis.txt';sparse=HERE/'sparsified_basis.txt'
    assert sha(control)==sha(sparse)
    _,_,control_roots=prior.parse_basis(control)
    _,_,sparse_roots=prior.parse_basis(sparse)
    assert control_roots==sparse_roots and len(control_roots)==2
    source=Path(protocol['sources']['control_input']['path']).read_text().strip().splitlines()
    original=[builder.parse(poly) for poly in '\n'.join(source[2:]).split(',\n')[:30]]
    transformed=[builder.parse(poly) for poly in '\n'.join((HERE/'sparsified.ms').read_text().strip().splitlines()[2:]).split(',\n')[:30]]
    build=json.loads((HERE/'build_result.json').read_text())
    inverse=[set(eq) for eq in transformed]
    for op in reversed(build['operations']):inverse[op['row']]^=inverse[op['add_row']]
    assert inverse==original
    algebra=json.loads(Path(protocol['sources']['algebra']['path']).read_text())
    relations=json.loads(Path(protocol['sources']['relations']['path']).read_text())
    basis=next(item['basis_bits'] for item in algebra['invariant_subspaces']
               if item['selection']=='S235')[1:]
    K,modulus=prior.field(31)
    assert modulus==algebra['field_modulus_bits']
    E=prior.curve(K)
    vals,coordinates=prior.values_and_coordinates(basis)
    arm=next(item for item in relations['arms'] if item['selection']=='S235')
    base={tuple(point) for point in arm['base_point_keys']}
    query=arm['query_rows'][1]
    Q=E(prior.from_bits(K,query['point_key'][0]),prior.from_bits(K,query['point_key'][1]))
    singleton=arm['base_point_keys'][2]
    P=E(prior.from_bits(K,singleton[0]),prior.from_bits(K,singleton[1]))
    R=Q-P
    assert prior.point_key(R)==[551855874,2085658254]
    recovered=[]
    for assignment in control_roots:
        assert all(sum((term&assignment)==term for term in eq)&1==0 for eq in original+transformed)
        u_mask=assignment&((1<<15)-1);v_mask=assignment>>15
        u=vals[u_mask];v=vals[v_mask]
        assert coordinates(u)==u_mask and coordinates(v)==v_mask
        witnesses=[]
        for Pa in E.lift_x(prior.from_bits(K,u),all=True):
            for Pb in E.lift_x(prior.from_bits(K,v),all=True):
                if Pa+Pb==R and tuple(prior.point_key(Pa)) in base and tuple(prior.point_key(Pb)) in base:
                    assert P+Pa+Pb==Q
                    witnesses.append([prior.point_key(Pa),prior.point_key(Pb)])
        assert witnesses
        recovered.append({'assignment_bits':assignment,'u_x_bits':u,'v_x_bits':v,
                          'group_pair_witnesses':witnesses})
    result={'status':'PASS','protocol_sha256':sha(HERE/'validation_protocol.json'),
            'basis_sha256':sha(control),'roots':recovered,
            'metrics':{arm['name']:parse_metrics(HERE/arm['artifacts']['stderr']['file'])
                       for arm in receipt['arms']}}
    (HERE/'validation.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':'PASS','roots':len(recovered),'basis_sha256':sha(control)}))

if __name__=='__main__':main()
