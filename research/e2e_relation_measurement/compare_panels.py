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
    h=hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda:stream.read(1024*1024),b''):
            h.update(chunk)
    return h.hexdigest()


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


def expected_config(plan, seed, mode):
    parsed=flags(plan['common_args'])
    required=('--p','--curve-a','--curve-b','--px','--py','--qx','--qy')
    if any(key not in parsed for key in required):
        raise ValueError('plan is missing required curve or target arguments')
    defaults={'walk-length':8,'x-min':0,'block-size':4,'group-size':8,
              'max-candidates':1000,'modulus':1,'residue':0}
    expected={key.lstrip('-').replace('-','_'):int(parsed[key]) for key in required}
    for key,default in defaults.items():
        expected[key.replace('-','_')]=int(parsed.get('--'+key,default))
    expected['x_max']=int(parsed.get('--x-max',expected['p']-1))
    expected['solver_protocol']=parsed.get('--solver-protocol',False)
    expected['seed']=seed
    expected['mode']=mode
    return expected


def config_matches(actual, expected):
    # Output and checkpoint paths are per-attempt locations, not plan inputs.
    return isinstance(actual,dict) and all(type(actual.get(key)) is type(value) and actual.get(key)==value for key,value in expected.items())


def load(directory):
    directory = Path(directory)
    plan = json.loads((directory / 'plan.json').read_text())
    summary = json.loads((directory / 'summary.json').read_text())
    if sha(directory / 'plan.json') != summary['plan_sha256']:
        raise ValueError(f'plan hash mismatch: {directory}')
    if (not summary['measure_source_unchanged'] or not summary['accounting_source_unchanged'] or
        summary['measure_sha256_at_start'] != summary['measure_sha256_at_end'] or
        summary['measure_sha256_at_end'] != summary['measure_sha256'] or
        summary['accounting_sha256_at_start'] != summary['accounting_sha256_at_end'] or
        not summary['audit_source_unchanged'] or summary['audit_sha256_at_start'] != summary['audit_sha256_at_end']):
        raise ValueError(f'source changed during panel: {directory}')
    expected = {(seed, mode) for seed in plan['seeds'] for mode in MODES}
    rows = {}
    for row in summary['runs']:
        key = (row['seed'], row['mode'])
        expected_args=expected_config(plan,*key)
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
        if row['result_status'] is None and any(row.get(key) is not None for key in ('target','useful_relations','relation_counts')):
            raise ValueError(f'unfinished attempt claims a result: {directory} {key}')
        audit=row.get('independent_audit')
        if not isinstance(audit,dict):
            raise ValueError(f'missing independent audit status: {directory} {key}')
        audit_status=audit.get('status')
        if audit_status not in ('verified','invalid_or_error','timeout','launch_error','unsupported','not_attempted'):
            raise ValueError(f'unknown independent audit status: {directory} {key}')
        if row['result_status']=='complete' and audit_status=='not_attempted':
            raise ValueError(f'completed result was not audited: {directory} {key}')
        if row['result_status']!='complete' and audit_status!='not_attempted':
            raise ValueError(f'audit not expected for incomplete result: {directory} {key}')
        if audit_status in ('unsupported','not_attempted'):
            if any(audit.get(field) is not None for field in ('wall_ns','result_sha256','stdout_sha256','stderr_sha256','exit_code')):
                raise ValueError(f'inconsistent unrun audit: {directory} {key}')
        elif (type(audit.get('wall_ns')) is not int or audit['wall_ns']<0 or
              audit.get('stdout_sha256') is None or audit.get('stderr_sha256') is None):
            raise ValueError(f'incomplete audit process receipt: {directory} {key}')
        for suffix, digest_key in (('audit.json','result_sha256'),('audit.stdout','stdout_sha256'),('audit.stderr','stderr_sha256')):
            expected_audit_hash=audit.get(digest_key); path=directory / f'{name}.{suffix}'
            if expected_audit_hash is not None and (not path.is_file() or sha(path)!=expected_audit_hash):
                raise ValueError(f'audit artifact hash mismatch: {path}')
        if audit.get('status')=='verified':
            if audit.get('exit_code')!=0 or audit.get('result_sha256') is None:
                raise ValueError(f'inconsistent audit status: {directory} {key}')
            audit_report=json.loads((directory / f'{name}.audit.json').read_text())
            if (audit_report.get('status')!='verified' or audit_report.get('record_sha256')!=row['result_sha256'] or
                audit_report.get('audit_source_sha256')!=summary['audit_sha256_at_start']):
                raise ValueError(f'audit does not match result or source: {directory} {key}')
        if row['result_status'] is not None:
            if row['status'] != 'exited' or row.get('exit_code') != 0 or row['result_sha256'] is None:
                raise ValueError(f'inconsistent completed result: {directory} {key}')
            result = json.loads((directory / f'{name}.result.json').read_text())
            if (result.get('schema') != 'e2e-relation-measurement/v1' or
                not config_matches(result.get('config'),expected_args) or
                result.get('status') != row['result_status'] or result.get('target') != row['target'] or
                result.get('useful_relations') != row['useful_relations'] or result.get('relation_counts') != row['relation_counts'] or
                result.get('counts') != row.get('candidate_counts') or result.get('total') != row.get('observed_total') or
                result.get('source_sha256_at_start') != summary['measure_sha256_at_start'] or
                result.get('accounting_sha256_at_start') != summary['accounting_sha256_at_start']):
                raise ValueError(f'result does not match receipt or sources: {directory} {key}')
        if row.get('observation_source') == 'checkpoint':
            if row['checkpoint_sha256'] is None:
                raise ValueError(f'missing observed checkpoint: {directory} {key}')
            snapshot=json.loads((directory / f'{name}.checkpoint.json').read_text())
            if (snapshot.get('schema') != 'e2e-relation-measurement/v1' or
                not config_matches(snapshot.get('config'),expected_args) or
                snapshot.get('counts') != row.get('candidate_counts') or snapshot.get('total') != row.get('observed_total') or
                snapshot.get('source_sha256_at_start') != summary['measure_sha256_at_start'] or
                snapshot.get('accounting_sha256_at_start') != summary['accounting_sha256_at_start']):
                raise ValueError(f'checkpoint does not match receipt or sources: {directory} {key}')
        elif row.get('observation_source') == 'result':
            if row['result_status'] is None:
                raise ValueError(f'missing observed result: {directory} {key}')
        elif row.get('observation_source') is None:
            if row.get('candidate_counts') is not None:
                raise ValueError(f'candidate counts lack an observation source: {directory} {key}')
        else:
            raise ValueError(f'unknown observation source: {directory} {key}')
        rows[key] = row
    if set(rows) != expected:
        raise ValueError(f'missing attempts: {directory}')
    return plan, summary, rows


