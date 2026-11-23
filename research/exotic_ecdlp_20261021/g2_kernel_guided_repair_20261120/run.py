#!/usr/bin/env python3
"""Compile the frozen kernel repair and verify its selected columns."""
import hashlib
import importlib.util
import json
import subprocess
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def main():
    protocol = json.loads((HERE/'protocol.json').read_text())
    assert protocol['status'] == 'frozen'
    assert sha(HERE/'run.py') == protocol['runner_sha256']
    assert sha(HERE/'repair.cpp') == protocol['repair_source_sha256']
    for name, item in protocol['sources'].items():
        assert sha(item['path']) == item['sha256'], name
    panel = json.loads(Path(protocol['sources']['panel']['path']).read_text())
    base = panel['information_set_columns']
    assert len(base) == 15856 and base == sorted(set(base))
    projection_path = Path(protocol['sources']['projector']['path'])
    spec = importlib.util.spec_from_file_location('frozen_g2_projection', projection_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    result = {'status':'PASS','protocol_sha256':sha(HERE/'protocol.json'),'cases':[]}
    with tempfile.TemporaryDirectory(prefix='g2-kernel-repair-') as directory:
        tmp = Path(directory)
        binary = tmp/'repair'
        compile_command = ['clang++','-O2','-std=c++17',
                           '-I/private/var/tmp/sage-10.9-current/local/include',
                           str(HERE/'repair.cpp'),
                           '-L/private/var/tmp/sage-10.9-current/local/lib','-lm4ri','-o',str(binary)]
        subprocess.run(compile_command,check=True,capture_output=True,text=True)
        columns = tmp/'columns.txt'
        columns.write_text(''.join(f'{col}\n' for col in base))
        for label in ('fresh_1','fresh_15'):
            source = protocol['sources'][label]['path']
            proc = subprocess.run([str(binary),source,str(columns)],check=True,
                                  capture_output=True,text=True,timeout=120)
            found = json.loads(proc.stdout)
            assert found['status']=='PASS' and found['quotient_rank']==6
            assert found['peak_rss_bytes_process'] < protocol['caps']['peak_rss_bytes_per_case']
            chosen = found['selected_columns']
            assert len(chosen)==6 and not set(chosen).intersection(base)
            projected = tmp/'projected.bin'
            selected = sorted(base+chosen)
            projection_started = time.monotonic_ns()
            nnz = module.project(source,projected,selected)
            projection_ns = time.monotonic_ns() - projection_started
            check = json.loads(subprocess.run([protocol['sources']['ranker']['path'],str(projected)],
                                              check=True,capture_output=True,text=True,timeout=30).stdout)
            assert check['rank']==15841 and check['nnz']==nnz
            assert check['maxrss_bytes_process'] < protocol['caps']['peak_rss_bytes_per_case']
            found.update({'label':label,'q_bits':next(c['q_bits'] for c in panel['cases'] if c['label']==label),
                          'verified_projection_columns':len(selected),
                          'verified_projection_nnz':nnz,
                          'verified_projection_ns_exploratory':projection_ns,
                          'verified_projection_sha256':sha(projected),
                          'independent_pluq_rank':check['rank'],
                          'independent_pluq_peak_rss_bytes_process':check['maxrss_bytes_process'],
                          'independent_pluq_wall_ns_exploratory':check['wall_ns_exploratory']})
            result['cases'].append(found)
            projected.unlink()
    print(json.dumps(result,sort_keys=True,indent=2))

if __name__=='__main__':
    main()
