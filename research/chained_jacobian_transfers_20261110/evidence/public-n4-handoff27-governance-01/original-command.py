"""Original data-custody gates, exact policy selection and normal payload commit."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,shutil,subprocess
from ct105_hold27_guard import intact,sha
from ct105_ram_payload_blob_stage27 import stage_payload
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';control=Path('/workspace/scratch/ct104-whole-closed-science-handoff-27-attempt02');base='69abf11274ce1f0d49b8e9a42f75af1791fc1bc8'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
assert Path('/workspace/scratch/ct105-payload27-staging-caller01.exit').read_text().strip()=='0'
d=s/'evidence/public-n4-handoff27-governance-01';d.mkdir();(d/'original-command.py').write_bytes(Path(__file__).read_bytes())
(d/'ram-payload-blob-stage27.py').write_bytes(Path('/workspace/scratch/ct105_ram_payload_blob_stage27.py').read_bytes());(d/'hold27-guard.py').write_bytes(Path('/workspace/scratch/ct105_hold27_guard.py').read_bytes())
for name,p in [('whole-archive-caller.txt',Path('/workspace/scratch/ct104-handoff27-prepare-caller02.txt')),('whole-payload-stage-caller.txt',Path('/workspace/scratch/ct105-payload27-staging-caller01.txt'))]:
 body=p.read_bytes();packed=lzma.compress(body);assert lzma.decompress(packed)==body;(d/(name+'.xz')).write_bytes(packed)
scope=[s.relative_to(r).as_posix(),'.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-15.md','.resume/RESUME.md','problems/README.md']
objects=json.loads((s/'evidence/public-n4-original-relation-rank-character-handoff-27/objects.json').read_text());expected=set(json.loads((control/'expected-migration-paths.json').read_text()));before=intact(control)
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,command,want=0):
 q=subprocess.run(command,cwd=r,capture_output=True);raw=q.stdout+q.stderr
 compressed=len(raw)>60000 or ('whitespace'in name and q.returncode!=0)
 (d/(name+'.txt.xz'if compressed else name+'.txt')).write_bytes(lzma.compress(raw)if compressed else raw);(d/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==want,raw[:1600];return q
pins=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(pins)==343 and all(sha(r/n)==h for n,h in pins.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','scripts','benchmarks','.github/workflows',str((s/'genus3-prime-descent').relative_to(r))],cwd=r)
subjects=[]
for name in paths():
 if name.startswith(b'pcc/claims/'):
  for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
   if proof.get('kind')=='attestation':v=proof['subject'];assert sha(r/v['path'])==v['sha256'];subjects.append(v['path'])
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]])
run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();large=set(stage_payload(r,objects,d/'new-immutable-payload-blob-cache.json'))
 selected_paths=[p for p in ps if p.decode()not in large and p!=b'.resume/RESUME.md'];f=Path('/workspace/scratch/ct105-root-whole27-governance.paths0');f.write_bytes(b'\0'.join(selected_paths)+b'\0')
 subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 # The cache receipt is created by the first stage; include its complete bytes.
 subprocess.run(['git','add','--sparse','--',(d/'new-immutable-payload-blob-cache.json').relative_to(r).as_posix()],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)|{(d/'new-immutable-payload-blob-cache.json').relative_to(r).as_posix().encode()}
 for name in subjects:assert name.encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+name],cwd=r,capture_output=True).returncode==0
 for row in objects:
  # All actual staged data, including ordinary small keeps, must be whole bytes.
  body=subprocess.check_output(['git','show',':'+row['path']],cwd=r);assert len(body)==row['bytes']and hashlib.sha256(body).hexdigest()==row['sha256']and body==Path(row['local_original']).read_bytes()
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();policy=run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
actual={line.split()[1]for line in policy.stdout.decode().splitlines()if line.strip()};assert actual==expected and len(expected)==11
(d/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,all343_original_science_hashes_unchanged=True,original_tools_scripts_workflows_native_benchmarks_policy_unchanged=True,all_original_positive_governance_exits=0,artifact_actual_expected_exit=1,exact_original_policy_selected_paths=sorted(expected),all_new_subject_hashes_and_staging_checked=True,complete_input_hold=before,all_large_payload_inputs_regular_and_git_mode100644=True,new_large_blob_cache='only immutable declared data blobs in RAM, all closed bytes and exposed Git paths recorded; no existing local/nonpromisor object replaced/deleted',scope='unsigned byte-custody governance; remoteoriginal replays/actualpublisher/normalFF/reassembly/terminal27guard checks still required'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GenuineNEW27 payload governance: all343producer scientific hashes/originaltools-scripts-workflows-native-benchmarks-artifactpolicy unchanged; originalledger-prose-workingcachedwhitespace-wiring0, exactnewsubjects stagedorparented and everywhole stagedpayloadblob byte/mode equals closed original. Original artifact gate actual1 selects exactly11declared paths, smallkeeps/full53962BPDF ordinaryGit; no policy exemption or syntheticmanifest. Whole125663108Barchive7005members59closedroots/all3578consumer622producerdelta paths roundtripped, all65886content keys179literal links/all58919prior keys177prior targets/exactconsumerledger/Rootledgerprefix held. Newimmutable declared payloaddata blobs alone use RAMcache with complete compressedcanonicalGitobject hashes/exposedpaths/fullregularRootinput backing/recovery recipe; existinglocal/nonpromisor objects untouched and smalltree-commit-metadata storednormally. Allnative-source-runtime-fullprepared/474CIartifact-logs/matrices-BSGStables/failures-controls-reportdrafts retained. HOLD through actual ORIGINAL remote replays BEFOREactualpublisher/bot, exactnormalFF/whole reassembly/allterminalguards-ownedbytes. Unsigned custody only, no remote/scientific/calibrated promotion.'
]
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();(Path('/workspace/scratch/ct105-payload27-staging-01')/'governance.claim').write_text(claim+'\n')
selected=stage();assert intact(control)==before;subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True)
subprocess.run(['git','commit','--quiet','-m','research: preserve whole relation-rank and source-character evidence through custody27'],cwd=r,check=True)
assert intact(control)==before
assert not subprocess.check_output(['git','status','--porcelain'],cwd=r)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();print(claim,head,'staged',len(selected),'exactpayloadfiles',len(objects),'selected',len(expected),flush=True)
