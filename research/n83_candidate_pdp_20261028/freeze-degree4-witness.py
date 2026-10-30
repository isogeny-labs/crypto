"""Metadata-only protocol adapter; all polynomial/closure computations are native."""
from pathlib import Path
import copy, hashlib, json, subprocess

study = Path(__file__).resolve().parent
scratch = Path('/home/ec2-user/n83-candidate-recovery')
parent = scratch / 'degree3-m3-r1'
suffix = scratch / 'degree3-continuation-r1'
read = lambda p: json.loads(p.read_text())
def sha(path):
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(block)
    return digest.hexdigest()
def reference(path):
    return {'sha256': sha(path), 'bytes': path.stat().st_size}

p = read(study / 'protocol-degree3-continuation.json')
p['freeze'] = {'status': 'draft', 'protocol_sha256': None, 'commit': None,
               'frozen_at': None, 'filed_before_affected_data': None}
p.update(protocol_id='N83-DEGREE4-WITNESS-20261029-R1', created='2026-10-29',
         question='Do the249 coordinate consequences of X0*F, X1*F and U*F contain strict degree4 falls outside the complete retained V3?',
         links={'design': 'DEGREE4_WITNESS_DESIGN.md'})
p['claims'][0]['text'] = 'Every tested candidate has an explicit ordinary-degree<=4 derivation and Boolean normal degree<=3. Nonzero remainder against replayed complete V3 proves LFD>=4 for that exact saved input. Zero remainders exclude only this named finite family. Inherited lower3 remains inherited; exact and global upper degrees stay unknown.'
p['claims'][1]['text'] = 'Universal FFD/LFD claims, complete V4, global LFD upper bounds, elite-base or regularity improvement, timing and ECDLP recovery are outside this batch; the broader objective remains open.'
head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=study, text=True).strip()
binary = scratch / 'target/release/degree4_witness'
source_files = ['src/bin/degree4_witness.rs', 'src/degree4_witness_impl.rs',
                'DEGREE4_WITNESS_DESIGN.md', 'DEGREE4_WITNESS_FORMAT.json',
                'run-degree4-witness.sh']
source_refs = {name: reference(study / name) for name in source_files}
for arm in p['arms']:
    arm['instrument'].update(commit=head, binary_sha256=sha(binary))
    arm['instrument']['build'][0][-1] = 'degree4_witness'
    arm['parameters'] = {'n': 83, 'm': [3], 'ell': 27, 'seconds': 1800,
                         'candidates': 249, 'target_kinds': ['planted', 'natural']}
    arm['description'] = arm['id'] + ': retained completeV3, fixed249 degree4 candidates'
cases = []
for old in p['extensions']['cases']:
    case = copy.deepcopy(old)
    key, arm, ident = case['key'], case['arm'], case['id']
    directory, v2 = suffix / 'inputs' / arm / ident, suffix / 'v2' / arm / ident
    prefix, continuation = parent / 'cases' / key, suffix / 'cases' / key
    assert case['input_json_sha256'] == sha(directory / 'input.json')
    assert case['system_sha256'] == sha(directory / 'system.sing')
    for label, name in [('rows', 'basis.rows.u64le'), ('operations', 'operations.jsonl'), ('result', 'result.json')]:
        assert case['v2'][label + '_sha256'] == sha(v2 / name)
    for name, ref in case['parent_files'].items():
        assert reference(prefix / name) == ref
    state = read(continuation / 'closure_state.json')
    assert state['complete'] and state['pending_candidates'] == 0
    verdict = read(continuation / 'result.json')
    assert verdict['closure_complete'] and verdict['last_fall_lower_bound'] == 3
    case.update(directory=str(directory), v2directory=str(v2),
                prefixdirectory=str(prefix), suffixdirectory=str(continuation))
    case['suffix_files'] = {name: reference(continuation / name) for name in
                           ['continuation.json', 'suffix.rows.u64le', 'suffix.operations.jsonl', 'closure_state.json', 'result.json']}
    case['complete_v3_rank'] = verdict['closure_rank']
    cases.append(case)
p['controls']['positive']['not_applicable_reason'] = 'Not a separate statistical arm: native runtime xyz+1 fixture must yield3 strict degree4 coordinate witnesses; independently enumerated tiny complete V3/V4 spaces must agree, with a fresh byte replay.'
p['controls']['negative']['not_applicable_reason'] = 'Not a separate statistical arm: native runtime xyz fixture must yield0 strict witnesses. Fourteen native tests include field arithmetic, ordinary-degree restrictions, trace/parent completeness, altered reducers/remainders, field-file headers/coefficients/truncation/trailing bytes.'
for factor in p['variable_ledger']['factors']:
    if factor['name'] == 'source':
        factor.update(value=source_refs['src/degree4_witness_impl.rs']['sha256'], how='Frozen new native witness instrument plus separately hash-bound predecessor replay copy')
    elif factor['name'] == 'host':
        factor['how'] = 'Existing host after retained failed EKS storage probe; fresh inventory rechecked successfully after one timeout. No timing claim.'
    elif factor['name'] == 'limits':
        factor['how'] = '8GiB virtual memory, one native process,1800s per producer/replay plus5s grace,48000s outer; all249 candidates per successful case; no automatic retry'
