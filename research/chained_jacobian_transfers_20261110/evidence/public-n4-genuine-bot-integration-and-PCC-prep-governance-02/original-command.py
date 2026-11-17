"""Ordinary genuine-bot integration and prospective unchanged-verifier CI prep."""
from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='893d05f5145b20d0cd18a84a7185b88c0175184c';bot='532c2d32e475f81e2562315c269faf3532fc9e4a';tag='refs/tags/chained-transfer-ledger-metadata02-43774098763-1';branch='codex/chained-transfer-ledger-immutable-20261115-c019'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
assert subprocess.check_output(['git','rev-parse','MERGE_HEAD'],cwd=r,text=True).strip()==bot
g=s/'evidence/public-n4-genuine-bot-integration-and-PCC-prep-governance-02';g.mkdir();shutil.copyfile(__file__,g/'original-command.py')
receipt=json.loads(Path('/dev/shm/ct144-root-ledger-transport02-terminal-01/whole-success-terminal-receipt.json').read_text());payload=receipt['whole_original_payload'];path=payload['path'];directory=Path(path).parent.as_posix()
env=os.environ.copy();n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
def run(n,c,want=0):
 q=subprocess.run(c,cwd=r,env=env,capture_output=True,timeout=180);b=q.stdout+q.stderr;(g/(n+'.txt.xz')).write_bytes(lzma.compress(b));(g/(n+'.exit')).write_text(str(q.returncode)+'\n');print(n,q.returncode,flush=True);assert q.returncode==want;return q
science=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(science)==343
def intact():
 assert all(hashlib.sha256((r/n).read_bytes()).hexdigest()==h for n,h in science.items())
 assert (r/path).read_bytes()==Path(payload['local_original']).read_bytes()
 assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools/pcc/pcc.py','tools/pcc/test_pcc.py','tools/artifacts','scripts','.artifacts/config.json',str(s/'genus3-prime-descent')],cwd=r)
intact()
scope=[s.relative_to(r).as_posix(),'.github/workflows/pcc-gate.yml','tools/pcc/prepare_legacy_evidence_aliases.py','tools/pcc/test_legacy_evidence_aliases.py','problems/chained-transfer-pcc-evidence-addresses.md','problems/README.md','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-15.md','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.gitattributes']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[n.decode()for n in paths()if n.endswith(b'.md')]]);run('PCC-workflow-actionlint',['/workspace/scratch/actionlint-bin/actionlint','.github/workflows/pcc-gate.yml']);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check']);run('original-archive-local-verify',['python3','tools/artifacts/artifacts.py','verify',directory])
old_bench=(s/'evidence/restored-benchrec-gate-01.txt').read_bytes();bench=run('unchanged-broad-benchrec',['python3','tools/benchrec.py','gate'],1);assert bench.stdout+bench.stderr==old_bench
def stage():
 names=paths();f=Path('/workspace/scratch/ct146-root-integration.paths0');f.write_bytes(b'\0'.join(n for n in names if n not in [b'.resume/RESUME.md',path.encode()])+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(names)
 for name in selected:
  if name.startswith(b'pcc/claims/'):
   for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     v=proof['subject'];assert hashlib.sha256((r/v['path']).read_bytes()).hexdigest()==v['sha256'];assert v['path'].encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+v['path']],cwd=r,capture_output=True).returncode==0
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree])
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),first_parent=base,genuine_second_parent=bot,ordinary_merge_distinct_from_isolated_source_normal_FF=True,all343_science_unchanged=True,whole_original_payload_regular_and_genuinely_ignored=True,actual_canonical_metadata_byte_exact=True,all_positive_governance=0,original_broad_benchrec=1,whole_benchrec_output_exact_historical=True,original_pcc_verifier_core_tests_keyring_claims_not_modified=True,only_PCC_workflow_data_preparation_and_new_metadata_helper=True,original_full_canaries_and_PCC_tests_already_pass_b55dd4=True,fresh_CI_not_yet_observed=True,no_signed_or_scientific_promotion=True),indent=2)+'\n')
c=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GenuineRootmetadata bot532 normal-fetchedandordinarynonFFmergepreparedinto893, separatefromactualisolatedsourceFF0/closedcustody0e8d86. Exactactualcanonicalmanifest/ignoreandcomplete726536Barchivekeptregularignored, all343science/originalpcc.py/coretests/artifacttools/policyunchanged. ExplicitPCCworkflowdata-preponly/newI-Ohelper/protocolb55dd4 retainsallverifier/canary/prefix/prose/test/signingrequirements; correctaliasstaysunsignednot_covered. Ledger/prose/actionlint/wiring/localarchiveverify/cachedwhitespace/artifact0, broadbenchrec1wholebyteequal189oldgaps106records; everynewsubjectexactstagedorparented. ProspectivefreshCI/source-tagpublication, no actualnewCI/merge-PR/scientificNEW28/math/solve/cost/signedacceptance.']
for f in sorted(g.iterdir()):c+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(c,cwd=r,text=True).strip()
entry=f'''# Integrate genuine metadata bot and publish unchanged-verifier evidence preparation

## What was done
{claim}: ordinary merge of genuinebot532 into893 prepared, with exact canonical metadata and whole ignored archive. All343 science and original verifier/core tests/artifact tools/policy unchanged; scoped source gates0 and broad benchmark1 exactly historical. The separate real isolated-source FF/custody result remains0e8d86. [pcc:unverified]

## Decisions made
Commit both real parents and owned metadata, then normal publish and new source tag. Fresh required CI must actually validate the data-address restoration/hydration; keep claims unsigned and every old failure. No PR or main merge bypass. [pcc:unverified]

## Open threads
Actual RootPCC final-head CI and named address problem; Consumer66d2 unit/proof CI; strict historical LP ingestion/CI03, scientificNEW28, EKS admission and original main-ledger/reference/global bridges. [pcc:unverified]

## Errors
No new source acceptance error so far. The previous cold-CI failure and first metadata-test wording failure are retained. Broad benchrec remains1; overall CI has not yet been observed with this source. [pcc:unverified]
'''
for n in ['docs/session-logs/2026-11-16.md',s.relative_to(r).as_posix()+'/QUEUE.md',s.relative_to(r).as_posix()+'/PUBLIC_TARGET_CHECKPOINT.md']:
 with(r/n).open('a')as out:out.write('\n'+datetime.now(timezone.utc).isoformat()+'\n\n'+entry+'\n---\n')
