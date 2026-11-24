#!/usr/bin/env python3
"""Compare two saved panel summaries without launching computations."""
import argparse
import hashlib
import json
from pathlib import Path

MODES = ('independent', 'walk', 'two-group')
FILTER_FLAGS = {'--x-min', '--x-max', '--modulus', '--residue'}
VALUE_FLAGS = {'--p', '--curve-a', '--curve-b', '--px', '--py', '--qx', '--qy',
               '--walk-length', '--block-size', '--group-size', '--max-candidates', *FILTER_FLAGS}
BOOL_FLAGS = {'--solver-protocol'}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def flags(argv):
    """Require unambiguous, canonical arguments rather than guessing defaults."""
    parsed = {}
    i = 0
    while i < len(argv):
        arg = argv[i]
        name, equals, value = arg.partition('=')
        if name in parsed or name not in VALUE_FLAGS | BOOL_FLAGS:
            raise ValueError(f'unknown or duplicate argument: {name}')
        if name in BOOL_FLAGS:
            if equals: raise ValueError(f'boolean argument has a value: {name}')
            parsed[name] = True
        else:
            if not equals:
                i += 1
                if i >= len(argv) or argv[i].startswith('--'):
                    raise ValueError(f'missing value for {name}')
                value = argv[i]
            parsed[name] = value
        i += 1
    return parsed


def load(directory):
    directory = Path(directory)
    plan = json.loads((directory / 'plan.json').read_text())
    summary = json.loads((directory / 'summary.json').read_text())
    if sha(directory / 'plan.json') != summary['plan_sha256']:
        raise ValueError(f'plan hash mismatch: {directory}')
    if not summary['measure_source_unchanged'] or not summary['accounting_source_unchanged']:
        raise ValueError(f'source changed during panel: {directory}')
    expected = {(seed, mode) for seed in plan['seeds'] for mode in MODES}
    rows = {}
    for row in summary['runs']:
        key = (row['seed'], row['mode'])
        if key in rows or key not in expected:
            raise ValueError(f'duplicate or unexpected attempt: {directory} {key}')
        name = f'seed-{key[0]}-{key[1]}'
        receipt = json.loads((directory / f'{name}.receipt.json').read_text())
        if row != receipt:
            raise ValueError(f'receipt mismatch: {directory} {key}')
        for suffix, digest_key in (('result.json','result_sha256'), ('checkpoint.json','checkpoint_sha256'),
                                   ('stdout','stdout_sha256'), ('stderr','stderr_sha256')):
            expected_hash = row[digest_key]
            path = directory / f'{name}.{suffix}'
            if expected_hash is not None and (not path.is_file() or sha(path) != expected_hash):
                raise ValueError(f'artifact hash mismatch: {path}')
        if row['result_status'] is not None:
            if row['status'] != 'exited' or row.get('exit_code') != 0 or row['result_sha256'] is None:
                raise ValueError(f'inconsistent completed result: {directory} {key}')
            result = json.loads((directory / f'{name}.result.json').read_text())
            if (result.get('schema') != 'e2e-relation-measurement/v1' or
                result.get('config',{}).get('seed') != key[0] or result.get('config',{}).get('mode') != key[1] or
                result.get('status') != row['result_status'] or result.get('target') != row['target'] or
                result.get('useful_relations') != row['useful_relations'] or result.get('relation_counts') != row['relation_counts'] or
                result.get('source_sha256_at_start') != summary['measure_sha256_at_start'] or
                result.get('accounting_sha256_at_start') != summary['accounting_sha256_at_start']):
                raise ValueError(f'result does not match receipt or sources: {directory} {key}')
        rows[key] = row
    if set(rows) != expected:
        raise ValueError(f'missing attempts: {directory}')
    return plan, summary, rows


def compare(left_dir, right_dir):
    lp, ls, lr = load(left_dir)
    rp, rs, rr = load(right_dir)
    la, ra = flags(lp['common_args']), flags(rp['common_args'])
    nonfilter_left = {k:v for k,v in la.items() if k not in FILTER_FLAGS}
    nonfilter_right = {k:v for k,v in ra.items() if k not in FILTER_FLAGS}
    reasons=[]
    if nonfilter_left != nonfilter_right: reasons.append('non-filter arguments differ')
    if lp['seeds'] != rp['seeds']: reasons.append('seed sequences differ')
    if lp['timeout_seconds'] != rp['timeout_seconds']: reasons.append('per-attempt timeouts differ')
    for key in ('measure_sha256_at_start', 'accounting_sha256_at_start', 'panel_sha256'):
        if ls[key] != rs[key]: reasons.append(f'{key} differs')
    if ls['platform'] != rs['platform']: reasons.append('platform strings differ')
    for side, plan, rows in (('left',lp,lr),('right',rp,rr)):
        for seed in plan['seeds']:
            targets=[rows[(seed,mode)]['target'] for mode in MODES]
            if all(target is not None for target in targets) and any(target != targets[0] for target in targets[1:]):
                reasons.append(f'{side} modes use different targets for seed {seed}')
    pairs=[]
    for key in sorted(set(lr) & set(rr)):
        left,right=lr[key],rr[key]
        target_match = None if left['target'] is None or right['target'] is None else left['target']==right['target']
        if target_match is False: reasons.append(f'target differs for {key}')
        pairs.append({'seed':key[0], 'mode':key[1], 'target_match':target_match,
                      'left_status':left['result_status'], 'right_status':right['result_status'],
                      'left_exit_status':left['status'], 'right_exit_status':right['status'],
                      'left_wall_ns':left['whole_process_wall_ns'], 'right_wall_ns':right['whole_process_wall_ns'],
                      'left_nonzero_b':None if left['relation_counts'] is None else left['relation_counts']['nonzero_b'],
                      'right_nonzero_b':None if right['relation_counts'] is None else right['relation_counts']['nonzero_b'],
                      'left_useful':left['useful_relations'], 'right_useful':right['useful_relations']})
    if any(pair['target_match'] is None for pair in pairs): reasons.append('some targets unavailable')
    if any(pair['left_status'] != 'complete' or pair['right_status'] != 'complete' for pair in pairs): reasons.append('some attempts incomplete or invalid')
    return {'schema':'relation-panel-comparison/v1', 'left_plan_sha256':ls['plan_sha256'],
            'right_plan_sha256':rs['plan_sha256'], 'left_filter':{k:v for k,v in la.items() if k in FILTER_FLAGS},
            'right_filter':{k:v for k,v in ra.items() if k in FILTER_FLAGS}, 'matching_checks_pass':not reasons,
            'mismatch_or_unknown':reasons, 'pairs':pairs,
            'limits':['Same platform string does not establish equal hardware, isolation or resource envelopes.',
                      'Equal per-attempt limits do not establish equal realized total cost.',
                      'These records do not include a matched single-target reference.']}


def main():
    p=argparse.ArgumentParser(); p.add_argument('left'); p.add_argument('right'); p.add_argument('--output',required=True)
    args=p.parse_args()
    try: result=compare(args.left,args.right)
    except (OSError, ValueError, KeyError, TypeError) as exc: p.exit(2,f'comparison refused: {exc}\n')
    with Path(args.output).open('x') as output:
        output.write(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'matching_checks_pass':result['matching_checks_pass'], 'mismatch_or_unknown':result['mismatch_or_unknown']}))

if __name__=='__main__': main()
