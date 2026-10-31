import os,json,hashlib,subprocess,time,pathlib
root=pathlib.Path(__file__).parent
lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean'
env=os.environ.copy();env['LEAN_SYSROOT']=str(pathlib.Path(lean).parent.parent);env['LEAN_PATH']=str(root)+':/var/tmp/independent-same-support-replay-ow8hrtl9:/var/tmp/pencil-word-refinement-v1/dependency-union'
for name in ['Controls']:
 d=root/(name+'-run1');d.mkdir(exist_ok=False)
 cmd=['timeout','-k','5s','120s',lean,'-j1','-M4096','-o',str(root/(name+'.olean')),str(root/(name+'.lean'))]
 receipt={'command':cmd,'start':time.time(),'source_sha256':hashlib.sha256((root/(name+'.lean')).read_bytes()).hexdigest(),'LEAN_PATH':env['LEAN_PATH']}
 with (d/'stdout').open('w') as out,(d/'stderr').open('w') as err:r=subprocess.run(cmd,cwd=root,env=env,stdout=out,stderr=err)
 receipt.update(exit_code=r.returncode,end=time.time());(d/'receipt.json').write_text(json.dumps(receipt,indent=2));print(name,r.returncode);print((d/'stdout').read_text());print((d/'stderr').read_text())
 if r.returncode:break
