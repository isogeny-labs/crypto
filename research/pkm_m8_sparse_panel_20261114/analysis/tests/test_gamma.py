#!/usr/bin/env python3
"""Unit tests of the gamma arithmetic and the decision rule, and an end-to-end run on a synthetic results directory.
Run: python3 -m pytest analysis/tests -q   or   python3 analysis/tests/test_gamma.py
Synthetic data only; every number is [pcc:unverified]."""
import json, math, os, subprocess, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import build_results as br   # noqa: E402

PASS = {k: dict(status='pass') for k in br.REQUIRED_GATES}

def rec(order, v, complete):
    return dict(order=order, _v=v, complete=complete)

def block(t2, t3, planned2=None, planned3=None, probe=None):
    n2 = {i + 1: rec(i + 1, v, True) for i, v in enumerate(t2)}
    n3 = {100 + i: rec(100 + i, v, c) for i, (v, c) in enumerate(t3)}
    return br.gamma_block(n2, n3, probe, planned2 if planned2 is not None else list(n2), planned3 if planned3 is not None else list(n3), key='_v')

def test_three_t2_complete_two_t3_censored():
    # t=2 complete c = 100, 120, 150 ; t=3 censored lower bounds 2000, 3000
    g = block([100.0, 120.0, 150.0], [(2000.0, False), (3000.0, False)])
    assert abs(g['gamma_lo'] - math.log2(2000 / 150)) < 1e-12          # = log2(40/3) = 3.7369655941662
    assert abs(g['gamma_lo'] - 3.7369655941662063) < 1e-9
    assert math.isinf(g['gamma_hi']) and g['gamma_hi'] > 0               # no completed t=3 row -> +inf
    assert g['n_t3_censored'] == 2 and g['n_t3_complete'] == 0
    assert abs(g['gamma_hi_lb_incl_censored'] - math.log2(3000 / 100)) < 1e-12   # reported lower bound 4.9069
    assert g['gamma_lo_8_16'] is None
    # decision: gates pass -> KILL on this observable
    v, why = br.decide_observable('cpu_s', g, 1.05, PASS, 'not evaluable', 0)
    assert v.startswith('KILL'), (v, why)
    # one censored row below the gate -> straddles / INCONCLUSIVE
    g2 = block([100.0, 120.0, 150.0], [(900.0, False), (3000.0, False)])
    assert abs(g2['gamma_lo'] - math.log2(900 / 150)) < 1e-12 and g2['gamma_lo'] < 3.5
    v2, why2 = br.decide_observable('cpu_s', g2, 1.05, PASS, None, 0)
    assert v2 == 'INCONCLUSIVE' and any('gamma_lo' in w for w in why2)

def test_gamma_hi_and_not_killed():
    g = block([100.0, 120.0, 150.0], [(1000.0, True), (1400.0, True)])
    assert abs(g['gamma_lo'] - math.log2(1000 / 150)) < 1e-12            # 2.7370
    assert abs(g['gamma_hi'] - math.log2(1400 / 100)) < 1e-12            # 3.8074
    # gamma_hi + log2(rho) >= 3.5 here: not NOT KILLED
    v, _ = br.decide_observable('cpu_s', g, 1.0, PASS, 'pass', 1)
    assert v == 'INCONCLUSIVE'
    g = block([100.0, 120.0, 150.0], [(500.0, True), (700.0, True)])
    assert abs(g['gamma_hi'] - math.log2(7.0)) < 1e-12                   # 2.8074
    v, _ = br.decide_observable('cpu_s', g, 1.2, PASS, 'pass', 1)
    assert v.startswith('NOT KILLED')
    assert br.decide_observable('cpu_s', g, 1.2, PASS, 'not evaluable: x', 1)[0] == 'INCONCLUSIVE'   # needs both readings
    assert br.decide_observable('cpu_s', g, 1.2, PASS, 'pass', 0)[0] == 'INCONCLUSIVE'               # needs a verified positive
    assert br.decide_observable('work_proxy', g, None, PASS, None, 1)[0].startswith('NOT KILLED')

