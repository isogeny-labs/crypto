import pathlib, subprocess,json,os,datetime,sys
root=pathlib.Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');os.chdir(root)
out=pathlib.Path('/var/tmp/pencil-current-base-validation-20261031-v3')
env=dict(os.environ,CARGO_NET_OFFLINE='true',CARGO_BUILD_JOBS='2',GIT_OPTIONAL_LOCKS='0')
commands=[('clippy',['bash','scripts/clippy-maintained.sh']),('examples',['cargo','test','--offline','--locked','--release','-j','2','--example','fb_coverage','--example','koblitz_s3_root_batch_experiment','--example','n53_s3_root_batch_report','--example','n53_s3_root_batch_run'])]
head=subprocess.check_output(['git','write-tree']).decode().strip()
for label,cmd in commands:
 d=out/label;d.mkdir(exist_ok=False)
 (d/'command.json').write_text(json.dumps({'index_tree':head,'argv':cmd,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scientific_execution':False},indent=2))
 with (d/'stdout').open('w') as so,(d/'stderr').open('w') as se:r=subprocess.run(['timeout','-k','5s','3600s']+cmd,env=env,stdout=so,stderr=se)
 (d/'receipt.json').write_text(json.dumps({'exit_code':r.returncode,'index_tree_unchanged':head==subprocess.check_output(['git','write-tree']).decode().strip(),'ended_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()},indent=2))
 print(label,r.returncode,flush=True)
 if r.returncode:sys.exit(r.returncode)
