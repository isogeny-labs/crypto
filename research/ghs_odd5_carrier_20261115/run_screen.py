#!/usr/bin/env python3
"""Compile and run the frozen exhaustive F3 genus-five model screen."""
import hashlib,json,subprocess,time
from pathlib import Path
P=Path(__file__).resolve().parent
src=P/'screen.cpp'; target=P/'screen.json'
if target.exists():raise SystemExit('refusing to overwrite immutable screen receipt')
compiler='clang++'
version=subprocess.check_output([compiler,'--version'],text=True).splitlines()[0]
started=time.time()
compile_result=subprocess.run([compiler,'-O3','-std=c++17',str(src),'-o',str(P/'screen.bin')],capture_output=True,text=True)
if compile_result.returncode:raise SystemExit(compile_result.stderr)
result=subprocess.run([str(P/'screen.bin')],capture_output=True,text=True,timeout=600)
if result.returncode:raise SystemExit(result.stderr)
d=json.loads(result.stdout)
d.update({'compiler':version,'screen_cpp_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'runner_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'wall_sec':time.time()-started})
target.write_text(json.dumps(d,indent=2)+'\n')
print(json.dumps({'stage_counts':d['stage_counts'],'retained_count':len(d['rows']),'first_rows':d['rows'][:10],'wall_sec':d['wall_sec']},indent=2))
