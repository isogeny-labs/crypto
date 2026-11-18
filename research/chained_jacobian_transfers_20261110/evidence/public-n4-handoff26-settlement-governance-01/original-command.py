"""Governance and normal publication of completed custody metadata only."""
from pathlib import Path
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='b7bc54fcba9ad9a7ac901037c263c0b8e8a300a8'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
g=s/'evidence/public-n4-handoff26-settlement-governance-01';g.mkdir();shutil.copyfile(__file__,g/'original-command.py')
for n in ['outcome.json','original-command.py']:
 shutil.copyfile(Path('/workspace/scratch/ct99-remote-cache-relocation-03')/n,g/('remote-cache-recovery03-'+n))
scope=[str(s.relative_to(r)),'.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','docs/session-logs/2026-11-15.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd):
 q=subprocess.run(cmd,cwd=r,capture_output=True);raw=q.stdout+q.stderr;(g/(name+'.txt.xz'if len(raw)>60000 else name+'.txt')).write_bytes(lzma.compress(raw)if len(raw)>60000 else raw);(g/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==0,raw[:1000]
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343 and all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in refs.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','scripts','.github/workflows','benchmarks',str(s/'genus3-prime-descent')],cwd=r)
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]]);run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();p=Path('/workspace/scratch/ct99-root-handoff26-settled-01.paths0');p.write_bytes(b'\0'.join(x for x in ps if x!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(p),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
 for name in selected:
  if name.startswith(b'pcc/claims/'):
   for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     v=proof['subject'];assert hashlib.sha256((r/v['path']).read_bytes()).hexdigest()==v['sha256'];assert v['path'].encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+v['path']],cwd=r,capture_output=True).returncode==0
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree])
(g/'custody.json').write_text(json.dumps(dict(base=base,all343_original_producer_hashes_unchanged=True,original_tools_scripts_native_workflows_policies_benchmarks_unchanged=True,all_recorded_governance_exits=0,new_subject_hashes_and_staging=True,actual_publication_claim=json.loads(Path('/workspace/scratch/ct99-whole-closed-science-handoff-26/actual-publication.claims.json').read_text())['root'],hold26='released after nine actual remote replays-before-bot/exactFF/all58919inputs177linksownedbytes/whole136572156Barchive7357members68roots',scope='unsigned settlement metadata; native IC100/pairedsampler101/induced proof and fullCI original scopes retained'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Genuine26 settlement governance: all343 original producer science hashes/native/tools/scripts/workflows/verifiers/policies/benchmarks unchanged; original ledger/prose/working-cachedwhitespace/wiring/artifact0 and exact subjects/staging. Original43720549312/job132831389944SUCCESS nine actual remote objects replayed BEFOREbotb7bc54fcba/direct99cb2b204d/exact11paths/two ordinary Git objects including full69112BPDF unchanged. All58919heldinputs177linksownedbytes/whole136572156Barchive7357members68roots/every3777consumerdeltafiles/normalFF-artifact-wiring0/HOLD26RELEASED; all51583prior25keys176links/both exactledgerprefixes/oldarchives retained. Two exact original canonical entry dictionaries/relative aliases/full100-101recordXZroundtrips, no newconsumerremoteread claimed. First whitespace governor and FF01lazyfetch128-before-merge preserved; bounded three-blob hydration and FF02 pass with no history enumeration or source/raw changes. New IC/sampler CI next; futurecustodyNEW27. Unsigned metadata only.']

for p in sorted(g.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','commit','--quiet','-m','research: settle genuine whole IC and sampler custody26'],cwd=r,check=True);head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();print(claim,head,len(selected),flush=True)
q=subprocess.run(['python3','/workspace/scratch/publish_normal_git_checkpoint.py','isogeny-labs/crypto',base,head,'research/chained-jacobian-transfer-20261110','/workspace/scratch/ct99-root-handoff26-settlement-publication-01'],cwd=r,capture_output=True);Path('/workspace/scratch/ct99-root-handoff26-settlement-publication-caller01.txt').write_bytes(q.stdout+q.stderr);print('normalpub',q.returncode,flush=True);assert q.returncode==0
