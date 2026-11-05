"""Metadata-only adapter. All polynomial computations and replay are native Rust."""
from pathlib import Path
import copy
import hashlib
import json
import subprocess

s = Path(__file__).resolve().parent
scratch = Path('/home/ec2-user/n83-candidate-recovery')
parent = scratch / 'degree4-propagation-r1'
read = lambda p: json.loads(p.read_text())

def sha(p):
    h = hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda: f.read(1024 * 1024), b''):
            h.update(b)
    return h.hexdigest()

def ref(p):
    return {'sha256': sha(p), 'bytes': p.stat().st_size}

p = read(s / 'protocol-degree4-propagation.json')
p['freeze'] = dict(status='draft', protocol_sha256=None, commit=None, frozen_at=None,
                   filed_before_affected_data=None)
p.update(protocol_id='N83-DEGREE4-QUARTIC-20261029-R1', created='2026-10-29',
         question='Can complete-or-capped quartic closure of the twelve unchanged retained m3 systems produce lower-degree consequences and a global LFD upper4 certificate?',
         links={'design': 'DEGREE4_QUARTIC_DESIGN.md'})
p['claims'][0]['text'] = ('Every admitted row has a degree<=4 derivation. Exhausting the complete product queue certifies the Boolean normal-form image of V4. Only unit or exhaustive affine ideal equality gives global LFD upper4; inherited strict lower4 then gives exact4. No cap or stall gives lower5.')
p['claims'][1]['text'] = ('Universal FFD/LFD, global upper from finite stabilization alone, elite-base/regularity improvement, performance and ECDLP recovery remain outside this batch and open in the broader objective.')
head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=s, text=True).strip()
binary = scratch / 'target/release/degree4_quartic'
names = ['src/bin/degree4_quartic.rs', 'src/degree4_propagation_parent.rs',
         'src/degree4_quartic_seed.rs', 'src/degree4_quartic_impl.rs',
         'DEGREE4_QUARTIC_DESIGN.md', 'DEGREE4_QUARTIC_FORMAT.json', 'run-degree4-quartic.sh']
sources = {name: ref(s / name) for name in names}
limits = dict(max_new_rows=16384, max_new_basis_payload_bytes=2147483648,
              max_product_attempts=131072, max_quartic_nonzeros=1048576,
              affine_free_variables=12)
for arm in p['arms']:
    arm['instrument'].update(commit=head, binary_sha256=sha(binary))
    arm['instrument']['build'][0][-1] = 'degree4_quartic'
    arm['parameters'] = dict(n=83, m=[3], ell=27, seconds=1800, target_kinds=['planted', 'natural'], **limits)
    arm['description'] = arm['id'] + ': replayed complete V3 and V4 seeds; hybrid quartic closure with certificate-only global upper gate'
cases = []
parent_names = ['added.rows.u64le', 'operations.jsonl', 'progress.jsonl', 'state.json',
                'pending.json', 'affine.json', 'propagation.json', 'result.json']
for old in p['extensions']['cases']:
    case = copy.deepcopy(old)
    key, arm, ident = case['key'], case['arm'], case['id']
    directory = parent / 'cases' / key
    result = read(directory / 'result.json')
    assert result['input_binding'] == old
    assert result['last_fall_lower_bound'] == 4
    assert result['state']['restricted_queue_exhausted'] is True
    assert result['state']['added_rows'] == 249
    status = read(parent / 'status' / (key + '.json'))
    assert status['run_exit_code'] == status['replay_exit_code'] == 0
    case['propagation_case'] = copy.deepcopy(old)
    case['propagationdirectory'] = str(directory)
    case['propagation_files'] = {name: ref(directory / name) for name in parent_names}
    case['directory'] = str(parent / 'inputs' / arm / ident)
    case['v2directory'] = str(parent / 'v2' / arm / ident)
    assert sha(Path(case['directory']) / 'input.json') == case['input_json_sha256']
    assert sha(Path(case['directory']) / 'system.sing') == case['system_sha256']
    cases.append(case)
p['controls']['positive']['not_applicable_reason'] = ('Separate native runtime controls: genuine quartic cancellation must produce a linear consequence outside V3; xyz+1 must reach exact4 with exhaustive affine completion. Explicit tiny dense V4 must equal completed hybrid spans. Unit and capped-affine gates have tests.')
p['controls']['negative']['not_applicable_reason'] = ('Separate native xyz control must complete V4 without a global upper certificate. All28 native tests pass, including independent dense checks, full queue coverage, quartic fill-in weight cap with saved reducer prefix, every resource cap, tampering and truncated files.')
for f in p['variable_ledger']['factors']:
    if f['name'] == 'source':
        f.update(value=sources['src/degree4_quartic_impl.rs']['sha256'], how='Native hybrid quartic closure; adapted immutable parent replay returns verified rows; sorted XOR donor documented')
    elif f['name'] == 'host':
        f['how'] = 'Existing host after retained failed EKS storage probe; fresh inventory confirms no duplicate n83 job and two active jobs owned elsewhere; no timing claim'
    elif f['name'] == 'limits':
        f['how'] = '8GiB one native process;16384 new rows;2GiB serialized new-basis payload;131072 attempts;1048576 active quartic nonzeros;12 affine free variables;1800s producer/replay plus5s grace;48000s outer;no automatic retry'
p['variable_ledger']['comparisons'][0]['threshold'] = 'Only unit or exhaustive affine completion gives upper4; inherited strict lower4 gives exact4. Complete V4, rank growth and caps alone never imply a global upper or lower5.'
p['sample_size']['power']['note'] = 'Entire twelve-case retained provenance panel; exact computations with correlated/repeated inputs; no independence or power claim'
p['sample_size']['preflight_runtime'].update(fixed_seconds=0, per_sample_seconds=1800,
    estimated_total_seconds=48000, measured_on='All28 native tests and independent tiny runtime controls pass; numbers are ceilings, not runtime predictions')

