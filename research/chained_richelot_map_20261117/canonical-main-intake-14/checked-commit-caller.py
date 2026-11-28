"""Checked partial-clone commit; all upstream CI/review/merge gates still apply."""
from pathlib import Path
from datetime import datetime,timezone
import json,os,subprocess,sys
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';base='c5cbb67b19fe39864da7f8c9b29726696ba01580';parent=sys.argv[1] if len(sys.argv)>1 else '6f1815b9f90b71f407f68e77901d3cc6f12cb8f9';old='265b97cabcc41ad60ba86d2d769b4cc24836c4ea';branch='refs/heads/lane/chained-richelot-map14'
E=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
def run(args,check=True):
 p=subprocess.run(args,cwd=r,env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 if check:assert p.returncode==0,(args,p.returncode,p.stdout,p.stderr)
 return p
def git(*a):return run(['git','-c','gc.auto=0',*a]).stdout.decode().strip()
assert git('rev-parse','HEAD')==parent and git('symbolic-ref','HEAD')==branch
assert run(['git','config','--get','core.hooksPath'],False).returncode==1
h=Path(git('rev-parse','--git-path','hooks'));assert not [p for p in h.iterdir() if p.is_file() and not p.name.endswith('.sample') and os.access(p,os.X_OK)]
v=run(['python3','tools/pcc/pcc.py','ledger','verify','--base',base],False)
expected=[x for x in (s/'canonical-main-intake-14/EXACT_C5_BASELINE_PCC.stdout').read_text().splitlines() if x.startswith('claim ')]
actual=[x for x in v.stdout.decode().splitlines() if x.startswith('claim ')]
assert v.returncode==1 and expected==actual and len(actual)==151 and not v.stderr
ledger=run(['git','cat-file','blob',base+':pcc/ledger.jsonl']).stdout;assert (r/'pcc/ledger.jsonl').read_bytes().startswith(ledger)
git('diff','--cached','--check')
frozen=['src','vendor','reused-core','original-sources','new-code','compiler-revision-D','compiler-revision-F','Cargo.toml','Cargo.lock','run_first_unit_14.py','run_first_unit_14_C.py','SOURCE_LOCK_14.json','SOURCE_LOCK_14C.json','PROTOCOL_14.md','PROOF_14.md','FIXTURE_14.csv']
git('diff','--cached','--exit-code',old,'--',*[str(s.relative_to(r)/x) for x in frozen])
for path in ['pcc/keyring.json','tools/pcc','.artifacts/config.json','tools/artifacts','AGENTS.md','tools/goal_brief.py']:
 git('diff','--cached','--exit-code',base,'--',path)
names=git('diff','--cached','--name-status','--no-renames').splitlines();allowed=('research/chained_richelot_map_20261117/','pcc/claims/','pcc/ledger.jsonl','docs/session-logs/2026-11-17.md','docs/ACTIVE.md','docs/lessons/entries/','benchmarks/agent-autonomy/episodes/')
for line in names:
 status,path=line.split('\t');assert status!='D' and path.startswith(allowed),(status,path);assert not path.endswith('.xz'),path
tree=git('write-tree','--missing-ok');args=['git','commit-tree',tree,'-p',parent,'-m',sys.argv[2] if len(sys.argv)>2 else 'research: validate joint-chart Richelot divisor maps and prime scalar transfers']
sign=run(['git','config','--bool','--get','commit.gpgsign'],False)
if sign.returncode==0 and sign.stdout.strip()==b'true':args.insert(2,'-S')
commit=run(args).stdout.decode().strip();assert git('rev-parse',commit+'^{tree}')==tree;git('update-ref',branch,commit,parent)
receipt={'time':datetime.now(timezone.utc).isoformat(),'canonical_base':base,'initial_tracking_commit':'6f1815b9f90b71f407f68e77901d3cc6f12cb8f9','previous_checked_head':parent,'head':commit,'tree':tree,'branch_compare_and_swap':branch,'all151_canonical_baseline_diagnostics_unchanged':True,'base_ledger_full_prefix':True,'all_frozen_scientific_source_paths_equal_to_old_public_checkpoint':True,'keyring_policy_verifiers_upstream_tools_and_instructions_unchanged_by_us':True,'cached_whitespace_exit':0,'no_active_hook_bypassed':True,'new_inline_compressed_data_files':0,'staged_files':names,'normal_push_and_all_upstream_gates_required':True}
Path(sys.argv[3] if len(sys.argv)>3 else '/workspace/scratch/ct764-actual-canonical14-result-commit.json').write_text(json.dumps(receipt,indent=2)+'\n');print(commit)
