"""Prospective private runtime supervisor; no action without reviewed root operator admission."""
import argparse, fcntl, hashlib, json, math, os, re, shutil, signal, stat, subprocess, sys, time
from pathlib import Path
from datetime import datetime, timezone
BASE='sha256:347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476'
ARCHIVE_SHA='43a645ed7b810e1ef4f5ec473c61694dffeb8a69511ec2e44409fd192a494a2c'
ARCHIVE_BYTES=549662208
DISK_BYTES=4*1024**3
ROOT=Path('/var/tmp')
REQUIRED_TOOLS={'python','systemd-run','systemctl','dockerd','docker','containerd','mkfs.ext4','mount','umount','losetup','runc'}

def need(ok,msg):
 if not ok:raise ValueError(msg)
def sha(data):return hashlib.sha256(data).hexdigest()
def canonical(x):return json.dumps(x,sort_keys=True,separators=(',',':')).encode()
def strict(raw):
 def pairs(xs):
  d={}
  for k,v in xs:need(k not in d,'duplicate JSON key');d[k]=v
  return d
 return json.loads(raw,object_pairs_hook=pairs)
def read_small(p,limit=1024*1024):
 p=Path(p);need(p.is_absolute(),'absolute input')
 for q in [p,*p.parents]:need(not q.is_symlink(),'input symlink')
 s=p.stat();need(stat.S_ISREG(s.st_mode) and s.st_size<=limit,'small regular input')
 return p.read_bytes()
def digest_file(p,deadline=lambda:None):
 h=hashlib.sha256();n=0
 with Path(p).open('rb') as f:
  while True:
   deadline();b=f.read(1024*1024)
   if not b:break
   h.update(b);n+=len(b)
 return h.hexdigest(),n

def expiry(x):return datetime.fromisoformat(x.replace('Z','+00:00')).timestamp()
def paths(owner,mode="assembly"):
 need(mode in ("assembly","qualification"),"phase");suffix="a" if mode=="assembly" else "q"
 need(re.fullmatch(r'[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}',owner) is not None,'owner UUID')
 root=ROOT/('ppr-'+owner)
 return {'root':root,'unit':'pencil-private-'+owner+'.service','mount':root/'mnt','socket':root/'mnt'/('d-'+suffix+'.sock'),'disk':root/'storage.ext4','exec':root/'mnt'/('e-'+suffix),'shim':root/'mnt'/('s-'+suffix),'data':root/'mnt'/'data'}

def socket_paths(owner,mode="assembly"):
 p=paths(owner,mode)
 return {'docker':str(p['socket']),'containerd':str(p['exec']/'containerd'/'containerd.sock'),'containerd_debug':str(p['exec']/'containerd'/'containerd-debug.sock'),'containerd_ttrpc':str(p['exec']/'containerd'/'containerd.sock.ttrpc'),'shim': '/run/containerd/s/'+'f'*64}
def validate_socket_paths(owner):
 values={mode+"."+name:path for mode in ("assembly","qualification") for name,path in socket_paths(owner,mode).items()}
 for name,path in values.items():need(len(os.fsencode(path))+1<=108,'Unix socket pathname overflow '+name)
 return values
def mount_record(target,raw=None):
 raw=Path('/proc/self/mountinfo').read_text() if raw is None else raw
 rows=[]
 for line in raw.splitlines():
  fields=line.split();sep=fields.index('-')
  if fields[4]==str(target):rows.append({'device':fields[2],'root':fields[3],'options':fields[5].split(','),'fstype':fields[sep+1]})
 need(bool(rows),'missing private mount');return rows[-1]

def envelope(mode):
 need(mode in ('assembly','qualification'),'phase')
 return {'total_seconds':600 if mode=='assembly' else 300,'reserve_seconds':60 if mode=='assembly' else 30,'cpu_percent':200,'memory_bytes':2*1024**3,'swap_bytes':0,'tasks':256,'disk_bytes':DISK_BYTES,'attempts':1,'stop_seconds':5}
