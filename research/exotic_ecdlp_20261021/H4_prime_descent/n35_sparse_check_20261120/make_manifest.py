#!/usr/bin/env python3
"""Derive an immutable IC1 candidate identity for the sparse target check."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
BASE=ROOT/'research/exotic_ecdlp_20261021/H4_prime_descent/n35_full_pipeline_20261120/candidate.json'
base=json.loads(BASE.read_text())
source_sha=hashlib.sha256((HERE/'sparse.py').read_bytes()).hexdigest()
record=json.loads(json.dumps(base['record']))
record['target_descent']['relation_check_rule']='verify full three-point sum and signed representative sum over occupied columns only'
record['target_descent']['source_digest']=source_sha
record['implementation']['source_sha256']['sparse_target_check']=source_sha
record['implementation']['algorithm_flags']['sparse_target_relation_check']=True
canonical=json.dumps(record,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
prefix='IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h'
assert base['candidate_id'].startswith(prefix)
candidate_id=prefix+hashlib.sha256(canonical).hexdigest()[:12]
assert candidate_id!=base['candidate_id']
(HERE/'candidate.json').write_text(json.dumps({'candidate_id':candidate_id,'record':record},indent=2,sort_keys=True)+'\n')
print(candidate_id)
