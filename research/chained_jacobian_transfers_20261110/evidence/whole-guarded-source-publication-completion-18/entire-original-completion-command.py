"""Complete staged metadata governor379 after preserving raw stderr whitespace."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');base='c8aa3f9d7b1584b859e04ee09ea73800da1ed201';branch='research/chained-transfer-whole-source-custody-20261116'
env=os.environ.copy();env.update(GIT_OPTIONAL_LOCKS='0',GIT_TERMINAL_PROMPT='0')
ram=Path('/dev/shm/ct379-whole-source-custody-publication');env['GIT_INDEX_FILE']=str(ram/'actual-current-index')
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip()==base
p=r/'research/chained_jacobian_transfers_20261110';g=p/'evidence/whole-guarded-source-publication-completion-18';g.mkdir()
shutil.copyfile(__file__,g/'entire-original-completion-command.py')
scope=[str(p.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.gitattributes','.github/workflows/pcc-gate.yml']
artifacts=[str(p.relative_to(r)/f) for f in ['evidence/cubic-word-current-own-CI-and-genuine-migration-07/PCC-entire-original-job-log.txt.xz','evidence/cubic-word-current-final-byte-guard-08/entire-current-hold.json.xz','evidence/cubic-word-current-final-byte-guard-08/entire-published-source-ledger.jsonl.xz','evidence/whole-guarded-source-original-object-12/entire-original-guarded-source.tar.xz']]
f=r/'.github/workflows/pcc-gate.yml'
def run(name,args,want=0):
 q=subprocess.run(args,cwd=r,env=env,capture_output=True,timeout=120);raw=q.stdout+q.stderr
 (g/(name+'.txt.xz')).write_bytes(lzma.compress(raw));(g/(name+'.exit')).write_text(str(q.returncode)+'\n')
 print(name,q.returncode,flush=True);assert q.returncode==want;return raw
def paths():
    return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r,env=env).split(b'\0')) | set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0'))) - {b''})

run('original-current-parent-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('all-current-authored-prose',['python3','tools/pcc/pcc.py','lint-prose',*[n.decode() for n in paths() if n.endswith(b'.md')]])
def stage():
    names=paths()
    selected=ram/'selected-paths0'
    selected.write_bytes(b'\0'.join(n for n in names if n != b'.resume/RESUME.md')+b'\0')
    subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(selected),'--pathspec-file-nul'],cwd=r,env=env,check=True)
    subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,env=env,check=True)
    actual=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0'))-{b''}
    assert actual==set(names)
    for name in actual:
        if name.startswith(b'pcc/claims/'):
            for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
                if proof.get('kind')=='attestation':
                    s=proof['subject']
                    assert hashlib.sha256((r/s['path']).read_bytes()).hexdigest()==s['sha256']
                    assert s['path'].encode() in actual or subprocess.run(['git','cat-file','-e',base+':'+s['path']],cwd=r,env=env,capture_output=True).returncode==0
stage()
run('cached-whitespace', ['git','diff','--cached','--check'])
tree=subprocess.check_output(['git','write-tree'],cwd=r,env=env,text=True).strip()
raw=run('original-artifact-policy-before-genuine-bot', ['python3','tools/artifacts/artifacts.py','check',base,tree],1)
assert b'4 data file(s)' in raw and all(a.encode() in raw for a in artifacts)
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Complete corrected staged governor after original raw whitespace preservation; original failure records82a89d/cb580e/0ce310 and append-only unpublished custody correctiond8025d retained. Publish complete lossless original1663-member guarded source as full14170288B XZ45155cf7 of39331840B TARb441b4, original local whole-byte replay56427e. Actual mergedword4415 c8aa and current exact27/0 gates/whole CI96eb44, full8959-key raw/published-ledger guard89348c, normal configured newbranch e77015 and current cross-repository queue9f7e56 remain separate exact scopes. Add exact four required whole objects to both original PCC gate/sign hydration commands before real artifact migration: complete current PCC log, complete current guard, complete published source ledger and entire source archive. Original current-parent ledger/prose/unchanged-proof scoped wiring/actionlint/cached checks0; original PCC template wiring1 is byte-identical to actual base diagnostic, four new hydration paths explicitly checked; unchanged historical benchrec1 exact raw SHA; original immediate artifact-policy1 names exactly those four complete payloads pending genuine upload/full remote replay/bot integration. Whole payload Git object is complete, not a pointer or missing-file waiver. Original verifiers/keyring/tools/policy/native and directly pinned norm/word version01 source/report/workflows unchanged; current successful mathematical gates are not rerun. KB tools absent/wrapper127; original conductor unavailable127; direct repository primary artifact/current workflow/source evidence reviewed without novelty inference. This checkpoint changes metadata/hydration only; no native/scientific population/paid launch/new identity/grant/provider, signed promotion, force/rechain/admin bypass. Actual source publication/current CI/genuine migration/normal guarded merge still follow; runtime portable helper/full LP/NEW28/ABI/admission and seven Consumer metadata objects remain distinct.']
for q in sorted(g.iterdir()): cmd += ['--proof','attestation:'+q.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip()
entry='\n---\n\n'+datetime.now(timezone.utc).isoformat()+'\n# Publish whole guarded-source payload and completed word custody\n\n## What was done\n'+claim+': complete14,170,288B original-source XZ and current whole PCC log/8959-key guard/source ledger selected by unchanged original policy. Both PCC jobs now hydrate all four exact paths. Original current-parent prefix/prose/unchanged-proof scoped wiring/actionlint/cached0; PCC wiring1 byte-identical to actual base, old benchrec1 exact historical SHA. All frozen scientific/proof assets unchanged. [pcc:unverified]\n\n## Decisions made\nUse normal inherited Git HTTPS, new owned source branch/immutable tag/ready PR, then genuine artifact upload/full remote byte replay and guarded bot integration. Sources checked: docs/artifacts.md, actual PCC workflow, original source56427e, completed current CI96eb44/guard89348c/mergee77015. KB unavailable: no MCP tools and wrapper127; conductor127. No novelty inference. [pcc:unverified]\n\n## Open threads\nOwn current CI/genuine full payload migration/merge; portable runtime source helper, seven Consumer metadata objects and full scientificNEW28. Actual LP60/240/62 population/checkpoint/ABI/EKS admission still unrun. [pcc:unverified]\n\n## Errors\nHistorical broad benchrec and PCC template diagnostic remain exact unchanged1; all prior failures/holds retained. Paid builds/jobs remain prohibited by actual exhaustede42949; no override. [pcc:unverified]\n'
for name in ['docs/session-logs/2026-11-16.md','.resume/RESUME.md']:
    with (r/name).open('a') as out: out.write(entry)
stage()
subprocess.run(['git','diff','--cached','--check'],cwd=r,env=env,check=True)
subprocess.run(['git','-c','gc.auto=0','commit','--quiet','-m','research: preserve complete guarded source through original artifact custody'],cwd=r,env=env,check=True)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip()
assert subprocess.check_output(['git','rev-parse','HEAD^'],cwd=r,env=env,text=True).strip()==base
shutil.copyfile(ram/'actual-current-index',r/'.git/index')
assert (ram/'actual-current-index').read_bytes()==(r/'.git/index').read_bytes()
print(claim,head,flush=True)
url='https://github.com/isogeny-labs/crypto.git'
for kind,ref in [('new-branch','refs/heads/'+branch),('new-tag','refs/tags/chained-transfer-whole-guarded-source-20261116-'+head[:12])]:
    out=ram/kind;out.mkdir()
    q=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60)
    (out/'entire-before.txt').write_bytes(q.stdout+q.stderr)
    assert q.returncode==0 and not q.stdout
    q=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+ref],cwd=r,env=env,capture_output=True,timeout=180)
    (out/'entire-actual-push.log.xz').write_bytes(lzma.compress(q.stdout+q.stderr))
    (out/'actual-push.exit').write_text(str(q.returncode)+'\n')
    assert q.returncode==0
    q=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60)
    (out/'entire-after.txt').write_bytes(q.stdout+q.stderr)
    assert q.returncode==0 and q.stdout.decode().split()[0]==head
    (out/'actual-receipt.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=head,parent=base,ref=ref,actual_normal_new_push=0,absent_before=True,exact_readback=True,force=False,authentication='normal inherited configured Git HTTPS; no manual token header'),indent=2)+'\n')
    print(kind,head,flush=True)
