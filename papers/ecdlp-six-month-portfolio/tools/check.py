"""Check the manuscript's provenance and recorded-data consistency."""
from pathlib import Path
from collections import Counter, defaultdict
import csv
import hashlib
import io
import json
import re
import subprocess
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
v7=d['volcano_v7']
assert v7['measured_records']==126992
assert v7['outcome_totals']=={'exhausted':39550,'timeout':5776,'verified':81666}
assert v7['relation_trials']==202646648 and v7['decomposition_calls']==193153057
assert v7['relations_found']==4117173
assert sum(x['vertices'] for x in v7['campaigns'][0]['coverage'] if x['state']=='audited')==7937
for filename,entries,digest in [
    ('v7-archive-integrity-review.json',79462,'b8a63f09ca0bfa8de76adf22dd17db5cac95b9623f17068334c8b12b1f0f90a1'),
    ('full-c-archive-integrity-review.json',125,'faeec1aeb755c347e1b22a5bec738e6706c1d1fd0df71683f60f67260ab47865')]:
    receipt=json.loads((ROOT/'evidence'/filename).read_text())
    assert receipt['checked']==receipt['manifest_entries']==entries
    assert receipt['archive_sha256']==digest
    assert not any(receipt[k] for k in ['mismatches','missing','unsafe_members','extra_files','duplicates'])
coverage=json.loads((ROOT/'evidence/reading-coverage-summary.json').read_text())
assert coverage['file_count']==305367 and coverage['literal_everything_read'] is False
assert coverage['reading_status_counts']=={'not_recorded_as_read':305223,'read_in_full':94,'prior_focused_review_only':50}
assert coverage['full_read_unique_blobs']==94 and coverage['full_read_unique_bytes']==1117504
assert not coverage['unacknowledged_emissions']
reading_archive=ROOT/'evidence/reading-coverage.tar.xz'
expected=(ROOT/'evidence/reading-coverage.tar.xz.sha256').read_text().split()[0]
assert hashlib.sha256(reading_archive.read_bytes()).hexdigest()==expected
with tarfile.open(reading_archive,'r:xz') as tar:
    manifest=tar.extractfile('reading-coverage/MANIFEST.sha256').read().decode()
    for line in manifest.splitlines():
        digest,name=line.split(None,1)
        assert hashlib.sha256(tar.extractfile('reading-coverage/'+name).read()).hexdigest()==digest
    assert json.load(tar.extractfile('reading-coverage/summary.json'))==coverage
    rows=csv.DictReader(io.TextIOWrapper(tar.extractfile('reading-coverage/file-reading-coverage.csv'),newline=''))
    counts=Counter(); full_objects=set()
    for row in rows:
        counts[row['reading_status']]+=1
        if row['reading_status']=='read_in_full':full_objects.add(row['object'])
    assert dict(counts)==coverage['reading_status_counts'] and len(full_objects)==94
    events=json.load(tar.extractfile('reading-coverage/reading-events.json'))
    ranges=defaultdict(list);sizes={};totals={}
    for event in events:
        if event['status']!='model_read':continue
        assert 1<=event['start_line']<=event['end_line']<=event['total_lines']
        ranges[event['object']].append((event['start_line'],event['end_line']))
        sizes[event['object']]=event['bytes'];totals[event['object']]=event['total_lines']
    completed=set()
    for obj,intervals in ranges.items():
        end=0
        for lo,hi in sorted(intervals):
            if lo>end+1:break
            end=max(end,hi)
        if end==totals[obj]:completed.add(obj)
    assert completed==full_objects and sum(sizes[obj] for obj in completed)==1117504
external=json.loads((ROOT/'evidence/external-reading.json').read_text())
assert len(external)==8 and all(e['reading_status']=='model_read_entire_returned_text' for e in external)
post=json.loads((ROOT/'evidence/post-integration-reading.json').read_text())
assert len(post)==2
assert all(1<=e['start_line']<=e['end_line']<=e['total_lines'] for e in post)
assert post[0]['start_line']==1 and post[0]['end_line']==post[0]['total_lines']==529
assert post[1]['start_line']==1352 and post[1]['end_line']==1794
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
for stem in ['activity','frontier','n83','gpu-stages','volcano-coverage','ml-controls','incumbent-improvements','research-map','conductor-controls','volcano-latest']:
    for ext in ['pdf','svg','png']:
        assert (ROOT/'figures'/(stem+'.'+ext)).stat().st_size>1000

# When the original sibling checkouts exist, also check the cited source bytes.
repo_parent=ROOT.parents[2]
checked=0
for source in sources:
    if 'repo' not in source or 'sha256' not in source:continue
    local=repo_parent/source['repo']/source['path']
    if local.is_file():
        # Integration can advance a sibling checkout after the scientific pin.
        # Check the cited committed bytes, not unrelated current worktree bytes.
        if 'commit' in source:
            result=subprocess.run(['git','-C',str(repo_parent/source['repo']),
                                   'show',source['commit']+':'+source['path']],
                                  check=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
            data=result.stdout
        else:data=local.read_bytes()
        assert hashlib.sha256(data).hexdigest()==source['sha256'], 'changed source '+source['key']
        checked+=1
receipt={'check':'manuscript metadata and recorded-data consistency',
         'source_entries':len(keys),'cited_keys':len(used),'locally_rechecked_source_files':checked,
         'figure_sets':10,'pinned_full_read_files':94,'exhaustive_reading_complete':False,'status':'pass',
         'not_established':'This check does not replay cryptographic solvers or check Lean proofs.'}
print(json.dumps(receipt,indent=2))
