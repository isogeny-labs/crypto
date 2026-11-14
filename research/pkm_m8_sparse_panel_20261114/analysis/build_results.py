#!/usr/bin/env python3
"""Post-run analysis pipeline for PKM-M8-PANELA-20261114 (protocol v2.2).   [pcc:unverified]

    python3 analysis/build_results.py --results results/<stamp> \
        [--results-replacement results/<stamp>/replacement] --out results/<stamp>/analysis

Reads the launcher's fetched layout (runs.jsonl, results.tgz, done.json, verify/, rows_meta/, env/), unpacks
results.tgz INTO --out (never over the raw files), joins rows with the plan by `order`, runs the instrument's
`check` and `compare-gb`, computes the quantities PROTOCOL.md section 4-5 define (and nothing else), and writes
RESULTS.md, analysis.json, figures/work_vs_B.{svg,png}, REPORT.typ (+ REPORT.pdf when typst exists).
Read-only on the study directory (PROTOCOL.md, FREEZE.json, pod/, cells/). Python 3 stdlib + matplotlib (figure only).
Every number is [pcc:unverified]: an assertion by this script, not a filed pcc claim.
"""
import argparse, datetime, hashlib, json, math, os, re, shutil, subprocess, sys, tarfile
from pathlib import Path

PCC = '[pcc:unverified]'
GATE = 3.5          # all-output gate of the frozen rule
GATE_ONE = 3.0      # one-output gate: reported, not used
DRIFT = 1.5         # canary drift flag (section 6)
RULE_FALLBACK = ("(PROTOCOL.md section 5 could not be read; the frozen rule is quoted from the study file at run time.)")
STUDY = Path(__file__).resolve().parent.parent
REQUIRED_GATES = ['exporter_c', 'second_instrument', 'positive', 'negative', 'aa', 'canary']

# ----------------------------------------------------------------------------- small helpers
def sha256_file(p):
    try:
        h = hashlib.sha256()
        with open(p, 'rb') as f:
            for blk in iter(lambda: f.read(1 << 20), b''):
                h.update(blk)
        return h.hexdigest()
    except OSError:
        return None

def jload(p, default=None):
    try:
        with open(p) as f:
            return json.load(f)
    except (OSError, ValueError):
        return default

def lg2(num, den):
    """log2(num/den); None if undefined; -inf when num == 0."""
    if num is None or den is None or den <= 0 or num < 0:
        return None
    return -math.inf if num == 0 else math.log2(num / den)

def fnum(x, nd=3):
    if x is None:
        return ''
    if isinstance(x, bool):
        return str(x).lower()
    if isinstance(x, float):
        if math.isinf(x):
            return '+inf' if x > 0 else '-inf'
        if x != 0 and (abs(x) >= 1e6 or abs(x) < 1e-3):
            return f'{x:.4g}'
        return f'{x:.{nd}f}'.rstrip('0').rstrip('.') if abs(x) < 1e6 else f'{x:.4g}'
    return str(x)

def na(x, nd=3):
    return 'n/a' if x is None else fnum(x, nd)

def fproxy(x):
    return '' if x is None else f'{x:.6g}'

def md_table(head, rows):
    out = ['| ' + ' | '.join(head) + ' |', '|' + '|'.join(['---'] * len(head)) + '|']
    for r in rows:
        out.append('| ' + ' | '.join(str(c).replace('|', '\\|') for c in r) + ' |')
    return '\n'.join(out)

def num(x):
    return x if isinstance(x, (int, float)) and not isinstance(x, bool) else None

# ----------------------------------------------------------------------------- loading
def safe_extract(tgz, dest, notes, tag):
    """Extract results.tgz into dest tolerating a truncated archive (partial rows are kept as evidence)."""
    dest.mkdir(parents=True, exist_ok=True)
    n = 0
    try:
        with tarfile.open(tgz, 'r:gz') as tf:
            while True:
                try:
                    m = tf.next()
                except (EOFError, tarfile.TarError, OSError) as e:
                    notes.append(f'{tag}: results.tgz read stopped after {n} members ({type(e).__name__}: {e}); archive truncated')
                    break
                if m is None:
                    break
                if not (m.isfile() or m.isdir()):
                    continue
                try:
                    tf.extract(m, dest, filter='data')
                    n += 1
                except (OSError, tarfile.TarError, EOFError) as e:
                    notes.append(f'{tag}: could not extract {m.name}: {e}')
    except (OSError, tarfile.TarError, EOFError) as e:
        notes.append(f'{tag}: cannot open results.tgz ({e})')
    return n

def load_source(resdir, outdir, tag):
    """Everything fetched for one pod: runs rows, done.json, unpacked archive."""
    resdir = Path(resdir)
    src = dict(tag=tag, dir=str(resdir), notes=[], runs=[], done=None, runs_sha256=None, tgz_sha256=None,
               unpack=None, exists=resdir.is_dir())
    if not src['exists']:
        src['notes'].append(f'{tag}: results directory {resdir} does not exist')
        return src
    tgz = resdir / 'results.tgz'
    if tgz.is_file():
        src['tgz_sha256'] = sha256_file(tgz)
        dest = outdir / 'unpacked' / tag
        if dest.exists():
            shutil.rmtree(dest)
        safe_extract(tgz, dest, src['notes'], tag)
        src['unpack'] = dest / 'out'
    else:
        src['notes'].append(f'{tag}: no results.tgz (rows may be partial; log tables come from rows_meta/)')
    src['done'] = jload(resdir / 'done.json')
    if src['done'] is None:
        src['notes'].append(f'{tag}: no done.json: PARTIAL results (pod lost, deadline, or still running)')
    elif src['tgz_sha256'] and src['done'].get('results_tgz_sha256') not in (None, src['tgz_sha256']):
        src['notes'].append(f'{tag}: results.tgz sha256 differs from done.json')
    cands = [resdir / 'runs.jsonl'] + ([src['unpack'] / 'runs.jsonl'] if src['unpack'] else [])
    best = None
    for c in cands:
        if c.is_file():
            rows, bad = [], 0
            for ln in c.read_text(errors='replace').splitlines():
                if ln.strip():
                    try:
                        rows.append(json.loads(ln))
                    except ValueError:
                        bad += 1
            if bad:
                src['notes'].append(f'{tag}: {bad} unparsable line(s) in {c}')
            if best is None or len(rows) > len(best[1]):
                best = (c, rows)
    if best:
        src['runs'] = best[1]
        src['runs_path'] = str(best[0])
        src['runs_sha256'] = sha256_file(best[0])
    else:
        src['notes'].append(f'{tag}: no runs.jsonl found')
    return src

def row_id(run):
    mode = run.get('msolve_mode') or 'native'
    return f"{int(run['order']):03d}_{run['cell']}_{run['target']}_{run['solver']}_{mode}"

def find_file(src, rid, name):
    """Locate rows/<rid>/<name> in the unpacked archive, else the launcher's synced copies."""
    c = []
    if src.get('unpack'):
        c.append(src['unpack'] / 'rows' / rid / name)
    d = Path(src['dir'])
    if name == 'verify.json':
        c.append(d / 'verify' / f'{rid}.json')
    elif name in ('log_table.json', 'watchdog.json'):
        c.append(d / 'rows_meta' / rid / name)
    for p in c:
        if p.is_file():
            return p
    return None

# ----------------------------------------------------------------------------- study data
class Study:
    def __init__(self, root, plan_path=None):
        self.root = Path(root)
        self.cells_dir = self.root / 'cells'
        self.freeze = jload(self.root / 'FREEZE.json', {})
        self.plan_path = plan_path
        self._cells, self._targets = {}, {}

    @staticmethod
    def split_cell(cell):
        m = re.fullmatch(r'(.+)-m(\d+)', cell)
        return (m.group(1), int(m.group(2))) if m else (cell, 8)

    def cell_json(self, cell):
        base, _ = self.split_cell(cell)
        if base not in self._cells:
            self._cells[base] = jload(self.cells_dir / base / 'cell.json', {})
        return self._cells[base]

    def targets(self, cell):
        base, m = self.split_cell(cell)
        key = (base, m)
        if key not in self._targets:
            sub = self.cells_dir / base / (f'm{m}' if m != 8 else '')
            t = jload(sub / 'targets.json', {})
            self._targets[key] = {x['name']: x for x in t.get('targets', [])}
        return self._targets[key]

    def hidden_path(self, cell):
        base, m = self.split_cell(cell)
        return self.cells_dir / base / (f'm{m}' if m != 8 else '') / 'hidden.json'

    def B(self, cell):
        b = self.cell_json(cell).get('B')
        if b:
            return int(b)
        m = re.match(r't(\d+)r', cell)
        return 2 ** int(m.group(1)) if m else None

    def setup_cpu_s(self, cell):
        """Base-construction cost in CPU s if cell.json records one; the instrument's cell.json does not today."""
        for k, v in self.cell_json(cell).items():
            if re.search(r'(base|setup|generation).*(cpu|secs|seconds)', k, re.I) and num(v) is not None:
                return float(v)
        return None

    def rule_text(self):
        try:
            t = (self.root / 'PROTOCOL.md').read_text()
            a = t.index('**Rule (one verdict per observable')
            b = t.index('Censoring reading, stated before data')
            c = t.index('\n\n', b)
            return t[a:c].strip()
        except (OSError, ValueError):
            return RULE_FALLBACK