def test_undefined_cases():
    # a t=2 row not complete -> gamma_lo undefined, KILL unavailable
    n2 = {1: rec(1, 100.0, True), 2: rec(2, 120.0, False), 3: rec(3, 150.0, True)}
    n3 = {100: rec(100, 5000.0, False)}
    g = br.gamma_block(n2, n3, None, [1, 2, 3], [100], key='_v')
    assert g['gamma_lo'] is None and 'did not complete' in g['gamma_lo_reason']
    assert br.decide_observable('cpu_s', g, 1.0, PASS, None, 0)[0] == 'INCONCLUSIVE'
    # a planned t=2 row with no record counts as not completed
    g = br.gamma_block({1: rec(1, 100.0, True)}, {100: rec(100, 5000.0, False)}, None, [1, 2], [100], key='_v')
    assert g['gamma_lo'] is None and g['t2_missing'] == [2]
    # nothing planned
    g = br.gamma_block({}, {}, None, [], [], key='_v')
    assert g['gamma_lo'] is None and 'no natural t = 2' in g['gamma_lo_reason']
    # planned t=3 row missing: provisional gamma_lo, no KILL
    g = block([100.0, 120.0, 150.0], [(2000.0, False)], planned3=[100, 101])
    assert g['gamma_lo'] is not None and not g['gamma_lo_complete_basis']
    assert br.decide_observable('cpu_s', g, 1.0, PASS, None, 0)[0] == 'INCONCLUSIVE'
    # a failed required gate blocks KILL
    bad = dict(PASS, canary=dict(status='fail'))
    g = block([100.0, 120.0, 150.0], [(2000.0, False), (3000.0, False)])
    v, why = br.decide_observable('cpu_s', g, 1.0, bad, None, 0)
    assert v == 'INCONCLUSIVE' and any('canary' in w for w in why)

def test_gamma_8_16():
    probe = rec(200, 80000.0, False)
    g = block([100.0, 120.0, 150.0], [(1000.0, True), (1400.0, True)], probe=probe)
    assert abs(g['gamma_lo_8_16'] - math.log2(80000 / 1400)) < 1e-12
    g = block([100.0, 120.0, 150.0], [(1000.0, True), (1400.0, False)], probe=probe)
    assert g['gamma_lo_8_16'] is None

def test_panel_verdict():
    assert br.panel_verdict(('KILL (scoped)', []), ('KILL (scoped)', []))[0] == 'KILL (scoped)'
    assert br.panel_verdict(('KILL (scoped)', []), ('INCONCLUSIVE', ['x']))[0] == 'INCONCLUSIVE'

# ---------------------------------------------------------------- end to end on a synthetic study
def make_study(root, planned, cells):
    (root / 'pod').mkdir(parents=True)
    (root / 'FREEZE.json').write_text(json.dumps({'commit': 'deadbeef', 'version': 'test', 'frozen_at': 'x', 'sha256': {}}))
    (root / 'pod' / 'plan.json').write_text(json.dumps({'schema': 'x', 'rows': planned}))
    for c, B in cells.items():
        d = root / 'cells' / c
        d.mkdir(parents=True)
        (d / 'cell.json').write_text(json.dumps({'B': B}))
        tg = [{'name': p['target'], 'label': 'sat' if p['target'].startswith('s') else 'unsat', 'algebraic_count': 1, 'real_count': 1}
              for p in planned if p['cell'] == c]
        (d / 'targets.json').write_text(json.dumps({'targets': tg}))

def mk_run(p, cpu, vcpu, proxy, censored='ok', rc=0, pod='podA', replacement=False):
    r = dict(order=p['order'], kind=p['kind'], cell=p['cell'], target=p['target'], solver='msolve', msolve_mode='param', threads=8,
             user_s=cpu * 0.9, sys_s=cpu * 0.1, wall_s=cpu / 6, max_rss_kb=1000, exit_code=rc, censored=censored, pod_id=pod, replacement=replacement,
             verify=dict(exit_code=0 if rc == 0 else None, wall_s=vcpu if rc == 0 else None),
             log_table=dict(sum_rows_x_cols_x_density=proxy, max_round_degree=8, rounds=10, up_to_degree_8=None), msolve_outcome='zero_dimensional_param')
    return r

def write_results(resdir, study_root, runs, with_done=True):
    resdir.mkdir(parents=True, exist_ok=True)
    (resdir / 'runs.jsonl').write_text('\n'.join(json.dumps(r) for r in runs) + '\n')
    (resdir / 'verify').mkdir(exist_ok=True)
    for r in runs:
        if r['verify']['exit_code'] == 0:
            rid = br.row_id(r)
            (resdir / 'verify' / f'{rid}.json').write_text(json.dumps(dict(cpu_secs=r['verify']['wall_s'] / 2, extraction=dict(real_signed_decompositions=0))))
    if with_done:
        (resdir / 'done.json').write_text(json.dumps(dict(rows_recorded=len(runs), rows_planned=len(runs), by_status={})))

