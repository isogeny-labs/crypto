"""Integrate genuine external merge and publish a new owned successor."""
from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');local='6e77f73d58d5565dfa7ba3d0d827c53822c003f1';parent='966648f5414694e1b1c0a0954568f0b586b73367';branch='research/chained-transfer-PCC-archive-source-20261116'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==local
assert subprocess.check_output(['git','rev-parse','MERGE_HEAD'],cwd=r,text=True).strip()==parent
assert subprocess.check_output(['git','branch','--show-current'],cwd=r,text=True).strip()==branch
component=r/'research/chained_jacobian_transfers_20261110';g=component/'evidence/pcc-archive-source-successor-governance-01';g.mkdir();shutil.copyfile(__file__,g/'original-command.py')
scope=[str(component.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-16.md','.resume/RESUME.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(n,c,expected=0):
 q=subprocess.run(c,cwd=r,capture_output=True);raw=q.stdout+q.stderr;(g/(n+'.txt.xz')).write_bytes(lzma.compress(raw));(g/(n+'.exit')).write_text(str(q.returncode)+'\n');print(n,q.returncode,flush=True);assert q.returncode==expected;return raw
assert not subprocess.check_output(['git','diff','--name-only',local,'--','src','isogeny_algos','tools','.github/workflows','.artifacts/config.json'],cwd=r)
assert subprocess.check_output(['git','rev-parse',parent+'^{tree}'],cwd=r)==subprocess.check_output(['git','rev-parse','d3bb7bdc00d3a48db275de159dfc3075384efa6a^{tree}'],cwd=r)
run('actual-parent-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',parent]);run('authored-prose',['python3','tools/pcc/pcc.py','lint-prose',*[n.decode()for n in paths()if n.endswith(b'.md')]])
run('unchanged-current-workflow-actionlint',['/workspace/scratch/actionlint-bin/actionlint','-oneline','.github/workflows/pcc-gate.yml'])
bench=run('historical-benchrec',['python3','tools/benchrec.py','gate'],1);assert hashlib.sha256(bench).hexdigest()=='dc4fc3d1e178e01f99a53162a18e81d96fb8c783b57b25929a986ceb38e17b18'
def stage():
 names=paths();f=Path('/workspace/scratch/ct196-source.paths0');f.write_bytes(b'\0'.join(n for n in names if n!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(names)
 for name in selected:
  if name.startswith(b'pcc/claims/'):
   for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     s=proof['subject'];assert hashlib.sha256((r/s['path']).read_bytes()).hexdigest()==s['sha256'];assert s['path'].encode()in selected or subprocess.run(['git','cat-file','-e',local+':'+s['path']],cwd=r,capture_output=True).returncode==0
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('actual-parent-original-artifact-policy',['python3','tools/artifacts/artifacts.py','check',parent,tree])
c=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Root successor source governance after77ebba: actual external merge966 tree equalsd3, ordinary pending merge into6e, no source/old-ledger rewrite. Original actual-parent ledger prefix/prose/actionlint/cached/artifact0; historical benchrec1 unchanged exact. Actual code and tests remain531e88/821559/80b1fb source; only whole failed publication/external merge observation metadata added. Every new subject exact staged or parented. Commit ordinary two-parent merge and publish only a fresh absent owned branch/tag; old deleted source branch is not recreated. New ready PR and fresh required CI follow; no scientific/signed promotion.']
for f in sorted(g.iterdir()):c+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(c,cwd=r,text=True).strip();stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','-c','gc.auto=0','commit','--quiet','-m','research: preserve external parent merge in PCC archive repair successor'],cwd=r,check=True)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();assert subprocess.check_output(['git','show','-s','--format=%P',head],cwd=r,text=True).split()==[local,parent];print(claim,head,flush=True)
env=os.environ.copy();n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'});url='https://github.com/isogeny-labs/crypto.git'
for label,ref in [('new-owned-branch','refs/heads/'+branch),('new-source-tag','refs/tags/chained-transfer-PCC-archive-source-20261116-'+head[:12])]:
 out=Path('/workspace/scratch/ct196-root-'+label+'-publication-01');out.mkdir()
 before=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'before.txt').write_bytes(before.stdout+before.stderr);assert before.returncode==0 and not before.stdout
 q=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+ref],cwd=r,env=env,capture_output=True,timeout=180);(out/'actual-push.log.xz').write_bytes(lzma.compress(q.stdout+q.stderr));(out/'actual-push.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
 after=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'after.txt').write_bytes(after.stdout+after.stderr);assert after.returncode==0 and after.stdout.decode().split()[0]==head
 (out/'receipt.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=head,new_ref=ref,normal_new_ref_push=0,absent_before=True,exact_readback=True,no_force=True,genuine_external_parent=parent,parents=[local,parent],fresh_required_CI_unobserved=True),indent=2)+'\n');print(label,0,ref,flush=True)
