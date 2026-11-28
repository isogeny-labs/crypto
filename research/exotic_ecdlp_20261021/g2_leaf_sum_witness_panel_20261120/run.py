#!/usr/bin/env python3
"""Execute the frozen five-residual alternating msolve panel."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'run.py')==protocol['runner_sha256']
    for name,item in protocol['sources'].items():assert sha(item['path'])==item['sha256'],name
    assert sha(HERE/'sage_runtime_info.json')==protocol['sage_runtime_sha256']
    assert json.loads((HERE/'sage_runtime_info.json').read_text())['status']=='verified'
    assert sha(protocol['msolve']['path'])==protocol['msolve']['sha256']
    assert sha(protocol['flint']['library_path'])==protocol['flint']['sha256']
    preparation=json.loads((HERE/'preparation.json').read_text())
    assert preparation['status']=='PASS' and preparation['protocol_sha256']==sha(HERE/'protocol.json')
    assert [r['query_index'] for r in preparation['records']]==protocol['query_indices']
    spec=importlib.util.spec_from_file_location('frozen_monitor',protocol['sources']['solver_monitor']['path'])
    monitor=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(monitor)
    run_dir=HERE/'runs';run_dir.mkdir(exist_ok=True)
    monitor.HERE=run_dir
    receipt_path=HERE/'panel_run.json'
    if receipt_path.exists():
        result=json.loads(receipt_path.read_text())
        assert result['protocol_sha256']==sha(HERE/'protocol.json')
    else:
        result={'status':'IN_PROGRESS','protocol_sha256':sha(HERE/'protocol.json'),
                'preparation_sha256':sha(HERE/'preparation.json'),'arms':[]}
    schedule=[]
    for row in preparation['records']:
        index=row['query_index']
        kinds=('control','leaf_sum') if index%2==0 else ('leaf_sum','control')
        for kind in kinds:schedule.append((index,kind,row))
    completed=[(a['query_index'],a['kind']) for a in result['arms']]
    assert completed==[(i,k) for i,k,_ in schedule[:len(completed)]]
    for index,kind,row in schedule[len(completed):]:
        field='control_input' if kind=='control' else 'sum_input'
        digest='control_sha256' if kind=='control' else 'sum_sha256'
        input_path=HERE/row[field]
        assert sha(input_path)==row[digest]
        name=f'q{index}_{kind}'
        arm=monitor.one_arm(name,input_path,protocol)
        arm.update({'query_index':index,'kind':kind,'residual_point':row['residual_point']})
        result['arms'].append(arm)
        receipt_path.write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
        print(json.dumps({'query_index':index,'kind':kind,'status':arm['status'],
                          'wall_seconds':arm['wall_ns_exploratory']/1e9}),flush=True)
    result['status']='COMPLETED'
    receipt_path.write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')

if __name__=='__main__':main()
