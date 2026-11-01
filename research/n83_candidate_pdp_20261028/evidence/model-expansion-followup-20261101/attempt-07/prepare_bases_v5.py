#!/usr/bin/env python3
"""Prepare hash-bound metadata and S3 shard indexes for r35-r37."""
import hashlib,json
from pathlib import Path
study=Path(__file__).resolve().parents[3]
protocol_path=study/'protocol-model-full-domain-v5.json'
protocol=json.loads(protocol_path.read_text())
assert hashlib.sha256(protocol_path.read_bytes()).hexdigest()=='3a041db64c7d6d70aa99200fe46500770998de5faffaf2acb7a2e366c855eb57'
attempt=Path(__file__).resolve().parent
source_root=attempt/'base-sources'; assert not source_root.exists(); source_root.mkdir()
tmp_root=Path('/tmp/n83-full-domain-bases-attempt07'); tmp_root.mkdir(parents=True,exist_ok=True)
records=[]
for src in protocol['design']['factor_base_sources']:
 bit=src['coordinate_bit']
 if bit not in (35,36,37): continue
 candidate=src['candidate_id']; idx=study/src['index_path']; assert hashlib.sha256(idx.read_bytes()).hexdigest()==src['index_sha256']
 s3idx=study/src['s3_file_index_path']; assert hashlib.sha256(s3idx.read_bytes()).hexdigest()==src['s3_file_index_sha256']
 index=json.loads(idx.read_text()); assert index['candidate_id']==candidate and index['candidate_x_count_per_curve']==1<<27
 dest=tmp_root/candidate; assert not dest.exists(); dest.mkdir()
 (dest/'index.json').write_bytes(idx.read_bytes())
 saved=source_root/candidate; saved.mkdir(); (saved/'index.json').write_bytes(idx.read_bytes())
 variant=study/'evidence/n83-factorbase-variants'/f'r{bit}-attempt-01'
 family_records={}
 for family,expected in src['family_manifests'].items():
  matches=[p for p in variant.rglob('manifest.json') if p.parent.name==family and hashlib.sha256(p.read_bytes()).hexdigest()==expected]
  assert len(matches)==1,(candidate,family,len(matches))
  raw=matches[0].read_bytes(); (dest/family).mkdir(); (dest/family/'manifest.json').write_bytes(raw)
  (saved/f'{family}-manifest.json').write_bytes(raw)
  family_records[family]={'path':str(matches[0].relative_to(study)),'sha256':expected,'bytes':len(raw)}
 raw_s3=s3idx.read_bytes(); remote=json.loads(raw_s3); assert len(remote['files'])>=780
 (dest/'s3-file-index.json').write_bytes(raw_s3); (saved/'s3-file-index.json').write_bytes(raw_s3)
 rec={'candidate_id':candidate,'coordinate_bit':bit,'index_sha256':src['index_sha256'],'family_manifests':family_records,'s3_file_index_sha256':src['s3_file_index_sha256'],'s3_file_count':len(remote['files']),'s3_file_index_schema':remote.get('schema')}
 (saved/'source-manifest.json').write_text(json.dumps(rec,indent=2)+'\n'); records.append(rec)
assert len(records)==3
(source_root/'source-index.json').write_text(json.dumps({'schema':'n83.full-domain-factor-base-sources/v2','protocol_sha256':hashlib.sha256(protocol_path.read_bytes()).hexdigest(),'prepared_bases':records,'shard_policy':'Fetch only selected immutable S3 shards; verify bytes and SHA-256 before decoding.'},indent=2)+'\n')
print(json.dumps({'prepared_bases':len(records),'protocol_sha256':hashlib.sha256(protocol_path.read_bytes()).hexdigest()},indent=2))
