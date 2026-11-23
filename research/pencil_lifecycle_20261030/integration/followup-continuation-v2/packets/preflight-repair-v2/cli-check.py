from pathlib import Path
import json,subprocess,os,time,hashlib
p=Path(__file__).parent;v=json.loads((p/'inventory-inputs.json').read_text());req={'schema':'pencil.static_inventory_request/v1','stage':'static_inventory_only','routes':[{'route':x['name'],'plan':{'path':x['path'],'sha256':x['sha256']},'spec':{'path':x['spec_path'],'sha256':x['spec_sha256']}} for x in v['routes']],'artifacts':[{'name':k,'file':x} for k,x in v['artifacts'].items()]};(p/'inventory-request.json').write_text(json.dumps(req,indent=2));cmd=['timeout','-k','5s','120s','cargo','build','--offline','--bin','inventory','-j','1'];r=subprocess.run(cmd,cwd=p,capture_output=True,text=True);(p/'cli-build.stdout').write_text(r.stdout);(p/'cli-build.stderr').write_text(r.stderr);assert r.returncode==0,r.stderr
results=[]
def run(name,path,h,expected):
 r=subprocess.run(['timeout','5s',str(p/'target/debug/inventory'),str(path),h],capture_output=True,text=True);results.append(dict(name=name,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr));assert r.returncode==expected,(name,r.stdout,r.stderr)
f=p/'inventory-request.json';h=hashlib.sha256(f.read_bytes()).hexdigest();run('actual nine immutable input inventory',f,h,0);run('wrong request hash',f,'0'*64,2)
for name,mut in [('all_missing',lambda x:x.update(routes=[])),('wrong_stage',lambda x:x.update(stage='actual_session_admission')),('unknown_field',lambda x:x.update(admit=True)),('duplicate_route',lambda x:x['routes'].__setitem__(8,x['routes'][0])),('certificate_bad_hash',lambda x:x['artifacts'][0]['file'].update(sha256='0'*64))]:
 x=json.loads(json.dumps(req));mut(x);f=p/(name+'.json');f.write_text(json.dumps(x));run(name,f,hashlib.sha256(f.read_bytes()).hexdigest(),2)
link=p/'request-symlink.json';link.symlink_to(p/'inventory-request.json');run('symlink request',link,h,2)
(p/'cli-results.json').write_text(json.dumps(results,indent=2));print('8 CLI checks passed')
