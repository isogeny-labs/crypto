"""Prepare a draft protocol from six unchanged identities; freeze separately before data."""
from pathlib import Path
import copy, hashlib, json, subprocess

s = Path('research/n83_candidate_pdp_20261028')
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
p = json.loads((s / 'protocol-hilbert-v2.json').read_text())
for key in ('freeze', 'amendments'):
    p.pop(key, None)
commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
binary = s / 'target/release/prime_first_fall'
p.update(protocol_id='N83-PRIME-FIRST-FALL-20261030', version=1, created='2026-10-30',
         question='What highest-form first-fall degree and separate strict filtered-fall certificates hold on all six retained n83 prime m2 ell27 systems?')
p['links'] = {'design': 'PRIME_FIRST_FALL_DESIGN.md'}
p['claims'] = [
    {'id': 'C1', 'class': 'exact_computation', 'text': 'Full source, primality and multiplication ranks plus an explicit annihilator certify highest-form first-fall degree under the written Koszul argument.', 'decided_by': []},
    {'id': 'C2', 'class': 'exact_computation', 'text': 'A nonzero directly checked witness functional certifies a strict filtered fall and LFD lower bound. A zero functional is inconclusive for this affine test; global upper/exact LFD remain unknown.', 'decided_by': []},
    {'id': 'C3', 'class': 'out_of_scope', 'text': 'No solving-degree, elite-base advantage, universal FFD/LFD, timing, independent scientific review or full algebraic formalization claim.', 'decided_by': []}]
for arm in p['arms']:
    arm['instrument'].update(commit=commit, binary_sha256=sha(binary),
        build=[['cargo', 'build', '--locked', '--release', '--manifest-path', str(s / 'Cargo.toml'), '--bin', 'prime_first_fall']])
p['variable_ledger']['factors'] = [
    {'name': n, 'treatment': t, 'how': h} for n,t,h in [
        ('instrument','fixed','One pinned Rust binary, exact v2 Hilbert donor prefix and single thread'),
        ('input','fixed','All six immutable metadata/source/base hashes; no new points, targets or equations'),
        ('dataset','crossed','Original, baseline and spike40 retained identities; duplicate natural inputs are not independent draws'),
        ('characteristic','fixed','Native full Lucas certificate for the saved prime'),
        ('host','measured','Existing-host fallback; CLI EKS role assumption denied and MCP overview timed out; current cluster inventory unavailable'),
        ('order','fixed','Original,baseline,spike40; planted then natural; no clock comparison'),
        ('limits','fixed','60s controls,300s producer,300s replay,2GiB,720s outer,no retry'),
        ('analyst','fixed','Store all six outcomes; zero strict functional is inconclusive; no target selection')]]
p['variable_ledger']['comparisons'][0]['threshold'] = 'All source, primality, rank, expanded-polynomial, dense-closure and fresh replay checks pass; preserve zero-functional exceptions'
p['controls']['positive']['not_applicable_reason'] = 'No extra data arm: 36 small expanded-polynomial controls and 12 dense filtered-closure controls include nonzero strict-fall functionals; inherited explicit rank matrices are retained.'
p['controls']['negative']['not_applicable_reason'] = 'Twelve exceptional-target controls must have zero functional and remain inconclusive. Wrong source suffix, extra source mutation, zero weights, small characteristic, bad/truncated binary data are rejected by native tests.'
p['sample_size']['power']['note'] = 'All six named retained inputs; exact algebraic certificates, no statistical test or independent-sample claim.'
p['sample_size']['preflight_runtime'].update(estimated_total_seconds=720, measured_on='Native development controls only;720s is a fixed budget,not a performance prediction')
def metric(name, definition):
    return {'id': name, 'name': name, 'unit': 'degree', 'kind': 'counted', 'definition': definition,
            'source': 'results/summary.json ' + name, 'aggregation': 'per_row', 'resolution': '1', 'uncertainty': 'none_exact'}
