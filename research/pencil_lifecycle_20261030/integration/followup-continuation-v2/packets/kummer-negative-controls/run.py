from pathlib import Path
import subprocess,json,time,hashlib,os
D=Path(__file__).parent;cmd=['timeout','-k','5s','120s','cargo','run','--locked','--offline','--manifest-path',str(D/'Cargo.toml'),'--bin','equivalence'];env=os.environ.copy();env['CARGO_TARGET_DIR']=str(D/'target');t=time.time()
with (D/'run.stdout').open('w') as out,(D/'run.stderr').open('w') as err:r=subprocess.run(cmd,cwd=D,env=env,stdout=out,stderr=err)
(D/'receipt.json').write_text(json.dumps({'command':cmd,'exit_code':r.returncode,'start':t,'end':time.time(),'preexecution_manifest_sha256':hashlib.sha256((D/'pre-execution-manifest.json').read_bytes()).hexdigest()},indent=2));print(r.returncode);print((D/'run.stderr').read_text()[-1800:])
