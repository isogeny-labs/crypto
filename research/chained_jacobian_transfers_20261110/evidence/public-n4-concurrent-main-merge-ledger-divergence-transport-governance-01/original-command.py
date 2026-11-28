"""Original policy/byte custody and normal immutable metadata publication."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,re,shutil,subprocess
def intact():
 refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343
 for name,digest in refs.items():assert hashlib.sha256((r/name).read_bytes()).hexdigest()==digest,name
 for row in objects:
  p=r/row['path'];assert p.is_file()and not p.is_symlink()and p.read_bytes()==Path(row['local_original']).read_bytes()and hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256']
 return dict(all343_protected_source_sha256=refs,exact_entire_regular_payloads=objects,original_tool_sha256=hashlib.sha256((r/'tools/artifacts/artifacts.py').read_bytes()).hexdigest(),original_policy_sha256=hashlib.sha256((r/'.artifacts/config.json').read_bytes()).hexdigest())
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='1487e3a55efaacf899ab2553af7ff8fbccae22f6'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-concurrent-main-merge-ledger-divergence-01';objects=json.loads(Path('/workspace/scratch/ct121-ledger-divergence-artifact-custody-01/objects.json').read_text());assert len(objects)==1
g=s/'evidence/public-n4-concurrent-main-merge-ledger-divergence-transport-governance-01';g.mkdir();shutil.copyfile(__file__,g/'original-command.py');shutil.copyfile('/workspace/scratch/ct121_prepare_divergence_metadata_transport_01.py',g/'prospective-original-preparer.py')
scope=[s.relative_to(r).as_posix(),'.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-15.md','.resume/RESUME.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd,want=0):
 q=subprocess.run(cmd,cwd=r,capture_output=True);raw=q.stdout+q.stderr;(g/(name+'.txt.xz'if len(raw)>60000 or('whitespace'in name and q.returncode)else name+'.txt')).write_bytes(lzma.compress(raw)if len(raw)>60000 or('whitespace'in name and q.returncode)else raw);(g/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==want;return raw
before=intact();(g/'before-full-guard.json').write_text(json.dumps(before,indent=2)+'\n')
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343 and all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in refs.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','scripts','.github/workflows','benchmarks','.artifacts/config.json',str(s/'genus3-prime-descent')],cwd=r)
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]]);run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();p=Path('/workspace/scratch/ct121-index-sidecar-payload-01.paths0');p.write_bytes(b'\0'.join(n for n in ps if n!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(p),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
 for name in selected:
  if name.startswith(b'pcc/claims/'):
   for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     v=proof['subject'];assert hashlib.sha256((r/v['path']).read_bytes()).hexdigest()==v['sha256'];assert v['path'].encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+v['path']],cwd=r,capture_output=True).returncode==0
 for row in objects:
  b=subprocess.check_output(['git','show',':'+row['path']],cwd=r);assert len(b)==row['bytes']and hashlib.sha256(b).hexdigest()==row['sha256']and b==Path(row['local_original']).read_bytes()
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();raw=run('original-policy-selected-sidecars',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
selected={m[1].decode()for m in re.finditer(rb'^\s*[0-9,]+\s+([^\n]+)\n',raw,re.M)};assert selected=={v['path']for v in objects}
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,complete_guard=before,all343_original_producer_science_hashes_unchanged=True,all_original_tools_scripts_workflows_benchmarks_policies_unchanged=True,all_positive_gates=0,original_artifact_expected_actual_exit=1,exact_one_selected_path=sorted(selected),all_regular_source_payload_blobs_whole_byte_equal=True,scope='unsigned complete immutable metadata sidecar preparation; original actual replays-before-bot/exactFF/terminal release pending; whole scientific custody NEW28 remains separate'),indent=2)+'\n')
cmd=['python3', 'tools/pcc/pcc.py', 'claim', 'new', '--kind', 'statement', '--claim-type', 'data', '--statement', 'Whole immutable ledger-divergence metadata transport preparation after preserved failures99baab/b50a7e: entire726536B34memberarchive SHAa7072da26d05a0e27ba2f43f26584887979484e1faee0e2e08594fb108e73fe1 keptregularandfullbyteequalexternalcopy, originalunchangedartifactpolicyactual1selectsexactthisonepath; no trimmingrenamingsplittingpolicyexception. All343producerprotectedscience/originaltools-script-native-workflow-policy unchanged; sourceledger/prose/workingcachedwhitespace/wiring0 and everynewsubject exactstagedorparented. ProspectiveMETADATA_TRANSPORT01 freezesRootarchive/source/newsubjects/ledger/science/toolpolicy holds through actual originalupload-and-replays BEFOREbot/exact3pathdirectparent/normalFF/entirepayloadrestoreignored/exactcanonicalentry/terminalguards. Newpreservedbranch normalpublication, stablebase2d snapshot keepsoriginaltrustedpolicy; shared223 mainmerge/original3877 ledger[569]prefixblock untouched. IndependentConsumer1573science remainsseparate and fullscientificNEW28pending. Unsigned sourceprep only, no migration or replay acceptance yet.']
for p in sorted(g.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);assert intact()==before
subprocess.run(['git','commit','--quiet','-m','research: retain whole ledger divergence evidence for original artifact migration'],cwd=r,check=True);head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();assert intact()==before;print(claim,head,'paths',len(selected),flush=True)
q=subprocess.run(['python3','/workspace/scratch/publish_normal_git_checkpoint.py','isogeny-labs/crypto',base,head,'research/chained-transfer-preserved-20261115-ct120','/workspace/scratch/ct121-index-sidecar-normal-publication-01'],cwd=r,capture_output=True);Path('/workspace/scratch/ct121-index-sidecar-normal-publication-caller01.txt').write_bytes(q.stdout+q.stderr);print('normalpub',q.returncode,flush=True);assert q.returncode==0
