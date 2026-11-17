#!/usr/bin/env python3
"""Read-only native receipt audit; writes one deterministic summary on success."""
import hashlib, json
from pathlib import Path
from datetime import datetime, timezone
B=Path(__file__).resolve().parent
V1=Path('/var/tmp/ml-dataset-binary-root-bsgs-20261031')
V2=Path('/mnt/spot-codex-state/tmp/ml-dataset-binary-root-bsgs-20261031-v2')
def read(p): return json.loads(p.read_text())
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def lines(p): return sum(1 for x in p.read_text().splitlines() if x)
counts={'native_verified_cells':0,'native_report_exit_zero':0,'strict_export_exit_zero':0,'complete_binary_root_query_certificates':0,'positive_pdp_witnesses':0,'independently_solved_bsgs_ecdlp_cells':0,'bsgs_timeout_cells':0}
row_counts={x:0 for x in ('features.jsonl','ecdlp-features.jsonl','restricted/targets.jsonl','restricted/ecdlp-records.jsonl')}
archive_sizes=[]; public={}; tuples=set()
for i in range(16):
 root=V1 if i<11 else V2
 prefix='protocol' if i<11 else 'protocol-v2'
 plan='plan' if i<11 else 'plan-v2'
 shard=root/f'shard-{i:02d}'
 assert (shard/'attempt-start.json').is_file()
 status=read(shard/'local-status.json'); receipt=read(shard/'runner/receipt.json')
 native=shard/f'runner/attempt/cells/{i:012d}'
 manifest=read(native/'manifest.json'); outcome=manifest['outcome']; config=manifest['config']
 cp=read(B/f'checkpoints/shard-{i:02d}.json')
 assert cp['index']==i and cp['published']['archive_fresh_get'] and cp['published']['pointer_fresh_get']
 assert cp['custody']['protocol_file_sha256']==sha(B/f'{prefix}.json')
 assert cp['custody']['plan_file_sha256']==sha(B/f'{plan}.json')
 assert all(a['sha256_before']==a['sha256_after'] for a in receipt['input_files'])
 assert receipt['binary_sha256_before']==receipt['binary_sha256_after']=='b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e'
 assert receipt['exit_code']==0 and not receipt['timed_out']
 assert status['verify']==status['report']==status['export']==0 and status['sealed_cell_verified'] and status['artifact_hashes_verified']
 assert config['family']=='binary' and config['n']==23 and config['m']==2 and config['seed']==20261031
 assert config['solver']==['root','bsgs'][i%2] and config['twist']==bool((i//2)%2)
 assert config['key_index']==(i//4)%2 and config['generator_index']==(i//8)%2
 tup=(config['solver'],config['twist'],config['generator_index'],config['key_index']);assert tup not in tuples;tuples.add(tup)
 pubsha=sha(native/'public.json'); pair=(config['twist'],config['generator_index'],config['key_index'])
 if pair in public: assert public[pair]==pubsha
 else: public[pair]=pubsha
 if config['solver']=='root':
  assert outcome['status']=='partial' and status['root_certificate_promoted'] and not status['ecdlp_solve_promoted']
  counts['complete_binary_root_query_certificates']+=1
  counts['positive_pdp_witnesses']+=int(status['root_positive_pdp_promoted'])
 else:
  assert outcome['status']=='verified' and status['ecdlp_solve_promoted'] and outcome['scalar_recovered_without_witness'] and outcome['scalar_replay_verified']
  counts['independently_solved_bsgs_ecdlp_cells']+=1
 for k,field in [('native_verified_cells','verify'),('native_report_exit_zero','report'),('strict_export_exit_zero','export')]: counts[k]+=int(status[field]==0)
 archive=root/f'archives/shard-{i:02d}.tar.gz'
 assert sha(archive)==cp['custody']['archive_sha256']==cp['published']['archive_sha256']
 assert archive.stat().st_size==cp['custody']['archive_bytes'];archive_sizes.append(archive.stat().st_size)
 for key in row_counts: row_counts[key]+=lines(shard/'export'/key)
assert len(tuples)==16 and len(public)==8
summary={'schema':'ml-dataset.binary-root-bsgs-campaign-audit/v1','protocol_v1_file_sha256':sha(B/'protocol.json'),'protocol_v2_file_sha256':sha(B/'protocol-v2.json'),'plan_v1_file_sha256':sha(B/'plan.json'),'plan_v2_file_sha256':sha(B/'plan-v2.json'),'planned_cells':16,'observed_indices':list(range(16)),'unique_solver_twist_generator_key_tuples':len(tuples),'paired_public_instance_sha256_matches':len(public),'one_attempt_marker_per_index':True,**counts,'strict_export_rows':row_counts,'local_archive_hashes_and_sizes_checked':16,'archive_total_bytes':sum(archive_sizes),'archive_min_bytes':min(archive_sizes),'archive_max_bytes':max(archive_sizes),'producer_archive_and_pointer_fresh_s3_get_flags':16,'v1_indices':'0..10','v2_indices':'11..15','storage_stop_before_v2':True,'broader_matrix_claim':False}
out=B/'AUDIT-01.json';data=json.dumps(summary,indent=2,sort_keys=True)+'\n'
if out.exists():assert out.read_text()==data
else:out.write_text(data)
print(json.dumps(summary,sort_keys=True))
