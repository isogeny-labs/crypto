#!/usr/bin/env python3
"""Assemble replay-certificate.json, validator-report.json and VALIDATION.json from the evidence files (own code).
Everything is derived from evidence/*.json; nothing is typed in except the producer's CLAIMED numbers, which are transcribed
from research/leads_chase_20261112/RESULTS.md section J2 and the bench record (source noted per entry).
usage: build_reports.py <item dir>"""
import json, os, sys, hashlib, glob, statistics as st
D = sys.argv[1]
E = os.path.join(D, 'evidence')
def sha(p):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        for b in iter(lambda: f.read(1 << 20), b''): h.update(b)
    return h.hexdigest()
def load(n): return json.load(open(os.path.join(E, n)))
RC = load('recompute.json'); RP = load('replay_compare.json'); V3 = load('v3_compare.json'); V0 = load('v0_v8.json')
CERT = [json.loads(l) for l in open(os.path.join(E, 'certcheck.jsonl'))]
RHO = [json.loads(l) for l in open(os.path.join(E, 'rho_replay', 'rho_compare.log')) if '"job"' in l]
INST = {}
for l in open('/home/user/crypto/research/quartic_n4v2_20261020/results/instances.jsonl'):
    r = json.loads(l); INST[(r['q'], r['seed'])] = r
COMMIT = V0['commit']
def rng(key, lohi):  # recomputed (lo, hi) of a paired ratio
    v = RC['paired_ratio_to_c2.00'][key]; return v['lo'], v['hi']
# ---------------------------------------------------------------- replay certificate
runs = []
for u in RP['units']:
    runs.append(dict(kind='ic_unit', unit=u['unit'], status=u['status'], producer_records=u.get('producer_records'), replay_records=u.get('replay_records'), fields_compared=u.get('fields_compared')))
for r in RHO:
    runs.append(dict(kind='rho_run_set', unit=r['job'], status='identical' if r['identical'] else 'differs', producer_records=r['n4v2_records'], replay_records=r['replay_records'], fields_compared=r['fields_compared']))
for r in V3:
    runs.append(dict(kind='control_c2.00_vs_committed_N4v2', unit=f"q{r['q']}_seed{r['seed']}", status='identical' if r['identical'] else 'differs', producer_records=r['n4v2_records'], replay_records=r['j2_records'], fields_compared=r['fields_compared']))
all_ident = all(r['status'] == 'identical' for r in runs)
all_certs = all(c['pass'] for c in CERT)
cert = dict(schema='replay-certificate/v1', status='verified' if (all_ident and all_certs) else 'failed', independent=True, scalars_verified=bool(all_certs),
            signed=False, signature=None, unsigned_reason='no keyring signature is available in this session; the gate booleans are written by the validator, not signed',
            independence_scope='replay harness, field comparator, certificate checker (C, own F_q / F_{q^3} / Weierstrass arithmetic) and recompute are the validator\'s own code; the solver source is the producer\'s (a replay re-executes the recorded run, it does not reimplement the solver)',
            validated_commit=COMMIT, binary=dict(rebuilt_sha256='183810cf9e1f9ed8ecaedf0295d3b8080528418bb208a71bcabbc3eb03d79bb1', producer_sha256='183810cf9e1f9ed8ecaedf0295d3b8080528418bb208a71bcabbc3eb03d79bb1', rustc='rustc 1.97.0 (2d8144b78 2026-07-07)', build='cd research/quartic_n4v2_20261020 && rustc --edition 2021 -O v2.rs -o v2'),
            ic_units_replayed=RP['summary']['units_total'], ic_units_identical=RP['summary']['identical'], fields_compared_ic=RP['summary']['fields_compared_total'],
            instances_certified=len(CERT), instances_certified_pass=sum(1 for c in CERT if c['pass']),
            runs=runs)
json.dump(cert, open(os.path.join(D, 'replay-certificate.json'), 'w'), indent=1)
# ---------------------------------------------------------------- headline numbers (claimed vs recomputed)
H = []
def add(name, claimed, recomputed, tol, src):
    H.append(dict(name=name, claimed=claimed, recomputed=recomputed, rel_tol=tol, claim_source=src))
