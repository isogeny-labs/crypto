#!/usr/bin/env python3
"""Checkpointed IPsec F_(2^155) modular-isogeny graph search.

One vertex represents an orbit under absolute Frobenius. Exhaustiveness of
levels 3,17,31 uses the GRH-conditional class-group completeness result in
full-class/. A GHS screen is a specific test, not a general security proof.
"""
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import sqlite3
import time

from analyze import F, IPSEC_B, IPSEC_ORDER, direct_info
from isogeny_search import ModularPolynomial, bound_for, evaluate, rational_roots

BASE = Path(__file__).resolve().parent
LEVELS = (3, 17, 31)
CLASS_SIZE = 74173379224089213153040
STOP = False


def hx(n):
    return f'{n:039x}'


def canonical(j):
    """Return min absolute-Frobenius conjugate and its exponent (0..154)."""
    best, shift, v = j, 0, j
    for k in range(1, 155):
        v = F.sq(v)
        if v < best:
            best, shift = v, k
        if v == j:
            raise ValueError('Unexpected proper-subfield j-invariant')
    assert F.sq(v) == j
    return best, shift


def connect(path):
    db = sqlite3.connect(path, timeout=30)
    db.row_factory = sqlite3.Row
    db.execute('PRAGMA journal_mode=WAL')
    db.execute('PRAGMA synchronous=FULL')
    return db


def inc(db, key, n=1):
    db.execute('INSERT INTO counters VALUES (?,?) ON CONFLICT(key) '
               'DO UPDATE SET value=value+excluded.value', (key, n))


