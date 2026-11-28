#!/usr/bin/env python3
"""Run the two frozen H4 n13 Gröbner arms under process-tree caps."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'run.py')==protocol['runner_sha256']
    assert sha(protocol['solver_monitor']['path'])==protocol['solver_monitor']['sha256']
    assert sha(protocol['msolve']['path'])==protocol['msolve']['sha256']
    assert sha(protocol['flint']['library_path'])==protocol['flint']['sha256']
    prep=json.loads((HERE/'preparation.json').read_text())
    assert prep['status']=='PASS' and prep['protocol_sha256']==sha(HERE/'protocol.json')
    assert [r['arm'] for r in prep['records']]==protocol['arms']
    spec=importlib.util.spec_from_file_location('pinned_monitor',protocol['solver_monitor']['path'])
    monitor=importlib.util.module_from_spec(spec);spec.loader.exec_module(monitor)
    monitor.HERE=HERE/'runs';monitor.HERE.mkdir(exist_ok=True)
    receipt=HERE/'run-receipt.json'
    if receipt.exists():
        result=json.loads(receipt.read_text())
        assert result['protocol_sha256']==sha(HERE/'protocol.json')
    else:result={'status':'IN_PROGRESS','protocol_sha256':sha(HERE/'protocol.json'),
                 'preparation_sha256':sha(HERE/'preparation.json'),'arms':[]}
    assert [a['arm'] for a in result['arms']]==protocol['arms'][:len(result['arms'])]
    for row in prep['records'][len(result['arms']):]:
        input_path=HERE/row['input_file']
        assert sha(input_path)==row['input_sha256']
        arm=monitor.one_arm(row['arm'],input_path,protocol)
        arm['arm']=row['arm']
        result['arms'].append(arm)
        receipt.write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
        print(json.dumps({'arm':row['arm'],'status':arm['status'],
                          'wall_seconds':arm['wall_ns_exploratory']/1e9}),flush=True)
    result['status']='COMPLETED'
    receipt.write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')

if __name__=='__main__':main()