p['variable_ledger']['comparisons'][0]['threshold'] = 'A degree<=3 candidate with degree<=4 source derivation and nonzero highest-pivot remainder against complete verified V3 gives lower4; allzero gives finite-family null only.'
p['sample_size']['power']['note'] = 'All12 retained provenance identities,249 deterministic candidates each; no independence or statistical-power claim'
p['sample_size']['preflight_runtime'].update(fixed_seconds=0, per_sample_seconds=1800,
    estimated_total_seconds=48000, measured_on='Fourteen native control tests passed; recorded budgets are resource ceilings, not measured runtime predictions')
def metric(name, definition, unit='integer'):
    return {'id': name, 'name': name.replace('_', ' '), 'unit': unit,
            'kind': 'counted', 'definition': definition, 'source': 'summary.json rows ' + name,
            'aggregation': 'per_row', 'resolution': '1', 'uncertainty': 'none_exact'}
p['metrics'] = {'primary': [metric('strict_candidates', 'Count among249 named coordinate candidates with verified nonzero remainder against complete V3; correlated witnesses, not independent samples')],
                'secondary': [metric('candidates', 'All named candidate coordinate rows checked'),
                              metric('v3_rank', 'Rank of complete parent V3, reverified and inherited'),
                              metric('last_fall_lower_bound', '4 only with strict new witness; otherwise inherited3', 'polynomial degree')]}
p['measurement']['counters'] = ['strict_candidates', 'candidates', 'v3_rank', 'last_fall_lower_bound']
p['execution']['timeout_seconds'] = 48000
p['stopping_rules'][0]['rule'] = 'Attempt all12 retained cases and all249 candidates per successful case; replay each successful producer in a fresh process; no automatic retry or first-positive stopping'
p['stopping_rules'][1]['rule'] = '1800s per producer/replay plus5s grace,8GiB1process,48000s outer; preserve every partial result and failed attempt'
p['invalidation_rules'] = ['Input/base/parent/suffix hash mismatch invalidates case',
    'Complete V3 replay and exhausted queue are required for strict nonmembership',
    'Candidate degree<=3 must follow from exact ordinary-degree<=4 products and Boolean field reductions',
    'Altered candidate/reducer/remainder bytes invalidate replay',
    'No global upper or exact LFD from finite-family null, resource cap or plateau',
    'No regularity or elite-base conclusion from witness count']
p['analysis_plan'].update(script={'path': 'src/degree4_witness_impl.rs', 'sha256': source_refs['src/degree4_witness_impl.rs']['sha256']},
                         inputs=['protocol-degree4-witness.json extensions.cases', 'cases/*/result.json', 'status/*.json'])
p['literature'] = [
    {'ref': 'https://arxiv.org/html/1504.01175', 'provenance': 'retrieved', 'claim': 'Section4.5 gives the characteristic-two X*S3 Boolean degree-drop identity', 'verified_by': 'Read full formula and its fixed-target limitation before implementation'},
    {'ref': 'https://arxiv.org/html/2112.05579', 'provenance': 'retrieved', 'claim': 'Definitions1.5/2.6 and Theorem2.8 establish strict membership/nonmembership lower certificates', 'verified_by': 'Read ordinary-degree closure and largest-strict-fall definition'},
    {'ref': 'https://arxiv.org/html/1503.08001', 'provenance': 'retrieved', 'claim': 'First-fall and fixed-target trace limitations', 'verified_by': 'Read propositions4.7/4.9 and Remark4.8'},
    {'ref': 'src/bin/degree3_continue.rs', 'provenance': 'internal', 'claim': 'Frozen exact parent/suffix replay', 'verified_by': sha(study / 'src/bin/degree3_continue.rs')}]
p['reporting']['graphs'] = ['Per-case strict candidate counts out of249 and inherited/new LFD lower bounds; no quality ranking']
p['extensions'] = {'cases': cases, 'source_files': source_refs,
    'raw_prefix': 's3://setec-astronomy-artifacts/runs/n83-degree4-witness-20261029-r1/local/',
    'detailed_design': 'DEGREE4_WITNESS_DESIGN.md', 'file_layout': 'DEGREE4_WITNESS_FORMAT.json',
    'backend_reason': 'Fresh EKS inventory recovered after one timeout; no duplicate n83 execution. Prior n83 storage probe remains failed with unconfirmed application cause. Existing-host fallback, no IAM/capacity changes and no timing claim.',
    'parent_archives': [
        {'role': 'immutable_v3_prefix', 'sha256': '37d67c62bc7a551c54485693a613a22384144ace1fdeb713802b19438b5e32d4', 'bytes': 1047336364, 'object': 's3://setec-astronomy-artifacts/store/sha256/37/37d67c62bc7a551c54485693a613a22384144ace1fdeb713802b19438b5e32d4', 'local_root': str(parent)},
        {'role': 'complete_v3_suffix', 'sha256': 'b6101a8e8a33b96ba6f6522802343eec2104b74e07b868279d4050765f059a4c', 'bytes': 285100024, 'object': 's3://setec-astronomy-artifacts/store/sha256/b6/b6101a8e8a33b96ba6f6522802343eec2104b74e07b868279d4050765f059a4c', 'local_root': str(suffix)}],
    'retained_objective': 'All n83 families,m2..9,FFD/LFD and elite bases remain open; this batch supplies exact strict-fall or finite-family null data on retained m3 binary systems'}
path = study / 'protocol-degree4-witness.json'
assert not path.exists(), 'Never overwrite a frozen or prior protocol'
path.write_text(json.dumps(p, indent=2) + '\n')
print(json.dumps({'source_commit': head, 'binary_sha256': sha(binary), 'cases': len(cases), 'planned_candidates': 249 * len(cases)}))
