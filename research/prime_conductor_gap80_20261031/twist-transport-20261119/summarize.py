#!/usr/bin/env python3
"""Join native certificates/analysis; no new curve arithmetic or statistics."""
import hashlib
import json
from pathlib import Path
HERE=Path(__file__).resolve().parent
load=lambda p:json.loads((HERE/p).read_text())
cv=load('case0.gpvec');main=load('results/case0-main-b/stdout')
labels=['case0-main-b','main-repeat2','main-repeat3','case0-probe-b','case0-dual-b']
attempts=[]
for label in labels:
    raw=(HERE/'results'/label/'resources.txt').read_text().strip()
    attempts.append({'label':label,'resource_text':raw,'RSS_observed':False})
for label in labels[1:3]:
    assert (HERE/'results'/label/'stdout').read_bytes()==(HERE/'results/case0-main-b/stdout').read_bytes()
summary={
 'schema':'conductor.evaluated-case/v1',
 'field_prime':str(cv[0]),'field_bits':53,'prime_isogeny_degree':str(cv[3]),
 'End_conductor_source':'1','End_conductor_target':str(cv[3]),
 'prime_ratio_integer_bit_length':26,'log2_ratio_native':'25.000042436094976878258764820941918114',
 'source':main['source'],'target':main['codomain'],'source_generator':cv[11],
 'transported_generator':main['images'][0]['image'],'original_subgroup_order':str(cv[10]),
 'twist_scale':str(cv[5]),'map_point_inputs':33,'matched_map_scalar_checks':32,
 'normalized_dual_compositions_checked':33,
 'CM_certificate':'cm-certificate.gpvec','CM_argument':'CM_ARGUMENT.md',
 'formal_proof_compiled':False,'whole_construction_attempts':attempts,
 'constructor_interval':'whole quotient construction plus supplied point evaluation; phases not separated',
 'constructor_output_repeats_identical':True,
 'ECDLP':load('pilot-analysis.json'),'ECDLP_native_audit':load('pilot-audit.json'),
 'ECDLP_independent_check':(HERE/'pilot-independent.stdout').read_text().strip(),
 'scalar':load('results/scalar-l0/manifest.json'),
 'scalar_independent_check':load('scalar-audit.stdout'),
 'negative_controls':load('negative-controls.json'),'raw_archive':load('archive-info.json'),
 'full_32_target_ECDLP_protocol_complete':False,'original_80bit_crossing_complete':False,
 'world_record_claimed':False,'wall_admitted':False,'performance_ranking_admitted':False,
 'producer_evidence':'unsigned/not_covered',
 'binaries':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path('/tmp/conductor-twist-20261119/complete_twisted_pipeline'),Path('/workspace/tools/conductor-local-20261119/ecbench'),Path('/workspace/tools/conductor-local-20261119/ecbench-scalar')]},
}
(HERE/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print('native evidence joined; no arithmetic/statistical aggregation added')
