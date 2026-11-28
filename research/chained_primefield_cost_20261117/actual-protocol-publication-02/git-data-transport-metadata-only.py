"""Git object/byte transport only; no research arithmetic or result generation."""
from pathlib import Path
import base64,hashlib,json,os,subprocess,sys
r=Path('/workspace/crypto-chained-balanced-main');E=dict(os.environ,GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1');action=sys.argv[1]
def git(*args):
 p=subprocess.run(['git','-c','gc.auto=0',*args],cwd=r,env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert p.returncode==0,(args,p.stderr);return p.stdout
if action=='prepare':
 base,head,out=sys.argv[2:5]
 head=git('rev-parse',head).decode().strip()
 known=set()
 for line in git('ls-tree','-r',base,'--','research/chained_cubic_inverse_20261117','research/chained_richelot_map_20261117','pcc/claims','docs/ACTIVE.md').decode().splitlines():known.add(line.split()[2])
 names=[p for p in git('diff','--name-only','-z',base,head).decode().split('\0') if p];rows=[]
 for path in names:
  desc=git('ls-tree',head,'--',path).decode().split();assert len(desc)>=4 and desc[1]=='blob',path
  mode,blob=desc[0],desc[2];b=git('cat-file','blob',head+':'+path)
  row={'path':path,'mode':mode,'sha':blob,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'server_known':blob in known}
  if path in ['pcc/ledger.jsonl','docs/session-logs/2026-11-17.md']:
   old=git('cat-file','blob',base+':'+path);assert b.startswith(old),path
   row.update(append_only=True,base_sha=git('rev-parse',base+':'+path).decode().strip(),base_bytes=len(old),append=b[len(old):].decode())
  rows.append(row)
 manifest={'base':base,'head':head,'base_tree':git('rev-parse',base+'^{tree}').decode().strip(),'head_tree':git('rev-parse',head+'^{tree}').decode().strip(),'files':rows}
 Path(out).write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps({'files':len(rows),'base':base,'head':head,'head_tree':manifest['head_tree'],'manifest':out}))
elif action=='manifest':print(Path(sys.argv[2]).read_text(),end='')
elif action=='read':
 j=json.loads(Path(sys.argv[2]).read_text());row=j['files'][int(sys.argv[3])];b=git('cat-file','blob',j['head']+':'+row['path']);assert hashlib.sha256(b).hexdigest()==row['sha256']
 try:content=b.decode();encoding='utf-8'
 except UnicodeDecodeError:content=base64.b64encode(b).decode();encoding='base64'
 print(json.dumps({'encoding':encoding,'content':content,'bytes':len(b)}))
else:raise ValueError(action)
