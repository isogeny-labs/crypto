from pathlib import Path
import subprocess,json,time,datetime,os
D=Path(__file__).resolve().parent;R=D.parent
assert not (D/'launch.json').exists() and not (D/'runs').exists()
with (D/'producer.log').open('x') as log:
 start=time.time();p=subprocess.Popen(['timeout','--signal=TERM','3600',str(R.parent/'.audit-venv/bin/python'),'-u',str(D/'check.py')],cwd=R,stdout=log,stderr=subprocess.STDOUT)
 children=Path('/proc')/str(p.pid)/'task'/str(p.pid)/'children'
 for _ in range(100):
  ids=children.read_text().split() if children.exists() else []
  if ids:break
  time.sleep(.01)
 assert ids,'producer handle not acquired'
 pid=int(ids[0]);ticks=(Path('/proc')/str(pid)/'stat').read_text().split()[21]
 receipt=dict(observed_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),predata_commit='3e77b2c',producer_pid=pid,start_ticks=ticks,timeout_pid=p.pid,launcher_pid=os.getpid(),timeout_seconds=3600,start_epoch=start,deadline_epoch=start+3600,deadline_authority='Native timeout process; never rearm',attempts=1,restart_allowed=False,protocol=json.loads((D/'protocol.json').read_text())['freeze'],seed_preflight_cells=1698,external_jobs_changed=False)
 (D/'launch.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt),flush=True)
 code=p.wait();(D/'exit-code.txt').write_text(str(code)+'\n');print('producer exit',code,flush=True)
