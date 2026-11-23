"""Create the required ACTIVE-only draft tracking commit. Metadata only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess
r=Path('/workspace/crypto-chained-balanced-main');old='265b97cabcc41ad60ba86d2d769b4cc24836c4ea';base='c5cbb67b19fe39864da7f8c9b29726696ba01580';branch='lane/chained-richelot-map14'
env=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
def call(args,check=True):
 p=subprocess.run(args,cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 if check:assert p.returncode==0,(args,p.returncode,p.stdout,p.stderr)
 return p
def git(*args):return call(['git','-c','gc.auto=0',*args]).stdout
current=git('rev-parse','HEAD').decode().strip();assert current in [old,base]
assert Path('/workspace/scratch/ct758-old14-custody-exact-readback.txt').read_text().split()[0]==old
assert git('write-tree','--missing-ok').strip()==git('rev-parse',old+'^{tree}').strip()
if current==old:assert call(['git','show-ref','--verify','--quiet','refs/heads/'+branch],False).returncode==1
else:assert git('symbolic-ref','HEAD').decode().strip()=='refs/heads/'+branch
cfg=call(['git','config','--get','core.hooksPath'],False);assert cfg.returncode==1
hooks=Path(git('rev-parse','--git-path','hooks').decode().strip());assert not [p for p in hooks.iterdir() if p.is_file() and not p.name.endswith('.sample') and os.access(p,os.X_OK)]
idx=Path(env['GIT_INDEX_FILE']);backup=Path('/run/lock/ct760-entire-owned-index-before-c5-tracking')
if not backup.exists():shutil.copy2(idx,backup)
assert idx.read_bytes()==backup.read_bytes()
if current==old:git('branch',branch,base);git('symbolic-ref','HEAD','refs/heads/'+branch)
git('read-tree','--reset','-i',base)
root=git('write-tree','--missing-ok').decode().strip();assert root==git('rev-parse',base+'^{tree}').decode().strip()
canonical=git('cat-file','blob',base+':docs/ACTIVE.md').decode();line='- [ ] Publish frozen joint-chart Richelot divisor map14 and full evidence (Codex, started 2026-11-17, branch '+branch+'; in progress)'
assert canonical.count('## Active items (top = next)')==1
active=canonical.replace('## Active items (top = next)\n','## Active items (top = next)\n\n'+line+'\n',1)
p=r/'docs/ACTIVE.md';p.unlink();p.write_text(active)
git('add','--sparse','--','docs/ACTIVE.md');git('diff','--cached','--check')
assert git('diff','--cached','--name-status').decode().strip()=='M\tdocs/ACTIVE.md'
tree=git('write-tree','--missing-ok').decode().strip();args=['git','commit-tree',tree,'-p',base,'-m','docs: track frozen Richelot divisor-map14 publication']
sign=call(['git','config','--bool','--get','commit.gpgsign'],False)
if sign.returncode==0 and sign.stdout.strip()==b'true':args.insert(2,'-S')
commit=call(args).stdout.decode().strip();git('update-ref','refs/heads/'+branch,commit,base)
receipt={'time':datetime.now(timezone.utc).isoformat(),'canonical_base':base,'old_complete_public_fork':old,'branch':branch,'head':commit,'base_complete_index_tree':root,'new_complete_index_tree':tree,'exactly_one_changed_file':'docs/ACTIVE.md','all_other_base_index_entries_unchanged':True,'all_other_sessions_ACTIVE_lines_preserved':True,'old_entire_owned_index_backup':{'path':str(backup),'bytes':backup.stat().st_size,'sha256':hashlib.sha256(backup.read_bytes()).hexdigest()},'no_active_hooks_bypassed':True,'no_research_or_PCC_policy_change_in_tracking_commit':True}
Path('/workspace/scratch/ct760-actual-ACTIVE-only-tracking-commit.json').write_text(json.dumps(receipt,indent=2)+'\n')
body='''Publish the completed frozen joint-chart Richelot divisor-map unit14 on current main, preserving the complete public source fork and every negative result. Full source, native output, both executables and dependencies are already retained at [immutable custody checkpoint265b97ca](https://github.com/isogeny-labs/crypto/tree/265b97cabcc41ad60ba86d2d769b4cc24836c4ea/research/chained_richelot_map_20261117). Producer results remain unsigned. [pcc:unverified]

This initial tracking commit changes only docs/ACTIVE.md and preserves every existing task line. Next: restore exact canonical PCC/tool metadata, merge complete journal blocks, import unchanged scientific sources and inspected reports, and link full raw containers under current artifact rules. Check frozen-source equality, ledger base prefix and existing diagnostic baseline, artifact/prose/whitespace checks, PR files and actual check count<=30. Native arithmetic is already retained; no repetition is planned. [pcc:unverified]

Mark ready and enable normal MERGE auto-merge when the result change is checked; all CI/review/merge gates remain required. Original full EKS populations, paidSTOP/vault lock, signed independent replay, general formalization and complete solver-cost comparisons remain open. Then continue cubic-subfield closed-fibre specialization. [pcc:unverified]
'''
Path('/workspace/scratch/ct760-draft-PR-body.md').write_text(body)
print(commit)
