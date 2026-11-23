import os
import json,glob,subprocess,os
raw=''+os.path.dirname(os.path.abspath(__file__))+'/../results/raw'
n=0;bad=[]
for f in sorted(glob.glob(raw+'/compare-*.json')):
    d=json.load(open(f)); argv=d['argv']
    track='C' if os.path.basename(f).startswith('compare-C') else 'B'
    binp='/dev/shm/restore/v2/bin/ecbench-'+track
    p=subprocess.run(['taskset','-c','0',binp]+argv,capture_output=True,text=True)
    if p.returncode!=0: bad.append((f,'rc',p.returncode,p.stderr[-200:])); continue
    res=json.loads(p.stdout); n+=1
    if res!=d['result']: bad.append((os.path.basename(f),'DIFF'))
print('rerun',n,'bad',bad)
