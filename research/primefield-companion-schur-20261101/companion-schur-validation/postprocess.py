"""Follow the one existing producer; never launches or restarts a trial."""
from pathlib import Path
import json,time,subprocess,datetime
D=Path(__file__).resolve().parent;R=D.parent
launch=json.loads((D/'launch.json').read_text())
def state(phase,**extra):
 (D/'postprocess-state.json').write_text(json.dumps(dict(observed_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),phase=phase,producer_pid=launch['producer_pid'],**extra),indent=2)+'\n')
state('waiting_for_existing_producer')
misses=0
while not (D/'exit-code.txt').exists():
 p=Path('/proc')/str(launch['producer_pid'])/'stat'
 try:alive=p.read_text().split()[21]==launch['start_ticks']
 except FileNotFoundError:alive=False
 misses=0 if alive else misses+1
 if misses>=3:
  state('producer_missing_without_exit_receipt',restarted=False);raise SystemExit(1)
 time.sleep(2)
code=int((D/'exit-code.txt').read_text())
if code:
 state('producer_failed_or_capped',producer_exit=code,restarted=False);raise SystemExit(code)
steps=[('analysis',[str(R.parent/'.audit-venv/bin/python'),str(D/'analyze.py')]),('export-check',[str(R.parent/'.venv/bin/python'),str(R/'tools/experiment_protocol.py'),'check-export',str(D/'runs/export.json'),'--protocol',str(D/'protocol.json'),'--verify-files'])]
for name,cmd in steps:
 state('running_'+name)
 with (D/(name+'.log')).open('w') as f:code=subprocess.run(cmd,cwd=R,stdout=f,stderr=subprocess.STDOUT).returncode
 (D/(name+'-exit.txt')).write_text(str(code)+'\n')
 if code:state(name+'_failed',exit_code=code);raise SystemExit(code)
state('all_terminal_checks_passed',producer_exit=0,analysis_exit=0,export_exit=0)
