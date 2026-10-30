#!/usr/bin/env python3
"""Exercise crash recovery, duplicate-worker exclusion, and graph witnesses."""
import json
import os
from pathlib import Path
import signal
import sqlite3
import subprocess
import sys
import tempfile
import time

from analyze import F, IPSEC_B, direct_info
from isogeny_search import ModularPolynomial, bound_for, evaluate
from resumable_search import BASE, LEVELS, canonical


def command(directory, count=None):
    cmd = [sys.executable, str(BASE/'resumable_search.py'), '--directory', str(directory)]
    return cmd + (['--max-nodes', str(count)] if count is not None else [])


def batch(directory, count):
    result = subprocess.run(command(directory, count), capture_output=True, text=True, timeout=90)
    assert result.returncode == 0, result.stderr


def rows(directory):
    with sqlite3.connect(directory/'search.sqlite3') as db:
        assert db.execute('PRAGMA integrity_check').fetchone()[0] == 'ok'
        return {table: db.execute(f'SELECT * FROM {table} ORDER BY 1,2').fetchall()
                for table in ['nodes', 'edges', 'expansions', 'counters']}


def main():
    start = time.monotonic()
    seed = F.inv(IPSEC_B)
    rep, _ = canonical(seed)
    assert canonical(F.sq(seed))[0] == rep
    info = direct_info(F.inv(rep), 5)
    assert info['m'] == 26 and bound_for(int(info['annihilator_hex'], 16)) == 992
    assert direct_info(F.inv(rep), 31)['m'] == 5
    assert direct_info(F.inv(rep), 1)['m'] == 126
    with tempfile.TemporaryDirectory(prefix='resume-test-', dir=BASE) as tmp:
        tmp = Path(tmp)
        resumed, reference = tmp/'resumed', tmp/'reference'
        batch(resumed, 1)
        before = rows(resumed)
        worker = subprocess.Popen(command(resumed), stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
        try:
            deadline = time.monotonic()+30
            while True:
                assert worker.poll() is None, worker.stderr.read()
                status = json.loads((resumed/'status.json').read_text())
                if status['pid'] == worker.pid and status.get('current_node') == 2 and status.get('current_level') == 31:
                    break
                assert time.monotonic() < deadline, 'No mid-level checkpoint observed'
                time.sleep(0.02)
            competing = subprocess.run(command(resumed, 1), capture_output=True, text=True, timeout=10)
            assert competing.returncode != 0 and 'already holds' in competing.stderr
            os.kill(worker.pid, signal.SIGKILL)
            worker.wait(timeout=10)
        finally:
            if worker.poll() is None:
                worker.kill()
                worker.wait()
        interrupted = rows(resumed)
        # A screen and level checkpoints survived, while the current node did not finish.
        assert dict(interrupted['counters'])['expanded'] == 1
        assert len(interrupted['expansions']) > len(before['expansions'])
        batch(resumed, 2)
        batch(reference, 3)
        assert rows(resumed) == rows(reference), 'Restart changed the logical search result'
        with sqlite3.connect(resumed/'search.sqlite3') as db:
            nodes = {r[0]: r for r in db.execute('SELECT id,j,depth,parent,via_level FROM nodes')}
            polynomials = {l: ModularPolynomial(l, BASE/'sources'/f'phi_j_{l}.txt') for l in LEVELS}
            count = 0
            for source, level, raw, shift, target in db.execute('SELECT * FROM edges'):
                a, raw_j, b = int(nodes[source][1], 16), int(raw, 16), int(nodes[target][1], 16)
                assert F.frob(raw_j, shift) == b
                assert canonical(raw_j)[0] == b
                assert evaluate(polynomials[level].at(a), raw_j) == 0
                assert evaluate(polynomials[level].at(raw_j), a) == 0
                count += 1
            assert count == 3*len(LEVELS)*2
        status = subprocess.run(command(resumed)+['--status'], capture_output=True, text=True, check=True)
        assert json.loads(status.stdout)['worker_active'] is False
    report = {
        'passed': True,
        'checks': ['seed screen and Frobenius invariance', 'single-worker exclusion',
                   'SIGKILL during degree-31 expansion', 'SQLite integrity after crash',
                   'restart equals uninterrupted run', '18 reverse modular equations and Frobenius witnesses',
                   'inactive-worker status detection'],
        'elapsed_seconds': round(time.monotonic()-start, 3),
    }
    (BASE/'resumable_verification.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