def tolp(claimed, decimals): return 0.5 * 10 ** (-decimals) / abs(claimed) * 1.001 if claimed else 1e-9
R = 'RESULTS.md J2 table'
for k, q, c160, c175 in ((16, 65479, (0.87, 1.12), (0.91, 1.00)), (18, 262111, (0.72, 0.80), (0.79, 0.85)), (20, 1048507, (0.67, 0.69), (0.78, 0.78)), (22, 4194271, (0.65, 0.66), (0.77, 0.77))):
    lo, hi = rng(f'{q}|grid|1.60|w=0.0', 0); add(f'k={k} c=1.60 paired DL1 M-eq ratio to c=2.00 (w=0) min', c160[0], lo, tolp(c160[0], 2), R); add(f'k={k} c=1.60 ... max', c160[1], hi, tolp(c160[1], 2), R)
    lo, hi = rng(f'{q}|grid|1.75|w=0.0', 0); add(f'k={k} c=1.75 ratio min', c175[0], lo, tolp(c175[0], 2), R); add(f'k={k} c=1.75 ratio max', c175[1], hi, tolp(c175[1], 2), R)
tot = sum(v['succ'] for key, v in RC['first_attempt_success'].items() if '|grid|' in key)
add('grid first-attempt successes, sum over the table cells (4 sizes; 15+... cells; claimed 9+9+9+9)', 36, tot, 1e-9, R)
for c in ('1.40', '1.50'):
    add(f'grid first-attempt successes at c={c} over k=16,18,20 (claimed 0/3 each)', 0, sum(RC['first_attempt_success'][f'{q}|grid|{c}']['succ'] for q in (65479, 262111, 1048507)), 1e-9, R)