p['metrics'] = {'primary': [metric('first_fall_degree','Least nontrivial highest-form syzygy degree under the certified annihilator argument')],
                'secondary': [metric('first_strict_filtered_fall_degree','First strict filtered fall when the nonzero witness is certified;omit unknown'),
                              metric('last_fall_lower_bound','Strict witness lower bound;omit unknown;not an exact/global upper LFD')]}
p['measurement']['counters'] = [x['id'] for xs in p['metrics'].values() for x in xs]
p['execution']['timeout_seconds'] = 720
p['stopping_rules'] = [
    {'kind':'fixed_n','preregistered':True,'rule':'All six retained cases; runtime controls, producer and a fresh-process exact replay; retain every partial/error/exception'},
    {'kind':'budget','preregistered':True,'rule':'60s controls,300s producer,300s replay,+5s termination grace each;2GiB;720s outer;no retries'}]
p['invalidation_rules'] = ['Any full-source, metadata, base hash, primality, rank, coefficient identity or replay mismatch invalidates',
                           'Zero functional does not certify an affine fall or imply nonexistence',
                           'Clock values are diagnostic;no performance claim']
p['decision_rule'].update(text='Record highest-form FFD only after all checks and replay. Record strict filtered fall and LFD lower bound only for a nonzero functional; retain exceptions and errors explicitly.',
                          verdict_vocabulary=['certified','affine_witness_inconclusive','error','incomplete'])
p['analysis_plan'].update(script={'path':'src/bin/prime_first_fall.rs','sha256':sha(s/'src/bin/prime_first_fall.rs')},
                          inputs=['Six unchanged saved input.json/system.sing pairs'],outputs=['results/summary.json','results/KEY/certificate.json','results/KEY/*.poly','controls.json'])
p['reporting']['graphs'] = ['Per-case certified highest-form FFD and strict LFD lower bounds; unknowns explicit;no quality ranking']
p['literature'] = [
    {'ref':u,'provenance':'retrieved','claim':claim,'verified_by':'Primary definitions/theorem and proof read before implementation;PRIME_FIRST_FALL_DESIGN.md'}
    for u,claim in [('https://arxiv.org/html/2112.05579','Definitions1.3,1.5 and sections2,4 distinguish nontrivial syzygies from actual filtered falls'),
                    ('https://arxiv.org/pdf/2211.13548','Theorem2.7 gives maximal-rank powers for characteristic greater than ell')]]
cases = p['extensions']['cases']
for case in cases:
    directory = Path(case['directory'])
    assert sha(directory/'input.json') == case['input_sha256']
    assert sha(directory/'system.sing') == case['source_sha256']
p['extensions'] = {'cases':cases,'raw_prefix':'s3://setec-astronomy-artifacts/runs/n83-prime-first-fall-20261030-r1/local/',
    'detailed_design':'PRIME_FIRST_FALL_DESIGN.md','file_layout':'PRIME_FIRST_FALL_FORMAT.json',
    'backend_reason':'Existing-host fallback retained. Fresh CLI EKS role assumption denied; MCP inventory timed out. No request for this new run has been submitted to EKS; current cluster inventory unavailable. No IAM/capacity changes.',
    'raw_upload_seconds':30,'retry_policy':'none','formalization':'Written algebraic bridge plus native exact arithmetic and independent small controls; full algebraic formalization and independent scientific review pending',
    'donor':{'path':'src/bin/hilbert_v2.rs','sha256':sha(s/'src/bin/hilbert_v2.rs'),
             'copy_with_adapters':'src/prime_first_fall_donor.rs','copy_sha256':sha(s/'src/prime_first_fall_donor.rs')},
    'source_files':[{'path':name,'sha256':sha(s/name)} for name in ['src/bin/prime_first_fall.rs','src/prime_first_fall_donor.rs','PRIME_FIRST_FALL_DESIGN.md','PRIME_FIRST_FALL_FORMAT.json','run-prime-first-fall.sh']]}
p['freeze'] = {'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None}
out=s/'protocol-prime-first-fall.json'
assert not out.exists()
out.write_text(json.dumps(p,indent=2)+'\n')
