"""Persistent index calculus for fixed K_0 parameters in a type-II normal basis.

Python integers retain the full field width. SQLite transactions preserve pair
construction, probe cursors and verified relations across process restarts.
"""
import argparse
from collections import Counter
import hashlib
import json
import math
import multiprocessing
from pathlib import Path
import shutil
import sqlite3
import sys
import time

import curves
import decomp
import indexcalc_e2e as engine
import indexcalc_selector as selector


FORMAT = 'fixed-koblitz-ic-v1'
# Pair-table transactions are small enough to bound crash loss and large
# enough to amortise SQLite's commit cost. The cursor and rolling checksum
# are committed with the rows, so a crash loses at most one chunk.
PAIR_CHUNK = 4096
# Above this many planned candidates, maintaining pair_lookup during inserts
# costs more than dropping it and rebuilding once at the end.
PAIR_INDEX_THRESHOLD = 100000
# Below this budget, worker start-up costs more than the saved field time.
PAIR_PARALLEL_MIN = 16384

_PAIR_WORKER = None


def _initPairWorker(m, points):
    """Reconstruct the field/curve once per spawned worker."""
    global _PAIR_WORKER
    ledger = engine.Ledger()
    onb = engine.AuditField(m, ledger)
    curve = engine.AuditCurve(onb, ledger)
    _PAIR_WORKER = (curve, points)


def _pairKeys(indices):
    """Return point keys (and operation counts) for `(i, j)` indices."""
    global _PAIR_WORKER
    curve, points = _PAIR_WORKER
    pairs = [(points[i], points[j]) for i, j in indices]
    keys = [None if point is None else pointKey(point) for point in curve.addBatch(pairs)]
    return keys, dict(curve.ledger.counts)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'))


def digest(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def integer(value):
    require(type(value) in (int, str), 'integers must be decimal or hexadecimal strings or JSON integers')
    try:
        result = int(value, 16 if value.lower().startswith('0x') else 10) if isinstance(value, str) else value
    except ValueError as error:
        raise ValueError('invalid integer') from error
    require(result >= 0, 'negative coordinates or scalars are invalid')
    return result


def keys(value, required, optional=()):
    require(isinstance(value, dict), 'expected an object')
    require(set(required) <= value.keys() and value.keys() <= set(required) | set(optional),
            'missing or unknown fields: expected ' + ', '.join(required))


def readJson(path):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, 'duplicate JSON key: ' + key)
            result[key] = value
        return result
    with open(path) as stream:
        return json.load(stream, object_pairs_hook=unique)


class Parameters:
    def __init__(self, document, ledger):
        keys(document, ['schema_version', 'curve', 'generator', 'factor_base', 'summands', 'targets'], ['name'])
        require(type(document['schema_version']) is int and document['schema_version'] == 1,
                'unsupported parameter schema')
        spec = document['curve']
        keys(spec, ['family', 'degree', 'basis', 'subgroup_order', 'cofactor'], ['polynomial', 'onb_root'])
        require(spec['family'] == 'koblitz-a0', 'supported family: y^2 + xy = x^3 + 1')
        m = integer(spec['degree'])
        require(3 <= m <= 131 and m % 2 == 1, 'degree must be odd, between 3 and 131')
        self.onb = engine.AuditField(m, ledger)
        self.curve = engine.AuditCurve(self.onb, ledger)
        self.prime = integer(spec['subgroup_order'])
        require(integer(spec['cofactor']) == 4 and curves.curveOrder(m) == 4 * self.prime
                and curves.isPrimeBig(self.prime), 'expected curve order = 4 * prime subgroup order')
        self.images = None
        require(spec['basis'] in ('type-ii-onb', 'polynomial'), 'unknown coordinate basis')
        if spec['basis'] == 'polynomial':
            require('polynomial' in spec and 'onb_root' in spec,
                    'polynomial coordinates require polynomial and onb_root')
            polynomial = integer(spec['polynomial'])
            require(polynomial.bit_length() == m + 1 and polynomial & 1,
                    'invalid degree-m binary polynomial')
            root = self.coordinate(spec['onb_root'])
            power, images = self.onb.one(), []
            value = 0
            for i in range(m + 1):
                if polynomial >> i & 1:
                    value = self.onb.add(value, power)
                if i < m:
                    images.append(self.onb.toCoords(power))
                power = self.onb.mul(power, root)
            require(value == 0, 'onb_root is not a root of the declared polynomial')
            pivots = {}
            for image in images:
                while image:
                    pivot = image.bit_length() - 1
                    if pivot not in pivots:
                        pivots[pivot] = image
                        break
                    image ^= pivots[pivot]
            require(len(pivots) == m, 'basis conversion is singular')
            self.images = images
        else:
            require('polynomial' not in spec and 'onb_root' not in spec,
                    'polynomial fields do not apply to ONB coordinates')
        self.generator = self.point(document['generator'])
        self.eigen = curves.frobeniusEigenvalue(self.curve, self.generator, self.prime)
        require(pow(self.eigen, m, self.prime) == 1, 'invalid Frobenius eigenvalue')
        base = document['factor_base']
        require(isinstance(base, dict), 'factor_base must be an object')
        if base.get('kind') == 'hamming-weight':
            keys(base, ['kind', 'weight'])
            self.weight = integer(base['weight'])
            require(1 <= self.weight <= (2 if m > 31 else 4),
                    'materialized Hamming bases allow weight <=2 above degree 31, <=4 otherwise')
            self.explicit = None
        else:
            keys(base, ['kind', 'representatives'])
            require(base['kind'] == 'explicit-orbits', 'unknown factor-base kind')
            require(isinstance(base['representatives'], list) and 0 < len(base['representatives']) <= 4096,
                    'supply between 1 and 4096 explicit orbit representatives')
            self.explicit = [self.point(p) for p in base['representatives']]
            self.weight = None
        self.summands = integer(document['summands'])
        require(self.summands in (2, 3), 'fixed workflow supports two or three summands')
        require(isinstance(document['targets'], list), 'targets must be an array')
        self.targets = {}
        for target in document['targets']:
            keys(target, ['id', 'x', 'y'])
            name = target['id']
            require(isinstance(name, str) and 0 < len(name) <= 128 and name not in self.targets,
                    'target IDs must be unique nonempty strings of at most 128 characters')
            self.targets[name] = self.point({'x': target['x'], 'y': target['y']})
        # Target-independent precomputation survives adding or removing targets.
        identity = {k: document[k] for k in ('schema_version', 'curve', 'generator', 'factor_base', 'summands')}
        self.identity = digest([FORMAT, identity])
        self.document = document

    def coordinate(self, value):
        value = integer(value)
        require(value < 1 << self.onb.m, 'coordinate exceeds the declared field width')
        return self.onb.fromCoords(value)

    def point(self, value):
        keys(value, ['x', 'y'])
        coordinates = []
        for axis in ('x', 'y'):
            number = integer(value[axis])
            require(number < 1 << self.onb.m, 'coordinate exceeds the declared field width')
            if self.images is not None:
                mapped = 0
                for i, image in enumerate(self.images):
                    if number >> i & 1:
                        mapped ^= image
                number = mapped
            coordinates.append(self.onb.fromCoords(number))
        point = tuple(coordinates)
        require(self.curve.onCurve(point), 'point is not on the declared curve')
        require(self.curve.mul(point, self.prime) is None, 'point is outside the declared prime subgroup')
        return point