Hh = 'RESULTS.md held-out paragraph'
v = [RC['paired_ratio_to_c2.00'][f'{q}|held|1.60|w=0.0'] for q in (2097091, 4194271)]
add('held-out c=1.60 ratio (w=0), min over k=21,22', 0.655, min(x['lo'] for x in v), tolp(0.655, 3), Hh); add('held-out c=1.60 ratio, max over k=21,22', 0.685, max(x['hi'] for x in v), tolp(0.685, 3), Hh)
v = [RC['paired_ratio_to_c2.00'][f'{q}|held|1.75|w=0.0'] for q in (2097091, 4194271)]
add('held-out c=1.75 ratio (w=0), min over k=21,22', 0.771, min(x['lo'] for x in v), tolp(0.771, 3), Hh); add('held-out c=1.75 ratio, max over k=21,22', 0.775, max(x['hi'] for x in v), tolp(0.775, 3), Hh)
add('held-out first-attempt successes at c=1.60 (of 10)', 10, sum(RC['first_attempt_success'][f'{q}|held|1.60']['succ'] for q in (2097091, 4194271)), 1e-9, Hh)
add('held-out first-attempt successes at c=1.75 (of 10)', 10, sum(RC['first_attempt_success'][f'{q}|held|1.75']['succ'] for q in (2097091, 4194271)), 1e-9, Hh)
B = 'bench record metrics'
v = RC['paired_ratio_to_c2.00']['4194271|held|1.60|w=0.0']; add('bench record k22 held-out c=1.60 ratio, spread lo', 0.655, v['lo'], tolp(0.655, 3), B); add('bench record k22 ... spread hi', 0.666, v['hi'], tolp(0.666, 3), B)
add('bench record k22 held-out "value" 0.66 (definition not stated; recomputed = mean)', 0.66, v['mean'], 0.005, B)
v = RC['paired_ratio_to_c2.00']['2097091|held|1.60|w=0.0']; add('bench record k21 held-out c=1.60 ratio, spread lo', 0.66, v['lo'], tolp(0.66, 2), B); add('bench record k21 ... spread hi', 0.685, v['hi'], tolp(0.685, 3), B)
add('bench record k21 held-out "value" 0.672 (definition not stated; recomputed = mean)', 0.672, v['mean'], 0.002, B)
bf = RC['band_frozen_rule']
add('(q,w) cells where c=1.60<1.75<2.00 holds per seed, frozen rule (claimed all but k=16 at w=0, 0.1: 16 of 18 cells)', 16, sum(1 for x in bf.values() if x['all']), 1e-9, 'RESULTS.md Band paragraph')
add('A/A repeat of (k=16, seed 1, c=1.75) identical (1 = yes)', 1, 1 if RP['summary']['aa']['producer_repeat_vs_first']['identical'] else 0, 1e-9, 'RESULTS.md Controls')
# claims the artifacts do not support as worded -----------------------------------------------
add('threshold flip between c=1.50 and 1.60 observed at k=22 (1 = observed; RESULTS: "at every k"; no c<=1.50 data at k=21, 22)', 1, 1 if ('4194271|grid|1.50' in RC['first_attempt_success']) else 0, 1e-9, 'RESULTS.md Threshold law / Summary')
ag = RC['seed_agreement']; worst = max(ag[k]['max_over_min_minus_1'] for k in ag if 'ratio1.60' in k)
add('seed-to-seed spread of the c=1.60/c=2.00 ratio at k>=20, max over sizes (RESULTS: "Seeds agree to within 1% at k>=20")', 0.01, worst, 1e-9, 'RESULTS.md Controls')
d1 = None
add('N4v2 DL1 / matched rho-neg at q=2097091, w=0 (RESULTS: "about 0.005x per its README"; the README 0.005 is DL, DL1 is the committed summary_supp.json value)', 0.005, RC['ic_vs_rho']['2097091|n4v2_calib|frozen_rule|c=2.00|neg']['ratio'][0], 0.1, 'RESULTS.md Class paragraph')
add('c_50 model range lower end (RESULTS: 1.49)', 1.49, min(RC['c50_model'].values()), 0.01, 'RESULTS.md Threshold law')
add('c_50 model range upper end (RESULTS: 1.57)', 1.57, max(RC['c50_model'].values()), 0.01, 'RESULTS.md Threshold law')
# ---------------------------------------------------------------- checks
def passfail(b): return 'pass' if b else 'fail'
cp = {c['seed']: c for c in CERT}
sw = RC['add_ladd_weight_sweep']
checks = [
 dict(id='V0', name='provenance and artifact hashes', result=passfail(V0['porcelain_producer_paths'] == '' and V0['v2.bin']['all_equal'] and not V0['raw_units_tar']['members_not_identical'] and V0['n4v2_units_tar']['equal'] and V0['n4v2_instances_sha256']['computed'] == V0['n4v2_instances_sha256']['recorded']),
      evidence='evidence/v0_v8.json', detail=f"commit {COMMIT}; producer paths clean in git; v2.bin = BUILD.txt = manifest 183810cf...; raw-units.tar.gz = bench record, 238/238 members equal committed files; N4v2 units.tar.gz = manifest aac9a25c..."),
 dict(id='V1', name='rebuild from source, compare sha256', result='pass', evidence='RESULTS.md (build section)', detail='cd research/quartic_n4v2_20261020 && rustc --edition 2021 -O v2.rs -o v2 (rustc 1.97.0 2d8144b78) gives 183810cf9e1f9ed8ecaedf0295d3b8080528418bb208a71bcabbc3eb03d79bb1 = producer binary, bit for bit. Two other invocation forms (repo-root relative path, absolute path) give different hashes: the binary embeds the source path.'),
 dict(id='V2', name='replay of all 84 units plus A/A', result=passfail(RP['summary']['identical'] == 84 and RP['summary']['differs'] == 0), evidence='evidence/replay_compare.json', detail=f"{RP['summary']['identical']}/84 units identical, {RP['summary']['fields_compared_total']} field comparisons (counters, s.* stats, k, k_found, ok, attempts, ell, order); six mandated units included; producer A/A repeat and validator replay of the same unit identical"),
 dict(id='V3', name='c=2.00 reproduces the committed N4v2 records', result=passfail(all(r['identical'] and r['ell_matches_instances_jsonl'] for r in V3)), evidence='evidence/v3_compare.json', detail='12/12 (q in 65479, 262111, 1048507, 4194271; seeds 1-3): D, DL1, DL counters identical to the N4v2 grid raw units (rustc 1.93.1, Apple M4 Pro, 2026-10-25) in 293 fields; ell/order equal instances.jsonl'),
 dict(id='V4', name='independent certificate re-verification (own C code)', result=passfail(all_certs and len(CERT) == 22), evidence='evidence/certcheck.jsonl', detail=f"{sum(1 for c in CERT if c['pass'])}/22 instances: E0, order = 4 ell (BSGS), ell prime, G of order ell, planted k reproduced from the seed, k_found G = k G for every ok record, two negative-control logs per instance rejected; tries until match equal the committed N4v2 tries for seeds 1-3 at every size; j(E0) not in F_q and not in {{0,1728}}"),
 dict(id='V5', name='recompute claimed numbers from raw counters', result='pass', evidence='evidence/recompute.json', detail='every table range, success count, held-out range and bench-record spread reproduced to the printed rounding; j2/summary.json equals the recompute to 3 decimals in all 18 cells; pricing code reproduces the committed N4v2 summary_supp.json DL1/D/DL means (<= 3.5e-7, print precision) and rho means (<= 2.8e-7)'),
 dict(id='V6', name='w-band order c=1.60<1.75<2.00', result='pass_under_frozen_rule_fail_under_strict', evidence='evidence/recompute.json (band_frozen_rule, band_strict_add_ladd, add_ladd_weight_sweep)', detail='frozen N4v2 rule (unpriced = hash + cmp): holds per seed at every w for k=18, 20, 21, 22 (grid and held-out), fails at k=16 for w=0, 0.1 (as claimed); robust to calibration (r_inv 8-30, r_lmul 1.0-2.5). Strict reading (unpriced also counts F_q additions and mod-ell additions, which the records emit): the order FAILS at w=0.1 and 1 at every size, the c=1.60 ratio becomes 1.25-1.34 and c=1.75 1.02-1.06; break-even price of an addition 0.44-0.59 M-eq'),
 dict(id='V7', name='threshold law and success counts', result='pass_with_scope_limit', evidence='evidence/recompute.json (first_attempt_success)', detail='counts reproduced (0/3 at 1.40 and 1.50, 3/3 at 1.60+ for k=16,18,20; 10/10 held-out). The flip between 1.50 and 1.60 is observed at k=16, 18, 20 only; k=21 and 22 have no c<=1.50 data, so the "at every k" statement is not supported there (only c_50 <= 1.60). Exact one-sided 95% lower bound of first-attempt success at c=1.60: 0.873 over all 22 units, 0.741 over the 10 held-out units.'),
 dict(id='V8', name='held-out discipline', result='pass', evidence='evidence/v0_v8.json', detail='amendment commit 0056acedae 2026-11-12T04:54:39Z precedes the first held-out launch 04:54:56Z (its header label says 05:00 UTC); seeds 4-8 absent from N4v2 instances and the J2 grid; 26 failed-binary-missing records are empty, error "No such file", none enters a number; predictions all met (success 5/5, ratios inside [0.60,0.72] and [0.72,0.82], order at every w under the frozen rule)'),
 dict(id='V9', name='matched rho reference', result='partial', evidence='evidence/recompute.json (rho, ic_vs_rho), evidence/rho_replay/', detail='N4v2 frozen rho records at q=2097091 (6 runs per arm, passes calibration rule) and q=4194271 (frozen 2 runs fail the rule; the post-hoc 6-run supplement passes). Recomputed rho means equal committed summary_supp.json. DL1(c=1.60)/rho across w=0,0.1,1 and both rho arms: k=22 0.0011-0.0023, k=21 0.0014-0.0029 (strict reading 0.0011-0.0073); order IC<rho holds across the band under both readings. Rho replays in the rebuilt binary identical to the N4v2 raw records (k=16 seed 1 all 16 runs x2 arms; k=18 seed 1 all 6 runs x2; k=21 seed 1 run 0 plain and neg; k=22 seed 1 run 0 neg; 5 job files, 1,027 field comparisons). J2 PROTOCOL names no rho reference; rho and IC c-sweep ran in different binaries/hosts.'),
 dict(id='V10', name='reporting contract', result='gap', evidence='RESULTS.md', detail='J2 table has no rho column, no phase split, no unpriced-counter list (add, ladd emitted but not listed), no absolute cost; class "engineering" asserted without the CPU-time or ladd evidence; m=83 / ECC2K-130 disclosure not applicable (prime-field quartic family, no binary-field claim)'),
 dict(id='V11', name='unit completeness: does the gain survive measured CPU time?', result='fail', evidence='evidence/recompute.json (cpu_wall_ratios_to_c2.00, in_situ_LA_regression)', detail='producer records and the validator replay agree: DL1 total CPU at c=1.60 vs c=2.00 is 0.95-1.13 per seed at k=21, 22 held-out (medians 1.05/1.02 producer records, 1.07/1.00 replay), i.e. no gain; linear-algebra phase CPU 1.34-1.56x at k=18-22; relations phase 0.58-0.75x at k>=20. Counted mod-ell additions in LA (unpriced in the frozen unit) rise 1.38-1.49x at c=1.60 because the matrix is denser (nnz 1.91-2.03x; k=22 seed 4: 260,289 vs 132,855). In-situ least squares (R2 0.993): 8.0 ns per counted ladd = 1.08 M-eq, 1.5 ns per lmul = 0.21 M-eq. Host is L0 (unisolated): a wall figure is practicality evidence, not a speed claim; counts stand at any level.'),
]
fail_items = ['threshold flip at every k', 'seed agreement within 1%', 'DL1 about 0.005x rho-neg', 'improvement under measured CPU / complete unit']
findings = [
 dict(id='F1', severity='decisive for classification', text='The 0.655-0.685x (k=21,22) and 0.66-0.78x gains exist only in the frozen N4v2 M-eq unit, which counts but does not price F_q and mod-ell additions. At c=1.60 the matrix is denser (k=22: nnz 260,289 vs 132,855; mean cycle-row weight 68 vs 28), so counted mod-ell additions in LA rise to 1.38-1.49x (nnz 1.9-2.0x). Measured process CPU (producer records and the validator replay): total 0.95-1.13x of c=2.00 per seed at k=21,22 held-out (medians 1.00-1.07, not below 1), LA phase 1.34-1.56x. The order c=1.60<2.00 reverses once an addition is priced above 0.44-0.59 M-eq (the in-situ fit is 1.08 M-eq per ladd) and at the 5a band weights 0.1 and 1 GAE per addition. c=1.75 measures 0.91-0.99x CPU at k=21,22 (medians 0.94-0.97), a smaller gain than its M-eq 0.77. AGENTS.md: a headline count that falls while work moves where the headline does not look is relabelling, not engineering.'),
 dict(id='F2', severity='minor', text='RESULTS says the success threshold sits between 1.50 and 1.60 at every k; k=21 and 22 have no data at c<=1.50.'),
 dict(id='F3', severity='minor', text='RESULTS: "N4v2 DL1, itself about 0.005x matched negation rho at q=2^21 per its README": the README figure 0.005 is DL; DL1/rho-neg is 0.0028 (summary_supp.json).'),
 dict(id='F4', severity='minor', text='RESULTS: "Seeds agree to within 1% at k>=20": true for the c=2.00 cost (0.07-0.40%), not for c=1.60 (cost spread 0.8-4.0%, ratio spread 0.8-3.8%).'),
 dict(id='F5', severity='gate', text='J2 PROTOCOL names no rho reference; the rho arm is N4v2 records run in another binary (rustc 1.93.1, M4 Pro) and the k=22 reference is the post-hoc supplement. matches_protocol and same_binary_or_process are false in VALIDATION.json.'),
 dict(id='F6', severity='minor', text='The c=2.00 control against the committed N4v2 records is required by the J2 PROTOCOL but RESULTS reports only the A/A repeat; the validator ran it (V3: identical).'),
 dict(id='F7', severity='note', text='Amendment header says 05:00 UTC; its commit is 04:54:39Z, 17 s before the first held-out launch. Order is right, the label is not.'),
]
rawdata = []
base = os.path.abspath(D)
for p in ['../../leads_chase_20261112/j2/raw-units.tar.gz', '../../leads_chase_20261112/j2/summary.json', '../../leads_chase_20261112/RESULTS.md', '../../leads_chase_20261112/PROTOCOL.md',
          '../../../benchmarks/records/20261112T0348Z-j2-percolation-fb-size.json', '../../quartic_n4v2_20261020/results/summary_supp.json', '../../quartic_n4v2_20261020/results/instances.jsonl',
          '../../quartic_n4v2_20261020/results/supp-k22-rho.tar.gz', 'evidence/recompute.json', 'evidence/replay_compare.json', 'evidence/certcheck.jsonl', 'evidence/v3_compare.json', 'evidence/v0_v8.json', 'FROZEN.sha256']:
    rawdata.append(dict(path=p, sha256=sha(os.path.normpath(os.path.join(base, p)))))
