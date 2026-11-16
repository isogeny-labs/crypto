#!/usr/bin/env python3
"""Attach the checked published isogeny-class status to every curve equation."""
from pathlib import Path
import hashlib
import json

here=Path(__file__).parent
source=here/'VERIFIED_HITS.json'
verified=json.loads(source.read_text())
assert verified['verified_count']==len(verified['records'])==62

# Read-only LMFDB av_fq_isog query, 2026-11-16. Every listed class has at
# least one hyperelliptic equation in its curves field. The 0 flag on the
# +/-26 classes must not be read as nonexistence; their curves field is set.
classes={
    -26:('5.3.a_a_a_a_ba',1),
    -22:('5.3.a_a_a_a_w',2),
    -14:('5.3.a_a_a_a_o',2),
    -10:('5.3.a_a_a_a_k',2),
     10:('5.3.a_a_a_a_ak',2),
     14:('5.3.a_a_a_a_ao',2),
}
checks=[]
for rec in verified['records']:
    t=rec['trace_t']
    label,hyp_count=classes[t]
    assert rec['point_counts']['2']['normalized']==10
    checks.append({'case':rec['case'],'hit_index':rec['hit_index'],
                   'trace_t':t,'LMFDB_isogeny_label':label,
                   'LMFDB_hyperelliptic_models':hyp_count,
                   'isogeny_class_status':'known',
                   'published_sextic_threshold_F9':32,
                   'candidate_F9':10,
                   'exact_equation_priority':'unresolved',
                   'subgroup_transfer':'not_constructed',
                   'complete_attack_cost':'not_measured'})

print(json.dumps({'schema':'ghs.quintic-node-candidate-prior-work/v1',
                  'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'input_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'LMFDB_table':'av_fq_isog',
                  'literature':["https://arxiv.org/pdf/2102.07270",
                                "https://arxiv.org/pdf/2204.06805"],
                  'checks':checks},sort_keys=True))
