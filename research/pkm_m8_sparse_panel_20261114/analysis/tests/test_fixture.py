#!/usr/bin/env python3
"""Structural fixture: regenerate the 5-row local dry run (timer floor, m = 4 gb and native pairs) with the frozen job.sh
--dry-run on a reduced copy of the frozen bundle, then run the pipeline on it. Needs /usr/bin/msolve, /usr/bin/time and the
built instrument; skipped otherwise. The 5 rows are real local msolve / native runs, nothing is fabricated.
Run: python3 -m pytest analysis/tests -q   or   python3 analysis/tests/test_fixture.py"""
import json, shutil, subprocess, sys, tarfile, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent.parent
INSTR = STUDY.parent.parent / 'target' / 'release' / 'examples' / 'pkm_m8_panel'

def build_mini(tmp):
    src = tmp / 'bundle_src'
    src.mkdir()
    with tarfile.open(STUDY / 'pod' / 'bundle.tgz') as tf:
        tf.extractall(src, filter='data')
    plan = json.loads((src / 'plan.json').read_text())
    keep = [r for r in plan['rows'] if r['order'] in (1, 16, 17, 18, 19)]
    for i, r in enumerate(sorted(keep, key=lambda r: r['order']), 1):
        r['order'] = i
    plan['rows'] = keep
    (src / 'plan.json').write_text(json.dumps(plan, indent=1))
    b = tmp / 'mini.tgz'
    with tarfile.open(b, 'w:gz') as tf:
        for p in sorted(src.iterdir()):
            tf.add(p, arcname=p.name)
    work = tmp / 'work'
    p = subprocess.run(['bash', str(STUDY / 'pod' / 'job.sh'), str(b), str(work), '--dry-run'], capture_output=True, text=True, timeout=600,
                       env={'PATH': '/usr/bin:/bin:/usr/local/bin', 'POD_ID': 'dry-run'})
    assert (work / 'out' / 'done.json').is_file(), p.stdout[-800:] + p.stderr[-800:]
    res = tmp / 'results' / 'mini'
    res.mkdir(parents=True)
    for n in ('runs.jsonl', 'results.tgz', 'done.json'):
        shutil.copy(work / 'out' / n, res / n)
    return res

def test_mini_fixture():
    import pytest   # noqa
    for need in ('/usr/bin/msolve', '/usr/bin/time'):
        if not Path(need).exists() or not INSTR.exists() or not (STUDY / 'pod' / 'bundle.tgz').exists():
            pytest.skip('msolve / time / instrument / bundle not available')
    check_mini()

def check_mini():
    with tempfile.TemporaryDirectory() as t:
        tmp = Path(t)
        res = build_mini(tmp)
        out = tmp / 'out'
        p = subprocess.run([sys.executable, str(HERE.parent / 'build_results.py'), '--results', str(res), '--out', str(out), '--no-pdf'], capture_output=True, text=True)
        assert p.returncode == 0, p.stdout + p.stderr
        a = json.loads((out / 'analysis.json').read_text())
        assert len(a['rows']) == 5
        cmp_ = a['compare_gb']
        assert len(cmp_) == 2 and all(c['equal'] is True for c in cmp_), cmp_
        assert a['controls']['second_instrument']['status'] == 'pass'
        assert a['gamma']['cpu_s']['gamma_lo'] is None and 'no natural t = 2' in a['gamma']['cpu_s']['gamma_lo_reason']
        assert a['verdict']['panel'] == 'INCONCLUSIVE'
        md = (out / 'RESULTS.md').read_text()
        import re
        assert len(re.findall(r'^\| \d+ \| (?:gb_check|native|timer_floor) \|', md, re.M)) == 5
        assert 'gamma_lo undefined: no natural t = 2 row is planned/recorded' in md
        print(p.stdout)

if __name__ == '__main__':
    check_mini()
    print('ok test_mini_fixture')
