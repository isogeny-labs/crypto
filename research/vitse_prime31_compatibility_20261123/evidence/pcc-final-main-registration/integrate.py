#!/usr/bin/env python3
"""Metadata-only integration preserving concurrent main and unpublished history."""
from pathlib import Path
import hashlib, importlib.util, json, subprocess, sys

R = Path('/workspace/scratch/857f9e575d1c/work/vitse-publication-direct')
B = Path('/workspace/scratch/857f9e575d1c/tmp/vitse-before-final-integration')
OLD = '26761272d78ff7108e4829808f2d2fa8ecc296f3'
NEW = '71408278696c957d4b5983d4049bdac3e1bc98ba'
PARENT = '50a9820906698926868f7c97a6f7ed23755ee29d'
STUDY = 'research/vitse_prime31_compatibility_20261123'

def git(*args): return subprocess.check_output(['git', *args], cwd=R)
def sha(data): return hashlib.sha256(data).hexdigest()
def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
def write_json(path, value): write(path, (json.dumps(value, indent=2, sort_keys=True)+'\n').encode())
def rel(path): return str(path.relative_to(R))

assert not B.exists()
assert git('rev-parse','HEAD').decode().strip() == PARENT
paths=git('diff','--cached','--name-only',OLD,'-z').decode().split('\0')[:-1]
files={}
for path in paths:
    data=git('show',':'+path)
    assert (R/path).read_bytes()==data,path
    write(B/'files'/path,data)
    files[path]={'bytes':len(data),'sha256':sha(data)}
old_tree=git('write-tree').decode().strip()
old_prefix=git('show',OLD+':pcc/ledger.jsonl')
new_prefix=git('show',NEW+':pcc/ledger.jsonl')
local=(R/'pcc/ledger.jsonl').read_bytes()
assert local.startswith(old_prefix) and new_prefix.startswith(old_prefix)
tail=local[len(old_prefix):]
entries=[json.loads(x) for x in tail.splitlines()]
assert len(entries)==7 and sum(e['type']=='claim' for e in entries)==6
assert all('sig' not in e for e in entries)
write(B/'ledger-prior-main.jsonl',old_prefix)
write(B/'ledger-final-main.jsonl',new_prefix)
write(B/'ledger-unpublished-tail.jsonl',tail)
manifest={'prior_main':OLD,'final_main':NEW,'tracking_parent':PARENT,
          'prior_staged_tree':old_tree,'files':files,
          'prior_main_prefix_bytes':len(old_prefix),'prior_main_prefix_sha256':sha(old_prefix),
          'original_local_ledger_sha256':sha(local),'original_tail_sha256':sha(tail),
          'final_main_prefix_bytes':len(new_prefix),'final_main_prefix_sha256':sha(new_prefix)}
write_json(B/'snapshot.json',manifest)

active_path='docs/ACTIVE.md'
own=[s for s in (R/active_path).read_text().splitlines() if 'Vitse Prime31 compatibility audit:' in s]
assert len(own)==1
active=git('show',NEW+':'+active_path).decode()
assert 'lane/vitse-prime31-compatibility' not in active
active=active.replace('## Active items\n','## Active items\n'+own[0]+'\n',1).encode()
log_path='docs/session-logs/2026-11-23.md'
old_log=git('show',OLD+':'+log_path); ours_log=(R/log_path).read_bytes()
assert ours_log.startswith(old_log)
log=git('show',NEW+':'+log_path)+ours_log[len(old_log):]
problem_path='problems/README.md'
own_problem=[s for s in (R/problem_path).read_text().splitlines() if '| [Vitse Prime31 full-library validation]' in s]
assert len(own_problem)==1
problem=git('show',NEW+':'+problem_path).decode()
assert 'vitse-prime31-full-library-validation.md' not in problem
separator='| --- | --- | --- | --- |\n'
assert separator in problem
problem=problem.replace(separator,separator+own_problem[0]+'\n',1).encode()

run=subprocess.run(['git','read-tree','--reset','-u',NEW],cwd=R,capture_output=True)
write(B/'read-tree.stdout',run.stdout);write(B/'read-tree.stderr',run.stderr)
assert run.returncode==0
assert git('write-tree').decode().strip()==git('rev-parse',NEW+'^{tree}').decode().strip()
shared={active_path,log_path,problem_path,'pcc/ledger.jsonl'}
for path in paths:
    if path not in shared:write(R/path,(B/'files'/path).read_bytes())
for path,data in [(active_path,active),(log_path,log),(problem_path,problem),('pcc/ledger.jsonl',new_prefix)]:write(R/path,data)

