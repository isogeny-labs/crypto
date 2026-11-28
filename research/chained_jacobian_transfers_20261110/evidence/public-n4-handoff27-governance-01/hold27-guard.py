"""Whole held-byte/link-state I/O guard, including retained failed-layout links."""
from pathlib import Path
import hashlib,json,os,stat,subprocess
def sha(path):
 p=Path(path);assert not p.resolve().is_relative_to(Path('/workspace/crypto-autoresearcher'))
 with p.open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def state(path):
 p=Path(path);row=dict(literal_target=os.readlink(p))
 try:st=p.stat()
 except FileNotFoundError:row['target_kind']='missing'
 except OSError as exc:row.update(target_kind='unresolvable',errno=exc.errno)
 else:
  if stat.S_ISREG(st.st_mode):row.update(target_kind='file',target_bytes=st.st_size,target_sha256=sha(p))
  elif stat.S_ISDIR(st.st_mode):row['target_kind']='directory'
  else:row['target_kind']='other'
 return row
def intact(control):
 o=Path(control);v=json.loads((o/'preparation-observation.json').read_text());contents=json.loads((o/'current-inputs.json').read_text());links=json.loads((o/'symlink-states.json').read_text())
 assert len(contents)==v['current_content_guard_keys']and len(links)==v['all_literal_link_guards']
 for p,want in contents.items():assert Path(p).is_file()and sha(p)==want,p
 for p,want in links.items():assert Path(p).is_symlink()and state(p)==want,p
 c=Path('/workspace/cryptanalysis');r=Path('/workspace/crypto')
 assert sha(c/'pcc/ledger.jsonl')==v['consumer_ledger_exact_sha256']
 prior=subprocess.check_output(['git','show',v['consumer_base']+':pcc/ledger.jsonl'],cwd=c)
 assert(c/'pcc/ledger.jsonl').read_bytes()==prior
 rootprior=subprocess.check_output(['git','show',v['root_base']+':pcc/ledger.jsonl'],cwd=r)
 assert(r/'pcc/ledger.jsonl').read_bytes().startswith(rootprior)
 return dict(content_keys=len(contents),literal_links=len(links),non_file_links={p:s for p,s in links.items()if s['target_kind']!='file'},consumer_ledger_exact=True,producer_ledger_prefix=True)