# ----------------------------------------------------------------------------- per-row records
def parse_time_v(path):
    try:
        tm = Path(path).read_text(errors='replace')
    except (OSError, TypeError):
        return None, None
    u = re.search(r'User time \(seconds\): ([\d.]+)', tm)
    s = re.search(r'System time \(seconds\): ([\d.]+)', tm)
    return (float(u.group(1)) if u else None, float(s.group(1)) if s else None)

def build_record(run, plan_row, src, study):
    rid = row_id(run)
    cell, target, kind = run['cell'], run['target'], run.get('kind')
    base, m = study.split_cell(cell)
    tg = study.targets(cell).get(target, {})
    mode = run.get('msolve_mode') or ('native' if run['solver'] == 'native' else None)
    rec = dict(rid=rid, order=int(run['order']), kind=kind, cell=cell, base_cell=base, m=m, target=target,
               solver=run['solver'], mode=mode, threads=run.get('threads'),
               B=study.B(cell) if cell != 'ctrl' else None,
               label=('planted' if kind == 'planted' else tg.get('label')) if tg or kind == 'planted' else None,
               algebraic_count=tg.get('algebraic_count'), real_count=tg.get('real_count'),
               host=run.get('pod_id'), host_tag=src['tag'], replacement=bool(run.get('replacement')) or src['tag'] != 'primary',
               wall_s=num(run.get('wall_s')), exit_code=run.get('exit_code'), censored_raw=run.get('censored'),
               max_rss_gb=(run['max_rss_kb'] * 1024 / 1e9) if num(run.get('max_rss_kb')) is not None else None,
               timeout_s=run.get('timeout_s'), notes=[])
    # --- cpu seconds of the solver process: user_s + sys_s from `time -v` (runs.jsonl), time_v.txt as fallback
    u, s = num(run.get('user_s')), num(run.get('sys_s'))
    rid_tv = find_file(src, rid, 'time_v.txt')
    if (u is None or s is None) and rid_tv:
        u2, s2 = parse_time_v(rid_tv)
        u, s = (u if u is not None else u2), (s if s is not None else s2)
        rec['notes'].append('cpu from time_v.txt (runs.jsonl lacked user_s/sys_s)')
    rec['cpu_s'] = (u + s) if u is not None and s is not None else None
    # --- log table (work proxy)
    lt_file = find_file(src, rid, 'log_table.json')
    lt = (jload(lt_file, {}) or {}).get('totals') if lt_file else None
    lt = lt or run.get('log_table') or {}
    rec['work_proxy'] = num(lt.get('sum_rows_x_cols_x_density'))
    rec['max_degree'] = lt.get('max_round_degree')
    rec['rounds'] = lt.get('rounds')
    rec['u8'] = lt.get('up_to_degree_8')
    rec['log_incomplete_last_row'] = lt.get('incomplete_last_row')
    # --- verify
    vinfo = run.get('verify') or {}
    vfile = find_file(src, rid, 'verify.json')
    vj = jload(vfile) if vfile else None
    planned_verify = plan_row.get('verify') if plan_row and 'verify' in plan_row else (
        run['solver'] == 'msolve' and mode == 'param' and kind in ('aa', 'planted', 'natural', 'probe'))
    rec['verify_planned'] = bool(planned_verify)
    rec['verify_ran'] = vinfo.get('exit_code') is not None or vj is not None
    rec['verify_exit'] = vinfo.get('exit_code')
    rec['verify_file'] = str(vfile) if vfile else None
    rec['verify_json'] = vj
    if vj and num(vj.get('cpu_secs')) is not None:
        rec['verify_cpu_s'], rec['verify_cpu_src'] = float(vj['cpu_secs']), 'verify.json:cpu_secs'
    elif num(vinfo.get('wall_s')) is not None:
        rec['verify_cpu_s'], rec['verify_cpu_src'] = float(vinfo['wall_s']), 'runs.jsonl:verify.wall_s (wall, not CPU)'
    else:
        rec['verify_cpu_s'], rec['verify_cpu_src'] = None, None
    ex = (vj or {}).get('extraction') or {}
    rec['real_dec'] = ex.get('real_signed_decompositions')
    rec['useful'] = ex.get('useful_relations')
    rec['rank_inc'] = ex.get('rank_increment')
    rec['fp_sols'] = ex.get('fp_rational_solutions')
    rec['verify_deadline_hit'] = bool((vj or {}).get('deadline_hit_in_root_finding'))
    # --- completeness (section 4: a verify that hits its deadline, or never completes, makes the row incomplete)
    outcome = None
    if run['solver'] == 'native':
        so = jload(find_file(src, rid, 'solve.out') or '') or {}
        raw_ok = run.get('censored') == 'ok' and run.get('exit_code') == 0 and so.get('censored') is False
        if run.get('censored') == 'ok' and so.get('censored') is not False:
            outcome = 'native_censored'
        gb = (src['unpack'] / 'rows' / rid / 'native.gb.txt') if src.get('unpack') else None
        rec['native_gb'] = str(gb) if gb and gb.is_file() else None
    else:
        raw_ok = run.get('censored') == 'ok' and run.get('exit_code') == 0
    rec['msolve_outcome'] = run.get('msolve_outcome')
    if not raw_ok:
        cls = outcome or str(run.get('censored') or 'no_status')
        complete = False
    elif rec['verify_planned'] and (not rec['verify_ran'] or rec['verify_exit'] not in (0,) or vj is None or rec['verify_deadline_hit']):
        cls, complete = 'verify_incomplete', False
        rec['notes'].append('verify missing, failed or hit its deadline: c is a lower bound')
    else:
        cls, complete = 'complete', True
    rec['class'], rec['complete'] = cls, complete
    # --- c = cpu_s(msolve) + cpu_s(verify); lower bound when incomplete
    if rec['cpu_s'] is not None:
        rec['c'] = rec['cpu_s'] + (rec['verify_cpu_s'] or 0.0)
        if rec['verify_planned'] and rec['verify_ran'] and rec['verify_cpu_s'] is None:
            rec['c'] = rec['cpu_s']
            rec['notes'].append('verify ran without timing: c omits verify (lower bound)')
            rec['complete'], rec['class'] = False, 'verify_incomplete'
    else:
        rec['c'] = None
    # observables: value is exact if complete, else a lower bound
    rec['v'] = {'cpu_s': rec['c'], 'work_proxy': rec['work_proxy']}
    st = study.setup_cpu_s(cell) if rec['B'] else None
    rec['setup_cpu_s'] = st
    rec['c_B'] = (rec['c'] + st) if (rec['c'] is not None and st is not None) else None
    rec['v_B'] = rec['c_B']
    rec['plan_mismatch'] = bool(plan_row and (plan_row.get('cell'), plan_row.get('target'), plan_row.get('kind')) != (cell, target, kind))
    return rec

