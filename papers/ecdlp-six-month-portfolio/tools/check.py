"""Check the manuscript's provenance and recorded-data consistency."""
from pathlib import Path
import hashlib
import json
import re
import tarfile

ROOT=Path(__file__).resolve().parents[1]
sources=json.loads((ROOT/'evidence/sources.json').read_text())
keys=[s['key'] for s in sources]
assert len(keys)==len(set(keys)), 'duplicate source key'
texfiles=[ROOT/'portfolio.tex']+list((ROOT/'sections').glob('*.tex'))
tex='\n'.join(p.read_text() for p in texfiles)
used=set(re.findall(r'\\src\{([^}]+)\}',tex))
assert used<=set(keys), 'unresolved source keys: '+str(used-set(keys))
assert all('\\label{art:'+k+'}' in tex for k in used), 'missing source anchors'
assert not any(ord(c)<32 and c not in '\n\r\t' for c in tex), 'control character in source'

d=json.loads((ROOT/'figures/recorded-data.json').read_text())
p=d['portfolio'];b=d['all_branches']
assert p['repository_count']==19 and len(p['repositories'])==19
assert sum(r['tracked_entries'] for r in p['repositories'])==305367
assert sum(r['window_reachable_commits'] for r in p['repositories'])==34276
assert b['heads']==5144 and b['window_unique_commit_union']==49936
assert all(len(r['commit'])==40 for r in p['repositories'])
f=json.loads((ROOT/'evidence/frontier-record-review.json').read_text())
assert f['sessions']==19 and all(r['hash_match'] for r in f['receipts'])
assert f['measured_outcomes']=={'verified':3178,'exhausted':54}
assert sum(f['all_outcomes'].values())==4848
v=d['volcano']
assert v['measured_records']==46960 and sum(v['outcome_totals'].values())==46960
assert v['outcome_totals']=={'exhausted':14626,'timeout':2064,'verified':30270}
assert sum(x['vertices'] for x in v['campaigns'][0]['coverage'] if x['state']=='audited')==2935
assert v['campaigns'][0]['planned_vertices']==24823
assert v['campaigns'][0]['strata_audited']==51
a=json.loads((ROOT/'evidence/volcano-archive-review.json').read_text())
assert a['checked']==a['manifest_entries']==29409
assert not any(a[k] for k in ['mismatches','missing','unsafe_members','extra_files'])
assert a['archive_sha256']=='10f63f511b17b0f0ae7af5ed7470bafcf861e672036cbe37428369e43d0f4109'
archive=ROOT/'evidence/catalogue.tar.xz'
expected=(ROOT/'evidence/catalogue.tar.xz.sha256').read_text().split()[0]
assert hashlib.sha256(archive.read_bytes()).hexdigest()==expected
with tarfile.open(archive,'r:xz') as tar:
    manifest=tar.extractfile('catalogue/MANIFEST.sha256').read().decode()
    for line in manifest.splitlines():
        digest,name=line.split(None,1)
        assert hashlib.sha256(tar.extractfile('catalogue/'+name).read()).hexdigest()==digest
n=d['n83'];assert n['selection_status']=='BOUNDED_UNKNOWN' and n['selected_candidate_id'] is None
for key,s in n['pipeline_summaries'].items():
    assert abs(sum(x['cold_process_ms'] for x in s['per_point'])/len(s['per_point'])-s['mean_cold_process_ms'])<1e-6
assert n['paired_intervals']['k1120|k1104']['lower_95_ms']<0<n['paired_intervals']['k1120|k1104']['upper_95_ms']
assert len([r for r in d['frontier'] if r['arm']=='ic-compact'])==8
assert len([r for r in d['frontier'] if r['arm']=='ic-pair'])==6
for stem in ['activity','frontier','n83','gpu-stages','volcano-coverage','ml-controls','incumbent-improvements','research-map']:
    for ext in ['pdf','svg','png']:
        assert (ROOT/'figures'/(stem+'.'+ext)).stat().st_size>1000

# When the original sibling checkouts exist, also check the cited source bytes.
repo_parent=ROOT.parents[2]
checked=0
for source in sources:
    if 'repo' not in source or 'sha256' not in source:continue
    local=repo_parent/source['repo']/source['path']
    if local.is_file():
        assert hashlib.sha256(local.read_bytes()).hexdigest()==source['sha256'], 'changed source '+source['key']
        checked+=1
receipt={'check':'manuscript metadata and recorded-data consistency',
         'source_entries':len(keys),'cited_keys':len(used),'locally_rechecked_source_files':checked,
         'figure_sets':8,'status':'pass',
         'not_established':'This check does not replay cryptographic solvers or check Lean proofs.'}
print(json.dumps(receipt,indent=2))