archive=R/STUDY/'evidence/pcc-before-final-main-integration'
registration=R/STUDY/'evidence/pcc-final-main-registration'
write(archive/'ledger-tail.jsonl',tail)
archive_manifest={k:v for k,v in manifest.items() if k!='files'}
archive_manifest['claim_payloads']={path:record for path,record in files.items() if path.startswith('pcc/claims/')}
write_json(archive/'manifest.json',archive_manifest)
for path in archive_manifest['claim_payloads']:write(archive/'claims'/Path(path).name,(B/'files'/path).read_bytes())

spec=importlib.util.spec_from_file_location('official_pcc_final',R/'tools/pcc/pcc.py')
pcc=importlib.util.module_from_spec(spec);sys.modules[spec.name]=pcc;spec.loader.exec_module(pcc)
mapping=[]
for old in entries:
    if old['type']!='claim':continue
    claim_path=R/old['body']['path'];data=claim_path.read_bytes()
    assert data==(archive/'claims'/claim_path.name).read_bytes()
    claim=json.loads(data);assert pcc.claim_digest(claim)==old['body']['claim_sha256']
    assert not any(json.loads(line).get('body',{}).get('claim_id')==claim['id'] for line in new_prefix.splitlines())
    new=pcc.append_entry(R/'pcc/ledger.jsonl','claim',dict(old['body']))
    mapping.append({'claim_id':claim['id'],'claim_sha256':pcc.claim_digest(claim),'claim_file_sha256':sha(data),
                    'prior_envelope':{k:old[k] for k in ['seq','hash','at']},
                    'final_envelope':{k:new[k] for k in ['seq','hash','at']}})
write_json(registration/'registration-map.json',{'canonical_base_commit':NEW,'canonical_prefix_sha256':sha(new_prefix),
           'archived_tail':{'path':rel(archive/'ledger-tail.jsonl'),'sha256':sha(tail)},
           'registrations':mapping,'standing':'unsigned; no independent verdict'})
note=pcc.append_entry(R/'pcc/ledger.jsonl','note',{
    'text':'A further batch of concurrent main merges landed before this study was published. Preserve the complete final-main prefix and archive the entire unpublished intermediate tail, including its prior reconciliation note. Re-register the same six immutable claim payloads using the official append function. Both prior archives, original producer commits, claim bytes, proof pins and original envelopes remain available. No signed verification is asserted.',
    'canonical_base_commit':NEW,'prior_base_commit':OLD,'original_prefix_sha256':sha(old_prefix),
    'original_tail':{'path':rel(archive/'ledger-tail.jsonl'),'sha256':sha(tail)},
    'registration_mapping':{'path':rel(registration/'registration-map.json'),'sha256':sha((registration/'registration-map.json').read_bytes())}})
write_json(registration/'receipt.json',{'canonical_base_commit':NEW,
    'main_prefix_preserved':(R/'pcc/ledger.jsonl').read_bytes().startswith(new_prefix),
    'registrations':len(mapping),'note_seq':note['seq'],'note_hash':note['hash'],
    'ledger_sha256_after':sha((R/'pcc/ledger.jsonl').read_bytes()),
    'pcc_tool_sha256':sha((R/'tools/pcc/pcc.py').read_bytes()),'script_sha256':sha(Path(__file__).read_bytes())})
write(registration/'integrate.py',Path(__file__).read_bytes())
source_paths=['src/cryptanalysis/pkm_smooth_tower.rs','src/cryptanalysis/identity_certificate.rs','src/hash/sha256.rs','src/cryptanalysis/semaev_selector.rs','src/prime_ecc/efd/tests.rs','Cargo.toml','Cargo.lock','tools/pcc/pcc.py','AGENTS.md']
rows=[]
for path in source_paths:
    blobs=[git('ls-tree',ref,'--',path).decode().split()[2] for ref in [OLD,NEW]]
    rows.append({'path':path,'prior_blob':blobs[0],'final_main_blob':blobs[1],'unchanged':blobs[0]==blobs[1]})
write_json(R/STUDY/'evidence/final-main-integration.json',{'prior_main':OLD,'final_main':NEW,
    'sources':rows,'qualification_scope':'Selected-source audit remains bound to unchanged bytes. EFD tests changed, so the old parser-failure diagnosis is historical; current-main root qualification is separately recorded.','publication_parent':PARENT})
print(json.dumps({'main':NEW,'preserved_files':len(paths),'archived_entries':len(entries),
    're_registered_claims':len(mapping),'note_seq':note['seq'],'source_changes':[x['path'] for x in rows if not x['unchanged']]}))
