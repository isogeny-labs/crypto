#!/usr/bin/env python3
"""Independent metadata-level audit for the completed r35 factor-base bundle."""
import hashlib
import json
from pathlib import Path

root=Path(__file__).resolve().parent
restored=root/'restored'
idx=json.loads((root/'s3-file-index.json').read_text())
by_path={x['path']:x for x in idx['files']}
by_family={}
assert idx['schema']=='n83.factor-base-s3-file-index/v2'
assert idx['candidate_id']=='spike35' and idx['coordinate_bit']==35
assert idx['protocol_sha256']=='7276bd557a5625c358a4d063366a415a0265c078ffb2eddaa74dee40174529b2'
assert len(idx['files'])==786
assert all(x['snapshot_key'].rsplit('/',1)[-1]==x['sha256'] for x in idx['files'])

positions=list(range(26))+[35]
d=json.loads((restored/'data/index.json').read_text())
assert d['schema']=='n83.shared-factor-bases/v2'
assert d['complete'] is True and d['candidate_id']=='spike35'
assert d['coordinate_bit_positions']==positions
assert d['candidate_x_count_per_curve']==1<<27
assert len(d['families'])==3

for family_entry in d['families']:
    family=family_entry['family']
    manifest_path='data/'+family+'/manifest.json'
    m=json.loads((restored/manifest_path).read_text())
    assert hashlib.sha256((restored/manifest_path).read_bytes()).hexdigest()==family_entry['manifest_sha256']
    assert m['candidate_id']=='spike35' and m['family']==family
    assert m['candidate_x_count']==1<<27
    assert m['coordinate_bit_positions']==positions
    assert m['reuse_for_m']==list(range(2,10))
    assert len(m['shards'])==128
    shards=sorted(m['shards'],key=lambda s:s['start'])
    for i,s in enumerate(shards):
        start=i*(1<<20); end=(i+1)*(1<<20)
        assert s['candidate_id']=='spike35' and s['family']==family
        assert s['coordinate_bit_positions']==positions
        assert s['start']==start and s['end']==end and s['candidates']==(1<<20)
        assert s['verified_all_candidates'] is True
        data_path=f'data/{family}/{s["file"]}'
        data_entry=by_path[data_path]
        assert data_entry['bytes']==s['bytes'] and data_entry['sha256']==s['sha256']
        assert data_entry['snapshot_key'].rsplit('/',1)[-1]==s['sha256']
        meta_path=f'data/{family}/part-{i:04}.json'
        meta_entry=by_path[meta_path]
        meta_bytes=(restored/meta_path).read_bytes()
        assert len(meta_bytes)==meta_entry['bytes'] and hashlib.sha256(meta_bytes).hexdigest()==meta_entry['sha256']
        meta=json.loads(meta_bytes)
        assert meta['candidate_id']=='spike35' and meta['family']==family
        assert meta['start']==start and meta['end']==end and meta['verified_all_candidates'] is True
        assert meta['sha256']==s['sha256'] and meta['bytes']==s['bytes']
    points=sum(s['point_count'] for s in shards)
    liftable=sum(s['liftable_x'] for s in shards)
    compressed=sum(s['bytes'] for s in shards)
    assert points==m['point_count']==family_entry['point_count']
    assert liftable==m['liftable_x']==family_entry['liftable_x']
    assert compressed==m['compressed_bytes']==family_entry['compressed_bytes']
    by_family[family]={'point_count':points,'liftable_x':liftable,'compressed_bytes':compressed,'shards':len(shards),'all_128_ranges_contiguous':True,'all_candidates_verified':True,'same_base_for_m':m['reuse_for_m']}

run=json.loads((restored/'run-manifest.json').read_text())
assert run['run_id']=='n83-fb-variants-r35-20261031-r1' and run['exit_code']==0
assert run['candidate_id']=='spike35' and run['coordinate_bit']==35
selfcheck=json.loads((restored/'selfcheck.json').read_text())
assert selfcheck['controls_passed'] is True
assert json.loads((restored/'index-validation.json').read_text()) is True

manifest_lines=(restored/'manifest.sha256').read_text().splitlines()
for line in manifest_lines:
    digest,path=line.split(None,1)
    path=path.lstrip(' *./')
    assert path in by_path and by_path[path]['sha256']==digest, path
assert len(manifest_lines)==784, len(manifest_lines)

report={'schema':'n83.factor-base-structural-audit/v1','verified':True,'candidate_id':'spike35','coordinate_bit':35,'candidate_x_count_per_curve':1<<27,'factor_base_reuse_m':list(range(2,10)),'family_count':3,'shard_count':384,'index_files':len(idx['files']),'manifest_sha256_entries_verified':len(manifest_lines),'families':by_family,'total_points':sum(x['point_count'] for x in by_family.values()),'total_compressed_bytes':sum(x['compressed_bytes'] for x in by_family.values()),'scope':'Complete fixed factor bases and metadata/inventory consistency. No FFD, LFD, PDP degree-of-regularity or solver result is measured by this audit.'}
(root/'structural-audit.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