def synthetic(tmp, drop_last=False):
    planned = [dict(order=i + 1, kind='natural', cell='t2r9', target=t, solver='msolve', msolve_mode='param', verify=True) for i, t in enumerate(['u0', 's1', 'u2'])]
    planned += [dict(order=10 + i, kind='natural', cell='t3r8', target=t, solver='msolve', msolve_mode='param', verify=True) for i, t in enumerate(['u0', 's1'])]
    root = Path(tmp) / 'study'
    make_study(root, planned, {'t2r9': 4, 't3r8': 8})
    # c = cpu(msolve) + verify.json cpu_secs (= verify.wall_s / 2 here): 100+0 ... use verify wall 0 -> cpu_secs 0
    runs = [mk_run(planned[0], 100, 0, 1.0e6), mk_run(planned[1], 120, 0, 1.2e6), mk_run(planned[2], 150, 0, 1.5e6),
            mk_run(planned[3], 2000, 0, 2.0e7, censored='timeout', rc=124), mk_run(planned[4], 3000, 0, 3.0e7, censored='timeout', rc=124)]
    if drop_last:
        runs = runs[:-1]
    res = Path(tmp) / 'results' / 'syn'
    write_results(res, root, runs, with_done=not drop_last)
    return root, res

def run_cli(root, res, out, extra=()):
    cmd = [sys.executable, str(HERE.parent / 'build_results.py'), '--results', str(res), '--out', str(out), '--study', str(root),
           '--no-instrument', '--no-pdf'] + list(extra)
    p = subprocess.run(cmd, capture_output=True, text=True)
    assert p.returncode == 0, p.stdout + p.stderr
    return p.stdout

def test_end_to_end_synthetic():
    with tempfile.TemporaryDirectory() as tmp:
        root, res = synthetic(tmp)
        out = Path(tmp) / 'out'
        so = run_cli(root, res, out)
        md = (out / 'RESULTS.md').read_text()
        a = json.loads((out / 'analysis.json').read_text())
        g = a['gamma']['cpu_s']
        assert abs(g['gamma_lo'] - math.log2(2000 / 150)) < 1e-9, g
        assert abs(a['gamma']['work_proxy']['gamma_lo'] - math.log2(2.0e7 / 1.5e6)) < 1e-9
        assert a['verdict']['panel'] == 'INCONCLUSIVE'     # controls are not evaluable without positive/negative/A-A/canary rows
        assert any('not evaluable' in r for r in a['verdict']['panel_reasons'])
        assert 'Table A' in md and 'PANEL VERDICT' in md and '[pcc:unverified]' in md and 'deadbeef' in md
        assert (out / 'REPORT.typ').is_file()
        assert 'PANEL VERDICT: INCONCLUSIVE' in so
        # raw directory untouched
        assert sorted(p.name for p in res.iterdir()) == ['done.json', 'runs.jsonl', 'verify']

def test_partial_results_do_not_crash():
    with tempfile.TemporaryDirectory() as tmp:
        root, res = synthetic(tmp, drop_last=True)
        out = Path(tmp) / 'out'
        run_cli(root, res, out)
        a = json.loads((out / 'analysis.json').read_text())
        md = (out / 'RESULTS.md').read_text()
        assert 'PARTIAL results' in md and 'Lost rows' in md
        assert a['gamma']['cpu_s']['t3_missing'] == [11]
        assert a['verdict']['panel'] == 'INCONCLUSIVE'
        # replacement directory: the lost row is re-run on a second pod and merged, tagged
        rep = Path(tmp) / 'results' / 'syn' / 'replacement'
        p = json.loads((res.parent.parent / 'study' / 'pod' / 'plan.json').read_text())['rows'][4]
        write_results(rep, root, [mk_run(p, 3000, 0, 3.0e7, censored='timeout', rc=124, pod='podB', replacement=True)])
        out2 = Path(tmp) / 'out2'
        run_cli(root, res, out2, ['--results-replacement', str(rep)])
        a2 = json.loads((out2 / 'analysis.json').read_text())
        assert a2['gamma']['cpu_s']['t3_missing'] == []
        assert any(r['host_tag'] == 'replacement' for r in a2['rows'])

if __name__ == '__main__':
    n = 0
    for k, f in sorted(globals().items()):
        if k.startswith('test_') and callable(f):
            f()
            n += 1
            print('ok', k)
    print(f'{n} tests passed')
