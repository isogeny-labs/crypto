#!/usr/bin/env python3
"""Compile, run and retain the exact even-degree carrier census."""
import hashlib
import json
import subprocess
import time
from pathlib import Path

P=Path(__file__).resolve().parent
source=P/'screen_even.cpp'
binary=P/'screen_even.bin'
target=P/'screen_even.json'
if target.exists():raise SystemExit('refusing to overwrite immutable receipt')
compiler='clang++'
started=time.time()
out={'schema':'ghs-odd5-even-receipt/v1',
     'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
     'runner_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'compiler':subprocess.check_output([compiler,'--version'],text=True).splitlines()[0]}
compile_result=subprocess.run([compiler,'-O3','-std=c++17',str(source),'-o',str(binary)],
                              capture_output=True,text=True)
out['compile']={'exit_code':compile_result.returncode,
                'stdout':compile_result.stdout,'stderr':compile_result.stderr}
if compile_result.returncode==0:
    out['binary_sha256']=hashlib.sha256(binary.read_bytes()).hexdigest()
    try:
        run=subprocess.run([str(binary)],capture_output=True,text=True,timeout=600)
        out['execution']={'exit_code':run.returncode,'stdout':run.stdout,
                          'stderr':run.stderr,'timed_out':False}
        if run.returncode==0:
            out['result']=json.loads(run.stdout)
    except subprocess.TimeoutExpired as exc:
        out['execution']={'exit_code':None,'stdout':(exc.stdout or b'').decode(errors='replace'),
                          'stderr':(exc.stderr or b'').decode(errors='replace'),
                          'timed_out':True}
out['wall_sec']=time.time()-started
target.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({'compile_exit':out['compile']['exit_code'],
                  'execution_exit':out.get('execution',{}).get('exit_code'),
                  'stage_counts':out.get('result',{}).get('stage_counts'),
                  'retained_count':len(out.get('result',{}).get('rows',[])),
                  'wall_sec':out['wall_sec']},indent=2))
