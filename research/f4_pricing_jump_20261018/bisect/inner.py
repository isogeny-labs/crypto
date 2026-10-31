import json,sys
sys.argv=['x']
exec(open('/home/user/bisect/drive.py').read().split('changes=[]')[0])
import subprocess
extra=sys.stdin.read().split()
for c in extra:
    full=subprocess.check_output(['git','-C','/home/user/crypto','rev-parse',c],text=True).strip()
    commits.append(full); print(c, sig(len(commits)-1), flush=True)