def validate(b,allow_expired=False):
 need(b.get('admitted') is True,'unadmitted draft');x=b['inputs'];validate_socket_paths(x['owner'])
 need(x['limits']==envelope(x['mode']),'fixed phase envelope');need(x['base_image']==BASE,'base image')
 need(x['base_archive']['sha256']==ARCHIVE_SHA and x['base_archive']['bytes']==ARCHIVE_BYTES,'archive pin')
 need(allow_expired or expiry(x['expires_at'])>time.time()+x['limits']['reserve_seconds']+10,'expired or insufficient reserve')
 need(x['supervisor_sha256']==sha(Path(__file__).read_bytes()),'supervisor source')
 need(set(x['tools'])==REQUIRED_TOOLS,'complete tool pins')
 for v in x['tools'].values():
  need(re.fullmatch(r'/[A-Za-z0-9_./-]+',v['path']) is not None and re.fullmatch('[0-9a-f]{64}',v['sha256']) is not None,'tool pin')
 review=strict(read_small(b['review_path']));need(sha(canonical(review))==b['review_sha256'],'canonical review digest')
 need(review['decision']=='approve' and review['scope']=='one-private-runtime-'+x['mode'] and review['inputs_sha256']==sha(canonical(x)),'review input/scope')
 need(review['independent'] is True and review['reviewer']!=x['author'] and review['execution_allowance_approved'] is True,'independent execution review')
 if x['mode']=='qualification':
  need(x['qualification'] is not None and x['predecessor'] is not None,'qualification bindings');q=x['qualification'];pred=x['predecessor'];need(re.fullmatch('[0-9a-f]{64}',pred.get('receipt_sha256') or '') is not None and re.fullmatch('[0-9a-f]{64}',q.get('binding_sha256') or '') is not None and q.get('binding') is not None,'qualification pins unresolved')
 else:need(x['qualification'] is None and x['predecessor'] is None,'assembly only')
 return x

def authenticate(b):
 need(os.geteuid()==0,'root operator launch required')
 # Actual operator approval is external; JSON assertions do not authenticate it.
 return validate(b)

def properties(x):
 e=x['limits'];p=paths(x['owner'],x['mode']);runtime=min(e['total_seconds'],math.floor(expiry(x['expires_at'])-time.time()))-e['stop_seconds']-5
 need(runtime>e['reserve_seconds'],'dispatch deadline')
 rw=str(p['root'])+' /sys/fs/cgroup'
 if x['mode']=='qualification':
  pred=x['predecessor'];need(pred['disk_path']==str(paths(pred['owner'])['disk']),'predecessor disk allowlist');rw+=' '+pred['disk_path']
 return ['Description=pencil-private-'+x['owner']+'-'+sha(canonical(x)),'Type=exec','ExitType=main','RemainAfterExit=yes','Restart=no','KillMode=control-group','SendSIGKILL=yes','TimeoutStopSec=5s','TimeoutStartSec=10s','RuntimeMaxSec='+str(runtime)+'s','RuntimeRandomizedExtraSec=0','CPUQuota=200%','MemoryMax=2147483648','MemorySwapMax=0','TasksMax=256','Delegate=yes','PrivateNetwork=yes','PrivateMounts=yes','ProtectSystem=strict','ReadWritePaths='+rw,'InaccessiblePaths=-/run/docker.sock -/var/run/docker.sock -/var/lib/docker','TemporaryFileSystem=/run/containerd:ro,size=4096,mode=0700','UMask=0077','StandardOutput=null','StandardError=null','LimitCORE=0','NoNewPrivileges=yes']
def launch_argv(x,binding):
 p=paths(x['owner'],x['mode']);return [x['tools']['systemd-run']['path'],'--unit='+p['unit'],'--no-block','--service-type=exec',*['--property='+s for s in properties(x)],x['tools']['python']['path'],'-I',str(p['root']/'supervise.py'),'--worker',str(binding)]
def daemon_argv(x,p,parent):
 return [x['tools']['dockerd']['path'],'--config-file='+str(p['mount']/('daemon-'+x['owner']+'.json')),'--host=unix://'+str(p['socket']),'--data-root='+str(p['data']),'--exec-root='+str(p['exec']),'--pidfile='+str(p['mount']/('dockerd-'+x['owner']+'.pid')),'--storage-driver=overlay2','--bridge=none','--iptables=false','--ip6tables=false','--ip-forward=false','--ip-masq=false','--exec-opt=native.cgroupdriver=cgroupfs','--cgroup-parent='+parent,'--group=0','--live-restore=false','--shutdown-timeout=5']