def metric(name, definition, state=False, unit='integer'):
    return dict(id=name, name=name.replace('_', ' '), unit=unit, kind='counted',
                definition=definition, source='summary.json rows ' + ('state.' if state else '') + name,
                aggregation='per_row', resolution='1', uncertainty='none_exact')

p['metrics'] = {'primary': [metric('added_rows', 'New independent admitted rows beyond all parents; not base quality', True)],
    'secondary': [metric('attempted_products', 'All product attempts including a blocked attempt', True),
                  metric('completed_products', 'Products fully reduced and consumed from queue', True),
                  metric('new_basis_payload_bytes', 'Serialized admitted row records, excluding headers and blocked candidate', True, 'byte'),
                  metric('pending_products', 'All remaining variable products in complete compact queue', True),
                  metric('linear_rank', 'Rank of admitted degree-one constraints'),
                  metric('last_fall_lower_bound', 'Replayed inherited strict lower4', unit='polynomial degree'),
                  metric('last_fall_upper_bound', '4 only with global unit/affine certificate; absent when unknown', unit='polynomial degree'),
                  metric('last_fall_degree', '4 only when inherited lower4 meets upper4; absent when unknown', unit='polynomial degree')]}
p['measurement']['counters'] = [m['id'] for m in p['metrics']['primary'] + p['metrics']['secondary']]
p['stopping_rules'][0]['rule'] = 'Attempt all12 cases once; unit, full queue exhaustion, new-row cap, attempt cap, or active weight/payload block stops each. Replay every successful producer in a fresh process. No automatic retry.'
p['stopping_rules'][1]['rule'] = '16384 new rows;2GiB new basis payload;131072 attempts;1048576 active quartic nonzeros;1800s producer/replay plus5s grace;8GiB one process;48000s outer. Save all admitted rows, blocked partial candidates and the entire pending queue.'
p['invalidation_rules'] = [
    'Any input/base/parent hash mismatch invalidates case; all four immutable parents must replay before new work',
    'Only sources of degree<=3 may be multiplied; ordinary degree<=4 before original Boolean field reduction',
    'All original equations must lie in initial span; initial basis pivots must be distinct',
    'Complete V4 normal forms requires complete initial cubic coverage and processing all new degree<=3 products, with no pending or blocked operation',
    'Weight cap retains partial reduction and reducer prefix; payload cap retains reduced unadmitted row; neither contributes rank or consumes source',
    'Altered/truncated rows, operations, queue, assignments or verdict invalidate replay',
    'Global upper requires unit or exhaustive affine equality with at most12 free variables; full V4 alone is insufficient',
    'No cap implies lower5; no regularity, speed, independence or elite-base claim follows from closure ranks']
p['analysis_plan'].update(script=dict(path='src/degree4_quartic_impl.rs', sha256=sources['src/degree4_quartic_impl.rs']['sha256']),
    inputs=['protocol-degree4-quartic.json extensions.cases', 'cases/*/result.json', 'status/*.json'])
p['literature'] = [
    dict(ref='https://arxiv.org/html/2112.05579', provenance='retrieved', claim='Definition1.5 and section2 distinguish degree-bounded closure from a global LFD upper certificate', verified_by='Read primary full text before design'),
    dict(ref='https://arxiv.org/html/1504.01175', provenance='retrieved', claim='Section4.5 field multiplier identity supplies inherited degree4 seeds; F4 bound is an assumption', verified_by='Read primary full text and ordinary/Boolean distinction'),
    dict(ref='DEGREE4_PROPAGATION_DESIGN.md', provenance='internal', claim='Previous method never multiplies cubics; no full V4 claim', verified_by=sha(s/'DEGREE4_PROPAGATION_DESIGN.md')),
    dict(ref='../../src/cryptanalysis/sparse_macaulay.rs', provenance='internal', claim='Reuse sorted sparse XOR primitive; retain different graded column convention and streaming replay', verified_by='SHA256 9f794a1e73774dcce0fa24d4468d73a2500d939858857700cfc9fcfe30ab4efa')]
p['reporting']['graphs'] = ['Admitted rows by degree and explicit stop states, pending work and certificate availability; no base-quality ranking']
archives = copy.deepcopy(p['extensions']['parent_archives'])
storage = read(s/'evidence/degree4-propagation-storage.json')
archives.append(dict(role='degree4_propagation', sha256=storage['archive']['sha256'],
    bytes=storage['archive']['bytes'], object=storage['object'], local_root=str(parent)))
p['extensions'] = dict(cases=cases, source_files=sources, parent_archives=archives, limits=limits,
    raw_prefix='s3://setec-astronomy-artifacts/runs/n83-degree4-quartic-20261029-r1/local/',
    detailed_design='DEGREE4_QUARTIC_DESIGN.md', file_layout='DEGREE4_QUARTIC_FORMAT.json',
    backend_reason='Fresh EKS inventory: no duplicate n83 job; retained failed storage probe has unconfirmed application cause; two other-owner jobs active. Existing-host fallback; no IAM/capacity change, timing or CRIU claim.',
    retained_objective='All n83 families,m2..9,FFD/LFD and elite bases remain open; this batch extends unchanged m3 binary systems to complete-or-capped V4 normal forms.')
out = s/'protocol-degree4-quartic.json'
assert not out.exists(), 'Never overwrite a frozen or previous protocol'
out.write_text(json.dumps(p, indent=2)+'\n')
print(json.dumps(dict(source_commit=head, binary_sha256=sha(binary), cases=len(cases), parent_archives=len(archives))))
