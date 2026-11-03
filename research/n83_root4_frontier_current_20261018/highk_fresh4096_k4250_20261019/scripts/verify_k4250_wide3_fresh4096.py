#!/usr/bin/env python3
"""Independently replay the K4250 base and all 4096 fresh controls."""
import argparse
import hashlib
import importlib.util
import json
import math
from pathlib import Path

ROOT=Path('/Volumes/SSD990/crypto/worktrees/n83-root4-frontier-20261018')
HERE=Path(__file__).resolve().parent
SOURCE=ROOT/'examples/koblitz_orbit_dlp_fast.rs'
TARGETS=Path('/private/tmp/n83_wide3_fresh4096_20261019/targets.jsonl')
ANSWERS=Path('/private/tmp/n83_wide3_fresh4096_20261019/answers.jsonl')
GROUP=ROOT/'research/n83_indexed5_20261013/analyze.py'
K=4250
STATES=83*K*(K-1)//2+42*K
REQUEST=(4*STATES+2)//3
SLOTS=max(16,1<<(2*REQUEST-1).bit_length())


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def lines(path):return [json.loads(line) for line in path.read_text().splitlines()]
def one(path):
    values=lines(path)
    if len(values)!=1:raise ValueError(f'expected one JSON row: {path}')
    return values[0]