def pointKey(point):
    return 'identity' if point is None else format(point[0], 'x') + ':' + format(point[1], 'x')


def consideredCount(size, i, j):
    """Candidates before cursor `(i, j)`; mirrors the Lean PairCursor model."""
    return i * size - i * (i - 1) // 2 + (j - i)


def requiredBytes(planned, bytesPerPair):
    """Disk estimate used before a pair-table chunk; 25% slack + 32 MiB."""
    return (planned * bytesPerPair * 5) // 4 + 32 * 1024 * 1024


# Index entries are roughly this large on the degree-131 text keys. When the
# pair index is absent during a bulk build, reserve room to create it once.
PAIR_INDEX_BYTES_PER_PAIR = 192


def pairTableRequiredBytes(planned, bytesPerPair, totalPairs, indexPresent):
    """Disk required for this chunk, including the final index when absent."""
    required = requiredBytes(planned, bytesPerPair)
    if not indexPresent:
        required += totalPairs * PAIR_INDEX_BYTES_PER_PAIR
    return required


class Progress:
    """Live human-readable progress on stderr.

    stdout is reserved for the final JSON report: callers such as `ic fixed`
    capture it while letting stderr through to the terminal, so these lines are
    visible immediately but never contaminate the artifact.  The elapsed clock
    starts when the object is created, which is the start of the invocation.
    """

    def __init__(self, enabled=False, every=100, stream=None):
        require(type(enabled) is bool, 'progress must be a boolean')
        require(type(every) is int and every >= 0, 'progress interval must be nonnegative')
        self.enabled = enabled
        self.every = every
        self.stream = sys.stderr if stream is None else stream
        self.started = time.monotonic()

    def line(self, message):
        if not self.enabled:
            return
        self.stream.write('[%9.3fs] ic: %s\n' % (time.monotonic() - self.started, message))
        self.stream.flush()

    def pairStart(self):
        if self.enabled and not hasattr(self, 'pairStarted'):
            self.pairStarted = time.monotonic()

    def pairTable(self, considered, total, pairs, complete, built, budget):
        if not self.enabled:
            return
        if not total:
            return
        if not hasattr(self, 'pairStarted'):
            self.pairStarted = time.monotonic()
        if complete:
            state = 'complete'
        elif built >= budget:
            state = 'paused at this invocation budget; rerun to continue'
        else:
            state = 'building'
        rate = '' if not built else ', %.0f candidates/s' % (
            built / max(1e-9, time.monotonic() - self.pairStarted))
        self.line('[1/4] pair table: %s; %d/%d candidates (%.1f%%), %d pairs%s'
                  % (state, considered, total, 100.0 * considered / total, pairs, rate))

    def attempt(self, stream, number, status, rank, columns, relations, independent, rate):
        if not self.enabled:
            return
        if self.every and number % self.every:
            return
        phase = '[4/4]' if stream.startswith('target:') else '[2/4]'
        self.line('%s %s attempt %d: %s; %d relations (%d independent), rank %d/%d, %.1f attempts/s'
                  % (phase, stream, number + 1, status, relations, independent, rank, columns, rate))

    def relation(self, record, support, rank, columns):
        if not self.enabled:
            return
        phase = '[4/4]' if record['stream'].startswith('target:') else '[2/4]'
        self.line('%s relation stream=%s number=%d independent=%s support=%d/%d rank=%d/%d'
                  % (phase, record['stream'], record['number'], str(record['independent']).lower(),
                     support, columns, rank, columns))

    def message(self, message):
        if self.enabled:
            self.line(message)


