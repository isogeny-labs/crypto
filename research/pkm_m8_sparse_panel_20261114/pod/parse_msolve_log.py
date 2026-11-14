#!/usr/bin/env python3
"""Tabulate an msolve 0.6.5 `-v 2` log (stdout trace, optionally with the stderr INPUT DATA box appended).
Tabulation only: no estimate, no model. Per F4 round: degree, sel pairs, pairs, matrix rows x cols,
density, new, zero reductions, real and cpu seconds. Totals: rounds, max round degree, sum rows*cols*density
("nonzero-entries work proxy"; `up_to_degree_8` = cumulative real/cpu seconds of the rounds with degree <= 8 (canary drift metric), counts the matrix entries each round's matrix holds, nothing else), wall.
A log cut by timeout/rlimit has an unfinished last row: it is reported as `incomplete_last_row`, not parsed.
`-P 1` runs without `-g` may hold several passes (variable-order retry, extra linear form `A`): each pass is
tabulated separately (banners are recorded) and totals are over all passes.
Usage: parse_msolve_log.py LOG [--json OUT] [--table]"""
import json, re, sys

ROW = re.compile(r'^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+([\d.]+)%\s+(\d+) new\s+(\d+) zero\s+([\d.]+) \| ([\d.]+)\s*$')
FIN = re.compile(r'^reduce final basis\s+(\d+) x (\d+)\s+([\d.]+)%\s+(\d+) new\s+(\d+) zero\s+([\d.]+) \| ([\d.]+)\s*$')
PARTIAL = re.compile(r'^\s*\d+\s+\d+\s+\d+')
KV = {'variables': r'#variables\s+(\d+)', 'equations': r'#equations\s+(\d+)', 'characteristic': r'field characteristic\s+(\d+)',
      'threads': r'#threads\s+(\d+)', 'dim_quotient': r'Dimension of quotient:\s*(\d+)',
      'overall_elapsed_s': r'overall\(elapsed\)\s+([\d.]+) sec', 'overall_cpu_s': r'overall\(cpu\)\s+([\d.]+) sec',
      'msolve_overall_elapsed_s': r'msolve overall time\s+([\d.]+) sec \(elapsed\)'}

def parse(text):
    passes, cur, banners, meta = [], None, [], {}
    incomplete = None
    lines = text.splitlines()
    for i, ln in enumerate(lines):
        if re.match(r'^Legend for f4? ?information', ln):       # "f4" in the first pass, "f" in later passes of -P 1
            cur = {'rows': [], 'final': None, 'banner': banners[-1] if banners else 'first pass (input variable order)'}
            passes.append(cur)
            continue
        if ln.startswith('Changing variable order') or ln.startswith('Adding a linear form'):
            banners.append(ln.strip())
            continue
        m = ROW.match(ln)
        if m and cur is not None:
            d, sel, pairs, r, c, dens, new, zero, rt, ct = m.groups()
            cur['rows'].append(dict(deg=int(d), sel=int(sel), pairs=int(pairs), rows=int(r), cols=int(c), density_pct=float(dens),
                                    new=int(new), zero=int(zero), real_s=float(rt), cpu_s=float(ct),
                                    entries=int(r) * int(c) * float(dens) / 100.0))
            continue
        m = FIN.match(ln)
        if m and cur is not None:
            r, c, dens, new, zero, rt, ct = m.groups()
            cur['final'] = dict(rows=int(r), cols=int(c), density_pct=float(dens), new=int(new), zero=int(zero), real_s=float(rt), cpu_s=float(ct),
                                entries=int(r) * int(c) * float(dens) / 100.0)
            continue
        if cur is not None and cur['final'] is None and PARTIAL.match(ln) and i >= len(lines) - 2:
            incomplete = ln.rstrip()
        for k, rx in KV.items():
            m = re.search(rx, ln)
            if m: meta.setdefault(k, []).append(m.group(1))
    return passes, meta, incomplete

def totals(passes, incomplete):
    rows = [r for p in passes for r in p['rows']]
    fin = [p['final'] for p in passes if p['final']]
    t = dict(passes=len(passes), rounds=len(rows), max_round_degree=max((r['deg'] for r in rows), default=None),
             sum_rows_x_cols_x_density=sum(r['entries'] for r in rows),
             sum_rows_x_cols_x_density_with_final=sum(r['entries'] for r in rows) + sum(f['entries'] for f in fin),
             max_matrix_entries=max((r['entries'] for r in rows), default=None),
             max_matrix_rows=max((r['rows'] for r in rows), default=None), max_matrix_cols=max((r['cols'] for r in rows), default=None),
             sum_new=sum(r['new'] for r in rows), sum_zero=sum(r['zero'] for r in rows),
             sum_round_real_s=sum(r['real_s'] for r in rows), sum_round_cpu_s=sum(r['cpu_s'] for r in rows),
             completed=all(p['final'] for p in passes) and bool(passes) and incomplete is None,
             incomplete_last_row=incomplete)
    # canary/drift metric: cumulative time of the rounds of degree <= 8 (all passes, file order); also the first-degree-9 cut
    le = [r for r in rows if r['deg'] <= 8]
    t['up_to_degree_8'] = dict(rounds=len(le), real_s=round(sum(r['real_s'] for r in le), 3), cpu_s=round(sum(r['cpu_s'] for r in le), 3),
                               entries=sum(r['entries'] for r in le), reached_degree_9=any(r['deg'] > 8 for r in rows),
                               rounds_before_first_degree_gt_8=next((n for n, r in enumerate(rows) if r['deg'] > 8), len(rows)))
    if rows:
        b = max(rows, key=lambda r: r['entries']); t['largest_matrix_round'] = dict(deg=b['deg'], rows=b['rows'], cols=b['cols'], density_pct=b['density_pct'])
    return t

def main(argv):
    if not argv or argv[0] in ('-h', '--help'): print(__doc__); return 2
    path = argv[0]; text = open(path, errors='replace').read()
    passes, meta, incomplete = parse(text)
    out = dict(schema='pkm.msolve-log-table/v1', log=path, meta={k: v[-1] if len(v) == 1 else v for k, v in meta.items()},
               totals=totals(passes, incomplete), passes=passes)
    if '--json' in argv: json.dump(out, open(argv[argv.index('--json') + 1], 'w'), indent=1)
    if '--table' in argv or '--json' not in argv:
        for n, p in enumerate(passes):
            print(f"# pass {n}: {p['banner']}")
            print('deg    sel      pairs         rows       cols  dens%    new    zero   real_s    cpu_s')
            for r in p['rows']:
                print(f"{r['deg']:3d} {r['sel']:6d} {r['pairs']:10d} {r['rows']:10d} {r['cols']:10d} {r['density_pct']:6.2f} {r['new']:6d} {r['zero']:7d} {r['real_s']:8.2f} {r['cpu_s']:8.2f}")
            if p['final']: f = p['final']; print(f"final {f['rows']} x {f['cols']} {f['density_pct']}%")
        print(json.dumps(out['totals'], indent=1))
    return 0

if __name__ == '__main__': sys.exit(main(sys.argv[1:]))