f=r/'.resume/RESUME.md';f.write_text(entry+'\n---\n'+f.read_text());stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);intact()
subprocess.run(['git','commit','--quiet','-m','research: integrate genuine metadata custody and restore frozen PCC data addresses'],cwd=r,check=True);head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();assert subprocess.check_output(['git','show','-s','--format=%P',head],cwd=r,text=True).split()==[base,bot];intact();print(claim,head,flush=True)
out=Path('/workspace/scratch/ct146-root-integration-normal-publication-01');out.mkdir();url='https://github.com/isogeny-labs/crypto.git';ref='refs/heads/'+branch
before=subprocess.run(['git','ls-remote','--heads',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'before.txt').write_bytes(before.stdout+before.stderr);assert before.returncode==0 and before.stdout.decode().split()==[base,ref]
q=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+ref],cwd=r,env=env,capture_output=True,timeout=180);(out/'actual-push.log.xz').write_bytes(lzma.compress(q.stdout+q.stderr));(out/'actual-push.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
after=subprocess.run(['git','ls-remote','--heads',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'after.txt').write_bytes(after.stdout+after.stderr);assert after.returncode==0 and after.stdout.decode().split()==[head,ref]
(out/'receipt.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=head,parents=[base,bot],actual_normal_push=0,no_force=True,exact_readback=True,actual_PCC_CI_pending=True),indent=2)+'\n')
newtag='refs/tags/chained-transfer-ledger-PCC-prep-20261116-'+head[:12];out2=Path('/workspace/scratch/ct146-root-integration-new-tag-01');out2.mkdir();pre=subprocess.run(['git','ls-remote','--tags',url,newtag],cwd=r,env=env,capture_output=True,timeout=60);(out2/'before.txt').write_bytes(pre.stdout+pre.stderr);assert pre.returncode==0 and not pre.stdout
pub=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+newtag],cwd=r,env=env,capture_output=True,timeout=180);(out2/'actual-push.log.xz').write_bytes(lzma.compress(pub.stdout+pub.stderr));(out2/'actual-push.exit').write_text(str(pub.returncode)+'\n');assert pub.returncode==0
post=subprocess.run(['git','ls-remote','--tags',url,newtag],cwd=r,env=env,capture_output=True,timeout=60);(out2/'after.txt').write_bytes(post.stdout+post.stderr);assert post.returncode==0 and post.stdout.decode().split()==[head,newtag]
(out2/'receipt.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=head,new_tag=newtag,normal_new_tag_push=0,exact_readback=True,no_existing_tag_update=True),indent=2)+'\n');print('normalbranch-and-newtag0',newtag,flush=True)
