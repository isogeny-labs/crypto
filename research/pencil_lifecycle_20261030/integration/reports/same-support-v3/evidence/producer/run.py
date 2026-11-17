import os,sys,pathlib,subprocess,time,json,hashlib
p=pathlib.Path(__file__).parent
mirror=pathlib.Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r')
name=sys.argv[1]
src=p/(name+'.lean')
h=lambda x:hashlib.sha256(x.read_bytes()).hexdigest()
env=os.environ.copy();env['LEAN_SYSROOT']=str(mirror/'toolchain');env['LEAN_PATH']=str(p)+':/var/tmp/independent-same-support-replay-ow8hrtl9:'+str(mirror/'deps')
cmd=['timeout','-k','5s','120s',str(mirror/'toolchain/bin/lean'),'-j1','-M4096','-o',name+'.olean',name+'.lean']
record={'command':cmd,'cwd':str(p),'source_sha256':h(src),'plan_sha256':h(p/'PLAN.md'),'inputs_sha256':h(p/'inputs.json'),'start_unix':time.time()}
(p/(name+'.command.json')).write_text(json.dumps(record,indent=2))
with (p/(name+'.stdout')).open('xb') as out,(p/(name+'.stderr')).open('xb') as err:
 t=time.monotonic();r=subprocess.run(cmd,cwd=p,env=env,stdout=out,stderr=err);record.update(exit_code=r.returncode,elapsed_seconds=time.monotonic()-t)
record['outputs']={x.name:h(x) for x in [p/(name+'.stdout'),p/(name+'.stderr'),p/(name+'.olean')] if x.exists()}
(p/(name+'.receipt.json')).write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record,indent=2));print((p/(name+'.stdout')).read_text());print((p/(name+'.stderr')).read_text())
