import json, math, collections, statistics as st
def gm(xs): xs = [max(x, 1) for x in xs]; return 2 ** (sum(math.log2(x) for x in xs) / len(xs)) if xs else None
def slope(pts):
    pts = [(x, math.log2(y)) for x, y in pts if y]
    if len(pts) < 2: return None
    mx = sum(x for x, _ in pts) / len(pts); my = sum(y for _, y in pts) / len(pts)
    return sum((x - mx) * (y - my) for x, y in pts) / sum((x - mx) ** 2 for x, _ in pts)
rows = []
for f in ('raw_sat3.jsonl', 'raw_sat2.jsonl', 'raw_l21.jsonl'):
    try: rows += [json.loads(x) for x in open(f)]
    except FileNotFoundError: pass
def metric(r, name):
    if name == 'l21_cands': return r['l21']['cands'] if 'l21' in r else None
    if name == 'l21_leaves': return r['l21']['leaves'] if 'l21' in r else None
    if name.startswith('sat_'): return r['sat'][name[4:]] if 'sat' in r else None
tables = {}; md = []
for m, names in ((3, ['l21_leaves', 'l21_cands', 'sat_conflicts', 'sat_decisions', 'sat_propagations']), (2, ['sat_conflicts', 'sat_decisions', 'sat_propagations'])):
    cells = sorted({(r['l'], r['n']) for r in rows if r['m'] == m})
    for name in names:
        for dec in (True, False):
            series = collections.defaultdict(list)
            md.append(f"\n### m={m} {name} {'decomposable' if dec else 'non-decomposable'} (geometric mean per instance)\n")
            md.append("| l | n | w(S) | w(A,B) | N S/A/B | S | A | B | S/A | A/B (A/A floor) | per-base gm range A∪B | per-base gm range S |")
            md.append("|---|---|---|---|---|---|---|---|---|---|---|---|")
            for (l, n) in cells:
                sel = [r for r in rows if r['m'] == m and r['l'] == l and r['n'] == n and r['dec'] == dec and metric(r, name) is not None]
                if not sel: continue
                g = {}; cnt = {}; pb = collections.defaultdict(list)
                for arm in 'SAB':
                    v = [metric(r, name) for r in sel if r['arm'] == arm]
                    g[arm] = gm(v); cnt[arm] = len(v)
                    for b in range(4):
                        vb = [metric(r, name) for r in sel if r['arm'] == arm and r['base'] == b]
                        if vb: pb[arm].append(gm(vb))
                    if g[arm]: series[arm].append((l, g[arm]))
                ws = sorted({r['w'] for r in sel if r['arm'] == 'S'}); wr = sorted({r['w'] for r in sel if r['arm'] != 'S'})
                f = lambda x: '-' if x is None else (f"{x:.3g}")
                rat = lambda a, b: '-' if not (a and b) else f"{a / b:.3f}"
                ab = pb['A'] + pb['B']
                md.append(f"| {l} | {n} | {ws} | {wr} | {cnt['S']}/{cnt['A']}/{cnt['B']} | {f(g['S'])} | {f(g['A'])} | {f(g['B'])} | {rat(g['S'], g['A'])} | {rat(g['A'], g['B'])} | "
                          f"{f(min(ab)) if ab else '-'}..{f(max(ab)) if ab else '-'} | {f(min(pb['S'])) if pb['S'] else '-'}..{f(max(pb['S'])) if pb['S'] else '-'} |")
                tables[f"m{m}|{name}|{'dec' if dec else 'nondec'}|l{l}n{n}"] = dict(gm=g, n=cnt, per_base=dict(pb), w_S=ws, w_AB=wr)
            sl = {arm: slope(series[arm]) for arm in 'SAB'}
            tables[f"m{m}|{name}|{'dec' if dec else 'nondec'}|slope_bits_per_l"] = sl
            md.append("\nslope (bits per l, least squares over the cells above): " + ", ".join(f"{a}={sl[a]:.2f}" if sl[a] is not None else f"{a}=-" for a in 'SAB'))
# fraction decomposable among random targets, and L21 decision rate
md.append("\n### fraction of the 4 shared random targets decomposable, per arm; L21 leaves with <=1 candidate\n")
md.append("| m | l | n | S | A | B | le1-rate S | le1-rate A | le1-rate B | truncated leaves |"); md.append("|---|---|---|---|---|---|---|---|---|---|")
for m in (3, 2):
    for (l, n) in sorted({(r['l'], r['n']) for r in rows if r['m'] == m}):
        fr = {}; le = {}; tr = 0
        for arm in 'SAB':
            sel = [r for r in rows if r['m'] == m and r['l'] == l and r['n'] == n and r['arm'] == arm]
            rr = [r for r in sel if r['tkind'] == 'random']
            fr[arm] = f"{sum(r['dec'] for r in rr)}/{len(rr)}"
            if m == 3:
                le[arm] = f"{sum(r['l21']['le1'] for r in sel) / max(1, sum(r['l21']['leaves'] for r in sel)):.3f}"; tr += sum(r['l21']['trunc'] for r in sel)
            else: le[arm] = '-'
        md.append(f"| {m} | {l} | {n} | {fr['S']} | {fr['A']} | {fr['B']} | {le['S']} | {le['A']} | {le['B']} | {tr} |")
# defect
try:
    D = json.load(open('defect.json'))
    md.append("\n### defect distributions (w = dim V.V; def2/def3 = literal Macaulay defect of the m=2 system at D=2/D=3)\n")
    md.append("| l | n | 2l-1 | random w: min / 5% / median / max | random def2 med (min..max) | random def3 med (min..max) | poly w, def2, def3 | frob w, def2, def3 | trace0 w, def2, def3 | hill blind start->end | hill guided start->end |")
    md.append("|---|---|---|---|---|---|---|---|---|---|---|")
    for c in D['cells']:
        ws = sorted(x['w'] for x in c['random']); d2 = sorted(x['def2'] for x in c['random']); d3 = sorted(x['def3'] for x in c['random'])
        u = lambda k: "; ".join(sorted({f"{x['w']},{x['def2']},{x['def3']}" for x in c[k]}))
        h = lambda k: "; ".join(f"{a}->{b}" for a, b in c[k])
        md.append(f"| {c['l']} | {c['n']} | {2 * c['l'] - 1} | {ws[0]} / {ws[9]} / {ws[100]} / {ws[-1]} | {d2[100]} ({d2[0]}..{d2[-1]}) | {d3[100]} ({d3[0]}..{d3[-1]}) | {u('poly')} | {u('frob')} | {u('trace0')} | {h('hill_blind')} | {h('hill_guided')} |")
    md.append("\nexhaustive: " + json.dumps(D['exhaustive'])); md.append("\npositive control (subfield, n=15): " + json.dumps(D['control']))
except FileNotFoundError: D = None
open('summary_tables.md', 'w').write("\n".join(md) + "\n")
json.dump(dict(master_seed=2026111411, tag='pcc:unverified pilot', summary=tables, defect=D, rows=rows), open('results.json', 'w'))
print("\n".join(md))
