"""Normal checkpoint publication of exact historical data-address repair."""
from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,lzma,os,shutil,subprocess

r=Path('/workspace/crypto');base='d3bb7bdc00d3a48db275de159dfc3075384efa6a'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
component=r/'research/chained_jacobian_transfers_20261110'
g=component/'evidence/pcc-archive-source-data-publication-governance-01';g.mkdir()
shutil.copyfile(__file__,g/'original-command.py')
shutil.copyfile('/workspace/scratch/ct189_publish_owned_codex_checkpoint_02.py',g/'normal-owned-branch-publication-command.py')
scope=[str(component.relative_to(r)),'tools/pcc/prepare_historical_workflow_data_02.py','tools/pcc/test_historical_workflow_data_02.py','.github/workflows/pcc-gate.yml','.gitattributes','pcc/claims','pcc/ledger.jsonl','problems/README.md','problems/chained-transfer-pcc-evidence-addresses.md','docs/session-logs/2026-11-16.md','.resume/RESUME.md']
def paths():
 return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd,expected=0):
 q=subprocess.run(cmd,cwd=r,capture_output=True);(g/(name+'.txt.xz')).write_bytes(lzma.compress(q.stdout+q.stderr));(g/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==expected;return q.stdout+q.stderr
allowed={'.gitattributes','.github/workflows/pcc-gate.yml','.resume/RESUME.md','docs/session-logs/2026-11-16.md','pcc/ledger.jsonl','problems/README.md','problems/chained-transfer-pcc-evidence-addresses.md'}
changed=set(subprocess.check_output(['git','diff','--name-only','--diff-filter=MDR',base],cwd=r,text=True).splitlines());assert changed<=allowed
assert not subprocess.check_output(['git','diff','--name-only',base,'--','tools/pcc/pcc.py','tools/pcc/test_pcc.py','pcc/keyring.json','tools/artifacts/artifacts.py','.artifacts/config.json','src','isogeny_algos','research/quartic_n4_la_20261021'],cwd=r)
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('authored-prose',['python3','tools/pcc/pcc.py','lint-prose',*[n.decode()for n in paths()if n.endswith(b'.md')]])
run('active-workflow-actionlint',['/workspace/scratch/actionlint-bin/actionlint','-oneline','.github/workflows/pcc-gate.yml'])
run('full-workflow-wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
bench=run('historical-benchrec',['python3','tools/benchrec.py','gate'],1)
assert hashlib.sha256(bench).hexdigest()=='dc4fc3d1e178e01f99a53162a18e81d96fb8c783b57b25929a986ceb38e17b18'
assert (component/'evidence/pcc-historical-workflow-source-02/entire-actual-new-workflow.yml').read_bytes()==(r/'.github/workflows/pcc-gate.yml').read_bytes()
assert hashlib.sha256((component/'evidence/pcc-historical-workflow-source-02/pcc-gate-before-archive-preparation.yml').read_bytes()).hexdigest()=='e3b1a7fa1d69c20550c9ba0a6e4563a850b657d8a7c90cca516f0ec284632b1a'
def stage():
 names=paths();pathfile=Path('/workspace/scratch/ct189-source.paths0');pathfile.write_bytes(b'\0'.join(n for n in names if n!=b'.resume/RESUME.md')+b'\0')
 subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(pathfile),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(names)
 for name in selected:
  if name.startswith(b'pcc/claims/'):
   for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     s=proof['subject'];assert hashlib.sha256((r/s['path']).read_bytes()).hexdigest()==s['sha256'];assert s['path'].encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+s['path']],cwd=r,capture_output=True).returncode==0
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('original-artifact-policy',['python3','tools/artifacts/artifacts.py','check',base,tree])
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Root archive/source-data repair02 source governance: ledger/prose/actionlint/wiring/cachedwhitespace/originalartifactpolicy actual0; every new subject exact staged or parented. Broad benchrec1 is byte-identical historical189 gaps/106 unchanged records, SHA dc4fc3d1..., not called0. All existing scientific source, original verifier/keyring/claims/tools/policy unchanged; only archive loader input and declared temporary unsigned historical workflow data address plus exact current restoration/safe metadata custody. Data821559/80b1fb preserve12 metadata controls, original22/62/8, actual b55 1/2/1 and inspected report. Historical source bytes are not current execution evidence. New claims use immutable snapshots, not the active workflow path. Actual required fresh CI and full science/EKS remain pending; no merge or signed scientific promotion.']
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True)
subprocess.run(['git','-c','gc.auto=0','commit','--quiet','-m','research: restore exact archival PCC inputs and preserve historical workflow data'],cwd=r,check=True)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();print(claim,head,flush=True)
branch='codex/chained-transfer-ledger-immutable-20261115-c019'
q=subprocess.run(['python3','/workspace/scratch/ct189_publish_owned_codex_checkpoint_02.py','isogeny-labs/crypto',base,head,branch,'/workspace/scratch/ct189-root-normal-publication-01'],cwd=r,capture_output=True);Path('/workspace/scratch/ct189-root-normal-publication-caller01.txt').write_bytes(q.stdout+q.stderr);print('normal-branch',q.returncode,flush=True);assert q.returncode==0
env=os.environ.copy();n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
url='https://github.com/isogeny-labs/crypto.git';ref='refs/tags/chained-transfer-PCC-archive-source-data-20261116-'+head[:12];out=Path('/workspace/scratch/ct189-root-new-source-tag-01');out.mkdir()
before=subprocess.run(['git','ls-remote','--tags',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'before.txt').write_bytes(before.stdout+before.stderr);assert before.returncode==0 and not before.stdout
pub=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+ref],cwd=r,env=env,capture_output=True,timeout=180);(out/'actual-push.log.xz').write_bytes(lzma.compress(pub.stdout+pub.stderr));(out/'actual-push.exit').write_text(str(pub.returncode)+'\n');assert pub.returncode==0
after=subprocess.run(['git','ls-remote','--tags',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'after.txt').write_bytes(after.stdout+after.stderr);assert after.returncode==0 and after.stdout.decode().split()[0]==head
(out/'receipt.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=head,new_tag=ref,actual_new_tag_push=0,exact_readback=True,normal_branch_publication_actual_exit=q.returncode,no_force=True,fresh_required_CI_unobserved=True),indent=2)+'\n');print('new-tag0',ref,flush=True)
