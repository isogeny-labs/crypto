#!/usr/bin/env python3
"""Construct matched trace-zero and leaf-sum systems for frozen queries."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def import_file(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def polynomial(eq):
    return '+'.join('1' if term==0 else '*'.join(f'v{j}' for j in range(30) if term>>j&1)
                    for term in sorted(eq,key=lambda value:(value.bit_count(),value)))

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'prepare.py')==protocol['preparer_sha256']
    for name,item in protocol['sources'].items():assert sha(item['path'])==item['sha256'],name
    assert sha(HERE/'sage_runtime_info.json')==protocol['sage_runtime_sha256']
    assert json.loads((HERE/'sage_runtime_info.json').read_text())['status']=='verified'
    ideas=Path(protocol['sources']['boolean_core']['path']).parent
    sys.path.insert(0,str(ideas))
    from boolean_core import ANFField
    from sage_probes import field,curve,from_bits,bits
    sum_builder=import_file(protocol['sources']['sum_builder']['path'],'frozen_sum_builder')
    algebra=json.loads(Path(protocol['sources']['algebra']['path']).read_text())
    relations=json.loads(Path(protocol['sources']['relations']['path']).read_text())
    K,modulus=field(31)
    assert modulus==algebra['field_modulus_bits']
    E=curve(K)
    basis_bits=next(item['basis_bits'] for item in algebra['invariant_subspaces']
                    if item['selection']=='S235')[1:]
    basis=[from_bits(K,value) for value in basis_bits]
    assert all(int(value.trace())==0 for value in basis)
    anf=ANFField(31,modulus)
    u=anf.linear(list(range(15)),basis_bits)
    v=anf.linear(list(range(15,30)),basis_bits)
    variables=','.join(f'v{i}' for i in range(30))
    def systems(qbits):
        q=from_bits(K,qbits)
        assert q and int(q.trace())==int((1/q).trace())==0
        full=anf.s3(u,v,anf.constant(qbits))
        inverse_squared=1/(q*q)
        trace_mask=sum(1<<j for j in range(31)
                       if int((from_bits(K,1<<j)*inverse_squared).trace()))
        assert trace_mask
        relation=set()
        for j,eq in enumerate(full):
            if trace_mask>>j&1:relation.symmetric_difference_update(eq)
        assert not relation
        dropped=(trace_mask&-trace_mask).bit_length()-1
        equations=[eq for j,eq in enumerate(full) if j!=dropped]
        assert len(equations)==30
        base_polys=[polynomial(eq) for eq in equations]
        boolean=[f'v{i}^2+v{i}' for i in range(30)]
        control=variables+'\n2\n'+',\n'.join(base_polys+boolean)+'\n'
        original=[sum_builder.parse(poly) for poly in base_polys+boolean]
        transformed=[sum_builder.transform(poly) for poly in original]
        assert all(sum_builder.transform(transformed[j])==original[j] for j in range(30))
        transformed_boolean=transformed[30:]
        sum_polys=transformed[:30]+transformed_boolean[:15]
        sum_polys.extend(transformed_boolean[j+15]^transformed_boolean[j] for j in range(15))
        assert len(sum_polys)==60
        leaf_sum=variables+'\n2\n'+',\n'.join(sum_builder.serialize(poly) for poly in sum_polys)+'\n'
        return control,leaf_sum,trace_mask,dropped,len(set().union(*equations)),sum(map(len,equations)),sum(map(len,sum_polys[:30]))
    old=systems(551855874)
    assert hashlib.sha256(old[0].encode()).hexdigest()==protocol['sources']['old_control']['sha256']
    assert hashlib.sha256(old[1].encode()).hexdigest()==protocol['sources']['old_sum']['sha256']
    arm=next(item for item in relations['arms'] if item['selection']=='S235')
    records=[];points=set()
    for index in protocol['query_indices']:
        query=arm['query_rows'][index]
        assert query['query_index']==index
        point=query['point_key']
        Q=E(from_bits(K,point[0]),from_bits(K,point[1]))
        singleton_index=protocol['witness_singleton_indices'][str(index)]
        assert singleton_index==query['witness']['indices'][0]
        singleton=arm['base_point_keys'][singleton_index]
        assert singleton==query['witness']['point_keys'][0]
        P=E(from_bits(K,singleton[0]),from_bits(K,singleton[1]))
        R=Q-P
        assert not R.is_zero() and relations['subgroup_order']*R==E(0)
        residual=[bits(R[0]),bits(R[1])]
        assert tuple(residual) not in points
        points.add(tuple(residual))
        control,leaf_sum,mask,dropped,active,control_terms,sum_terms=systems(residual[0])
        folder=HERE/'inputs'/f'q{index}'
        folder.mkdir(parents=True,exist_ok=True)
        control_path=folder/'control.ms';sum_path=folder/'leaf_sum.ms'
        control_path.write_text(control);sum_path.write_text(leaf_sum)
        records.append({'query_index':index,'witness_singleton_index':singleton_index,'query_point':point,'archived_query_status':query['status'],
                        'singleton_point':singleton,'residual_point':residual,
                        'trace_mask':mask,'dropped_equation':dropped,
                        'control_input':str(control_path.relative_to(HERE)),'control_sha256':sha(control_path),
                        'sum_input':str(sum_path.relative_to(HERE)),'sum_sha256':sha(sum_path),
                        'control_s3_terms':control_terms,'sum_s3_terms':sum_terms,
                        'active_equation_monomials':active,'symbolic_inverse':'PASS'})
    result={'status':'PASS','protocol_sha256':sha(HERE/'protocol.json'),
            'old_control_sha256':protocol['sources']['old_control']['sha256'],
            'old_sum_sha256':protocol['sources']['old_sum']['sha256'],
            'records':records}
    (HERE/'preparation.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':'PASS','residuals':[r['residual_point'] for r in records]}))

if __name__=='__main__':main()