class Campaign:
    def __init__(self, document, directory, solver='pairs', memory=False, selectorModel=None,
                 progress=None, relationsOut=None, pairWorkers=1):
        require(__debug__, 'run without Python -O; the shared arithmetic engine uses assertions')
        require(type(pairWorkers) is int and 1 <= pairWorkers <= 64,
                'pair workers must be between 1 and 64')
        self.ledger = engine.Ledger()
        self.uses = Counter()
        self.started = time.perf_counter_ns()
        self.pairWorkers = pairWorkers
        self.db = None
        self.lock = None
        self.relationsOut = None
        self.progress = Progress(False) if progress is None else progress
        self.relationsOutPath = None if relationsOut is None else Path(relationsOut)
        try:
            with self.ledger.phase('parameter_validation'):
                self.params = Parameters(document, self.ledger)
            require(solver in ('pairs', 'sat', 'learned'), 'unknown decomposition solver')
            require((solver == 'learned') == (selectorModel is not None),
                    '--selector-model is required exactly when --solver learned is used')
            require(solver != 'sat' or self.params.weight is not None,
                    'SAT requires the Hamming-weight factor-base recipe')
            self.solver = solver
            path = Path(directory)
            self.directory = path
            self.memory = memory
            if not memory:
                import fcntl
                path.mkdir(parents=True, exist_ok=True)
                self.lock = open(path / 'campaign.lock', 'a')
                try:
                    fcntl.flock(self.lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
                except BlockingIOError as error:
                    raise ValueError('campaign is in use; use one writer per directory') from error
            self.db = sqlite3.connect(':memory:' if memory else path / 'campaign.sqlite3')
            self.db.execute('PRAGMA synchronous=FULL')
            self.db.executescript('''
                CREATE TABLE IF NOT EXISTS metadata (name TEXT PRIMARY KEY, value TEXT NOT NULL);
                CREATE TABLE IF NOT EXISTS pairs (serial INTEGER PRIMARY KEY, point TEXT NOT NULL,
                    first INTEGER NOT NULL, second INTEGER NOT NULL);
                CREATE INDEX IF NOT EXISTS pair_lookup ON pairs(point);
                CREATE TABLE IF NOT EXISTS attempts (stream TEXT NOT NULL, number INTEGER NOT NULL,
                    result TEXT NOT NULL, PRIMARY KEY(stream,number));
                CREATE TABLE IF NOT EXISTS relations (stream TEXT NOT NULL, number INTEGER NOT NULL,
                    witness TEXT NOT NULL, coefficients TEXT NOT NULL, a TEXT NOT NULL, b TEXT NOT NULL,
                    independent INTEGER,
                    PRIMARY KEY(stream,number));
                CREATE TABLE IF NOT EXISTS targets (id TEXT PRIMARY KEY, point TEXT NOT NULL);
                CREATE TABLE IF NOT EXISTS solutions (id TEXT PRIMARY KEY, scalar TEXT NOT NULL);
                CREATE TABLE IF NOT EXISTS invocations (number INTEGER PRIMARY KEY, report TEXT NOT NULL);
            ''')
            # Older campaigns predate the independence flag; keep them readable
            # and let future exports report it instead of silently dropping it.
            if 'independent' not in {row[1] for row in self.db.execute('PRAGMA table_info(relations)')}:
                with self.db:
                    self.db.execute('ALTER TABLE relations ADD COLUMN independent INTEGER')
            with self.db:
                saved = self.meta('identity')
                require(saved is None or saved == self.params.identity,
                        'campaign parameters differ; use a new directory for a different curve or factor base')
                self.put('identity', self.params.identity)
                self.put('parameters', {k: v for k, v in document.items() if k != 'targets'})
                for name, point in self.params.targets.items():
                    prior = self.db.execute('SELECT point FROM targets WHERE id=?', (name,)).fetchone()
                    require(prior is None or prior[0] == pointKey(point), 'target ID was already bound to another point')
                    self.db.execute('INSERT OR IGNORE INTO targets VALUES (?,?)', (name, pointKey(point)))
            self.base()
            self.openRelations()
            self.context = None
            self.selector = None
            self.selectorDomains = {}
            self.queryRank, self.queryDirect = 0, False
            if solver == 'learned':
                with self.ledger.phase('selector_load'):
                    self.selector = selector.Model(readJson(selectorModel))
        except BaseException:
            self.close()
            raise

    def close(self):
        if self.relationsOut is not None:
            self.relationsOut.close()
            self.relationsOut = None
        if self.db is not None:
            self.db.close()
            self.db = None
        if self.lock is not None:
            self.lock.close()
            self.lock = None

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.close()

    def meta(self, name):
        row = self.db.execute('SELECT value FROM metadata WHERE name=?', (name,)).fetchone()
        return None if row is None else json.loads(row[0])

    def put(self, name, value):
        self.db.execute('INSERT OR REPLACE INTO metadata VALUES (?,?)', (name, canonical(value)))

    def openRelations(self):
        """Materialise saved relations as JSON Lines, then append live.

        The rewrite makes the file a complete view even when a previous run
        saved relations before this option existed, and also repairs a missing
        tail after a crash between the SQLite commit and the file append.  New
        rows are flushed immediately so `tail -f` shows each verified relation
        as it lands.
        """
        if self.relationsOutPath is None:
            return
        path = self.relationsOutPath
        path.parent.mkdir(parents=True, exist_ok=True)
        database = self.db.execute('PRAGMA database_list').fetchone()[2]
        if database and Path(database).resolve() == path.resolve():
            raise ValueError('relations export must not replace the campaign database')
        self.relationsOut = open(path, 'w', encoding='utf-8')
        for stream, number, witness, coefficients, a, b, independent in self.db.execute(
                'SELECT stream,number,witness,coefficients,a,b,independent FROM relations ORDER BY stream,number'):
            record = self.relationRecord(stream, number, json.loads(witness), json.loads(coefficients),
                                         a, b, None if independent is None else bool(independent))
            self.relationsOut.write(canonical(record) + '\n')
        self.relationsOut.flush()
        self.progress.message('relations: %d saved to %s' % (
            self.db.execute('SELECT COUNT(*) FROM relations').fetchone()[0], path))

    def relationRecord(self, stream, number, witness, coefficients, a, b, independent):
        return {'schema_version': 1, 'operation': 'fixed-relation', 'stream': stream,
                'target': None if stream == 'precompute' else stream[len('target:'):],
                'number': number, 'witness': witness, 'coefficients': coefficients,
                'a': a, 'b': b, 'independent': independent, 'verified': True}

    def saveRelation(self, record):
        if self.relationsOut is not None:
            self.relationsOut.write(canonical(record) + '\n')
            self.relationsOut.flush()

    def base(self):
        p = self.params
        with self.ledger.phase('factor_base'):
            saved = self.meta('base')
            if saved is None:
                if p.explicit is None:
                    reps, _ = engine.subgroupBase(p.onb, p.curve, p.prime, p.eigen, p.weight)
                else:
                    reps = p.explicit
                payload = [[hex(p.onb.toCoords(x)), hex(p.onb.toCoords(y))] for x, y in reps]
                with self.db:
                    self.put('base', {'points': payload, 'hash': digest(payload)})
                self.uses['base_builds'] += 1
            else:
                require(digest(saved['points']) == saved['hash'], 'factor-base checksum mismatch')
                reps = [tuple(p.coordinate(v) for v in point) for point in saved['points']]
                self.uses['base_loads'] += 1
            self.reps, self.lookup = reps, {}
            require(bool(reps), 'empty factor base')
            for column, rep in enumerate(reps):
                require(p.curve.onCurve(rep) and p.curve.mul(rep, p.prime) is None,
                        'invalid factor-base subgroup point')
                if p.weight is not None:
                    require(0 < bin(p.onb.toCoords(rep[0])).count('1') <= p.weight,
                            'factor-base point violates its Hamming-weight recipe')
                point, coefficient = rep, 1
                for _ in range(p.onb.m):
                    for signed, coeff in ((point, coefficient), (p.curve.neg(point), -coefficient % p.prime)):
                        old = self.lookup.get(signed)
                        require(old is None or old == (column, coeff), 'overlapping or inconsistent Frobenius orbits')
                        self.lookup[signed] = (column, coeff)
                    point = p.curve.frob(point)
                    coefficient = coefficient * p.eigen % p.prime
            self.points = sorted(self.lookup)
            self.index = {point: i for i, point in enumerate(self.points)}
            self.baseHash = digest([pointKey(point) for point in self.points])
            require(self.meta('base_order') in (None, self.baseHash), 'factor-base ordering changed')
            with self.db:
                self.put('base_order', self.baseHash)

    def pairIndexExists(self):
        return self.db.execute(
            "SELECT 1 FROM sqlite_master WHERE type='index' AND name='pair_lookup'"
        ).fetchone() is not None

    def ensurePairIndex(self):
        if self.memory or self.pairIndexExists():
            return
        with self.db:
            self.db.execute('CREATE INDEX IF NOT EXISTS pair_lookup ON pairs(point)')

    def dropPairIndex(self):
        if not self.memory and self.pairIndexExists():
            with self.db:
                self.db.execute('DROP INDEX IF EXISTS pair_lookup')

    def buildPairs(self, budget):
        """Build at most budget candidate pairs; each chunk and cursor commit together."""
        require(type(budget) is int and budget >= 0, 'pair budget must be nonnegative')
        size = len(self.points)
        with self.ledger.phase('pair_table'):
            state = self.meta('pair_state') or {'i': 0, 'j': 0, 'serial': 0, 'hash': '0' * 64}
            self.checkPairs(state)
            total = size * (size + 1) // 2
            self.progress.pairStart()
            if state['i'] == size:
                self.uses['pair_table_loads'] += 1
                self.ensurePairIndex()
                self.progress.pairTable(total, total, state['serial'], True, 0, budget)
                return True
            considered = consideredCount(size, state['i'], state['j'])
            remaining = max(0, total - considered)
            planned = min(budget, remaining)
            if not self.memory:
                pages = self.db.execute('PRAGMA page_count').fetchone()[0]
                page_size = self.db.execute('PRAGMA page_size').fetchone()[0]
                if state['serial'] > 0:
                    bytes_per_pair = max(128, (pages * page_size + state['serial'] - 1) // state['serial'])
                else:
                    bytes_per_pair = 256
                required = pairTableRequiredBytes(planned, bytes_per_pair, total,
                                                  self.pairIndexExists())
                free = shutil.disk_usage(self.directory).free
                if free < required:
                    raise ValueError(
                        'pair table needs about %d MiB free in %s, but only %d MiB is available; '
                        'choose --dir on a volume with more space or stop earlier with --pair-budget'
                        % (required // (1024 * 1024), self.directory.resolve(),
                           free // (1024 * 1024)))
            if planned >= PAIR_INDEX_THRESHOLD:
                self.dropPairIndex()
            done = 0
            reportEvery = max(PAIR_CHUNK, budget // 100)
            nextReport = reportEvery
            # The pair table is rebuildable, so NORMAL synchronous commits are
            # enough: a crash can lose one chunk, and checkPairs validates the
            # committed cursor before resuming. Keep FULL for every other table.
            previous_sync = self.db.execute('PRAGMA synchronous').fetchone()[0]
            self.db.execute('PRAGMA synchronous=NORMAL')
            self.db.execute('PRAGMA cache_size=-65536')
            self.db.execute('PRAGMA temp_store=MEMORY')
            pool = None
            if self.pairWorkers > 1 and budget >= PAIR_PARALLEL_MIN:
                try:
                    context = multiprocessing.get_context('spawn')
                    pool = context.Pool(self.pairWorkers, initializer=_initPairWorker,
                                        initargs=(self.params.onb.m, self.points))
                except (OSError, ValueError) as error:
                    self.progress.message('pair workers unavailable (%s); using one process' % error)
                    pool = None
            try:
                while done < budget and state['i'] < size:
                    with self.db:
                        entries = []
                        for _ in range(min(PAIR_CHUNK, budget - done)):
                            i, j = state['i'], state['j']
                            if i == size:
                                break
                            left, right = self.points[i], self.points[j]
                            if left != self.params.curve.neg(right):
                                entries.append((i, j, left, right))
                            state['j'] += 1
                            if state['j'] == size:
                                state['i'] += 1
                                state['j'] = state['i']
                            done += 1
                            self.uses['pair_candidates_built'] += 1
                        if entries:
                            if pool is not None and len(entries) >= self.pairWorkers * 64:
                                width = (len(entries) + self.pairWorkers - 1) // self.pairWorkers
                                chunks = [[(i, j) for i, j, _, _ in entries[start:start + width]]
                                          for start in range(0, len(entries), width)]
                                keys = []
                                worker_counts = Counter()
                                for chunk_keys, counts in pool.map(_pairKeys, chunks):
                                    keys.extend(chunk_keys)
                                    worker_counts.update(counts)
                                self.ledger.counts.update(worker_counts)
                            else:
                                points = self.params.curve.addBatch(
                                    [(left, right) for _, _, left, right in entries])
                                keys = [None if point is None else pointKey(point) for point in points]
                            rows = []
                            for (i, j, _, _), key in zip(entries, keys):
                                if key is None:
                                    continue
                                serial = state['serial']
                                rows.append((serial, key, i, j))
                                state['hash'] = digest([state['hash'], serial, key, i, j])
                                state['serial'] += 1
                            if rows:
                                self.db.executemany('INSERT INTO pairs VALUES (?,?,?,?)', rows)
                        self.put('pair_state', state)
                    considered = consideredCount(size, state['i'], state['j'])
                    if done >= nextReport and done != budget:
                        self.progress.pairTable(considered, total, state['serial'], False, done, budget)
                        nextReport += reportEvery
            finally:
                self.db.execute('PRAGMA synchronous=%d' % previous_sync)
                if pool is not None:
                    pool.close()
                    pool.join()
            complete = state['i'] == size
            if complete:
                self.ensurePairIndex()
            considered = consideredCount(size, state['i'], state['j'])
            self.progress.pairTable(considered, total, state['serial'], complete, done, budget)
            return complete

    def checkPairs(self, state):
        size = len(self.points)
        require(0 <= state['i'] <= size and state['i'] <= state['j'] <= size,
                'invalid pair-table cursor')
        value, count = '0' * 64, 0
        for serial, key, i, j in self.db.execute('SELECT serial,point,first,second FROM pairs ORDER BY serial'):
            require(serial == count and 0 <= i <= j < size, 'invalid pair-table record')
            value = digest([value, serial, key, i, j])
            count += 1
        considered = consideredCount(size, state['i'], state['j'])
        excluded = sum(1 for i, point in enumerate(self.points)
                       if i <= self.index[self.params.curve.neg(point)] and
                       (i < state['i'] or (i == state['i'] and self.index[self.params.curve.neg(point)] < state['j'])))
        require(count == considered - excluded, 'pair-table cursor does not match its records')
        require(count == state['serial'] and value == state['hash'], 'pair-table checksum mismatch')
        require(self.meta('pair_state') is not None or count == 0, 'pair table has no checkpoint')

    def verify(self, witness, target):
        require(isinstance(witness, list) and len(witness) == self.params.summands,
                'invalid decomposition witness length')
        require(all(type(i) is int and 0 <= i < len(self.points) for i in witness), 'invalid witness point index')
        actual = [self.points[i] for i in witness]
        require(engine.proper(actual, [1] * len(actual), self.params.curve), 'inverse-cancelling relation')
        total, row = None, [0] * len(self.reps)
        for point in actual:
            total = self.params.curve.add(total, point)
            column, coeff = self.lookup[point]
            row[column] = (row[column] + coeff) % self.params.prime
        require(total == target, 'decomposition does not sum to the target')
        transported = None
        for coeff, point in zip(row, self.reps):
            transported = self.params.curve.add(transported, self.params.curve.mul(point, coeff))
        require(transported == target, 'orbit coefficients do not reconstruct the target')
        return row

    def prepareSat(self):
        require(self.params.weight is not None, 'SAT requires the Hamming-weight factor-base recipe')
        if self.context is None:
            with self.ledger.phase('symbolic_setup'):
                p = self.params
                prog, roots = decomp.buildSystem(p.onb.m, p.onb.n, p.summands, 12)
                self.context = (p.onb, p.curve, p.prime, p.generator, p.eigen,
                                self.reps, self.lookup, prog, roots)

    def decomposePolicy(self, target, seconds, arm):
        require(arm in selector.ARMS, 'unknown solver policy')
        fraction = selector.ARMS[arm]
        trace = []
        witness = None
        if fraction:
            started = time.perf_counter_ns()
            witness, details = self.decomposeExact(target, seconds * fraction, 'sat')
            trace.append({'solver': 'sat', 'budget_seconds': seconds * fraction,
                          'elapsed_ns': time.perf_counter_ns() - started, 'result': details})
        if witness is None:
            started = time.perf_counter_ns()
            witness, details = self.decomposeExact(target, seconds * (1 - fraction), 'pairs')
            trace.append({'solver': 'pairs', 'budget_seconds': seconds * (1 - fraction),
                          'elapsed_ns': time.perf_counter_ns() - started, 'result': details})
        return witness, {'status': 'verified' if witness is not None else details['status'],
                         'policy': arm, 'calls': trace}

    def decompose(self, target, seconds):
        if self.selector is None:
            return self.decomposeExact(target, seconds, self.solver)
        started = time.perf_counter_ns()
        with self.ledger.phase('selector_inference'):
            if seconds not in self.selectorDomains:
                self.selectorDomains[seconds] = selector.domain(self, seconds)
            values = selector.features(self, target, self.queryRank, self.queryDirect)
            arm, reason = self.selector.select(self.selectorDomains[seconds], values)
            if self.params.weight is None and arm != 'pairs':
                arm, reason = 'pairs', 'sat_unavailable_for_explicit_base'
        inference = time.perf_counter_ns() - started
        self.uses['selector_queries'] += 1
        self.uses['selector_policy_' + arm] += 1
        self.uses['selector_reason_' + reason] += 1
        witness, details = self.decomposePolicy(target, seconds, arm)
        details['selector'] = {'model_sha256': self.selector.hash, 'features': values,
                               'reason': reason, 'inference_ns': inference}
        return witness, details

    def decomposeExact(self, target, seconds, solver):
        require(solver in ('pairs', 'sat'), 'unknown exact solver')
        if target is None:
            return None, {'status': 'identity_excluded'}
        if solver == 'sat':
            self.prepareSat()
            row, details = engine.decompose(self.context, target, self.params.summands,
                self.params.weight, 'candidate', self.ledger, budget=seconds)
            if row is None:
                return None, details
            with self.ledger.phase('witness_verification'):
                lifted = engine.strictLift(self.params.onb, self.params.curve,
                    details['x_coordinates'], target, self.lookup)
                require(lifted is not None, 'SAT result failed independent lifting')
                actual, signs = lifted
                witness = [self.index[self.params.curve.neg(point) if sign < 0 else point]
                           for point, sign in zip(actual, signs)]
                require(self.verify(witness, target) == row, 'SAT row and witness disagree')
            return witness, details
        state = self.meta('pair_state')
        require(state is not None and state['i'] == len(self.points), 'pair table is incomplete')
        self.ensurePairIndex()
        deadline = time.monotonic() + seconds
        with self.ledger.phase('decomposition'):
            prefixes = [(None, None)] if self.params.summands == 2 else enumerate(self.points)
            for first, point in prefixes:
                if time.monotonic() >= deadline:
                    return None, {'status': 'budget'}
                residual = target if first is None else self.params.curve.add(target, self.params.curve.neg(point))
                self.ledger.counts['pairs.lookups'] += 1
                for i, j in self.db.execute('SELECT first,second FROM pairs WHERE point=? ORDER BY serial',
                                             (pointKey(residual),)):
                    if time.monotonic() >= deadline:
                        return None, {'status': 'budget'}
                    witness = [i, j] if first is None else [first, i, j]
                    actual = [self.points[index] for index in witness]
                    if engine.proper(actual, [1] * len(actual), self.params.curve):
                        self.verify(witness, target)
                        return witness, {'status': 'verified'}
            return None, {'status': 'unsat'}

    def probe(self, stream, number):
        # Rejection sampling avoids modulo bias; the immutable stream/counter
        # makes an interrupted probe reproducible without saving PRNG internals.
        def scalar(label):
            counter = 0
            size = (self.params.prime.bit_length() + 7) // 8
            while True:
                value = int.from_bytes(hashlib.shake_256(canonical(
                    [FORMAT, self.params.identity, stream, number, label, counter]).encode()).digest(size), 'big')
                value &= (1 << self.params.prime.bit_length()) - 1
                if 0 < value < self.params.prime:
                    return value
                counter += 1
        a, b = scalar('a'), 0 if stream == 'precompute' else scalar('b')
        target = self.params.curve.mul(self.params.generator, a)
        if b:
            target = self.params.curve.add(target, self.params.curve.mul(self.params.targets[stream[7:]], b))
        return a, b, target

    def matrix(self, targetName=None):
        size = len(self.reps) + (targetName is not None)
        matrix = engine.RelationMatrix(size, self.params.prime, self.ledger)
        streams = ['precompute'] + ([] if targetName is None else ['target:' + targetName])
        with self.ledger.phase('relation_replay'):
            for stream in streams:
                for number, encoded, coefficients, a, b in self.db.execute(
                        'SELECT number,witness,coefficients,a,b FROM relations WHERE stream=? ORDER BY number', (stream,)):
                    expectedA, expectedB, target = self.probe(stream, number)
                    require(integer(a) == expectedA and integer(b) == expectedB, 'relation probe coefficients changed')
                    row = self.verify(json.loads(encoded), target)
                    require(row == json.loads(coefficients), 'saved relation coefficients changed')
                    self.push(matrix, row, expectedA, expectedB, targetName is not None)
                    self.uses['relations_replayed'] += 1
        return matrix

    def push(self, matrix, row, a, b, direct):
        try:
            return matrix.push(row + ([-b % self.params.prime] if direct else []), a)
        except AssertionError as error:
            raise ValueError('inconsistent relation matrix') from error

    def collect(self, attempts, seconds, targetName=None, matrix=None):
        stream = 'precompute' if targetName is None else 'target:' + targetName
        matrix = self.matrix(targetName) if matrix is None else matrix
        cursor = self.db.execute('SELECT COALESCE(MAX(number)+1,0) FROM attempts WHERE stream=?', (stream,)).fetchone()[0]
        for _ in range(attempts):
            if (targetName is None and matrix.solution() is not None) or (
                    targetName is not None and len(self.reps) in matrix.rows):
                break
            with self.ledger.phase('probe_generation'):
                a, b, target = self.probe(stream, cursor)
            self.queryRank, self.queryDirect = len(matrix.rows), targetName is not None
            witness, details = self.decompose(target, seconds)
            details['solver'] = self.solver
            relation = None
            with self.ledger.phase('relation_commit'):
                with self.db:
                    if witness is not None:
                        row = self.verify(witness, target)
                        independent = self.push(matrix, row, a, b, targetName is not None)
                        details['independent'] = independent
                        self.db.execute('INSERT INTO relations(stream,number,witness,coefficients,a,b,independent) '
                                        'VALUES (?,?,?,?,?,?,?)',
                            (stream, cursor, canonical(witness), canonical(row), str(a), str(b),
                             int(bool(independent))))
                        self.uses['new_relations'] += 1
                        self.uses['new_independent_relations'] += int(independent)
                        relation = self.relationRecord(stream, cursor, witness, row, str(a), str(b),
                                                       bool(independent))
                    self.db.execute('INSERT INTO attempts VALUES (?,?,?)', (stream, cursor, canonical(details)))
                self.uses['new_attempts'] += 1
            if relation is not None:
                self.saveRelation(relation)
                self.progress.relation(relation, sum(1 for value in row if value),
                                       len(matrix.rows), matrix.size)
            elapsed = time.monotonic() - self.progress.started
            self.progress.attempt(stream, cursor, details['status'], len(matrix.rows), matrix.size,
                                  self.uses['new_relations'], self.uses['new_independent_relations'],
                                  self.uses['new_attempts'] / max(1e-9, elapsed))
            cursor += 1
        return matrix

    def logs(self, matrix=None):
        with self.ledger.phase('log_load'):
            logs = self.meta('logs')
            if logs is not None:
                self.certifyLogs(logs)
                self.uses['log_database_loads'] += 1
                return logs
        matrix = self.matrix() if matrix is None else matrix
        logs = matrix.solution()
        if logs is not None:
            with self.ledger.phase('log_certification'):
                self.certifyLogs(logs)
                with self.db:
                    self.put('logs', logs)
                self.uses['log_database_builds'] += 1
        return logs

    def certifyLogs(self, logs):
        require(isinstance(logs, list) and len(logs) == len(self.reps), 'invalid logarithm database size')
        for value, point in zip(logs, self.reps):
            require(type(value) is int and 0 <= value < self.params.prime and
                    self.params.curve.mul(self.params.generator, value) == point,
                    'factor-base logarithm certificate failed')

    def solve(self, attempts, seconds, logs):
        results = {}
        for name, target in self.params.targets.items():
            self.progress.message('[4/4] target %s: %s' % (
                name, 'replaying saved scalar' if self.db.execute(
                    'SELECT 1 FROM solutions WHERE id=?', (name,)).fetchone() is not None else 'collecting'))
            saved = self.db.execute('SELECT scalar FROM solutions WHERE id=?', (name,)).fetchone()
            if saved is not None:
                value = integer(saved[0])
                with self.ledger.phase('solution_replay'):
                    self.certifySolution(value, target)
                results[name] = {'status': 'complete', 'scalar': str(value), 'verified': True, 'reused': True}
                continue
            if logs is None:
                matrix = self.collect(attempts, seconds, name)
                pinned = matrix.rows.get(len(self.reps))
                value = pinned[-1] if pinned is not None else None
            else:
                # Existing target relations can complete descent after a crash
                # between saving the relation and saving the scalar.
                value = None
                self.matrix(name)  # Verify every reused witness and coefficient.
                for coefficients, a, b in self.db.execute(
                        'SELECT coefficients,a,b FROM relations WHERE stream=? ORDER BY number', ('target:' + name,)):
                    row = json.loads(coefficients)
                    value = (sum(x * y for x, y in zip(row, logs)) - integer(a)) * pow(integer(b), -1, self.params.prime) % self.params.prime
                    break
                if value is None:
                    # One relation suffices once the base logs are known. Seed
                    # the reduced matrix with those already certified values.
                    matrix = self.matrix(name)
                    with self.ledger.phase('known_logs_matrix'):
                        for column, log in enumerate(logs):
                            row = [int(i == column) for i in range(len(self.reps))]
                            self.push(matrix, row, log, 0, True)
                    matrix = self.collect(attempts, seconds, name, matrix)
                    pinned = matrix.rows.get(len(self.reps))
                    value = pinned[-1] if pinned is not None else None
            if value is None:
                results[name] = {'status': 'insufficient_relations', 'verified': False}
            else:
                with self.ledger.phase('scalar_certification'):
                    self.certifySolution(value, target)
                    with self.db:
                        self.db.execute('INSERT INTO solutions VALUES (?,?)', (name, str(value)))
                results[name] = {'status': 'complete', 'scalar': str(value), 'verified': True, 'reused': False}
        return results

    def certifySolution(self, value, target):
        require(0 <= value < self.params.prime and self.params.curve.mul(self.params.generator, value) == target,
                'scalar certificate failed')

    def run(self, stage='all', attempts=32, pairBudget=10000, seconds=.25):
        require(stage in ('select', 'pairs', 'collect', 'logs', 'solve', 'all', 'status'), 'invalid stage')
        require(type(attempts) is int and 0 <= attempts <= 1000000, 'attempt budget must be between 0 and 1000000')
        require(math.isfinite(seconds) and 0 < seconds <= 3600, 'solver budget must be between 0 and 3600 seconds')
        require(type(pairBudget) is int and 0 <= pairBudget <= 10000000, 'pair budget must be between 0 and 10000000')
        report = {'schema_version': 1, 'operation': 'fixed', 'stage': stage,
                  'status': 'stopped', 'identity': self.params.identity, 'degree': self.params.onb.m,
                  'subgroup_order': str(self.params.prime), 'orbit_columns': len(self.reps),
                  'signed_base_points': len(self.points), 'solver': self.solver, 'targets': {}}
        self.progress.message('[0/4] campaign: degree %d, %d orbit columns, %d signed points, stage=%s, solver=%s'
                              % (self.params.onb.m, len(self.reps), len(self.points), stage, self.solver))
        if self.relationsOutPath is not None:
            report['relations_out'] = str(self.relationsOutPath)
        count = math.comb(len(self.points) + self.params.summands - 1, self.params.summands)
        denominator = self.params.prime - 1
        report['coverage_ceiling'] = {'numerator': str(min(count, denominator)), 'denominator': str(denominator),
            'fraction': min(1.0, count / denominator), 'kind': 'counting_bound_on_uniform_nonzero_targets'}
        ready = True
        savedTargets = bool(self.params.targets) and all(self.db.execute(
            'SELECT 1 FROM solutions WHERE id=?', (name,)).fetchone() is not None for name in self.params.targets)
        replayOnly = stage in ('solve', 'all') and savedTargets
        if self.solver in ('pairs', 'learned') and not replayOnly and stage not in ('select', 'status', 'logs'):
            ready = self.buildPairs(pairBudget)
            if not ready:
                report['status'] = 'pair_budget'
        if ready:
            matrix = None
            if not replayOnly and stage in ('all', 'collect') and self.logs() is None:
                self.progress.message('[2/4] collection: starting; up to %d attempts, %s solver, %s s per query'
                                      % (attempts, self.solver, seconds))
                matrix = self.collect(attempts, seconds)
            logs = self.logs(matrix) if stage in ('all', 'collect', 'logs', 'solve') else None
            if stage in ('all', 'collect', 'logs', 'solve'):
                self.progress.message('[3/4] factor-base logs: %s'
                                      % ('available' if logs is not None else 'incomplete'))
            if stage in ('collect', 'logs'):
                report['status'] = 'complete' if logs is not None else 'insufficient_relations'
            if stage in ('all', 'solve'):
                report['targets'] = self.solve(attempts, seconds, logs)
                complete = bool(report['targets']) and all(t['verified'] for t in report['targets'].values())
                report['status'] = 'complete' if complete or (not report['targets'] and logs is not None) else 'insufficient_relations'
            if stage == 'pairs':
                report['status'] = 'complete'
        report['selector_model_sha256'] = None if self.selector is None else self.selector.hash
        report['pair_table'] = self.meta('pair_state')
        report['logs_available'] = self.meta('logs') is not None
        report['relations_saved'] = self.db.execute('SELECT COUNT(*) FROM relations').fetchone()[0]
        report['attempts_saved'] = self.db.execute('SELECT COUNT(*) FROM attempts').fetchone()[0]
        report['reuse'] = dict(self.uses)
        report['accounting'] = self.ledger.report()
        report['accounting']['scope'] = 'through result assembly, including artifact reads, work commits and certificate checks; excludes final report archival/output and connection close; external benchmark wall time includes these'
        report['cumulative_measured_ns'] = report['accounting']['elapsed_ns'] + sum(
            json.loads(row[0])['accounting']['elapsed_ns'] for row in self.db.execute('SELECT report FROM invocations'))
        report['source_sha256'] = {name: hashlib.sha256(Path(__file__).with_name(name).read_bytes()).hexdigest()
                                  for name in ('indexcalc_fixed.py', 'indexcalc_selector.py', 'indexcalc_e2e.py', 'field.py', 'curves.py', 'decomp.py', 'cnf.py')}
        report['previous_invocations'] = self.db.execute('SELECT COUNT(*) FROM invocations').fetchone()[0]
        self.progress.message('[4/4] done: status=%s, %d relations saved, %d attempts saved'
                              % (report['status'], report['relations_saved'], report['attempts_saved']))
        with self.db:
            self.db.execute('INSERT INTO invocations(report) VALUES (?)', (canonical(report),))
        return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--params', required=True)
    parser.add_argument('--dir', required=True)
    parser.add_argument('--stage', choices=['select', 'pairs', 'collect', 'logs', 'solve', 'all', 'status'], default='all')
    parser.add_argument('--solver', choices=['pairs', 'sat', 'learned'], default='pairs')
    parser.add_argument('--selector-model', help='JSON model trained on matched natural-query panels')
    parser.add_argument('--attempts', type=int, default=32)
    parser.add_argument('--pair-budget', type=int, default=10000)
    parser.add_argument('--query-seconds', type=float, default=.25)
    parser.add_argument('--progress', action='store_true',
                        help='print live stage, pair-table, attempt and relation progress to stderr')
    parser.add_argument('--progress-every', type=int, default=100,
                        help='attempts between progress heartbeats; 0 prints only relations and stage changes')
    parser.add_argument('--relations-out',
                        help='rewrite saved relations as JSON Lines, then append each new verified relation')
    parser.add_argument('--pair-workers', type=int, default=1,
                        help='processes used to compute pair sums (1 disables the pool)')
    args = parser.parse_args()
    try:
        with Campaign(readJson(args.params), args.dir, args.solver, selectorModel=args.selector_model,
                      progress=Progress(args.progress, args.progress_every),
                      relationsOut=args.relations_out, pairWorkers=args.pair_workers) as campaign:
            report = campaign.run(args.stage, args.attempts, args.pair_budget, args.query_seconds)
    except (ValueError, OSError, sqlite3.Error, RuntimeError) as error:
        message = str(error)
        report = {'schema_version': 1, 'operation': 'fixed', 'stage': args.stage,
                  'status': 'error', 'message': message}
        lowered = message.lower()
        if 'disk is full' in lowered or 'disk i/o error' in lowered or 'database or disk is full' in lowered:
            report['hint'] = ('the campaign directory ran out of space; choose --dir on a volume '
                              'with free space or stop earlier with --pair-budget')
        elif 'pair table needs about' in lowered:
            report['hint'] = ('choose a campaign directory on a volume with more free space, '
                              'or lower --pair-budget to build incrementally')
        elif 'readonly' in lowered or 'read-only' in lowered:
            report['hint'] = 'the campaign directory is not writable; check permissions or use --dir'
        elif 'no such table' in lowered or 'malformed' in lowered:
            report['hint'] = ('the campaign database changed or disappeared while running; '
                              'do not move or delete an active campaign directory')
    print(json.dumps(report, sort_keys=True))
    return 0 if report['status'] in ('complete', 'stopped') else 2


if __name__ == '__main__':
    sys.exit(main())
