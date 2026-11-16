#!/usr/bin/env python3
"""Replay secondary N83 nearby-K relation probes on sixteen public points."""
import argparse
import gzip
import hashlib
import json
import math
from pathlib import Path
import runpy

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_indexed5_20261013 import analyze as group
from research.n83_online_finalists_20261018.run import ic_environment

ROOT = Path(__file__).resolve().parent
SINGLE = Path('/Volumes/data/cryptanalysis/n83_nearby_k_current_source_20261021_run2/verify.py')
REPLAY_SUFFIX = runpy.run_path(str(SINGLE))['replay_new_columns']
PRIOR = base.HERE/'k1107_current_source_base/base_dump.jsonl.gz'
PRIOR_SHA256 = '08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc'
PHASES = ('target_query_ms','target_pdp_ms','target_relation_check_ms',
          'target_descent_ms','target_recovery_check_ms')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(k):
    out=ROOT/f'k{k}'
    if not (out/'receipt.json').exists():
        return {'schema':'n83.nearby-k-batch16-replay/v1','k':k,
                'status':'BOUNDED_UNKNOWN','reason':'NO_RECEIPT'}
    launch=json.loads((out/'launch.json').read_text())
    receipt=json.loads((out/'receipt.json').read_text())
    if receipt['status']!='PROCESS_EXITED' or receipt['exit_code']!=0:
        return {'schema':'n83.nearby-k-batch16-replay/v1','k':k,
                'status':'BOUNDED_UNKNOWN','reason':receipt['status']}
    summary=json.loads((out/'stdout.jsonl').read_text())
    rows=[json.loads(line) for line in (out/'targets_result.jsonl').read_text().splitlines() if line]
    dump=json.loads((out/'base_dump.jsonl').read_text())
    targets=[json.loads(line) for line in (out/'targets.jsonl').read_text().splitlines() if line]
    answers=[json.loads(line) for line in (base.ROOT/'research/n83_highk_beyond4405_20261021/screen_answers.jsonl').read_text().splitlines()[:16]]
    prior_blob=gzip.open(PRIOR,'rb').read();prior=json.loads(prior_blob)
    prior_replay=json.loads((base.HERE/'k1107_current_source_base/independent_replay.json').read_text())
    expected_flags={name:value for name,value in ic_environment(out,'root11_dynamic','k1107_root11').items() if name.startswith('KIC_')}
    checks={
        'source_and_binary': launch['source_sha256']==base.SOURCE_SHA256==sha(base.SOURCE) and launch['binary_sha256']==sha(Path(launch['command'][0])),
        'flags_and_target_hash': launch['k']==k and launch['flags']==expected_flags and launch['target_sha256']==sha(out/'targets.jsonl'),
        'receipt_hashes': receipt['stdout_sha256']==sha(out/'stdout.jsonl') and receipt['stderr_sha256']==sha(out/'stderr.log') and receipt['base_dump_sha256']==sha(out/'base_dump.jsonl') and receipt['target_result_sha256']==sha(out/'targets_result.jsonl'),
        'count_and_rank': len(targets)==len(answers)==len(rows)==16 and summary['targets']==summary['targets_solved']==16 and summary['targets_failed']==0 and summary['rank']==summary['orbit_columns']==k and summary['factor_base_points']==166*k,
        'base_identity': dump['base_hash']==summary['base_hash'] and len(dump['factor_base_point_coordinates'])==166*k and all(dump[name][:min(len(dump[name]),len(prior[name]))]==prior[name][:min(len(dump[name]),len(prior[name]))] for name in ('factor_base_point_coordinates','factor_base_point_labels','factor_base_representatives')),
        'prior_base_replay': hashlib.sha256(prior_blob).hexdigest()==PRIOR_SHA256 and prior_replay['status']=='PASS_INDEPENDENT_BASE_REPLAY' and prior_replay['base_dump_sha256']==PRIOR_SHA256,
    }
    if k>1107:
        checks['new_base_suffix_replay']=REPLAY_SUFFIX(dump,prior,k)
    checked=[]
    if len(rows)==len(targets)==len(answers)==16:
        for i,(row,target,answer) in enumerate(zip(rows,targets,answers)):
            relation=None
            indices=row.get('point_indices')
            valid_indices=isinstance(indices,list) and len(indices)==4 and all(isinstance(index,int) and not isinstance(index,bool) and 0<=index<166*k for index in indices)
            if valid_indices:
                for index in indices:
                    relation=group.add(relation,group.point_from_json(dump['factor_base_point_coordinates'][index]))
            q=group.point_from_json(target)
            scalar=int(row['recovered_scalar'])
            valid=(answer['fixture_index']==i and row['fixture_index']==i and row['target']==target and scalar==int(answer['scalar']) and group.multiply(group.G,scalar)==q and row['group_verified'] is True and valid_indices and relation==q and isinstance(row['probes'],int) and row['probes']>0 and all(isinstance(row[name],(int,float)) and math.isfinite(row[name]) and row[name]>=0 for name in PHASES) and abs(math.fsum(row[name] for name in PHASES)-row['target_ms'])<=1)
            checked.append({'fixture_index':i,'verified':valid,'probes':row['probes'],'online_ms':row['target_ms'],'point_indices':indices})
    checks['all_sixteen_targets']=len(checked)==16 and all(row['verified'] for row in checked)
    timing=summary['timing_ms']
    checks['process_accounting']=(math.isfinite(timing['process_total']) and timing['process_total']>0 and all(math.isfinite(timing[name]) and timing[name]>=0 for name in ('setup_total','rank_stage','targets_total')) and math.fsum(timing[name] for name in ('setup_total','rank_stage','targets_total'))<=timing['process_total']+1)
    result={'schema':'n83.nearby-k-batch16-replay/v1','k':k,'status':'PASS_SECONDARY_PROBE_REPLAY' if all(checks.values()) else 'VERIFICATION_FAILED','tier':'shared-host secondary deterministic-probe diagnostic','checks':checks,'targets':checked,'source_sha256':launch['source_sha256'],'binary_sha256':launch['binary_sha256'],'base_dump_sha256':receipt['base_dump_sha256'],'cold_process_ms':timing['process_total'],'usable_points':summary['factor_base_points'],'folded_columns':k}
    (out/'independent_replay.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    return result


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('k',type=int);args=parser.parse_args()
    result=verify(args.k)
    print(json.dumps({'k':args.k,'status':result['status'],'checks':result.get('checks')},sort_keys=True))
    raise SystemExit(0 if result['status']=='PASS_SECONDARY_PROBE_REPLAY' else 2)
