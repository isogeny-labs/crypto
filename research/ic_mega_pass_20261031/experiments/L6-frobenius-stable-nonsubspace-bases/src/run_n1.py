import sys, os, json, subprocess
sys.path.insert(0, os.path.dirname(__file__))
from field import *
T=os.environ['TMPDIR']; S=setup(23); d=cyc(S,2); xs=sorted(d)
assert all(len(v)==1 for v in d.values())
open(T+'/n1_x.txt','w').write('\n'.join(map(str,xs)))
open(T+'/n1_e.txt','w').write('\n'.join(f"{d[x][0][0]} {d[x][0][1]}" for x in xs))
open(T+'/n1_onb.txt','w').write('\n'.join(map(str,S['conv'])))
# CRT cross-check of convolution model: lift sum maps to field under z->gamma
out={}
for nt in (256,4096):
    o=subprocess.run([T+'/l6n1','n1','23',str(S['F']),str(S['r']),T+'/n1_x.txt',T+'/n1_e.txt',str(nt),T+'/n1_onb.txt'],capture_output=True,text=True)
    out[nt]=json.loads(o.stdout) if o.returncode==0 else dict(err=o.stderr,rc=o.returncode); print(nt,out[nt],flush=True)
json.dump(out,open(sys.argv[1],'w'),indent=1)