def main():
    parser=argparse.ArgumentParser();parser.add_argument('output',type=Path)
    out=parser.parse_args().output.resolve()
    protocol=json.loads((HERE/'k4250_wide3_fresh4096_protocol.json').read_text())
    source=json.loads((out/'source.json').read_text())
    host=json.loads((out/'host_before.json').read_text())
    usage=json.loads((out/'usage.json').read_text())
    binary=Path(source['command'][0])
    if (protocol['k']!=K or protocol['symmetric_states_expected']!=STATES
            or protocol['index_request_entries_expected']!=REQUEST
            or protocol['pilot_runner_sha256']!=sha(HERE/'pilot_k4250_wide3_fresh4096.py')
            or protocol['targets_sha256']!=sha(TARGETS)
            or protocol['answers_sha256']!=sha(ANSWERS)
            or source['source_sha256']!=sha(SOURCE)
            or source['target_sha256']!=sha(TARGETS)
            or source['binary_sha256']!=sha(binary)
            or source['runner_sha256']!=sha(HERE/'pilot_k4250_wide3_fresh4096.py')
            or source['command'][1:]!=['construct:83:1:4250',str(TARGETS),
                                        '29',str(out/'targets_result.jsonl')]
            or host['physical_memory_bytes']<40*1024**3
            or host['free_percent']<50
            or usage['peak_sampled_rss_bytes']>24*1024**3
            or usage['min_sampled_free_percent']<15):
        raise ValueError('source, inputs, execution or resource admission differs')
    result={'schema':'n83.k4250-wide3-fresh4096-analysis/v1',
            'status':'BOUNDED_UNKNOWN','candidate_id':None,
            'selected_online_candidate_id':None,
            'hardware_tier':'local_correctness_diagnostic',
            'source_sha256':source['source_sha256'],
            'binary_sha256':source['binary_sha256'],
            'runner_sha256':source['runner_sha256'],
            'fixture_sha256':source['target_sha256'],
            'producer_status':usage['status'],
            'peak_sampled_rss_bytes':usage['peak_sampled_rss_bytes']}
    if usage['status']!='PASS_PRODUCER' or usage['exit_code']!=0:
        (out/'analysis.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
        print(json.dumps(result,sort_keys=True));return 1
    required={'KIC_PACKED_ROOT_TABLE':'1','KIC_PACKED_ROOT_TABLE_DENSE':'0',
              'KIC_PACKED_ROOT_TABLE_LOAD75':'0','KIC_ROOT_ONLY11':'1',
              'KIC_INDEXED_ROOT5':'1','KIC_INDEXED_ROOT4':'0',
              'KIC_INDEXED5_LOAD75':'1','KIC_INDEXED5_FIXED30':'0',
              'KIC_INDEXED5_ID30':'1','KIC_ROOT_BLOOM':'1',
              'KIC_ROOT_BLOOM_PROFILE':'wide3','KIC_SYMMETRIC_S3':'1',
              'KIC_SPARSE_CANONICAL':'1','KIC_RANK_THREADS':'4',
              'KIC_TARGET_THREADS':'2','KIC_TARGET_CANCEL':'1',
              'KIC_INDEX_ROOT_BATCH':'1'}
    flags=source['algorithm_flags']
    if any(flags.get(key)!=value for key,value in required.items()):
        raise ValueError('executed algorithm flags differ')
    spec=importlib.util.spec_from_file_location('n83_k4250_16_group',GROUP)
    group=importlib.util.module_from_spec(spec);spec.loader.exec_module(group)
    public=lines(TARGETS);answers=lines(ANSWERS)
    if len(public)!=4096 or len(answers)!=4096 or len(set(map(str,public)))!=4096:
        raise ValueError('fixture count or uniqueness differs')
    points=[];scalars=[]
    for i,(point_json,answer) in enumerate(zip(public,answers)):
        point=group.point_from_json(point_json);scalar=int(answer['scalar'])
        if (answer['fixture_index']!=i or not group.on_curve(point)
                or group.multiply(point,group.R) is not None
                or group.multiply(group.G,scalar)!=point):
            raise ValueError(f'fixture scalar replay differs at {i}')
        points.append(point);scalars.append(scalar)
    summary=one(out/'stdout.jsonl')
    expected={'n':83,'a':1,'orbit_columns':K,'rank':K,
              'factor_base_points':166*K,'regular_states':STATES,
              'root_state_slot_bytes':11,'root_table_slot_bytes':5,
              'root_table_request_entries':REQUEST,
              'root_table_allocated_bytes':5*SLOTS,
              'root_table_layout':'indexed83_5byte_exact_id30',
              'targets':4096,'targets_solved':4096,'targets_failed':0,
              'target_threads':2,'target_cancel':True,
              'root_bloom_enabled':True,'root_bloom_profile':'wide3',
              'root_bloom_hashes':3,'root_bloom_bits_per_request':16,
              'root_bloom_bytes':2*(1<<30),'root_table_indexed5_id30_enabled':True,
              'indexed5_load75_enabled':True,'indexed5_fixed30_enabled':False}
    for key,value in expected.items():
        if summary.get(key)!=value:
            raise ValueError(f'{key}: {summary.get(key)!r} != {value!r}')
    dump_path=out/'base_dump.jsonl';base=group.replay_base(one(dump_path),summary)
    if len(base)!=166*K:raise ValueError('base size differs')
    targets=lines(out/'targets_result.jsonl')
    by_index={row['fixture_index']:row for row in targets}
    if len(targets)!=4096 or set(by_index)!=set(range(4096)):
        raise ValueError('target result count or fixture indices differ')
    online=[]
    for i in range(4096):
        target=by_index[i]
        indices=target['point_indices'];recovered=int(target['recovered_scalar'])
        if (target['fixture_index']!=i
                or group.point_from_json(target['target'])!=points[i]
                or recovered!=scalars[i]
                or target['group_verified'] is not True
                or group.multiply(group.G,recovered)!=points[i]
                or len(indices)!=4
                or any(not 0<=index<len(base) for index in indices)):
            raise ValueError(f'target scalar or indices differ at {i}')
        relation=None
        for index in indices:relation=group.add(relation,base[index])
        if relation!=points[i]:raise ValueError(f'point relation differs at {i}')
        phases=[float(target[key]) for key in ('target_query_ms','target_pdp_ms',
                 'target_relation_check_ms','target_descent_ms',
                 'target_recovery_check_ms')]
        value=float(target['target_ms'])
        if (value<=0 or any(not math.isfinite(v) or v<0 for v in phases)
                or abs(value-sum(phases))>1):
            raise ValueError(f'online phase timing differs at {i}')
        online.append(value)
    result.update({'status':'PASS_INDEPENDENT_REPLAY',
                   'actual_factor_base_points':len(base),
                   'effective_columns':K,'base_dump_sha256':sha(dump_path),
                   'base_hash':summary['base_hash'],
                   'root_table_entries':summary['root_table_entries'],
                   'root_table_allocated_bytes':summary['root_table_allocated_bytes'],
                   'cold_wall_seconds_diagnostic':usage['wall_seconds'],
                   'online_ms_diagnostic':online,
                   'mean_online_ms_diagnostic':sum(online)/len(online)})
    (out/'analysis.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
    print(json.dumps({'status':result['status'],'actual_factor_base_points':len(base),
                      'mean_online_ms_diagnostic':result['mean_online_ms_diagnostic'],
                      'peak_sampled_rss_bytes':result['peak_sampled_rss_bytes']},sort_keys=True))
    return 0


if __name__=='__main__':raise SystemExit(main())
