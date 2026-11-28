#!/usr/bin/env python3
"""Run matched fresh msolve arms with bounded records."""
import hashlib
import json
import os
import shutil
import signal
import subprocess
import tempfile
import time
from pathlib import Path
import psutil

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def one_arm(name,input_path,protocol):
    wall_cap=protocol['wall_cap_seconds_per_arm']
    rss_gate=protocol['observed_peak_rss_gate_bytes']
    output_cap=protocol['combined_output_cap_bytes']
    with tempfile.TemporaryDirectory(prefix=f'g2-sparse-{name}-') as directory:
        tmp=Path(directory)
        out=tmp/'msolve.out';stdout=tmp/'stdout.txt';stderr=tmp/'stderr.txt'
        command=[protocol['msolve']['path'],*protocol['flags'],'-f',str(input_path),'-o',str(out)]
        environment=os.environ.copy()
        environment['DYLD_LIBRARY_PATH']=protocol['flint']['library_directory']
        start=time.monotonic_ns()
        with stdout.open('wb') as sout,stderr.open('wb') as serr:
            process=subprocess.Popen(command,stdout=sout,stderr=serr,start_new_session=True,env=environment)
            peak=0;samples=0;reason=None
            while process.poll() is None:
                try:
                    root=psutil.Process(process.pid)
                    members=[root,*root.children(recursive=True)]
                    rss=sum(p.memory_info().rss for p in members if p.is_running())
                    peak=max(peak,rss)
                except psutil.Error:pass
                samples+=1
                if (time.monotonic_ns()-start)/1e9>wall_cap:reason='WALL_CAP'
                if peak>rss_gate:reason='RSS_CAP'
                if sum(p.stat().st_size for p in (out,stdout,stderr) if p.exists())>output_cap:reason='OUTPUT_CAP'
                if reason:
                    os.killpg(process.pid,signal.SIGTERM)
                    try:process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        os.killpg(process.pid,signal.SIGKILL)
                        process.wait()
                    break
                time.sleep(0.1)
            exit_code=process.wait()
        wall=time.monotonic_ns()-start
        status=reason or ('COMPLETED' if exit_code==0 and out.exists() else 'SOLVER_FAILURE')
        artifacts={}
        for key,path in [('stdout',stdout),('stderr',stderr),('basis',out)]:
            if path.exists():
                dest=HERE/f'{name}_{key}.txt'
                shutil.copyfile(path,dest)
                artifacts[key]={'file':dest.name,'bytes':dest.stat().st_size,'sha256':sha(dest)}
        return {'name':name,'status':status,'command':command,
                'dyld_library_path':environment['DYLD_LIBRARY_PATH'],
                'input_sha256':sha(input_path),
                'exit_code':exit_code,'wall_ns_exploratory':wall,
                'peak_process_tree_rss_bytes':peak,'rss_samples':samples,'artifacts':artifacts}

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'run_solver.py')==protocol['runner_sha256']
    assert sha(HERE/'sage_runtime_info.json')==protocol['sage_runtime_sha256']
    assert json.loads((HERE/'sage_runtime_info.json').read_text())['status']=='verified'
    assert sha(protocol['msolve']['path'])==protocol['msolve']['sha256']
    assert sha(protocol['flint']['library_path'])==protocol['flint']['sha256']
    control=Path(protocol['control_input']['path'])
    assert sha(control)==protocol['control_input']['sha256']
    build=json.loads((HERE/'build_result.json').read_text())
    transformed=HERE/'sparsified.ms'
    assert build['status']=='PASS' and sha(transformed)==build['output_sha256']
    result={'status':'IN_PROGRESS','protocol_sha256':sha(HERE/'protocol.json'),
            'msolve_sha256':protocol['msolve']['sha256'],'arms':[]}
    for name,path in [('control',control),('sparsified',transformed)]:
        result['arms'].append(one_arm(name,path,protocol))
        (HERE/'run_receipt.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
        print(json.dumps({'arm':name,'status':result['arms'][-1]['status'],
                          'wall_seconds':result['arms'][-1]['wall_ns_exploratory']/1e9}),flush=True)
    result['status']='COMPLETED'
    (HERE/'run_receipt.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')

if __name__=='__main__':main()
