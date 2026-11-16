"""Original root governance for evidence-only proof/CI projection and normal publication."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='1487e3a55efaacf899ab2553af7ff8fbccae22f6'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
g=s/'evidence/public-n4-concurrent-main-merge-ledger-divergence-governance-01';g.mkdir();shutil.copyfile(__file__,g/'original-command.py')
scope=['problems/README.md','.gitattributes',s.relative_to(r).as_posix(),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-15.md','.resume/RESUME.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd):
 q=subprocess.run(cmd,cwd=r,capture_output=True);b=q.stdout+q.stderr;(g/(name+'.txt.xz'if len(b)>60000 else name+'.txt')).write_bytes(lzma.compress(b)if len(b)>60000 else b);(g/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==0,b[:1000]
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343
for n,h in refs.items():assert hashlib.sha256((r/n).read_bytes()).hexdigest()==h,n
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','scripts','.github/workflows','benchmarks',str(s/'genus3-prime-descent')],cwd=r)
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]])
run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();f=Path('/workspace/scratch/ct120-root-concurrent-main-merge-ledger-divergence.paths0');f.write_bytes(b'\0'.join(p for p in ps if p!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
 for n in selected:
  if n.startswith(b'pcc/claims/'):
   for proof in json.loads((r/n.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     v=proof['subject'];assert hashlib.sha256((r/v['path']).read_bytes()).hexdigest()==v['sha256'];assert v['path'].encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+v['path']],cwd=r,capture_output=True).returncode==0
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree])
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,all343_producer_science_hashes_unchanged=True,original_tools_scripts_native_workflows_benchmarks_unchanged=True,all_recorded_governance_exits=0,new_subjects_exact_and_staged_or_parented=True,scope='unsigned evidence projection governance; exact published CBC fresh-CI/prospective-rank evidence mirrored; no calibrated/global/signed standing'),indent=2)+'\n')
cmd=['python3', 'tools/pcc/pcc.py', 'claim', 'new', '--kind', 'statement', '--claim-type', 'data', '--statement', 'Preserved Rootlineage governance after actualsharedbranch223 mainmerge changed existing2d ledgerprefix: unchangedoriginal PCC check rejects ledger[569], fulloldremote/localledgers/cache/commands/refguardfailure/newrefpublication losslesslyretained/everyoriginalbytearchive roundtrip. Local1487genuinelypublished on new research/chained-transfer-preserved-20261115-ct120 ref, sharedrefnotchanged; noforce/reset/rebase/merge/rechain/sidepick. All343protectedRootscience/originaltools/scripts/native/workflows/benchmarks/artifactpolicy unchanged, originalledgerverify againstpreserved1487/prose/workingcachedwhitespace/wiring/artifact0, exactnewsubjects stagedorparented. ContinuecurrentRootcheckpoint onpreservedbranch withregularcontinuationPR, original3877 hasfrozenledgermergeblock; independentConsumer1573/fullfreshCI/sourcef136 continue. Unsignedgovernance/dataonly, scientificNEW28pending.']
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','commit','--quiet','-m','research: preserve complete history after external ledger divergence'],cwd=r,check=True)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();print(claim,head,len(selected),flush=True)
q=subprocess.run(['python3','/workspace/scratch/publish_normal_git_checkpoint.py','isogeny-labs/crypto',base,head,'research/chained-transfer-preserved-20261115-ct120','/workspace/scratch/ct120-root-concurrent-main-merge-ledger-divergence-publication-01'],cwd=r,capture_output=True);Path('/workspace/scratch/ct120-root-concurrent-main-merge-ledger-divergence-publication-caller01.txt').write_bytes(q.stdout+q.stderr);print('normalpub',q.returncode,flush=True);assert q.returncode==0
