"""Read-only reconciliation for the single retained assembly; never launches workloads."""
import hashlib,json,os,stat,subprocess,time
from pathlib import Path
OWNER='2d885f49-d570-4a8b-990e-7ea14ee9ea80'
ROOT=Path('/var/tmp/ppr-'+OWNER)
BINDING='a1dfe56b2a490e3e63290540ccdf45ff547d1d9e6555f08dcac5f2c34ecd307a'
SOURCE='f2c638ed4ec07509a3254632fb7831e009559667b6a82eacb9fb5ea31ac9cb66'
UNIT='pencil-private-'+OWNER+'.service'
GROUP='/system.slice/'+UNIT

def need(ok,msg):
 if not ok:raise ValueError(msg)
def sha(raw):return hashlib.sha256(raw).hexdigest()
def strict(raw):
 def pairs(xs):
  d={}
  for k,v in xs:need(k not in d,'duplicate key');d[k]=v
  return d
 return json.loads(raw,object_pairs_hook=pairs)
def canonical(x):return json.dumps(x,sort_keys=True,separators=(',',':')).encode()
def custody(p):
 p=Path(p)
 for q in [p,*p.parents]:need(not q.is_symlink(),'symlink')
 s=p.stat();need(stat.S_ISREG(s.st_mode) and s.st_uid==0 and s.st_nlink==1 and stat.S_IMODE(s.st_mode)==0o600,'file custody')
 return {'device':s.st_dev,'inode':s.st_ino,'bytes':s.st_size,'mtime_ns':s.st_mtime_ns}
def read(p,mode=0o600):
 p=Path(p)
 for q in [p,*p.parents]:need(not q.is_symlink(),'symlink')
 s=p.stat();need(stat.S_ISREG(s.st_mode) and s.st_uid==0 and s.st_nlink==1 and stat.S_IMODE(s.st_mode)==mode and s.st_size<=1024*1024,'input custody')
 return p.read_bytes()
def loops_empty(raw):
 x=strict(raw);need(isinstance(x,dict) and set(x)=={'loopdevices'} and x['loopdevices']==[],'loop attachment/shape unresolved');return True
def stopped(props,description,invocation):
 if props.get('LoadState')=='not-found':
  need(props.get('ActiveState')=='inactive' and props.get('SubState')=='dead' and props.get('ControlGroup','')=='','collected state unresolved')
 else:
  need(props.get('LoadState')=='loaded' and props.get('ActiveState')=='inactive' and props.get('SubState')=='dead' and props.get('Description')==description and props.get('InvocationID') in ('',invocation),'owned unit stop unresolved')
 return True
def saved_ok(x,before,worker):
 desc='pencil-private-'+OWNER+'-'+sha(canonical(x))
 need(x['owner']==OWNER and x['mode']=='assembly','phase/owner')
 need(before.get('Description')==desc and before.get('ActiveState')=='active' and before.get('SubState')=='exited' and before.get('Result')=='success' and before.get('ExecMainStatus')=='0','completed snapshot')
 need(worker.get('owner')==OWNER and worker.get('mode')=='assembly' and worker.get('status')=='worker_phase_complete' and worker.get('service_invocation_id')==before.get('InvocationID') and worker.get('unit')==UNIT and worker.get('service_cgroup')==GROUP,'worker identity')
 need(all(worker.get(k) is False for k in ('mounted','shim_mounted','shim_uncertain','endpoint_live')),'worker teardown unresolved')
 need(worker.get('disk_path')==str(ROOT/'storage.ext4') and worker.get('disk_custody',{}).get('bytes')==4*1024**3,'disk identity')
 return desc

def main():
 need(os.geteuid()==0,'read-only root custody required');started=time.monotonic();deadline=started+30
 def guard():need(time.monotonic()<deadline,'read-only deadline')
 def query(tool,args):
  guard();p=Path(tool['path']);need(sha(p.read_bytes())==tool['sha256'],'tool hash')
  r=subprocess.run([str(p),*args],capture_output=True,timeout=min(5,deadline-time.monotonic()),env={'PATH':'/usr/bin:/bin','LANG':'C'})
  need(r.returncode==0 and not r.stderr,'query failed');guard();return r.stdout
 raws={n:read(ROOT/n,0o500 if n=='supervise.py' else 0o600) for n in ['binding.json','completed-state.json','worker-result.json','receipt.json','supervise.py']}
 need(sha(raws['binding.json'])==BINDING and sha(raws['supervise.py'])==SOURCE,'original source/binding hash')
 b=strict(raws['binding.json']);x=b['inputs'];need(b.get('admitted') is True and x['supervisor_sha256']==SOURCE,'original admitted binding')
 before=strict(raws['completed-state.json']);w=strict(raws['worker-result.json']);desc=saved_ok(x,before,w);disk=ROOT/'storage.ext4';disk_before=custody(disk);need(disk_before==w['disk_custody'],'disk changed')
 def observe():
  raw=query(x['tools']['systemctl'],['show',UNIT,'--property=LoadState,ActiveState,SubState,ControlGroup,Description,InvocationID'])
  pairs=[line.split('=',1) for line in raw.decode().splitlines()];need(all(len(p)==2 for p in pairs) and len(dict(pairs))==len(pairs),'properties shape');props=dict(pairs);stopped(props,desc,w['service_invocation_id'])
  cg=Path('/sys/fs/cgroup'+GROUP)
  if cg.exists():
   pairs=[s.split() for s in (cg/'cgroup.events').read_text().splitlines()];need(dict(pairs).get('populated')=='0','cgroup populated')
  associated=query(x['tools']['losetup'],['--list','--json','--associated',str(disk)]);loops_empty(associated)
  # Require entire loop inventory empty for this narrowly scoped recovery. An unrelated loop is conservatively unresolved.
  all_loops=query(x['tools']['losetup'],['--list','--json','--output','NAME,BACK-FILE']);loops_empty(all_loops)
  mount_lines=Path('/proc/self/mountinfo').read_text().splitlines()
  need(not any(len(s.split())>4 and (s.split()[4]==str(ROOT/'mnt') or s.split()[4].startswith(str(ROOT/'mnt')+'/')) for s in mount_lines),'host mount remains')
  return {'unit':props,'associated_stdout_sha256':sha(associated),'all_loops_stdout_sha256':sha(all_loops),'cgroup_absent':not cg.exists()}
 first=observe();second=observe();need(custody(disk)==disk_before,'disk custody changed during readback')
 for n,raw in raws.items():need(read(ROOT/n,0o500 if n=='supervise.py' else 0o600)==raw,'receipt changed during readback')
 guard()
 print(json.dumps({'schema':'research.readonly-reconciliation/v1','status':'assembled_stopped_observed','owner':OWNER,'original_launcher_status':'failed_reconciliation_preserved','image_id':w['image_id'],'disk_sha256_from_worker':w['disk_sha256'],'disk_content_rehashed':False,'disk_custody':disk_before,'observations':[first,second],'original_hashes':{n:sha(raw) for n,raw in raws.items()},'endpoint_live':False,'qualification_admitted':False,'elapsed_seconds':time.monotonic()-started},indent=2))
if __name__=='__main__':main()
