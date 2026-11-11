#!/usr/bin/env python3
import pathlib,subprocess,json,time,os,hashlib,sys,datetime
P=pathlib.Path(__file__).resolve().parent
W=pathlib.Path('/var/tmp/pencil-kummer-lock-v4-20261111');U='pencil-kummer-lock-v4-20261111.service'
r=json.loads((P/'release.json').read_text());assert r['released'] is True and r['serial_owner_confirmed'] is True
assert r['launcher_sha256']==hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest()
deadline=r['absolute_deadline'];assert deadline.endswith('Z')
deadline=datetime.datetime.fromisoformat(deadline[:-1]+'+00:00')
assert datetime.datetime.now(datetime.timezone.utc)+datetime.timedelta(seconds=60)<deadline
W.mkdir(exist_ok=False);(W/'caller-owner.json').write_text(json.dumps({'unit':U,'pid':os.getpid(),'release_sha256':hashlib.sha256((P/'release.json').read_bytes()).hexdigest()}))
for name in ['unit.stdout','unit.stderr']:
 fd=os.open(str(W/name),os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600);os.close(fd)
argv=json.loads((P/'commands.json').read_text())['prospective_argv']
start=time.monotonic();owner=None;terminal={}
def show():
 q=subprocess.run(['systemctl','show',U,'--property=InvocationID,ControlGroup,MainPID,ActiveState,SubState,Result,ExecMainStatus,LoadState'],capture_output=True,text=True,timeout=2,check=True)
 return dict(line.split('=',1) for line in q.stdout.splitlines() if '=' in line)
try:
 launch=subprocess.run(argv,capture_output=True,text=True,timeout=5)
 (W/'launch.stdout').write_text(launch.stdout);(W/'launch.stderr').write_text(launch.stderr)
 if launch.returncode:raise RuntimeError('systemd launch rejected')
 owner=show();assert owner.get('InvocationID');(W/'owner.json').write_text(json.dumps(owner,indent=2))
 while time.monotonic()-start<52:
  terminal=show();assert terminal['InvocationID']==owner['InvocationID']
  if terminal.get('ActiveState') in ['inactive','failed'] or (terminal.get('SubState')=='exited' and terminal.get('MainPID')=='0'):break
  time.sleep(.25)
finally:
 if owner:
  current=show();assert current['InvocationID']==owner['InvocationID']
  (W/'pre-stop.json').write_text(json.dumps(current,indent=2))
  if current.get('ActiveState') not in ['inactive','failed']:
   subprocess.run(['sudo','systemctl','stop','--no-block',U],timeout=2,check=True)
  empty=False
  while time.monotonic()-start<58:
   terminal=show()
   vanished=terminal.get('LoadState')=='not-found' and not terminal.get('InvocationID')
   assert vanished or terminal['InvocationID']==owner['InvocationID']
   cg=current.get('ControlGroup') or owner.get('ControlGroup');path=pathlib.Path('/sys/fs/cgroup'+cg) if cg else None
   empty=not path or not path.exists() or ('populated 0' in (path/'cgroup.events').read_text())
   if (vanished or terminal.get('MainPID')=='0') and empty:break
   time.sleep(.1)
  terminal.update(elapsed_seconds=time.monotonic()-start,owned_cgroup_empty=bool(empty))
  (W/'terminal.json').write_text(json.dumps(terminal,indent=2)+'\n')
  if not empty:raise RuntimeError('owned cleanup not verified; no retry')
result=json.loads((W/'result.json').read_text())
pre=json.loads((W/'pre-stop.json').read_text())
sys.exit(0 if result['status']=='generated_pending_publication' and set(result['locks'])=={'driver','adapter'} and pre.get('Result')=='success' and pre.get('ExecMainStatus')=='0' and pre.get('MainPID')=='0' and terminal.get('owned_cgroup_empty') else 1)