def initialize(db, polynomials):
    db.executescript('''
      CREATE TABLE IF NOT EXISTS metadata(key TEXT PRIMARY KEY,value TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS counters(key TEXT PRIMARY KEY,value INTEGER NOT NULL);
      CREATE TABLE IF NOT EXISTS nodes(
        id INTEGER PRIMARY KEY, j TEXT UNIQUE NOT NULL, depth INTEGER NOT NULL,
        parent INTEGER REFERENCES nodes(id), via_level INTEGER,
        screened INTEGER NOT NULL DEFAULT 0, expanded INTEGER NOT NULL DEFAULT 0,
        b TEXT, m INTEGER, direct_genus TEXT, annihilator TEXT, hess_bound INTEGER,
        direct_screens TEXT);
      CREATE INDEX IF NOT EXISTS frontier ON nodes(expanded,id);
      CREATE TABLE IF NOT EXISTS expansions(
        node_id INTEGER NOT NULL REFERENCES nodes(id), level INTEGER NOT NULL,
        root_count INTEGER NOT NULL, PRIMARY KEY(node_id,level));
      CREATE TABLE IF NOT EXISTS edges(
        source INTEGER NOT NULL REFERENCES nodes(id), level INTEGER NOT NULL,
        raw_j TEXT NOT NULL, frobenius_shift INTEGER NOT NULL,
        target INTEGER NOT NULL REFERENCES nodes(id),
        PRIMARY KEY(source,level,raw_j));
    ''')
    configuration = {
        'schema': 1, 'field_modulus': hex(F.modulus),
        'seed_b': hex(IPSEC_B), 'curve_order': str(IPSEC_ORDER),
        'levels': list(LEVELS), 'quotient': 'absolute Frobenius, orbit size 155',
        'coverage': 'full isogeny class conditional on GRH class-group completeness',
        'conditional_class_size': str(CLASS_SIZE),
        'conditional_orbit_count': str(CLASS_SIZE//155),
        'polynomial_sha256': {str(p.level): p.sha256 for p in polynomials},
        'source_sha256': {name: hashlib.sha256((BASE/name).read_bytes()).hexdigest()
                          for name in ['analyze.py', 'isogeny_search.py', 'resumable_search.py']},
        'screens': ['direct GHS to F2, F32, F_(2^31)',
                    'Hess two-factor necessary genus bound to F32'],
        'interpretation': 'small genus or bound is a candidate, not a proven DLP attack',
    }
    encoded = json.dumps(configuration, sort_keys=True)
    old = db.execute("SELECT value FROM metadata WHERE key='configuration'").fetchone()
    if old and old[0] != encoded:
        raise RuntimeError('Database configuration does not match code/data')
    with db:
        db.execute('INSERT OR IGNORE INTO metadata VALUES (?,?)', ('configuration', encoded))
        seed, shift = canonical(F.inv(IPSEC_B))
        cur = db.execute('INSERT OR IGNORE INTO nodes(j,depth) VALUES (?,0)', (hx(seed),))
        if cur.rowcount:
            inc(db, 'discovered')
            db.execute('INSERT INTO metadata VALUES (?,?)', ('seed_frobenius_shift', str(shift)))
    return configuration


def snapshot(db, directory, state, **extra):
    counts = dict(db.execute('SELECT key,value FROM counters'))
    result = {
        'utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
        'pid': os.getpid(), 'state': state,
        'discovered_frobenius_orbits': counts.get('discovered', 0),
        'screened_frobenius_orbits': counts.get('screened', 0),
        'expanded_frobenius_orbits': counts.get('expanded', 0),
        'pending_expansion_orbits': counts.get('discovered', 0)-counts.get('expanded', 0),
        'completed_level_expansions': counts.get('level_expansions', 0),
        'direct_m_distribution': {k[2:]: v for k, v in counts.items() if k.startswith('m:')},
        'hess_bound_distribution': {k[6:]: v for k, v in counts.items() if k.startswith('bound:')},
        'conditional_total_orbits': str(CLASS_SIZE//155),
        'complete': False,
        **extra,
    }
    temp = directory/'status.json.tmp'
    with temp.open('w') as f:
        json.dump(result, f, indent=2)
        f.write('\n')
        f.flush()
        os.fsync(f.fileno())
    temp.replace(directory/'status.json')
    return result


def stop_requested(signum, frame):
    global STOP
    STOP = True


def run(args):
    directory = Path(args.directory).resolve()
    directory.mkdir(parents=True, exist_ok=True)
    with (directory/'worker.lock').open('a') as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            raise SystemExit('A worker already holds this database lock')
        polynomials = [ModularPolynomial(l, BASE/'sources'/f'phi_j_{l}.txt') for l in LEVELS]
        db = connect(directory/'search.sqlite3')
        config = initialize(db, polynomials)
        (directory/'configuration.json').write_text(json.dumps(config, indent=2)+'\n')
        (directory/'worker.pid').write_text(str(os.getpid())+'\n')
        signal.signal(signal.SIGTERM, stop_requested)
        signal.signal(signal.SIGINT, stop_requested)
        start = time.monotonic()
        completed = 0
        snapshot(db, directory, 'running')
        try:
            while not STOP and (args.max_nodes is None or completed < args.max_nodes):
                node = db.execute('SELECT * FROM nodes WHERE expanded=0 ORDER BY id LIMIT 1').fetchone()
                if node is None:
                    # Never silently promote component exhaustion to a full-class proof.
                    snapshot(db, directory, 'component_exhausted_requires_coverage_review')
                    return
                j, node_id = int(node['j'], 16), node['id']
                if not node['screened']:
                    b = F.inv(j)
                    screens = {str(step): direct_info(b, step) for step in [1, 5, 31]}
                    direct = screens['5']
                    bound = bound_for(int(direct['annihilator_hex'], 16))
                    with db:
                        db.execute('UPDATE nodes SET screened=1,b=?,m=?,direct_genus=?,annihilator=?,hess_bound=?,direct_screens=? WHERE id=?',
                                   (hx(b), direct['m'], str(direct['exact_genus']), direct['annihilator_hex'], bound,
                                    json.dumps(screens, sort_keys=True), node_id))
                        inc(db, 'screened')
                        inc(db, f'm:{direct["m"]}')
                        inc(db, f'bound:{bound}')
                    if direct['m'] <= 6 or bound <= 31 or screens['31']['m'] < 5 or screens['1']['m'] <= 6:
                        print(json.dumps({'candidate': node_id, 'j': node['j'], 'm': direct['m'], 'bound': bound}), flush=True)
                        with (directory/'candidates.jsonl').open('a') as f:
                            f.write(json.dumps({'node_id': node_id, 'j': node['j'], 'b': hx(b), 'direct_screens': screens, 'hess_bound': bound})+'\n')
                done = {r[0] for r in db.execute('SELECT level FROM expansions WHERE node_id=?', (node_id,))}
                for polynomial in polynomials:
                    if STOP:
                        break
                    if polynomial.level in done:
                        continue
                    snapshot(db, directory, 'running', current_node=node_id, current_level=polynomial.level)
                    roots = rational_roots(polynomial.at(j))
                    # Each selected prime splits with conductor coprime to the degree.
                    # Distinct rational roots: exactly two horizontal neighbors here.
                    if len(roots) != 2 or any(root == 0 for root in roots):
                        raise RuntimeError(f'Unexpected rational neighbor count at {node_id}, level {polynomial.level}: {len(roots)}')
                    neighbors = []
                    for raw in roots:
                        assert evaluate(polynomial.at(raw), j) == 0
                        child, shift = canonical(raw)
                        neighbors.append((raw, child, shift))
                    with db:
                        for raw, child, shift in neighbors:
                            cur = db.execute('INSERT OR IGNORE INTO nodes(j,depth,parent,via_level) VALUES (?,?,?,?)',
                                             (hx(child), node['depth']+1, node_id, polynomial.level))
                            if cur.rowcount:
                                inc(db, 'discovered')
                            child_id = db.execute('SELECT id FROM nodes WHERE j=?', (hx(child),)).fetchone()[0]
                            db.execute('INSERT INTO edges VALUES (?,?,?,?,?)',
                                       (node_id, polynomial.level, hx(raw), shift, child_id))
                        db.execute('INSERT INTO expansions VALUES (?,?,?)', (node_id, polynomial.level, len(roots)))
                        inc(db, 'level_expansions')
                if STOP:
                    break
                with db:
                    db.execute('UPDATE nodes SET expanded=1 WHERE id=?', (node_id,))
                    inc(db, 'expanded')
                completed += 1
                status = snapshot(db, directory, 'running', elapsed_this_run_seconds=round(time.monotonic()-start, 3))
                print(json.dumps(status, separators=(',', ':')), flush=True)
            snapshot(db, directory, 'stopped' if STOP else 'batch_complete', elapsed_this_run_seconds=round(time.monotonic()-start, 3))
        except BaseException as exc:
            snapshot(db, directory, 'error', error=repr(exc))
            raise
        finally:
            db.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--directory', default=str(BASE/'live-search'))
    parser.add_argument('--max-nodes', type=int, help='completed node expansions this run; omit for unbounded')
    parser.add_argument('--status', action='store_true')
    args = parser.parse_args()
    if args.status:
        directory = Path(args.directory)
        result = json.loads((directory/'status.json').read_text())
        with (directory/'worker.lock').open('a') as lock:
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
                result['worker_active'] = False
            except BlockingIOError:
                result['worker_active'] = True
        print(json.dumps(result, indent=2))
    else:
        run(args)


if __name__ == '__main__':
    main()
