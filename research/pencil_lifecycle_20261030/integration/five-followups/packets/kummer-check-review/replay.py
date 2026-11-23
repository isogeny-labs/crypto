from pathlib import Path
import os,subprocess,time,json,hashlib
p=Path(__file__).parent;cmd=['timeout','-k','5s','120s','cargo','run','--locked','--offline','--bin','equivalence'];t=time.time();r=subprocess.run(cmd,cwd=p/'crate',env=dict(os.environ,CARGO_TARGET_DIR=str(p/'target')),capture_output=True,text=True);(p/'replay.stdout').write_text(r.stdout);(p/'replay.stderr').write_text(r.stderr);(p/'replay.receipt.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,elapsed_seconds=time.time()-t),indent=2));print(r.returncode);print(r.stderr[-1000:]);
if r.returncode==0:
 old=Path('/var/tmp/pencil-five-leads-kummer-check-20261030/run2.stdout').read_bytes();print('stdout identical',r.stdout.encode()==old)
