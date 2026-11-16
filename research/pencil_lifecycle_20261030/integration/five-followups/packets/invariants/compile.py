import os,subprocess,json,time,hashlib,pathlib
p=pathlib.Path(__file__).parent
os.chdir(p)
env=dict(os.environ,LEAN_SYSROOT='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain',LEAN_PATH=str(p)+':/var/tmp/independent-same-support-replay-ow8hrtl9:/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/deps')
cmd=['timeout','-k','5s','120s',env['LEAN_SYSROOT']+'/bin/lean','-j1','-M4096','-o','InvariantBranches.olean','InvariantBranches.lean']
t=time.time();r=subprocess.run(cmd,env=env,capture_output=True,text=True)
(p/'stdout').write_text(r.stdout);(p/'stderr').write_text(r.stderr)
(p/'receipt.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,elapsed_seconds=time.time()-t,source_sha256=hashlib.sha256((p/'InvariantBranches.lean').read_bytes()).hexdigest(),lean_path=env['LEAN_PATH']),indent=2))
print(r.returncode,r.stdout,r.stderr)
