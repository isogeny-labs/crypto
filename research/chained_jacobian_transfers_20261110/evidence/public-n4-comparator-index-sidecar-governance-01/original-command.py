"""Original policy/byte custody and normal immutable metadata publication."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,re,shutil,subprocess
from ct108_index_sidecar_guard import intact
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='990ec6b152fbc8e5bad4bf31c1231f6aa4d2e465'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-comparator-index-sidecar-01';objects=json.loads((d/'objects.json').read_text());assert len(objects)==2
g=s/'evidence/public-n4-comparator-index-sidecar-governance-01';g.mkdir();shutil.copyfile(__file__,g/'original-command.py');shutil.copyfile('/workspace/scratch/ct108_index_sidecar_guard.py',g/'complete-guard-helper.py')
scope=[s.relative_to(r).as_posix(),'.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-15.md','.resume/RESUME.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd,want=0):
 q=subprocess.run(cmd,cwd=r,capture_output=True);raw=q.stdout+q.stderr;(g/(name+'.txt.xz'if len(raw)>60000 or('whitespace'in name and q.returncode)else name+'.txt')).write_bytes(lzma.compress(raw)if len(raw)>60000 or('whitespace'in name and q.returncode)else raw);(g/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==want;return raw
before=intact();(g/'before-full-guard.json').write_text(json.dumps(before,indent=2)+'\n')
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343 and all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in refs.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','scripts','.github/workflows','benchmarks','.artifacts/config.json',str(s/'genus3-prime-descent')],cwd=r)
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]]);run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();p=Path('/workspace/scratch/ct108-index-sidecar-payload-01.paths0');p.write_bytes(b'\0'.join(n for n in ps if n!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(p),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
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
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,complete_guard=before,all343_original_producer_science_hashes_unchanged=True,all_original_tools_scripts_workflows_benchmarks_policies_unchanged=True,all_positive_gates=0,original_artifact_expected_actual_exit=1,exact_two_selected_paths=sorted(selected),all_regular_source_payload_blobs_whole_byte_equal=True,scope='unsigned complete immutable metadata sidecar preparation; original actual replays-before-bot/exactFF/terminal release pending; whole scientific custody NEW28 remains separate'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Complete immutable comparator failed-index metadata sidecar preparation: whole2140576Bindex/SHAa879984e98342913e4614533083ca6a25aa51c0963cfab03faadea222e5426c7 pinned4a8f8f and whole current guard851696B preserved as regular payloads; original policy actual1 selects exactly these two files, all other original source/ledger/prose/workingcachedwhitespace/wiring gates0. All343original producer scientific hashes/tools/scripts/native/workflows/benchmarks/policy unchanged. Full66706content keys185literal-link states/538consumerownedpaths/consumerHEAD-status-exactledger/Rootledgerprefix held, retains all65886prior keys179links with exactlysix declared released metadata updates after genuine27. Full comparator compiled174696/firsttestfailure32398d/governancewhitespace4a8f8f/artifactcdcaa2 failures retained. HOLD through actual original remote replays BEFORE actual bot, exact normalFF and all terminal guards. Exact canonical entry/relative alias after release only; no syntheticmanifest/policywaiver/historyrewrite. This sidecar is distinct from future whole scientific custody NEW28; no comparator/native/CI result.' ]
for p in sorted(g.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);assert intact()==before
subprocess.run(['git','commit','--quiet','-m','research: preserve complete failed-index metadata in original artifact store'],cwd=r,check=True);head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();assert intact()==before;print(claim,head,'paths',len(selected),flush=True)
q=subprocess.run(['python3','/workspace/scratch/publish_normal_git_checkpoint.py','isogeny-labs/crypto',base,head,'research/chained-jacobian-transfer-20261110','/workspace/scratch/ct108-index-sidecar-normal-publication-01'],cwd=r,capture_output=True);Path('/workspace/scratch/ct108-index-sidecar-normal-publication-caller01.txt').write_bytes(q.stdout+q.stderr);print('normalpub',q.returncode,flush=True);assert q.returncode==0
