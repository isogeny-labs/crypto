#!/usr/bin/env python3
"""Bind independently replayed K4300 wide3 base and 16-point diagnostic to IC1."""
import argparse
import copy
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
TEMPLATE=HERE/'candidate_IC1N83Ckb1a1fb679936PDP4rootRCguidedLAgaussTDdirectISO0h55e77acbaacf.json'
FIXTURE=Path('/Volumes/SSD990/crypto/worktrees/n83-root4-frontier-20261018/research/n83_root4_frontier_current_20261018/fresh_manifest.json')
K=4300
B=166*K
STATES=83*K*(K-1)//2+42*K
REQUEST=(4*STATES+2)//3
SLOTS=max(16,1<<(2*REQUEST-1).bit_length())


def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
def digest(value):return hashlib.sha256(canonical(value)).hexdigest()
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def freeze(path,value):
    blob=(json.dumps(value,sort_keys=True,indent=2)+'\n').encode()
    if path.exists() and path.read_bytes()!=blob:raise ValueError(f'frozen file differs: {path}')
    if not path.exists():path.write_bytes(blob)
    return sha(path)


def main():
    parser=argparse.ArgumentParser();parser.add_argument('pilot',type=Path)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();pilot=args.pilot.resolve();out=args.output.resolve()
    analysis=json.loads((pilot/'analysis.json').read_text())
    source=json.loads((pilot/'source.json').read_text())
    summary=json.loads((pilot/'stdout.jsonl').read_text().strip())
    dump=json.loads((pilot/'base_dump.jsonl').read_text().strip())
    protocol=json.loads((HERE/'k4300_wide3_protocol.json').read_text())
    if (analysis['status']!='PASS_INDEPENDENT_REPLAY'
            or analysis['actual_factor_base_points']!=B
            or analysis['effective_columns']!=K
            or source['runner_sha256']!=sha(HERE/'pilot_k4300_wide3.py')
            or analysis['base_dump_sha256']!=sha(pilot/'base_dump.jsonl')
            or analysis['base_hash']!=summary['base_hash']
            or analysis['base_hash']!=dump['base_hash']
            or analysis['root_table_allocated_bytes']!=5*SLOTS
            or summary['root_table_request_entries']!=REQUEST
            or protocol['runner_sha256']!=source['runner_sha256']):
        raise ValueError('K4300 replay or exact object differs')
    points=dump['factor_base_point_coordinates']
    if len(points)!=B or len(dump['factor_base_representatives'])!=K:
        raise ValueError('base point or representative count differs')
    ordered=sorted(points,key=lambda item:(int(item[0]),int(item[1])))
    record=copy.deepcopy(json.loads(TEMPLATE.read_text()))
    base=record['factor_base']
    base.update({'actual_usable_points':B,'effective_columns':K,
                 'enumerated_set_sha256':digest(ordered),
                 'ordered_base_dump_sha256':sha(pilot/'base_dump.jsonl'),
                 'source_base_hash_blake3':summary['base_hash']})
    bloom=record['point_decomposition']['root_bloom_filter']
    bloom.update({'allocated_bytes':2*(1<<30),'bits_per_request':16,'hashes':3,'profile':'wide3'})
    record['point_decomposition']['variant']=record['point_decomposition']['variant'].replace('bloom_standard','bloom_wide3')
    record['implementation']['algorithm_flags']['root_bloom_profile']='wide3'
    slot=record['point_decomposition']['root_slot']
    slot.update({'allocated_bytes':5*SLOTS,'request_entries':REQUEST})
    record['relation_collection']['stop_criterion']=f'rank_{K}'
    record['relation_linear_algebra']['columns']=K
    record['relation_linear_algebra']['rank_criterion']=K
    body={key:value for key,value in record.items() if key!='candidate_id'}
    candidate_id=f'IC1N83Ckb1a1fb{B}PDP4rootRCguidedLAgaussTDdirectISO0h'+digest(body)[:12]
    record['candidate_id']=candidate_id
    out.mkdir(parents=True,exist_ok=True)
    candidate_path=out/f'candidate_{candidate_id}.json'
    candidate_sha=freeze(candidate_path,record)
    fixture=json.loads(FIXTURE.read_text())
    workload={'schema':'n83.k4300-wide3-local-diagnostic-workload/v1',
              'curve_id':fixture['curve_id'],
              'target_count':fixture['point_count'],
              'targets_sha256':fixture['targets_sha256'],
              'seed_domain':fixture['domain'],
              'scalar_rule':fixture['scalar_rule'],
              'target_independent_preparation':'cold',
              'target_online_mode':'16 points after shared base/index/log setup'}
    workload_id=digest(workload)[:12]
    workload_sha=freeze(out/'workload.json',workload)
    row={'schema':'n83.k4300-wide3-local-ic1-row/v1','candidate_id':candidate_id,
         'workload_id':workload_id,
         'run_id':candidate_id+'W'+workload_id+'R1',
         'status':'PASS_INDEPENDENT_REPLAY',
         'hardware_tier':'local_correctness_diagnostic',
         'candidate_manifest_sha256':candidate_sha,
         'workload_manifest_sha256':workload_sha,
         'analysis_sha256':sha(pilot/'analysis.json'),
         'source_sha256':source['source_sha256'],
         'binary_sha256':analysis['binary_sha256'],
         'actual_usable_points':B,'effective_columns':K,
         'root_table_entries':analysis['root_table_entries'],
         'target_count':16,'solved_target_count':16,
         'mean_online_ms_diagnostic':analysis['mean_online_ms_diagnostic'],
         'cold_wall_seconds_diagnostic':analysis['cold_wall_seconds_diagnostic'],
         'selected_online_candidate_id':None}
    row_sha=freeze(out/'pilot_ic1_row.json',row)
    print(json.dumps({'candidate_id':candidate_id,
                      'candidate_manifest_sha256':candidate_sha,
                      'workload_id':workload_id,'row_sha256':row_sha},sort_keys=True))


if __name__=='__main__':main()