# ----------------------------------------------------------------------------- gamma arithmetic
def gamma_block(nat2, nat3, probe, planned2, planned3, key='v'):
    """gamma(4->8) secants for one observable.

    nat2 / nat3: {order: record}; record[key] is the observable (None if unrecorded) and record['complete'] says
    whether it is an exact c (True) or a lower bound c_lo (False). planned2 / planned3: planned natural orders.
    probe: record of the t4 probe row or None.
    """
    val = lambda r: r[key] if isinstance(r[key], (int, float)) else None
    g = dict(gamma_lo=None, gamma_lo_reason=None, gamma_lo_complete_basis=False, gamma_hi=math.inf, gamma_hi_lb_incl_censored=None,
             gamma_lo_8_16=None, gamma_lo_8_16_reason=None)
    miss2 = [o for o in planned2 if o not in nat2]
    inc2 = [o for o in planned2 if o in nat2 and not nat2[o]['complete']]
    done2 = {o: val(r) for o, r in nat2.items() if r['complete'] and val(r) is not None}
    miss3 = [o for o in planned3 if o not in nat3]
    present3 = {o: r for o, r in nat3.items()}
    g.update(n_t2_planned=len(planned2), n_t2_complete=len(done2), t2_missing=miss2, t2_incomplete=inc2,
             n_t3_planned=len(planned3), n_t3_present=len(present3), t3_missing=miss3)
    c4max = max(done2.values()) if done2 else None
    c4min = min(done2.values()) if done2 else None
    g['max_c4'], g['min_c4'] = c4max, c4min
    comp3 = [val(r) for r in present3.values() if r['complete'] and val(r) is not None]
    unk3 = [o for o, r in present3.items() if val(r) is None]
    lo8 = [val(r) for r in present3.values() if val(r) is not None]
    g['min_c_lo8'] = min(lo8) if lo8 else None
    g['max_c8_complete'] = max(comp3) if comp3 else None
    g['n_t3_complete'] = len(comp3)
    g['n_t3_censored'] = sum(1 for r in present3.values() if not r['complete'])
    # gamma_lo
    why = None
    if not planned2:
        why = 'no natural t = 2 row is planned/recorded'
    elif miss2 or inc2:
        why = ('natural t = 2 row(s) did not complete: ' + ', '.join(f'order {o} ({"not run" if o in miss2 else nat2[o].get("class", "incomplete")})' for o in miss2 + inc2))
    elif not present3:
        why = 'no natural t = 3 row recorded'
    elif unk3:
        why = f'natural t = 3 row(s) {unk3} carry no recorded value for this observable (lower bound unknown)'
    elif c4max is None or c4max <= 0:
        why = 'max c(4) is not positive'
    if why is None:
        g['gamma_lo'] = lg2(g['min_c_lo8'], c4max)
        g['gamma_lo_complete_basis'] = not miss3
        if miss3:
            g['gamma_lo_reason'] = (f'provisional: planned natural t = 3 row(s) {miss3} have no record, the minimum over the rows present is '
                                    'anti-conservative for KILL')
    else:
        g['gamma_lo_reason'] = why
    # gamma_hi: defined only if >= 1 natural t = 3 row completed in mode param
    if comp3 and c4min:
        g['gamma_hi'] = lg2(max(comp3), c4min)
    if lo8 and c4min and g['n_t3_censored']:
        g['gamma_hi_lb_incl_censored'] = lg2(max(lo8), c4min)
    # gamma_lo(8->16)
    if probe is None:
        g['gamma_lo_8_16_reason'] = 'probe did not run / not recorded'
    elif val(probe) is None:
        g['gamma_lo_8_16_reason'] = 'probe carries no value for this observable'
    elif miss3 or g['n_t3_censored'] or not comp3:
        g['gamma_lo_8_16_reason'] = 'undefined: a natural t = 3 row is censored or missing'
    else:
        g['gamma_lo_8_16'] = lg2(val(probe), max(comp3))
        g['probe_complete'] = probe['complete']
    return g

def decide_observable(obs, g, rho_aa, gates, reading_b=None, positives_t3=0):
    """One verdict for one observable by the frozen rule. Returns (verdict, [reasons])."""
    reasons = []
    bad = [k for k in REQUIRED_GATES if gates.get(k, {}).get('status') == 'fail']
    ne = [k for k in REQUIRED_GATES if gates.get(k, {}).get('status') != 'pass' and k not in bad]
    lo, hi = g['gamma_lo'], g['gamma_hi']
    notk_terms = []
    if obs == 'cpu_s':
        rho_term = lg2(rho_aa, 1.0) if rho_aa else None
        hi_eff = (hi + rho_term) if (rho_term is not None and not math.isinf(hi)) else (math.inf if math.isinf(hi) else None)
    else:
        hi_eff = hi
    # --- KILL
    kill = lo is not None and g['gamma_lo_complete_basis'] and lo >= GATE
    if lo is None:
        reasons.append(f'gamma_lo undefined: {g["gamma_lo_reason"]}')
    elif not g['gamma_lo_complete_basis']:
        reasons.append(g['gamma_lo_reason'])
    elif lo < GATE:
        reasons.append(f'gamma_lo = {fnum(lo)} < {GATE}')
    if bad:
        reasons.append('required gate(s) failed: ' + ', '.join(bad))
    if ne:
        reasons.append('required control(s) not passed / not evaluable: ' + ', '.join(ne))
    if kill and not bad and not ne:
        return 'KILL (scoped)', []
    # --- NOT KILLED at B <= 8
    nk_ok = (hi_eff is not None and hi_eff < GATE and g['n_t3_complete'] > 0 and g['n_t3_censored'] == 0
             and not g['t3_missing'] and not g['t2_missing'] and not g['t2_incomplete'] and g['n_t2_planned'] > 0
             and positives_t3 > 0 and not bad and not ne)
    if obs == 'cpu_s' and reading_b is not None and reading_b != 'pass':
        nk_ok = False
        reasons.append(f'cost reading B (with setup share): {reading_b}; NOT KILLED must hold on both readings')
    if nk_ok:
        return 'NOT KILLED at B <= 8', []
    if g['n_t3_complete'] == 0 and g['n_t3_present'] > 0:
        reasons.append('no natural t = 3 row completed: gamma_hi = +inf')
    elif g['n_t3_censored']:
        reasons.append('a censored t = 3 natural row never supports NOT KILLED')
    if hi_eff is not None and not math.isinf(hi_eff) and hi_eff >= GATE:
        reasons.append(f'gamma_hi{" + log2(rho_AA)" if obs == "cpu_s" else ""} = {fnum(hi_eff)} >= {GATE}')
    if lo is not None and not math.isinf(hi) and lo < GATE <= (hi_eff if hi_eff is not None else hi):
        reasons.append(f'interval [{fnum(lo)}, {fnum(hi_eff)}] straddles {GATE}')
    if g['n_t3_complete'] and positives_t3 == 0:
        reasons.append('no verified natural positive at t = 3')
    return 'INCONCLUSIVE', reasons

def panel_verdict(vc, vp):
    a, b = vc[0], vp[0]
    if a.startswith('KILL') and b.startswith('KILL'):
        return 'KILL (scoped)', []
    if a.startswith('NOT KILLED') and b.startswith('NOT KILLED'):
        return 'NOT KILLED at B <= 8 (reported as INCONCLUSIVE with direction; held-out sizes unavailable, no ADVANCE)', []
    why = []
    if a != b:
        why.append(f'observables disagree: cpu_s {a}, work_proxy {b}')
    why += [f'cpu_s: {r}' for r in vc[1]] + [f'work_proxy: {r}' for r in vp[1]]
    return 'INCONCLUSIVE', why

# ----------------------------------------------------------------------------- instrument runs
def run_instrument(cmd, timeout=600):
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
        return p.returncode, (p.stdout + p.stderr).strip()
    except (OSError, subprocess.TimeoutExpired) as e:
        return -1, f'{type(e).__name__}: {e}'

def do_checks(recs, study, instr, outdir, notes):
    chk, cmp_ = {}, []
    cdir, mdir = outdir / 'checks', outdir / 'compare'
    cdir.mkdir(exist_ok=True)
    mdir.mkdir(exist_ok=True)
    if not instr or not Path(instr).is_file():
        notes.append(f'instrument binary not found ({instr}): check / compare-gb NOT run; controls depending on them are not evaluable')
        return chk, cmp_
    for r in recs:
        if r['solver'] == 'msolve' and r['verify_file']:
            hid = study.hidden_path(r['cell'])
            out = cdir / f"{r['rid']}.check.json"
            if not hid.is_file():
                chk[r['rid']] = dict(error=f'hidden file {hid} not found')
                continue
            rc, msg = run_instrument([instr, 'check', '--hidden', str(hid), '--verify', r['verify_file'], '--out', str(out)])
            d = jload(out)
            chk[r['rid']] = d if d is not None else dict(error=f'check rc={rc}: {msg[:300]}')
            chk[r['rid']]['_file'] = str(out)
    by = {}
    for r in recs:
        by.setdefault((r['cell'], r['target']), {})[r['kind'] if r['kind'] in ('gb_check', 'native') else r['solver'] + r['kind']] = r
    for (cell, target), d in sorted(by.items()):
        g, n = d.get('gb_check'), d.get('native')
        if not g or not n:
            continue
        item = dict(cell=cell, target=target, m=Study.split_cell(cell)[1], gb_order=g['order'], native_order=n['order'], equal=None, reason=None)
        # a = solve.out of the gb row, b = native.gb.txt of the native row
        item['gb_complete'], item['native_complete'] = g['complete'], n['complete']
        if not g['complete'] or not n['complete']:
            item['reason'] = f"not evaluable: gb row {g['class']}, native row {n['class']}"
        else:
            gfile = g.get('_solve_out')
            nfile = n.get('native_gb')
            if not gfile or not nfile:
                item['reason'] = 'not evaluable: solve.out or native.gb.txt missing from the unpacked archive'
            else:
                out = mdir / f"{cell}_{target}.compare.json"
                rc, msg = run_instrument([instr, 'compare-gb', '--a', gfile, '--b', nfile, '--out', str(out)])
                dd = jload(out)
                if dd is None:
                    item['reason'] = f'compare-gb failed rc={rc}: {msg[:200]}'
                else:
                    item.update(equal=dd.get('equal'), a_elements=dd.get('a_elements'), b_elements=dd.get('b_elements'),
                                only_in_a=dd.get('only_in_a'), only_in_b=dd.get('only_in_b'), file=str(out))
        cmp_.append(item)
    return chk, cmp_

