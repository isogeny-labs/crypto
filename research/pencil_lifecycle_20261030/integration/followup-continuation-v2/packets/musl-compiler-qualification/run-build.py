from pathlib import Path
import subprocess,time,json,hashlib
D=Path(__file__).parent;cmd=['timeout','-k','5s','600s','bash',str(D/'build.sh')];env={'PATH':'/usr/bin:/bin','LC_ALL':'C','TMPDIR':'/tmp'};t=time.time()
with (D/'build.stdout').open('w')as out,(D/'build.stderr').open('w')as err:r=subprocess.run(cmd,env=env,stdout=out,stderr=err)
(D/'build-receipt.json').write_text(json.dumps({'command':cmd,'environment':env,'exit_code':r.returncode,'start':t,'end':time.time(),'source_verification_sha256':hashlib.sha256((D/'source-verification.json').read_bytes()).hexdigest(),'build_script_sha256':hashlib.sha256((D/'build.sh').read_bytes()).hexdigest()},indent=2));print(r.returncode);print((D/'build.stderr').read_text()[-1000:])
