"""Shared helpers for SWB-RECOMPUTE-01 row scripts (python3 -I, reads committed files only)."""
import json
import os
import re
import subprocess
from decimal import Decimal, ROUND_HALF_UP

REPO = '/home/user/crypto'
TEXT_EXT = ('.md', '.typ', '.txt', '.yaml', '.yml', '.py', '.sh', '.log', '.csv', '.tex')

OFF_RE = re.compile(r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-/]+|Modal|"
                    r"not committed|scratch|remain local|\bS3\b|Google Drive")
AA_RE = re.compile(r"A/A|[Nn]ull[ -]control|[Nn]egative control|[Pp]ositive control|\bcontrol\b|\bnull\b")
ISO_RE = re.compile(r"[Ii]solat|[Cc]ontended|\bL[0-3]\b|\bPMU\b|pinned|reserved CPU|[Uu]ncontended")


def abspath(rel):
    return os.path.join(REPO, rel)


def rel(p):
    return os.path.relpath(p, REPO)


def stop(msg):
    raise SystemExit('STOP: ' + msg)


def study_files(dirs):
    out = []
    for d in dirs:
        p = abspath(d)
        if os.path.isfile(p):
            out.append(p)
            continue
        if not os.path.isdir(p):
            stop('study directory does not exist: ' + d)
        for root, _, files in os.walk(p):
            for f in files:
                if f.endswith(TEXT_EXT):
                    out.append(os.path.join(root, f))
    return sorted(out)


def grep(files, rx):
    hits = []
    for f in files:
        try:
            lines = open(f, encoding='utf-8', errors='replace').read().splitlines()
        except Exception as e:  # unreadable file: stop per card
            stop('unreadable file %s: %s' % (rel(f), e))
        for i, line in enumerate(lines, 1):
            if rx.search(line):
                hits.append('%s:%d: %s' % (rel(f), i, line.strip()))
    return hits


def store_refs(files):
    out = []
    try:
        idx = open(abspath('store/index.json'), encoding='utf-8').read()
    except Exception:
        idx = ''
    for hit in grep(files, re.compile(r"store:[A-Za-z0-9_.\-/]+")):
        for name in re.findall(r"store:[A-Za-z0-9_.\-/]+", hit):
            listed = 'yes' if ('"%s"' % name.split(':', 1)[1]) in idx else 'no'
            out.append('%s  [store/index.json lists %s: %s]' % (hit, name.split(':', 1)[1], listed))
    return out


def git_add(path):
    """Commit date of the (oldest) commit that adds path, read-only git log."""
    r = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                       capture_output=True, text=True, timeout=120)
    lines = [l for l in r.stdout.splitlines() if l.strip()]
    if not lines:
        return ''
    h, iso = lines[-1].split(' ', 1)
    return '%s %s %s' % (iso, h, path)


def first_raw_date(paths):
    dates = [(git_add(p), p) for p in paths]
    dates = [d for d, _ in dates if d]
    if not dates:
        return ''
    return sorted(dates, key=lambda s: s.split(' ')[0])[0]


def places_of(claimed):
    s = str(claimed)
    return len(s.split('.')[1]) if '.' in s else 0


def same(claimed, value, places=None):
    """match if equal at the printed precision (half-up) or relative difference <= 1 %."""
    if places is None:
        places = places_of(claimed)
    c = Decimal(str(claimed))
    v = Decimal(str(value))
    q = Decimal(1).scaleb(-places)
    if v.quantize(q, rounding=ROUND_HALF_UP) == c.quantize(q, rounding=ROUND_HALF_UP):
        return 'match'
    if c != 0 and abs(v - c) / abs(c) <= Decimal('0.01'):
        return 'match'
    return 'mismatch'


def C(claimed, computed, definition, files, status=None):
    if status is None:
        status = same(claimed, computed)
    return {'claimed': str(claimed), 'computed': str(computed), 'definition': definition,
            'files': list(files), 'status': status}


def no_identical():
    return {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
            'files': [], 'status': 'not_claimed'}


def emit(row_no, slug, claims, identical, offrepo, aa, iso, protocol_date, first_raw,
         script, notes):
    obj = {'row_no': row_no, 'slug': slug, 'claims': claims, 'identical': identical,
           'offrepo_refs': offrepo, 'aa_lines': aa, 'isolation_lines': iso,
           'protocol_git_date': protocol_date, 'first_raw_git_date': first_raw,
           'script': script, 'notes': notes}
    print(json.dumps(obj, ensure_ascii=False))


def scan_study(dirs):
    files = study_files(dirs)
    return files, grep(files, OFF_RE), grep(files, AA_RE), grep(files, ISO_RE)


def median(xs):
    xs = sorted(xs)
    n = len(xs)
    if n == 0:
        return None
    m = n // 2
    return xs[m] if n % 2 else (xs[m - 1] + xs[m]) / 2