def docker_argv(x,p,args):return [x['tools']['docker']['path'],'--config',str(p['mount']/('client-config-'+x['owner'])),'--host','unix://'+str(p['socket']),*args]
def managed_children(pid):
 tasks=list(Path('/proc/'+str(pid)+'/task').iterdir());need(len(tasks)<=256,'daemon thread bound');children=set()
 for task in tasks:
  try:children.update(int(p) for p in (task/'children').read_text().split())
  except FileNotFoundError:continue
 need(len(children)<=256,'daemon child bound');return children

def service_group(unit):return '/system.slice/'+unit

def require_group(pid,ancestor):
 raw=Path('/proc/'+str(pid)+'/cgroup').read_text();lines=raw.splitlines();need(len(lines)==1 and lines[0].startswith('0::'),'unified cgroup required');cg=lines[0][3:]
 need(cg==ancestor or cg.startswith(ancestor+'/'),'process escaped service');return cg

def cgroup_empty(group):
 p=Path('/sys/fs/cgroup'+group)
 if not p.exists():return True
 values=dict(line.split() for line in (p/'cgroup.events').read_text().splitlines());return values.get('populated')=='0'
def file_custody(p):
 s=Path(p).lstat();need(stat.S_ISREG(s.st_mode) and s.st_uid==0 and s.st_nlink==1 and stat.S_IMODE(s.st_mode)==0o600,'private file custody')
 return {'device':s.st_dev,'inode':s.st_ino,'bytes':s.st_size,'mtime_ns':s.st_mtime_ns}
def save(p,d):
 raw=json.dumps(d,indent=2).encode()+b'\n';need(len(raw)<=1024*1024,'receipt size');p=Path(p);tmp=p.with_name(p.name+'.tmp')
 with tmp.open('xb') as f:f.write(raw);f.flush();os.fsync(f.fileno())
 os.replace(tmp,p)

