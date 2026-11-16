"""Whole dirty owned-byte baseline after payload commit and before bot FF."""
from pathlib import Path
import hashlib,json,os,subprocess
r=Path('/workspace/crypto');c=Path('/workspace/cryptanalysis');o=Path('/workspace/scratch/ct77-whole-model-and-third-handoff-18');assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()=='c35045cefbaf48048858a6783133ddd1ec133a77'
scopes={'root':['research/chained_jacobian_transfers_20261110','.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md'],'consumer':['experiments/chained-transfer-public-target','docs/research-artifacts/chained-transfer','.gitattributes','.github/workflows/chained-transfer-norm-halving.yml','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','problems/chained-transfer-odd-product-kernel-and-odd-factor.md']}
def owned(repo,scope):
 names=sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=repo).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=repo).split(b'\0')))-{b''})
 return {n.decode():{'sha256':hashlib.sha256((repo/n.decode()).read_bytes()).hexdigest(),'link':os.readlink(repo/n.decode())if(repo/n.decode()).is_symlink()else None}for n in names}
b={'scopes':scopes,'owned':{name:owned(repo,scopes[name])for name,repo in [('root',r),('consumer',c)]}}
(o/'owned-before.json').write_text(json.dumps(b,indent=2)+'\n');print('owned counts',{k:len(v)for k,v in b['owned'].items()})
