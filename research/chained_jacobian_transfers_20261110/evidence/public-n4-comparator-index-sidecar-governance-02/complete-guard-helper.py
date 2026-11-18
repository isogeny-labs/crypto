"""Exact temporary metadata-sidecar hold; no mathematical calculations."""
from pathlib import Path
import json,os,subprocess
from ct105_hold27_guard import sha,state
control=Path('/workspace/scratch/ct108-index-sidecar-custody-01')
def intact():
    g=json.loads((control/'complete-current-guard.json').read_text());r=Path('/workspace/crypto');c=Path('/workspace/cryptanalysis')
    for p,h in g['contents'].items():assert Path(p).is_file()and sha(p)==h,p
    for p,want in g['literal_link_states'].items():assert Path(p).is_symlink()and state(p)==want,p
    assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==g['consumer_HEAD']
    assert subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=c).decode()==g['consumer_status']
    for p,want in g['consumer_owned'].items():
        q=c/p;assert dict(sha256=sha(q)if q.is_file()else None,literal_link=os.readlink(q)if q.is_symlink()else None)==want,p
    assert sha(c/'pcc/ledger.jsonl')==g['consumer_exact_ledger_sha256']
    previous=subprocess.check_output(['git','show',g['producer_ledger_prefix_commit']+':pcc/ledger.jsonl'],cwd=r)
    assert(r/'pcc/ledger.jsonl').read_bytes().startswith(previous)
    return dict(content_keys=len(g['contents']),literal_links=len(g['literal_link_states']),consumer_owned_paths=len(g['consumer_owned']),consumer_HEAD_status_owned_ledger_exact=True,producer_ledger_prefix=True)
