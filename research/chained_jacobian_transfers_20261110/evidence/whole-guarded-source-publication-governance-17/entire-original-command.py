"""Whole source custody publication; file/Git metadata only, no science execution."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, lzma, os, shutil, subprocess

r = Path('/workspace/crypto')
base = 'c8aa3f9d7b1584b859e04ee09ea73800da1ed201'
branch = 'research/chained-transfer-whole-source-custody-20261116'
env = os.environ.copy()
env.update(GIT_OPTIONAL_LOCKS='0', GIT_TERMINAL_PROMPT='0')
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=r, env=env, text=True).strip() == base
assert subprocess.check_output(['git', 'symbolic-ref', '--short', 'HEAD'], cwd=r, env=env, text=True).strip() == branch
ram = Path('/dev/shm/ct379-whole-source-custody-publication')
ram.mkdir()
shutil.copyfile(r / '.git/index', ram / 'entire-index-before')
shutil.copyfile(r / '.git/index', ram / 'actual-current-index')
env['GIT_INDEX_FILE'] = str(ram / 'actual-current-index')
p = r / 'research/chained_jacobian_transfers_20261110'
g = p / 'evidence/whole-guarded-source-publication-governance-17'
g.mkdir()
shutil.copyfile(__file__, g / 'entire-original-command.py')
scope = [str(p.relative_to(r)), 'pcc/claims', 'pcc/ledger.jsonl',
         'docs/session-logs/2026-11-16.md', '.resume/RESUME.md', '.gitattributes',
         '.github/workflows/pcc-gate.yml']

def run(name, args, want=0):
    q = subprocess.run(args, cwd=r, env=env, capture_output=True, timeout=120)
    raw = q.stdout + q.stderr
    (g / (name + '.txt.xz')).write_bytes(lzma.compress(raw))
    (g / (name + '.exit')).write_text(str(q.returncode) + '\n')
    print(name, q.returncode, flush=True)
    assert q.returncode == want
    return raw

run('original-conductor-scope', ['bash', '-c', 'conductor check --summary "Publish complete original guarded source and completed word CI/merge custody" --scope path:research/chained_jacobian_transfers_20261110 --scope path:pcc --scope path:docs/session-logs/2026-11-16.md --scope path:.resume/RESUME.md --scope path:.github/workflows/pcc-gate.yml'], 127)
run('KB-status', ['bash', 'scripts/research-kb', 'status'], 127)
run('KB-exact-source-custody', ['bash', 'scripts/research-kb', 'search', 'chained transfer whole guarded source artifact custody', '--top-k', '8'], 127)
run('KB-source-negative', ['bash', 'scripts/research-kb', 'search', 'runtime source tracked symlink sparse checkout failure', '--top-k', '8'], 127)

artifacts = [
    'evidence/cubic-word-current-own-CI-and-genuine-migration-07/PCC-entire-original-job-log.txt.xz',
    'evidence/cubic-word-current-final-byte-guard-08/entire-current-hold.json.xz',
    'evidence/cubic-word-current-final-byte-guard-08/entire-published-source-ledger.jsonl.xz',
    'evidence/whole-guarded-source-original-object-12/entire-original-guarded-source.tar.xz',
]
artifacts = [str(p.relative_to(r) / f) for f in artifacts]
f = r / '.github/workflows/pcc-gate.yml'
text = f.read_text()
needle = '          python3 tools/artifacts/artifacts.py pull --jobs 16 '
assert text.count(needle) == 2
lines = text.splitlines(keepends=True)
for i, line in enumerate(lines):
    if line.startswith(needle):
        assert all(a in line for a in artifacts)
        assert line.rstrip('\n').endswith(' '.join(artifacts))
f.write_text(''.join(lines))
assert not subprocess.check_output(['git', 'diff', '--name-only', base, '--', 'src', 'isogeny_algos', 'tools', '.artifacts/config.json', 'pcc/keyring.json'], cwd=r, env=env)
unchanged = ['lean/CubicNormDescent.lean', 'lean/CubicNormWordBridge01.lean',
             'CUBIC_WORD_BRIDGE_01.md', 'report-cubic-word-bridge-01/report.typ',
             'report-cubic-word-bridge-01/REPORT.pdf']
for name in unchanged:
    path = p / name
    prior = subprocess.check_output(['git', 'show', base + ':' + str(path.relative_to(r))], cwd=r, env=env)
    assert path.read_bytes() == prior
for name in ['.github/workflows/chained-transfer-cubic-word-proof-01.yml', '.github/workflows/chained-transfer-cubic-norm-proof-01.yml']:
    assert (r/name).read_bytes() == subprocess.check_output(['git', 'show', base + ':' + name], cwd=r, env=env)
(g/'unchanged-pinned-proof-assets-and-source-scope.json').write_text(json.dumps({'base':base, 'unchanged_proof_assets':unchanged, 'unchanged_original_tools_policy_keyring_native':True, 'four_whole_artifact_paths':artifacts, 'KB':'MCP tools absent; wrapper127, no absence or novelty inference', 'sources_checked':['docs/artifacts.md', 'actual current .github/workflows/pcc-gate.yml', 'actual whole source custody56427e', 'completed exact current word CI96eb44/guard89348c/mergee77015'], 'science_execution':False}, indent=2)+'\n')

def paths():
    return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r,env=env).split(b'\0')) | set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0'))) - {b''})

run('original-merged-parent-ledger-prefix', ['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('authored-prose', ['python3','tools/pcc/pcc.py','lint-prose',*[n.decode() for n in paths() if n.endswith(b'.md')]])
raw=run('unchanged-historical-benchrec', ['python3','tools/benchrec.py','gate'], 1)
assert hashlib.sha256(raw).hexdigest() == 'dc4fc3d1e178e01f99a53162a18e81d96fb8c783b57b25929a986ceb38e17b18'
run('unchanged-proof-scoped-original-wiring', ['python3','tools/artifacts/wire_workflows.py','--check','.github/workflows/chained-transfer-cubic-word-proof-01.yml'])
baseline_workflow=ram/'baseline-workflow'/'pcc-gate.yml'
baseline_workflow.parent.mkdir()
baseline_workflow.write_bytes(subprocess.check_output(['git','show',base+':.github/workflows/pcc-gate.yml'],cwd=r,env=env))
prior_diagnostic=run('original-base-PCC-wiring-diagnostic', ['python3','tools/artifacts/wire_workflows.py','--check',str(baseline_workflow)],1)
current_diagnostic=run('current-PCC-wiring-diagnostic', ['python3','tools/artifacts/wire_workflows.py','--check','.github/workflows/pcc-gate.yml'],1)
assert current_diagnostic==prior_diagnostic

run('unchanged-proof-and-current-PCC-actionlint', ['/workspace/scratch/actionlint-bin/actionlint','-oneline','.github/workflows/chained-transfer-cubic-word-proof-01.yml','.github/workflows/pcc-gate.yml'])
shutil.copyfile(f,g/'actual-current-pcc-gate.yml')

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
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Publish complete lossless original1663-member guarded source as full14170288B XZ45155cf7 of39331840B TARb441b4, original local whole-byte replay56427e. Actual mergedword4415 c8aa and current exact27/0 gates/whole CI96eb44, full8959-key raw/published-ledger guard89348c, normal configured newbranch e77015 and current cross-repository queue9f7e56 remain separate exact scopes. Add exact four required whole objects to both original PCC gate/sign hydration commands before real artifact migration: complete current PCC log, complete current guard, complete published source ledger and entire source archive. Original current-parent ledger/prose/unchanged-proof scoped wiring/actionlint/cached checks0; original PCC template wiring1 is byte-identical to actual base diagnostic, four new hydration paths explicitly checked; unchanged historical benchrec1 exact raw SHA; original immediate artifact-policy1 names exactly those four complete payloads pending genuine upload/full remote replay/bot integration. Whole payload Git object is complete, not a pointer or missing-file waiver. Original verifiers/keyring/tools/policy/native and directly pinned norm/word version01 source/report/workflows unchanged; current successful mathematical gates are not rerun. KB tools absent/wrapper127; original conductor unavailable127; direct repository primary artifact/current workflow/source evidence reviewed without novelty inference. This checkpoint changes metadata/hydration only; no native/scientific population/paid launch/new identity/grant/provider, signed promotion, force/rechain/admin bypass. Actual source publication/current CI/genuine migration/normal guarded merge still follow; runtime portable helper/full LP/NEW28/ABI/admission and seven Consumer metadata objects remain distinct.']
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