# ----------------------------------------------------------------------------- controls
def status_of(vals):
    if any(v is False for v in vals):
        return 'fail'
    if any(v is True for v in vals):
        return 'pass'
    return 'not evaluable'

def compute_controls(recs, chk, cmp_, canary, src_primary):
    G = {}
    def put(k, name, status, detail, expected=''):
        G[k] = dict(id=k, name=name, status=status, detail=detail, required=k in REQUIRED_GATES, expected=expected)
    put('exporter_a', 'exporter gate (a): planted vanishing', 'not evaluable',
        'pre-run gate (cargo test and per cell); no post-run data to evaluate here', 'zero violations')
    put('exporter_b', 'exporter gate (b): L1 replication', 'not evaluable',
        'pre-run gate (committed A3 log comparison); no post-run data to evaluate here', 'identical row structure')
    # (c) variety size from check
    vals, det = [], []
    for r in recs:
        if r['solver'] == 'msolve' and r['mode'] == 'param' and r['complete']:
            c = chk.get(r['rid'])
            if not c or 'error' in c:
                det.append(f"order {r['order']}: check unavailable")
                continue
            ok = (c.get('consistent_with_classifier') is True and (c.get('variety_gate') or {}).get('msolve_ge_native') is True
                  and c.get('all_real_decompositions_recovered') in (True, None))
            vals.append(ok)
            if not ok:
                det.append(f"order {r['order']} {r['cell']}/{r['target']}: consistent={c.get('consistent_with_classifier')}, "
                           f"msolve>=native={(c.get('variety_gate') or {}).get('msolve_ge_native')}, recovered={c.get('all_real_decompositions_recovered')}")
    put('exporter_c', 'exporter gate (c): variety size / consistency', status_of(vals),
        f'{sum(vals)} of {len(vals)} checked completed param rows ok' + ('; ' + '; '.join(det) if det else ''), 'msolve count >= native, real decompositions recovered')
    # second instrument
    ev = [c for c in cmp_ if c['equal'] is not None]
    put('second_instrument', 'second instrument: native f4 vs msolve -g 2', status_of([c['equal'] for c in ev]),
        f"{sum(1 for c in ev if c['equal'])} of {len(ev)} evaluable pairs equal; {len(cmp_) - len(ev)} pair(s) not evaluable",
        'equal reduced bases')
    # positive (t = 2 planted, B = 4)
    pl = [r for r in recs if r['kind'] == 'planted']
    pv = []
    det = []
    for r in pl:
        c = chk.get(r['rid'])
        v = None
        if r['complete'] and c and 'error' not in c:
            v = c.get('positive_control_planted_recovered')
        pv.append((r, v))
    t2 = [v for r, v in pv if r['B'] == 4]
    put('positive', 'positive: planted decomposition recovered (t = 2 cells)', status_of(t2),
        '; '.join(f"{r['cell']}: {'recovered' if v else ('NOT recovered' if v is False else ('not evaluable (' + r['class'] + ')'))}" for r, v in pv) or 'no planted row recorded',
        'planted decomposition among extracted real decompositions')
    # negative: natural unsat
    nv = []
    for r in recs:
        if r['kind'] in ('natural', 'probe', 'aa') and r['label'] == 'unsat' and r['complete']:
            c = chk.get(r['rid'])
            if c and 'error' not in c:
                nv.append(c.get('negative_control_ok'))
            elif r.get('real_dec') is not None:
                nv.append(r['real_dec'] == 0)
    put('negative', 'negative: no real decomposition on natural UNSAT targets', status_of(nv),
        f"{sum(1 for v in nv if v)} of {len(nv)} completed UNSAT rows ok", 'zero real decompositions')
    # A/A
    aa = [r for r in recs if r['kind'] == 'aa' and not r['replacement']]
    aa_c = [r for r in aa if r['complete'] and r['cpu_s']]
    aa_extra = [r for r in recs if r['kind'] == 'natural' and aa and (r['cell'], r['target']) == (aa[0]['cell'], aa[0]['target'])]
    rho = None
    proxy_eq = None
    if len(aa_c) >= 2:
        cs = [r['cpu_s'] for r in aa_c]
        rho = max(cs) / min(cs)
        ps = [r['work_proxy'] for r in aa_c]
        proxy_eq = None if any(p is None for p in ps) else all(abs(p - ps[0]) <= 1e-9 * max(1.0, abs(ps[0])) for p in ps)
    extra = ''
    allsame = [r for r in aa_c + [x for x in aa_extra if x['complete'] and x['cpu_s']]]
    if len(allsame) > len(aa_c):
        cs = [r['cpu_s'] for r in allsame]
        extra = f"; incl. the natural row of the same target: max/min cpu_s = {max(cs) / min(cs):.4f} over {len(cs)} runs, proxy {'identical' if len({r['work_proxy'] for r in allsame}) == 1 else 'DIFFERS'}"
    put('aa', 'A/A: t2r9 SAT 1 repeats', ('not evaluable' if proxy_eq is None else ('pass' if proxy_eq else 'fail')),
        (f"rho_AA = {fnum(rho, 4)} (max/min cpu_s over {len(aa_c)} aa rows); work_proxy {'identical' if proxy_eq else ('DIFFERS' if proxy_eq is False else 'n/a')}" if rho else
         f'fewer than 2 completed A/A rows ({len(aa_c)} of {len(aa)} recorded)') + extra, 'work_proxy identical; rho_AA recorded')
    G['aa']['rho_aa'] = rho
    # canary
    ev = [c for c in canary['rows'] if c['ratio'] is not None]
    flagged = [c['order'] for c in ev if c['ratio'] > DRIFT]
    put('canary', 'canary drift (up_to_degree_8 cpu vs first canary)', 'fail' if flagged else ('pass' if ev else 'not evaluable'),
        f"{len(ev)} of {len(canary['rows'])} canaries comparable; flagged (> {DRIFT}x): {flagged or 'none'}; "
        f"{sum(1 for c in ev if c['ratio'] < 1 / DRIFT)} faster than 1/{DRIFT}x", 'within 1.5x of the first canary')
    tf = [r for r in recs if r['kind'] == 'timer_floor']
    put('timer_floor', 'timer floor', 'pass' if tf and tf[0]['cpu_s'] is not None else 'not evaluable',
        (f"recorded: cpu {fnum(tf[0]['cpu_s'])} s, wall {fnum(tf[0]['wall_s'], 3)} s" if tf else 'no timer-floor row recorded'), 'recorded')
    done = src_primary.get('done')
    put('blinding', 'blinding: analysis after done.json', 'pass' if done else 'not evaluable',
        'done.json present' if done else 'no done.json: analysis ran on partial results', 'analyst reads runs.jsonl after done.json')
    return G

def compute_canary(recs):
    cs = sorted([r for r in recs if r['kind'] == 'canary'], key=lambda r: r['order'])
    res = dict(rows=[], first=None)
    if not cs:
        return res
    first = cs[0]
    res['first'] = first['order']
    f8 = first.get('u8') or {}
    for c in cs:
        u = c.get('u8') or {}
        ratio, why = None, None
        if not f8 or not num(f8.get('cpu_s')) or f8.get('cpu_s') <= 0:
            why = 'first canary has no degree-8 cpu time'
        elif not u or num(u.get('cpu_s')) is None:
            why = 'no log table'
        elif abs(u.get('entries', -1) - f8.get('entries', -2)) > 1e-9 * max(1.0, f8.get('entries', 1)):
            why = 'different work covered up to degree 8 (cut before the same rounds)'
        else:
            ratio = u['cpu_s'] / f8['cpu_s']
        res['rows'].append(dict(order=c['order'], ratio=ratio, why=why, cpu=u.get('cpu_s')))
    byorder = {c['order']: c for c in res['rows']}
    for r in recs:
        if r['kind'] in ('natural', 'planted', 'probe') and (r['B'] or 0) >= 8:
            prev = [o for o in byorder if o < r['order']]
            if prev:
                c = byorder[max(prev)]
                r['canary_order'], r['canary_ratio'] = c['order'], c['ratio']
                r['drift'] = None if c['ratio'] is None else c['ratio'] > DRIFT
    return res

# ----------------------------------------------------------------------------- figure
OKABE = ['#0072B2', '#D55E00', '#009E73', '#CC79A7', '#E69F00', '#56B4E9', '#F0E442']

