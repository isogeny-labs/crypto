from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');base='aa1e8236f8e2c9ee6aec5657e68d74dd32da7794';branch='research/chained-transfer-cubic-norm-ci-custody-20261116'
env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0'
env['PATH']='/dev/shm/ct267-cubic-norm-final-dual-gates/proof-bin:/home/agent/.elan/bin:/home/agent/.cargo/bin:'+env['PATH']
env['ELAN_TOOLCHAIN']='leanprover/lean4:v4.35.0'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip()==base
assert subprocess.check_output(['git','symbolic-ref','--short','HEAD'],cwd=r,env=env,text=True).strip()==branch
ram=Path('/dev/shm/ct315-norm-CI-custody-source-whole-index');ram.mkdir()
shutil.copyfile(r/'.git/index',ram/'entire-index-before');shutil.copyfile(r/'.git/index',ram/'actual-current-index')
env['GIT_INDEX_FILE']=str(ram/'actual-current-index')
p=r/'research/chained_jacobian_transfers_20261110';g=p/'evidence/cubic-norm-CI-custody-source-governance-11';g.mkdir()
shutil.copyfile(__file__,g/'original-command.py')
scope=[str(p.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.gitattributes','.github/workflows/pcc-gate.yml','.github/workflows/chained-transfer-cubic-norm-proof-01.yml']
def paths():
    return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r,env=env).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0')))-{b''})
def run(name,args,want=0):
    q=subprocess.run(args,cwd=r,env=env,capture_output=True)
    raw=q.stdout+q.stderr;(g/(name+'.txt.xz')).write_bytes(lzma.compress(raw));(g/(name+'.exit')).write_text(str(q.returncode)+'\n')
    print(name,q.returncode,flush=True);assert q.returncode==want;return raw
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','.artifacts/config.json','pcc/keyring.json'],cwd=r,env=env)
run('original-merged-parent-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('authored-prose',['python3','tools/pcc/pcc.py','lint-prose',*[n.decode() for n in paths() if n.endswith(b'.md')]])
raw=run('unchanged-historical-benchrec',['python3','tools/benchrec.py','gate'],1)
assert hashlib.sha256(raw).hexdigest()=='dc4fc3d1e178e01f99a53162a18e81d96fb8c783b57b25929a986ceb38e17b18'
run('new-workflow-scoped-original-wiring',['python3','tools/artifacts/wire_workflows.py','--check','.github/workflows/chained-transfer-cubic-norm-proof-01.yml'])
run('both-actual-workflow-actionlint',['/workspace/scratch/actionlint-bin/actionlint','-oneline','.github/workflows/chained-transfer-cubic-norm-proof-01.yml','.github/workflows/pcc-gate.yml'])
run('original-checker-exact-final-nonsquare-binding',['python3','tools/pcc/pcc.py','verify','pcc/claims/PCC-20261116-2f1fe7.json','--json'])
for f in ['.github/workflows/pcc-gate.yml','.github/workflows/chained-transfer-cubic-norm-proof-01.yml']:
    shutil.copyfile(r/f,g/('immutable-actual-'+Path(f).name))
def stage():
    names=paths();f=ram/'selected-paths0';f.write_bytes(b'\0'.join(n for n in names if n!=b'.resume/RESUME.md')+b'\0')
    subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,env=env,check=True)
    subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,env=env,check=True)
    selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0'))-{b''};assert selected==set(names)
    for n in selected:
        if n.startswith(b'pcc/claims/'):
            for proof in json.loads((r/n.decode()).read_text()).get('proofs',[]):
                if proof.get('kind')=='attestation':
                    s=proof['subject'];assert hashlib.sha256((r/s['path']).read_bytes()).hexdigest()==s['sha256']
                    assert s['path'].encode() in selected or subprocess.run(['git','cat-file','-e',base+':'+s['path']],cwd=r,env=env,capture_output=True).returncode==0
stage();run('cached-whitespace',['git','diff','--cached','--check'])
tree=subprocess.check_output(['git','write-tree'],cwd=r,env=env,text=True).strip()
raw=run('original-artifact-policy-before-genuine-bot',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
assert b'2 data file(s)' in raw and b'cubic-norm-final-guarded-integration-08/entire-current-hold.json.xz' in raw and b'cubic-norm-final-guarded-integration-08/entire-published-source-ledger.jsonl.xz' in raw
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Publish completed norm current-CI whole-byte/raw/actual merge/queue checkpoint from genuine merged4413 aa1e8236 on fresh owned branch. Current norm dual original15/0 and eight started successes6631ae, actual8723-key FF6774a3 and normal expected-head merge/fresh branch94c4ad remain exact prior scopes. Original immediate merged-parent ledger/prose/scoped wiring/both-actionlint/exact immutable nonsquare PCC binding+kernel/cached0; unchanged broad benchrec1 has identical historical raw SHA. No new math/native algorithm/source change, rerun, signed promotion or computational-cost claim. New pre-bot artifact-policy1 names exactly the complete current8723-key guard and published ledger XZ in cubic-norm-final-guarded-integration-08; both gate/sign exact hydrations precede real migration. Original verifiers/keyring/tools/policy/native and directly pinned norm01 source/report/workflow unchanged. Queue713621 records successful Consumer2c native/cold compiler/proof941dbe, whole1663-file snapshot10ced7 and fifth whole-JSON artifact requirement, with runtime/OCI/full LP/NEW28/main-ledger/locked-vault gaps retained. Actual exhausted budgete42949 still stops all paid builds/jobs. Normal new source/tag publication and own current CI/genuine bot integration follow; no deleted merged head resurrection or force/rechain/admin bypass.']
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip();stage()
subprocess.run(['git','diff','--cached','--check'],cwd=r,env=env,check=True)
subprocess.run(['git','-c','gc.auto=0','commit','--quiet','-m','research: checkpoint complete cubic norm CI custody and guarded merge'],cwd=r,env=env,check=True)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip()
assert subprocess.check_output(['git','rev-parse','HEAD^'],cwd=r,env=env,text=True).strip()==base
shutil.copyfile(ram/'actual-current-index',r/'.git/index');assert (r/'.git/index').read_bytes()==(ram/'actual-current-index').read_bytes()
print(claim,head,flush=True)
n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
url='https://github.com/isogeny-labs/crypto.git'
for kind,ref in [('new-branch','refs/heads/'+branch),('new-tag','refs/tags/chained-transfer-cubic-norm-ci-custody-source-20261116-'+head[:12])]:
    out=ram/kind;out.mkdir()
    q=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'entire-before.txt').write_bytes(q.stdout+q.stderr);assert q.returncode==0 and not q.stdout
    q=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+ref],cwd=r,env=env,capture_output=True,timeout=180);(out/'entire-actual-push.log.xz').write_bytes(lzma.compress(q.stdout+q.stderr));(out/'actual-push.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
    q=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'entire-after.txt').write_bytes(q.stdout+q.stderr);assert q.returncode==0 and q.stdout.decode().split()[0]==head
    (out/'actual-receipt.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=head,parent=base,ref=ref,actual_normal_new_push=0,absent_before=True,exact_readback=True,force=False),indent=2)+'\n')
    print(kind,head,flush=True)
