import os, re, subprocess, json, math, csv

ROOT = '/home/user/crypto'
OFFREPO_RX = r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|modal volume|not committed|scratch"
AA_RX = r"A/A|A/B|null|control|Control|negative|positive"
ISO_RX = r"isolat|shared|unpinned|pinned|PMU|\bL0\b|\bL1\b|\bL2\b|\bL3\b|Tier|isolation|eviction|NUMA|movable"


def p(x):
    return os.path.join(ROOT, x)


def grep_lines(files, rx):
    rx = re.compile(rx)
    out = []
    for f in files:
        if not os.path.exists(p(f)):
            out.append(f"MISSING:{f}")
            continue
        with open(p(f), encoding='utf-8', errors='replace') as fh:
            for i, line in enumerate(fh, 1):
                if rx.search(line):
                    out.append(f"{f}:{i}: {line.rstrip()}")
    return out


def git_added(path):
    if not path:
        return ''
    r = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                       cwd=ROOT, capture_output=True, text=True)
    return r.stdout.strip().replace('\n', ' | ')


def mean_sd_ci(xs):
    n = len(xs)
    m = sum(xs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1)) if n > 1 else float('nan')
    return {'n': n, 'mean': m, 'sd': sd, 'ci95': 1.96 * sd / math.sqrt(n)}


def rel_status(claimed, computed, rel=0.01):
    if computed is None:
        return 'cannot_define'
    if claimed == 0:
        return 'match' if computed == 0 else 'mismatch'
    return 'match' if abs(computed - claimed) / abs(claimed) <= rel else 'mismatch'


def write_row(row_no, row, path):
    with open(path, 'w') as fh:
        fh.write(json.dumps(row, ensure_ascii=False) + '\n')
    print(json.dumps(row, ensure_ascii=False, indent=1))