class Worker:
 def __init__(self,x):
  self.x=x;self.p=paths(x['owner'],x['mode']);self.start=time.monotonic();self.deadline=min(self.start+x['limits']['total_seconds']-x['limits']['reserve_seconds'],self.start+expiry(x['expires_at'])-time.time()-x['limits']['reserve_seconds']);self.sequence=0;self.daemon=None;self.mounted=False;self.shim_mounted=False;self.shim_uncertain=False;self.disk=None;self.lock=None
  self.env={'PATH':'/usr/sbin:/usr/bin:/sbin:/bin','LANG':'C','HOME':str(self.p['mount']/('client-config-'+x['owner'])),'DOCKER_BUILDKIT':'0','INVOCATION_ID':os.environ.get('INVOCATION_ID','')}
 def guard(self):need(time.monotonic()<self.deadline and time.time()<expiry(self.x['expires_at']),'worker reserve or absolute deadline reached')
 def call(self,args,cap=15):
  self.guard();self.sequence+=1;cap=min(cap,self.deadline-time.monotonic());need(cap>0,'deadline')
  # Pipe capture capped by reader logic; hard storage cap is filesystem size.
  out=self.p['mount']/('command-'+self.x['owner']+'-'+str(self.sequence)+'.out');err=out.with_suffix('.err')
  with out.open('wb') as so,err.open('wb') as se:
   proc=subprocess.Popen(args,env=self.env,stdout=so,stderr=se)
   try:
    end=time.monotonic()+cap
    while proc.poll() is None:
     self.guard();need(time.monotonic()<end,'command timeout');need(max(so.tell(),se.tell())<=16*1024**2,'output bound');time.sleep(.05)
    need(proc.returncode==0,'command failed '+str(proc.returncode))
   finally:
    if proc.poll() is None:proc.kill();proc.wait(timeout=2)
  need(max(out.stat().st_size,err.stat().st_size)<=16*1024**2,'output bound');return out.read_bytes()
 def docker(self,args,cap=15):return self.call(docker_argv(self.x,self.p,args),cap)
 def check_tools(self):
  for name,v in self.x['tools'].items():
   need(digest_file(v['path'],self.guard)[0]==v['sha256'],'tool changed '+name)
 def setup_groups(self):
  group=service_group(self.p['unit']);require_group(os.getpid(),group);root=Path('/sys/fs/cgroup'+group)
  need((root/'memory.max').read_text().strip()=='2147483648' and (root/'memory.swap.max').read_text().strip()=='0' and (root/'pids.max').read_text().strip()=='256','live resource limits')
  quota,period=(root/'cpu.max').read_text().split();need(quota!='max' and int(quota)<=2*int(period),'live CPU quota');leaf=root/'control';payload=root/'payload';leaf.mkdir();(leaf/'cgroup.procs').write_text(str(os.getpid()))
  need(not (root/'cgroup.procs').read_text().strip(),'service parent occupied');(root/'cgroup.subtree_control').write_text('+cpu +memory +pids');payload.mkdir();(payload/'cgroup.subtree_control').write_text('+cpu +memory +pids');return group,group+'/payload'
 def setup_storage(self):
  x=self.x;p=self.p
  if x['mode']=='assembly':
   self.disk=p['disk'];self.lock=self.disk.open('xb+');self.lock.truncate(DISK_BYTES);self.lock.flush();os.fsync(self.lock.fileno());fcntl.flock(self.lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
   # Before mount, use bounded stdout pipes for the single filesystem command.
   subprocess.run([x['tools']['mkfs.ext4']['path'],'-q','-F',str(self.disk)],env=self.env,check=True,timeout=min(30,self.deadline-time.monotonic()),stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
  else:
   pred=x['predecessor'];raw=read_small(pred['receipt_path']);need(sha(raw)==pred['receipt_sha256'],'predecessor receipt pin');r=strict(raw)
   need(r['status']=='assembled_stopped' and r['cgroup_empty'] and r['loop_detached'],'predecessor unresolved');self.disk=Path(r['disk_path']);need(self.disk==paths(r['owner'])['disk'] and r['owner']==pred['owner'] and str(self.disk)==pred['disk_path'],'predecessor disk path');need(file_custody(self.disk)==r['disk_custody'],'predecessor disk custody')
   self.lock=self.disk.open('rb+');fcntl.flock(self.lock,fcntl.LOCK_EX|fcntl.LOCK_NB);need(digest_file(self.disk,self.guard)==(r['disk_sha256'],DISK_BYTES),'predecessor disk digest')
   need(cgroup_empty(r['service_cgroup']),'predecessor active cgroup')
  p['mount'].mkdir(mode=0o700)
  subprocess.run([x['tools']['mount']['path'],'-o','loop,noatime',str(self.disk),str(p['mount'])],env=self.env,check=True,timeout=min(15,self.deadline-time.monotonic()),stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);self.mounted=True;p['mount'].chmod(0o700);need(stat.S_IMODE(p['mount'].stat().st_mode)==0o700,'mounted root custody')
  self.setup_runtime_dirs()
  (p['mount']/('daemon-'+x['owner']+'.json')).write_text('{}\n')
 def setup_runtime_dirs(self):
  for directory in [self.p['mount']/('client-config-'+self.x['owner']),self.p['exec'],self.p['shim']]:directory.mkdir(mode=0o700,exist_ok=False)
 def setup_shim_mount(self):
  self.guard();target=Path('/run/containerd');record=mount_record(target)
  need(record['fstype']=='tmpfs' and 'ro' in record['options'],'initial private shim mask')
  source=self.p['shim'];need(source.is_dir() and stat.S_IMODE(source.stat().st_mode)==0o700,'private shim source')
  self.shim_uncertain=True
  subprocess.run([self.x['tools']['mount']['path'],'--bind',str(source),str(target)],env=self.env,check=True,timeout=min(15,self.deadline-time.monotonic()),stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
  self.shim_mounted=True;self.shim_uncertain=False
  record=mount_record(target);a=source.stat();b=target.stat()
  need((a.st_dev,a.st_ino)==(b.st_dev,b.st_ino) and record['fstype']=='ext4' and 'rw' in record['options'],'shim disk binding')
 def teardown_shim_mount(self):
  need(not self.shim_uncertain,'shim mount outcome uncertain')
  if self.shim_mounted:
   self.guard();self.shim_uncertain=True
   subprocess.run([self.x['tools']['umount']['path'],'/run/containerd'],env=self.env,check=True,timeout=min(15,self.deadline-time.monotonic()),stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
   self.shim_mounted=False;self.shim_uncertain=False
   record=mount_record('/run/containerd');need(record['fstype']=='tmpfs' and 'ro' in record['options'],'shim private mask restored')
 def check_daemon(self,group):
  need(self.daemon is not None and self.daemon.poll() is None,'daemon dead');require_group(self.daemon.pid,group)
  children=managed_children(self.daemon.pid);found=[]
  for child in children:
   pid=int(child);child=str(child);exe=Path('/proc/'+child+'/exe')
   try:target=os.readlink(exe)
   except FileNotFoundError:continue
   if Path(target).name=='containerd':
    require_group(pid,group);status=dict(line.split(':',1) for line in Path('/proc/'+child+'/status').read_text().splitlines() if ':' in line);need(int(status['PPid'])==self.daemon.pid and int(status['Tgid'])==pid,'managed containerd parent');need(digest_file(exe,self.guard)[0]==self.x['tools']['containerd']['sha256'],'managed containerd executable')
    args=Path('/proc/'+child+'/cmdline').read_bytes().split(b'\0');need(any(str(self.p['exec']).encode() in a for a in args),'managed private containerd arguments');found.append(pid)
  need(len(found)==1,'one managed private containerd');return found[0]
 def context(self):
  x=self.x;root=Path(x['context']['path']);members=x['context']['members'];need(len(members)<=128,'context bound');names={m['path'] for m in members};need(len(names)==len(members),'context duplicates');dest=self.p['mount']/'context';dest.mkdir(mode=0o700);dest.chmod(int(x['context']['root_mode'],8))
  need({str(f.relative_to(root)) for f in root.rglob('*')}==names,'source context inventory')
  for m in sorted(members,key=lambda m:(len(Path(m['path']).parts),m['path'])):
   rel=Path(m['path']);need(not rel.is_absolute() and '..' not in rel.parts,'context traversal');src=root/rel;dst=dest/rel;need(not src.is_symlink(),'context symlink');need(stat.S_IMODE(src.stat().st_mode)==int(m['mode'],8),'source context mode')
   if m['kind']=='directory':need(src.is_dir(),'context directory');dst.mkdir(mode=0o700);dst.chmod(int(m['mode'],8))
   else:
    need(src.is_file() and src.stat().st_size==m['bytes'],'context file');shutil.copyfile(src,dst);need(digest_file(dst,self.guard)==(m['sha256'],m['bytes']),'context digest');dst.chmod(int(m['mode'],8))
  for m in members:need(stat.S_IMODE((dest/m['path']).stat().st_mode)==int(m['mode'],8),'snapshot context mode')
  recipe=(dest/'Dockerfile').read_bytes();need(sha(recipe)==x['context']['recipe_sha256'],'recipe digest')
  instructions=[line.split()[0].upper() for line in recipe.decode().splitlines() if line.strip() and not line.lstrip().startswith('#')];need(set(instructions)<={'ARG','FROM','COPY','USER','ENV','LABEL','WORKDIR','ENTRYPOINT','CMD'} and 'RUN' not in instructions,'COPY-only recipe')
  return dest
 def build(self):
  x=self.x;archive=x['base_archive'];snapshot=self.p['mount']/'base-input.tar';shutil.copyfile(archive['path'],snapshot);need(digest_file(snapshot,self.guard)==(ARCHIVE_SHA,ARCHIVE_BYTES),'base archive changed');self.docker(['load','--input',str(snapshot)],180)
  base=strict(self.docker(['image','inspect',BASE]))[0];need(base['Id']==BASE and base['Os']=='linux' and base['Architecture']=='amd64' and not base['Config'].get('OnBuild') and base['RootFS']['Layers']==x['base_layers'],'base configuration')
  ctx=self.context();iid=self.p['mount']/'runtime.iid';tag='pencil-runtime-private:'+x['owner'];self.docker(['build','--pull=false','--network=none','--platform=linux/amd64','--iidfile',str(iid),'--build-arg','BASE_IMAGE='+BASE,'--label','isolab.private.owner='+x['owner'],'--tag',tag,'--file',str(ctx/'Dockerfile'),str(ctx)],240)
  image=read_small(iid).decode().strip();need(re.fullmatch('sha256:[0-9a-f]{64}',image) is not None,'IID');built=strict(self.docker(['image','inspect',image]))[0]
  need(built['Id']==image and built['Os']=='linux' and built['Architecture']=='amd64' and built['Config']['Labels'].get('isolab.private.owner')==x['owner'] and strict(self.docker(['image','inspect',tag]))[0]['Id']==image and built['Config']['User']=='0:0' and not built['Config'].get('Entrypoint') and not built['Config'].get('OnBuild'),'built runtime config');need(built['Config']['Labels'].get('org.opencontainers.image.revision')==x['source_revision'],'source revision');need(built['RootFS']['Layers'][:len(x['base_layers'])]==x['base_layers'] and len(built['RootFS']['Layers'])>len(x['base_layers']),'runtime layers');return image
 def qualification_bundle(self):
  q=self.x['qualification'];dest=self.p['mount']/'qualifier-source';dest.mkdir(mode=0o700)
  need(set(q['files'])=={'supervise.py','head_contract.py','private_contract.py','gate.c'},'qualifier source closure')
  for name,pin in q['files'].items():
   raw=read_small(pin['path']);need(sha(raw)==pin['sha256'],'qualifier dependency pin');(dest/name).write_bytes(raw);(dest/name).chmod(0o500)
  need(q['source_sha256']==q['files']['supervise.py']['sha256'],'qualifier source join')
  raw=read_small(q['binding']);need(sha(raw)==q['binding_sha256'],'qualifier binding pin');need(strict(raw)['gate_source_sha256']==q['files']['gate.c']['sha256'],'gate source binding');(dest/'binding.json').write_bytes(raw);(dest/'binding.json').chmod(0o400)
  return dest/'supervise.py',dest/'binding.json'
 def endpoint(self,group,parent,image,daemon_id):
  st=self.p['socket'].stat();need(stat.S_ISSOCK(st.st_mode) and st.st_uid==0,'private socket');os.chmod(self.p['socket'],0o600);st=self.p['socket'].stat();pid=self.daemon.pid;start=Path('/proc/'+str(pid)+'/stat').read_text().rsplit(')',1)[1].split()[19]
  return {'schema':'software.private-runtime-endpoint/v1','mode':'qualification','owner':self.x['owner'],'socket_path':str(self.p['socket']),'socket_uid':st.st_uid,'socket_gid':st.st_gid,'socket_mode':stat.S_IMODE(st.st_mode),'socket_device':st.st_dev,'socket_inode':st.st_ino,'daemon_id':daemon_id,'daemon_pid':pid,'daemon_start_ticks':int(start),'service_unit':self.p['unit'],'service_invocation_id':os.environ.get('INVOCATION_ID'),'service_cgroup':group,'cgroup_parent':parent,'supervisor_pid':os.getpid(),'source_binding_sha256':self.x['source_binding_sha256'],'image_id':image,'input_manifest_sha256':self.x['input_manifest_sha256'],'assembly_receipt_sha256':self.x['predecessor']['receipt_sha256'],'expires_at':datetime.fromtimestamp(min(expiry(self.x['expires_at']),time.time()+max(0,self.deadline-time.monotonic())),timezone.utc).isoformat().replace('+00:00','Z')}
 def ready_info(self):
  end=min(self.deadline,time.monotonic()+30);last=None
  for _ in range(30):
   self.guard();need(self.daemon.poll() is None,'daemon startup exit')
   try:return strict(self.docker(['info','--format','{{json .}}'],min(2,max(.1,end-time.monotonic()))))
   except (ValueError,subprocess.SubprocessError,OSError) as e:last=type(e).__name__
   if time.monotonic()>=end:break
   time.sleep(.1)
  raise ValueError('daemon metadata readiness '+str(last))
 def run(self):
  x=self.x;group=service_group(self.p['unit']);result={'status':'worker_unresolved','owner':x['owner'],'mode':x['mode'],'service_cgroup':group,'unit':self.p['unit'],'service_invocation_id':os.environ.get('INVOCATION_ID')};log=None
  try:
   self.check_tools();group,parent=self.setup_groups();self.setup_storage();self.setup_shim_mount()
   log=(self.p['mount']/('daemon-'+x['owner']+'.log')).open('xb');self.daemon=subprocess.Popen(daemon_argv(x,self.p,parent),env=self.env,stdout=log,stderr=log)
   info=self.ready_info();containerd_pid=self.check_daemon(group);need(info['CgroupDriver']=='cgroupfs' and info['DockerRootDir']==str(self.p['data']),'private daemon config')
   if x['mode']=='assembly':image=self.build()
   else:
    pred=strict(read_small(x['predecessor']['receipt_path']));need(info['ID']==pred['daemon_id'],'retained private daemon ID');image=pred['image_id'];need(strict(self.docker(['image','inspect',image]))[0]['Id']==image,'qualification image');ep=self.endpoint(group,parent,image,info['ID']);save(self.p['root']/'endpoint.json',ep);qsource,qbinding=self.qualification_bundle();self.call([x['tools']['python']['path'],'-B',str(qsource),'--binding',str(qbinding),'--receipts',str(self.p['mount']/'qualification-receipts'),'--image-id',image,'--binary-sha256',x['binary_sha256'],'--private-endpoint',str(self.p['root']/'endpoint.json'),'--private-endpoint-sha256',sha(read_small(self.p['root']/'endpoint.json'))],240)
   self.check_daemon(group);result.update(status='worker_phase_complete',image_id=image,daemon_id=info['ID'],containerd_pid=containerd_pid)
  except BaseException as e:result['error']=type(e).__name__+': '+str(e)
  finally:
   try:
    if self.daemon is not None and self.daemon.poll() is None:self.daemon.terminate();self.daemon.wait(timeout=5)
    if self.mounted:
     leaf=Path('/sys/fs/cgroup'+group+'/control');remaining={int(v) for v in (leaf/'cgroup.procs').read_text().split()};need(remaining<={os.getpid()},'managed descendants remain');need(cgroup_empty(group+'/payload'),'payload descendants remain')
     if log is not None:log.close()
     self.teardown_shim_mount();self.guard();subprocess.run([x['tools']['umount']['path'],str(self.p['mount'])],env=self.env,check=True,timeout=min(15,self.deadline-time.monotonic()),stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);self.mounted=False
    if self.disk is not None and not self.mounted:
     disk_sha,n=digest_file(self.disk,self.guard);need(n==DISK_BYTES,'disk size');result.update(disk_path=str(self.disk),disk_sha256=disk_sha,disk_custody=file_custody(self.disk))
   except BaseException as e:result.update(status='worker_unresolved',cleanup_error=type(e).__name__+': '+str(e))
   result.update(mounted=self.mounted,shim_mounted=self.shim_mounted,shim_uncertain=self.shim_uncertain,endpoint_live=False);save(self.p['root']/'worker-result.json',result)
   if self.lock is not None:self.lock.close()
  need(result['status']=='worker_phase_complete','worker unresolved');return result

def remaining(deadline,cap=5):
 value=cap if deadline is None else min(cap,deadline-time.monotonic());need(value>0,'launcher deadline');return value
def query(x,args,deadline=None):return subprocess.check_output([x['tools']['systemctl']['path'],*args],timeout=remaining(deadline),env={'PATH':'/usr/bin:/bin','LANG':'C'})
def properties_readback(x,deadline=None):
 raw=query(x,['show',paths(x['owner'],x['mode'])['unit'],'--property=LoadState,ActiveState,SubState,ControlGroup,Result,InvocationID,ExecMainStatus,Description'],deadline)
 return dict(line.split('=',1) for line in raw.decode().splitlines() if '=' in line)
def expected_description(x):return 'pencil-private-'+x['owner']+'-'+sha(canonical(x))
def completed_snapshot(x,props):
 need(props.get('Description')==expected_description(x) and props.get('ActiveState')=='active' and props.get('SubState')=='exited' and props.get('Result')=='success' and props.get('ExecMainStatus')=='0','service not successfully exited')
 worker=strict(read_small(paths(x['owner'],x['mode'])['root']/'worker-result.json'));need(worker['service_invocation_id']==props.get('InvocationID') and worker['status']=='worker_phase_complete','worker invocation/result');return props

def reconcile(x,deadline=None):
 p=paths(x['owner'],x['mode']);before=strict(read_small(p['root']/'completed-state.json'));completed_snapshot(x,before);props=properties_readback(x,deadline)
 need(props.get('LoadState')=='not-found' or (props.get('Description')==expected_description(x) and props.get('ActiveState')=='inactive'),'unit stop unresolved')
 group=service_group(p['unit']);need(cgroup_empty(group),'cgroup unresolved')
 worker=strict(read_small(p['root']/'worker-result.json'));need(worker['owner']==x['owner'] and worker['mounted'] is False and worker.get('shim_mounted') is False and worker.get('shim_uncertain') is False,'worker unresolved');disk=Path(worker['disk_path']);need(file_custody(disk)==worker['disk_custody'],'disk changed after worker')
 raw=subprocess.check_output([x['tools']['losetup']['path'],'--json','--associated',str(disk)],timeout=remaining(deadline));need(strict(raw)['loopdevices']==[],'loop attachment unresolved');remaining(deadline)
 return {**worker,'status':'assembled_stopped' if x['mode']=='assembly' else 'qualified_stopped','cgroup_empty':True,'loop_detached':True,'completed_unit_state':before,'stopped_unit_state':props,'endpoint_live':False}
def stop_owned(x,deadline=None):
 p=paths(x['owner'],x['mode']);description=query(x,['show',p['unit'],'--property=Description','--value'],deadline).decode().strip();need(description==expected_description(x),'unit ownership uncertain');query(x,['stop','--no-block',p['unit']],deadline)
def wait_stopped(x,deadline):
 while time.monotonic()<deadline-10:
  props=properties_readback(x,deadline)
  stopped=props.get('LoadState')=='not-found' or (props.get('Description')==expected_description(x) and props.get('ActiveState')=='inactive')
  if stopped and cgroup_empty(service_group(paths(x['owner'],x['mode'])['unit'])):return
  time.sleep(.1)
 raise ValueError('stop reconciliation reserve reached')
def launch(b):
 x=authenticate(b);p=paths(x['owner'],x['mode']);need(query(x,['show',p['unit'],'--property=LoadState','--value']).decode().strip()=='not-found','unit already exists');p['root'].mkdir(mode=0o700)
 save(p['root']/'binding.json',b);(p['root']/'supervise.py').write_bytes(Path(__file__).read_bytes());os.chmod(p['root']/'supervise.py',0o500)
 result={'status':'unresolved','owner':x['owner'],'automatic_retry':False};started=time.monotonic();deadline=min(started+x['limits']['total_seconds'],started+expiry(x['expires_at'])-time.time())
 try:
  subprocess.run(launch_argv(x,p['root']/'binding.json'),check=True,timeout=remaining(deadline,10),stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
  while time.monotonic()<deadline-x['limits']['reserve_seconds']:
   props=properties_readback(x,deadline)
   if props.get('ActiveState')=='active' and props.get('SubState')=='exited':save(p['root']/'completed-state.json',completed_snapshot(x,props));break
   need(props.get('ActiveState') not in ('failed','inactive'),'service exited before successful snapshot');time.sleep(.5)
  stop_owned(x,deadline)
  wait_stopped(x,deadline)
  result=reconcile(x,deadline);need(time.monotonic()<deadline and time.time()<expiry(x['expires_at']),'completion expired')
 except BaseException as e:
  result={'status':'unresolved','owner':x['owner'],'automatic_retry':False,'error':type(e).__name__+': '+str(e)}
  try:stop_owned(x,deadline)
  except BaseException as stop:result['stop_error']=type(stop).__name__
 finally:save(p['root']/'receipt.json',result)
 return 0 if result['status'] in ('assembled_stopped','qualified_stopped') else 1

def main():
 ap=argparse.ArgumentParser();g=ap.add_mutually_exclusive_group(required=True);g.add_argument('--launch');g.add_argument('--worker');g.add_argument('--reconcile');a=ap.parse_args();b=strict(read_small(a.launch or a.worker or a.reconcile))
 if a.launch:return launch(b)
 need(os.geteuid()==0,'root operator required');x=validate(b,allow_expired=bool(a.reconcile))
 if a.reconcile:print(json.dumps(reconcile(x),indent=2));return 0
 need(Path(a.worker)==paths(x['owner'],x['mode'])['root']/'binding.json','worker state path');Worker(x).run();return 0
if __name__=='__main__':raise SystemExit(main())