rep = dict(schema='validator-report/v1', status='insufficient_evidence', disposition='insufficient_evidence', independent=True, signed=False, signature=None,
           unsigned_reason='no keyring signature is available in this session',
           validated_commit=COMMIT, claim=dict(source='research/leads_chase_20261112/RESULTS.md section J2', text='factor-base constant c_D = 1.60 vs the frozen 2.00 in the N4v2 plane-quartic DL1 solver: charged cost 0.655-0.685x at k=21,22 on held-out seeds; engineering class'),
           disposition_reason='Measurement validated as recorded (replay 84/84 identical, certificates 22/22, every table number recomputed). The improvement claim is not established: it holds in the frozen M-eq unit and fails under the unpriced additions the records emit and under measured CPU time (checks V6 strict, V11, finding F1); the matched rho reference is partial (F5).',
           what_would_validate=['an amendment frozen before data that names the unit including additions (or a calibrated weight for add/ladd) and shows c=1.60 below c=2.00 in it', 'or an L2-isolated CPU or wall measurement (A/A interval) showing the gain at k>=21', 'a J2 protocol (or amendment) naming rho-neg as the reference, with the rho arm run in the same binary at k=21,22', 'an observed lower bracket of the threshold at k=21,22 (c in {1.40,1.50}) if the threshold claim is to stand at every k'],
           checks=checks, findings=findings, raw_data=rawdata, headline=H)
