#!/usr/bin/env python3
"""Execute candidate then same-binary control using the frozen monitor."""
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
    runtime=json.loads((HERE/'sage_runtime_info.json').read_text())
    assert runtime['status']=='verified' and sha(HERE/'sage_runtime_info.json')==protocol['sage_runtime_sha256']
    spec=importlib.util.spec_from_file_location('frozen_msolve_monitor',protocol['sources']['monitor']['path'])
    monitor=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(monitor)
    monitor.HERE=HERE
    assert sha(protocol['msolve']['path'])==protocol['msolve']['sha256']
    assert sha(protocol['flint']['library_path'])==protocol['flint']['sha256']
    previous=json.loads(Path(protocol['sources']['control_before']['path']).read_text())
    assert previous['status']=='COMPLETED' and previous['arms'][0]['status']=='COMPLETED'
    record={'status':'IN_PROGRESS','protocol_sha256':sha(HERE/'protocol.json'),
            'control_before_sha256':sha(protocol['sources']['control_before']['path']),
            'arms':[]}
    for name,path in [('sum_coordinate',protocol['sources']['sum_input']['path']),
                      ('control_after',protocol['sources']['control_input']['path'])]:
        arm=monitor.one_arm(name,Path(path),protocol)
        record['arms'].append(arm)
        (HERE/'run_receipt.json').write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')
        print(json.dumps({'arm':name,'status':arm['status'],
                          'wall_seconds':arm['wall_ns_exploratory']/1e9}),flush=True)
    record['status']='COMPLETED'
    (HERE/'run_receipt.json').write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')

if __name__=='__main__':main()
