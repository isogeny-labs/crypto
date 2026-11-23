from pathlib import Path
import json,subprocess,difflib
P=Path('/home/ec2-user/ml-dataset-resume-20261029/owned-search-activation-audit-20261107');A=P.parent/'owned-search-continuation-delivery-20261107/assembled';repo='/home/ec2-user/crypto-ml-ordinary-ownership-20261101'
rows=json.loads((P/'DIFFERENCES.json').read_text())['differences'];mods=[r for r in rows if r['head_blob']]
p=subprocess.Popen(['git','-C',repo,'cat-file','--batch'],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
patch=[];stats=[]
for r in mods:
 p.stdin.write((r['head_blob']+'\n').encode());p.stdin.flush();hdr=p.stdout.readline().decode().split();data=p.stdout.read(int(hdr[2]));assert p.stdout.read(1)==b'\n'
 a=data.decode().splitlines(True);b=(A/r['path']).read_text().splitlines(True)
 delta=list(difflib.unified_diff(a,b,fromfile='HEAD/'+r['path'],tofile='assembled/'+r['path']))
 patch.extend(delta);stats.append({'path':r['path'],'added':sum(x.startswith('+') and not x.startswith('+++') for x in delta),'removed':sum(x.startswith('-') and not x.startswith('---') for x in delta)})
p.stdin.close();p.wait();(P/'MODIFIED.diff').write_text(''.join(patch));(P/'DIFF_STATS.json').write_text(json.dumps(stats,indent=2)+'\n');print(json.dumps(stats,indent=2))