def make_figure(recs, outdir, notes):
    try:
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
    except ImportError:
        notes.append('matplotlib not installed: figure skipped')
        return None
    matplotlib.rcParams['svg.fonttype'] = 'none'
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.8))
    cells = sorted({r['base_cell'] for r in recs if r['kind'] in ('natural', 'probe', 'aa', 'planted') and r['B']})
    col = {c: OKABE[i % len(OKABE)] for i, c in enumerate(cells)}
    offs = {c: (i - (len(cells) - 1) / 2) * 0.045 for i, c in enumerate(cells)}
    muted = '#9a9a9a'
    for ax, (obs, title) in zip(axes, [('cpu_s', 'cpu_s (msolve + verify), s'), ('work_proxy', 'work proxy (sum rows x cols x density)')]):
        anchor = None
        pts = []
        for r in recs:
            if r['kind'] not in ('natural', 'probe', 'aa', 'planted') or not r['B']:
                continue
            y = r['v'][obs]
            if y is None or y <= 0:
                continue
            x = math.log2(r['B']) + offs.get(r['base_cell'], 0)
            nat = r['kind'] in ('natural', 'probe')
            mk = 's' if r['label'] == 'unsat' else 'o'
            if r['kind'] == 'planted':
                mk = 'D'
            if r['kind'] == 'aa':
                mk = 'P'
            c = col[r['base_cell']] if nat else muted
            if not r['complete']:
                ax.scatter([x], [y], marker='^', s=46, color=c, zorder=3, edgecolors='k', linewidths=0.4)
                ax.annotate('', xy=(x, y * 1.9), xytext=(x, y * 1.06), arrowprops=dict(arrowstyle='->', color=c, lw=1.2), zorder=3)
            else:
                ax.scatter([x], [y], marker=mk, s=34, color=c, zorder=3, edgecolors='k' if nat else 'none', linewidths=0.4, alpha=1.0 if nat else 0.8)
            pts.append((x, y, r))
            if r['kind'] == 'natural' and r['B'] == 4 and r['complete']:
                anchor = max(anchor, (y, x), key=lambda t: t[0]) if anchor else (y, x)
        for c in cells:
            xs = [p for p in pts if p[2]['base_cell'] == c and p[2]['kind'] in ('natural', 'probe')]
            if xs:
                px, py, _ = max(xs, key=lambda p: p[1])
                ax.annotate(c, (px, py), textcoords='offset points', xytext=(4, 6), fontsize=7, color=col[c])
        if anchor:
            y0, x0 = anchor
            xx = [2.0, 4.0]
            ax.plot(xx, [y0 * 2 ** (3.5 * (x - 2.0)) for x in xx], ls='--', color='#444444', lw=1, zorder=1)
            ax.annotate('slope 2^3.5 per doubling of B', (3.0, y0 * 2 ** 3.5), textcoords='offset points', xytext=(-6, 8), fontsize=7.5, color='#444444', ha='right')
        ax.set_yscale('log')
        ax.set_xlim(1.6, 4.5)
        ax.set_xticks([2, 3, 4])
        ax.set_xticklabels(['4', '8', '16'])
        ax.set_xlabel('B = 2^t')
        ax.set_title(title, fontsize=10)
        ax.grid(True, which='major', alpha=0.3)
        for s in ('top', 'right'):
            ax.spines[s].set_visible(False)
        if not pts:
            ax.text(0.5, 0.5, 'no data', transform=ax.transAxes, ha='center', color='#777')
    from matplotlib.lines import Line2D
    leg = [Line2D([], [], marker='o', ls='', color='#333', label='SAT (natural)'), Line2D([], [], marker='s', ls='', color='#333', label='UNSAT (natural)'),
           Line2D([], [], marker='^', ls='', color='#333', label='censored: lower bound (arrow)'),
           Line2D([], [], marker='D', ls='', color=muted, label='planted (control)'), Line2D([], [], marker='P', ls='', color=muted, label='A/A repeat'),
           Line2D([], [], ls='--', color='#444444', label='2^3.5 gate slope')]
    fig.legend(handles=leg, loc='lower center', ncol=6, fontsize=7.5, frameon=False)
    fig.suptitle('PKM m=8 panel A: per-call work vs B (one point per call; censored = lower bound)   ' + PCC, fontsize=10)
    fig.tight_layout(rect=(0, 0.07, 1, 0.95))
    fd = outdir / 'figures'
    fd.mkdir(exist_ok=True)
    fig.savefig(fd / 'work_vs_B.svg')
    fig.savefig(fd / 'work_vs_B.png', dpi=150)
    plt.close(fig)
    return str(fd / 'work_vs_B.svg')

# ----------------------------------------------------------------------------- report writers
def row_cells(r):
    vj = r.get('verify_json') or {}
    chk = r.get('_check') or {}
    vg = chk.get('variety_gate') or {}
    drift = ''
    if r.get('canary_ratio') is not None:
        drift = f"{r['canary_ratio']:.3f}{' DRIFT' if r.get('drift') else ''}"
    elif r.get('canary_order') is not None:
        drift = 'n/e'
    if r['kind'] == 'canary':
        cr = r.get('_canary_self')
        drift = '' if cr is None else f'{cr:.3f}'
    host = (r['host'] or '')[:8] + ('/R' if r['replacement'] else '')
    return [r['order'], r['kind'], r['cell'], r['B'] or '', r['target'], r['label'] or '', r['algebraic_count'] if r['algebraic_count'] is not None else '',
            r['mode'] or '', r['threads'] or '', fnum(r['cpu_s']), fnum(r['wall_s']), fnum(r['max_rss_gb'], 3), r['class'] if r['class'] != 'complete' else 'ok',
            fproxy(r['work_proxy']), r['max_degree'] if r['max_degree'] is not None else '', r['rounds'] if r['rounds'] is not None else '',
            fnum(r['verify_cpu_s']), vg.get('msolve_distinct_solutions', ''), r['fp_sols'] if r['fp_sols'] is not None else '',
            r['real_dec'] if r['real_dec'] is not None else '', r['useful'] if r['useful'] is not None else '',
            ('' if chk.get('consistent_with_classifier') is None else chk['consistent_with_classifier']), drift, host]

TABLE_A_HEAD = ['order', 'kind', 'cell', 'B', 'target', 'label', 'alg_count', 'mode', 'thr', 'cpu_s', 'wall_s', 'rss_GB', 'censored', 'work_proxy',
                'max_deg', 'rounds', 'verify_cpu_s', 'alg_sols', 'fp_sols', 'real_dec', 'useful', 'consistent', 'canary_drift', 'host']

def cell_table(recs, planned_by_cell):
    rows = []
    for c in sorted({r['cell'] for r in recs if r['kind'] in ('natural', 'probe')}):
        rs = [r for r in recs if r['cell'] == c and r['kind'] in ('natural', 'probe')]
        comp = [r for r in rs if r['complete']]
        cen = [r for r in rs if not r['complete']]
        cv = [r['c'] for r in comp if r['c'] is not None]
        pv = [r['work_proxy'] for r in comp if r['work_proxy'] is not None]
        rows.append([c, (rs[0]['B'] or ''), planned_by_cell.get(c, ''), len(rs), len(comp), len(cen),
                     fnum(min(cv)) if cv else '', fnum(max(cv)) if cv else '', fproxy(min(pv)) if pv else '', fproxy(max(pv)) if pv else '',
                     fnum(min(r['c'] for r in cen if r['c'] is not None)) if any(r['c'] is not None for r in cen) else '',
                     fproxy(min(r['work_proxy'] for r in cen if r['work_proxy'] is not None)) if any(r['work_proxy'] is not None for r in cen) else ''])
    return rows

TABLE_B_HEAD = ['cell', 'B', 'planned', 'recorded', 'n complete', 'n censored', 'min c (s)', 'max c (s)', 'min proxy', 'max proxy',
                'min c_lo of censored (s)', 'min proxy_lo of censored']

def gamma_rows(G):
    rows = []
    for label, g in G:
        def dist(x):
            return '' if x is None or math.isinf(x) else fnum(GATE - x)
        rows.append([label, fnum(g['gamma_lo']) if g['gamma_lo'] is not None else 'undefined', dist(g['gamma_lo']),
                     fnum(g['gamma_hi']), dist(g['gamma_hi']), fnum(g['gamma_lo_8_16']) if g['gamma_lo_8_16'] is not None else 'undefined',
                     f"{g['n_t2_complete']}/{g['n_t2_planned']}", f"{g['n_t3_complete']} complete, {g['n_t3_censored']} censored, {len(g['t3_missing'])} not run of {g['n_t3_planned']}",
                     g['gamma_lo_reason'] or ''])
    return rows

TABLE_C_HEAD = ['observable / reading', 'gamma_lo(4->8)', f'{GATE} - gamma_lo', 'gamma_hi(4->8)', f'{GATE} - gamma_hi', 'gamma_lo(8->16)', 't=2 natural complete', 't=3 natural', 'note']

def tq(s):
    """Typst string-literal expression for arbitrary text."""
    return '#"' + str(s).replace('\\', '\\\\').replace('"', '\\"') + '"'

def typst_table(head, rows, widths=None):
    n = len(head)
    out = [f'#table(columns: {n}, inset: 3pt, stroke: 0.4pt + luma(170), align: left,']
    out.append('  ' + ', '.join(f'[*{tq(h)}*]' for h in head) + ',')
    for r in rows:
        out.append('  ' + ', '.join(f'[{tq(c)}]' for c in r) + ',')
    out.append(')')
    return '\n'.join(out)