def outcome_totals(rows, mode):
    selected=[row for (_, row_mode), row in rows.items() if row_mode == mode]
    candidate_values=[row.get('candidate_counts') for row in selected]
    def counter(name):
        values=[value[name] for value in candidate_values if isinstance(value,dict) and type(value.get(name)) is int and value[name] >= 0]
        return {'observed_attempts':len(values), 'unknown_attempts':len(selected)-len(values), 'observed_sum':sum(values) if values else None}
    def total_observation(name, aggregate):
        values=[row['observed_total'][name] for row in selected if isinstance(row.get('observed_total'),dict) and type(row['observed_total'].get(name)) is int and row['observed_total'][name] >= 0]
        return {'observed_attempts':len(values),'unknown_attempts':len(selected)-len(values),'observed_value':aggregate(values) if values else None}
    return {'attempts':len(selected), 'complete':sum(row['result_status']=='complete' for row in selected),
            'useful_runs':sum(row['result_status']=='complete' and (row['useful_relations'] or 0)>0 for row in selected),
            'nonzero_b_runs':sum(row['relation_counts'] is not None and row['relation_counts']['nonzero_b']>0 for row in selected),
            'nonzero_b_unknown_runs':sum(row['relation_counts'] is None for row in selected),
            'independently_verified_runs':sum(row['independent_audit']['status']=='verified' for row in selected),
            'audit_wall_ns_sum':(sum(row['independent_audit']['wall_ns'] for row in selected if type(row['independent_audit']['wall_ns']) is int) if any(type(row['independent_audit']['wall_ns']) is int for row in selected) else None),
            'observed_child_and_audit_wall_ns':sum(row['whole_process_wall_ns']+(row['independent_audit']['wall_ns'] or 0) for row in selected),
            'audit_wall_unknown_runs':sum(row['independent_audit']['wall_ns'] is None for row in selected),
            'whole_process_wall_ns':sum(row['whole_process_wall_ns'] for row in selected),
            'driver_cpu_ns_sum':total_observation('cpu_ns',sum),
            'peak_rss_native_max':total_observation('peak_rss_native',max),
            'candidates':counter('candidates'), 'filtered':counter('filtered'),
            'identity':counter('identity'), 'duplicate_x':counter('duplicate_x')}


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
    for key in ('measure_sha256_at_start', 'accounting_sha256_at_start', 'audit_sha256_at_start', 'panel_sha256'):
        if ls[key] != rs[key]: reasons.append(f'{key} differs')
    if ls['platform'] != rs['platform']: reasons.append('platform strings differ')
    resource_values=[ls.get('resource_metadata_at_start'),ls.get('resource_metadata_at_end'),rs.get('resource_metadata_at_start'),rs.get('resource_metadata_at_end')]
    if any(not isinstance(value,dict) for value in resource_values):
        resource_status='unavailable'; reasons.append('resource metadata unavailable')
    elif any(not value.get('cpu_model') or not value.get('logical_cpus') or not value.get('parent_limits') or value.get('rss_native_unit') not in ('bytes','KiB') or not isinstance(value.get('thread_environment'),dict) for value in resource_values):
        resource_status='incomplete'; reasons.append('resource metadata incomplete')
    elif not all(value==resource_values[0] for value in resource_values[1:]):
        resource_status='different'; reasons.append('recorded resource metadata differs')
    else:
        resource_status='metadata_match_only'
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
                      'left_useful':left['useful_relations'], 'right_useful':right['useful_relations'],
                      'left_audit_status':left['independent_audit']['status'],'right_audit_status':right['independent_audit']['status']})
    if any(pair['target_match'] is None for pair in pairs): reasons.append('some targets unavailable')
    if any(pair['left_status'] != 'complete' or pair['right_status'] != 'complete' for pair in pairs): reasons.append('some attempts incomplete or invalid')
    if any(pair['left_audit_status'] != 'verified' or pair['right_audit_status'] != 'verified' for pair in pairs): reasons.append('some independent audits not verified')
    return {'schema':'relation-panel-comparison/v1', 'left_plan_sha256':ls['plan_sha256'],
            'right_plan_sha256':rs['plan_sha256'], 'left_filter':{k:v for k,v in la.items() if k in FILTER_FLAGS},
            'right_filter':{k:v for k,v in ra.items() if k in FILTER_FLAGS}, 'matching_checks_pass':not reasons,
            'mismatch_or_unknown':reasons, 'resource_metadata_status':resource_status, 'pairs':pairs,
            'per_mode':{mode:{'left':outcome_totals(lr,mode),'right':outcome_totals(rr,mode)} for mode in MODES},
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