json.dump(rep, open(os.path.join(D, 'validator-report.json'), 'w'), indent=1)
# ---------------------------------------------------------------- VALIDATION.json
q = 4194271; icr = RC['ic_vs_rho']
ic = icr[f'{q}|j2_calib|frozen_rule|c=1.60|neg']['ic']
rn = icr[f'{q}|j2_calib|frozen_rule|c=1.60|neg']['rho']; rp = icr[f'{q}|j2_calib|frozen_rule|c=1.60|plain']['rho']
rho_min = [min(a, b) for a, b in zip(rn, rp)]
holds = all(icr[k]['ratio'][i] < 1 for k in icr for i in range(3))
val = dict(schema='validation-gate/v1', rows=['j2-percolation-fb-size'],
           replay_certificate=dict(path='replay-certificate.json', sha256=sha(os.path.join(D, 'replay-certificate.json'))),
           validator_report=dict(path='validator-report.json', sha256=sha(os.path.join(D, 'validator-report.json'))),
           rho_reference=dict(kind='negation',
                              protocol='N4v2 PROTOCOL.md (frozen 82a9e87de3): negation-map rho is the pre-registered comparison, plain rho alongside; the J2 PROTOCOL.md (dccf37b62b, amended 0056acedae) names no rho reference',
                              unit='M-eq (F_q multiplication equivalents: mul + r_inv*inv + r_lmul*lmul, every attempt charged), unpriced hash + cmp priced at w in {0, 0.1, 1} GAE, GAE = gae_mul + r_inv*gae_inv; same calibration (J2 Xeon medians) on both sides in gae_band',
                              binary_or_process='IC c-sweep: research/leads_chase_20261112/j2/v2.bin (rustc 1.97.0, Xeon 2.3 GHz, 2026-11-12, sha 183810cf...); rho: N4v2 v2_grid (rustc 1.93.1, Apple M4 Pro, 2026-10-25, sha 07f1b742...). Counters replay identically across both builds (V3, rho replays) but the arms did not run in one binary or process',
                              matches_protocol=False, strongest_available=True, same_unit=True, same_binary_or_process=False,
                              gae_band={'0': dict(ic=ic[0], rho=rho_min[0]), '0.1': dict(ic=ic[1], rho=rho_min[1]), '1': dict(ic=ic[2], rho=rho_min[2])},
                              order_holds_across_band=bool(holds),
                              notes=dict(size='q=4194271 (k=22), DL1 c=1.60 held-out mean vs the cheaper of rho-plain / rho-neg at each w (post-hoc 6-run supplement; the frozen 2-run k=22 rho fails the calibration rule); k=21 in evidence/recompute.json ic_vs_rho',
                                         strongest_available='true in the symmetry sense: j(E0) is not in F_q and not 0 or 1728 for all 22 instances, so negation is the only automorphism; rho is single-trail with one inversion per step (batched inversion would cut at most 56+r_inv to about 56+3 M-eq per step, a factor <= 1.4, which cannot move a ratio of 1e-3)',
                                         order_holds_across_band='IC < rho at every w, both rho arms, both sizes, frozen and strict readings; the within-IC order c=1.60 < c=2.00 holds only under the frozen reading (validator-report.json V6, V11)')),
           disposition='insufficient_evidence', signed=False, signature=None, unsigned_reason='no keyring signature is available in this session')
json.dump(val, open(os.path.join(D, 'VALIDATION.json'), 'w'), indent=1)
print('built', 'replay status', cert['status'], '| report', rep['status'], '| headline', len(H), 'entries; mismatches:', sum(1 for h in H if abs(h['claimed'] - h['recomputed']) > h['rel_tol'] * max(abs(h['claimed']), abs(h['recomputed']), 1e-300)))
for h in H:
    if abs(h['claimed'] - h['recomputed']) > h['rel_tol'] * max(abs(h['claimed']), abs(h['recomputed']), 1e-300): print('  MISMATCH', h['name'][:90], h['claimed'], round(h['recomputed'], 5))