# ----------------------------------------------------------------------------- main
def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--results', required=True, help='results/<stamp> directory fetched by launch.sh')
    ap.add_argument('--results-replacement', help='directory of the replacement pod (rows tagged replacement)')
    ap.add_argument('--out', required=True, help='output directory (created; raw files are never touched)')
    ap.add_argument('--study', default=str(STUDY), help='study directory holding PROTOCOL.md, FREEZE.json, pod/, cells/')
    ap.add_argument('--plan', help='plan.json to join against (default: out/env/plan.json from the archive, else study/pod/plan.json)')
    ap.add_argument('--instrument', help='pkm_m8_panel binary (default: repo target/release/examples/pkm_m8_panel)')
    ap.add_argument('--rho', action='append', help='rho.json file(s) (default: search results dir and cells/*/rho.json)')
    ap.add_argument('--no-instrument', action='store_true', help='skip check / compare-gb')
    ap.add_argument('--no-figure', action='store_true')
    ap.add_argument('--no-pdf', action='store_true')
    a = ap.parse_args(argv)

    study = Study(a.study)
    out = Path(a.out)
    res = Path(a.results)
    try:
        if out.resolve() == res.resolve() or res.resolve() in [out.resolve()] or (out.resolve() / 'results.tgz').exists():
            sys.exit('refusing: --out must be a new directory separate from the raw results')
    except OSError:
        pass
    out.mkdir(parents=True, exist_ok=True)
    notes = []
    prim = load_source(res, out, 'primary')
    srcs = [prim]
    if a.results_replacement:
        srcs.append(load_source(a.results_replacement, out, 'replacement'))
    for s in srcs:
        notes += s['notes']

    # ---- plan
    plan_path = None
    if a.plan:
        plan_path = Path(a.plan)
    elif prim.get('unpack') and (prim['unpack'] / 'env' / 'plan.json').is_file():
        plan_path = prim['unpack'] / 'env' / 'plan.json'
    elif (Path(a.study) / 'pod' / 'plan.json').is_file():
        plan_path = Path(a.study) / 'pod' / 'plan.json'
    plan = jload(plan_path, {}) if plan_path else {}
    plan_rows = plan.get('rows', [])
    if not plan_rows:
        notes.append('no plan.json found: lost / not-run rows cannot be listed and planned sets are empty')
    use_repl = bool(a.results_replacement)
    plan_rows = [r for r in plan_rows if use_repl or not r.get('on_replacement')]
    plan_by_order = {r['order']: r for r in plan_rows}
    plan_sha = sha256_file(plan_path) if plan_path else None
    frz = (study.freeze.get('sha256') or {})

    # ---- merge rows
    merged, dups = {}, []
    for s in srcs:
        for run in s['runs']:
            try:
                o = int(run['order'])
            except (KeyError, TypeError, ValueError):
                notes.append(f'{s["tag"]}: runs.jsonl line without an order skipped')
                continue
            if s['tag'] != 'primary':
                run['replacement'] = True
            rec = build_record(run, plan_by_order.get(o), s, study)
            if o in merged:
                dups.append(rec)
            else:
                merged[o] = rec
            # remember where solve.out lives for compare-gb
            rec['_solve_out'] = str(find_file(s, row_id(run), 'solve.out') or '') or None
    recs = [merged[o] for o in sorted(merged)]
    if dups:
        notes.append(f'{len(dups)} duplicate order(s) in the replacement directory kept as evidence only: ' + ', '.join(str(d['order']) for d in dups))
    for r in recs:
        if r['plan_mismatch']:
            notes.append(f"order {r['order']}: runs.jsonl row ({r['kind']},{r['cell']},{r['target']}) differs from the plan row at that order")
    unplanned = [r['order'] for r in recs if plan_rows and r['order'] not in plan_by_order]
    if unplanned:
        notes.append(f'rows with no plan entry: {unplanned}')

    # ---- instrument checks
    instr = None if a.no_instrument else (a.instrument or str(STUDY.parent.parent / 'target' / 'release' / 'examples' / 'pkm_m8_panel'))
    if a.no_instrument:
        notes.append('instrument checks skipped (--no-instrument): check / compare-gb not evaluated')
    chk, cmp_ = do_checks(recs, study, instr, out, notes) if instr else ({}, [])
    for r in recs:
        r['_check'] = chk.get(r['rid'])
    # ---- canary
    canary = compute_canary(recs)
    for r in recs:
        if r['kind'] == 'canary':
            for c in canary['rows']:
                if c['order'] == r['order']:
                    r['_canary_self'] = c['ratio']
    gates = compute_controls(recs, chk, cmp_, canary, prim)
    rho_aa = gates['aa'].get('rho_aa')

    # ---- gamma
    planned2 = [o for o, p in plan_by_order.items() if p['kind'] == 'natural' and (study.B(p['cell']) == 4)]
    planned3 = [o for o, p in plan_by_order.items() if p['kind'] == 'natural' and (study.B(p['cell']) == 8)]
    nat2 = {r['order']: r for r in recs if r['kind'] == 'natural' and r['B'] == 4}
    nat3 = {r['order']: r for r in recs if r['kind'] == 'natural' and r['B'] == 8}
    # a recorded natural row at a B outside the plan is still analysed
    planned2 = sorted(set(planned2) | set(o for o in nat2 if plan_rows and o not in plan_by_order))
    probes = [r for r in recs if r['kind'] == 'probe']
    probe = probes[0] if probes else None
    t3_completed = any(r['complete'] for r in nat3.values())
    if probe and not t3_completed:
        notes.append('t4r7 probe ran but no natural t = 3 row completed: probe is listed, not analysed (protocol 9a item 9)')
        probe = None

    def blk(obs, key):
        n2 = {o: dict(r, **{'_v': r['v'][obs] if key == 'v' else r['c_B']}) for o, r in nat2.items()}
        n3 = {o: dict(r, **{'_v': r['v'][obs] if key == 'v' else r['c_B']}) for o, r in nat3.items()}
        pr = dict(probe, _v=(probe['v'][obs] if key == 'v' else probe['c_B'])) if probe else None
        return gamma_block(n2, n3, pr, planned2, planned3, key='_v')
    g_cpu, g_prox = blk('cpu_s', 'v'), blk('work_proxy', 'v')
    setup_known = all(study.setup_cpu_s(c) is not None for c in {r['cell'] for r in nat2.values()} | {r['cell'] for r in nat3.values()}) and bool(nat2 or nat3)
    g_cpuB = blk('cpu_s', 'B') if setup_known else None
    positives_t3 = sum(1 for r in nat3.values() if r['complete'] and r['label'] == 'sat' and (r.get('_check') or {}).get('consistent_with_classifier') is True
                       and (r.get('_check') or {}).get('all_real_decompositions_recovered') is True and (r.get('real_dec') or 0) > 0)
    readingB = None
    if g_cpuB is None:
        readingB = 'not evaluable: cell.json records no base-construction CPU cost'
    else:
        vB, _ = decide_observable('cpu_s', g_cpuB, rho_aa, gates, None, positives_t3)
        readingB = 'pass' if vB.startswith('NOT KILLED') else f'does not hold ({vB})'
    v_cpu = decide_observable('cpu_s', g_cpu, rho_aa, gates, readingB, positives_t3)
    v_prox = decide_observable('work_proxy', g_prox, rho_aa, gates, None, positives_t3)
    pv = panel_verdict(v_cpu, v_prox)
    v_cpu_A = decide_observable('cpu_s', g_cpu, rho_aa, gates, None, positives_t3)

    # ---- rho
    rho_files = [Path(p) for p in (a.rho or [])]
    if not a.rho:
        for pat in (res.glob('rho.json'), res.glob('rho-*.json'), res.glob('rho/*.json'), study.cells_dir.glob('*/rho.json')):
            rho_files += sorted(pat)
    rho_rows = []
    for f in rho_files:
        d = jload(f)
        if not d:
            continue
        cell = next((c.parent.name for c in study.cells_dir.glob('*/cell.json') if (jload(c, {}) or {}).get('sha256') == d.get('cell_sha256')), f.parent.name)
        runs = d.get('runs') or []
        adds = [x.get('walk_steps_group_additions') for x in runs if num(x.get('walk_steps_group_additions')) is not None]
        cpu = [x.get('cpu_secs') for x in runs if num(x.get('cpu_secs')) is not None]
        rho_rows.append([cell, d.get('bits', ''), d.get('r', ''), fnum(d.get('mean_group_additions')), ', '.join(str(x) for x in adds),
                         fnum(sum(cpu) / len(cpu), 4) if cpu else '', all(x.get('solved') for x in runs) if runs else '', str(f)])

    # ---- lost / not-run
    have = {r['order'] for r in recs}
    notrun = [p for o, p in sorted(plan_by_order.items()) if o not in have]
    inflight = []
    for s in srcs:
        for base in ([s['unpack'] / 'rows'] if s.get('unpack') else []) + [Path(s['dir']) / 'rows_meta']:
            if base.is_dir():
                for d in sorted(base.iterdir()):
                    m = re.match(r'(\d+)_', d.name)
                    if m and int(m.group(1)) not in have and d.name not in inflight:
                        inflight.append(d.name)

    # ---- integrity facts
    integ = []
    if prim.get('done'):
        d = prim['done']
        for k, fk in (('plan_sha256', 'pod/plan.json'), ('job_sh_sha256', 'pod/job.sh')):
            if d.get(k) and frz.get(fk):
                integ.append(f'{k} {"matches" if d[k] == frz[fk] else "DIFFERS from"} FREEZE.json {fk}')
        if d.get('msolve_sha256'):
            integ.append(f'msolve sha256 {"matches" if d["msolve_sha256"] == d.get("msolve_sha_expected") else "DIFFERS from"} the plan pin')
        integ.append(f'pod_id {d.get("pod_id")}, dry_run={d.get("dry_run")}, rows {d.get("rows_recorded")}/{d.get("rows_planned")}, by_status {d.get("by_status")}')
    if plan_sha and frz.get('pod/plan.json'):
        integ.append(f'plan used for the join ({plan_path}) sha256 {"matches" if plan_sha == frz["pod/plan.json"] else "DIFFERS from"} FREEZE.json pod/plan.json')

    # ---- figure
    fig = None if a.no_figure else make_figure(recs, out, notes)

    # ---- machine-readable
    def strip(r):
        return {k: v for k, v in r.items() if k not in ('verify_json', '_check')}
    analysis = dict(schema='pkm.m8-panel.analysis/v1', standing=PCC, generated_utc=datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
                    freeze_commit=study.freeze.get('commit'), runs_sha256=prim['runs_sha256'], rows=[strip(r) for r in recs],
                    gamma=dict(cpu_s=g_cpu, work_proxy=g_prox, cpu_s_with_setup=g_cpuB), rho_aa=rho_aa,
                    controls=gates, compare_gb=cmp_, canary=canary,
                    verdict=dict(cpu_s=v_cpu[0], cpu_s_reasons=v_cpu[1], work_proxy=v_prox[0], work_proxy_reasons=v_prox[1], panel=pv[0], panel_reasons=pv[1],
                                 reading_b=readingB, positives_t3=positives_t3),
                    notes=notes)
    def jdefault(o):
        return str(o)
    (out / 'analysis.json').write_text(json.dumps(analysis, indent=1, default=jdefault, allow_nan=True))

    # ---- RESULTS.md
    md = []
    A = md.append
    A(f'# RESULTS: PKM m = 8 sparse panel A (PKM-M8-PANELA-20261114)   {PCC}')
    A('')
    A(f'Every number below is produced mechanically by `analysis/build_results.py` from the fetched rows; nothing is filed as a pcc claim. {PCC}')
    A('')
    A(f'- **Freeze commit:** `{study.freeze.get("commit")}` (protocol {study.freeze.get("version")}, frozen {study.freeze.get("frozen_at")})')
    A(f'- **runs.jsonl sha256:** `{prim["runs_sha256"]}` ({len(prim["runs"])} rows)' + (f'; replacement runs.jsonl sha256 `{srcs[1]["runs_sha256"]}` ({len(srcs[1]["runs"])} rows)' if len(srcs) > 1 else ''))
    A(f'- **results.tgz sha256:** `{prim["tgz_sha256"]}`; done.json: {"present" if prim.get("done") else "ABSENT (partial results)"}')
    for i in integ:
        A(f'- {i}')
    A(f'- Rows analysed: {len(recs)} of {len(plan_rows)} planned; not run / no record: {len(notrun)}')
    A('')
    if notes:
        A('**Pipeline notes:**')
        for n in notes:
            A(f'- {n}')
        A('')
    A('## Cost definitions used (PROTOCOL.md section 4)')
    A('')
    A('- `cpu_s` of a call = `user_s + sys_s` of the msolve process from `/usr/bin/time -v` (runs.jsonl fields `user_s`, `sys_s`; `time_v.txt` only as fallback).')
    A('- `c = cpu_s(msolve) + cpu_s(verify)`. Verify CPU: `verify.json` field `cpu_secs` when present, else the wall `verify.wall_s` of runs.jsonl (the row says which in `analysis.json`: `verify_cpu_src`).')
    A('- `work_proxy` = `totals.sum_rows_x_cols_x_density` of the `-v 2` log table (all passes, without the final-reduction line). It has no verify component.')
    A('- Censored rows contribute lower bounds: `c_lo = cpu_s at the stop` (+ verify CPU if it ran) and the partial `work_proxy`. A row whose msolve exited 0 but whose verify is missing, failed or hit its deadline is incomplete (section 4).')
    A('- Reading A (decision reading): solve + verify. Reading B (with setup share): requires a base-construction cost in `cell.json`; '
      + ('recorded for every analysed cell.' if setup_known else '`cell.json` records none, so reading B is not evaluable here (counted field operations of the target setup are in each `verify.json` `work.target_setup`, a different unit).'))
    A('')
    A(f'## Table A: per-call rows   {PCC}')
    A('')
    A(md_table(TABLE_A_HEAD, [row_cells(r) for r in recs]))
    A('')
    A('`alg_sols` = msolve distinct solutions from `check` (variety gate); `fp_sols` = F_p-rational solutions from `verify.json`; `real_dec` / `useful` = real decompositions / useful relations from `verify.json`; `consistent` = `check` `consistent_with_classifier`; `canary_drift` = degree-8 canary cpu relative to the first canary (canary rows show their own ratio), `n/e` = not comparable; `/R` host suffix = replacement pod. Memory in GB = kB x 1024 / 1e9.')
    A('')
    A(f'## Table B: per cell (natural rows and the probe)   {PCC}')
    A('')
    planned_by_cell = {}
    for p in plan_rows:
        if p['kind'] in ('natural', 'probe'):
            planned_by_cell[p['cell']] = planned_by_cell.get(p['cell'], 0) + 1
    A(md_table(TABLE_B_HEAD, cell_table(recs, planned_by_cell)))
    A('')
    A(f'## Table C: gamma secants and A/A   {PCC}')
    A('')
    Gs = [('cpu_s, reading A (solve + verify)', g_cpu), ('work_proxy', g_prox)] + ([('cpu_s, reading B (with setup share)', g_cpuB)] if g_cpuB else [])
    A(md_table(TABLE_C_HEAD, gamma_rows(Gs)))
    A('')
    A(f'- gamma(4->8) is a local secant over one doubling of B containing the degree step and the variable step; never an asymptotic exponent. Distances are `{GATE} - gamma` (positive: gamma below the gate); the one-output gate {GATE_ONE} is reported, not used: `{GATE_ONE} - gamma_lo` = '
      + (fnum(GATE_ONE - g_cpu['gamma_lo']) if g_cpu['gamma_lo'] is not None and not math.isinf(g_cpu['gamma_lo']) else 'n/a') + ' (cpu_s), '
      + (fnum(GATE_ONE - g_prox['gamma_lo']) if g_prox['gamma_lo'] is not None and not math.isinf(g_prox['gamma_lo']) else 'n/a') + ' (work_proxy).')
    for lab, g in Gs:
        A(f'- {lab}: max c(4) = {na(g["max_c4"])}, min c(4) = {na(g["min_c4"])}, min c_lo(8) = {na(g["min_c_lo8"])}, max c(8) over completed = {na(g["max_c8_complete"])}'
          + (f'; gamma_hi including censored lower bounds >= {fnum(g["gamma_hi_lb_incl_censored"])}' if g['gamma_hi_lb_incl_censored'] is not None else '')
          + ('' if g['gamma_lo_8_16'] is not None else f'; gamma_lo(8->16): {g["gamma_lo_8_16_reason"]}'))
    A(f'- A/A: rho_AA = {na(rho_aa, 4)} (max/min cpu_s of the t2r9 SAT 1 repeats); proxy equality: {gates["aa"]["detail"]}; log2(rho_AA) = {fnum(lg2(rho_aa, 1.0)) if rho_aa else "n/a"}.')
    A('')
    A(f'## Table D: controls   {PCC}')
    A('')
    A(md_table(['control', 'status', 'required for verdict', 'expected', 'detail'],
               [[g['name'], g['status'], 'yes' if g['required'] else 'no', g['expected'], g['detail']] for g in gates.values()]))
    A('')
    A('"Required for verdict" = the gates the frozen rule names as satisfiable by a censored t = 3 row: exporter gates (here: (c) and the second instrument), t = 2 planted and negative controls, A/A, canaries. Pre-run gates (a), (b) have no post-run data. A control that cannot complete is not evaluable, never passed or failed.')
    if cmp_:
        A('')
        A('Second instrument, per pair:')
        A('')
        A(md_table(['cell', 'm', 'target', 'gb order', 'native order', 'equal', 'elements gb/native', 'note'],
                   [[c['cell'], c['m'], c['target'], c['gb_order'], c['native_order'], c['equal'] if c['equal'] is not None else 'not evaluable',
                     f"{c.get('a_elements', '')}/{c.get('b_elements', '')}" if c['equal'] is not None else '', c['reason'] or ''] for c in cmp_]))
    A('')
    A(f'## Verdict   {PCC}')
    A('')
    A('Frozen rule as applied (quoted from PROTOCOL.md section 5):')
    A('')
    for ln in study.rule_text().splitlines():
        A('> ' + ln if ln.strip() else '>')
    A('')
    A('Implementation notes: required gates are those of Table D marked yes; a required gate that is not `pass` blocks both KILL and NOT KILLED. '
      'NOT KILLED additionally needs no censored and no unrecorded natural t = 3 row, every natural t = 2 row complete, and at least one t = 3 natural row that is complete, `sat`, `consistent_with_classifier` and has all real decompositions recovered. '
      'KILL needs every planned natural t = 2 row complete and every planned natural t = 3 row recorded (the minimum over a subset is anti-conservative).')
    A('')
    A(md_table(['observable', 'verdict', 'reasons'],
               [['cpu_s (reading A alone)', v_cpu_A[0], '; '.join(v_cpu_A[1])], ['cpu_s (both readings; B: ' + readingB + ')', v_cpu[0], '; '.join(v_cpu[1])],
                ['work_proxy', v_prox[0], '; '.join(v_prox[1])]]))
    A('')
    A(f'### PANEL VERDICT: **{pv[0]}**   {PCC}')
    if pv[1]:
        A('')
        A('Reasons: ' + '; '.join(pv[1]))
    A('')
    A(f'## Rho reference rows   {PCC}')
    A('')
    if rho_rows:
        A(md_table(['cell', 'bits', 'r', 'mean group additions', 'per-seed group additions', 'mean cpu_s', 'all solved', 'file'], rho_rows))
        A('')
        A('Reference rows only: msolve CPU seconds are not a counted native operation and cannot be priced at 0 / 0.1 / 1 GAE (PROTOCOL.md section 7); any comparison against these rows treats msolve as an unpriced lower bound and no cross-method ranking is made.')
    else:
        A('No `rho.json` found (searched the results directory and `cells/*/rho.json`).')
    A('')
    A('## Lost rows / not run')
    A('')
    if notrun:
        A(md_table(['order', 'kind', 'cell', 'target', 'status'], [[p['order'], p['kind'], p['cell'], p['target'], 'planned, no record in runs.jsonl'] for p in notrun]))
    else:
        A('None: every planned row has a record.')
    if inflight:
        A('')
        A('Row directories with a partial log / meta but no runs.jsonl record (row in flight when the pod was lost; evidence only): ' + ', '.join(inflight))
    cens = [r for r in recs if r['class'] not in ('complete',) and r['kind'] in ('natural', 'probe', 'aa', 'planted')]
    if cens:
        A('')
        A('Censored / incomplete analysed-kind rows: ' + ', '.join(f"{r['order']} ({r['cell']}/{r['target']}: {r['class']})" for r in cens))
    A('')
    if fig:
        A('## Figure')
        A('')
        A('![per-call work vs B](figures/work_vs_B.svg)')
        A('')
    (out / 'RESULTS.md').write_text('\n'.join(md) + '\n')

    # ---- REPORT.typ
    ty = ['#set page(margin: 1.6cm, numbering: "1")', '#set text(size: 9.5pt)', '#set par(justify: false)',
          f'= PKM m = 8 sparse panel A: results {tq(PCC)}', '',
          tq(f'Freeze commit {study.freeze.get("commit")}; runs.jsonl sha256 {prim["runs_sha256"]}; {len(recs)} of {len(plan_rows)} planned rows analysed; done.json {"present" if prim.get("done") else "ABSENT (partial results)"}.'), '',
          f'== Panel verdict {tq(PCC)}', '', f'#text(weight: "bold")[{tq(pv[0])}]', '']
    if pv[1]:
        ty += [tq('Reasons: ' + '; '.join(pv[1])), '']
    ty += [typst_table(['observable', 'verdict', 'reasons'], [['cpu_s (reading A)', v_cpu_A[0], '; '.join(v_cpu_A[1])], ['cpu_s (both readings)', v_cpu[0], '; '.join(v_cpu[1])], ['work_proxy', v_prox[0], '; '.join(v_prox[1])]]), '']
    ty += ['== Frozen rule applied', '']
    ty += [tq(' '.join(study.rule_text().split())), '']
    ty += [f'== gamma secants {tq(PCC)}', '', typst_table(TABLE_C_HEAD, gamma_rows(Gs)), '',
           tq(f'rho_AA = {fnum(rho_aa, 4)}. Distances are {GATE} - gamma; gamma(4->8) is a local secant, never an asymptotic exponent.'), '',
           f'== Controls {tq(PCC)}', '', typst_table(['control', 'status', 'detail'], [[g['name'], g['status'], g['detail']] for g in gates.values()]), '']
    if fig:
        ty += ['== Figure', '', '#figure(image("figures/work_vs_B.svg", width: 100%), caption: [Per-call work against B; censored calls are lower bounds.])', '']
    ty += [f'== Per-cell summary {tq(PCC)}', '', typst_table(TABLE_B_HEAD, cell_table(recs, planned_by_cell)), '']
    ty += [f'== Natural, planted, A/A and probe rows {tq(PCC)}', '',
           typst_table(['order', 'kind', 'cell', 'target', 'label', 'cpu_s', 'work_proxy', 'censored', 'real_dec', 'useful'],
                       [[r['order'], r['kind'], r['cell'], r['target'], r['label'] or '', fnum(r['cpu_s']), fproxy(r['work_proxy']), r['class'] if r['class'] != 'complete' else 'ok',
                         r['real_dec'] if r['real_dec'] is not None else '', r['useful'] if r['useful'] is not None else ''] for r in recs if r['kind'] in ('natural', 'planted', 'aa', 'probe')]), '']
    ty += ['== Lost rows / not run', '', tq(f'{len(notrun)} planned row(s) without a record' + (': ' + ', '.join(str(p['order']) for p in notrun[:40]) if notrun else '.')), '',
           tq('Full tables: RESULTS.md. All numbers ' + PCC + '.')]
    (out / 'REPORT.typ').write_text('\n'.join(ty) + '\n')
    pdf_msg = 'REPORT.pdf: skipped (--no-pdf)'
    if not a.no_pdf:
        exe = shutil.which('typst')
        if exe:
            try:
                p = subprocess.run([exe, 'compile', 'REPORT.typ', 'REPORT.pdf'], cwd=out, capture_output=True, text=True, timeout=300)
                pdf_msg = 'REPORT.pdf written' if p.returncode == 0 else f'typst compile failed (rc {p.returncode}): {(p.stderr or p.stdout)[:300]}'
            except (OSError, subprocess.TimeoutExpired) as e:
                pdf_msg = f'typst compile failed: {e}'
        else:
            pdf_msg = 'typst not on PATH: REPORT.typ left uncompiled (no REPORT.pdf)'

    # ---- one-screen summary
    cls = {}
    for r in recs:
        cls[r['class']] = cls.get(r['class'], 0) + 1
    print(f'== PKM m=8 panel A analysis {PCC}')
    print(f'results {res}  freeze {str(study.freeze.get("commit"))[:12]}  runs.jsonl sha256 {str(prim["runs_sha256"])[:16]}')
    print(f'rows {len(recs)}/{len(plan_rows)} planned, not run {len(notrun)}, classes {cls}, done.json {"yes" if prim.get("done") else "NO (partial)"}')
    for lab, g in Gs:
        print(f'  {lab:38s} gamma_lo(4->8)={fnum(g["gamma_lo"]) if g["gamma_lo"] is not None else "undefined":>9} gamma_hi={fnum(g["gamma_hi"]):>6}  '
              f't2 {g["n_t2_complete"]}/{g["n_t2_planned"]} t3 {g["n_t3_complete"]}c/{g["n_t3_censored"]}x/{len(g["t3_missing"])}miss')
    print('  controls: ' + ', '.join(f'{k}={v["status"]}' for k, v in gates.items()))
    print(f'  rho_AA={fnum(rho_aa, 4) if rho_aa else "n/a"}')
    print(f'  verdict cpu_s: {v_cpu[0]}   work_proxy: {v_prox[0]}')
    print(f'PANEL VERDICT: {pv[0]}')
    for w in pv[1][:6]:
        print(f'  - {w}')
    print(f'wrote {out}/RESULTS.md, analysis.json' + (', figures/work_vs_B.svg|png' if fig else '') + f', REPORT.typ; {pdf_msg}')
    return 0

if __name__ == '__main__':
    sys.exit(main())
