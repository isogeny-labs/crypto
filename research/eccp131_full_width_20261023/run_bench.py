#!/usr/bin/env python3
"""Run the nine predeclared benchmark cells once, in fixed order."""
import json
import subprocess
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
root = here.parents[1]
cases = [
    (0,'tower'),(0,'small_a'),(0,'small_b'),
    (1,'small_a'),(1,'tower'),(1,'small_b'),
    (2,'small_a'),(2,'small_b'),(2,'tower'),
]
records = []
for rep,arm in cases:
    stem = here/'runs'/f'bench-r{rep}-{arm}'
    cmd = [sys.executable,str(here/'run_bounded.py'),
           '--wall-seconds','30','--rss-mib','2048',
           '--stdout',str(stem)+'.json','--stderr',str(stem)+'.stderr',
           '--receipt',str(stem)+'.receipt.json','--',sys.executable,
           str(here/'bench_screen.py'),'--rep',str(rep),'--arm',arm]
    result = subprocess.run(cmd,cwd=root,capture_output=True,text=True)
    records.append({'rep':rep,'arm':arm,'command':cmd,'exit_code':result.returncode,
                    'driver_stdout':result.stdout,'driver_stderr':result.stderr})
(here/'runs/bench-driver.json').write_text(json.dumps(records,indent=2)+'\n')
if any(r['exit_code'] != 0 for r in records):
    raise SystemExit(1)
