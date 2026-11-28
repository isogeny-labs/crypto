"""Metadata-only checkpoint; keep full Git index custody in RAM while staging."""
from pathlib import Path
from datetime import datetime, timezone
import os, subprocess, shutil, hashlib, json, lzma, base64
r=Path('/workspace/crypto');base='5ae1a82cc453102e63c828affda5b17faf65eabe'
branch='research/chained-transfer-runtime-progress-20261116'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
assert subprocess.check_output(['git','branch','--show-current'],cwd=r,text=True).strip()==branch
ram=Path('/dev/shm/ct264-root-CI-metadata-checkpoint');ram.mkdir()
original=r/'.git/index';shutil.copyfile(original,ram/'entire-index-before');shutil.copyfile(original,ram/'current-index')
env=os.environ.copy();env['GIT_INDEX_FILE']=str(ram/'current-index');env['GIT_OPTIONAL_LOCKS']='0'
component=r/'research/chained_jacobian_transfers_20261110';g=component/'evidence/merged-CI-runtime-progress-governance-05';g.mkdir();shutil.copyfile(__file__,g/'entire-original-command.py')
scope=[str(component.relative_to(r)),'problems/README.md','problems/chained-transfer-pcc-evidence-addresses.md','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.github/workflows/pcc-gate.yml']
def paths():
 return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r,env=env).split(b'\0')) | set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0')))-{b''})

def run(name,cmd,expected=0):
 p=subprocess.run(cmd,cwd=r,env=env,capture_output=True);raw=p.stdout+p.stderr;(g/(name+'.txt.xz')).write_bytes(lzma.compress(raw));(g/(name+'.exit')).write_text(str(p.returncode)+'\n');print(name,p.returncode,flush=True);assert p.returncode==expected;return raw
assert not subprocess.check_output(['git','diff',base,'--name-only','--','src','isogeny_algos','tools','.artifacts/config.json'],cwd=r,env=env)
run('actual-merged-parent-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('authored-prose',['python3','tools/pcc/pcc.py','lint-prose',*[x.decode() for x in paths() if x.endswith(b'.md')]])
bench=run('unchanged-historical-benchrec',['python3','tools/benchrec.py','gate'],1);assert hashlib.sha256(bench).hexdigest()=='dc4fc3d1e178e01f99a53162a18e81d96fb8c783b57b25929a986ceb38e17b18'
def stage():
 names=paths();f=ram/'selected.paths0';f.write_bytes(b'\0'.join(n for n in names if n!=b'.resume/RESUME.md')+b'\0')
 subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,env=env,check=True)
 subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,env=env,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0'))-{b''};assert selected==set(names)
 for n in selected:
  if n.startswith(b'pcc/claims/'):
   for p in json.loads((r/n.decode()).read_text()).get('proofs',[]):
    if p.get('kind')=='attestation':
     s=p['subject'];assert hashlib.sha256((r/s['path']).read_bytes()).hexdigest()==s['sha256']
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,env=env,text=True).strip()
raw=run('original-artifact-policy-before-genuine-bot',['python3','tools/artifacts/artifacts.py','check',base,tree],1);assert b'2 data file(s)' in raw and b'entire-current-hold.json.xz' in raw and b'entire-published-source-ledger.jsonl.xz' in raw
run('actual-workflow-actionlint',['/workspace/scratch/actionlint-bin/actionlint','-oneline','.github/workflows/pcc-gate.yml'])
shutil.copyfile(r/'.github/workflows/pcc-gate.yml',g/'entire-actual-workflow-source.yml')
receipt={'original_index_bytes':original.stat().st_size,'before_sha256':hashlib.sha256((ram/'entire-index-before').read_bytes()).hexdigest(),'alternate_index':str(ram/'current-index'),'source_unchanged_from':base,'scope':'Git metadata only, no science input or hold changed'}
(g/'entire-index-custody.json').write_text(json.dumps(receipt,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Root finished currentCI/runtime progress checkpoint from genuine merged5ae1: complete current genuine3dff migration/CI1a3751, all8483-key actual guarded FF270903 and normal4401 merge3dbf17 preserved. Metadata-only ENOSPC recovery bfeb59 preserves whole old Git index and identical HEAD/staged tree, no research/raw deletion. Actual empty-tree normal switch2f55ff uses fresh owned runtime-progress branch with42 selected current working files byte-exact and both indices held; no deleted-branch resurrection. Persistent actual GitHub queue/readyPR and repaired mergedPR description d853c5 retain complete old/current API replies. Original merged-parent prefix/prose/cached/actionlint0; broad benchrec1 byte-identical to previous historical gaps. Original pre-bot artifact policy1 exactly2 whole current-hold/published-ledger XZ objects; exact same two paths are added to both original gate/sign data hydration calls, while original verifiers/keyring/policy/science source stay unchanged and current workflow is pinned only via immutable evidence snapshot. Genuine trusted migration/current applicable green checks remain prerequisites for merge. Consumer0405429a driver source normally published, source5e80 actual native/post/proof8db45c and runtime-only1169c2/metadataefa891 do not execute the full historical population. Actual budgete42949 stops all paid builds/jobs. Full LP/OCI/admission/scientificNEW28 and main ledger remain open; no computational/signed promotion.']
for p in sorted(g.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip();stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,env=env,check=True)
subprocess.run(['git','-c','gc.auto=0','commit','--quiet','-m','research: preserve merged CI runtime progress and exhausted budget checkpoint'],cwd=r,env=env,check=True)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip();assert subprocess.check_output(['git','rev-parse','HEAD^'],cwd=r,env=env,text=True).strip()==base
shutil.copyfile(ram/'current-index',original);assert hashlib.sha256(original.read_bytes()).digest()==hashlib.sha256((ram/'current-index').read_bytes()).digest()
print(claim,head,flush=True)
n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
for label,ref in [('new-branch','refs/heads/'+branch),('new-tag','refs/tags/chained-transfer-runtime-progress-20261116-'+head[:12])]:
 out=ram/label;out.mkdir();url='https://github.com/isogeny-labs/crypto.git'
 p=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'before.txt').write_bytes(p.stdout+p.stderr);assert p.returncode==0 and not p.stdout
 p=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+ref],cwd=r,env=env,capture_output=True,timeout=180);(out/'entire-push.log.xz').write_bytes(lzma.compress(p.stdout+p.stderr));(out/'actual-push.exit').write_text(str(p.returncode)+'\n');assert p.returncode==0
 p=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'after.txt').write_bytes(p.stdout+p.stderr);assert p.returncode==0 and p.stdout.decode().split()[0]==head
 (out/'receipt.json').write_text(json.dumps({'source':head,'parent':base,'ref':ref,'normal_new_ref_push':0,'absent_before':True,'exact_readback':True,'force':False,'at':datetime.now(timezone.utc).isoformat()},indent=2)+'\n');print(label,head,flush=True)
